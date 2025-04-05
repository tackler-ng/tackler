/// Metadata: Git Storage
#let md-git-storage(data) = {
  table(
    columns: 2,
    [ commit ], data.at("commit", default: [n/a]),
    [ ref ], data.at("ref", default: [n/a]),
    [ dir ], data.at("dir", default: [n/a]),
    [ suffix ], data.at("suffix", default: [n/a]),
    [ message ], data.at("message", default: [n/a]),
  )
}

/// Metadata: Txn Set Checksum
#let md-txn-set-checksum(data) = {
  table(
    columns: 2,
    [ size ], [ #data.at("size", default: [n/a]) ], 
    [ #data.hash.algorithm ], [ #data.hash.value ]
  )
}

/// Metadata: Account Selector Checksum
#let md-acc-selector-checksum(data) = {
  table(
    columns: 2,
    [ #data.hash.algorithm ], [ #data.hash.value ]
  )
}


/// Generic JSON table 
#let json-table(data, keys) = {
  table(
    columns: keys.len(),
    ..keys,
    ..data.map(
      row => keys.map(
        key => row.at(key, default: [n/a])
      )
    ).flatten()
  )
}


#let bal = json("data/audit.bal.json")

//
// Report starts here
//

= Metadata 

#for mdi in bal.metadata.items {
  if "GitInputReference" in mdi and  mdi.at("GitInputReference") != "" {
    [ == GIT Storage ]
    md-git-storage(mdi.at("GitInputReference"))
  } else if "TxnSetChecksum" in mdi and  mdi.at("TxnSetChecksum") != "" {
    [ == Txn Set Checksum ]
    md-txn-set-checksum(mdi.at("TxnSetChecksum")) 
  } else if "AccountSelectorChecksum" in mdi and  mdi.at("AccountSelectorChecksum") != "" {
    [ == Account Selector Checksum ]
    md-acc-selector-checksum(mdi.at("AccountSelectorChecksum")) 
  } 
}

= #bal.title

#let keys = ("accountSum", "accountTreeSum", "account")
#json-table(bal.balances, keys)

#json-table(bal.deltas, ("delta", ))

