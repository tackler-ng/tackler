/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Account;
use crate::model::{AccountTreeNode, Commodity};
use crate::{config, tackler};
use itertools::Itertools;
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
    /// Is account defined by Chart of Accounts?
    pub(crate) fn is_defined(&self, name: &str) -> bool {
        self.defined_accounts.contains_key(name)
    }

    /// Get sorted iterator of defined accounts
    pub(crate) fn defined_accounts(
        &self,
    ) -> impl Iterator<Item = (&String, &Arc<AccountTreeNode>)> {
        // This can't be a String (account name) based sort, as natural sort won't work with numerical accounts
        // Below is natural sort order for account names with numbers:
        //    "E:01234567:Sweets:Ice·Cream",
        //    "E:0123:567:Sweets:Ice·Cream",
        // which is in wrong order, as "E:0123:5..." should sort before "E:012345"
        self.defined_accounts.iter().sorted_by(|a, b| a.1.cmp(b.1))
    }

    /// Insert new account in non-CoA mode
    pub(super) fn insert(&mut self, atn: &Arc<AccountTreeNode>) -> Result<(), tackler::Error> {
        self.defined_accounts
            .insert(atn.account.spec.name.clone(), atn.clone());
        AccountTree::build_account_tree(
            &mut self.synthetic_parents,
            atn,
            Some(&self.defined_accounts),
        )
    }
}

impl AccountTree {
    fn build_account_tree(
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

    pub(crate) fn from(account_names: &[Arc<Account>]) -> Result<AccountTree, tackler::Error> {
        let mut defined_accounts = HashMap::new();
        for account in account_names {
            match AccountTreeNode::try_from(account) {
                Ok(atn) => {
                    defined_accounts.insert(account.spec.name.clone(), Arc::new(atn));
                }
                Err(e) => {
                    let msg = format!("Invalid Chart of Accounts: {e}");
                    return Err(msg.into());
                }
            }
        }

        let mut synthetic_parents = HashMap::new();
        for atn_entry in &defined_accounts {
            if !&defined_accounts.contains_key(atn_entry.1.parent.as_str()) {
                // Parent is missing from CoA-> Let's build synthetic tree
                let (_, atn) = atn_entry;
                Self::build_account_tree(&mut synthetic_parents, atn, Some(&defined_accounts))?;
            }
        }

        Ok(AccountTree {
            defined_accounts,
            synthetic_parents,
        })
    }
}
