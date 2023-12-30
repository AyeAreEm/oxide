use std::env;
use std::fs;
use parser::parser;
mod parser;

// add booleans, comments, if statements, while / for loops, function parameters, maybe more ints, return values, arrays & vectors
#[derive(Debug, Clone)]
enum Token {
    Plus((String, String)),
    Minus((String, String)),
    Multiply((String, String)),
    Divide((String, String)),
    LParen((String, String)),
    RParen((String, String)),
    SglQuote((String, String)),
    DblQuote((String, String)),
    LSquirly((String, String)),
    RSquirly((String, String)),
    EqualsTo((String, String)),
    Equality((String, String)),
    Number((String, i32)),
    Strings((String, String)),
    LetInt((String, String)),
    LetString((String, String)),
    VarName((String, String)),
    Semicolon((String, String)),
    Comma((String, String)),
    Function((String, String)),
    FuncName((String, String)),
    Print((String, String)),
    // WhiteSpace((String, String)),
    NewLine((String, String)),
}

fn handle_ending_value(tokens: &mut Vec<Token>, current_token: &mut String, making_string: &mut i8) {
    if current_token.is_empty() {
        return;
    }

    let mut named = Vec::new();
    for token in tokens.clone() {
        match token {
            Token::VarName(name) | Token::FuncName(name)=> named.push(name),
            _ => (),
        }
    }

    let new_token_result = current_token.parse::<i32>();
    match new_token_result {
        Ok(new_token) => tokens.push(Token::Number((String::from("NUMBER"), new_token))),
        Err(_) => {
            if current_token == "=" {
                tokens.push(Token::EqualsTo((String::from("EQUALSTO"), current_token.to_string())))
            } else if current_token == "==" {
                tokens.push(Token::Equality((String::from("EQUALITY"), current_token.to_string())))
            } else if current_token == "int" {
                tokens.push(Token::LetInt((String::from("LETINT"), current_token.to_string())))
            } else if current_token == "string" {
                tokens.push(Token::LetString((String::from("LETSTRING"), current_token.to_string())))
            } else if current_token == "proc" {
                tokens.push(Token::Function((String::from("FUNCTION"), current_token.to_string())))
            } else if current_token == "print" {
                tokens.push(Token::Print((String::from("PRINT"), current_token.to_string())))
            } else {
                match tokens.last().unwrap() {
                    Token::LetInt(_) | Token::LetString(_) => tokens.push(Token::VarName((String::from("VARNAME"), current_token.to_string()))),
                    Token::Function(_) => tokens.push(Token::FuncName((String::from("FUNCNAME"), current_token.to_string()))),
                    Token::DblQuote(_) => {
                        if *making_string == 1 {
                            tokens.push(Token::Strings((String::from("STRINGS"), current_token.to_string())));
                        } else if *making_string == 2 {
                            tokens.push(Token::Strings((String::from("STRINGS"), current_token.to_string())));
                            *making_string = 0;
                        }
                    },
                    _ => {
                        let mut found = false;

                        for (type_class, value) in named {
                            if current_token.to_owned() == value && type_class == String::from("VARNAME") {
                                tokens.push(Token::VarName((String::from("VARNAME"), current_token.to_string())));
                                found = true;
                            } else if current_token.to_owned() == value && type_class == String::from("FUNCNAME") {
                                tokens.push(Token::FuncName((String::from("FUNCNAME"), current_token.to_string())));
                                found = true;
                            }
                        }

                        if !found {
                            panic!("invalid characters: {}", current_token)
                        }
                    },
                }
            }
        },
    };

    current_token.clear();
}

fn tokeniser(content: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut making_string: i8 = 0;

    // fix singel quotes
    for c in content.chars() {
        if c.is_digit(10) || c.is_alphabetic() || (making_string > 0 && c != '"') {
            current_token.push(c);
        } else if c == '+' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Plus((String::from("PLUS"), String::from(c))));
        } else if c == '-' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Minus((String::from("MINUS"), String::from(c))));
        } else if c == '*' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Multiply((String::from("MULTIPLY"), String::from(c))));
        } else if c == '/' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Divide((String::from("DIVIDE"), String::from(c))));
        } else if c == '"' {
            making_string += 1;
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::DblQuote((String::from("DBLQUOTE"), String::from(c))));
        } else if c == '\'' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::SglQuote((String::from("SGLQUOTE"), String::from(c))));
        } else if c == '(' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::LParen((String::from("LPAREN"), String::from(c))));
        } else if c == ')' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::RParen((String::from("RPAREN"), String::from(c))));
        } else if c == '{' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::LSquirly((String::from("LSQUIRLY"), String::from(c))));
        } else if c == '}' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::RSquirly((String::from("RSQUIRLY"), String::from(c))));
        } else if c == ',' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Comma((String::from("COMMA"), String::from(c))));
        } else if c == '\n' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::NewLine((String::from("NEWLINE"), String::from(c))));
        } else if c == ' ' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            // tokens.push(Token::WhiteSpace((String::from("WHITESPACE"), String::from(c))));
        } else if c == ';' {
            handle_ending_value(&mut tokens, &mut current_token, &mut making_string);
            tokens.push(Token::Semicolon((String::from("SEMICOLON"), String::from(c))))
        } else if c == '=' {
            current_token.push(c);
        } else {
            current_token.push(c);
        }
    };

    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents_result = fs::read_to_string(args[1].clone());
    let contents = match contents_result {
        Ok(contents) => contents,
        Err(_) => String::from("couldn't read"),
    };

    let tokenised = tokeniser(contents);
    let parsed = parser(tokenised);

    for parse in parsed {
        println!("{:?}", parse);
    }

    // let generated = parser::parser(tokenised);

    // match fs::write("./gen.rs", generated) {
    //     Ok(_) => println!("produced gen.rs"),
    //     Err(_) => panic!("error writing to file"),
    // }
}
