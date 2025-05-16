/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

//! Transaction header
//!
use jiff::Zoned;
use jiff::tz::TimeZone;
use serde::Serialize;
use std::cmp::Ordering;
use std::fmt::Write;
use std::sync::Arc;
use uuid::Uuid;

/// Collection of Txn Tags
pub type Tags = Vec<Arc<String>>;

/// Single Txn Tag
pub type Tag = String;

/// Collection of Txn comments
pub type Comments = Vec<String>;

use crate::location::GeoPoint;

/// Transaction Header Structure
///
#[derive(Serialize, Debug, Clone, Default)]
pub struct TxnHeader {
    /// Txn timestamp with Zone information
    pub timestamp: Zoned,
    /// Txn Code field, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Txn Description, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Txn UUID, if any. This is mandatory, if audit-mode is on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    /// Txn location, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoPoint>,
    /// Txn tags, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Txn comments, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
}

impl TxnHeader {
    fn t_to_s(tags: &Tags) -> String {
        let t = tags.iter().fold((0, String::new()), |mut tags, t| {
            if tags.0 == 0 {
                let _ = write!(tags.1, "{t}");
            } else {
                let _ = write!(tags.1, ", {t}");
            }
            (tags.0 + 1, tags.1)
        });
        t.1
    }
    /// Get Tags as string.
    ///
    /// String will be empty, if there isn't any tag
    #[must_use]
    pub fn tags_to_string(&self) -> String {
        match &self.tags {
            Some(t) => Self::t_to_s(t),
            None => String::new(),
        }
    }
}

impl Ord for TxnHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        let date_comp = self.timestamp.cmp(&other.timestamp);
        if date_comp.is_ne() {
            date_comp
        } else {
            let empty = String::new();

            let code_cmp = self
                .code
                .as_ref()
                .unwrap_or(&empty)
                .cmp(other.code.as_ref().unwrap_or(&empty));
            if code_cmp.is_ne() {
                code_cmp
            } else {
                let desc_cmp = self
                    .description
                    .as_ref()
                    .unwrap_or(&empty)
                    .cmp(other.description.as_ref().unwrap_or(&empty));
                if desc_cmp.is_ne() {
                    desc_cmp
                } else {
                    let uuid_this = self
                        .uuid
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    let uuid_other = other
                        .uuid
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();

                    uuid_this.cmp(&uuid_other)
                }
            }
        }
    }
}

impl PartialOrd for TxnHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TxnHeader {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.code == other.code
            && self.description == other.description
            && self.uuid == other.uuid
    }
}

impl Eq for TxnHeader {}

impl TxnHeader {
    /// Get Txn header as string, with `indent` and Txn TS formatter
    ///
    /// See [`txn_ts`](crate::txn_ts) module for default formatters
    pub fn to_string_with_indent(
        &self,
        indent: &str,
        ts_formatter: fn(&Zoned, TimeZone) -> String,
        tz: TimeZone,
    ) -> String {
        format!(
            "{}{}{}\n{}{}{}{}",
            // txn header line: ts, code, desc
            ts_formatter(&self.timestamp, tz),
            self.code
                .as_ref()
                .map_or_else(String::new, |c| format!(" ({c})")),
            self.description
                .as_ref()
                .map_or_else(String::new, |desc| format!(" '{desc}")),
            // metadata
            self.uuid
                .as_ref()
                .map_or_else(String::new, |uuid| format!("{indent}# uuid: {uuid}\n")),
            self.location
                .as_ref()
                .map_or_else(String::new, |geo| format!("{indent}# location: {geo}\n")),
            self.tags.as_ref().map_or_else(String::new, |tags| format!(
                "{}# tags: {}\n",
                indent,
                Self::t_to_s(tags)
            )),
            // txn comments
            self.comments.as_ref().map_or_else(String::new, |comments| {
                comments
                    .iter()
                    .fold(String::with_capacity(128), |mut output, c| {
                        let _ = writeln!(output, "{indent}; {c}");
                        output
                    })
            })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::formatdoc;
    use indoc::indoc;
    use jiff::fmt::strtime;
    use rust_decimal_macros::dec;
    use tackler_rs::IndocUtils;

    use crate::{txn_header::TxnHeader, txn_ts};

    #[test]
    #[allow(clippy::too_many_lines)]
    fn txn_header_display() {
        let ts = strtime::parse(
            "%Y-%m-%dT%H:%M:%S%.f%:z",
            "2023-02-04T14:03:05.047974+02:00",
        )
        .unwrap()
        .to_zoned()
        .unwrap();

        let ts_second = strtime::parse("%Y-%m-%dT%H:%M:%S%.f%:z", "2025-01-08T14:15:16-05:00")
            .unwrap()
            .to_zoned()
            .unwrap();

        let ts_nano = strtime::parse(
            "%Y-%m-%dT%H:%M:%S%.f%:z",
            "2025-01-08T14:15:16.123456789-05:00",
        )
        .unwrap()
        .to_zoned()
        .unwrap();

        let uuid_str = "ed6d4110-f3c0-4770-87fc-b99e46572244";
        let uuid = Uuid::parse_str(uuid_str).unwrap(/*:test:*/);

        let geo = GeoPoint::from(dec!(60.167), dec!(24.955), Some(dec!(5.0))).unwrap(/*:test:*/);

        let txn_tags = vec![
            Arc::new("a".to_string()),
            Arc::new("b".to_string()),
            Arc::new("c".to_string()),
            Arc::new("a:b:c".to_string()),
        ];
        let comments = vec![
            "z 1st line".to_string(),
            "c 2nd line".to_string(),
            "a 3rd line".to_string(),
        ];

        let tests: Vec<(TxnHeader, String)> = vec![
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts_second.clone(),
                    code: None,
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2025-01-08T14:15:16-05:00
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts_nano.clone(),
                    code: None,
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2025-01-08T14:15:16.123456789-05:00
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: Some("#123".to_string()),
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 (#123)
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: Some("#123".to_string()),
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc! {
                    "|2023-02-04T14:03:05.047974+02:00 (#123) 'desc
                     |"
                }
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: Some(uuid),
                    location: None,
                    tags: None,
                    comments: None,
                },
                formatdoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # uuid: {uuid_str}
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: Some(geo.clone()),
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # location: geo:60.167,24.955,5.0
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: Some(txn_tags.clone()),
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # tags: a, b, c, a:b:c
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: Some(comments.clone()),
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   ; z 1st line
                     |   ; c 2nd line
                     |   ; a 3rd line
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: Some(uuid),
                    location: Some(geo),
                    tags: Some(txn_tags),
                    comments: Some(comments),
                },
                formatdoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # uuid: {uuid_str}
                     |   # location: geo:60.167,24.955,5.0
                     |   # tags: a, b, c, a:b:c
                     |   ; z 1st line
                     |   ; c 2nd line
                     |   ; a 3rd line
                     |"
                )
                .strip_margin(),
            ),
        ];

        let mut count = 0;
        let should_be_count = tests.len();
        for t in tests {
            let txn_hdr_str = t.0.to_string_with_indent(
                "   ",
                |ts, _tz| txn_ts::rfc_3339(ts),
                jiff::tz::TimeZone::UTC,
            );
            assert_eq!(txn_hdr_str, t.1);
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }
}
