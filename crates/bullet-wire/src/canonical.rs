use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Deserializer, Serialize, de, de::DeserializeOwned};
use serde_json::{Number, Value};
use unicode_normalization::UnicodeNormalization;

use crate::{WireError, canonical_json};

pub const MAX_CANONICAL_DOCUMENT_BYTES: usize = 1_048_576;
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

pub fn decode_canonical<T>(bytes: &[u8]) -> Result<T, WireError>
where
    T: DeserializeOwned + Serialize,
{
    let value = decode_canonical_value(bytes)?;
    serde_json::from_value(value).map_err(|error| {
        WireError::new(
            "DOCUMENT_SCHEMA_INVALID",
            format!("canonical document does not match its strict type: {error}"),
        )
    })
}

pub fn decode_canonical_value(bytes: &[u8]) -> Result<Value, WireError> {
    validate_input_size(bytes)?;
    let text = std::str::from_utf8(bytes).map_err(|error| {
        WireError::new(
            "INVALID_UTF8",
            format!("document is not strict UTF-8: {error}"),
        )
    })?;
    if text.starts_with('\u{feff}') {
        return Err(WireError::new(
            "UTF8_BOM_FORBIDDEN",
            "canonical documents do not carry a UTF-8 byte-order mark",
        ));
    }

    let unique = serde_json::from_str::<UniqueValue>(text).map_err(parse_error)?;
    validate_value(&unique.0)?;
    let canonical = canonical_json(&unique.0)?;
    if bytes != canonical {
        return Err(WireError::new(
            "NON_CANONICAL_JSON",
            "input bytes differ from their RFC 8785 encoding",
        ));
    }
    Ok(unique.0)
}

fn validate_input_size(bytes: &[u8]) -> Result<(), WireError> {
    if bytes.is_empty() {
        return Err(WireError::new("EMPTY_DOCUMENT", "document is empty"));
    }
    if bytes.len() > MAX_CANONICAL_DOCUMENT_BYTES {
        return Err(WireError::new(
            "DOCUMENT_TOO_LARGE",
            format!(
                "document is {} bytes; maximum is {MAX_CANONICAL_DOCUMENT_BYTES}",
                bytes.len()
            ),
        ));
    }
    Ok(())
}

fn parse_error(error: serde_json::Error) -> WireError {
    let reason = error.to_string();
    let code = if reason.contains("duplicate object key") {
        "DUPLICATE_JSON_KEY"
    } else {
        "INVALID_JSON"
    };
    WireError::new(code, reason)
}

fn validate_value(value: &Value) -> Result<(), WireError> {
    match value {
        Value::Array(values) => values.iter().try_for_each(validate_value),
        Value::Object(values) => values.iter().try_for_each(|(key, value)| {
            validate_string(key)?;
            validate_value(value)
        }),
        Value::String(value) => validate_string(value),
        Value::Number(number) => validate_number(number),
        Value::Null | Value::Bool(_) => Ok(()),
    }
}

fn validate_number(number: &Number) -> Result<(), WireError> {
    let outside_safe_range = number
        .as_u64()
        .is_some_and(|value| value > MAX_SAFE_INTEGER)
        || number
            .as_i64()
            .is_some_and(|value| value.unsigned_abs() > MAX_SAFE_INTEGER);
    if outside_safe_range {
        return Err(WireError::new(
            "UNSAFE_JSON_INTEGER",
            "integer is outside the interoperable IEEE-754 safe range",
        ));
    }
    Ok(())
}

fn validate_string(value: &str) -> Result<(), WireError> {
    for character in value.chars() {
        let codepoint = character as u32;
        if character.is_control() && !matches!(character, '\t' | '\n' | '\r') {
            return Err(WireError::new(
                "CONTROL_CHARACTER_FORBIDDEN",
                format!("string contains control character U+{codepoint:04X}"),
            ));
        }
        if is_directional_control(character) {
            return Err(WireError::new(
                "DIRECTIONAL_CONTROL_FORBIDDEN",
                format!("string contains directional control U+{codepoint:04X}"),
            ));
        }
        if matches!(character, '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{2060}') {
            return Err(WireError::new(
                "ZERO_WIDTH_CHARACTER_FORBIDDEN",
                format!("string contains zero-width character U+{codepoint:04X}"),
            ));
        }
        if is_noncharacter(codepoint) {
            return Err(WireError::new(
                "UNICODE_NONCHARACTER_FORBIDDEN",
                format!("string contains Unicode noncharacter U+{codepoint:04X}"),
            ));
        }
    }
    if !value.nfc().eq(value.chars()) {
        return Err(WireError::new(
            "NON_NFC_STRING",
            "string is not Unicode NFC",
        ));
    }
    Ok(())
}

fn is_directional_control(character: char) -> bool {
    matches!(
        character,
        '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
    )
}

fn is_noncharacter(codepoint: u32) -> bool {
    (0xfdd0..=0xfdef).contains(&codepoint) || codepoint & 0xffff >= 0xfffe
}

struct UniqueValue(Value);

impl<'de> Deserialize<'de> for UniqueValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UniqueValueVisitor)
    }
}

struct UniqueValueVisitor;

impl<'de> de::Visitor<'de> for UniqueValueVisitor {
    type Value = UniqueValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value without duplicate object keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .map(UniqueValue)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::String(value.to_owned())))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(UniqueValue(Value::Null))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<UniqueValue>()? {
            values.push(value.0);
        }
        Ok(UniqueValue(Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(de::Error::custom(format!("duplicate object key {key:?}")));
            }
            values.insert(key, map.next_value::<UniqueValue>()?.0);
        }
        Ok(UniqueValue(Value::Object(values.into_iter().collect())))
    }
}
