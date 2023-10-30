use crate::grammar::context::*;
use crate::tokenize::token::*;

fn get_value(variables: &Vec<Context>, variable: String) -> f64 {
    match variables.iter().position(|x| x.variable == variable) {
        Some(index) => {
            return variables[index].value;
        }
        None => {
            panic!("Error ! Variable {} not assignated !", variable);
        }
    }
}

pub fn f(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.first() {
        None => {
            panic!("Error !");
        }
        Some(item) => match item.content.as_str() {
            "(" => match exp.iter().position(|x| x.content == ")") {
                Some(index) => {
                    let term: Vec<Token> = Vec::from_iter(exp[1..index].iter().cloned());
                    return e(term, context);
                }
                None => {
                    panic!("Error in F ! No ')' found to close the '(' !");
                }
            },
            _ => match item.info {
                Type::Double => return item.content.parse::<f64>().unwrap(),
                Type::Variable => return get_value(context, item.content.clone()),
                _ => {
                    panic!("Error of type in F!");
                }
            },
        },
    }
}

pub fn g(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    let mut tmp: Vec<Token> = exp.clone();
    tmp.remove(0);
    return t(tmp, context);
}

pub fn t(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.iter().position(|x| x.content == "*") {
        Some(index) => {
            let (tmp_1, tmp_2) = exp.split_at(index);
            let tmp_1 = tmp_1.to_vec();
            let tmp_2 = tmp_2.to_vec();
            return f(tmp_1, context) * g(tmp_2, context);
        }
        None => return f(exp, context),
    }
}

pub fn d(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    let mut tmp: Vec<Token> = exp.clone();
    tmp.remove(0);
    return e(tmp, context);
}

pub fn e(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.iter().position(|x| x.content == "+") {
        Some(index) => {
            let (tmp_1, tmp_2) = exp.split_at(index);
            let tmp_1 = tmp_1.to_vec();
            let tmp_2 = tmp_2.to_vec();
            return t(tmp_1, context) + d(tmp_2, context);
        }
        None => return t(exp, context),
    }
}
