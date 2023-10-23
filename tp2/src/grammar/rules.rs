use crate::tokenize::token::*;

pub fn f(exp: Vec<Token>) -> i32 {
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
                Type::Double => return 0,
                _ => println!("TODO"),
            },
        },
    }
    return 0;
}

pub fn e(exp: Vec<Token>) -> i32 {
    match exp.iter().position(|x| x.content == "+") {
        Some(index) => {
            let (term, d) = exp.split_at(index);
        }
        None => println!("TODO"),
    }
    return 0;
}
