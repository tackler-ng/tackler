/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::filters::IndentDisplay;
use crate::filters::TxnFilter;
use crate::tackler;
use base64::{Engine as _, engine::general_purpose};
use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::from_utf8;

/// The main filter definition
///
/// This is the main handle for Txn Filter  definition, and this can be used to serialize
/// and deserialize filters from JSON.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # use tackler_api::filters::FilterDefinition;
/// # use tackler_api::filters::TxnFilter;
///
/// let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;
///
/// let tf = serde_json::from_str::<FilterDefinition>(filter_json_str)?;
///
/// match tf.txn_filter {
///      TxnFilter::NullaryTRUE(_) => (),
///      _ => panic!(),
/// }
///
/// assert_eq!(serde_json::to_string(&tf)?, filter_json_str);
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterDefinition {
    #[doc(hidden)]
    #[serde(rename = "txnFilter")]
    pub txn_filter: TxnFilter,
}

/// Helper used to carry Timezone information to Display Trait
pub struct FilterDefZoned<'a> {
    /// Transaction Filter Definition
    pub filt_def: &'a FilterDefinition,
    /// Timezone to be by Display
    pub tz: TimeZone,
}
impl Display for FilterDefZoned<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Filter")?;
        self.filt_def.txn_filter.i_fmt("  ", self.tz.clone(), f)
    }
}

impl FilterDefinition {
    const FILTER_ARMOR: &'static str = "base64:";

    /// Generate filter from JSON String
    ///
    /// # Errors
    ///
    /// Return `Err` if the filter definition is not valid
    ///
    /// # Examples
    /// ```
    /// # use tackler_api::tackler;
    /// # use tackler_api::filters::FilterDefinition;
    /// # use tackler_api::filters::TxnFilter;
    ///
    /// let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;
    ///
    /// let tf = FilterDefinition::from_json_str(filter_json_str)?;
    ///
    /// match tf.txn_filter {
    ///      TxnFilter::NullaryTRUE(_) => (),
    ///      _ => panic!(),
    /// }
    ///
    /// # Ok::<(), tackler::Error>(())
    /// ```
    pub fn from_json_str(filt_str: &str) -> Result<FilterDefinition, tackler::Error> {
        match serde_json::from_str::<FilterDefinition>(filt_str) {
            Ok(flt) => Ok(flt),
            Err(err) => {
                let msg = format!("Txn Filter definition is not valid JSON: {err}");
                Err(msg.into())
            }
        }
    }

    /// Test if filter string is ascii armored
    ///
    #[must_use]
    pub fn is_armored(filt: &str) -> bool {
        filt.starts_with(FilterDefinition::FILTER_ARMOR)
    }

    /// Generate filter from ascii armor JSON String
    ///
    /// The ascii armor must be be prefixed with `base64`
    ///
    /// # Errors
    ///
    /// Returns `Err` if the filter definition is not valid or encoding is unknown
    ///
    /// # Examples
    /// ```
    /// # use tackler_api::tackler;
    /// # use tackler_api::filters::FilterDefinition;
    /// # use tackler_api::filters::TxnFilter;
    ///
    /// let filter_ascii_armor = "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K";
    ///
    /// let tf = FilterDefinition::from_armor(filter_ascii_armor)?;
    ///
    /// match tf.txn_filter {
    ///      TxnFilter::NullaryTRUE(_) => (),
    ///      _ => panic!(),
    /// }
    ///
    /// # Ok::<(), tackler::Error>(())
    /// ```
    pub fn from_armor(filt_armor_str: &str) -> Result<FilterDefinition, tackler::Error> {
        let filt_armor = if FilterDefinition::is_armored(filt_armor_str) {
            filt_armor_str.trim_start_matches(FilterDefinition::FILTER_ARMOR)
        } else {
            let filt_begin = match filt_armor_str.char_indices().nth(10) {
                None => filt_armor_str,
                Some((idx, _)) => &filt_armor_str[..idx],
            };
            let msg = format!(
                "Unknown filter encoding, supported armor is: {}, (first 10 chars are): [{}]",
                FilterDefinition::FILTER_ARMOR,
                filt_begin
            );
            return Err(msg.into());
        };
        let filt_json = match general_purpose::STANDARD.decode(filt_armor) {
            Ok(data) => data,
            Err(err) => {
                let msg = format!("Transaction Filter Ascii Armor decoding failure: {err}");
                return Err(msg.into());
            }
        };

        FilterDefinition::from_json_str(from_utf8(&filt_json)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::NullaryTRUE;
    use indoc::indoc;
    use jiff::tz;
    use tackler_rs::IndocUtils;

    #[test]
    // test: c6fe4f86-1daa-4e29-b327-467aed6dc5bb
    // desc: filter definition, JSON
    fn filter_definition_json() {
        let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  All pass
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::NullaryTRUE(_) = tf.txn_filter {
        } else {
            panic!(/*:test:*/)
        }

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 5e90f6cb-4414-4d4e-a496-1bb26abb9ba1
    // desc: filter definition, Text
    fn filter_definition_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::NullaryTRUE(NullaryTRUE {}),
        };

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
    }

    #[test]
    fn filter_definition_is_encoded() {
        assert!(FilterDefinition::is_armored(FilterDefinition::FILTER_ARMOR));
        assert!(!FilterDefinition::is_armored("hello there"));
    }

    #[test]
    // test: 939516a3-3c7a-4af8-b8fc-bcec2839965d
    // desc: decode txn filter from base64 armored JSON
    fn filter_definition_from_decoded() {
        let filters = vec![
            "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K",
            "base64:IHsgInR4bkZpbHRlciI6eyJOdWxsYXJ5VFJVRSI6e30gfSB9Cg==",
        ];

        for s in filters {
            let tf_res = FilterDefinition::from_armor(s);
            assert!(tf_res.is_ok());

            let tf = tf_res.unwrap(/*:test:*/);
            if let TxnFilter::NullaryTRUE(_) = tf.txn_filter {
            } else {
                panic!(/*:test:*/)
            }
        }
    }

    #[test]
    fn filter_definition_check_err_msg() {
        let s_err = "eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K";

        let tf_res = FilterDefinition::from_armor(s_err);
        assert!(tf_res.is_err());

        let msg = tf_res.err().unwrap(/*:test:*/).to_string();

        assert!(msg.contains(FilterDefinition::FILTER_ARMOR));
        // test malformed cut-off
        assert!(msg.contains("eyJ0eG5GaW"));
        assert!(!msg.contains("eyJ0eG5GaWx"));
    }
}
