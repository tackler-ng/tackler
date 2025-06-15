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
use crate::report::{FormatWriter, Report, report_timezone};
use crate::tackler;
use crate::tackler::Error;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use std::cmp::max;
use std::io;
use std::io::Write;
use tackler_api::metadata::Metadata;
use tackler_api::metadata::items::{CreditAccountReport, MetadataItem};
use tackler_api::reports::balance_report::{BalanceItem, BalanceReport, Delta};

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
            let s: Vec<_> = ras.iter().map(String::as_str).collect();
            let ras = BalanceByAccountSelector::try_from(&s)?;
            Ok(Box::new(ras))
        }
    }

    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, tackler::Error> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }
}

impl BalanceReporter {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn txt_report<W: io::Write + ?Sized>(
        writer: &mut W,
        bal_report: &Balance,
        bal_settings: &BalanceSettings,
    ) -> Result<(), tackler::Error> {
        fn get_max_delta_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(_, d)| format!("{d}").chars().count())
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

            for delta in &bal_report.deltas {
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

    fn btn_to_api(btn: &BalanceTreeNode, report_settings: &BalanceSettings) -> BalanceItem {
        let acc_sum = match report_settings.bal_type {
            BalanceType::Tree => Some(format_with_scale(
                0,
                &btn.sub_acc_tree_sum,
                &report_settings.scale,
            )),
            BalanceType::Flat => None,
        };
        BalanceItem {
            account_sum: format_with_scale(0, &btn.account_sum, &report_settings.scale),
            account_tree_sum: acc_sum,
            account: btn.acctn.atn.account.clone(),
            commodity: if btn.acctn.comm.is_any() {
                Some(btn.acctn.comm.name.clone())
            } else {
                None
            },
        }
    }

    #[must_use]
    pub fn balance_to_api(
        metadata: Option<&Metadata>,
        bal: &Balance,
        report_settings: &BalanceSettings,
    ) -> BalanceReport {
        let balances = bal
            .bal
            .iter()
            .map(|btn| Self::btn_to_api(btn, report_settings))
            .collect::<Vec<BalanceItem>>();

        let deltas = bal
            .deltas
            .iter()
            .map(|(c, v)| Delta {
                commodity: c.as_ref().map(|c| c.name.clone()),
                delta: format_with_scale(0, v, &report_settings.scale),
            })
            .collect::<Vec<Delta>>();

        BalanceReport {
            metadata: metadata.cloned(),
            title: bal.title.clone(),
            balances,
            deltas,
        }
    }
}

impl Report for BalanceReporter {
    fn write_reports<W: Write + ?Sized>(
        &self,
        cfg: &Settings,
        writers: &mut Vec<FormatWriter<'_>>,
        metadata: Option<&Metadata>,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Error> {
        assert_eq!(self.report_settings.inverted, cfg.inverted);

        let acc_sel = self.get_acc_selector()?;

        let price_lookup_ctx = self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            self.report_settings.report_commodity.clone(),
            &cfg.price.price_db,
        );
        let bal_report = Balance::from(
            &self.report_settings.title,
            txn_data,
            &price_lookup_ctx,
            acc_sel.as_ref(),
            cfg,
        )?;

        let mut metadata = match metadata {
            Some(md) => md.clone(),
            None => Metadata::default(),
        };

        if let Some(hash) = cfg.get_hash() {
            let asc = acc_sel.account_selector_metadata(hash);
            metadata.push(asc);
        }

        if !price_lookup_ctx.is_empty() {
            let rtz = MetadataItem::TimeZoneInfo(report_timezone(cfg)?);
            metadata.push(rtz);

            let pr = MetadataItem::PriceRecords(price_lookup_ctx.metadata());
            metadata.push(pr);
        }

        if self.report_settings.inverted {
            let credit = MetadataItem::CreditAccountReport(CreditAccountReport {});
            metadata.push(credit);
        }

        for w in writers {
            match w {
                FormatWriter::TxtFormat(writer) => {
                    if !metadata.is_empty() {
                        writeln!(writer, "{}\n", metadata.text(cfg.report.tz.clone()))?;
                    }

                    BalanceReporter::txt_report(writer, &bal_report, &self.report_settings)?;
                }
                FormatWriter::JsonFormat(writer) => {
                    let md = if metadata.is_empty() {
                        None
                    } else {
                        Some(&metadata)
                    };
                    serde_json::to_writer_pretty(
                        &mut *writer,
                        &Self::balance_to_api(md, &bal_report, &self.report_settings),
                    )?;
                    writeln!(writer)?;
                }
            }
        }
        Ok(())
    }
}
