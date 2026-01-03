use super::value::BencodeValue;
use std::collections::HashMap;
use std::error::Error;

pub fn get_int(dict: &HashMap<String, BencodeValue>, key: &str) -> Result<i64, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::Integer(i) => Ok(*i),
        _ => Err(format!("'{}' must be an integer", key).into()),
    }
}

pub fn get_string<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::String(s) => Ok(s.clone()),
        _ => Err(format!("'{}' must be a string", key).into()),
    }
}

pub fn get_bytes<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a Vec<u8>, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::Bytes(b) => Ok(b),
        _ => Err(format!("'{}' must be a vector of bytes", key).into()),
    }
}

pub fn get_list<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a Vec<BencodeValue>, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::List(l) => Ok(l),
        _ => Err(format!("'{}' must be a list", key).into()),
    }
}

pub fn get_dict<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a HashMap<String, BencodeValue>, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::Dictionary(d) => Ok(d),
        _ => Err(format!("'{}' must be a dictionary", key).into()),
    }
}
