use crate::Token;

#[derive(Debug)]
pub enum Parsed {
    VariableDeclare(VariableDeclare),
    FunctionDeclare(FunctionDeclare),
    FunctionCall(FunctionCall),
    Print(String),
    RSquirly(String),
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
                Token::LetInt(_) => {
                    let mut name = String::new();
                    let mut value = String::new();
                    let mut is_assigned = false;
                    let mut is_semicoloned = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::VarName((_, var_name)) => name = var_name.to_owned(),
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
                    let mut value = String::new();
                    let mut is_assigned = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::VarName((_, var_name)) => name = var_name.to_owned(),
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
            }

            i += 1;
        }
    }

    return parsed_lines;
}