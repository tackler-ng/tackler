/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config::BalanceType;
use crate::kernel::balance::{BTNs, Balance, Deltas};
use crate::kernel::report_item_selector::{
    BalanceAllSelector, BalanceByAccountSelector, BalanceSelector,
};
use crate::kernel::{BalanceSettings, Settings};
use crate::math::format::format_with_scale;
use crate::model::{BalanceTreeNode, TxnSet};
use crate::report::{Report, write_acc_sel_checksum, write_price_metadata, write_report_timezone};
use crate::tackler;
use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use std::cmp::max;
use std::io;

#[derive(Debug, Clone)]
pub struct BalanceReporter {
    pub report_settings: BalanceSettings,
}

impl TryFrom<&Settings> for BalanceReporter {
    type Error = tackler::Error;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(BalanceReporter {
            report_settings: BalanceSettings::try_from(settings)?,
        })
    }
}

impl BalanceReporter {
    pub(crate) fn acc_selector(ras: &[String]) -> Result<Box<dyn BalanceSelector>, tackler::Error> {
        if ras.is_empty() {
            Ok(Box::<BalanceAllSelector>::default())
        } else {
            let s: Vec<_> = ras.iter().map(|s| s.as_str()).collect();
            let ras = BalanceByAccountSelector::from(&s)?;
            Ok(Box::new(ras))
        }
    }

    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, tackler::Error> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }
}

impl BalanceReporter {
    pub(crate) fn txt_report<W: io::Write + ?Sized>(
        writer: &mut W,
        bal_report: &Balance,
        bal_settings: &BalanceSettings,
    ) -> Result<(), tackler::Error> {
        let get_max_sum_len = |bal: &BTNs, f: fn(&BalanceTreeNode) -> Decimal| -> usize {
            bal.iter()
                .map(|btn| {
                    let d = f(btn);
                    // include space for '+-' to the length always
                    format!("{:+.prec$}", d, prec = bal_settings.scale.get_precision(&d))
                        .chars()
                        .count()
                })
                .fold(0, max)
        };
        fn get_max_delta_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(_, d)| format!("{}", d).chars().count())
                .fold(0, max)
        }
        /// Max used length of commodity could be calculated from deltas
        /// because all balance account commodities are present in there
        fn get_max_commodity_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(opt_comm, _)| {
                    opt_comm
                        .as_ref()
                        .map_or(0, |comm| comm.name.chars().count())
                })
                .fold(0, max)
        }

        let delta_max_len = get_max_delta_len(&bal_report.deltas);
        let comm_max_len = get_max_commodity_len(&bal_report.deltas);

        // max of 12, max_sum_len or delta_max_len
        let left_sum_len = max(
            12,
            max(
                get_max_sum_len(&bal_report.bal, |btn| btn.account_sum),
                delta_max_len,
            ),
        );

        let sub_acc_tree_sum_len = get_max_sum_len(&bal_report.bal, |btn| btn.sub_acc_tree_sum);

        // filler between account sums (acc and accTree sums)
        // width of this filler is mandated by delta sum's max commodity length,
        // because then AccTreesSum won't overlap with delta's commodity
        let filler_field = if comm_max_len.is_zero() {
            " ".repeat(3)
        } else {
            " ".repeat(4 + comm_max_len)
        };

        fn make_commodity_field(
            comm_max_len: usize,
            btn: &BalanceTreeNode,
            bal_settings: &BalanceSettings,
        ) -> String {
            if comm_max_len.is_zero() {
                // This is the space between acc_tree_sum (ACCTS), commodity, account
                match bal_settings.bal_type {
                    BalanceType::Tree => {
                        // -> always separate with two spaces ACCTS and account
                        " ".repeat(2)
                    }
                    BalanceType::Flat => {
                        // no need to separate, as there is filler before account field
                        String::default()
                    }
                }
            } else {
                let comm = &btn.acctn.comm;
                match &comm.is_any() {
                    true => {
                        match bal_settings.bal_type {
                            BalanceType::Tree => {
                                format!(" {: <cl$}  ", comm.name, cl = comm_max_len)
                            }
                            BalanceType::Flat => {
                                // there is filler before this field
                                format!("{: <cl$}  ", comm.name, cl = comm_max_len)
                            }
                        }
                    }
                    false => {
                        match bal_settings.bal_type {
                            BalanceType::Tree => {
                                format!(" {}  ", " ".repeat(comm_max_len))
                            }
                            BalanceType::Flat => {
                                // there is filler before this field
                                format!("{}  ", " ".repeat(comm_max_len))
                            }
                        }
                    }
                }
            }
        }

        let left_ruler = " ".repeat(9);

        writeln!(writer, "{}", bal_report.title)?;
        writeln!(writer, "{}", "-".repeat(bal_report.title.chars().count()))?;

        if !bal_report.is_empty() {
            for btn in &bal_report.bal {
                let acc_sum =
                    format_with_scale(left_sum_len, &btn.account_sum, &bal_settings.scale);
                let comm = make_commodity_field(comm_max_len, btn, bal_settings);
                let atn = &btn.acctn.atn;

                match bal_settings.bal_type {
                    BalanceType::Tree => {
                        writeln!(
                            writer,
                            "{left_ruler}{acc_sum}{filler_field}{acc_tree_sum}{comm}{atn}",
                            acc_tree_sum = format_with_scale(
                                sub_acc_tree_sum_len,
                                &btn.sub_acc_tree_sum,
                                &bal_settings.scale
                            )
                        )?;
                    }
                    BalanceType::Flat => {
                        writeln!(writer, "{left_ruler}{acc_sum}{filler_field}{comm}{atn}")?;
                    }
                }
            }

            writeln!(
                writer,
                "{}",
                "=".repeat(
                    left_ruler.chars().count()
                        + left_sum_len
                        + (if comm_max_len.is_zero() {
                            0
                        } else {
                            comm_max_len + 1
                        })
                )
            )?;

            let deltas = bal_report.deltas.iter().sorted_by_key(|i| {
                i.0.as_ref()
                    .map_or(String::default(), |comm| comm.name.clone())
            });
            for delta in deltas {
                writeln!(
                    writer,
                    "{left_ruler}{}{}",
                    format_with_scale(left_sum_len, delta.1, &bal_settings.scale),
                    delta
                        .0
                        .as_ref()
                        .map_or(String::default(), |c| format!(" {}", &c.name)),
                )?;
            }
        }
        Ok(())
    }
}

impl Report for BalanceReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let bal_acc_sel = self.get_acc_selector()?;

        let price_lookup_ctx = self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            self.report_settings.report_commodity.clone(),
            &cfg.price.price_db,
        );

        write_acc_sel_checksum(cfg, writer, bal_acc_sel.as_ref())?;

        if !price_lookup_ctx.is_empty() {
            write_report_timezone(cfg, writer)?;
        }

        write_price_metadata(cfg, writer, &price_lookup_ctx)?;

        writeln!(writer)?;

        let bal_report = Balance::from(
            &self.report_settings.title,
            txn_data,
            &price_lookup_ctx,
            bal_acc_sel.as_ref(),
            cfg,
        )?;

        BalanceReporter::txt_report(writer, &bal_report, &self.report_settings)?;
        Ok(())
    }
}
