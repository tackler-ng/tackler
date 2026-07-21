/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]

mod cli_args;
mod commands;

use crate::cli_args::Commands;
use clap::Parser;
use log::error;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let exe_name = std::env::args().next().expect("No executable name");
    let cli = cli_args::Cli::parse();

    let command = cli.cmd();

    let res = match command {
        Commands::New { name } => commands::new::exec(&exe_name, name.as_str()),
        Commands::Init {} => commands::init::exec(&exe_name, "."),
        Commands::Report(args) => commands::default::exec(args),
    };

    match res {
        Ok(msg) => {
            if let Some(msg) = msg {
                println!("{msg}");
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
