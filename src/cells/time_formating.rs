//! Use the `YYYY-MM-DD` format when serializing and deserializing a [`time::Date`].
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! [with]: https://serde.rs/field-attrs.html#with

use crate::cells::Serialize;
use serde::{self, Deserialize, Deserializer, Serializer};
use time::Date;
use time::format_description::FormatItem;
use time::macros::format_description;

// The format description for `YYYY-MM-DD`. This is checked at compile time.
const FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// Serialize a [`Date`] to a `YYYY-MM-DD` string.
pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    date.format(&FORMAT)
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

/// Deserialize a [`Date`] from a `YYYY-MM-DD` string.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Date::parse(&s, &FORMAT).map_err(serde::de::Error::custom)
}
