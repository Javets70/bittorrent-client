use super::errors::BencodeError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BencodeValue {
    Integer(i64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dictionary(std::collections::HashMap<String, BencodeValue>),
}

impl std::fmt::Display for BencodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BencodeValue::Integer(i) => write!(f, "{}", i),
            BencodeValue::String(s) => write!(f, " \" {} \" ", s),
            BencodeValue::Bytes(b) => write!(f, "{}", b.len()),
            BencodeValue::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            BencodeValue::Dictionary(d) => {
                write!(f, "{{")?;
                for (i, (k, v)) in d.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl BencodeValue {
    pub fn type_name(&self) -> &str {
        match &self {
            BencodeValue::Integer(_) => "Int",
            BencodeValue::String(_) => "String",
            BencodeValue::Bytes(_) => "Bytes",
            BencodeValue::List(_) => "List",
            BencodeValue::Dictionary(_) => "Dict",
        }
    }

    pub fn as_dict(&self) -> Result<&HashMap<String, BencodeValue>, BencodeError> {
        match self {
            BencodeValue::Dictionary(d) => Ok(d),
            _ => Err(BencodeError::WrongType {
                expected: "Dict".into(),
                found: self.type_name().into(),
            }),
        }
    }
    pub fn as_int(&self) -> Result<&i64, BencodeError> {
        match self {
            BencodeValue::Integer(i) => Ok(i),
            _ => Err(BencodeError::WrongType {
                expected: "Integer".into(),
                found: self.type_name().into(),
            }),
        }
    }
    pub fn as_string(&self) -> Result<&str, BencodeError> {
        match self {
            BencodeValue::String(s) => Ok(s),
            _ => Err(BencodeError::WrongType {
                expected: "String".into(),
                found: self.type_name().into(),
            }),
        }
    }
    pub fn as_list(&self) -> Result<&Vec<BencodeValue>, BencodeError> {
        match self {
            BencodeValue::List(l) => Ok(l),
            _ => Err(BencodeError::WrongType {
                expected: "List".into(),
                found: self.type_name().into(),
            }),
        }
    }
    pub fn as_bytes(&self) -> Result<&Vec<u8>, BencodeError> {
        match self {
            BencodeValue::Bytes(b) => Ok(b),
            _ => Err(BencodeError::WrongType {
                expected: "Bytes".into(),
                found: self.type_name().into(),
            }),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        crate::bencode::encoder::encode(self)
    }
}
