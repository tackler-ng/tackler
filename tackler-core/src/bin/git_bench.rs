/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tackler_api::metadata::items::MetadataItem;
use tackler_core::kernel::Settings;
use tackler_core::model::TxnData;
use tackler_core::{parser, tackler};

use mimalloc::MiMalloc;
use tackler_core::kernel::settings::GitInputSelector;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const REPO_PATH: &str = "suite/audit/audit-repo.git/";
const TXN_SET_1E1_CHECKSUM: &str =
    "4a0eb2f8836447a025030a87136c047b4a737031162f593cb00f390c6ba113a3";
const TXN_SET_1E1_COMMIT_ID: &str = "ed6e4b10de2daea8d143569c473d14a9b09c3270";

const TXN_SET_1E5_CHECKSUM: &str =
    "2f4bc22df78502182aa27037d8d0f72462adb018be3e768399e0b803fa75baa7";
const TXN_SET_1E5_COMMIT_ID: &str = "4648a2994b41ed341b544a148b3060fd2d267d79";

#[rustfmt::skip]
fn verify_git_run(result: Result<TxnData, tackler::Error>, commit: &str, checksum: &str) {
    match result {
        Ok(txn_data) => {
            let txn_set = txn_data.get_all().unwrap(/*:test:*/);
            match txn_set.metadata() {
                Some(md) => {
                    assert_eq!(md.items.len(), 2, "Metadata Item count is wrong");
                    match &md.items[0] {
                        MetadataItem::GitInputReference(gitmd) => {
                            assert_eq!(gitmd.commit, commit);
                        }
                        _ => {
                            panic!(/*:test:*/ "The first item is not Git Input Metadata item")
                        }
                    }
                    match &md.items[1] {
                        MetadataItem::TxnSetChecksum(tscsmd) => {
                            assert_eq!(tscsmd.hash.value, checksum);
                        }
                        _ => {
                            panic!(/*:test:*/
                                "The second item is not Txn Set Checksum Metadata item"
                            )
                        }
                    }
                }
                None => {
                    panic!(/*:test:*/ "no metadata")
                }
            }
        }
        Err(err) => {
            eprintln!("{err:#}");
            panic!(/*:test:*/);
        }
    }
}

// test: 074f5549-346c-4780-90a1-07d60ae0e79d
// test: 33d85471-a04c-49b9-b7a0-9d7f7f5762eb
#[allow(non_snake_case)]
#[allow(clippy::cast_precision_loss)]
fn test_10_loops_with_txns_1E5() {
    eprintln!("\n\nMake 5 loops with set-1e5:");
    let mut settings = Settings::default_audit();
    let mut all_txns_per_s = 0.0;
    for i in 1..=5 {
        let ts_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
        let result = parser::git_to_txns(
            Path::new(REPO_PATH),
            "txns/2016",
            "txn",
            GitInputSelector::Reference("set-1e5".to_string()),
            &mut settings,
        );
        let ts_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
        verify_git_run(result, TXN_SET_1E5_COMMIT_ID, TXN_SET_1E5_CHECKSUM);

        let txn_per_s = 100_000.0 / ((ts_end.as_millis() - ts_start.as_millis()) as f64 / 1000.0);

        eprintln!("Done: {i:10} of 5 loops {txn_per_s:.0} txn/s");
        all_txns_per_s += txn_per_s;
    }
    let txn_per_s_ave = all_txns_per_s / 5.0;

    let txn_per_s_reference = 100_000.0;

    eprintln!(
        "
On average: {txn_per_s_ave:8>.0} txn/s
Reference:  {txn_per_s_reference:8>.0} txn/s ({:>+6.0} txn/s)",
        txn_per_s_ave - txn_per_s_reference
    );
}

// test: ce2e6523-ee83-46e7-a767-441c5b9f2802
// test: fae31eb0-bd4a-483e-9eb7-9e4c36e7f785
#[allow(non_snake_case)]
fn test_10000_loops_with_txns_1E1() {
    let mut settings = Settings::default_audit();
    let loops = 10_000;
    eprintln!("\n\nMake 10_000 loops with set-1e1:");
    let mut r = 0;
    let mut loop_count = 0;
    for i in 1..=loops {
        let result = parser::git_to_txns(
            Path::new(REPO_PATH),
            "txns/2016",
            "txn",
            GitInputSelector::Reference("set-1e1".to_string()),
            &mut settings,
        );
        verify_git_run(result, TXN_SET_1E1_COMMIT_ID, TXN_SET_1E1_CHECKSUM);

        let print_round = i / (loops / 10);
        if r < print_round {
            eprintln!("Done: {i:10.0} of {loops} loops");
            r = print_round;
        }
        loop_count += 1;
    }
    assert_eq!(loop_count, 10000);
}

fn main() {
    test_10000_loops_with_txns_1E1();

    test_10_loops_with_txns_1E5();
}
