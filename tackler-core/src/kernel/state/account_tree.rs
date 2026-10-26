/*
 * Tackler-NG 2023-2026
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Account;
use crate::config::Spans;
use crate::model::{AccountTreeNode, Commodity};
use crate::{config, tackler};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Write;
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
    strict_mode: bool,
}
impl AccountTree {
    pub(crate) fn from(
        account_names: &[Account],
        strict_mode: bool,
    ) -> Result<AccountTree, tackler::Error> {
        let mut defined_accounts = HashMap::with_capacity(account_names.len());
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

        let mut at = AccountTree {
            defined_accounts,
            synthetic_parents,
            strict_mode,
        };
        if strict_mode {
            at.finalize_temporals()?;
        }
        Ok(at)
    }

    /// Check and finalize temporal settings for `AccountTree`
    ///
    /// This checks the logic of temporal definitions for account tree,
    /// e.g. that there isn't nested parent-child definitions.
    ///
    /// Then it updates children's temporals based on parent's temporals,
    /// if they don't have their owns.
    fn finalize_temporals(&mut self) -> Result<(), tackler::Error> {
        // Todo: revisit this mess at some sunny day
        //
        // a) Check temporal settings (this is about ok)
        // b) Update ATN if it's missing temporal and parent have one
        // b.1) This is mess, find a better way to update while folding references etc.
        let spans: Vec<Option<(String, Spans)>> = self
            .defined_accounts
            .iter()
            .sorted_unstable_by(|a, b| a.1.depth_first(b.1))
            .map(|kv| -> Result<Option<(String, Spans)>, tackler::Error> {
                if kv.1.account.is_temporal() {
                    let _ = self.check_temporality(kv.1, Some(kv.1.clone()))?;
                    // No need to update: I already have my own temporal definition
                    Ok(None)
                } else if let Some(temporal) = self.check_temporality(kv.1, None)? {
                    // Temporal for me has to be updated: One of my parents had temporal definition
                    Ok(Some((kv.1.account.spec.name.clone(), temporal)))
                } else {
                    // No need to update: Neither me, nor any of my parents have temporal definition
                    Ok(None)
                }
            })
            .try_collect()?;

        spans.into_iter().flatten().for_each(|i| {
            let a = self
                .defined_accounts
                .get_mut(i.0.as_str())
                .expect("IE: hashmap missing item");
            Arc::get_mut(a)
                .expect("IE: ATN with temporal check")
                .account
                .spans = Some(i.1);
        });
        Ok(())
    }

    /// Is account defined by Chart of Accounts?
    pub(crate) fn is_defined(&self, name: &str) -> bool {
        self.defined_accounts.contains_key(name)
    }

    /// Get sorted iterator of defined accounts
    pub(crate) fn defined_accounts(
        &self,
    ) -> impl Iterator<Item = (&String, &Arc<AccountTreeNode>)> {
        // This can't be a key / string (e.g. account name) based sort, as natural sort won't work with numerical accounts
        // Below is natural sort order for account names with numbers:
        //    "E:01234567:Sweets:Ice·Cream",
        //    "E:0123:567:Sweets:Ice·Cream",
        // which is in wrong order, as "E:0123:5..." should sort before "E:012345"
        self.defined_accounts.iter().sorted_by(|a, b| a.1.cmp(b.1))
    }

    /// Insert new account in non-CoA mode
    pub(super) fn insert(&mut self, atn: &Arc<AccountTreeNode>) -> Result<(), tackler::Error> {
        if self.strict_mode {
            let msg = format!("IE: AccountTree: insert in strict mode, account: '{atn}'");
            return Err(msg.into());
        }
        self.defined_accounts
            .insert(atn.account.spec.name.clone(), atn.clone());
        AccountTree::build_account_tree(
            &mut self.synthetic_parents,
            atn,
            Some(&self.defined_accounts),
        )
    }

    /// Get parent `AccountTreeNode` of child
    ///
    /// # Errors
    /// Error if child is root
    pub(super) fn parent_account(
        &self,
        child: &Arc<AccountTreeNode>,
    ) -> Result<Arc<AccountTreeNode>, tackler::Error> {
        if child.is_root() {
            let msg = format!("can't get parent from root account: '{child}'");
            return Err(msg.into());
        }
        let parent = child.parent.as_str();

        match self.defined_accounts.get(parent) {
            Some(atn) => Ok(atn.clone()),
            None => {
                if let Some(synth_atn) = self.synthetic_parents.get(parent) {
                    Ok(synth_atn.clone())
                } else {
                    let msg = format!("Unknown parent account: '{parent}'");
                    Err(msg.into())
                }
            }
        }
    }

    fn check_temporality(
        &self,
        atn: &Arc<AccountTreeNode>,
        child_with_temporal: Option<Arc<AccountTreeNode>>,
    ) -> Result<Option<Spans>, tackler::Error> {
        if atn.is_root() {
            return Ok(None);
        }
        let parent = self.parent_account(atn)?;
        if let Some(child) = &child_with_temporal
            && parent.account.is_temporal()
        {
            let mut msg = String::new();
            writeln!(msg, "Account validity is set for both parent and child.")?;
            writeln!(msg, "   Parent : {}", parent.account.spec.name)?;
            writeln!(msg, "   Child  : {}", child.account.spec.name)?;
            writeln!(msg, "Nested temporal settings are not allowed.")?;
            Err(msg.into())
        } else {
            if let Some(temporal) = self.check_temporality(&parent, child_with_temporal)? {
                Ok(Some(temporal))
            } else {
                if parent.account.spans.is_some() {
                    Ok(parent.account.spans.clone())
                } else {
                    Ok(None)
                }
            }
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Accounts, Timestamp};

    // test: 92c1a607-1420-44c0-9c9a-8544359538da
    // desc: parent without temporal
    #[test]
    fn test_account_tree_ok() {
        let value = r#"
accounts = [
    {
        name = "a:b:c",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a:b:d",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a",
    },
]
"#;
        let accounts = Accounts::from_str(value, &Timestamp::default()).unwrap(/*:test:*/);
        let mut acc_tree = AccountTree::from(&accounts.items, false).unwrap(/*:test:*/);
        assert!(acc_tree.finalize_temporals().is_ok());

        let acc_strict = AccountTree::from(&accounts.items, true);
        assert!(acc_strict.is_ok());
    }

    // test: 971eac87-ea36-4bba-bc27-22c50f2e3454
    // desc: err, nested temporal settings (at top)
    #[test]
    fn error_account_tree_nested_1() {
        let value = r#"
accounts = [
    {
        name = "a:b:c",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a:b:d",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a",
        open = 2026-01-01,
    },
]
"#;
        let accounts = Accounts::from_str(value, &Timestamp::default()).unwrap(/*:test:*/);
        let mut acc_tree = AccountTree::from(&accounts.items, false).unwrap(/*:test:*/);
        assert!(acc_tree.finalize_temporals().is_err());

        let acc_strict = AccountTree::from(&accounts.items, true);
        assert!(acc_strict.is_err());
    }

    // test: 2c180bf9-bf27-4e52-a17e-966e579e2b99
    // desc: err, nested temporal settings (in the middle)
    #[test]
    fn error_account_tree_nested_2() {
        let value = r#"
accounts = [
    {
        name = "a:b:c:1",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a:b:c:2",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a:b",
        open = 2026-01-01,
    },
]
"#;
        let accounts = Accounts::from_str(value, &Timestamp::default()).unwrap(/*:test:*/);
        let mut acc_tree = AccountTree::from(&accounts.items, false).unwrap(/*:test:*/);
        assert!(acc_tree.finalize_temporals().is_err());

        let acc_strict = AccountTree::from(&accounts.items, true);
        assert!(acc_strict.is_err());
    }

    // test: bf45205f-7373-4eaf-89c4-bc619748d329
    // desc: err, plain and temporal
    #[test]
    fn error_duplicate_plain_and_temporal() {
        let value = r#"
accounts = [
    {
        name = "a:b:c",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    {
        name = "a:b:d",
        open = 2026-01-01,
        close = 2027-01-01,
    },
    "a:b:c",
]
"#;
        let accounts = Accounts::from_str(value, &Timestamp::default());
        assert!(accounts.is_err());
    }
}
