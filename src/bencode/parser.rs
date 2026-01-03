use super::errors::BencodeError;
use super::value::BencodeValue;
pub struct BencodeParser;

impl BencodeParser {
    pub fn parse_value(encoded_str: &str) -> Result<(BencodeValue, &str), BencodeError> {
        if encoded_str.is_empty() {
            // return Err("Unexpected end of input".into());
            return Err(BencodeError::UnexpectedEof);
        }

        match encoded_str.chars().next().unwrap() {
            'i' => Self::parse_int(encoded_str),
            'l' => Self::parse_list(encoded_str),
            'd' => Self::parse_dict(encoded_str),
            '0'..='9' => Self::parse_string(encoded_str),
            _ => Err(BencodeError::WrongType {
                expected: "String/List/Integer/Dictionary".into(),
                found: format!("Unknown: {encoded_str}").into(),
            }),
        }
    }
    // Integers are represented by an 'i' followed by the number in base 10 followed by an 'e'.
    // For example i3e corresponds to 3 and i-3e corresponds to -3.
    // Integers have no size limitation. i-0e is invalid.
    // All encodings with a leading zero, such as i03e,
    // are invalid, other than i0e, which of course corresponds to 0.
    pub fn parse_int(encoded_str: &str) -> Result<(BencodeValue, &str), BencodeError> {
        if !encoded_str.starts_with("i") {
            return Err(BencodeError::InvalidInteger(
                format!("Input does not start with 'i': {}", encoded_str).into(),
            ));
        }

        let end = encoded_str
            .find("e")
            .ok_or_else(|| BencodeError::InvalidInteger("Input missing ending 'e'".into()))?;
        let num_str = &encoded_str[1..end];
        let value = num_str.parse::<i64>().map_err(|_| {
            BencodeError::InvalidInteger(format!("Error while parsing {num_str} to i64").into())
        })?;

        Ok((BencodeValue::Integer(value), &encoded_str[end + 1..]))
    }

    // Strings: Strings are length-prefixed base ten followed by a colon and the string.
    // For example 4:spam corresponds to 'spam'.
    pub fn parse_string(encoded_str: &str) -> Result<(BencodeValue, &str), BencodeError> {
        let colon_pos = encoded_str
            .find(':')
            .ok_or_else(|| BencodeError::InvalidString("Missing ':'".into()))?;

        let len = encoded_str[..colon_pos].parse::<usize>().map_err(|_| {
            BencodeError::InvalidString(format!(
                "Error while parsing \
                string length: {encoded_str}"
            ))
            .into()
        })?;

        let start = colon_pos + 1;
        let end = start + len;

        if encoded_str.len() < end {
            // return Err("String length exceeds input".into());
            return Err(BencodeError::InvalidString(
                "String length exceeds input length".into(),
            ));
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
    pub fn parse_list(encoded_str: &str) -> Result<(BencodeValue, &str), BencodeError> {
        if !encoded_str.starts_with('l') {
            return Err(BencodeError::InvalidList(format!(
                "Input does not start with 'l': {encoded_str}"
            )));
        }

        let mut values = Vec::new();
        let mut rest = &encoded_str[1..];
        while !rest.is_empty() && !rest.starts_with('e') {
            let (value, remaining) = Self::parse_value(rest)?;
            values.push(value);
            rest = remaining;
        }

        if !rest.starts_with('e') {
            return Err(BencodeError::InvalidList(format!(
                "Missing ending 'e' for input: {encoded_str}"
            )));
        }

        Ok((BencodeValue::List(values), &rest[1..]))
    }

    // Dictionaries are encoded as a 'd' followed by a list of alternating
    // keys and their corresponding values followed by an 'e'.
    // For example, d3:cow3:moo4:spam4:eggse corresponds to
    // {'cow': 'moo', 'spam': 'eggs'} and d4:spaml1:a1:bee corresponds to
    // {'spam': ['a', 'b']}.
    // Keys must be strings and appear in sorted order (sorted as raw strings, not alphanumerics).
    pub fn parse_dict(encoded_str: &str) -> Result<(BencodeValue, &str), BencodeError> {
        if !encoded_str.starts_with('d') {
            return Err(BencodeError::InvalidDict(format!(
                "Input does not start with 'd': {encoded_str}"
            )));
        }

        let mut dict = std::collections::HashMap::new();
        let mut rest = &encoded_str[1..];

        while !rest.is_empty() && !rest.starts_with('e') {
            let (key, remaining) = Self::parse_string(rest)?;
            rest = remaining;
            let key_str = match key {
                BencodeValue::String(val) => val,
                _ => {
                    return Err(BencodeError::InvalidString(
                        "Dictionary key must be String type".into(),
                    ));
                }
            };

            let (value, remaining) = Self::parse_value(rest)?;
            dict.insert(key_str, value);
            rest = remaining;
        }

        if !rest.starts_with('e') {
            // return Err("Missing 'e' at the end of dictionary".into());
            return Err(BencodeError::InvalidDict(format!(
                "Missing ending 'e' for the dictionary: {encoded_str}"
            )));
        }

        Ok((BencodeValue::Dictionary(dict), &rest[1..]))
    }
}
