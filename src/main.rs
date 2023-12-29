use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Token<T> {
    type_class: String,
    value: T
}

#[derive(Debug, Clone)]
enum TokenClass {
    Plus(Token<String>),
    Minus(Token<String>),
    Multiply(Token<String>),
    Divide(Token<String>),
    LParen(Token<String>),
    RParen(Token<String>),
    SglQuote(Token<String>),
    DblQuote(Token<String>),
    LSquirly(Token<String>),
    RSquirly(Token<String>),
    EqualsTo(Token<String>),
    Equality(Token<String>),
    Number(Token<i32>),
    Strings(Token<String>),
    Let(Token<String>),
    VarName(Token<String>),
    Semicolon(Token<String>),
    Comma(Token<String>),
    Function(Token<String>),
    FuncName(Token<String>),
    Print(Token<String>),
    // WhiteSpace(Token<String>),
    NewLine(Token<String>),
}

fn handle_ending_value(tokens: &mut Vec<TokenClass>, current_token: &mut String) {
    if current_token.is_empty() {
        return;
    }
    
    let mut named = Vec::new();
    for token in tokens.clone() {
        match token {
            TokenClass::VarName(name) | TokenClass::FuncName(name)=> named.push(name),
            _ => (),
        }
    }

    let new_token_result = current_token.parse::<i32>();
    match new_token_result {
        Ok(new_token) => tokens.push(TokenClass::Number(Token { type_class: String::from("NUMBER"), value: new_token })),
        Err(_) => {
            if current_token == "=" {
                tokens.push(TokenClass::EqualsTo(Token { type_class: String::from("EQUALSTO"), value: current_token.to_string() }))
            } else if current_token == "==" {
                tokens.push(TokenClass::Equality(Token { type_class: String::from("EQUALILTY"), value: current_token.to_string() }))
            } else if current_token == "let" {
                tokens.push(TokenClass::Let(Token { type_class: String::from("LET"), value: current_token.to_string() }))
            } else if current_token == "proc" {
                tokens.push(TokenClass::Function(Token { type_class: String::from("FUNCTION"), value: current_token.to_string() }))
            } else if current_token == "print" {
                tokens.push(TokenClass::Print(Token { type_class: String::from("PRINT"), value: current_token.to_string() }))
            } else {
                match tokens.last().unwrap() {
                    TokenClass::Let(_) => tokens.push(TokenClass::VarName(Token { type_class: String::from("VARNAME"), value: current_token.to_string() })),
                    TokenClass::Function(_) => tokens.push(TokenClass::FuncName(Token { type_class: String::from("FUNCNAME"), value: current_token.to_string() })),
                    TokenClass::DblQuote(_) => tokens.push(TokenClass::Strings(Token { type_class: String::from("STRINGS"), value: current_token.to_string() })),
                    TokenClass::Strings(_) => tokens.push(TokenClass::Strings(Token { type_class: String::from("STRINGS"), value: current_token.to_string() })),
                    _ => {
                        let mut found = false;

                        for name in named {
                            if current_token.to_owned() == name.value && name.type_class == String::from("VARNAME") {
                                tokens.push(TokenClass::VarName(Token { type_class: String::from("VARNAME"), value: current_token.to_string() }));
                                found = true;
                            } else if current_token.to_owned() == name.value {
                                tokens.push(TokenClass::FuncName(Token { type_class: String::from("FUNCNAME"), value: current_token.to_string() }));
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

fn tokeniser(content: String) -> Vec<TokenClass> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in content.chars() {
        if c.is_digit(10) || c.is_alphabetic() {
            current_token.push(c);
        } else if c == '+' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Plus(Token { type_class: String::from("PLUS"), value: String::from(c) }));
        } else if c == '-' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Minus(Token { type_class: String::from("MINUS"), value: String::from(c) }));
        } else if c == '*' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Multiply(Token { type_class: String::from("MULTIPLY"), value: String::from(c) }));
        } else if c == '/' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Divide(Token { type_class: String::from("DIVIDE"), value: String::from(c) }));
        } else if c == '"' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::DblQuote(Token { type_class: String::from("DBLQUOTE"), value: String::from(c) }));
        } else if c == '\'' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::SglQuote(Token { type_class: String::from("SGLQUOTE"), value: String::from(c) }));
        } else if c == '(' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::LParen(Token { type_class: String::from("LPAREN"), value: String::from(c) }));
        } else if c == ')' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::RParen(Token { type_class: String::from("RPAREN"), value: String::from(c) }));
        } else if c == '{' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::LSquirly(Token { type_class: String::from("LSQUIRLY"), value: String::from(c) }));
        } else if c == '}' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::RSquirly(Token { type_class: String::from("RSQUIRLY"), value: String::from(c) }));
        } else if c == ',' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Comma(Token { type_class: String::from("COMMA"), value: String::from(c) }));
        } else if c == '\n' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::NewLine(Token { type_class: String::from("NEWLINE"), value: String::from(c) }));
        } else if c == ' ' {
            handle_ending_value(&mut tokens, &mut current_token);
            // tokens.push(TokenClass::WhiteSpace(Token { type_class: String::from("WHITESPACE"), value: String::from(c) }));
        } else if c == ';' {
            handle_ending_value(&mut tokens, &mut current_token);
            tokens.push(TokenClass::Semicolon(Token { type_class: String::from("SEMICOLON"), value: String::from(c) }))
        } else if c == '=' {
            current_token.push(c);
        } else {
            panic!("invalid character: {}", c);
        }
    };

    return tokens;
}

fn parser(tokens: Vec<TokenClass>) {
    let mut generated = String::new();
    // let mut simplified = Vec::new();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents_result = fs::read_to_string(args[1].clone());
    let contents = match contents_result {
        Ok(contents) => contents,
        Err(_) => String::from("couldn't read"),
    };

    let tokenised = tokeniser(contents);
    for token in tokenised {
        println!("{:?}", token);
    }
}
