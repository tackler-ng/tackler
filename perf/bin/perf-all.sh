#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Tackler-NG 2018-2025
# SPDX-License-Identifier: Apache-2.0
#
#############################################################################

exe_dir=$(dirname $0)

data_dir="$exe_dir/../data/perf-data"

reports="balance balance-group register"
#reports="balance"

#sets="1e1 1e2 1e3 1e4 1e5 1e6"
sets="1e3 1e4 1e5 1e6"
#sets="1e3"
#sets="1e5"
#sets="1e6"

versions="25.01.1"
#versions="devel"

fltStr="base64:"$(cat << EOF | base64 --wrap=0
{ "txnFilter": { "TxnFilterAND" : { "txnFilters" : [ { "TxnFilterTxnCode": { "regex": "#.*" }},  { "TxnFilterTxnDescription": { "regex": "txn-.*" } } ] } } }
EOF
)

for s in $sets; do
	(cd $data_dir; git checkout txns-$s)
for v in $versions; do
for r in $reports "balance register"; do

#for frmt in txt json; do
for frmt in txt; do

for filter in "" "$fltStr"; do
#for filter in ""; do

if [ -n "$filter" ]; then
       flt="filter"
else
       flt="all"
fi


echo "$(date "+%H:%M:%S") run: $v fs $s $r $frmt $flt"
$exe_dir/perf-run.sh dist/tackler-$v fs "$s" "$r" "$frmt" "$filter"

done
done
done
done
done


for v in $versions; do
for s in $sets; do
for r in balance; do
for frmt in txt; do

filter=""

echo "$(date "+%H:%M:%S") run: $v git $s $r $frmt all"
$exe_dir/perf-run.sh dist/tackler-$v git $s $r "$frmt" "$filter"

done
done
done
done

(cd $data_dir; git checkout set-1e3)
