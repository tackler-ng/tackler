/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::tackler;
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Write;

const INDENT_INC: usize = 4;

fn datetime_pretty_string(dt: &toml::value::Datetime) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    if let Some(d) = dt.date {
        write!(s, "{d}")?;
    }
    if let Some(t) = dt.time {
        write!(s, " {t}")?;
        if let Some(z) = dt.offset {
            write!(s, "{z}")?;
        }
    }
    Ok(s)
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize)]
#[serde(try_from = "toml::Value")]
#[serde(deny_unknown_fields)]
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
    fn pretty_serialize(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        let indstr = " ".repeat(indent);
        let my_indent = indent + INDENT_INC;
        let mind = " ".repeat(my_indent);

        if self.open.is_none() && self.close.is_none() && self.spans.is_none() {
            write!(f, "{indstr}\"{}\"", self.name)
        } else {
            writeln!(f, "{indstr}{{")?;
            writeln!(f, "{mind}name = \"{}\",", self.name)?;
            if let Some(open) = self.open {
                writeln!(f, "{mind}open = {},", datetime_pretty_string(&open)?)?;
            }
            if let Some(close) = self.close {
                writeln!(f, "{mind}close = {},", datetime_pretty_string(&close)?)?;
            }
            if let Some(spans) = &self.spans {
                writeln!(f, "{mind}spans = [")?;
                for span in spans.iter().sorted() {
                    span.pretty_serialize(f, my_indent + INDENT_INC)?;
                    writeln!(f, ",")?;
                }
                writeln!(f, "{mind}],")?;
            }
            write!(f, "{indstr}}}")
        }
    }
}

impl Display for AccountSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_serialize(f, INDENT_INC)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SpanSpec {
    pub(crate) open: toml::value::Datetime,
    pub(crate) close: toml::value::Datetime,
}

impl SpanSpec {
    fn pretty_serialize(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        write!(f, "{}{{ ", " ".repeat(indent))?;
        write!(f, "open = {}, ", datetime_pretty_string(&self.open)?)?;
        write!(f, "close = {} ", datetime_pretty_string(&self.close)?)?;
        write!(f, "}}")
    }
}

impl PartialOrd for SpanSpec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpanSpec {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.open.cmp(&other.open) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.close.cmp(&other.close),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::fmt;
    use tackler_rs::IndocUtils;

    impl SpanSpec {
        fn debug_print(&self) -> String {
            struct PrettyFmt<'a>(&'a SpanSpec);
            impl Display for PrettyFmt<'_> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.pretty_serialize(f, 2)
                }
            }
            format!("{}", PrettyFmt(self))
        }
    }

    #[test]
    fn span_ok() {
        let tests = vec![
            (
                "\nopen = 2025-01-01 \n close = 2026-01-01",
                "  { open = 2025-01-01, close = 2026-01-01 }",
            ),
            (
                "\nopen = 2025-01-01 12:13:14 \n close = 2026-01-01",
                "  { open = 2025-01-01 12:13:14, close = 2026-01-01 }",
            ),
            (
                "\nopen = 2025-01-01 12:13:14+02:00 \n close = 2026-01-01",
                "  { open = 2025-01-01 12:13:14+02:00, close = 2026-01-01 }",
            ),
            (
                "\nopen = 2025-01-01 \n close = 2026-01-01",
                "  { open = 2025-01-01, close = 2026-01-01 }",
            ),
            (
                "\nopen = 2025-01-01 \n close = 2026-01-01 12:13:14",
                "  { open = 2025-01-01, close = 2026-01-01 12:13:14 }",
            ),
            (
                "\nopen = 2025-01-01 \n close = 2026-01-01 12:13:14+02:00",
                "  { open = 2025-01-01, close = 2026-01-01 12:13:14+02:00 }",
            ),
        ];
        for test in tests {
            let spec: Result<SpanSpec, _> = toml::from_str(test.0);
            assert!(spec.is_ok());
            let spec = spec.unwrap(/*:test:*/);

            assert_eq!(spec.debug_print(), test.1);
        }
    }
    #[test]
    fn span_err() {
        let tests = vec![
            "\nopen = 2025-01-01 \n",
            "\nclose = 2025-01-01 \n",
            "\nopen = 2025-01-01 \n close = 2026-01-01 \n valid = 0",
        ];
        for test in tests {
            let spec: Result<SpanSpec, _> = toml::from_str(test);
            assert!(spec.is_err());
        }
    }

    #[test]
    fn account_ok() {
        let tests = vec![
            (
                "\nname = \"eman\" \n open = 2025-01-01 \n close = 2026-01-01",
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        open = 2025-01-01,
                       |        close = 2026-01-01,
                       |    }"#
                )
                .strip_margin(),
            ),
            (
                "\nname = \"eman\" \n open = 2025-01-01",
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        open = 2025-01-01,
                       |    }"#
                )
                .strip_margin(),
            ),
            (
                "\nname = \"eman\" \n close = 2026-01-01",
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        close = 2026-01-01,
                       |    }"#
                )
                .strip_margin(),
            ),
            (
                "\nname = \"eman\" \n open = 2025-01-01 12:13:14.999+02:00 \n close = 2026-07-01 22:23:24+03:00",
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        open = 2025-01-01 12:13:14.999+02:00,
                       |        close = 2026-07-01 22:23:24+03:00,
                       |    }"#
                )
                .strip_margin(),
            ),
            (
                r#"
                 name = "eman"
                 spans = [ { open = 2025-01-01, close = 2026-01-01 } ]
                "#,
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        spans = [
                       |            { open = 2025-01-01, close = 2026-01-01 },
                       |        ],
                       |    }"#
                )
                .strip_margin(),
            ),
            (
                r#"
                 name = "eman"
                 spans = [ { open = 2025-01-01, close = 2026-01-01 },
                 { open = 2024-01-01, close = 2025-01-01 },
                 { open = 2026-01-01, close = 2027-01-01 },
                 ]
                "#,
                indoc!(
                    r#"|    {
                       |        name = "eman",
                       |        spans = [
                       |            { open = 2024-01-01, close = 2025-01-01 },
                       |            { open = 2025-01-01, close = 2026-01-01 },
                       |            { open = 2026-01-01, close = 2027-01-01 },
                       |        ],
                       |    }"#
                )
                .strip_margin(),
            ),
        ];
        for test in tests {
            let spec: Result<AccountSpec, _> = toml::from_str(test.0);
            assert!(spec.is_ok());
            let spec = spec.unwrap(/*:test:*/);
            let spec_str = format!("{spec}");
            assert_eq!(spec_str, test.1);
        }
    }

    #[test]
    fn account_err() {
        let tests = vec![("\nname = \"eman\" \n open = 2025-01-01 \n valid = 0",)];
        for test in tests {
            let spec: Result<AccountSpec, _> = toml::from_str(test.0);
            assert!(spec.is_err());
        }
    }
}
