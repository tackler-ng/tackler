/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::TxnAccount;
use rust_decimal::Decimal;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct BalanceTreeNode {
    pub(crate) acctn: TxnAccount,
    pub(crate) sub_acc_tree_sum: Decimal,
    pub(crate) account_sum: Decimal,
}

pub(crate) fn ord_by_btn(before: &BalanceTreeNode, after: &BalanceTreeNode) -> Ordering {
    before.cmp(after)
}

impl Ord for BalanceTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.acctn.cmp(&other.acctn)
    }
}

impl PartialOrd for BalanceTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BalanceTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.acctn == other.acctn
    }
}
