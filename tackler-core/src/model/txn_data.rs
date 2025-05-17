/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use itertools::Itertools;

use crate::kernel::Predicate;
use crate::kernel::hash::Hash;
use crate::model::{TxnRefs, Txns, transaction};
use crate::tackler;
use tackler_api::filters::FilterDefinition;
use tackler_api::metadata::items::{MetadataItem, TxnFilterDescription, TxnSetChecksum};
use tackler_api::metadata::{Checksum, Metadata};

pub struct TxnData {
    metadata: Option<Metadata>,
    txns: Txns,
    hash: Option<Hash>,
}

pub struct TxnSet<'a> {
    pub(crate) metadata: Option<Metadata>,
    pub(crate) txns: TxnRefs<'a>,
}

impl TxnSet<'_> {
    #[must_use]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.txns.is_empty()
    }
}

impl TxnData {
    #[must_use]
    pub fn len(&self) -> usize {
        self.txns.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.txns.is_empty()
    }

    pub fn from(mdi_opt: Option<MetadataItem>, txns: Txns, hash: &Option<Hash>) -> TxnData {
        let metadata = mdi_opt.map(Metadata::from_mdi);

        let mut t = txns;
        t.sort_by(transaction::ord_by_txn);

        TxnData {
            metadata,
            txns: t,
            hash: hash.clone(),
        }
    }

    fn make_metadata(&self, txns: &TxnRefs<'_>) -> Result<Metadata, tackler::Error> {
        let mut metadata = match &self.metadata {
            Some(md) => Metadata::from_metadata(md),
            None => Metadata::new(),
        };

        if let Some(hash) = &self.hash {
            let new_tsc_mdi = MetadataItem::TxnSetChecksum(TxnSetChecksum {
                size: txns.len(),
                hash: calc_txn_checksum(txns, hash)?,
            });

            metadata.push(new_tsc_mdi);
        }

        Ok(metadata)
    }

    /// # Errors
    /// Returns `Err` in case resulting Txn Set is not valid (e.g. there are missing UUIDs)
    pub fn filter(&self, tf: &FilterDefinition) -> Result<TxnSet<'_>, tackler::Error> {
        let refvec: TxnRefs<'_> = self.txns.iter().filter(|txn| tf.eval(txn)).collect();

        let mut metadata = self.make_metadata(&refvec)?;
        let filter_mdi = MetadataItem::TxnFilterDescription(TxnFilterDescription::from(tf.clone()));
        metadata.push(filter_mdi);

        Ok(TxnSet {
            metadata: Some(metadata),
            txns: refvec,
            //hash: &self.hash,
        })
    }

    /// # Errors
    /// Returns `Err` in case resulting Txn Set is not valid (e.g. there are missing UUIDs)
    pub fn get_all(&self) -> Result<TxnSet<'_>, tackler::Error> {
        let txns: TxnRefs<'_> = self.txns.iter().collect();

        let metadata = if self.hash.is_some() || self.metadata.is_some() {
            Some(self.make_metadata(&txns)?)
        } else {
            None
        };

        Ok(TxnSet { metadata, txns })
    }
}

fn calc_txn_checksum(txns: &TxnRefs<'_>, hasher: &Hash) -> Result<Checksum, tackler::Error> {
    let uuids: Result<Vec<String>, tackler::Error> = txns
        .iter()
        .map(|txn| if let Some(uuid) = txn.header.uuid { Ok(uuid.to_string()) } else {
            let msg = "Txn without UUID. Txn UUID is mandatory with transaction set checksum calculation.";
            Err(msg.into())
        })
        .collect();

    let mut u = uuids?;
    u.sort();

    let dups: Vec<String> = u.iter().duplicates().cloned().collect();
    if !dups.is_empty() {
        let dups_count = dups.len();
        let msg = if dups_count < 10 {
            format!(
                "Found {} duplicate txn uuids with txn set checksum.\nDuplicate ids are:\n{}",
                dups.len(),
                dups.join(",\n")
            )
        } else {
            format!(
                "Found {} duplicate txn uuids with txn set checksum.\nFirst ten duplicate ids are:\n{}",
                dups.len(),
                dups[0..10].join(",\n")
            )
        };
        return Err(msg.into());
    }

    let cs = hasher.checksum(&u, "\n".as_bytes());
    Ok(cs)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    // desc: check that uuid::to_string returns normalized lower-case UUID
    fn uuid_as_lower_case() {
        let uuid_ref = "e274c99e-1ebb-45e8-832d-58caf54ed95f";
        let uuid_mixed = "E274C99E-1ebb-45e8-832d-58Caf54Ed95f";
        let uuid_upper = "E274C99E-1EBB-45E8-832D-58CAF54ED95F";

        assert_eq!(
            Uuid::parse_str(uuid_ref).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
        assert_eq!(
            Uuid::parse_str(uuid_mixed).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
        assert_eq!(
            Uuid::parse_str(uuid_upper).unwrap(/*:test:*/).to_string(),
            uuid_ref
        );
    }
}
