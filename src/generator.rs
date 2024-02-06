use crate::parser::{Parsed, FunctionCall, VariableDeclare};

pub fn generator(parsed: Vec<Parsed>) -> String {
    let mut gen = String::new();

    for elem in parsed {
        match elem {
            Parsed::VariableDeclare(VariableDeclare { type_class, name, value }) => {
                let mut to_rust = String::new();

                if type_class == String::from("String") {
                    to_rust = format!("let {name}: {type_class} = String::from(\"{value}\");\n");
                } else if type_class == String::from("i32") {
                    to_rust = format!("let {name}: {type_class} = {value};\n");
                } else if type_class == String::from("bool") {
                    to_rust = format!("let {name}: {type_class} = {value};")
                }

                gen.push_str(&to_rust);
            },
            Parsed::FunctionDeclare(declare) => {
                let content_change = vec![
                    (String::from("string"), String::from("String")),
                    (String::from("int"), String::from("i32")),
                    (String::from("bool"), String::from("bool")),
                ];

                let re_params = declare.sanitise_params(content_change);

                let to_rust = format!("fn {}({re_params}) {{", declare.name);
                gen.push_str(&to_rust);
            },
            Parsed::FunctionCall(FunctionCall { name, parameters }) => {
                let mut output = String::new();
                let mut inside_quotes = false;
                let mut word_start = 0;

                for (i, c) in parameters.char_indices() {
                    if c == '"' {
                        inside_quotes = !inside_quotes;
                        if inside_quotes {
                            output.push_str("String::from(\"");
                            word_start = i + 1;
                        } else {
                            let word = &parameters[word_start..i];
                            output.push_str(word);
                            output.push_str("\")");
                        }
                    } else if !inside_quotes {
                        output.push(c);
                    }
                }

                let to_rust = format!("{name}({output});");
                gen.push_str(&to_rust);
            },
            Parsed::Print(body) => {
                let to_rust = format!("println!({body});");
                gen.push_str(&to_rust);
            },
            Parsed::RSquirly(_) => gen.push_str("}"),
            Parsed::If(body) => {
                let to_rust = format!("if {body} {{");
                gen.push_str(&to_rust);
            },
            Parsed::OrIf(body) => {
                let to_rust = format!("else if {body} {{");
                gen.push_str(&to_rust);
            },
            Parsed::Else => {
                let to_rust = format!("else {{");
                gen.push_str(&to_rust);
            },
        }
    }

    return gen;
}
