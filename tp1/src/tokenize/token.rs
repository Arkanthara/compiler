use std::fmt;

#[derive(Debug)]
pub enum Type {
    Double,
    Strings,
    ReservedSymbol,
    Variable,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Variable => write!(f, "Variable"),
            Type::Double => write!(f, "Double"),
            Type::Strings => write!(f, "String"),
            Type::ReservedSymbol => write!(f, "Reserved Symbol"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub info: Type,
    pub content: String,
}

impl Token {
    pub fn create_token(type_token: Type, content: String) -> Token {
        Token {
            info: type_token,
            content: content.clone(),
        }
    }
}
