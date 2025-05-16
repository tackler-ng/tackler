/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Txn Geo Location (2D) filter
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterBBoxLatLon {
    /// min latitude
    pub south: Decimal,
    /// min longitude
    pub west: Decimal,
    /// max latitude
    pub north: Decimal,
    /// max longitude
    pub east: Decimal,
}

impl IndentDisplay for TxnFilterBBoxLatLon {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        let my_indent = format!("{indent}  ");
        writeln!(f, "{indent}Txn Bounding Box 2D")?;
        writeln!(
            f,
            "{my_indent}North, East: geo:{},{}",
            self.north, self.east
        )?;
        writeln!(
            f,
            "{my_indent}South, West: geo:{},{}",
            self.south, self.west
        )
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
    use rust_decimal_macros::dec;
    use tackler_rs::IndocUtils;

    #[test]
    // test: 05bfe9c0-0dc1-462a-b452-39c2eaf55d02
    // desc: BBoxLatLon, JSON
    fn txn_bbox_lat_lon_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLon":{"south":"59.85","west":"24.0","north":"60.8","east":"27.5"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn Bounding Box 2D
         |    North, East: geo:60.8,27.5
         |    South, West: geo:59.85,24.0
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterBBoxLatLon(_) = tf.txn_filter {
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
    // test: 89d31f9c-029f-47ce-acb9-ddfaaa089782
    // desc: BBoxLatLon, Text
    fn txn_bbox_lat_lon_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn Bounding Box 2D
         |      North, East: geo:60.8,27.5
         |      South, West: geo:59.85,24.0
         |    AND
         |      Txn Bounding Box 2D
         |        North, East: geo:60.8,27.5
         |        South, West: geo:59.85,24.0
         |      All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterBBoxLatLon(TxnFilterBBoxLatLon {
                        south: dec!(59.85),
                        west: dec!(24.0),
                        north: dec!(60.8),
                        east: dec!(27.5),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterBBoxLatLon(TxnFilterBBoxLatLon {
                                south: dec!(59.85),
                                west: dec!(24.0),
                                north: dec!(60.8),
                                east: dec!(27.5),
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
