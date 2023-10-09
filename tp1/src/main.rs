mod postfix;
mod tokenize;

use crate::postfix::eval::eval_postfix;
use crate::postfix::topostfix::infix_to_postfix;
use crate::tokenize::token::*;
use crate::tokenize::tokenize::tokenize;

fn main() {
    let stack: Vec<Token> = tokenize("((2*2)/2-1)/(2+2)".to_string());
    //    println!("My vect: {:?}", stack);
    let post: Vec<Token> = infix_to_postfix(stack);
    //    println!("postfix: {:?}", post);
    let eval: Option<f64> = eval_postfix(post);
    match eval {
        None => println!("Error !"),
        Some(value) => {
            println!("The result of postfix evaluation is: {}", value);
            let result: f64 = ((2.0 * 2.0) / 2.0 - 1.0) / (2.0 + 2.0);
            assert_eq!(value, result);
        }
    }
}
