mod tokenize;

use crate::tokenize::token::*;
use crate::tokenize::tokenize::*;

fn main() {
    let stack: Vec<Token> = tokenize("3.43+76-3couc+*o.u|salut*234|3453.43".to_string());
    println!("My vect: {:?}", stack);
}
