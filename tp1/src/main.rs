mod tokenize;

use crate::tokenize::*;

fn main() {
    tokenize("3.43+76-3|couc+*o.u|3453.43".to_string());
}
