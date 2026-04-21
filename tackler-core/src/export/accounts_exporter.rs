/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::model::AccountTreeNode;
use crate::{export::Export, kernel::Settings, model::TxnSet, tackler};
use std::sync::Arc;
use std::{collections::BTreeSet, io};

pub struct AccountsExporter {}

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
        let mut accounts: BTreeSet<Arc<AccountTreeNode>> = BTreeSet::new();
        for txn in &txn_data.txns {
            for post in &txn.posts {
                accounts.insert(post.acctn.atn.clone());
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
