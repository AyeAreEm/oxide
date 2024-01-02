use crate::parser::{Parsed, FunctionDeclare, FunctionCall, VariableDeclare};

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
                }

                gen.push_str(&to_rust);
            },
            Parsed::FunctionDeclare(FunctionDeclare { name, parameters }) => {
                let seperated: Vec<&str> = parameters.split(',').collect();
                let mut re_params = String::new();

                for elem in seperated {
                    if elem.trim().contains("string") {
                        let new: Vec<&str> = elem.trim().split("string").collect();
                        let new_param = if re_params.is_empty() {
                            format!("{}: {}", new[1], "String")
                        } else {
                            format!(", {}: {}", new[1], "String")
                        };

                        re_params.push_str(&new_param);
                    } else if elem.trim().contains("int") {
                        let new: Vec<&str> = elem.trim().split("int").collect();
                        let new_param = if re_params.is_empty() {
                            format!("{}: {}", new[1], "i32")
                        } else {
                            format!(", {}: {}", new[1], "i32")
                        };

                        re_params.push_str(&new_param);

                    }
                }

                let to_rust = format!("fn {name}({re_params}) {{");
                gen.push_str(&to_rust);
            },
            Parsed::FunctionCall(FunctionCall { name, parameters }) => {
                let to_rust = format!("{name}({parameters});");
                gen.push_str(&to_rust);
            },
            Parsed::Print(body) => {
                let to_rust = format!("println!({body});");
                gen.push_str(&to_rust);
            },
            Parsed::RSquirly(_) => gen.push_str("}"),
        }
    }

    return gen;
}