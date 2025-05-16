/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! Rusty services for Tackler
//!
//! This crate is a collection of utilities for Tackler.
//!
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use log::error;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Regex helpers to have full haystack matcher (aka - JDK matches method)
pub mod regex;

/// Generic tackler namespace
pub mod tackler {
    /// Generic Error type
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
}

/// Normalize extension
///
/// This will remove single leading dot from extension
/// # Examples
/// ```
/// use tackler_rs::normalize_extension;
/// let ext = ".txt";
/// let txt = "txt";
/// let dot = "..dot";
/// assert_eq!(normalize_extension(ext), "txt");
/// assert_eq!(normalize_extension(txt), "txt");
/// assert_eq!(normalize_extension(dot), ".dot");
/// ```
#[must_use]
pub fn normalize_extension(ext: &str) -> &str {
    ext.strip_prefix('.').unwrap_or(ext)
}

///
/// Get full path based on
/// directory, filename prefix, filename and extension
///
#[must_use]
pub fn path_from_parts(dir: &Path, prefix: &str, name: &str, ext: &str) -> PathBuf {
    // #[unstable(feature = "path_add_extension", issue = "127292")]
    // pub fn with_added_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf {
    let filename = prefix.to_string() + name + "." + ext;
    dir.join(filename)
}

/// Creates a new file
///
/// `dir`, `prefix`, `name` and `ext`
///     "dir/prefix.name.ext"
///
/// See [`File::create_new`] for platform specific semantics.
///
/// # Errors
///
/// Returns `Err` if file exists or if it can't be created
pub fn create_output_file(
    dir: &Path,
    prefix: &str,
    name: &str,
    ext: &str,
) -> Result<(Box<dyn io::Write>, String), tackler::Error> {
    let rpt = ".".to_string() + name;
    let p = path_from_parts(dir, prefix, rpt.as_str(), ext);
    let f = match File::create_new(&p) {
        Ok(f) => f,
        Err(err) => {
            let msg = format!("{}: '{}'", err, p.as_path().to_string_lossy());
            error!("{msg}");
            return Err(msg.into());
        }
    };
    let bw = BufWriter::new(f);
    let path = p.to_string_lossy().to_string();
    Ok((Box::new(bw), path))
}

/// Convert path to absolute by anchor file
///
/// If the path is already absolute, then use the path as it is
///
/// # Errors
///
/// Returns `Err` if `Path::canonicalize()` fails for  resulting path
pub fn get_abs_path<P: AsRef<Path>>(anchor: P, path: &str) -> Result<PathBuf, tackler::Error> {
    let p = Path::new(path);
    if p.is_absolute() {
        return Ok(p.to_path_buf());
    }

    let a: &Path = anchor.as_ref();
    let abspath = match a.canonicalize()?.parent() {
        Some(parent) => parent.join(p),
        None => p.to_path_buf(),
    };
    Ok(abspath)
}

/// Get a list of paths by base dir and file extension
///
/// # Errors
///
/// Return `Err` if directory walks fails
pub fn get_paths_by_ext(base_dir: &Path, extension: &str) -> Result<Vec<PathBuf>, tackler::Error> {
    fn is_txn_file(entry: &walkdir::DirEntry, extension: &str) -> bool {
        (entry.file_type().is_file() || entry.file_type().is_symlink())
            && match entry.path().extension() {
                Some(ext) => ext == extension,
                None => false,
            }
    }
    let dir_entries: Result<Vec<DirEntry>, _> = WalkDir::new(base_dir)
        .follow_links(true)
        .into_iter()
        .collect();

    let paths: Vec<PathBuf> = dir_entries?
        .iter()
        .filter(|e| is_txn_file(e, extension))
        .map(|x| x.path().to_owned())
        .collect();

    Ok(paths)
}

/// Extensions to be used with [Indoc](https://docs.rs/indoc/latest/indoc/)
pub trait IndocUtils {
    #[allow(clippy::needless_doctest_main)]
    /// Strip away `|` -- prefix marker
    ///
    /// For full documentation, see  [`indoc!` -- docs](https://docs.rs/indoc/latest/indoc/).
    ///
    /// ```
    /// fn main() {
    ///     use indoc::indoc;
    ///     use tackler_rs::IndocUtils;
    ///     let testing = indoc! {
    ///         "|def hello():
    ///          |    print('Hello, bar!')
    ///          |
    ///          |hello()
    ///          |"
    ///     }.strip_margin();
    ///     let expected = "def hello():\n    print('Hello, bar!')\n\nhello()\n";
    ///     assert_eq!(testing, expected);
    ///
    ///     let second = indoc! {
    ///          "def hello():
    ///          |    print('Hello, bar!')
    ///          |
    ///          |hello()
    ///          |"
    ///     }.strip_margin();
    ///     assert_eq!(second, testing);
    /// }
    /// ```
    fn strip_margin(&self) -> String;
}

impl IndocUtils for str {
    fn strip_margin(&self) -> String {
        match self.strip_prefix('|') {
            Some(s) => s.to_string().replace("\n|", "\n"),
            None => self.replace("\n|", "\n"),
        }
    }
}
