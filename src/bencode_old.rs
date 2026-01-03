use std::{collections::HashMap, error::Error};

pub enum BencodeValue {
    Int(i64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(std::collections::HashMap<String, BencodeValue>),
}

pub fn get_int(dict: &HashMap<String, BencodeValue>, key: &str) -> Result<i64, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::Int(i) => Ok(*i),
        _ => Err(format!("'{}' must be an integer", key).into()),
    }
}

pub fn get_string<'a>(
    dict: &'a HashMap<String, BencodeValue>,
    key: &str,
) -> Result<&'a str, Box<dyn Error>> {
    let value = dict.get(key).ok_or(format!("Missing key: {}", key))?;
    match value {
        BencodeValue::String(s) => Ok(s),
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
        BencodeValue::Dict(d) => Ok(d),
        _ => Err(format!("'{}' must be a dictionary", key).into()),
    }
}

pub fn encode_value(input: &BencodeValue) -> Vec<u8> {}

impl std::fmt::Display for BencodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BencodeValue::Int(i) => write!(f, "{}", i),
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
            BencodeValue::Dict(d) => {
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

pub fn parse_value(encoded_str: &str) -> Result<(BencodeValue, &str), Box<dyn Error>> {
    if encoded_str.is_empty() {
        return Err("Unexpected end of input".into());
    }

    match encoded_str.chars().next().unwrap() {
        'i' => parse_int(encoded_str),
        'l' => parse_list(encoded_str),
        'd' => parse_dict(encoded_str),
        '0'..='9' => parse_string(encoded_str),
        _ => Err(format!("Unknown bencode type: {}", &encoded_str[..1]).into()),
    }
}

// Integers are represented by an 'i' followed by the number in base 10 followed by an 'e'.
// For example i3e corresponds to 3 and i-3e corresponds to -3.
// Integers have no size limitation. i-0e is invalid.
// All encodings with a leading zero, such as i03e,
// are invalid, other than i0e, which of course corresponds to 0.
pub fn parse_int(encoded_str: &str) -> Result<(BencodeValue, &str), Box<dyn Error>> {
    if !encoded_str.starts_with("i") {
        return Err("Expected integer".into());
    }

    let end = encoded_str.find("e").ok_or("Missing 'e'")?;
    let num_str = &encoded_str[1..end];
    let value = num_str.parse::<i64>()?;

    Ok((BencodeValue::Int(value), &encoded_str[end + 1..]))
}

// Strings: Strings are length-prefixed base ten followed by a colon and the string.
// For example 4:spam corresponds to 'spam'.
pub fn parse_string(encoded_str: &str) -> Result<(BencodeValue, &str), Box<dyn Error>> {
    let colon_pos = encoded_str.find(':').ok_or("Missing ':'")?;

    let len: usize = encoded_str[..colon_pos].parse()?;

    let start = colon_pos + 1;
    let end = start + len;

    if encoded_str.len() < end {
        return Err("String length exceeds input".into());
    }

    let bytes = &encoded_str[start..end].as_bytes();

    let value = match std::str::from_utf8(bytes) {
        Ok(s) => BencodeValue::String(s.to_string()),
        Err(_) => BencodeValue::Bytes(bytes.to_vec()),
    };
    let rest = &encoded_str[end..];

    Ok((value, rest))
}

// Lists: Lists are encoded as an 'l' followed by their elements (also bencoded) followed by an 'e'.
// For example l4:spam4:eggse corresponds to ['spam', 'eggs'].
pub fn parse_list(encoded_str: &str) -> Result<(BencodeValue, &str), Box<dyn Error>> {
    if !encoded_str.starts_with('l') {
        return Err("Invalid list".into());
    }

    let mut values = Vec::new();
    let mut rest = &encoded_str[1..];
    while !rest.is_empty() && !rest.starts_with('e') {
        let (value, remaining) = parse_value(rest)?;
        values.push(value);
        rest = remaining;
    }

    if !rest.starts_with('e') {
        return Err("Missing 'e' at the end of list".into());
    }

    Ok((BencodeValue::List(values), &rest[1..]))
}

// Dictionaries are encoded as a 'd' followed by a list of alternating
// keys and their corresponding values followed by an 'e'.
// For example, d3:cow3:moo4:spam4:eggse corresponds to
// {'cow': 'moo', 'spam': 'eggs'} and d4:spaml1:a1:bee corresponds to
// {'spam': ['a', 'b']}.
// Keys must be strings and appear in sorted order (sorted as raw strings, not alphanumerics).
pub fn parse_dict(encoded_str: &str) -> Result<(BencodeValue, &str), Box<dyn Error>> {
    if !encoded_str.starts_with('d') {
        return Err("Invalid dictionary".into());
    }

    let mut dict = std::collections::HashMap::new();
    let mut rest = &encoded_str[1..];

    while !rest.is_empty() && !rest.starts_with('e') {
        let (key, remaining) = parse_string(rest)?;
        rest = remaining;
        let key_str = match key {
            BencodeValue::String(val) => val,
            _ => return Err("Dictionary key must be string".into()),
        };

        let (value, remaining) = parse_value(rest)?;
        dict.insert(key_str, value);
        rest = remaining;
    }

    if !rest.starts_with('e') {
        return Err("Missing 'e' at the end of dictionary".into());
    }

    Ok((BencodeValue::Dict(dict), &rest[1..]))
}
