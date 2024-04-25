use std::fmt::Display;

use serde::{de, ser};

#[derive(Debug)]
pub enum SymbolError {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),
    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`
    Eof,
    ExpectedInteger,
    ExpectedString,
    TrailingCharacters,
}

impl ser::Error for SymbolError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SymbolError::Message(msg.to_string())
    }
}

impl de::Error for SymbolError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SymbolError::Message(msg.to_string())
    }
}

impl Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::Message(msg) => f.write_str(msg),
            SymbolError::Eof => f.write_str("unexpected end of file"),
            SymbolError::ExpectedInteger => f.write_str("expect integer"),
            SymbolError::ExpectedString => f.write_str("expect String"),
            SymbolError::TrailingCharacters => f.write_str("trailing characters"),
        }
    }
}

impl std::error::Error for SymbolError {}
