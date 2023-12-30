use crate::Token;

#[derive(Debug)]
pub enum Parsed {
    VariableDeclare(VariableDeclare),
    FunctionDeclare(FunctionDeclare),
    FunctionCall(String),
    Print(Print),
}

#[derive(Debug)]
struct VariableDeclare {
    type_class: String,
    name: String,
    value: String,
}

#[derive(Debug)]
struct FunctionDeclare {
    name: String,
    parameters: String,
}

#[derive(Debug)]
struct Print {
    body: String,
    parameters: String,
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
                Token::RSquirly(_) => (),
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
                            Token::Plus((_, expr)) | Token::Multiply((_, expr)) => {
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
                    let parameters = String::new();
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::FuncName((_, func_name)) => name = func_name.to_owned(),
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

                    match &line[i-1] {
                        Token::Function(_) => is_declare = true,
                        _ => (),
                    }

                    if !is_declare {
                        match &line[i+1] {
                            Token::LParen(_) => parsed_lines.push(Parsed::FunctionCall(name.to_owned())),
                            _ => (),
                        }
                    }
                },
                Token::Print(_) => {
                    let mut body = String::new();
                    let mut parameters = String::new();
                    let mut is_parameters = false;
                    let mut is_closed = false;
                    let mut j = i + 1;

                    while j < line.len() {
                        match &line[j] {
                            Token::Strings((_, value)) => body.push_str(value),
                            Token::Semicolon(_) => {
                                match &line[j-1] {
                                    Token::RParen(_) => is_closed = true,
                                    _ => (),
                                }
                            },
                            Token::Comma((_, value)) => {
                                if !is_parameters {
                                    is_parameters = true;
                                } else {
                                    parameters.push_str(value);
                                }
                            },
                            Token::VarName((_, name)) | Token::Plus((_, name)) |
                            Token::Minus((_, name)) | Token::Multiply((_, name)) | Token::Divide((_, name)) |
                            Token::LParen((_, name)) | Token::RParen((_, name)) => {
                                if is_parameters && !is_closed {
                                    parameters.push_str(name);
                                }
                            },
                            Token::Number((_, name)) => {
                                if is_parameters && !is_closed {
                                    parameters.push_str(&name.to_string());
                                }
                            }
                            _ => (),
                        }

                        j += 1;
                    }

                    parsed_lines.push(Parsed::Print(Print {
                        body,
                        parameters,
                    }));
                },
                Token::NewLine(_) => (),
            }

            i += 1;
        }
    }

    return parsed_lines;
}