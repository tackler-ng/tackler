/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use itertools::Itertools;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::str;
//use std::time::{SystemTime, UNIX_EPOCH};

use crate::kernel::Settings;
use crate::model::{TxnData, Txns};
use crate::parser::tackler_parser;
use crate::tackler;
use gix as git;
use gix::hash as gix_hash;
use gix::objs::tree::EntryKind;
use tackler_api::metadata::items::{GitInputReference, MetadataItem};

pub enum GitInputSelector {
    CommitId(String),
    Reference(String),
}

pub fn string_to_txns(
    input: &mut &str,
    settings: &mut Settings,
) -> Result<TxnData, tackler::Error> {
    let txns = tackler_parser::txns_text(input, settings)?;

    // feature: a94d4a60-40dc-4ec0-97a3-eeb69399f01b
    // coverage: "sorted" tested by 200aad57-9275-4d16-bdad-2f1c484bcf17

    TxnData::from(None, txns, &settings.get_hash())
}

pub fn paths_to_txns(
    paths: &[PathBuf],
    settings: &mut Settings,
) -> Result<TxnData, tackler::Error> {
    let txns: Result<Txns, tackler::Error> = paths
        .iter()
        .map(|p| tackler_parser::txns_file(p, settings))
        .flatten_ok()
        .collect();

    TxnData::from(None, txns?, &settings.get_hash())
}

pub fn git_to_txns(
    repo_path: &Path,
    dir: &str,
    extension: &str,
    input_selector: GitInputSelector,
    settings: &mut Settings,
) -> Result<TxnData, tackler::Error> {
    // perf: let mut ts_par_total: u128 = 0;
    // perf: let ts_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);

    let repo = git::open(repo_path)?;

    let (object, reference) = match input_selector {
        GitInputSelector::CommitId(id) => {
            let mut candidates = Some(HashSet::default());
            let prefix = match gix_hash::Prefix::try_from(id.as_str()) {
                Ok(v) => v,
                Err(err) => {
                    let msg = format!("Invalid commit id '{id}': {err}");
                    return Err(msg.into());
                }
            };

            let res = repo.objects.lookup_prefix(prefix, candidates.as_mut())?;
            let object_id = match res {
                Some(Ok(id)) => id,
                Some(Err(())) => return Err(format!("Ambiguous abbreviated commit id {id}").into()),
                None => return Err(format!("Unknown commit id '{id}'").into()),
            };
            // This is originally commit, so no need to peel it
            (repo.find_object(object_id)?.try_into_commit()?, None)
        }
        GitInputSelector::Reference(ref_str) => {
            let id = repo.rev_parse_single(ref_str.as_bytes())?;
            let reference = if id.to_string().starts_with(ref_str.as_str()) {
                // This is tackler specific logic: don't show ref if it's plain commit id
                None
            } else {
                Some(ref_str.clone())
            };
            // Peel it so that tags are ok
            (id.object()?.peel_to_commit()?, reference)
        }
    };

    let gitmd = GitInputReference {
        commit: object.id.to_string(),
        reference,
        dir: dir.to_string(),
        suffix: extension.to_string(),
        message: object.message()?.title.to_string().trim().to_string(), // todo: This is bug in Gix
    };

    let tree = object.tree()?;
    // fixme: Optimization
    //      In the future, this could be optimized with custom walker,
    //      which does the filtering in the first place.
    let txns: Result<Txns, tackler::Error> = tree
        .traverse()
        .breadthfirst
        .files()?
        .iter()
        .map(|entry| {
            use git::objs::tree::EntryKind::{Blob, Link};
            match EntryKind::from(entry.mode) {
                Blob => {
                    if entry.filepath.starts_with(str::as_bytes(dir))
                        && entry.filepath.ends_with(str::as_bytes(extension))
                    {
                        let obj = repo.find_object(entry.oid)?;
                        // perf: let ts_par_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);

                        let par_res =
                            tackler_parser::txns_text(&mut str::from_utf8(&obj.data)?, settings);

                        // perf: let ts_par_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
                        // perf: ts_par_total = ts_par_total + (ts_par_end.as_millis() - ts_par_start.as_millis());
                        match par_res {
                            Ok(txns) => Ok(txns),
                            Err(err) => {
                                let msg = format!(
                                    "\
                                    GIT: Error while processing git object\n\
                                    \x20  commit id: {}\n\
                                    \x20  object id: {}\n\
                                    \x20  path: {}\n\
                                    \x20  msg: {}\
                                    ",
                                    object.id, obj.id, entry.filepath, err
                                );
                                Err(msg.into())
                            }
                        }
                    } else {
                        // It's blob but outside of our file path filter
                        Ok(Vec::default())
                    }
                }
                Link => {
                    let obj = repo.find_object(entry.oid)?;
                    let msg = format!(
                        "\
                        GIT: Error while processing git object\n\
                        \x20  commit id: {}\n\
                        \x20  object id: {}\n\
                        \x20  path: {}\n\
                        \x20  msg: {}\
                        ",
                        object.id,
                        obj.id,
                        entry.filepath,
                        "Links inside repository are not supported"
                    );
                    Err(msg.into())
                }
                // It's not a blob
                _ => Ok(Vec::default()),
            }
        })
        .flatten_ok()
        .collect::<Result<Txns, tackler::Error>>();

    // perf: let ts_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
    // perf: eprintln!("total time: {}ms, parse time: {}ms, git: {}ms", (ts_end.as_millis() - ts_start.as_millis()), ts_par_total, (ts_end.as_millis() - ts_start.as_millis())-ts_par_total);

    let hash = &settings.get_hash();
    TxnData::from(Some(MetadataItem::GitInputReference(gitmd)), txns?, hash)
}
