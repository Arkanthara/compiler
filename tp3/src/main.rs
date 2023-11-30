mod grammar;
mod postfix;
mod tokenize;

use crate::grammar::rules::*;
use crate::postfix::eval::eval_postfix;
use crate::postfix::topostfix::infix_to_postfix;
use crate::tokenize::token::*;
use crate::tokenize::tokenize::tokenize;

fn main() {
    let stack_0: Vec<Token> = tokenize(
        "((2. + 2. * 8.) / 2. + 2.0 * 5. + 1.) / (2.0 + 1. * 2.0)".to_string(),
        true,
    );
    let post: Vec<Token> = infix_to_postfix(stack_0);
    let eval: Option<f64> = eval_postfix(post);
    match eval {
        None => println!("Error !"),
        Some(value) => {
            println!("The result of postfix evaluation is: {}", value);
            let result: f64 = ((2. + 2. * 8.) / 2. + 2.0 * 5. + 1.) / (2.0 + 1. * 2.0);
            assert_eq!(value, result);
        }
    }

    println!("I make the same thing with the grammar rules. So if the results are differents, an error occur !");
    println!("Result of the grammar rules:");
    let stack: Vec<Token> = tokenize("var_1 = inv 2.; var_2 = inv (2. + 1. * 2.); afficher ((2. + 2. * 8.) * var_1 + 2.0 * 5. + 1.) * var_2;aff_ral;".to_string(), false);
    script(stack);
}
