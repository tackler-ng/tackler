/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::accumulator::TxnGroupByOp;
use crate::kernel::balance::Balance;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::kernel::{BalanceGroupSettings, accumulator};
use crate::kernel::{BalanceSettings, Settings};
use crate::model::{Transaction, TxnSet};
use crate::report::Report;
use crate::report::{BalanceReporter, FormatWriter, report_timezone};
use crate::tackler;
use crate::tackler::Error;
use jiff::tz::TimeZone;
use std::io;
use std::io::Write;
use tackler_api::metadata::Metadata;
use tackler_api::metadata::items::MetadataItem;
use tackler_api::reports::balance_group_report::BalanceGroupReport;
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;

#[derive(Debug, Clone)]
pub struct BalanceGroupReporter {
    pub report_settings: BalanceGroupSettings,
}

impl BalanceGroupReporter {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, tackler::Error> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }

    fn get_group_by_op(&self) -> TxnGroupByOp<'_> {
        let tz: TimeZone = self.report_settings.report_tz.clone();
        match self.report_settings.group_by {
            GroupBy::IsoWeekDate => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_iso_week_date(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::IsoWeek => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_iso_week(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Date => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_date(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Month => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_month(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Year => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_year(&txn.header.timestamp, tz.clone())
            }),
        }
    }

    #[allow(dead_code)]
    fn to_api(&self, metadata: Option<&Metadata>, bal_groups: &[Balance]) -> BalanceGroupReport {
        let bal_settings: BalanceSettings = self.report_settings.clone().into();
        let groups = bal_groups
            .iter()
            .map(|bg| BalanceReporter::balance_to_api(None, bg, &bal_settings))
            .collect();

        BalanceGroupReport {
            metadata: metadata.cloned(),
            title: self.report_settings.title.clone(),
            groups,
        }
    }
}

impl Report for BalanceGroupReporter {
    fn write_reports<W: Write + ?Sized>(
        &self,
        cfg: &Settings,
        writers: &mut Vec<FormatWriter<'_>>,
        metadata: Option<&Metadata>,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Error> {
        let acc_sel = self.get_acc_selector()?;

        let price_lookup_ctx = self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            self.report_settings.report_commodity.clone(),
            &cfg.price.price_db,
        );

        let group_by_op = self.get_group_by_op();
        let bal_groups = accumulator::balance_groups(
            &txn_data.txns,
            group_by_op,
            &price_lookup_ctx,
            acc_sel.as_ref(),
            cfg,
        );

        let mut metadata = match metadata {
            Some(md) => md.clone(),
            None => Metadata::default(),
        };

        if let Some(hash) = cfg.get_hash() {
            let asc = acc_sel.account_selector_checksum(hash)?;
            metadata.push(asc);
        }

        let rtz = MetadataItem::TimeZoneInfo(report_timezone(cfg)?);
        metadata.push(rtz);

        if !price_lookup_ctx.is_empty() {
            let pr = MetadataItem::PriceRecords(price_lookup_ctx.metadata());
            metadata.push(pr);
        }

        for w in writers {
            match w {
                FormatWriter::TxtFormat(writer) => {
                    // There is always at least TimeZoneInfo
                    writeln!(writer, "{}\n", metadata.text(cfg.report.report_tz.clone()))?;

                    let title = &self.report_settings.title;
                    writeln!(writer, "{}", title)?;
                    writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

                    let bal_settings = self.report_settings.clone().into();
                    for bal in &bal_groups {
                        BalanceReporter::txt_report(writer, bal, &bal_settings)?
                    }
                }
                FormatWriter::JsonFormat(writer) => {
                    let md = if metadata.is_empty() {
                        None
                    } else {
                        Some(&metadata)
                    };

                    serde_json::to_writer_pretty(&mut *writer, &self.to_api(md, &bal_groups))?;
                    writeln!(writer)?;
                }
            }
        }
        Ok(())
    }

    fn write_txt_report<'w, W: io::Write + ?Sized + 'w>(
        &self,
        cfg: &Settings,
        writer: &'w mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let mut writers: Vec<FormatWriter<'_>> = vec![FormatWriter::TxtFormat(Box::new(writer))];
        self.write_reports::<dyn io::Write>(cfg, &mut writers, None, txn_data)
    }
}
