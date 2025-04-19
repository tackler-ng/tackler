/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]

mod cli_args;
mod commands;

use log::error;
use std::io;
use tackler_core::export::write_exports;
use tackler_core::kernel::settings::Settings;
use tackler_core::report::write_txt_reports;
use tackler_core::{parser, tackler};

use clap::Parser;
use tackler_api::filters::FilterDefinition;
use tackler_core::config::Config;

use crate::cli_args::{Commands, DefaultModeArgs};
use tackler_core::kernel::settings::InputSettings;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn run(cli: DefaultModeArgs) -> Result<Option<String>, tackler::Error> {
    let cfg = match Config::from(cli.conf_path.as_ref().unwrap()) {
        Ok(cfg) => cfg,
        Err(err) => {
            let msg = format!(
                "Configuration error with '{}': {err}",
                cli.conf_path.as_ref().unwrap().display()
            );
            error!("{}", msg);
            return Err(msg.into());
        }
    };

    let overlaps = cli.get_overlaps();

    let mut settings = Settings::try_from(cfg, overlaps)?;

    let input_type = cli.get_input_type(&settings)?;

    #[rustfmt::skip]
    let result = match input_type {
        InputSettings::File(f) => {
            parser::paths_to_txns(&[f.path], &mut settings)
        },
        InputSettings::Fs(fs) => {
            let paths = tackler_rs::get_paths_by_ext(fs.dir.as_path(), fs.suffix.as_str())?;
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
            error!("{}", msg);
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
        error!("{}", msg);
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

fn main() {
    let exe_name = std::env::args().next().expect("No executable name");
    let cli = cli_args::Cli::parse();

    let command = cli.cmd();

    let res = match command {
        Commands::New { name } => commands::new::exec(&exe_name, name.as_str()),
        Commands::Init {} => commands::init::exec(&exe_name, "."),
        Commands::Report(args) => run(args),
    };

    match res {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{}", msg);
            }
            std::process::exit(0)
        }
        Err(err) => {
            let msg = format!("Tackler error: {err}");
            error!("{msg}");
            eprintln!("{msg}");
            std::process::exit(1)
        }
    }
}
