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
use crate::math::format::format_with_scale;
use crate::model::{RegisterEntry, TxnSet};
use crate::report::{FormatWriter, Report, report_timezone};
use crate::tackler;
use crate::tackler::Error;
use jiff::Zoned;
use jiff::tz::TimeZone;
use rust_decimal::Decimal;
use std::io;
use std::io::Write;
use tackler_api::metadata::Metadata;
use tackler_api::metadata::items::{CreditAccountReport, MetadataItem};
use tackler_api::reports::register_report::{RegisterPosting, RegisterReport, RegisterTxn};
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
            let s: Vec<_> = ras.iter().map(String::as_str).collect();
            let ras = RegisterByAccountSelector::try_from(&s)?;

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

fn register_entry_to_api(
    re: &RegisterEntry<'_>,
    register_settings: &RegisterSettings,
) -> Option<RegisterTxn> {
    if re.posts.is_empty() {
        return None;
    }

    let inverter = Decimal::from(-1);

    let scale = &register_settings.scale;
    let ts_style = register_settings.timestamp_style;
    let report_tz = register_settings.report_tz.clone();

    let fmt: fn(&Zoned, TimeZone) -> String = match ts_style {
        TimestampStyle::Date => txn_ts::as_tz_date,
        TimestampStyle::Secodns => txn_ts::as_tz_seconds,
        TimestampStyle::Full => txn_ts::as_tz_full,
    };

    let txn = re.txn;

    let r = re
        .posts
        .iter()
        .map(|p| {
            let commodity = if p.target_commodity.is_any() {
                Some(p.target_commodity.name.clone())
            } else {
                None
            };
            let base_commodity = if p.is_commodity_conv() {
                Some(p.post.acctn.comm.name.clone())
            } else {
                None
            };

            let (a, rt) = if register_settings.inverted {
                (&(p.post.amount * inverter), &(p.amount * inverter))
            } else {
                (&p.post.amount, &p.amount)
            };

            RegisterPosting {
                account: p.post.acctn.atn.account.clone(),
                amount: format_with_scale(0, a, scale),
                running_total: format_with_scale(0, rt, scale),
                commodity,
                rate: p.rate.map(|r| format_with_scale(0, &r, scale)),
                base_commodity,
            }
        })
        .collect();

    Some(RegisterTxn {
        display_time: fmt(&txn.header.timestamp, report_tz),
        txn: txn.header.clone(),
        postings: r,
    })
}

impl Report for RegisterReporter {
    fn write_reports<W: Write + ?Sized>(
        &self,
        cfg: &Settings,
        writers: &mut Vec<FormatWriter<'_>>,
        metadata: Option<&Metadata>,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Error> {
        assert_eq!(self.report_settings.inverted, cfg.inverted);

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
            let asc = acc_sel.account_selector_metadata(hash);
            metadata.push(asc);
        }

        let rtz = MetadataItem::TimeZoneInfo(report_timezone(cfg)?);
        metadata.push(rtz);

        if !price_lookup_ctx.is_empty() {
            let pr = MetadataItem::PriceRecords(price_lookup_ctx.metadata());
            metadata.push(pr);
        }

        if self.report_settings.inverted {
            let credit = MetadataItem::CreditAccountReport(CreditAccountReport {});
            metadata.push(credit);
        }

        let ras = self.get_acc_selector()?;

        let register =
            accumulator::register_engine(&txn_data.txns, &price_lookup_ctx, ras.as_ref());

        for w in writers {
            match w {
                FormatWriter::TxtFormat(writer) => {
                    // There is always at least TimeZoneInfo
                    writeln!(writer, "{}\n", metadata.text(cfg.report.tz.clone()))?;

                    let title = &self.report_settings.title;
                    writeln!(writer, "{title}")?;
                    writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

                    for re in &register {
                        reg_entry_txt_writer(writer, re, &self.report_settings)?;
                    }
                }
                FormatWriter::JsonFormat(writer) => {
                    let transactions = register
                        .iter()
                        .filter_map(|re| register_entry_to_api(re, &self.report_settings))
                        .collect();

                    let rr = RegisterReport {
                        metadata: Some(metadata.clone()), // at least TimeZoneInfo
                        title: self.report_settings.title.clone(),
                        transactions,
                    };
                    serde_json::to_writer_pretty(&mut *writer, &rr)?;
                    writeln!(writer)?;
                }
            }
        }
        Ok(())
    }
}
