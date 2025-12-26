/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::location::{MAX_LAT, MAX_LON, MIN_LAT, MIN_LON};
use rust_decimal::Decimal;

pub(super) mod txn_bbox_lat_lon;
pub(super) mod txn_bbox_lat_lon_alt;

const SOUTH: &str = "south";
const WEST: &str = "west";
const NORTH: &str = "north";
const EAST: &str = "east";
const DEPTH: &str = "depth";
const HEIGHT: &str = "height";

pub(super) fn validate(
    south: Decimal,
    west: Decimal,
    north: Decimal,
    east: Decimal,
) -> Result<(), String> {
    if south > north {
        let msg =
            format!("Invalid Bounding Box: North is below South. South: {south}; North: {north}");
        return Err(msg);
    }
    // The case when east < west is needed for filter over 180th meridian

    /*
       This is checked by south > north
       south > max_lat
       north < min_lat
    */
    if south < MIN_LAT {
        return Err(format!(
            "Invalid Bounding Box: South is beyond pole. South: {south}"
        ));
    }
    if MAX_LAT < north {
        return Err(format!(
            "Invalid Bounding Box: North is beyond pole. North: {north}"
        ));
    }
    if west < MIN_LON || MAX_LON < west {
        return Err(format!(
            "Invalid Bounding Box: West is beyond 180th Meridian. West: {west}"
        ));
    }
    if east < MIN_LON || MAX_LON < east {
        return Err(format!(
            "Invalid Bounding Box: East is beyond 180th Meridian. East: {east}"
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    // (south, west, north, east, expected_error_substring)
    pub(super) fn err_bboxes() -> Vec<(Decimal, Decimal, Decimal, Decimal, String)> {
        #[rustfmt::skip]
        let err_bboxes: Vec<(Decimal, Decimal, Decimal, Decimal, String)> =
            vec![
                (dec!(65.0),  dec!(0), dec!(40),   dec!(0), "North is below South. South: 65.0; North: 40".to_string()),
                (dec!(-2),    dec!(0), dec!(-30.0),dec!(0), "North is below South. South: -2; North: -30.0".to_string()),
                (dec!(22),    dec!(0), dec!(-25),  dec!(0), "North is below South. South: 22; North: -25".to_string()),

                (dec!(-90.1), dec!(0), dec!(0),    dec!(0), "South is beyond pole. South: -90.1".to_string()),
                (dec!(0),     dec!(0), dec!(90.1), dec!(0), "North is beyond pole. North: 90.1".to_string()),

                (dec!(0),     dec!(-180.1), dec!(0), dec!(0), "West is beyond 180th Meridian. West: -180.1".to_string()),
                (dec!(0),     dec!(180.1),  dec!(0), dec!(0), "West is beyond 180th Meridian. West: 180.1".to_string()),

                (dec!(0),     dec!(0), dec!(0), dec!(180.1),  "East is beyond 180th Meridian. East: 180.1".to_string()),
                (dec!(0),     dec!(0), dec!(0), dec!(-180.1), "East is beyond 180th Meridian. East: -180.1".to_string()),
            ];
        err_bboxes
    }
}
