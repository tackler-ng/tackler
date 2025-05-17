/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::export::Export;
use crate::kernel::Settings;
use crate::kernel::balance::Balance;
use crate::kernel::price_lookup::PriceLookupCtx;
use crate::kernel::report_item_selector::{
    BalanceNonZeroByAccountSelector, BalanceNonZeroSelector, BalanceSelector,
};
use crate::model::{Transaction, TxnSet};
use crate::tackler;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::io;
use tackler_api::metadata::items::{AccountSelectorChecksum, Text};
use tackler_api::txn_ts::rfc_3339;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EquitySettings {
    pub eqa: Option<String>,
    pub ras: Vec<String>,
}

impl EquitySettings {
    #[must_use]
    pub fn from(settings: &Settings) -> EquitySettings {
        EquitySettings {
            eqa: Some(settings.export.equity.equity_account.clone()),
            ras: settings.get_equity_ras(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EquityExporter {
    pub export_settings: EquitySettings,
}

impl EquityExporter {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, tackler::Error> {
        let v = &self.export_settings.ras;
        if v.is_empty() {
            Ok(Box::new(BalanceNonZeroSelector {}))
        } else {
            let s: Vec<_> = v.iter().map(String::as_str).collect();
            let ras = BalanceNonZeroByAccountSelector::try_from(&s)?;

            Ok(Box::new(ras))
        }
    }
}

impl Export for EquityExporter {
    #[allow(clippy::too_many_lines)]
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let bal_acc_sel = self.get_acc_selector()?;

        let bal = Balance::from(
            &String::default(),
            txn_data,
            &PriceLookupCtx::default(),
            bal_acc_sel.as_ref(),
            cfg,
        )?;

        if bal.is_empty() {
            // todo: check if this is actually possible?
            return Ok(());
        }

        let eq_txn_indent = "   ";
        let equity_account = "Equity:Default·Account".to_string();

        let hdr_str = |last_txn: Option<&&Transaction>, c: &String| -> String {
            let comm_str = || -> String {
                if c.is_empty() {
                    String::default()
                } else {
                    format!(" for {c}")
                }
            };
            let txn_uuid_str = |uuid: Option<Uuid>| -> String {
                match uuid {
                    Some(u) => format!(": last txn (uuid): {u}"),
                    None => String::default(),
                }
            };
            match last_txn {
                Some(txn) => {
                    format!(
                        "{} 'Equity{}{}",
                        rfc_3339(&txn.header.timestamp),
                        comm_str(),
                        txn_uuid_str(txn.header.uuid)
                    )
                }
                _ => {
                    "Internal logic error".to_string() // todo: fix this
                }
            }
        };

        let last_txn = txn_data.txns.last();

        let acc_sel_checksum = cfg.get_hash().map(|hash| AccountSelectorChecksum {
            hash: bal_acc_sel.checksum(hash),
            selectors: bal_acc_sel.selectors(),
        });

        let equity_txn_str: Vec<String> = bal
            .bal
            .iter()
            .chunk_by(|btn| &btn.acctn.comm.name)
            .into_iter()
            .flat_map(|(c, bs)| {
                let btns: Vec<_> = bs.collect();
                let dsum: Decimal = btns.clone().into_iter().map(|b| b.account_sum).sum();
                let bal_posting = {
                    let value = if c.is_empty() {
                        format!("{}", -dsum)
                    } else {
                        format!("{} {}", -dsum, c)
                    };
                    let ea = match &self.export_settings.eqa {
                        Some(eqa) => eqa,
                        None => &equity_account,
                    };
                    format!("{eq_txn_indent}{ea}  {value}")
                };
                /*
                 * equity transaction per commodity
                 */
                let eq_postings = btns
                    .into_iter()
                    .map(|b| {
                        let comm = &b.acctn.comm;
                        format!(
                            "{}{}  {}{}",
                            eq_txn_indent,
                            b.acctn.atn.account,
                            b.account_sum,
                            if comm.is_any() { format!(" {}", comm.name) } else { String::new() }
                        )
                    })
                    .collect::<Vec<String>>();

                let mut eq_txn = Vec::<String>::new();

                eq_txn.push(hdr_str(last_txn, c));
                if let Some(md) = &txn_data.metadata {
                        for mdi in md.items.clone() {
                            eq_txn.extend(mdi.text(cfg.report.tz.clone()).iter().map(|v| {
                                format!("{eq_txn_indent}; {v}")
                            }).collect::<Vec<_>>());
                            eq_txn.push(format!("{eq_txn_indent}; "));
                        }

                        if let Some(asc) = &acc_sel_checksum {
                            for v in asc.text(cfg.report.tz.clone()) {
                                eq_txn.push(format!("{}; {}", eq_txn_indent, &v));
                            }
                            eq_txn.push(format!("{eq_txn_indent}; "));
                        }
                }
                if dsum.is_zero() {
                    eq_txn.push(format!("{eq_txn_indent}; WARNING:"));
                    eq_txn.push(format!("{eq_txn_indent}; WARNING: The sum of equity transaction is zero without equity account."));
                    eq_txn.push(format!("{eq_txn_indent}; WARNING: Therefore there is no equity posting row, and this is probably not right."));
                    eq_txn.push(format!("{eq_txn_indent}; WARNING: Is the account selector correct for this Equity export?"));
                    eq_txn.push(format!("{eq_txn_indent}; WARNING:"));
                }

                eq_txn.extend(eq_postings);
                if !dsum.is_zero() {
                    eq_txn.push(bal_posting);
                }
                eq_txn.push(String::new());
                eq_txn
            })
            .collect();

        for i in equity_txn_str {
            writeln!(writer, "{i}")?;
        }

        Ok(())
    }
}
