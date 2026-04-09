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
        Self(value.split(":").collect())
    }
}

impl<'a> std::fmt::Display for AccountName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(":"))
    }
}

impl<'a> Ord for AccountName<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (acc_a, acc_b) in self.0.iter().zip(other.0.iter()) {
            match acc_a.cmp(acc_b) {
                Ordering::Equal => continue,
                ordering => return ordering,
            }
        }

        if self.0.len() > other.0.len() {
            Ordering::Greater
        } else if self.0.len() < other.0.len() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Export for AccountsExporter {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        _cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let mut accounts: BTreeSet<AccountName<'_>> = BTreeSet::new();
        for txn in &txn_data.txns {
            for post in txn.posts.iter() {
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
