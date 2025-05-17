/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::kernel::Settings;
use crate::model::TxnSet;
pub use equity_exporter::EquityExporter;
pub use equity_exporter::EquitySettings;
use std::io;
use std::path::Path;

use crate::config::ExportType;
use crate::tackler;
pub use identity_exporter::IdentityExporter;
use tackler_rs::create_output_file;

mod equity_exporter;
mod identity_exporter;

pub trait Export {
    /// Write export
    /// # Errors
    /// Returns `Err` in case of error
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        w: &mut W,
        txns: &TxnSet<'_>,
    ) -> Result<(), tackler::Error>;
}

/// Write all exports
///
/// # Errors
/// Returns `Err` in case of error
pub fn write_exports<ProgW: io::Write + ?Sized>(
    output_dir: &Path,
    output_name: &str,
    exports: &Vec<ExportType>,
    txn_set: &TxnSet<'_>,
    settings: &mut Settings,
    prog_writer: &mut Option<Box<ProgW>>,
) -> Result<(), tackler::Error> {
    for e in exports {
        match e {
            ExportType::Equity => {
                let eq_exporter = EquityExporter {
                    export_settings: EquitySettings::from(settings),
                };

                let (mut out_writer, path) =
                    create_output_file(output_dir, output_name, "equity", "txn")?;
                eq_exporter.write_export(settings, &mut out_writer, txn_set)?;
                if let Some(p) = prog_writer.as_mut() {
                    writeln!(p, "{:>21} : {}", "Equity Export", path)?;
                }
            }
            ExportType::Identity => {
                let id_exporter = IdentityExporter {};

                let (mut out_writer, path) =
                    create_output_file(output_dir, output_name, "identity", "txn")?;
                id_exporter.write_export(settings, &mut out_writer, txn_set)?;
                if let Some(p) = prog_writer.as_mut() {
                    writeln!(p, "{:>21} : {}", "Identity Export", path)?;
                }
            }
        }
    }

    Ok(())
}
