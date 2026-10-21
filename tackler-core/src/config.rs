/*
 * Tackler-NG 2024-2026
 * SPDX-License-Identifier: Apache-2.0
 */
pub(crate) use items::AccountSelectors;
pub use items::BalanceType;
pub use items::Config;
pub(crate) use items::Export;
pub use items::ExportType;
pub use items::FormatType;
pub use items::Input;
pub(crate) use items::Kernel;
pub use items::PriceLookupType;
pub(crate) use items::Report;
pub use items::ReportType;
pub(crate) use items::Scale;
pub use items::StorageType;
pub(crate) use items::account::Account;
use jiff::tz::TimeZone;
use jiff::{Zoned, tz};
use toml::value::Offset;
use toml::value::{Date, Datetime, Time};

use crate::config::items::Timestamp;
use crate::tackler;
pub use items::NONE_VALUE;

mod items;
pub mod overlaps;
pub(crate) mod raw_items;

#[allow(clippy::cast_possible_wrap)]
fn toml_time_to_jiff(t: Time) -> Result<jiff::civil::Time, jiff::Error> {
    jiff::civil::Time::new(
        t.hour as i8,
        t.minute as i8,
        t.second.unwrap_or_default() as i8,
        t.nanosecond.unwrap_or_default() as i32,
    )
}
#[allow(clippy::cast_possible_wrap)]
fn toml_date_to_jiff(d: Date) -> Result<jiff::civil::Date, jiff::Error> {
    jiff::civil::Date::new(d.year as i16, d.month as i8, d.day as i8)
}
pub(crate) fn toml_datetime_to_zoned(
    dt: &Datetime,
    ts: &Timestamp,
) -> Result<Zoned, tackler::Error> {
    let Some(d) = dt.date else {
        let msg = format!(
            "\nTOML datetime must have date component. Partial time part of datetime is: {dt}"
        );
        return Err(msg.into());
    };

    if let Some(t) = dt.time
        && let Some(ttz) = dt.offset
    {
        let jt = toml_time_to_jiff(t)?;
        let jd = toml_date_to_jiff(d)?.to_datetime(jt);

        let tz = match ttz {
            Offset::Z => TimeZone::UTC,
            Offset::Custom { minutes } => {
                TimeZone::fixed(tz::Offset::from_seconds(i32::from(minutes) * 60)?)
            }
        };
        match jd.to_zoned(tz) {
            Ok(tsz) => Ok(tsz),
            Err(err) => {
                let msg = format!("time is invalid '{err:?}'");
                Err(msg.into())
            }
        }
    } else {
        let jd = if let Some(t) = dt.time {
            let jt = toml_time_to_jiff(t)?;
            toml_date_to_jiff(d)?.to_datetime(jt)
        } else {
            toml_date_to_jiff(d)?.to_datetime(ts.default_time)
        };

        match jd.to_zoned(ts.timezone.clone()) {
            Ok(tsz) => Ok(tsz),
            Err(err) => {
                let msg = format!("time is invalid '{err:?}'");
                Err(msg.into())
            }
        }
    }
}

/// Converter list of strings to report formats
///
/// # Errors
/// Returns `Err` in case invalid format or if same format is defined multiple times
pub fn to_report_formats(formats: Option<&[String]>) -> Result<Vec<FormatType>, tackler::Error> {
    match formats {
        Some(formats) => {
            if formats.is_empty() {
                let msg =
                    "Report formats has to contain at least one format type, it can't be empty.";
                return Err(msg.into());
            }
            // TODO: Detect same format multiple times
            let trgs = formats
                .iter()
                .try_fold(
                    Vec::new(),
                    |mut trgs: Vec<FormatType>, trg| match FormatType::try_from(trg.as_str()) {
                        Ok(t) => {
                            trgs.push(t);
                            Ok::<Vec<FormatType>, tackler::Error>(trgs)
                        }
                        Err(e) => {
                            let msg = format!("Invalid report format: {e}");
                            Err(msg.into())
                        }
                    },
                )?;
            Ok(trgs)
        }
        None => Ok(vec![FormatType::default()]),
    }
}

/// Converter list of strings to report targets
///
/// # Errors
/// Returns `Err` in case invalid target or if same target is defined multiple times
pub fn to_report_targets(targets: &[String]) -> Result<Vec<ReportType>, tackler::Error> {
    if targets.len() == 1 && targets[0].is_empty() {
        // this is used to disable reports from CLI: `--reports ""`
        return Ok(vec![]);
    }
    // TODO: Detect same target multiple times
    let trgs = targets
        .iter()
        .try_fold(
            Vec::new(),
            |mut trgs: Vec<ReportType>, trg| match ReportType::try_from(trg.as_str()) {
                Ok(t) => {
                    trgs.push(t);
                    Ok::<Vec<ReportType>, tackler::Error>(trgs)
                }
                Err(e) => {
                    let msg = format!("Invalid report target: {e}");
                    Err(msg.into())
                }
            },
        )?;
    Ok(trgs)
}

/// Converter list of strings to export formats
///
/// # Errors
/// Returns `Err` in case invalid format or if same format is defined multiple times
pub fn to_export_targets(targets: &[String]) -> Result<Vec<ExportType>, tackler::Error> {
    if targets.len() == 1 && targets[0].is_empty() {
        // this is used to disable exports from CLI: `--exports ""`
        return Ok(vec![]);
    }
    let trgs = targets
        .iter()
        .try_fold(
            Vec::new(),
            |mut trgs: Vec<ExportType>, trg| match ExportType::try_from(trg.as_str()) {
                Ok(t) => {
                    trgs.push(t);
                    Ok::<Vec<ExportType>, tackler::Error>(trgs)
                }
                Err(e) => {
                    let msg = format!("Invalid export target: {e}");
                    Err(msg.into())
                }
            },
        )?;
    Ok(trgs)
}
