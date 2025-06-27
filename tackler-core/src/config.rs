/*
 * Tackler-NG 2024-2025
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

use crate::tackler;
pub use items::NONE_VALUE;

mod items;
pub mod overlaps;
mod raw_items;

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
