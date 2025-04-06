/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::accumulator::TxnGroupByOp;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::kernel::{BalanceGroupSettings, accumulator};
use crate::kernel::{BalanceSettings, Settings};
use crate::model::{Transaction, TxnSet};
use crate::report::{BalanceReporter, write_price_metadata};
use crate::report::{Report, write_acc_sel_checksum, write_report_timezone};
use crate::tackler;
use jiff::tz::TimeZone;
use std::io;
use tackler_api::metadata::Metadata;
use tackler_api::reports::balance_group_report::BalanceGroupReport;
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;
use crate::kernel::balance::Balance;
use crate::report::balance_reporter::balance_to_api;

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
    fn to_api(&self, metadata: Option<Metadata>, bal_groups: &[Balance]) -> BalanceGroupReport {
        let groups = bal_groups.iter().map(balance_to_api).collect();

        BalanceGroupReport {
            metadata,
            title: self.report_settings.title.clone(),
            groups,
        }
    }

}


impl Report for BalanceGroupReporter {
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

        let group_by_op = self.get_group_by_op();
        let bal_groups = accumulator::balance_groups(
            &txn_data.txns,
            group_by_op,
            &price_lookup_ctx,
            bal_acc_sel.as_ref(),
            cfg,
        );

        write_acc_sel_checksum(cfg, writer, bal_acc_sel.as_ref())?;

        write_report_timezone(cfg, writer)?;

        write_price_metadata(cfg, writer, &price_lookup_ctx)?;

        writeln!(writer)?;
        writeln!(writer)?;

        let title = &self.report_settings.title;
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

        let bal_settings = BalanceSettings {
            title: String::default(),
            bal_type: self.report_settings.bal_type.clone(),
            ras: vec![],
            scale: self.report_settings.scale.clone(),
            report_commodity: self.report_settings.report_commodity.clone(),
            price_lookup: self.report_settings.price_lookup.clone(),
        };
        for bal in &bal_groups {
            BalanceReporter::txt_report(writer, bal, &bal_settings)?
        }

        //serde_json::to_writer_pretty(&mut *writer, &self.to_api(None, &bal_groups))?;
        //writeln!(writer)?;

        Ok(())
    }
}
