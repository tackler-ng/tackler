/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use std::{cmp::Ordering, collections::BTreeSet, io};

use crate::{export::Export, kernel::Settings, model::TxnSet, tackler};

pub struct AccountsExporter {}

/// Wrapper struct to order account names.
/// Compares 2 account names piecewise splitting on ':'
///
/// If `account_name_a` is a prefix of `account_name_b` then `account_name_a < account_name_b`
#[derive(PartialEq, Eq, Hash, PartialOrd)]
struct AccountName<'a>(Vec<&'a str>);

impl<'a> From<&'a String> for AccountName<'a> {
    fn from(value: &'a String) -> Self {
        Self(value.split(':').collect())
    }
}

impl std::fmt::Display for AccountName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(":"))
    }
}

impl Ord for AccountName<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (acc_a, acc_b) in self.0.iter().zip(other.0.iter()) {
            match acc_a.cmp(acc_b) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        // So far subcomponents are equal, but which one has more subcomponents?
        self.0.len().cmp(&other.0.len())
    }
}

impl Export for AccountsExporter {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        _cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        // This can't be a String based set because natural sort won't work with numerical accounts
        // Below is natural sort order for the following account names:
        //    "E:01234567:Sweets:Ice·Cream",
        //    "E:0123:567:Sweets:Ice·Cream",
        // which is in wrong order ("E:0123:..." should come first)
        let mut accounts: BTreeSet<AccountName<'_>> = BTreeSet::new();
        for txn in &txn_data.txns {
            for post in &txn.posts {
                accounts.insert(AccountName::from(&post.acctn.atn.account));
            }
        }

        writeln!(writer, "accounts = [")?;

        for i in accounts {
            writeln!(writer, "   \"{i}\",")?;
        }
        writeln!(writer, "]")?;
        Ok(())
    }
}
