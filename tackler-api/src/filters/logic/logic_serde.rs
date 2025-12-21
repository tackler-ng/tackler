/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::filters::TxnFilter;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(super) struct Serde<T>(pub T);

impl<'de> Deserialize<'de> for Serde<Vec<TxnFilter>> {
    fn deserialize<D>(d: D) -> Result<Serde<Vec<TxnFilter>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = <Vec<TxnFilter>>::deserialize(d)?;

        if v.len() > 1 {
            Ok(Serde(v))
        } else {
            let msg = "Expected multiple filters for logical filter".to_string();
            Err(D::Error::custom(msg))
        }
    }
}

pub(super) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    Serde<T>: Deserialize<'de>,
{
    Serde::deserialize(deserializer).map(|x| x.0)
}

pub(super) fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
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

impl<T> From<T> for Serde<T> {
    fn from(val: T) -> Serde<T> {
        Serde(val)
    }
}

impl Serialize for Serde<&Vec<TxnFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl Serialize for Serde<Vec<TxnFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
