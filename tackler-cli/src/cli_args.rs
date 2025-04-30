/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;
use tackler_api::txn_ts;
use tackler_core::config::PriceLookupType;
use tackler_core::config::overlaps::{
    AuditOverlap, OverlapConfig, PriceOverlap, ReportOverlap, StrictOverlap, TargetOverlap,
};
use tackler_core::kernel::Settings;
use tackler_core::kernel::settings::{FileInput, FsInput, GitInput, InputSettings};
use tackler_core::parser::GitInputSelector;
use tackler_core::{config, tackler};

use tackler_core::config::FormatType;

pub(crate) const PRICE_BEFORE: &str = "price.before";
//
// Default subcommand setup:
// https://github.com/clap-rs/clap/issues/975
//
#[derive(Parser)]
#[command(author, version=env!("VERSION"), about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(flatten)]
    pub args: DefaultModeArgs,
}

impl Cli {
    pub(crate) fn cmd(&self) -> Commands {
        let command = self
            .command
            .clone()
            .unwrap_or(Commands::Report(self.args.clone()));
        match self.command {
            Some(_) => command,
            None => {
                if self.args.conf_path.is_none() {
                    let mut cmd = Cli::command();
                    let msg = format!(
                        "config file is not provided, use: \n\n{} --config <path/to/config-file>",
                        cmd.get_name()
                    );

                    cmd.error(ErrorKind::MissingRequiredArgument, msg.as_str())
                        .exit();
                }
                command
            }
        }
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(multiple = false)]
pub(crate) struct GitInputGroup {
    /// Git reference name
    #[arg(
        long = "input.git.ref",
        value_name = "refname",
        group = "git_input_group"
    )]
    pub(crate) input_git_ref: Option<String>,

    /// Git object name (commit id)
    #[arg(
        long = "input.git.commit",
        value_name = "sha",
        group = "git_input_group"
    )]
    pub(crate) input_git_commit: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct PriceLookupParser;

impl TypedValueParser for PriceLookupParser {
    type Value = PriceLookupType;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_str()
            .ok_or_else(|| clap::Error::new(clap::error::ErrorKind::InvalidUtf8))?;

        match PriceLookupType::try_from(val) {
            Ok(v) => Ok(v),
            Err(_) => {
                let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
                if let Some(arg) = arg {
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(arg.to_string()),
                    );
                }
                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(val.to_string()),
                );
                Err(err)
            }
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(
            [
                config::NONE_VALUE,
                PriceLookupType::TXN_TIME,
                PriceLookupType::LAST_PRICE,
                PriceLookupType::GIVEN_TIME,
            ]
            .into_iter()
            .map(clap::builder::PossibleValue::new),
        ))
    }
}

#[derive(Clone, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Commands {
    /// create new bookkeeping setup
    New { name: String },
    /// Initialize existing bookkeeping setup
    Init {},
    /// This is the default action: run specified reports and exports
    Report(DefaultModeArgs),
}

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct DefaultModeArgs {
    #[arg(long = "config", value_name = "path_to_config-file")]
    pub(crate) conf_path: Option<PathBuf>,

    /// Strict txn data mode
    ///
    /// Turn on strict validation of transactions (accounts, commodities and tags).
    #[arg(long = "strict.mode", value_name = "true|false")]
    pub(crate) strict_mode: Option<bool>,

    /// Txn set audit mode
    ///
    /// Produce checksums for transaction data and account selectors
    #[arg(long = "audit.mode", value_name = "true|false")]
    pub(crate) audit_mode: Option<bool>,

    /// Path to output directory
    #[arg(
        long = "output.dir",
        value_name = "path_to_output-directory",
        requires("output_name")
    )]
    pub(crate) output_directory: Option<PathBuf>,

    /// Basename of report files
    #[arg(
        long = "output.prefix",
        value_name = "filename-prefix",
        requires("output_directory")
    )]
    pub(crate) output_name: Option<String>,

    /// Path to single transaction journal file
    #[arg(long="input.file",
        value_name = "path_to_journal-file",
        conflicts_with_all([
            "input_storage",
            "input_fs_dir",
            "input_fs_ext",
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"])
    )]
    pub(crate) input_filename: Option<PathBuf>,

    ///
    /// Select used transaction storage
    ///
    #[arg(long="input.storage",
        value_name = "type_of_storage",
        value_parser([
            PossibleValue::new(config::StorageType::STORAGE_FS),
            PossibleValue::new(config::StorageType::STORAGE_GIT),
        ]),
        conflicts_with_all([
            "input_fs_dir",
            "input_fs_ext",
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"])
    )]
    pub(crate) input_storage: Option<String>,

    /// Filsystem path to transaction directory
    ///
    /// This could be a root or node of txn shard tree
    #[arg(long="input.fs.dir",
        value_name = "path_to_transaction-directory",
        requires("input_fs_ext"),
        conflicts_with_all([
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"])
    )]
    pub(crate) input_fs_dir: Option<PathBuf>,

    /// Txn file extension
    #[arg(
        long = "input.fs.ext",
        value_name = "txn_file-suffix",
        requires("input_fs_dir")
    )]
    pub(crate) input_fs_ext: Option<String>,

    /// Path to git repository
    ///
    /// Path to '.git' directory or bare git-repository.
    ///
    /// This could be a path to '.git' directory inside working copy
    #[arg(
        long = "input.git.repository",
        value_name = "path",
        requires("input_git_dir"),
        requires("git_input_group")
    )]
    pub(crate) input_git_repo: Option<PathBuf>,

    #[clap(flatten)]
    git_input_selector: GitInputGroup,

    /// Path (inside git repository) to transaction directory
    ///
    /// This could be a root or node of txn shard tree
    #[arg(
        long = "input.git.dir",
        value_name = "path_to_transaction-directory",
        requires("input_git_repo")
    )]
    pub(crate) input_git_dir: Option<String>,

    /// Account selectors for reports and exports
    ///
    /// List of patterns (regex) for account names.
    ///
    /// Use anchors ('^...$') for exact match.
    ///
    /// Use empty string "" to list all accounts
    #[arg(long = "accounts", value_name = "regex", num_args(1..))]
    pub(crate) accounts: Option<Vec<String>>,

    /// List of Reports to generate
    ///
    /// The list is space separated
    #[arg(long = "reports", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new("register"),
            PossibleValue::new("balance"),
            PossibleValue::new("balance-group"),
        ])
    )]
    pub(crate) reports: Option<Vec<String>>,

    #[arg(long = "formats", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new(FormatType::TXT),
            PossibleValue::new(FormatType::JSON),
        ]),
        requires("output_directory"),
        requires("output_name"),
    )]
    pub(crate) formats: Option<Vec<String>>,

    /// Path to single PriceDB file
    #[arg(long = "pricedb", value_name = "path_to_pricedb-file")]
    pub(crate) pricedb_filename: Option<PathBuf>,

    /// Name of the commodity to do the reports in
    #[arg(long = "report.commodity", value_name = "commodity")]
    pub(crate) report_commodity: Option<String>,

    /// Type of price lookup method
    #[arg(
        long = "price.lookup-type",
        value_name = "lookup-type",
        value_parser = PriceLookupParser
    )]
    pub(crate) price_lookup_type: Option<PriceLookupType>,

    /// Timestamp to use for price lookup "<ISO-8066-timestamp>",
    #[arg(long = PRICE_BEFORE, value_name = "price-before")]
    pub(crate) price_before_ts: Option<String>,

    /// Group-by -selector for 'balance-group' report
    #[arg(long = "group-by", value_name = "group-by", num_args(1),
        value_parser([
            PossibleValue::new(txn_ts::GroupBy::YEAR),
            PossibleValue::new(txn_ts::GroupBy::MONTH),
            PossibleValue::new(txn_ts::GroupBy::DATE),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK_DATE),
        ])
    )]
    pub(crate) group_by: Option<String>,

    /// List of Exports to generate
    ///
    /// The list is space separated
    #[arg(long = "exports", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new("identity"),
            PossibleValue::new("equity"),
        ]),
        requires("output_directory"),
        requires("output_name"),
    )]
    pub(crate) exports: Option<Vec<String>>,

    /// Txn Filter definition in JSON
    ///
    /// This could be ascii armored with base64 encoding
    ///
    /// The ascii armor must have prefix 'base64:'
    ///
    /// e.g. "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K"
    #[arg(long = "api-filter-def", value_name = "txn_filter")]
    pub(crate) api_filter_def: Option<String>,
}

impl DefaultModeArgs {
    pub(crate) fn get_overlaps(&self) -> OverlapConfig {
        OverlapConfig {
            audit: AuditOverlap {
                mode: self.audit_mode,
            },
            strict: StrictOverlap {
                mode: self.strict_mode,
            },
            price: PriceOverlap {
                db_path: self.pricedb_filename.clone(),
                lookup_type: self.price_lookup_type,
                before_time: self.price_before_ts.clone(),
            },
            report: ReportOverlap {
                commodity: self.report_commodity.clone(),
                account_overlap: self.accounts.clone(),
                group_by: self.group_by.clone(),
            },
            target: TargetOverlap {
                reports: self.reports.clone(),
                exports: self.exports.clone(),
                formats: self.formats.clone(),
            },
        }
    }

    fn get_git_selector(&self) -> Option<GitInputSelector> {
        match (
            &self.git_input_selector.input_git_commit,
            &self.git_input_selector.input_git_ref,
        ) {
            (Some(commit), None) => Some(GitInputSelector::CommitId(commit.clone())),
            (None, Some(git_ref)) => Some(GitInputSelector::Reference(git_ref.clone())),
            (None, None) => None,
            (Some(_), Some(_)) => {
                panic!("IE: this should not be possible, Clap configuration is broken")
            }
        }
    }

    pub(crate) fn get_input_type(
        &self,
        settings: &Settings,
    ) -> Result<InputSettings, tackler::Error> {
        let git_selector = self.get_git_selector();

        if let Some(filename) = &self.input_filename {
            let i = FileInput {
                path: filename.clone(),
            };
            Ok(InputSettings::File(i))
        } else if self.input_fs_dir.is_some() {
            let i = FsInput {
                dir: self
                    .input_fs_dir
                    .clone()
                    .expect("IE: This should not be possible (Clap)"),
                suffix: self
                    .input_fs_ext
                    .clone()
                    .expect("IE: This should not be possible (Clap)"),
            };
            Ok(InputSettings::Fs(i))
        } else if self.input_git_repo.is_some() {
            let i = GitInput {
                repo: self.input_git_repo.clone().unwrap(/*:ok: is_some */),
                git_ref: git_selector.expect("IE: This should not be possible (Clap)"),
                dir: self
                    .input_git_dir
                    .clone()
                    .expect("IE: This should not be possible (Clap)"),
                ext: String::from("txn"),
            };
            Ok(InputSettings::Git(i))
        } else if self.input_git_repo.is_none() && git_selector.is_some() {
            match settings.get_input_settings(
                Some(&config::StorageType::STORAGE_GIT.to_string()),
                Some(self.conf_path.as_ref().unwrap().as_path()),
            )? {
                InputSettings::Git(git) => Ok(InputSettings::Git(GitInput {
                    git_ref: git_selector.unwrap(/*:ok: is_some */),
                    ..git
                })),
                _ => {
                    let msg = "CLI Arg handling: Internal logic error";
                    Err(msg.into())
                }
            }
        } else {
            settings.get_input_settings(
                self.input_storage.as_ref(),
                Some(self.conf_path.as_ref().unwrap().as_path()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
