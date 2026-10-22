/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Account;
use crate::model::{AccountTreeNode, Commodity};
use crate::{config, tackler};
use std::collections::HashMap;
use std::sync::Arc;
use tackler_api::txn_header::Tag;

#[derive(Debug, Default)]
pub(crate) struct Tags {
    pub(crate) names: HashMap<String, Arc<String>>,
}

impl Tags {
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn from(cfg_tags: &config::Tags) -> Result<Tags, tackler::Error> {
        let names = cfg_tags.names.iter().fold(HashMap::new(), |mut tags, tag| {
            let t = Tag::from(tag.clone());
            tags.insert(tag.into(), Arc::new(t));
            tags
        });
        Ok(Tags { names })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Commodities {
    pub(super) names: HashMap<String, Arc<Commodity>>,
    pub(super) mandatory_commodity: bool,
}

impl Commodities {
    pub(crate) fn from(cfg_comm: &config::Commodities) -> Result<Commodities, tackler::Error> {
        let mandatory_commodity = !cfg_comm.permit_empty_commodity.unwrap_or(false);

        let comms =
            cfg_comm.names.iter().try_fold(
                HashMap::new(),
                |mut chm, comm| match Commodity::from(comm) {
                    Ok(c) => {
                        chm.insert(comm.into(), Arc::new(c));
                        Ok(chm)
                    }
                    Err(e) => {
                        let msg = format!("Invalid Chart of Commodities: {e}");
                        Err(msg)
                    }
                },
            )?;
        Ok(Commodities {
            names: comms,
            mandatory_commodity,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct AccountTree {
    pub(super) defined_accounts: HashMap<String, Arc<AccountTreeNode>>,
    pub(super) synthetic_parents: HashMap<String, Arc<AccountTreeNode>>,
}

impl AccountTree {
    /// Is account defined by Chart of Accounts
    ///
    /// Virtual accounts won't count
    pub(crate) fn is_defined(&self, name: &str) -> bool {
        self.defined_accounts.contains_key(name)
    }

    pub(super) fn build_account_tree(
        target_account_tree: &mut HashMap<String, Arc<AccountTreeNode>>,
        atn: &Arc<AccountTreeNode>,
        other_account_tree: Option<&HashMap<String, Arc<AccountTreeNode>>>,
    ) -> Result<(), tackler::Error> {
        let parent = atn.parent.as_str();
        let has_parent = other_account_tree.is_some_and(|a| a.contains_key(parent))
            || target_account_tree.contains_key(parent);

        if has_parent || atn.is_root() {
            // this breaks recursion
            Ok(())
        } else {
            let parent_atn = Arc::new(AccountTreeNode::make_parent(atn)?);
            target_account_tree.insert(parent.to_string(), parent_atn.clone());

            Self::build_account_tree(target_account_tree, &parent_atn, other_account_tree)
        }
    }

    pub(crate) fn from(
        account_names: &[Arc<Account>],
        strict_mode: bool,
    ) -> Result<AccountTree, tackler::Error> {
        let mut defined_accounts =
            account_names
                .iter()
                .try_fold(
                    HashMap::new(),
                    |mut accs, account| match AccountTreeNode::try_from(account) {
                        Ok(atn) => {
                            accs.insert(account.spec.name.clone(), Arc::new(atn));
                            Ok(accs)
                        }
                        Err(e) => {
                            let msg = format!("Invalid Chart of Accounts: {e}");
                            Err(msg)
                        }
                    },
                )?;

        let synthetic_parents = if strict_mode {
            // Synthetic Account Parents are only needed in strict mode
            let mut sap = HashMap::new();
            for atn_entry in &defined_accounts {
                if !&defined_accounts.contains_key(atn_entry.1.parent.as_str()) {
                    // Parent is missing -> Let's build synthetic tree
                    let (_, atn) = atn_entry;
                    Self::build_account_tree(&mut sap, atn, Some(&defined_accounts))?;
                }
            }
            sap
        } else {
            let a: Vec<Arc<AccountTreeNode>> = defined_accounts.values().cloned().collect();
            for atn in a {
                Self::build_account_tree(&mut defined_accounts, &atn, None)?;
            }
            HashMap::new()
        };
        Ok(AccountTree {
            defined_accounts,
            synthetic_parents,
        })
    }
}
