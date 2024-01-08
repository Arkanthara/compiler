use crate::tokenize::token::*;

// Allow to evaluate a postfix notation given in input.
pub fn eval_postfix(mut input: Vec<Token>) -> Option<f64> {
    for i in &input {
        match i.info {
            Type::Variable => {
                println!("Error of type");
                return None;
            }
            Type::Strings => {
                println!("Error of type !");
                return None;
            }
            _ => continue,
        }
    }

    let mut stack: Vec<f64> = Vec::new();

    input.reverse();
    while !input.is_empty() {
        let item: Token = input.pop().unwrap();

        match item.info {
            Type::Double => {
                stack.push(item.content.parse::<f64>().unwrap());
            }
            Type::ReservedSymbol => {
                if stack.len() < 2 {
                    println!("Dynamic error: operator {} takes 2 arguments", item.content);
                    return None;
                }
                match item.content.as_str() {
                    "+" => {
                        let var_1 = stack.pop().unwrap();
                        let var_2 = stack.pop().unwrap();
                        stack.push(var_2 + var_1);
                    }
                    "-" => {
                        let var_1 = stack.pop().unwrap();
                        let var_2 = stack.pop().unwrap();
                        stack.push(var_2 - var_1);
                    }
                    "*" => {
                        let var_1 = stack.pop().unwrap();
                        let var_2 = stack.pop().unwrap();
                        stack.push(var_2 * var_1);
                    }
                    "/" => {
                        let var_1 = stack.pop().unwrap();
                        let var_2 = stack.pop().unwrap();
                        stack.push(var_2 / var_1);
                    }
                    "^" => {
                        let var_1 = stack.pop().unwrap();
                        let var_2 = stack.pop().unwrap();
                        stack.push(var_2.powf(var_1));
                    }
                    _ => {
                        println!("Reserved symbol {} not known !", item.content);
                        return None;
                    }
                }
            }
            _ => continue,
        }
    }
    if stack.len() != 1 {
        println!("Error: the stack hasn't a len equal to 1 !");
        return None;
    }
    Some(stack.pop().unwrap())
}
