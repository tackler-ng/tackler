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

pub fn to_report_targets(targets: &[String]) -> Result<Vec<ReportType>, tackler::Error> {
    // TODO: Detect same target multiple times
    let trgs =
        targets.iter().try_fold(
            Vec::new(),
            |mut trgs: Vec<ReportType>, trg| match ReportType::from(trg.as_str()) {
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

pub fn to_export_targets(targets: &[String]) -> Result<Vec<ExportType>, tackler::Error> {
    let trgs =
        targets.iter().try_fold(
            Vec::new(),
            |mut trgs: Vec<ExportType>, trg| match ExportType::from(trg.as_str()) {
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
