/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;
use tackler_api::txn_ts;
use tackler_core::config;
use tackler_core::config::overlaps::{
    AuditOverlap, FileInputOverlap, FsInputOverlap, GitInputOverlap, InputOverlap, OverlapConfig,
    PriceOverlap, ReportOverlap, StorageOverlap, StrictOverlap, TargetOverlap,
};
use tackler_core::config::{PriceLookupType, StorageType};
use tackler_core::kernel::settings::GitInputSelector;

use tackler_core::config::FormatType;

pub(crate) const PRICE_BEFORE: &str = "price.before";
//
// Default subcommand setup:
// https://github.com/clap-rs/clap/issues/975
//
#[derive(Parser)]
#[command(author, version=env!("VERSION"), about, long_about = None,
after_help = "See tackler documentation for more information and examples:\nhttps://tackler.e257.fi/docs/",
)]
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
        value_name = "reference",
        group = "git_input_group"
    )]
    pub(crate) input_git_ref: Option<String>,

    /// Git object name (commit id)
    #[arg(
        long = "input.git.commit",
        value_name = "commit-id",
        group = "git_input_group"
    )]
    pub(crate) input_git_commit: Option<String>,
}
#[derive(Debug, Clone, Copy)]
struct StorageTypeParser;

impl TypedValueParser for StorageTypeParser {
    type Value = StorageType;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let val = value
            .to_str()
            .ok_or_else(|| clap::Error::new(clap::error::ErrorKind::InvalidUtf8))?;

        match StorageType::try_from(val) {
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
            [StorageType::FS, StorageType::GIT]
                .into_iter()
                .map(clap::builder::PossibleValue::new),
        ))
    }
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
    /// Create a new journal and configuration setup
    New { name: String },
    /// Initialize journal at the current working directory
    Init {},
    /// Run specified reports and exports - this is the default action
    Report(DefaultModeArgs),
}

#[rustfmt::skip]
#[derive(Debug, Clone, clap::Args)]
pub(crate) struct DefaultModeArgs {

    /// The journal configuration is mandatory argument
    #[arg(long = "config", value_name = "filename")]
    pub(crate) conf_path: Option<PathBuf>,

    /// Strict journal data mode
    ///
    /// Turn on strict validation of transaction data
    /// (accounts, commodities and tags).
    #[arg(
        long = "strict.mode",
        value_name = "true|false",
        verbatim_doc_comment
    )]
    pub(crate) strict_mode: Option<bool>,

    /// Journal Audit mode
    ///
    /// Produce checksums for transaction data
    /// and account selectors.
    #[arg(
        long = "audit.mode",
        value_name = "true|false",
        verbatim_doc_comment
    )]
    pub(crate) audit_mode: Option<bool>,

    /// Path to output directory
    #[arg(
        long = "output.dir",
        value_name = "directory",
        requires("output_name")
    )]
    pub(crate) output_directory: Option<PathBuf>,

    /// Basename of report files
    #[arg(
        long = "output.prefix",
        value_name = "filename",
        requires("output_directory")
    )]
    pub(crate) output_name: Option<String>,

    /// Path to single transaction journal file
    #[arg(long="input.file",
        value_name = "filename",
        conflicts_with_all([
            "input_storage",
            "input_fs_path",
            "input_fs_dir",
            "input_fs_ext",
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir",
            "input_git_ext",
        ])
    )]
    pub(crate) input_filename: Option<PathBuf>,

    ///
    /// Select used transaction storage
    ///
    #[arg(long="input.storage",
        value_name = "fs|git",
        conflicts_with_all([
            "input_filename",
        ]),
        value_parser = StorageTypeParser
    )]
    pub(crate) input_storage: Option<StorageType>,

    /// Filesystem path to journal directory
    ///
    /// This is the root of journal, see also `--input.fs.dir`
    #[arg(long="input.fs.path",
        value_name = "path",
        requires("input_fs_dir"),
        requires("input_fs_ext"),
        conflicts_with_all([
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"
        ])
    )]
    pub(crate) input_fs_path: Option<String>,

    /// Txn directory inside journal
    ///
    /// This is the root node of txn tree inside journal.
    /// See also `--input.fs.path`
    #[arg(long="input.fs.dir",
        value_name = "txns-directory",
        conflicts_with_all([
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"
        ]),
        verbatim_doc_comment
    )]
    pub(crate) input_fs_dir: Option<String>,

    /// Txn file extension
    #[arg(
        long = "input.fs.ext",
        value_name = "extension",
        requires("input_fs_dir"),
        conflicts_with_all([
            "input_git_repo",
            "input_git_ref",
            "input_git_commit",
            "input_git_dir"
        ])
    )]
    pub(crate) input_fs_ext: Option<String>,

    /// Path to git repository
    ///
    /// Path to '.git' directory or bare git-repository.
    ///
    /// This could be a path to '.git' directory inside working copy
    #[arg(
        long = "input.git.repository",
        alias = "input.git.repo",
        value_name = "path",
        requires("input_git_dir"),
        requires("input_git_ext"),
        requires("git_input_group")
    )]
    pub(crate) input_git_repo: Option<String>,

    #[clap(flatten)]
    git_input_selector: GitInputGroup,

    /// Path (inside git repository) to transaction directory
    ///
    /// This could be a root or node of txn shard tree
    #[arg(long = "input.git.dir", value_name = "txns-directory")]
    pub(crate) input_git_dir: Option<String>,

    /// Txn file extension
    #[arg(
        long = "input.git.ext",
        value_name = "extension",
        requires("input_git_repo"),
        requires("input_git_dir")
    )]
    pub(crate) input_git_ext: Option<String>,

    /// List of Account Selectors for reports and exports
    ///
    /// These are full match regular expressions (regex),
    /// and they are matched against the full account name.
    /// Use wildcard patterns e.g.
    ///    'Assets:.*' 'Assets(:.*)?'
    /// when needed.
    ///
    /// Empty string "" lists all accounts
    #[arg(
        long = "accounts",
        value_name = "regex",
        num_args(1..),
        verbatim_doc_comment
    )]
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

    /// List of report output format types
    ///
    /// This has only effect when used with `--output.*`
    ///
    /// The list is space separated.
    #[arg(long = "formats", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new(FormatType::TXT),
            PossibleValue::new(FormatType::JSON),
        ]),
        requires("output_directory"),
        requires("output_name"),
    )]
    pub(crate) formats: Option<Vec<String>>,

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

    /// Path to PriceDB file
    #[arg(
        long = "pricedb",
        value_name = "pricedb-file",
    )]
    pub(crate) pricedb_filename: Option<PathBuf>,

    /// Name of the target commodity in reports
    ///
    /// This is the target commodity when commodity price
    /// conversion is activated, and there is existing conversion
    /// from the base commodity to this target commodity.
    #[arg(
        long = "report.commodity",
        value_name = "commodity",
        verbatim_doc_comment
    )]
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
    fn verify_storage_mode(&self, allowed_type: StorageType) -> Result<(), clap::Error> {
        match self.input_storage {
            Some(st) => {
                if st != allowed_type {
                    let mut err = clap::Error::new(ErrorKind::ArgumentConflict);
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(format!("--input.storage {}", st)),
                    );
                    err.insert(
                        ContextKind::PriorArg,
                        ContextValue::String(format!("--input.{}.*", allowed_type)),
                    );
                    Err(err)
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    fn input_overlap(&self) -> Result<Option<InputOverlap>, clap::Error> {
        let git_selector = self.git_selector();

        if let Some(filename) = &self.input_filename {
            let i = FileInputOverlap {
                path: filename.clone(),
            };
            Ok(Some(InputOverlap::File(i)))
        } else if self.input_fs_path.is_some()
            || self.input_fs_dir.is_some()
            || self.input_fs_ext.is_some()
        {
            self.verify_storage_mode(StorageType::Fs)?;

            let i = FsInputOverlap {
                path: self.input_fs_path.clone(),
                dir: self.input_fs_dir.clone(),
                ext: self.input_fs_ext.clone(),
            };
            Ok(Some(InputOverlap::Fs(i)))
        } else if self.input_git_repo.is_some()
            || self.input_git_dir.is_some()
            || self.input_git_ext.is_some()
            || git_selector.is_some()
        {
            self.verify_storage_mode(StorageType::Git)?;

            let i = GitInputOverlap {
                repo: self.input_git_repo.clone(),
                git_ref: git_selector,
                dir: self.input_git_dir.clone(),
                ext: self.input_git_ext.clone(),
            };
            Ok(Some(InputOverlap::Git(i)))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn overlaps(&self) -> Result<OverlapConfig, clap::Error> {
        Ok(OverlapConfig {
            audit: AuditOverlap {
                mode: self.audit_mode,
            },
            strict: StrictOverlap {
                mode: self.strict_mode,
            },
            storage: StorageOverlap {
                storage_type: self.input_storage,
                input: self.input_overlap()?,
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
        })
    }

    fn git_selector(&self) -> Option<GitInputSelector> {
        match (
            &self.git_input_selector.input_git_commit,
            &self.git_input_selector.input_git_ref,
        ) {
            (Some(commit), None) => Some(GitInputSelector::CommitId(commit.clone())),
            (None, Some(git_ref)) => Some(GitInputSelector::Reference(git_ref.clone())),
            (None, None) => None,
            (Some(_), Some(_)) => {
                panic!("IE: invalid combination of git input selectors (Clap)")
            }
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
