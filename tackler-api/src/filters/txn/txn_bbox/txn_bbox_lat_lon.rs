/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use crate::filters::txn::txn_bbox::{EAST, NORTH, SOUTH, WEST, validate};
use crate::tackler;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, MapAccess, Visitor},
    ser::SerializeStruct,
};
use std::fmt::Formatter;

/// Txn Geo Location (2D) filter
///
#[derive(Clone, Debug)]
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

impl TxnFilterBBoxLatLon {
    /// Create a new 2D Bounding Box
    /// # Errors
    /// Return error in case the bounding box is invalid
    pub fn new(
        south: Decimal,
        west: Decimal,
        north: Decimal,
        east: Decimal,
    ) -> Result<TxnFilterBBoxLatLon, tackler::Error> {
        let bbox = TxnFilterBBoxLatLon {
            south,
            west,
            north,
            east,
        };
        validate(bbox.south, bbox.west, bbox.north, bbox.east)?;
        Ok(bbox)
    }
}

impl Serialize for TxnFilterBBoxLatLon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("TxnFilterBBoxLatLon", 4)?;
        st.serialize_field(SOUTH, &self.south)?;
        st.serialize_field(WEST, &self.west)?;
        st.serialize_field(NORTH, &self.north)?;
        st.serialize_field(EAST, &self.east)?;
        st.end()
    }
}

impl<'de> Deserialize<'de> for TxnFilterBBoxLatLon {
    fn deserialize<D>(deserializer: D) -> Result<TxnFilterBBoxLatLon, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            South,
            West,
            North,
            East,
        }

        struct FieldVisitor;

        impl Visitor<'_> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(format!("one of: {SOUTH}, {WEST}, {NORTH}, {EAST}").as_str())
            }

            fn visit_str<E>(self, v: &str) -> Result<Field, E>
            where
                E: DeError,
            {
                match v {
                    SOUTH => Ok(Field::South),
                    WEST => Ok(Field::West),
                    NORTH => Ok(Field::North),
                    EAST => Ok(Field::East),
                    _ => Err(E::unknown_field(v, &[SOUTH, WEST, NORTH, EAST])),
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D2>(deserializer: D2) -> Result<Field, D2::Error>
            where
                D2: Deserializer<'de>,
            {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TxnFilterBBoxLatLonVisitor;

        impl<'de> Visitor<'de> for TxnFilterBBoxLatLonVisitor {
            type Value = TxnFilterBBoxLatLon;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    format!(
                        "TxnFilterBBoxLatLon as a map with keys  {SOUTH}, {WEST}, {NORTH}, {EAST}"
                    )
                    .as_str(),
                )
            }

            fn visit_map<A>(self, mut map: A) -> Result<TxnFilterBBoxLatLon, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut south: Option<Decimal> = None;
                let mut west: Option<Decimal> = None;
                let mut north: Option<Decimal> = None;
                let mut east: Option<Decimal> = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::South => {
                            if south.is_some() {
                                return Err(A::Error::duplicate_field(SOUTH));
                            }
                            south = Some(map.next_value()?);
                        }
                        Field::West => {
                            if west.is_some() {
                                return Err(A::Error::duplicate_field(WEST));
                            }
                            west = Some(map.next_value()?);
                        }
                        Field::North => {
                            if north.is_some() {
                                return Err(A::Error::duplicate_field(NORTH));
                            }
                            north = Some(map.next_value()?);
                        }
                        Field::East => {
                            if east.is_some() {
                                return Err(A::Error::duplicate_field(EAST));
                            }
                            east = Some(map.next_value()?);
                        }
                    }
                }

                let bbox = TxnFilterBBoxLatLon::new(
                    south.ok_or_else(|| A::Error::missing_field(SOUTH))?,
                    west.ok_or_else(|| A::Error::missing_field(WEST))?,
                    north.ok_or_else(|| A::Error::missing_field(NORTH))?,
                    east.ok_or_else(|| A::Error::missing_field(EAST))?,
                )
                .map_err(A::Error::custom)?;

                Ok(bbox)
            }
        }

        deserializer.deserialize_struct(
            "TxnFilterBBoxLatLon",
            &[SOUTH, WEST, NORTH, EAST],
            TxnFilterBBoxLatLonVisitor,
        )
    }
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
    use crate::filters::txn::txn_bbox::tests::err_bboxes;
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

    #[test]
    // test: 37063f39-0796-44bd-a300-511f36db8f48
    // desc: detects illegal arguments
    fn detects_illegal_arguments() {
        let count: usize = err_bboxes()
            .iter()
            .map(|(south, west, north, east, expected_msg)| {
                let res = TxnFilterBBoxLatLon::new(*south, *west, *north, *east);

                let err = res.expect_err("expected illegal arguments to be rejected");
                assert!(
                    err.to_string().contains(expected_msg.as_str()),
                    "error message mismatch.\nexpected to contain: {expected_msg}\nactual: {err}"
                );
                1usize
            })
            .sum();

        assert_eq!(count, 9);
    }
    #[test]
    // test: e690ce1d-4e0c-4f73-9b71-5a6a84dc52b8
    // desc: detects illegal arguments via JSON
    fn detects_illegal_arguments_via_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLon":{"south":"60","west":"24.0","north":"10","east":"27.5"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        let err = tf_res.expect_err("expected illegal arguments to be rejected");
        assert!(
            err.to_string()
                .contains("North is below South. South: 60; North: 10")
        );
    }
}
