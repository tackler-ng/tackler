tackler \
   --config conf/hq-conf.toml \
   --api-filter-def \
      '{ "txnFilter": { "TxnFilterTxnTags" : { "regex": "store-a" }}}' \
   "$@"
