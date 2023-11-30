use crate::Token;
use crate::Type;

// Function used to parse the input text into a vector of tokens
// We use a vector string to stock all the symbols in "" and then to stock them in the result
// We use a vector variable to stock all the symbols when we have a symbol which is not a Reserved
// Symbol, a Double or a String (Begin by ")... And when we meet something different, we stock the
// contain of variable in a token variable
// We use also string to stock all the digit that we meet, and we stock them if we meet something
// different (If we meet a '.', we add it into the string because we can have float digit... But we
// have a counter to have just one . in a digit...)
// We can decide to print the result or not...
pub fn tokenize(text: String, print_result: bool) -> Vec<Token> {
    let mut counter = 0;
    let mut printer = String::new();
    let mut string = String::new();
    let mut variable = String::new();
    let mut result: Vec<Token> = Vec::new();
    for c in text.chars() {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' | '=' | ';' => match string.is_empty() {
                true => match variable.is_empty() {
                    true => {
                        printer.push_str("Reserved symbol: ");
                        printer.push(c);
                        printer.push_str("\n");
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    }
                    false => {
                        printer.push_str("Variable: ");
                        printer.push_str(variable.as_str());
                        printer.push_str("\n");
                        result.push(Token::create_token(Type::Variable, variable.clone()));
                        variable.clear();
                        printer.push_str("Reserved symbol: ");
                        printer.push(c);
                        printer.push_str(" ");
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    }
                },
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        printer.push_str("Double: ");
                        printer.push_str(string.as_str());
                        printer.push_str("\n");
                        counter = 0;
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                        printer.push_str("Reserved symbol: ");
                        printer.push(c);
                        printer.push_str("\n");
                        result.push(Token::create_token(Type::ReservedSymbol, c.to_string()));
                    } else {
                        string.push(c);
                    }
                }
            },
            x if x.is_digit(10) => match variable.is_empty() {
                true => string.push(c),
                false => variable.push(c),
            },
            '"' => match string.is_empty() {
                true => {
                    match variable.is_empty() {
                        true => string.push(c),
                        false => {
                            printer.push_str("Variable: ");
                            printer.push_str(variable.as_str());
                            printer.push_str("\n");
                            result.push(Token::create_token(Type::Variable, variable.clone()));
                            variable.clear();
                            string.push(c);
                        }
                    }
                    string.push(c);
                }
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        printer.push_str("Double: ");
                        printer.push_str(string.as_str());
                        printer.push_str("\n");
                        counter = 0;
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                        string.push(c);
                    } else {
                        string.push(c);
                        printer.push_str("String: ");
                        printer.push_str(string.as_str());
                        printer.push_str("\n");
                        result.push(Token::create_token(Type::Strings, string.clone()));
                        string.clear();
                    }
                }
            },
            '.' => match string.is_empty() {
                true => variable.push(c),
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        if counter != 0 {
                            panic!("Error ! The digit {} has more than one '.' !", string);
                        }
                        string.push(c);
                        counter = 1;
                    } else {
                        string.push(c);
                    }
                }
            },
            ' ' => match string.is_empty() {
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        printer.push_str("Double: ");
                        printer.push_str(string.as_str());
                        printer.push_str("\n");
                        counter = 0;
                        result.push(Token::create_token(Type::Double, string.clone()));
                        string.clear();
                    } else {
                        string.push(c);
                    }
                }
                true => match variable.is_empty() {
                    false => {
                        printer.push_str("Variable: ");
                        printer.push_str(variable.as_str());
                        printer.push_str("\n");
                        result.push(Token::create_token(Type::Variable, variable.clone()));
                        variable.clear();
                    }
                    true => continue,
                },
            },
            y => match string.is_empty() {
                false => {
                    if let Ok(_test) = string.parse::<f64>() {
                        printer.push_str("Double: ");
                        printer.push_str(string.as_str());
                        printer.push_str("\n");
                        counter = 0;
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
            printer.push_str("Double: ");
            printer.push_str(string.as_str());
            result.push(Token::create_token(Type::Double, string.clone()));
            string.clear();
        } else {
            printer.push_str("The string is not empty ! content of the string: ");
            printer.push_str(string.as_str());
            printer.push_str("\n");
            string.clear()
        }
    }
    if !variable.is_empty() {
        printer.push_str("Variable: ");
        printer.push_str(variable.as_str());
        printer.push_str("\n");
        result.push(Token::create_token(Type::Variable, variable.clone()));
        variable.clear();
    }
    if print_result {
        println!("{}", printer);
    }
    result
}
