/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! Transaction Geo location
//!
use crate::tackler;
use rust_decimal::Decimal;
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// Geo Point
///
#[derive(Serialize, Debug, Clone)]
pub struct GeoPoint {
    /// Latitude in decimal format
    pub lat: Decimal,
    /// Longitude in decimal format
    pub lon: Decimal,
    /// optional depth/altitude, in meters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<Decimal>,
}

impl Display for GeoPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let alt = match &self.alt {
            Some(a) => format!(",{a}"),
            None => String::new(),
        };
        write!(f, "geo:{},{}{}", self.lat, self.lon, alt)
    }
}
/// Maximum latitude 90 deg (of North)
pub const MAX_LAT: Decimal = Decimal::from_parts(90, 0, 0, false, 0);
/// Minimum latitude -90 deg (of South)
pub const MIN_LAT: Decimal = Decimal::from_parts(90, 0, 0, true, 0);
/// Maximum longitude 180 (of East)
pub const MAX_LON: Decimal = Decimal::from_parts(180, 0, 0, false, 0);
/// Minimum longitude -180 (of West)
pub const MIN_LON: Decimal = Decimal::from_parts(180, 0, 0, true, 0);
/// Minimum altitude (Jules Verne: Voyage au centre de la Terre)
pub const MIN_ALTITUDE: Decimal = Decimal::from_parts(6_378_137, 0, 0, true, 0);

#[allow(clippy::manual_range_contains)]
impl GeoPoint {
    /// Make Geo point from given coordinates.
    ///
    /// * `lat` in decimals, must be inclusive -90 -- 90
    /// * `lon` in decimals, must be inclusive -180 -- 180
    /// * `alt` in meters, must be more than -6378137 meters
    ///
    /// # Errors
    ///
    /// Returns `Err` in case of invalid coordinates
    pub fn from(
        lat: Decimal,
        lon: Decimal,
        alt: Option<Decimal>,
    ) -> Result<GeoPoint, tackler::Error> {
        if lat < MIN_LAT || MAX_LAT < lat {
            let msg = format!("Value out of specification for Latitude: {lat}");
            return Err(msg.into());
        }
        if lon < MIN_LON || MAX_LON < lon {
            let msg = format!("Value out of specification for Longitude: {lon}");
            return Err(msg.into());
        }
        if let Some(z) = alt {
            if z < Decimal::from(-6_378_137) {
                // Jules Verne: Voyage au centre de la Terre
                let msg = format!("Value Out of specification for Altitude: {z}");
                return Err(msg.into());
            }
        }
        Ok(GeoPoint { lat, lon, alt })
    }
}
#[cfg(test)]
mod tests {
    use crate::location::GeoPoint;
    use rust_decimal_macros::dec;

    // todo: GeoPoint::from + checks

    #[test]
    fn geo_display() {
        let tests: Vec<(GeoPoint, String)> = vec![
            (
                GeoPoint::from(dec!(60), dec!(24), None).unwrap(/*:test:*/),
                "geo:60,24".to_string(),
            ),
            (
                GeoPoint::from(dec!(60), dec!(24), Some(dec!(5))).unwrap(/*:test:*/),
                "geo:60,24,5".to_string(),
            ),
            (
                GeoPoint::from(dec!(60.167), dec!(24.955), Some(dec!(5.0))).unwrap(/*:test:*/),
                "geo:60.167,24.955,5.0".to_string(),
            ),
            (
                GeoPoint::from(dec!(60.167000), dec!(24.955000), Some(dec!(5.000))).unwrap(/*:test:*/),
                "geo:60.167000,24.955000,5.000".to_string(),
            ),
            (
                GeoPoint::from(dec!(-60), dec!(-24), Some(dec!(-5))).unwrap(/*:test:*/),
                "geo:-60,-24,-5".to_string(),
            ),
            (
                GeoPoint::from(dec!(-60.167), dec!(-24.955), Some(dec!(-5.0))).unwrap(/*:test:*/),
                "geo:-60.167,-24.955,-5.0".to_string(),
            ),
            (
                GeoPoint::from(dec!(-60.167000), dec!(-24.955000), Some(dec!(-5.000))).unwrap(/*:test:*/),
                "geo:-60.167000,-24.955000,-5.000".to_string(),
            ),
        ];

        let mut count = 0;
        let should_be_count = tests.len();
        for t in tests {
            let geo_str = format!("{}", t.0);
            assert_eq!(geo_str, t.1);
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }
}
