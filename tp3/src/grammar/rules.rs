use crate::grammar::context::*;
use crate::tokenize::token::*;

// Give the value of a variable, stocked in the context.
fn get_value(context: &Vec<Context>, variable: String) -> f64 {
    match context.iter().position(|x| x.variable == variable) {
        Some(index) => {
            return context[index].value;
        }
        None => {
            panic!("Error ! Variable {} not assignated !", variable);
        }
    }
}

// Return the value of a variable or a number or an expression given into '()'
// F -> id
// F -> nb
// F -> (E)
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

// Rule:
// G -> *T
pub fn g(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    let mut tmp: Vec<Token> = exp.clone();
    tmp.remove(0);
    return t(tmp, context);
}

// Rules:
// T -> F G
// G -> epsilon (We don't call G if we don't find a '*')
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

// Rule:
// D -> + E
pub fn d(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    let mut tmp: Vec<Token> = exp.clone();
    tmp.remove(0);
    return e(tmp, context);
}

// Rules:
// E -> T D
// D -> epsilon (We don't call D if we don't find a '+')
pub fn e(mut exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.first().unwrap().content.as_str() {
        "(" => {
            exp.reverse();
            match exp.iter().position(|x| x.content == ")") {
                Some(index) => {
                    let (tmp_2, tmp_1) = exp.split_at(index);
                    let mut tmp_1 = tmp_1.to_vec();
                    let mut tmp_2 = tmp_2.to_vec();
                    tmp_1.reverse();
                    tmp_2.reverse();
                    tmp_1.remove(0);
                    tmp_1.pop();
                    let tmp = e(tmp_1, context);
                    let mut newexp: Vec<Token> = Vec::new();
                    newexp.push(Token::create_token(Type::Double, tmp.to_string()));
                    newexp.append(&mut tmp_2);
                    return e(newexp, context);
                }
                None => {
                    panic!("Error in F ! No ')' found to close the '(' !");
                }
            };
        }

        _ => (),
    }

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

// Rules:
// PD_AFF -> E
// PD_AFF -> inv E
// PD_AFF -> racine E
pub fn pd_aff(exp: Vec<Token>, context: &Vec<Context>) -> f64 {
    match exp.first().unwrap().content.as_str() {
        "inv" => {
            let mut result: Vec<Token> = exp.clone();
            result.remove(0);
            return 1.0 / (e(result, context));
        }
        "racine" => {
            let mut result: Vec<Token> = exp.clone();
            result.remove(0);
            return f64::sqrt(e(result, context));
        }
        _ => return e(exp, context),
    }
}

// Rules:
// INSTR -> afficher E;
// INSTR -> aff_ral;
// INSTR -> id = PD_AFF;
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
                    print!("\n");
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
                        _ => {
                            panic!("Error ! We must have a variable, or 'afficher', or 'aff_ral' in instr() function !");
                        }
                    };
                }
            }
        }
        _ => {
            panic!("Error ! No ';' at the end of the instruction !");
        }
    }
}

// Rules:
// LISTINSTR -> INSTR LISTINSTR
// LISTINSTR -> epsilon
pub fn listinstr(exp: Vec<Token>, context: Vec<Context>) -> Vec<Context> {
    println!("List_instr: {:?}", exp);
    match exp.first() {
        None => return context,
        Some(item) => match item.content.as_str() {
            "boucle" => {
                let mut tmp: Vec<Token> = exp.clone();
                tmp.remove(0);
                match tmp[0].info {
                    Type::Double => {
                        if let Ok(iter) = tmp[0].content.parse::<i32>() {
                            tmp.remove(0);
                            match tmp[0].content.as_str() {
                                "{" => {
                                    tmp.remove(0);
                                    tmp.reverse();
                                    match tmp.iter().position(|x| x.content == "}") {
                                        None => panic!("No end bracket found !"),
                                        Some(index) => {
                                            let (tmp_2, tmp_1) = tmp.split_at(index);
                                            let mut tmp_1 = tmp_1.to_vec();
                                            let mut tmp_2 = tmp_2.to_vec();
                                            tmp_1.remove(0);
                                            tmp_1.reverse();
                                            tmp_2.reverse();
                                            let mut newcontext = context.clone();
                                            for _i in 0..iter {
                                                newcontext = listinstr(tmp_1.clone(), newcontext);
                                            }
                                            return listinstr(tmp_2, newcontext);
                                        }
                                    }
                                }
                                _ => panic!("Error ! bad syntax for the 'boucle'"),
                            }
                        } else {
                            panic!("The number given in boucle is not an integer !");
                        }
                    }
                    _ => {
                        panic!("Error ! boucle must be precedeed by a number !");
                    }
                }
            }
            _ => (),
        },
    }

    match exp.iter().position(|x| x.content == ";") {
        None => match exp.is_empty() {
            true => return context,
            false => panic!(
                "Error ! No ';' at the end of the instruction {} !",
                exp[0].content
            ),
        },
        Some(index) => {
            let (var_1, var_2) = exp.split_at(index + 1);
            let var_1: Vec<Token> = var_1.to_vec();
            let var_2: Vec<Token> = var_2.to_vec().clone();
            let newcontext: Vec<Context> = instr(var_1, context);
            listinstr(var_2, newcontext)
        }
    }
}

// Rule:
// SCRIPT -> LISTINSTR
pub fn script(exp: Vec<Token>) {
    let context: Vec<Context> = Vec::new();
    listinstr(exp, context);
}
