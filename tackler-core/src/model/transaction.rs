/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::{Posts, posting};
use crate::tackler;
use jiff::tz;
use std::cmp::Ordering;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use tackler_api::txn_header::TxnHeader;
use tackler_api::txn_ts;

#[derive(Debug, Default)]
pub struct Transaction {
    pub(crate) header: TxnHeader,
    pub(crate) posts: Posts,
}

impl Transaction {
    /// # Errors
    /// Returns `Err` if transaction sum is not zero
    pub fn try_from(header: TxnHeader, posts: Posts) -> Result<Transaction, tackler::Error> {
        let txn_sum = posting::txn_sum(&posts);
        if !txn_sum.is_zero() {
            let msg = format!("TXN postings do not zero: {txn_sum}");
            return Err(msg.into());
        }

        Ok(Transaction { header, posts })
    }
}

#[must_use]
pub fn ord_by_txn(before: &Transaction, after: &Transaction) -> Ordering {
    before.cmp(after)
}

impl Eq for Transaction {}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.header.cmp(&other.header)
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = "   ";
        write!(
            f,
            "{}{}",
            self.header.to_string_with_indent(
                indent,
                |ts, _tz| { txn_ts::rfc_3339(ts) },
                tz::TimeZone::UTC
            ),
            self.posts
                .iter()
                .fold(String::with_capacity(256), |mut output, p| {
                    let _ = writeln!(output, "{indent}{p}");
                    output
                })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use tackler_rs::IndocUtils;

    use crate::model::TxnAccount;
    use crate::model::{AccountTreeNode, Commodity, Posting};
    use tackler_api::txn_header::TxnHeader;
    use tackler_api::txn_ts::rfc3339_to_zoned;

    fn atn2txntn(atn: AccountTreeNode) -> TxnAccount {
        TxnAccount {
            atn: Arc::new(atn),
            comm: Arc::new(Commodity::default()),
        }
    }

    #[test]
    fn txn_to_display() {
        let ts = rfc3339_to_zoned("2023-02-04T14:03:05.047974+02:00").unwrap(/*:test:*/);

        let tnx_hdr = TxnHeader {
            timestamp: ts,
            code: None,
            description: Some("desc".to_string()),
            uuid: None,
            location: None,
            tags: None,
            comments: None,
        };

        let atn_ab = AccountTreeNode::from("a:b").unwrap(/*:test:*/);
        let atn_cd = AccountTreeNode::from("c:d").unwrap(/*:test:*/);
        let atn_ef = AccountTreeNode::from("e:f").unwrap(/*:test:*/);

        let ef_post = Posting::from(
            atn2txntn(atn_ef),
            Decimal::from_str_exact("1").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            Arc::new(Commodity::default()),
            None,
        )
        .unwrap(/*:test:*/);
        let cd_post = Posting::from(
            atn2txntn(atn_cd),
            Decimal::from_str_exact("2").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            Arc::new(Commodity::default()),
            None,
        )
        .unwrap(/*:test:*/);
        let ab_post = Posting::from(
            atn2txntn(atn_ab),
            Decimal::from_str_exact("-3").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            Arc::new(Commodity::default()),
            None,
        )
        .unwrap(/*:test:*/);

        let tests: Vec<(Transaction, String)> = vec![(
            Transaction {
                header: tnx_hdr,
                posts: vec![ef_post, cd_post, ab_post],
            },
            indoc!(
                "|2023-02-04T14:03:05.047974+02:00 'desc
                 |   e:f   1
                 |   c:d   2
                 |   a:b  -3
                 |"
            )
            .strip_margin(),
        )];

        let mut count = 0;
        let should_be_count = tests.len();
        for t in tests {
            let txn_hdr_str = format!("{}", t.0);
            assert_eq!(txn_hdr_str, t.1);
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }
}
