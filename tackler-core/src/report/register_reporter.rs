/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::Settings;
use crate::kernel::accumulator;
use crate::kernel::report_item_selector::{
    RegisterAllSelector, RegisterByAccountSelector, RegisterSelector,
};
use crate::kernel::report_settings::RegisterSettings;
use crate::model::{RegisterEntry, TxnSet};
use crate::report::{FormatWriter, Report, report_timezone};
use crate::tackler;
use crate::tackler::Error;
use jiff::Zoned;
use jiff::tz::TimeZone;
use std::io;
use std::io::Write;
use tackler_api::metadata::Metadata;
use tackler_api::metadata::items::MetadataItem;
use tackler_api::txn_ts;
use tackler_api::txn_ts::TimestampStyle;

#[derive(Debug, Clone)]
pub struct RegisterReporter {
    pub report_settings: RegisterSettings,
}

impl RegisterReporter {
    fn get_acc_selector(&self) -> Result<Box<dyn RegisterSelector<'_>>, tackler::Error> {
        let ras = &self.report_settings.ras;
        if ras.is_empty() {
            Ok(Box::<RegisterAllSelector>::default())
        } else {
            let s: Vec<_> = ras.iter().map(|s| s.as_str()).collect();
            let ras = RegisterByAccountSelector::from(&s)?;

            Ok(Box::new(ras))
        }
    }
}

fn reg_entry_txt_writer<W: io::Write + ?Sized>(
    f: &mut W,
    re: &RegisterEntry<'_>,
    register_settings: &RegisterSettings,
) -> Result<(), tackler::Error> {
    let ts_style = register_settings.timestamp_style;
    let report_tz = register_settings.report_tz.clone();

    let fmt: fn(&Zoned, TimeZone) -> String = match ts_style {
        TimestampStyle::Date => txn_ts::as_tz_date,
        TimestampStyle::Secodns => txn_ts::as_tz_seconds,
        TimestampStyle::Full => txn_ts::as_tz_full,
    };

    if !re.posts.is_empty() {
        write!(f, "{}", re.fmt_with_cfg(fmt, report_tz, register_settings))?;
    }
    Ok(())
}

impl Report for RegisterReporter {
    fn write_reports<W: Write + ?Sized>(
        &self,
        cfg: &Settings,
        writers: &mut Vec<FormatWriter<'_>>,
        metadata: Option<&Metadata>,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Error> {
        let acc_sel = self.get_acc_selector()?;

        let report_commodity = self.report_settings.report_commodity.clone();
        let price_lookup_ctx = self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            report_commodity,
            &cfg.price.price_db,
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

                    let ras = self.get_acc_selector()?;

                    accumulator::register_engine(
                        &txn_data.txns,
                        &price_lookup_ctx,
                        ras.as_ref(),
                        writer,
                        reg_entry_txt_writer,
                        &self.report_settings,
                    )?;
                }
                FormatWriter::JsonFormat(_writer) => {
                    unimplemented!();
                }
            }
        }
        Ok(())
    }
}
