/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterBBoxLatLon;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterBBoxLatLon {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header.location.as_ref().is_some_and(|point| {
            if self.west < self.east {
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
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_geo_txn;
    use crate::model::Transaction;
    use rust_decimal_macros::dec;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 320d92b9-f8e7-4283-9296-74ff4340fff1
    // desc: Filter 2D Txns
    fn txn_bbox_lat_lon() {
        let tf = TxnFilterBBoxLatLon {
            south: dec!(40.0),
            west: dec!(20.0),
            north: dec!(65.0),
            east: dec!(26.0),
        };

        // test: 05983503-0aa4-42e1-a1c9-cc5df93285f7
        // desc: 3D txn with 2D filter
        let cases: Vec<(Transaction, bool)> = vec![
            (make_geo_txn(dec!(0.0), dec!(0.0), None), false),
            (make_geo_txn(dec!(60.170833), dec!(24.9375), None), true),
            (
                make_geo_txn(dec!(60.170833), dec!(24.9375), Some(dec!(10.0))),
                true,
            ),
            (make_geo_txn(dec!(39.0), dec!(23.0), None), false),
            (make_geo_txn(dec!(66.0), dec!(23.0), None), false),
            (make_geo_txn(dec!(50.0), dec!(19.0), None), false),
            (make_geo_txn(dec!(50.0), dec!(27.0), None), false),
        ];

        for t in &cases {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: 78412bff-ef54-41e5-aed8-c1bf6965a4e6
        // desc: TxnFilter::TxnFilterBBoxLatLon
        let filt = TxnFilter::TxnFilterBBoxLatLon(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
