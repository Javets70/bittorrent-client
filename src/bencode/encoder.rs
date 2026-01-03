use super::value::BencodeValue;
use std::collections::HashMap;

pub fn encode(input: &BencodeValue) -> Vec<u8> {
    match input {
        BencodeValue::Integer(i) => encode_integer(*i),
        BencodeValue::String(s) => encode_string(s),
        BencodeValue::Bytes(b) => encode_bytes(b),
        BencodeValue::List(l) => encode_list(l),
        BencodeValue::Dictionary(d) => encode_dict(d),
    }
}

pub fn encode_integer(n: i64) -> Vec<u8> {
    format!("i{}e", n).into_bytes()
}

pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut result = format!("{}:", bytes.len()).into_bytes();
    result.extend_from_slice(bytes);
    result
}

pub fn encode_string(s: &str) -> Vec<u8> {
    encode_bytes(s.as_bytes())
}

pub fn encode_list(l: &[BencodeValue]) -> Vec<u8> {
    let mut result = b"l".to_vec();

    for item in l {
        result.extend(encode(item))
    }

    result.push(b'e');
    result
}

pub fn encode_dict(dict: &HashMap<String, BencodeValue>) -> Vec<u8> {
    let mut result = b"d".to_vec();

    let mut keys: Vec<_> = dict.keys().collect();
    keys.sort();

    for key in keys {
        result.extend(encode_bytes(key.as_bytes()));
        result.extend(encode(dict.get(key).unwrap()));
    }

    result.push(b'e');
    result
}
