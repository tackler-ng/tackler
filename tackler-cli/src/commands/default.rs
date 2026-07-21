/*
 * Tackler-NG 2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::cli_args::DefaultModeArgs;
use log::error;
use std::io;
use tackler_api::filters::FilterDefinition;
use tackler_core::config::Config;
use tackler_core::export::write_exports;
use tackler_core::kernel::Settings;
use tackler_core::kernel::settings::InputSettings;
use tackler_core::report::write_txt_reports;
use tackler_core::{parser, tackler};

pub(crate) fn exec(cli: DefaultModeArgs) -> Result<Option<String>, tackler::Error> {
    let cfg = match Config::try_from(
        cli.conf_path
            .as_ref()
            .expect("IE: logic error with CLI arguments"),
    ) {
        Ok(cfg) => cfg,
        Err(err) => {
            let msg = format!(
                "Configuration error with '{}': {err}",
                cli.conf_path.as_ref().unwrap().display()
            );
            error!("{msg}");
            return Err(msg.into());
        }
    };

    let overlaps = cli.overlaps()?;

    let mut settings = Settings::try_from(cfg, overlaps)?;

    let input_type = settings.input();

    #[rustfmt::skip]
    let result = match input_type {
        InputSettings::File(f) => {
            parser::paths_to_txns(&[f.path], &mut settings)
        },
        InputSettings::Fs(fs) => {
            let journal = fs.path.join(fs.dir);
            let paths = tackler_rs::get_paths_by_ext(&journal, fs.ext.as_str())?;
            parser::paths_to_txns(&paths, &mut settings)
        }
        InputSettings::Git(git) => {
            parser::git_to_txns(
                git.repo.as_path(),
                git.dir.as_str(),
                git.ext.as_str(),
                git.git_ref,
                &mut settings,
            )
        },
    };

    let txn_data = match result {
        Ok(txn_data) => txn_data,
        Err(err) => {
            let msg = format!("Txn Data: {err}");
            error!("{msg}");
            return Err(msg.into());
        }
    };

    let txn_filt = match cli.api_filter_def {
        Some(filt_str) => {
            if FilterDefinition::is_armored(&filt_str) {
                Some(FilterDefinition::from_armor(&filt_str)?)
            } else {
                Some(FilterDefinition::from_json_str(&filt_str)?)
            }
        }
        None => None,
    };

    let txn_set = match txn_filt {
        Some(tf) => txn_data.filter(&tf)?,
        None => txn_data.get_all()?,
    };

    if txn_set.is_empty() {
        let msg = "Txn Data: no transactions (txn set is empty)";
        error!("{msg}");
        return Err(msg.into());
    }

    let mut console_output = if cli.output_directory.is_none() {
        Some(Box::new(io::stdout()))
    } else {
        None
    };

    let reports = settings.get_report_targets();

    if !reports.is_empty() {
        write_txt_reports(
            &mut console_output,
            cli.output_directory.as_ref(),
            &cli.output_name,
            &reports,
            &txn_set,
            &settings,
            &mut Some(Box::new(io::stdout())),
        )?;
    }

    let exports = settings.get_export_targets();
    #[allow(clippy::unnecessary_unwrap)] // output_directory is always Some()
    if !exports.is_empty() && cli.output_directory.is_some() {
        write_exports(
            cli.output_directory
                .as_ref()
                .expect("IE: logic error with CLI arguments"),
            cli.output_name
                .expect("IE: logic error with CLI arguments")
                .as_str(),
            &exports,
            &txn_set,
            &mut settings,
            &mut Some(Box::new(io::stdout())),
        )?;
    }
    Ok(None)
}
