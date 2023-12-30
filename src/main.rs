use std::env;
use std::fs;

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
    Let((String, String)),
    VarName((String, String)),
    Semicolon((String, String)),
    Comma((String, String)),
    Function((String, String)),
    FuncName((String, String)),
    Print((String, String)),
    WhiteSpace((String, String)),
    NewLine((String, String)),
}

// impl Token {
//     fn gen(&self) {
//         match self {
//             Token::Let((type_class, value)) => todo!(),
//             Token::VarName((type_class, value)) => todo!(),
//             Token::Function((type_class, value)) => todo!(),
//             Token::FuncName((type_class, value)) => todo!(),
//             Token::Print((type_class, value)) => todo!(),
//             _ => (),
//         }
//     }
// }

fn handle_ending_value(tokens: &mut Vec<Token>, current_token: &mut String) {
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
            } else if current_token == "let" {
                tokens.push(Token::Let((String::from("LET"), current_token.to_string())))
            } else if current_token == "proc" {
                tokens.push(Token::Function((String::from("FUNCTION"), current_token.to_string())))
            } else if current_token == "print" {
                tokens.push(Token::Print((String::from("PRINT"), current_token.to_string())))
            } else {
                match tokens.last().unwrap() {
                    Token::Let(_) => tokens.push(Token::VarName((String::from("VARNAME"), current_token.to_string()))),
                    Token::Function(_) => tokens.push(Token::FuncName((String::from("FUNCNAME"), current_token.to_string()))),
                    Token::DblQuote(_) | Token::Strings(_) | Token::WhiteSpace(_) => tokens.push(Token::Strings((String::from("STRINGS"), current_token.to_string()))),
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

    for c in content.chars() {
        println!("{}", c);
        if c.is_digit(10) || c.is_alphabetic() {
            current_token.push(c);
        } else if c == '+' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Plus((String::from("PLUS"), String::from(c))));
        } else if c == '-' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Minus((String::from("MINUS"), String::from(c))));
        } else if c == '*' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Multiply((String::from("MULTIPLY"), String::from(c))));
        } else if c == '/' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Divide((String::from("DIVIDE"), String::from(c))));
        } else if c == '"' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::DblQuote((String::from("DBLQUOTE"), String::from(c))));
        } else if c == '\'' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::SglQuote((String::from("SGLQUOTE"), String::from(c))));
        } else if c == '(' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::LParen((String::from("LPAREN"), String::from(c))));
        } else if c == ')' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::RParen((String::from("RPAREN"), String::from(c))));
        } else if c == '{' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::LSquirly((String::from("LSQUIRLY"), String::from(c))));
        } else if c == '}' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::RSquirly((String::from("RSQUIRLY"), String::from(c))));
        } else if c == ',' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Comma((String::from("COMMA"), String::from(c))));
        } else if c == '\n' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::NewLine((String::from("NEWLINE"), String::from(c))));
        } else if c == ' ' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::WhiteSpace((String::from("WHITESPACE"), String::from(c))));
        } else if c == ';' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(Token::Semicolon((String::from("SEMICOLON"), String::from(c))))
        } else if c == '=' {
            current_token.push(c);
        } else {
            panic!("invalid character: {}", c);
        }
    };

    return tokens;
}

fn parser(tokens: Vec<Token>) -> String {
    let mut generated = String::new();

    for token in tokens {
        match token {
            Token::Plus((_, value)) => generated.push_str(&value),
            Token::Minus((_, value)) => generated.push_str(&value),
            Token::Multiply((_, value)) => generated.push_str(&value),
            Token::Divide((_, value)) => generated.push_str(&value),
            Token::LParen((_, value)) => generated.push_str(&value),
            Token::RParen((_, value)) => generated.push_str(&value),
            Token::SglQuote((_, value)) => generated.push_str(&value),
            Token::DblQuote((_, value)) => generated.push_str(&value),
            Token::LSquirly((_, value)) => generated.push_str(&value),
            Token::RSquirly((_, value)) => generated.push_str(&value),
            Token::EqualsTo((_, value)) => generated.push_str(&value),
            Token::Equality((_, value)) => generated.push_str(&value),
            Token::Number((_, value)) => generated.push_str(&value.to_string()),
            Token::Strings((_, value)) => generated.push_str(&value),
            Token::Let((_, value)) => generated.push_str(&value),
            Token::VarName((_, value)) => generated.push_str(&value),
            Token::Semicolon((_, value)) => generated.push_str(&value),
            Token::Comma((_, value)) => generated.push_str(&value),
            Token::Function((_, _)) => generated.push_str("fn"),
            Token::FuncName((_, value)) => generated.push_str(&value),
            Token::Print((_, _)) => generated.push_str("println!"),
            Token::NewLine((_, value)) => generated.push_str(&value),
            Token::WhiteSpace((_, value)) => generated.push_str(&value),
        }
    }

    return generated;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents_result = fs::read_to_string(args[1].clone());
    let contents = match contents_result {
        Ok(contents) => contents,
        Err(_) => String::from("couldn't read"),
    };

    let tokenised = tokeniser(contents);
    let generated = parser(tokenised);

    match fs::write("./gen.rs", generated) {
        Ok(_) => println!("produced gen.rs"),
        Err(_) => panic!("error writing to file"),
    }
}
