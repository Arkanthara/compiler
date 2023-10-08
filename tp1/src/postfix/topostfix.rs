use crate::tokenize::token::*;

fn give_priority(item: &String) -> i32 {
    match item.as_str() {
        "(" | ")" => {
            return 0;
        }
        "+" | "-" => {
            return 1;
        }
        "*" | "/" => {
            return 2;
        }
        "^" => {
            return 3;
        }
        _ => {
            println!("Error !");
            return -1;
        }
    }
}

pub fn infix_to_postfix(mut input: Vec<Token>) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut stack: Vec<String> = Vec::new();
    let mut priority = 0;
    input.reverse();
    while !input.is_empty() {
        let item: Token = input
            .pop()
            .unwrap_or(Token::create_token(Type::Strings, " ".to_owned()));
        match item.info {
            Type::ReservedSymbol => {
                if item.content.as_str() == "(" {
                    priority = give_priority(&item.content);
                    stack.push(item.content);
                } else if item.content.as_str() == ")" {
                    while !(stack
                        .last()
                        .unwrap_or(&"Error! None value".to_string())
                        .as_str()
                        == "(")
                        && !stack.is_empty()
                    {
                        result.push(Token::create_token(
                            Type::ReservedSymbol,
                            stack.pop().unwrap_or("Error! None value".to_string()),
                        ));
                    }
                    stack.pop().unwrap_or("Error! 3".to_owned());
                    if !stack.is_empty() {
                        priority = give_priority(&stack.last().unwrap_or(&"Error! 4".to_owned()));
                    }
                } else if give_priority(&item.content) > priority {
                    priority = give_priority(&item.content);
                    stack.push(item.content);
                } else {
                    while give_priority(&item.content) <= priority && !stack.is_empty() {
                        result.push(Token::create_token(
                            Type::ReservedSymbol,
                            stack.pop().unwrap_or("Error! 5".to_owned()),
                        ));
                        if !stack.is_empty() {
                            priority =
                                give_priority(&stack.last().unwrap_or(&"Error! 6".to_owned()));
                        }
                    }
                    priority = give_priority(&item.content);
                    stack.push(item.content);
                }
            }
            _ => {
                result.push(item);
            }
        }
    }
    while !stack.is_empty() {
        result.push(Token::create_token(
            Type::ReservedSymbol,
            stack.pop().unwrap(),
        ));
    }
    result
}
