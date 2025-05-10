/*
 * Tackler-NG 2019-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg_attr(rustfmt, rustfmt_skip)]
use std::path::Path;
use tackler_api::metadata::items::MetadataItem;
use tackler_core::kernel::Settings;
use tackler_core::model::TxnData;
use tackler_core::{parser, tackler};
use tackler_core::kernel::settings::GitInputSelector;
// val cfg = ConfigFactory.parseString(
//     """
//       |{
//       |   #  ./ = non-forked JVM
//       |   # ../ = forked JVM
//       |   basedir = "../tests/audit/"
//       |   input {
//       |     storage = git
//       |     git {
//       |       repository = "audit-repo.git"
//       |       dir = "txns/2016"
//       |       suffix = ".txn"
//       |     }
//       |   }
//       |}
//     """.stripMargin)

const REPO_PATH: &str = "../suite/audit/audit-repo.git/";
const TXN_SET_1E1_CHECKSUM: &str = "4a0eb2f8836447a025030a87136c047b4a737031162f593cb00f390c6ba113a3";
const TXN_SET_1E1_COMMIT_ID: &str = "ed6e4b10de2daea8d143569c473d14a9b09c3270";

const TXN_SET_1E5_CHECKSUM: &str = "2f4bc22df78502182aa27037d8d0f72462adb018be3e768399e0b803fa75baa7";
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
                            panic!(/*:test:*/
                                   "The first item is not Git Input Metadata item")
                        }
                    }
                    match &md.items[1] {
                        MetadataItem::TxnSetChecksum(tscsmd) => {
                            assert_eq!(tscsmd.hash.value, checksum);
                        }
                        _ => {
                            panic!(/*:test:*/
                                   "The second item is not Txn Set Checksum Metadata item")
                        }
                    }
                },
                None => {
                    panic!(/*:test:*/ "no metadata")
                },
            }
        },
        Err(err) => {
            panic!(/*:test:*/ "{:#?}", err);
        }
    }
}

#[test]
//desc: "handle ref with 10 (1E1) txns"
#[allow(non_snake_case)]
fn id_ce2e6523_ee83_46e7_a767_441c5b9f2802__normal_txns_1E1() {
    let result = parser::git_to_txns(Path::new(REPO_PATH), "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("set-1e1".to_string()),
                                     &mut Settings::default_audit());
    verify_git_run(result, TXN_SET_1E1_COMMIT_ID, TXN_SET_1E1_CHECKSUM);
}

#[test]
//desc: "handle ref with 100_000 (1E5) txns"
#[allow(non_snake_case)]
fn id_074f5549_346c_4780_90a1_07d60ae0e79d__normal_txns_1E5() {
    let result = parser::git_to_txns(Path::new(REPO_PATH),
                                     "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("set-1e5".to_string()),
                                     &mut Settings::default_audit());

    verify_git_run(result, TXN_SET_1E5_COMMIT_ID, TXN_SET_1E5_CHECKSUM);
}

#[test]
// test: a6cfe3b6-feec-4422-afbf-faeca5baf752
// desc: "report reasonable details in case of audit error"
fn test_git_error_reporting() {
    // See txn_header.rs::parse_txn_header
    //
    // GIT: Error while processing git object
    //      commit id: c984f946d1b76e3a175a07542859baf09be18c89
    //      object id: 82d58a5c5b2928baee5e93f1143e88a442087ebe
    //      path: txns/2016/04/01/20160401T120000-26.txn
    //      msg : Audit mode is activated and there is a txn without UUID ...
    //
    let result = parser::git_to_txns(Path::new(REPO_PATH), "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("err-1e2".to_string()),
                                     &mut Settings::default_audit());

    assert!(result.is_err());
    let msg = result.err().unwrap(/*:test:*/).to_string();
    assert!(msg.contains("c984f946d1b76e3a175a07542859baf09be18c89"));
    // git ls-tree \
    //    c984f946d1b76e3a175a07542859baf09be18c89 \
    //    txns/2016/04/01/20160401T120000-26.txn
    assert!(msg.contains("82d58a5c5b2928baee5e93f1143e88a442087ebe"));
    assert!(msg.contains("without UUID"));
    assert!(msg.contains("path: txns/2016/04/01/20160401T120000-26.txn"));
    assert!(msg.contains("txn date: 2016-04-01T12:00:00+00:00[UTC]"));
    assert!(msg.contains("txn code: #0000026"));
}
