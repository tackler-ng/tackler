/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::{parser, tackler};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Commodity {
    pub name: String,
}

impl Display for Commodity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Commodity {
    pub fn is_any(&self) -> bool {
        !self.name.is_empty()
    }
    pub fn is_void(&self) -> bool {
        self.name.is_empty()
    }
}

impl Commodity {
    pub fn from(name: String) -> Result<Commodity, tackler::Error> {
        if !parser::is_valid_id(&name) {
            let msg = format!("This is not a valid commodity: '{name}'");
            return Err(msg.into());
        }
        Ok(Commodity { name })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq)]
pub(crate) struct AccountTreeNode {
    pub(crate) depth: usize,
    root: String,
    /// parent account (path)
    pub(crate) parent: String,
    /// account of posting (path)
    pub(crate) account: String,
    /// account name (leaf)
    name: String,
}

impl AccountTreeNode {
    pub(crate) fn is_root(&self) -> bool {
        self.depth == 1
    }
    pub(crate) fn my_parent_is_root(&self) -> bool {
        self.depth == 2
    }
}
#[derive(Debug, Clone, Eq)]
pub struct TxnAccount {
    pub(crate) atn: Arc<AccountTreeNode>,
    pub(crate) comm: Arc<Commodity>,
}

impl Hash for TxnAccount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.atn.account.hash(state);
        self.comm.name.hash(state);
    }
}
impl PartialEq for TxnAccount {
    fn eq(&self, other: &Self) -> bool {
        self.atn == other.atn && self.comm == other.comm
    }
}
impl Ord for TxnAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.comm.cmp(&other.comm) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.atn.account.cmp(&other.atn.account),
        }
    }
}
impl PartialOrd for TxnAccount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TxnAccount {
    pub(crate) fn is_parent_of(&self, atn: &TxnAccount) -> bool {
        self.atn.account == atn.atn.parent && self.comm.name == atn.comm.name
    }
    pub(crate) fn is_root(&self) -> bool {
        self.atn.is_root()
    }

    pub(crate) fn my_parent_is_root(&self) -> bool {
        self.atn.my_parent_is_root()
    }
}

impl Display for AccountTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.account)
    }
}

impl PartialEq for AccountTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
    }
}

#[cfg(test)]
impl AccountTreeNode {
    pub(crate) fn get_root(&self) -> &str {
        self.root.as_str()
    }
    pub(crate) fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl AccountTreeNode {
    pub(crate) fn from(account: &str) -> Result<AccountTreeNode, tackler::Error> {
        {
            let acc = account.trim();

            if acc.len() != account.len() {
                let msg = format!("Account name contains whitespaces '{account}'");
                return Err(msg.into());
            }
        }

        let parts: Vec<&str> = account.split(':').collect();

        if parts.is_empty() {
            let msg = format!(
                "Empty account names are not allowed (all sub-components are empty): '{account}'"
            );
            return Err(msg.into());
        }
        if parts
            .iter()
            .map(|subpath| parser::is_valid_sub_id(subpath.trim()))
            .any(|valid| !valid)
        {
            let msg = format!("This is not a valid account name: '{account}'");
            return Err(msg.into());
        }

        let depth = parts.len();
        let root = String::from(parts[0]);

        let mut rev_parts = parts;
        rev_parts.reverse();
        let name = String::from(rev_parts.remove(0));

        rev_parts.reverse();
        let parent = rev_parts.join(":");

        Ok(AccountTreeNode {
            depth,
            root,
            parent,
            account: account.to_string(),
            name,
        })
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn id_270d505b_76f6_4e49_a24c_2fbdfb6e5adf__commodity_ok() {
        let res = Commodity::from("He·bar".to_string());
        assert!(res.is_ok());
        let c = res.unwrap(/*:test:*/);
        assert_eq!(c.name, "He·bar".to_string());

        assert!(Commodity::from("$".to_string()).is_ok());
        assert!(Commodity::from("¢".to_string()).is_ok());
        assert!(Commodity::from("£".to_string()).is_ok());
        assert!(Commodity::from("¤".to_string()).is_ok());
        assert!(Commodity::from("¥".to_string()).is_ok());
    }

    #[test]
    fn id_699aadb1_d1ba_44b6_ae6a_158cf5be13e5__commodity_err() {
        assert!(Commodity::from("123".to_string()).is_err());
        assert!(Commodity::from("-USD".to_string()).is_err());
        assert!(Commodity::from("_USD".to_string()).is_err());
        assert!(Commodity::from("·USD".to_string()).is_err());
        assert!(Commodity::from("He:bar".to_string()).is_err());
        assert!(Commodity::from("He bar".to_string()).is_err());
    }

    /*
    todo: commodity tests as part of AccountTreeNode vs. TxnAccount
    #[test]
    fn id_e11d5d26_c149_4d8c_b150_5cb2e2f80608__atn_commodity() {
        let atn = AccountTreeNode::from(
            "a",
            Some(Commodity::from("He·bar".to_string()).unwrap(/*:test:*/)),
        )
        .unwrap(/*:test:*/);

        assert!(!atn.commodity.name.is_empty());
        assert_eq!(atn.commodity.name, "He·bar".to_string());
    }
     */

    #[test]
    fn atn_is_root() {
        let atn_a = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        let atn_ab = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        let atn_abc = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b:c")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        assert_eq!(atn_a.is_root(), true);
        assert_eq!(atn_ab.is_root(), false);
        assert_eq!(atn_abc.is_root(), false);
    }
    #[test]
    fn atn_my_parent_is_root() {
        let atn_a = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        let atn_ab = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        let atn_abc = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b:c")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        assert_eq!(atn_a.my_parent_is_root(), false);
        assert_eq!(atn_ab.my_parent_is_root(), true);
        assert_eq!(atn_abc.my_parent_is_root(), false);
    }

    #[test]
    fn atn_is_parent() {
        let parent = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b")
                .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        let leaf = TxnAccount {
            atn: Arc::new(AccountTreeNode::from("a:b:c")
            .unwrap(/*:test:*/)),
            comm: Arc::new(Commodity::default()),
        };
        assert!(parent.is_parent_of(&leaf));
        assert!(!parent.is_parent_of(&parent));
    }

    #[test]
    fn atn_ok_display() {
        let atn = AccountTreeNode::from("a:b:c")
        .unwrap(/*:test:*/);

        let atn_str = format!("{atn}");
        assert_eq!(atn_str, String::from("a:b:c"));
    }

    #[test]
    fn id_88c5cb23_5995_4b93_8c26_a3f7374e96d9__atn_root() {
        let atn = AccountTreeNode::from("a").unwrap(/*:test:*/);

        assert_eq!(atn.depth, 1);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "");
        assert_eq!(atn.account, "a");
        assert_eq!(atn.name, "a");
    }

    #[test]
    fn id_fc69f9b2_1faf_425c_87d3_aed63d66171b__atn_two() {
        let atn = AccountTreeNode::from("a:b").unwrap(/*:test:*/);

        assert_eq!(atn.depth, 2);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a");
        assert_eq!(atn.account, "a:b");
        assert_eq!(atn.name, "b");
    }

    #[test]
    fn id_38c103d3_4cc7_4af7_86cd_bf24ca37d026__atn_three() {
        let atn = AccountTreeNode::from("a:b:c").unwrap(/*:test:*/);

        assert_eq!(atn.depth, 3);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a:b");
        assert_eq!(atn.account, "a:b:c");
        assert_eq!(atn.name, "c");
    }

    #[test]
    fn id_76a6c300_5569_4e1d_a0a1_ae2ee31d919a__atn_more() {
        let atn = AccountTreeNode::from("a:b:c:leaf").unwrap(/*:test:*/);

        assert_eq!(atn.depth, 4);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a:b:c");
        assert_eq!(atn.account, "a:b:c:leaf");
        assert_eq!(atn.name, "leaf");
    }

    #[test]
    fn id_55407835_34d8_4de9_a362_4172f0e4d54b__err_empty() {
        let atn = AccountTreeNode::from("");
        assert!(atn.is_err());
    }

    #[test]
    fn id_0609e72f_c509_4b62_950e_fce432122d10__err_empty_sub() {
        // new tests
        assert!(AccountTreeNode::from("a:").is_err());
        assert!(AccountTreeNode::from(":a").is_err());
        assert!(AccountTreeNode::from("a::b").is_err());
        assert!(AccountTreeNode::from("::").is_err());
        // old tests
        assert!(AccountTreeNode::from(":").is_err());
        assert!(AccountTreeNode::from(": :").is_err());
    }
}
