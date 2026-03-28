use std::{collections::HashSet, io};

use crate::{export::Export, kernel::Settings, model::TxnSet, tackler};

pub struct AccountsExporter {}

impl Export for AccountsExporter {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        _cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let mut accounts: HashSet<String> = HashSet::new();
        for txn in &txn_data.txns {
            for post in txn.posts.iter() {
                accounts.insert(format!("{}", post.acctn.atn.account));
            }
        }
        for i in accounts {
            writeln!(writer, "{i}")?;
        }
        Ok(())
    }
}
