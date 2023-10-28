use crate::tokenize::token::*;

#[derive(Debug)]
pub enum Item {
    Nb(f64),
    Id(String),
    Empty,
}

pub fn f(exp: Vec<Token>) -> Item {
    match exp.first() {
        None => println!("Error !"),
        Some(item) => match item.content.as_str() {
            "(" => match exp.iter().position(|x| x.content == ")") {
                Some(index) => {
                    let term: Vec<Token> = Vec::from_iter(exp[1..index].iter().cloned());
                    return e(term);
                }
                None => println!("Error in f !"),
            },
            _ => match item.info {
                Type::Double => return Item::Nb(item.content.parse::<f64>().unwrap()),
                Type::Variable => return Item::Id(item.content.clone()),
                _ => println!("Error of type !"),
            },
        },
    }
    return Item::Empty;
}

pub fn g(exp: Vec<Token>) -> Item {
    match exp.first() {
        None => println!("Error in G !"),
        Some(item) => {
            let mut result: Vec<Token> = exp.clone();
            result.remove(0);
            return t(result);
        }
    }
    return Item::Empty;
}
// TODO: verify that Items aren't variables before make operations...
pub fn t(exp: Vec<Token>) -> Item {
    match exp.first() {
        None => println!("Error !"),
        Some(item) => match item.content.as_str() {
            "(" => match exp.iter().position(|x| x.content == ")") {
                Some(index) => {
                    let (tmp_1, tmp_2) = exp.split_at(index + 1);
                    let tmp_1 = tmp_1.to_vec();
                    let tmp_2 = tmp_2.to_vec();
                    let result: Item = f(tmp_1);
                    let var_2: Item = g(tmp_2);
                    match var_2 {
                        Item::Empty => return result,
                        Item::Nb(nb) => {
                            match result {
                                Item::Nb(number) => return Item::Nb(number * nb),
                                // TODO here
                                _ => return Item::Empty,
                            }
                        }
                        Item::Id(variable) => {
                            println!("A variable here in t: {}", variable);
                            return Item::Empty;
                        }
                    }
                }
                None => {
                    println!("Error in t: no ')' for end the (e)...");
                    return Item::Empty;
                }
            },
            _ => match item.info {
                Type::Double => {
                    let mut result: Vec<Token> = exp.clone();
                    result.remove(0);
                    let var: Item = g(result);
                    match var {
                        Item::Empty => return Item::Nb(item.content.parse::<f64>().unwrap()),
                        Item::Nb(number) => {
                            return Item::Nb(item.content.parse::<f64>().unwrap() * number)
                        }
                        Item::Id(id) =>
                        //TODO
                        {
                            println!("TODO");
                            return Item::Empty;
                        }
                    }
                }
                Type::Variable => {
                    // TODO
                    println!("TODO");
                }
                _ => println!(
                    "Error in t because {} is not a double or a variable...",
                    item.content
                ),
            },
        },
    }
    return Item::Empty;
}

pub fn e(exp: Vec<Token>) -> Item {
    match exp.iter().position(|x| x.content == "+") {
        Some(index) => {
            let (term, d) = exp.split_at(index);
        }
        None => println!("TODO"),
    }
    return Item::Empty;
}
