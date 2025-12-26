/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::filters::IndentDisplay;
use crate::filters::txn::txn_bbox::{DEPTH, EAST, HEIGHT, NORTH, SOUTH, WEST, validate};
use crate::location::MIN_ALTITUDE;
use crate::tackler;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use std::fmt::Formatter;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, MapAccess, Visitor},
    ser::SerializeStruct,
};

/// Txn Geo Location (3D) filter
///
/// `BBoxLatLonAlt` will select only 3D transactions with altitude,
/// e.g. it will not select any 2D txn.
#[derive(Clone, Debug)]
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

impl TxnFilterBBoxLatLonAlt {
    /// Create a new 3D Bounding Box
    /// # Errors
    /// Return error in case the bounding box is invalid
    pub fn new(
        south: Decimal,
        west: Decimal,
        depth: Decimal,
        north: Decimal,
        east: Decimal,
        height: Decimal,
    ) -> Result<TxnFilterBBoxLatLonAlt, tackler::Error> {
        let bbox = TxnFilterBBoxLatLonAlt {
            south,
            west,
            depth,
            north,
            east,
            height,
        };
        validate(bbox.south, bbox.west, bbox.north, bbox.east)?;

        if height < depth {
            Err(format!(
                "Invalid Bounding Box: height is less than depth. Depth: {depth}; Height: {height}"
            )
            .into())
        } else if depth < MIN_ALTITUDE {
            // height is tested by height < depth test
            Err(
                format!("Invalid Bounding Box: Depth is beyond center of Earth. Depth: {depth}")
                    .into(),
            )
        } else {
            Ok(bbox)
        }
    }
}

impl Serialize for TxnFilterBBoxLatLonAlt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("TxnAFilterBBoxLatLonAlt", 6)?;
        st.serialize_field(SOUTH, &self.south)?;
        st.serialize_field(WEST, &self.west)?;
        st.serialize_field(DEPTH, &self.depth)?;
        st.serialize_field(NORTH, &self.north)?;
        st.serialize_field(EAST, &self.east)?;
        st.serialize_field(HEIGHT, &self.height)?;
        st.end()
    }
}

impl<'de> Deserialize<'de> for TxnFilterBBoxLatLonAlt {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D>(deserializer: D) -> Result<TxnFilterBBoxLatLonAlt, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            South,
            West,
            Depth,
            North,
            East,
            Height,
        }

        struct FieldVisitor;

        impl Visitor<'_> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    format!("one of: {SOUTH}, {WEST}, {DEPTH} ,{NORTH}, {EAST}, {HEIGHT}").as_str(),
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Field, E>
            where
                E: DeError,
            {
                match v {
                    SOUTH => Ok(Field::South),
                    WEST => Ok(Field::West),
                    DEPTH => Ok(Field::Depth),
                    NORTH => Ok(Field::North),
                    EAST => Ok(Field::East),
                    HEIGHT => Ok(Field::Height),
                    _ => Err(E::unknown_field(
                        v,
                        &[SOUTH, WEST, DEPTH, NORTH, EAST, HEIGHT],
                    )),
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

        struct TxnFilterBBoxLatLonAltVisitor;

        impl<'de> Visitor<'de> for TxnFilterBBoxLatLonAltVisitor {
            type Value = TxnFilterBBoxLatLonAlt;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    format!(
                        "TxnFilterBBoxLatLonAlt as a map with keys  {SOUTH}, {WEST}, {DEPTH}, {NORTH}, {EAST}, {HEIGHT}"
                    )
                    .as_str(),
                )
            }

            fn visit_map<A>(self, mut map: A) -> Result<TxnFilterBBoxLatLonAlt, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut south: Option<Decimal> = None;
                let mut west: Option<Decimal> = None;
                let mut depth: Option<Decimal> = None;
                let mut north: Option<Decimal> = None;
                let mut east: Option<Decimal> = None;
                let mut height = Option::<Decimal>::None;

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
                        Field::Depth => {
                            if depth.is_some() {
                                return Err(A::Error::duplicate_field(DEPTH));
                            }
                            depth = Some(map.next_value()?);
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
                        Field::Height => {
                            if height.is_some() {
                                return Err(A::Error::duplicate_field(HEIGHT));
                            }
                            height = Some(map.next_value()?);
                        }
                    }
                }

                let bbox = TxnFilterBBoxLatLonAlt::new(
                    south.ok_or_else(|| A::Error::missing_field(SOUTH))?,
                    west.ok_or_else(|| A::Error::missing_field(WEST))?,
                    depth.ok_or_else(|| A::Error::missing_field(DEPTH))?,
                    north.ok_or_else(|| A::Error::missing_field(NORTH))?,
                    east.ok_or_else(|| A::Error::missing_field(EAST))?,
                    height.ok_or_else(|| A::Error::missing_field(HEIGHT))?,
                )
                .map_err(A::Error::custom)?;

                Ok(bbox)
            }
        }

        deserializer.deserialize_struct(
            "TxnFilterBBoxLatLonAlt",
            &[SOUTH, WEST, DEPTH, NORTH, EAST, HEIGHT],
            TxnFilterBBoxLatLonAltVisitor,
        )
    }
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
    use crate::filters::txn::txn_bbox::tests::err_bboxes;
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

    #[test]
    // test: 1d6f4fb9-bcfd-41ae-8720-2584ec2f4087
    // desc: detects illegal arguments (2D of 3D)
    fn detects_illegal_arguments() {
        let count: usize = err_bboxes()
            .iter()
            .map(|(south, west, north, east, expected_msg)| {
                let res =
                    TxnFilterBBoxLatLonAlt::new(*south, *west, dec!(0), *north, *east, dec!(1));

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
    // test: 426a47cd-2373-48a6-b241-2e9627ed26fb
    // desc: detects illegal arguments (3D)
    fn detects_illegal_arguments_3d() {
        #[rustfmt::skip]
        let err_bboxes = [
            (dec!(0), dec!(0), dec!(10),         dec!(0), dec!(0), dec!(8.1), "height is less than depth. Depth: 10; Height: 8.1".to_string()),
            (dec!(0), dec!(0), dec!(-8.1),       dec!(0), dec!(0), dec!(-10), "height is less than depth. Depth: -8.1; Height: -10".to_string()),
            (dec!(0), dec!(0), dec!(2),          dec!(0), dec!(0), dec!(-2), "height is less than depth. Depth: 2; Height: -2".to_string()),
            (dec!(0), dec!(0), dec!(-6378137.1), dec!(0), dec!(0), dec!(-2), "Depth is beyond center of Earth. Depth: -6378137.1".to_string()),
        ];
        let count: usize = err_bboxes
            .iter()
            .map(|(south, west, depth, north, east, heigth, expected_msg)| {
                let res =
                    TxnFilterBBoxLatLonAlt::new(*south, *west, *depth, *north, *east, *heigth);

                let err = res.expect_err("expected illegal arguments to be rejected");
                assert!(
                    err.to_string().contains(expected_msg.as_str()),
                    "error message mismatch.\nexpected to contain: {expected_msg}\nactual: {err}"
                );
                1usize
            })
            .sum();

        assert_eq!(count, 4);
    }
    #[test]
    // test: 92232872-cea2-4787-8ba4-892d958796cb
    // desc: detects illegal arguments via JSON
    fn detects_illegal_arguments_via_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLonAlt":{"south":"-1.0","west":"-2.0","depth":"3.0","north":"1.0","east":"2.0","height":"-3.0"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        let err = tf_res.expect_err("expected illegal arguments to be rejected");
        assert!(
            err.to_string()
                .contains("height is less than depth. Depth: 3.0; Height: -3.0")
        );
    }
}
