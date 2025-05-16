/*
 * Tackler-NG 2024
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

//
// This code is based on: https://github.com/tailhook/serde-regex,
// which is licensed as Apache-2.0 OR MIT
//

use regex::Regex;
use std::{
    borrow::Cow,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::regex::{new_full_haystack_regex, peeled_pattern};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

/// A wrapper type which implements `Serialize` and `Deserialize` for
/// types involving `Regex`
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Serde<T>(pub T);

impl<'de> Deserialize<'de> for Serde<Regex> {
    fn deserialize<D>(d: D) -> Result<Serde<Regex>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <Cow<'_, str>>::deserialize(d)?;

        match new_full_haystack_regex(s.as_ref()) {
            Ok(regex) => Ok(Serde(regex)),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

/// Deserialize function, see crate docs to see how to use it
/// # Errors
/// Will return `Err` in case of error
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    Serde<T>: Deserialize<'de>,
{
    Serde::deserialize(deserializer).map(|x| x.0)
}

/// Serialize function, see crate docs to see how to use it
/// # Errors
/// Will return `Err` in case of error
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    for<'a> Serde<&'a T>: Serialize,
{
    Serde(value).serialize(serializer)
}

impl<T> Deref for Serde<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Serde<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Serde<T> {
    /// Consumes the `Serde`, returning the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Serde<T> {
    fn from(val: T) -> Serde<T> {
        Serde(val)
    }
}

impl Serialize for Serde<&Regex> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        peeled_pattern(self.0).serialize(serializer)
    }
}

impl Serialize for Serde<Regex> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        peeled_pattern(&self.0).serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::regex::into_full_haystack_pattern;
    use regex::Regex;
    use serde_json::{from_str, to_string};

    const SAMPLE: &str = r#"[a-z"\]]+\d{1,10}""#;
    const SAMPLE_JSON: &str = r#""[a-z\"\\]]+\\d{1,10}\"""#;

    #[test]
    fn test_regex() {
        let re: Serde<Regex> = from_str(SAMPLE_JSON).unwrap();

        assert_eq!(re.as_str(), into_full_haystack_pattern(SAMPLE));
        assert_eq!(to_string(&re).unwrap(), SAMPLE_JSON);
    }
}
