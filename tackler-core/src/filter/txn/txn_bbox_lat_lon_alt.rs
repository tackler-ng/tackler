/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterBBoxLatLonAlt;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterBBoxLatLonAlt {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header.location.as_ref().is_some_and(|point| {
            let res2d = if self.west < self.east {
                self.south <= point.lat
                    && point.lat <= self.north
                    && self.west <= point.lon
                    && point.lon <= self.east
            } else {
                // (west > east) => BBox is over 180th meridian (over antimeridian):
                // 1.1 The left (west) hand side of BBox is actually the longitude of East (+deg)
                // 1.2 The right (east) hand side of BBox is actually the longitude of West (-deg)
                // 2. (+deg) <----- meri_|_dian ----> (-deg)
                // 3. Valid points are
                // 3.1   from left (west) hand to the meridian (180deg)
                // 3.2   from right (east) hand to the meridian (-180deg)
                // 3.3   This is true also if both edges (left and rights) are on
                //       the same sign of Longitude (e.g. box is super slide (>180deg))
                self.south <= point.lat
                    && point.lat <= self.north
                    && (self.west <= point.lon || point.lon <= self.east)
            };
            if res2d {
                match point.alt {
                    Some(z) => self.depth <= z && z <= self.height,
                    None => {
                        // 3d filter, but point has no altitude
                        false
                    }
                }
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::{geo2d3d_tests, make_geo_txn};
    use crate::model::Transaction;
    use rust_decimal_macros::dec;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 607d4e0e-e05b-43cf-87b6-d3cad309be73
    // desc: Filter 3D Txns
    fn txn_bbox_lat_lon() {
        let tf = TxnFilterBBoxLatLonAlt {
            south: dec!(40.0),
            west: dec!(20.0),
            depth: dec!(-2000.0),
            north: dec!(65.0),
            east: dec!(26.0),
            height: dec!(14000.0),
        };

        // test: 00d5f743-4eca-4d06-a5e5-4de035909828
        // desc: 3D filter doesn't filter 2D txns
        // test: d6764e33-f20c-4c50-8452-d249d1f0c902
        // desc: check altitude functionality
        let cases: Vec<(Transaction, bool)> = vec![
            (make_geo_txn(dec!(60.0), dec!(24.0), None), false),
            (
                make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(-2001.0))),
                false,
            ),
            (
                make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(-2000.0))),
                true,
            ),
            (make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(0.0))), true),
            (make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(1.0))), true),
            (
                make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(14000.0))),
                true,
            ),
            (
                make_geo_txn(dec!(60.0), dec!(24.0), Some(dec!(14001.0))),
                false,
            ),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: 5405a3cd-504f-4668-af57-563cbbe10298
        // desc: TxnFilter::TxnFilterBBoxLatLonAlt
        let filt = TxnFilter::TxnFilterBBoxLatLonAlt(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }

    #[test]
    // test: 9aa6d324-3bcc-4fcd-ac75-2447f3a65d3b
    // desc: 3D check edge cases (points and/or BBoxes
    fn geo_3d_check_edge_cases() {
        let tests = geo2d3d_tests();

        let mut top_count = 0usize;

        for (expected_count, _bbox2d, bbox3d, tvecs) in &tests {
            let mut inner_count = 0usize;
            for v in tvecs {
                let tf = TxnFilterBBoxLatLonAlt {
                    south: bbox3d.lat1,
                    west: bbox3d.lon1,
                    depth: bbox3d.min_z,
                    north: bbox3d.lat2,
                    east: bbox3d.lon2,
                    height: bbox3d.max_z,
                };

                let txn = make_geo_txn(v.lat, v.lon, v.z);

                assert_eq!(tf.eval(&txn), v.inside_3d);
                let filt = TxnFilter::TxnFilterBBoxLatLonAlt(tf);
                let filtered = filt.eval(&txn);
                assert_eq!(
                    filtered, v.inside_3d,
                    "filter result differed for bbox {filt:?} on vector {v:?}"
                );
                inner_count += 1;
            }

            assert_eq!(
                *expected_count, inner_count,
                "test vector size for one filter is wrong"
            );
            top_count += 1;
        }
        assert_eq!(top_count, 7, "test count for filter is wrong");
    }
}
