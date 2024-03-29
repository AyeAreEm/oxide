use crate::Token;

#[derive(Debug)]
pub enum Parsed {
    VariableDeclare(VariableDeclare),
    FunctionDeclare(FunctionDeclare),
    FunctionCall(FunctionCall),
    Print(String),
    RSquirly(String),
    If(String),
    OrIf(String),
    Else,
}

#[derive(Debug)]
pub struct VariableDeclare {
    pub type_class: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct FunctionDeclare {
    pub name: String,
    pub parameters: String, 
}

impl FunctionDeclare {
    pub fn sanitise_params(&self, content_change: Vec<(String, String)>) -> String {
        let seperated: Vec<_> = self.parameters.split(',').collect();
        let mut re_params = String::new();

        for elem in seperated {
            for (from, to) in &content_change {
                if elem.trim().contains(*&from) {
                    let new: Vec<&str> = elem.trim().split(*&from).collect();
                    let new_param = if re_params.is_empty() {
                        format!("{}: {}", new[1], to)
                    } else {
                        format!(", {}: {}", new[1], to)
                    };

                    re_params.push_str(&new_param);
                }
            }
        }

        return re_params;
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub parameters: String, 
}

pub fn parser(tokens: Vec<Token>) -> Vec<Parsed> {
    let mut lines = Vec::new();
    let mut starting_point: usize = 0;
    let mut parsed_lines: Vec<Parsed> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::NewLine(_) => {
                lines.push(&tokens[starting_point..index]);
                starting_point = index;
            },
            _ => (),
        }
    }

    if starting_point != tokens.len() {
        lines.push(&tokens[starting_point..tokens.len()]);
    }

    for line in lines {
        let mut i = 0;
        while i < line.len() {
            match &line[i] {
                Token::Plus(_) => (),
                Token::Minus(_) => (),
                Token::Multiply(_) => (),
                Token::Divide(_) => (),
                Token::LParen(_) => (),
                Token::RParen(_) => (),
                Token::SglQuote(_) => (),
                Token::DblQuote(_) => (),
                Token::LSquirly(_) => (),
                Token::RSquirly(_) => parsed_lines.push(Parsed::RSquirly(String::from("}"))),
                Token::EqualsTo(_) => (),
                Token::Equality(_) => (),
                Token::Number(_) => (),
                Token::Strings(_) => (),
                Token::Boolean(_) => (),
                Token::LetInt(_) => {
                    let mut name = String::new();
                    let mut found_first_name = false;
                    let mut value = String::new();
                    let mut is_assigned = false;
                    let mut is_semicoloned = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::VarName((_, var_name)) => {
                                if !found_first_name {
                                    name = var_name.to_owned();
                                    found_first_name = true;
                                } else {
                                    value.push_str(var_name);
                                }
                            },
                            Token::EqualsTo(_) => is_assigned = true,
                            Token::Semicolon(_) => is_semicoloned = true,
                            Token::Number((_, num)) => {
                                if is_assigned && !is_semicoloned {
                                    value.push_str(&num.to_string());
                                }
                            }
                            Token::Divide((_, expr)) | Token::Minus((_, expr)) |
                            Token::Plus((_, expr)) | Token::Multiply((_, expr)) |
                            Token::LParen((_, expr)) | Token::RParen((_, expr)) => {
                                if is_assigned && !is_semicoloned {
                                    value.push_str(&expr.to_string());
                                }
                            },
                            _ => (),
                        }

                        j += 1;
                    }

                    if is_assigned && is_semicoloned {
                        parsed_lines.push(Parsed::VariableDeclare(VariableDeclare {
                            type_class: String::from("i32"),
                            name,
                            value,
                        }));
                        i = j;
                    } else {
                        panic!("invalid syntax")
                    }
                },
                Token::LetString(_) => {
                    let mut name = String::new();
                    let mut found_first_name = false;
                    let mut value = String::new();
                    let mut is_assigned = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::VarName((_, var_name)) => {
                                if !found_first_name {
                                    name = var_name.to_owned();
                                    found_first_name = true;
                                } else {
                                    value.push_str(var_name);
                                }
                            },
                            Token::EqualsTo(_) => is_assigned = true,
                            Token::Strings((_, v)) => value = v.to_owned(),
                            _ => (),
                        }

                        j += 1;
                    }

                    if is_assigned {
                        parsed_lines.push(Parsed::VariableDeclare(VariableDeclare {
                            type_class: String::from("String"),
                            name,
                            value,
                        }));
                        
                        i = j;
                    } else {
                        panic!("invalid syntax");
                    }
                },
                Token::LetBool(_) => {
                    let mut name = String::new();
                    let mut found_first_name = false;
                    let mut value = String::new();
                    let mut is_assigned = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::VarName((_, var_name)) => {
                                if !found_first_name {
                                    name = var_name.to_owned();
                                    found_first_name = true;
                                } else {
                                    value.push_str(var_name);
                                }
                            },
                            Token::EqualsTo(_) => is_assigned = true,
                            Token::Equality(_) => {
                                if is_assigned {
                                    value.push_str("==");
                                }
                            },
                            Token::Boolean((_, v)) => {
                                match v {
                                    true => value = String::from("true"),
                                    false => value = String::from("false"),
                                }
                            },
                            _ => (),
                        }

                        j += 1;
                    }

                    if is_assigned {
                        parsed_lines.push(Parsed::VariableDeclare(VariableDeclare {
                            type_class: String::from("bool"),
                            name,
                            value
                        }));

                        i = j
                    } else {
                        panic!("invalid syntax");
                    }
                },
                Token::VarName(_) => (),
                Token::Semicolon(_) => (),
                Token::Comma(_) => (),
                Token::Function(_) => {
                    let mut name = String::new();
                    let mut parameters = String::new();
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::FuncName((_, func_name)) => name = func_name.to_owned(),
                            Token::Parameters((_, value)) => parameters.push_str(value),
                            _ => (),
                        }

                        j += 1;
                    }

                    parsed_lines.push(Parsed::FunctionDeclare(FunctionDeclare {
                        name,
                        parameters
                    }));
                    i = j;
                },
                Token::FuncName((_, name)) => {
                    let mut is_declare = false;
                    let mut is_calling = false;
                    let mut parameters = String::new();

                    match &line[i-1] {
                        Token::Function(_) => is_declare = true,
                        _ => (),
                    }

                    if !is_declare {
                        let mut j = i + 1;
                        while j < line.len() {
                            match &line[j] {
                                Token::LParen(_) => {
                                    is_calling = true;
                                },
                                Token::Parameters((_, value)) => parameters.push_str(value),
                                _ => (),
                            }

                            j += 1;
                        }

                        if is_calling {
                            parsed_lines.push(Parsed::FunctionCall(FunctionCall {
                                name: name.to_owned(),
                                parameters,
                            }));
                        } else {
                            panic!("invalid syntax");
                        }
                        i = j;
                    }
                },
                Token::Print(_) => {
                    let mut body = String::new();
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::Parameters((_, value)) => body.push_str(value),
                            _ => (),
                        }

                        j += 1;
                    }

                    parsed_lines.push(Parsed::Print(body));
                },
                Token::NewLine(_) => (),
                Token::Parameters(_) => (),
                Token::Comment(_) => (),
                Token::If(_) => {
                    let mut is_opened = false;
                    let mut parameter = String::new();
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::Parameters((_, value)) => parameter.push_str(value),
                            Token::LSquirly(_) => is_opened = true,
                            _ => (),
                        }
                        j += 1;
                    }
                    
                    if is_opened {
                        parsed_lines.push(Parsed::If(parameter));
                    }

                    i = j;
                },
                Token::OrIf(_) => {
                    let mut is_opened = false;
                    let mut parameter = String::new();
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::Parameters((_, value)) => parameter.push_str(value),
                            Token::LSquirly(_) => is_opened = true,
                            _ => (),
                        }
                        j += 1;
                    }
                    
                    if is_opened {
                        parsed_lines.push(Parsed::OrIf(parameter));
                    }

                    i = j;
                },
                Token::Else(_) => parsed_lines.push(Parsed::Else),
                Token::Vector(_) => (),
                Token::LSquare(_) => (),
                Token::RSquare(_) => (),
            }

            i += 1;
        }
    }

    return parsed_lines;
}
