use std::fmt;

#[derive(Debug)]
pub enum BencodeError {
    InvalidInteger(String),
    InvalidString(String),
    InvalidList(String),
    InvalidDict(String),
    UnexpectedEof,
    MissingKey(String),
    WrongType { expected: String, found: String },
}

impl std::fmt::Display for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BencodeError::UnexpectedEof => write!(f, "Unexpected EOF"),
            BencodeError::InvalidInteger(msg) => write!(f, "Invalid Integer: {}", msg),
            BencodeError::InvalidString(msg) => write!(f, "Invalid String: {}", msg),
            BencodeError::InvalidList(msg) => write!(f, "Invalid List: {}", msg),
            BencodeError::InvalidDict(msg) => write!(f, "Invalid Dict: {}", msg),
            BencodeError::MissingKey(msg) => write!(f, "Missing Key: {}", msg),
            BencodeError::WrongType { expected, found } => {
                write!(f, "Wrong type, \nExpected:{} Found:{}", expected, found)
            }
        }
    }
}
impl std::error::Error for BencodeError {}
