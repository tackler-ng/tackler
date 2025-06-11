/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::{FormatType, ReportType};
use crate::kernel::{BalanceGroupSettings, RegisterSettings, Settings};
use crate::model::TxnSet;
use crate::tackler;
pub use balance_group_reporter::BalanceGroupReporter;
pub use balance_reporter::BalanceReporter;
pub use register_reporter::RegisterReporter;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use tackler_api::metadata::Metadata;
use tackler_api::metadata::items::TimeZoneInfo;
use tackler_rs::create_output_file;

mod balance_group_reporter;
mod balance_reporter;
mod register_reporter;

pub enum FormatWriter<'w> {
    TxtFormat(Box<dyn io::Write + 'w>),
    JsonFormat(Box<dyn io::Write + 'w>),
}

pub trait Report {
    /// # Errors
    /// Returns `Err` in case of error
    fn write_txt_report<'w, W: io::Write + ?Sized + 'w>(
        &self,
        cfg: &Settings,
        w: &'w mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), tackler::Error> {
        let mut writers: Vec<FormatWriter<'_>> = vec![FormatWriter::TxtFormat(Box::new(w))];
        self.write_reports::<dyn io::Write>(cfg, &mut writers, None, txn_data)
    }

    /// # Errors
    /// Returns `Err` in case of error
    fn write_reports<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writers: &mut Vec<FormatWriter<'_>>,
        metadata: Option<&Metadata>,
        txns: &TxnSet<'_>,
    ) -> Result<(), tackler::Error>;
}

pub(crate) fn report_timezone(cfg: &Settings) -> Result<TimeZoneInfo, tackler::Error> {
    Ok(TimeZoneInfo {
        zone_id: if let Some(tz) = cfg.report.tz.iana_name() {
            tz.to_string()
        } else {
            let msg = "no name for tz!?!";
            return Err(msg.into());
        },
    })
}

fn report_output<W: io::Write + ?Sized>(
    prog_writer: &mut Option<Box<W>>,
    paths: Vec<(String, String)>,
    title: &str,
) -> Result<(), tackler::Error> {
    if let Some(pw) = prog_writer.as_mut() {
        for p in paths {
            writeln!(pw, "{:>21} ({}) : {}", title, p.0, p.1)?;
        }
    }
    Ok(())
}
type ReportWriters<'w> = (Vec<FormatWriter<'w>>, Vec<(String, String)>);

fn report_writers<'w>(
    output_dir: &Path,
    output_prefix: &str,
    report_type: &ReportType,
    settings: &Settings,
) -> Result<ReportWriters<'w>, tackler::Error> {
    match report_type {
        ReportType::Balance => {
            let mut writers = Vec::new();
            let mut paths = Vec::new();

            for rt in &settings.report.formats {
                match rt {
                    FormatType::Txt => {
                        let (txt_writer, txt_path) =
                            create_output_file(output_dir, output_prefix, "bal", "txt")?;

                        writers.push(FormatWriter::TxtFormat(Box::new(txt_writer)));
                        paths.push(("TEXT".to_string(), txt_path));
                    }
                    FormatType::Json => {
                        let (json_writer, json_path) =
                            create_output_file(output_dir, output_prefix, "bal", "json")?;

                        writers.push(FormatWriter::JsonFormat(Box::new(json_writer)));
                        paths.push(("JSON".to_string(), json_path));
                    }
                }
            }
            Ok((writers, paths))
        }
        ReportType::BalanceGroup => {
            let mut writers = Vec::new();
            let mut paths = Vec::new();

            for rt in &settings.report.formats {
                match rt {
                    FormatType::Txt => {
                        let (txt_writer, txt_path) =
                            create_output_file(output_dir, output_prefix, "balgrp", "txt")?;

                        writers.push(FormatWriter::TxtFormat(Box::new(txt_writer)));
                        paths.push(("TEXT".to_string(), txt_path));
                    }
                    FormatType::Json => {
                        let (json_writer, json_path) =
                            create_output_file(output_dir, output_prefix, "balgrp", "json")?;

                        writers.push(FormatWriter::JsonFormat(Box::new(json_writer)));
                        paths.push(("JSON".to_string(), json_path));
                    }
                }
            }
            Ok((writers, paths))
        }
        ReportType::Register => {
            let mut writers = Vec::new();
            let mut paths = Vec::new();

            for rt in &settings.report.formats {
                match rt {
                    FormatType::Txt => {
                        let (txt_writer, txt_path) =
                            create_output_file(output_dir, output_prefix, "reg", "txt")?;

                        writers.push(FormatWriter::TxtFormat(Box::new(txt_writer)));
                        paths.push(("TEXT".to_string(), txt_path));
                    }
                    FormatType::Json => {
                        let (json_writer, json_path) =
                            create_output_file(output_dir, output_prefix, "reg", "json")?;

                        writers.push(FormatWriter::JsonFormat(Box::new(json_writer)));
                        paths.push(("JSON".to_string(), json_path));
                    }
                }
            }
            Ok((writers, paths))
        }
    }
}

/// # Errors
/// Return `Err` in case of error
#[allow(clippy::too_many_arguments)]
pub fn write_txt_reports<W: io::Write + ?Sized>(
    console_writer: &mut Option<Box<W>>,
    output_dir: Option<&PathBuf>,
    output_prefix: &Option<String>,
    reports: &Vec<ReportType>,
    txn_set: &TxnSet<'_>,
    settings: &Settings,
    prog_writer: &mut Option<Box<W>>,
) -> Result<(), tackler::Error> {
    if !(output_dir.is_some() && output_prefix.is_some() && console_writer.is_none()
        || output_dir.is_none() && output_prefix.is_none() && console_writer.is_some())
    {
        return Err("IE: Logic error, console output is not supported with file ouput".into());
    }

    let report_separator_len = 82;

    let metadata = &txn_set
        .metadata()
        .map(|md| format!("{}\n", md.text(settings.report.tz.clone())))
        .unwrap_or_default();

    if let Some(cw) = console_writer.as_mut() {
        write!(cw, "{metadata}")?;
    }

    for r in reports {
        match r {
            ReportType::Balance => {
                let bal_reporter = BalanceReporter::try_from(settings)?;

                if let (Some(output_name), Some(output_dir)) = (output_prefix, output_dir) {
                    let (mut writers, paths) =
                        report_writers(output_dir, output_name, r, settings)?;

                    bal_reporter.write_reports::<dyn io::Write>(
                        settings,
                        &mut writers,
                        txn_set.metadata(),
                        txn_set,
                    )?;

                    report_output(prog_writer, paths, "Balance Report")?;
                } else {
                    let Some(mut cw) = console_writer.as_mut() else {
                        return Err("IE: Logic error: console output".into());
                    };

                    writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                    bal_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                    writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                }
            }
            ReportType::BalanceGroup => {
                let bal_group_reporter = BalanceGroupReporter {
                    report_settings: BalanceGroupSettings::try_from(settings)?,
                };
                if let (Some(output_name), Some(output_dir)) = (output_prefix, output_dir) {
                    let (mut writers, paths) =
                        report_writers(output_dir, output_name, r, settings)?;

                    bal_group_reporter.write_reports::<dyn io::Write>(
                        settings,
                        &mut writers,
                        txn_set.metadata(),
                        txn_set,
                    )?;
                    report_output(prog_writer, paths, "Balance Group Report")?;
                } else {
                    let Some(mut cw) = console_writer.as_mut() else {
                        return Err("IE: Logic error: console output".into());
                    };

                    writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                    bal_group_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                    writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                }
            }
            ReportType::Register => {
                let reg_reporter = RegisterReporter {
                    report_settings: RegisterSettings::try_from(settings)?,
                };

                if let (Some(output_name), Some(output_dir)) = (output_prefix, output_dir) {
                    let (mut writers, paths) =
                        report_writers(output_dir, output_name, r, settings)?;

                    reg_reporter.write_reports::<dyn io::Write>(
                        settings,
                        &mut writers,
                        txn_set.metadata(),
                        txn_set,
                    )?;

                    report_output(prog_writer, paths, "Register Report")?;
                } else {
                    let Some(mut cw) = console_writer.as_mut() else {
                        return Err("IE: Logic error: console output".into());
                    };

                    writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                    reg_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                    writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                }
            }
        }
    }
    Ok(())
}
