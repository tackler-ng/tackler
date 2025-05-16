/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Txn Geo Location (3D) filter
///
/// `BBoxLatLonAlt` will select only 3D transactions with altitude,
/// e.g. it will not select any 2D txn.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterBBoxLatLonAlt {
    /// min latitude
    pub south: Decimal,
    /// min longitude
    pub west: Decimal,
    /// max depth
    pub depth: Decimal,
    /// max latitude
    pub north: Decimal,
    /// max longitude
    pub east: Decimal,
    /// max height
    pub height: Decimal,
}

impl IndentDisplay for TxnFilterBBoxLatLonAlt {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        let my_indent = format!("{indent}  ");
        writeln!(f, "{indent}Txn Bounding Box 3D")?;
        writeln!(
            f,
            "{my_indent}North, East, Height: geo:{},{},{}",
            self.north, self.east, self.height
        )?;
        writeln!(
            f,
            "{my_indent}South, West, Depth:  geo:{},{},{}",
            self.south, self.west, self.depth
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
    // test: c027ef27-3287-411f-aad9-8185f1b55380
    // desc: BBoxLatLonAlt, JSON
    fn txn_bbox_lat_lon_alt_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLonAlt":{"south":"-1.0","west":"-2.0","depth":"-3.0","north":"1.0","east":"2.0","height":"3.0"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn Bounding Box 3D
         |    North, East, Height: geo:1.0,2.0,3.0
         |    South, West, Depth:  geo:-1.0,-2.0,-3.0
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        if let TxnFilter::TxnFilterBBoxLatLonAlt(_) = tf.txn_filter {
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
    // test: 54528f53-49fc-43cf-b3a2-221e02e87bcc
    // desc: BBoxLatLonAlt, Text
    fn txn_bbox_lat_lon_alt_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn Bounding Box 3D
         |      North, East, Height: geo:1,2,3
         |      South, West, Depth:  geo:-1,-2,-3
         |    AND
         |      Txn Bounding Box 3D
         |        North, East, Height: geo:1,2,3
         |        South, West, Depth:  geo:-1,-2,-3
         |      All pass
         |"}
        .strip_margin();

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt {
                        south: dec!(-1),
                        west: dec!(-2),
                        depth: dec!(-3),
                        north: dec!(1),
                        east: dec!(2),
                        height: dec!(3),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt {
                                south: dec!(-1),
                                west: dec!(-2),
                                depth: dec!(-3),
                                north: dec!(1),
                                east: dec!(2),
                                height: dec!(3),
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
