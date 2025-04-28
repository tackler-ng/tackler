// Report Metadata Section
//
// Report metadata is a list of optional metadata items.
// Metadata could be empty an empty list.
//
// List of all possible metadata items:
// https://docs.rs/tackler-api/latest/tackler_api/metadata/items/index.html
//

////////////////////////////////////////////////////////////////////////////////
// Metadata: Git Storage
//
// API Documentation
// https://docs.rs/tackler-api/latest/tackler_api/metadata/items/struct.GitInputReference.html
//
#let md-git-storage(data) = {
  table(
    columns: 2,
    [ Reference ], data.at("ref", default: [FIXED by commit]),
    [ Directory ], data.at("dir"),
    [ Extension ], data.at("extension"),
    [ Commit ], data.at("commit"),
    [ Author ], data.at("author"),
    [ Date ], data.at("date"),
    [ Summary ], data.at("subject"),
  )
}

////////////////////////////////////////////////////////////////////////////////
// Metadata: Txn Set Checksum
//
// API Documentation
// https://docs.rs/tackler-api/latest/tackler_api/metadata/items/struct.TxnSetChecksum.html
//
#let md-txn-set-checksum(data) = {
  table(
    columns: 2,
    [ Size ], [ #data.at("size", default: [n/a]) ],
    [ #data.hash.algorithm ], [ #data.hash.value ]
  )
}

////////////////////////////////////////////////////////////////////////////////
// Metadata: Account Selector Checksum
//
// API Documentation
// https://docs.rs/tackler-api/latest/tackler_api/metadata/items/struct.AccountSelectorChecksum.html
//
#let md-acc-selector-checksum(data) = {
  table(
    columns: 2,
    [ #data.hash.algorithm ], [ #data.hash.value ]
  )
}


////////////////////////////////////////////////////////////////////////////////
// Generic JSON table
//
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

////////////////////////////////////////////////////////////////////////////////
//
// Report starts here
//
////////////////////////////////////////////////////////////////////////////////

= Balance Report

== Report Metadata

#for mdi in bal.metadata.items {
  if "GitInputReference" in mdi and  mdi.at("GitInputReference") != "" {
    [ === GIT Storage ]
    md-git-storage(mdi.at("GitInputReference"))
  } else if "TxnSetChecksum" in mdi and  mdi.at("TxnSetChecksum") != "" {
    [ === Txn Set Checksum ]
    md-txn-set-checksum(mdi.at("TxnSetChecksum")) 
  } else if "AccountSelectorChecksum" in mdi and  mdi.at("AccountSelectorChecksum") != "" {
    [ === Account Selector Checksum ]
    md-acc-selector-checksum(mdi.at("AccountSelectorChecksum")) 
  } 
}

== #bal.title

#let keys = ("accountSum", "accountTreeSum", "account")
#json-table(bal.balances, keys)

#json-table(bal.deltas, ("delta", ))

