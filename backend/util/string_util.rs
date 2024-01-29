use anyhow::Result;
use serde::Serialize;
use serde_json::to_string;

/**
* Converts an enum to a string, removing the first and last characters
* which are the enclosing quotes.
*/
pub fn enum_to_string<T: Sized + Serialize>(e: T) -> Result<String> {
    let string = to_string(&e)?;
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    Ok(chars.as_str().to_string())
}
