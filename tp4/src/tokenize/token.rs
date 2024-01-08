use std::fmt;

// Used in Token to indicate what is the type of the token
#[derive(Clone, Debug)]
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

// Structure that indicate what is the string in content... It can be a Double, a Strings, a ReservedSymbol, and a Variable...
#[derive(Clone, Debug)]
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
