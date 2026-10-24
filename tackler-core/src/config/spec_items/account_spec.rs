/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::tackler;
use serde::Deserialize;

#[derive(Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(try_from = "toml::Value")]
pub(crate) struct AccountSpec {
    pub(crate) name: String,
    pub(crate) open: Option<toml::value::Datetime>,
    pub(crate) close: Option<toml::value::Datetime>,
    pub(crate) spans: Option<Vec<SpanSpec>>,
}

impl AccountSpec {
    pub(crate) fn from_str(s: &str) -> Self {
        AccountSpec {
            name: s.to_string(),
            ..AccountSpec::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub(crate) struct SpanSpec {
    pub(crate) open: toml::value::Datetime,
    pub(crate) close: toml::value::Datetime,
}

#[derive(Deserialize)]
struct AccountT {
    name: String,
    open: Option<toml::value::Datetime>,
    close: Option<toml::value::Datetime>,
    spans: Option<Vec<SpanSpec>>,
}

impl TryFrom<toml::Value> for AccountSpec {
    type Error = tackler::Error;
    fn try_from(value: toml::Value) -> Result<Self, Self::Error> {
        match value {
            toml::Value::Table(t) => {
                let at: AccountT = t.try_into()?;
                if at.spans.is_some() && (at.open.is_some() || at.close.is_some()) {
                    let msg = format!(
                        "Account '{}' has main open-close section and spans defined",
                        at.name
                    );
                    Err(msg.into())
                } else {
                    let a = AccountSpec {
                        name: at.name,
                        open: at.open,
                        close: at.close,
                        spans: at.spans,
                    };
                    Ok(a)
                }
            }
            toml::Value::String(s) => Ok(AccountSpec::from_str(&s)),
            _ => Err("IE: TOML deserializing error - unexpected Value type"
                .to_string()
                .into()),
        }
    }
}
