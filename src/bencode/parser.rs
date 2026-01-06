use super::errors::BencodeError;
use super::value::BencodeValue;

pub fn parse_value(input: &[u8]) -> Result<(BencodeValue, &[u8]), BencodeError> {
    if input.is_empty() {
        return Err(BencodeError::UnexpectedEof);
    }

    match input[0] {
        b'i' => parse_int(input),
        b'l' => parse_list(input),
        b'd' => parse_dict(input),
        b'0'..=b'9' => parse_string(input),
        _ => Err(BencodeError::WrongType {
            expected: "String/List/Integer/Dictionary".into(),
            found: format!("Unknown byte: {}", input[0]),
        }),
    }
}
// Integers are represented by an 'i' followed by the number in base 10 followed by an 'e'.
// For example i3e corresponds to 3 and i-3e corresponds to -3.
// Integers have no size limitation. i-0e is invalid.
// All encodings with a leading zero, such as i03e,
// are invalid, other than i0e, which of course corresponds to 0.
pub fn parse_int(input: &[u8]) -> Result<(BencodeValue, &[u8]), BencodeError> {
    if !input.starts_with(b"i") {
        return Err(BencodeError::InvalidInteger("Missing 'i'".to_string()));
    }

    let end = input
        .iter()
        .position(|&b| b == b'e')
        .ok_or_else(|| BencodeError::InvalidInteger("Missing 'e'".into()))?;

    let num_str = std::str::from_utf8(&input[1..end])
        .map_err(|_| BencodeError::InvalidInteger("Invalid UTF-8 in number".into()))?;

    if (num_str.starts_with("0") && num_str.len() > 1) || num_str.starts_with("-0") {
        return Err(BencodeError::InvalidInteger(
            "Integer starts with 0 or -0".to_string(),
        ));
    }

    let value = num_str
        .parse::<i64>()
        .map_err(|_| BencodeError::InvalidInteger(format!("Cannot parse: {}", num_str)))?;

    Ok((BencodeValue::Integer(value), &input[end + 1..]))
}

// Strings: Strings are length-prefixed base ten followed by a colon and the string.
// For example 4:spam corresponds to 'spam'.
pub fn parse_string(input: &[u8]) -> Result<(BencodeValue, &[u8]), BencodeError> {
    let colon_pos = input
        .iter()
        .position(|&b| b == b':')
        .ok_or_else(|| BencodeError::InvalidString("Missing ':'".into()))?;

    let len_str = std::str::from_utf8(&input[..colon_pos])
        .map_err(|_| BencodeError::InvalidInteger("Invalid UTF-8 in number".into()))?;
    let len = len_str
        .parse::<usize>()
        .map_err(|_| BencodeError::InvalidInteger(format!("Cannot parse: {}", len_str)))?;

    let start = colon_pos + 1;
    let end = start + len;

    if input.len() < end {
        return Err(BencodeError::InvalidString(
            "String length exceeds input length".into(),
        ));
    }

    let bytes = &input[start..end];

    let value = match std::str::from_utf8(bytes) {
        Ok(s) => BencodeValue::String(s.to_string()),
        Err(_) => BencodeValue::Bytes(bytes.to_vec()),
    };
    let rest = &input[end..];

    Ok((value, rest))
}

// Lists: Lists are encoded as an 'l' followed by their elements (also bencoded) followed by an 'e'.
// For example l4:spam4:eggse corresponds to ['spam', 'eggs'].
pub fn parse_list(input: &[u8]) -> Result<(BencodeValue, &[u8]), BencodeError> {
    if !input.starts_with(b"l") {
        return Err(BencodeError::InvalidList(format!(
            "Input does not start with 'l': {}",
            input[0]
        )));
    }

    let mut values = Vec::new();
    let mut rest = &input[1..];

    while !rest.is_empty() && !rest.starts_with(b"e") {
        let (value, remaining) = parse_value(rest)?;
        values.push(value);
        rest = remaining;
    }

    if !rest.starts_with(b"e") {
        return Err(BencodeError::InvalidList("Missing ending 'e'".into()));
    }

    Ok((BencodeValue::List(values), &rest[1..]))
}

// Dictionaries are encoded as a 'd' followed by a list of alternating
// keys and their corresponding values followed by an 'e'.
// For example, d3:cow3:moo4:spam4:eggse corresponds to
// {'cow': 'moo', 'spam': 'eggs'} and d4:spaml1:a1:bee corresponds to
// {'spam': ['a', 'b']}.
// Keys must be strings and appear in sorted order (sorted as raw strings, not alphanumerics).
pub fn parse_dict(input: &[u8]) -> Result<(BencodeValue, &[u8]), BencodeError> {
    if !input.starts_with(b"d") {
        return Err(BencodeError::InvalidDict(format!(
            "Input does not start with 'd': {}",
            input[0]
        )));
    }

    let mut dict = std::collections::HashMap::new();
    let mut rest = &input[1..];

    while !rest.is_empty() && !rest.starts_with(b"e") {
        let (key, remaining) = parse_string(rest)?;
        rest = remaining;
        let key_str = key.as_string()?;

        let (value, remaining) = parse_value(rest)?;
        dict.insert(key_str.to_string(), value);
        rest = remaining;
    }

    if !rest.starts_with(b"e") {
        return Err(BencodeError::InvalidDict("Missing ending 'e'".into()));
    }

    Ok((BencodeValue::Dictionary(dict), &rest[1..]))
}
