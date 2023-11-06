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

pub fn pd_aff(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.first().unwrap().content.as_str() {
        "inv" => {
            let mut result: Vec<Token> = exp.clone();
            result.remove(0);
            return -(e(result, context));
        }
        _ => return e(exp, context),
    }
}

pub fn instr(exp: Vec<Token>, context: Vec<Context>) -> Vec<Context> {
    if exp.is_empty() {
        return context;
    }
    match exp.last().unwrap().content.as_str() {
        ";" => {
            let mut newexp: Vec<Token> = exp.clone();
            newexp.pop();
            match newexp.first().unwrap().content.as_str() {
                "afficher" => {
                    newexp.remove(0);
                    let result = e(newexp, &context);
                    print!("{}", result);
                    return context;
                }
                "aff_ral" => {
                    println!("");
                    return context;
                }
                _ => {
                    let item: Token = newexp.first().unwrap().clone();
                    match item.info {
                            Type::Variable => {
                            let name: String = item.content;
                            newexp.remove(0);
                            if newexp.first().unwrap().content.as_str() != "=" {
                                panic!("Error: we don't have the '=' for a variable assignation");
                            }
                            newexp.remove(0);
                            let val: f64 = pd_aff(newexp, &context);
                            let mut newcontext: Vec<Context> = context.clone();
                            newcontext.push(Context {
                                variable: name,
                                value: val,
                            });
                            return newcontext;
                    }
                    _ => panic!("Error ! We must have a variable, or 'afficher', or 'aff_ral' in instr() function !"),
                };
                }
            }
        }
        _ => panic!("Error ! No ';' at the end of the instruction !"),
    }
}

pub fn listinstr(exp: Vec<Token>, context: Vec<Context>) {
    match exp.iter().position(|x| x.content == ";") {
        None => return,
        Some(index) => {
            let (var_1, var_2) = exp.split_at(index + 1);
            let var_1: Vec<Token> = var_1.to_vec();
            let var_2: Vec<Token> = var_2.to_vec().clone();
            let newcontext: Vec<Context> = instr(var_1, context);
            listinstr(var_2, newcontext);
        }
    }
}

pub fn script(exp: Vec<Token>) {
    let context: Vec<Context> = Vec::new();
    return listinstr(exp, context);
}
