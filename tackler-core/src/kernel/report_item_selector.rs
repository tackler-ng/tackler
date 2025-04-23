/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::Predicate;
use crate::kernel::hash::Hash;
use crate::model::{BalanceTreeNode, RegisterPosting};
use crate::tackler;
use regex::RegexSet;
use tackler_api::metadata::Checksum;
use tackler_api::metadata::items::{AccountSelectorChecksum, MetadataItem};
use tackler_rs::regex::{new_full_haystack_regex_set, peeled_patterns};

pub trait ReportItemSelector {
    fn checksum(&self, _: Hash) -> Result<Checksum, tackler::Error>;
    fn account_selector_checksum(&self, hash: Hash) -> Result<MetadataItem, tackler::Error> {
        let asc = MetadataItem::AccountSelectorChecksum(AccountSelectorChecksum {
            hash: self.checksum(hash)?,
        });
        Ok(asc)
    }
}

pub trait BalanceItemSelector: Predicate<BalanceTreeNode> {}
pub trait BalanceSelector: BalanceItemSelector + ReportItemSelector {}

#[derive(Default)]
pub struct BalanceAllSelector {}

impl BalanceSelector for BalanceAllSelector {}
impl BalanceItemSelector for BalanceAllSelector {}

impl ReportItemSelector for BalanceAllSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, tackler::Error> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all".to_string(),
        })
    }
}

impl Predicate<BalanceTreeNode> for BalanceAllSelector {
    fn eval(&self, _: &BalanceTreeNode) -> bool {
        true
    }
}

#[derive(Default)]
pub struct BalanceNonZeroSelector {}
impl BalanceSelector for BalanceNonZeroSelector {}
impl BalanceItemSelector for BalanceNonZeroSelector {}

impl ReportItemSelector for BalanceNonZeroSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, tackler::Error> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all non-zero".to_string(),
        })
    }
}

impl Predicate<BalanceTreeNode> for BalanceNonZeroSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        !btn.account_sum.is_zero()
    }
}

pub struct BalanceNonZeroByAccountSelector {
    acc_sel: BalanceByAccountSelector,
}
impl BalanceSelector for BalanceNonZeroByAccountSelector {}
impl BalanceItemSelector for BalanceNonZeroByAccountSelector {}

impl ReportItemSelector for BalanceNonZeroByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, tackler::Error> {
        self.acc_sel.checksum(hash)
    }
}

impl Predicate<BalanceTreeNode> for BalanceNonZeroByAccountSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        !btn.account_sum.is_zero() && self.acc_sel.eval(btn)
    }
}

impl BalanceNonZeroByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<BalanceNonZeroByAccountSelector, tackler::Error> {
        let bfa = BalanceByAccountSelector {
            regexs: new_full_haystack_regex_set(patterns)?,
        };
        let bnza = BalanceNonZeroByAccountSelector { acc_sel: bfa };
        Ok(bnza)
    }
}

pub struct BalanceByAccountSelector {
    regexs: RegexSet,
}

impl BalanceByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<BalanceByAccountSelector, tackler::Error> {
        let bfa = BalanceByAccountSelector {
            regexs: new_full_haystack_regex_set(patterns)?,
        };
        Ok(bfa)
    }
}

impl BalanceItemSelector for BalanceByAccountSelector {}
impl BalanceSelector for BalanceByAccountSelector {}

impl Predicate<BalanceTreeNode> for BalanceByAccountSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        self.regexs.is_match(&btn.acctn.atn.account)
    }
}

impl ReportItemSelector for BalanceByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, tackler::Error> {
        let mut accsel = peeled_patterns(&self.regexs);
        accsel.sort();
        let h = hash.checksum(&accsel, "\n".as_bytes())?;
        Ok(h)
    }
}

pub trait RegisterItemSelector<'a>: Predicate<RegisterPosting<'a>> {}
pub trait RegisterSelector<'a>: RegisterItemSelector<'a> + ReportItemSelector {}

pub struct RegisterByAccountSelector {
    regexs: RegexSet,
}

impl RegisterByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<RegisterByAccountSelector, tackler::Error> {
        let ras = RegisterByAccountSelector {
            regexs: new_full_haystack_regex_set(patterns)?,
        };
        Ok(ras)
    }
}

impl RegisterSelector<'_> for RegisterByAccountSelector {}
impl RegisterItemSelector<'_> for RegisterByAccountSelector {}

impl Predicate<RegisterPosting<'_>> for RegisterByAccountSelector {
    fn eval(&self, rep: &RegisterPosting<'_>) -> bool {
        self.regexs.is_match(&rep.post.acctn.atn.account)
    }
}

impl ReportItemSelector for RegisterByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, tackler::Error> {
        let mut accsel = peeled_patterns(&self.regexs);
        accsel.sort();
        let h = hash.checksum(&accsel, "\n".as_bytes())?;
        Ok(h)
    }
}

#[derive(Default)]
pub struct RegisterAllSelector {}

impl Predicate<RegisterPosting<'_>> for RegisterAllSelector {
    fn eval(&self, _: &RegisterPosting<'_>) -> bool {
        true
    }
}

impl RegisterItemSelector<'_> for RegisterAllSelector {}
impl RegisterSelector<'_> for RegisterAllSelector {}

impl ReportItemSelector for RegisterAllSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, tackler::Error> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all".to_string(),
        })
    }
}
