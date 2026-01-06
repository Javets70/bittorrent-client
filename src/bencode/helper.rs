use super::errors::BencodeError;
use super::value::BencodeValue;
use std::collections::HashMap;

pub fn get_int(dict: &HashMap<String, BencodeValue>, key: &str) -> Result<i64, BencodeError> {
    let value = dict
        .get(key)
        .ok_or(BencodeError::MissingKey(key.to_string()))?;
    match value {
        BencodeValue::Integer(i) => Ok(*i),
        _ => Err(BencodeError::WrongType {
            expected: "Integer".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

pub fn get_string(dict: &HashMap<String, BencodeValue>, key: &str) -> Result<String, BencodeError> {
    let value = dict
        .get(key)
        .ok_or(BencodeError::MissingKey(key.to_string()))?;
    match value {
        BencodeValue::String(s) => Ok(s.clone()),
        _ => Err(BencodeError::WrongType {
            expected: "String".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

pub fn get_bytes<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a Vec<u8>, BencodeError> {
    let value = dict
        .get(key)
        .ok_or(BencodeError::MissingKey(key.to_string()))?;
    match value {
        BencodeValue::Bytes(b) => Ok(b),
        _ => Err(BencodeError::WrongType {
            expected: "Bytes".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

pub fn get_list<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a Vec<BencodeValue>, BencodeError> {
    let value = dict
        .get(key)
        .ok_or(BencodeError::MissingKey(key.to_string()))?;
    match value {
        BencodeValue::List(l) => Ok(l),
        _ => Err(BencodeError::WrongType {
            expected: "List".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

pub fn get_dict<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a HashMap<String, BencodeValue>, BencodeError> {
    let value = dict
        .get(key)
        .ok_or(BencodeError::MissingKey(key.to_string()))?;
    match value {
        BencodeValue::Dictionary(d) => Ok(d),
        _ => Err(BencodeError::WrongType {
            expected: "Dict".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}
