/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::commands::init;
use std::fs;
use std::path::Path;
use tackler_core::tackler;

pub(crate) fn exec(exe_name: &str, name: &str) -> Result<Option<String>, tackler::Error> {
    if fs::exists(name)? {
        let p = Path::new(name).canonicalize()?;
        return Err(format!("destination `{}` already exists", p.display()).into());
    }
    fs::create_dir_all(name)?;

    init::exec(exe_name, name)
}
