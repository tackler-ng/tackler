/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::{export::Export, kernel::Settings, model::TxnSet, tackler};
use std::io;

pub struct AccountsExporter {}

impl Export for AccountsExporter {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        _txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        writeln!(writer, "accounts = [")?;
        for a in cfg.state.accounts.defined_accounts() {
            writeln!(writer, "   \"{}\",", a.1)?;
        }
        writeln!(writer, "]")?;
        Ok(())
    }
}
