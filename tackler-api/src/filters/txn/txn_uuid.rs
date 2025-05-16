/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use uuid::Uuid;

use crate::filters::IndentDisplay;

/// Txn UUID filter
///
/// Select transaction if its UUID code is specified `uuid`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnUUID {
    #[doc(hidden)]
    pub uuid: Uuid,
}

impl IndentDisplay for TxnFilterTxnUUID {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Txn UUID: {}", self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{
        FilterDefZoned, FilterDefinition, NullaryTRUE, TxnFilter, logic::TxnFilterAND,
    };
    use indoc::indoc;
    use jiff::tz;
    use tackler_rs::IndocUtils;

    #[test]
    // test: 9ad41df9-c153-458b-a941-3b4763c25548
    // desc: TxnUUID, JSON
    fn txn_uuid_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnUUID":{"uuid":"8c913372-48e9-466c-a897-11b151548a19"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn UUID: 8c913372-48e9-466c-a897-11b151548a19
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterTxnUUID(_) = tf.txn_filter {
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
    // test: e388aecd-8500-4f89-98c6-9588199c104f
    // desc: TxnUUID, Text
    fn txn_uuid_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn UUID: 76a0f143-d64e-4497-b357-5ae2eb092219
         |    AND
         |      Txn UUID: f01df5b5-18e2-477c-aaac-3e0b672b2729
         |      All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnUUID(TxnFilterTxnUUID {
                        uuid: Uuid::parse_str("76a0f143-d64e-4497-b357-5ae2eb092219").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnUUID(TxnFilterTxnUUID {
                                uuid: Uuid::parse_str("f01df5b5-18e2-477c-aaac-3e0b672b2729")
                                    .unwrap(/*:test:*/),
                            }),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
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
}
