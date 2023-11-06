use crate::Token;
use crate::Type;

pub fn tokenize(text: String) -> Vec<Token> {
    let mut string = String::new();
    let mut variable = String::new();
    let mut result: Vec<Token> = Vec::new();
    for c in text.chars() {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' | '=' | ';' => match string.is_empty() {
                true => match variable.is_empty() {
                    true => {
                        println!("Reserved symbol: {}", c);
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    }
                    false => {
                        println!("Variable: {}", variable);
                        result.push(Token::create_token(Type::Variable, variable.clone()));
                        variable.clear();
                        println!("Reserved symbol: {}", c);
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    }
                },
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        println!("Double: {}", string);
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                        println!("Reserved symbol: {}", c);
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    } else {
                        string.push(c);
                    }
                }
            },
            x if x.is_digit(10) => string.push(x),
            '"' => match string.is_empty() {
                true => {
                    match variable.is_empty() {
                        true => string.push(c),
                        false => {
                            println!("Variable: {}", variable);
                            result.push(Token::create_token(Type::Variable, variable.clone()));
                            variable.clear();
                            string.push(c);
                        }
                    }
                    string.push(c);
                }
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        println!("Double: {}", string);
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                        string.push(c);
                    } else {
                        string.push(c);
                        println!("String: {}", string);
                        result.push(Token::create_token(Type::Strings, string.clone()));
                        string.clear();
                    }
                }
            },
            '.' => match string.is_empty() {
                true => variable.push(c),
                false => string.push(c),
            },
            ' ' => match string.is_empty() {
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        println!("Double: {}", string);
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                    } else {
                        string.push(c);
                    }
                }
                true => match variable.is_empty() {
                    false => {
                        println!("Variable: {}", variable);
                        result.push(Token::create_token(Type::Variable, variable.clone()));
                        variable.clear();
                    }
                    true => continue,
                },
            },
            y => match string.is_empty() {
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        println!("Double: {}", string);
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                        variable.push(y);
                    } else {
                        string.push(y);
                    }
                }
                true => variable.push(c),
            },
        }
    }
    if !string.is_empty() {
        if let Ok(_test) = string.parse::<f64>() {
            println!("Double: {}", string);
            result.push(Token::create_token(Type::Double, string.clone()));
            string.clear();
        } else {
            println!(
                "The string is not empty ! content of the string: {}",
                string
            );
            string.clear()
        }
    }
    if !variable.is_empty() {
        println!("Variable: {}", variable);
        result.push(Token::create_token(Type::Variable, variable.clone()));
        variable.clear();
    }
    result
}
