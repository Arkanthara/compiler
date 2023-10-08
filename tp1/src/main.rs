mod postfix;
mod tokenize;

use crate::postfix::topostfix::*;
use crate::tokenize::token::*;
use crate::tokenize::tokenize::*;

fn main() {
    let stack: Vec<Token> = tokenize("((A*B)/D-F)/(G+H)".to_string());
    //    println!("My vect: {:?}", stack);
    let post: Vec<Token> = infix_to_postfix(stack);
    println!("postfix: {:?}", post);
}
