#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### PRICE: STRICT = TRUE
###
module=price
mode="strict=true"

#
# price-00
#
# test: 7cbe3cb4-1b23-44bb-8e5e-ba2c8991e589
# desc: strict, none
rm -f $OUTPUT_DIR/*
test_name=price-00
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type "none"


echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity

cmp_result_ref $module price $test_name txn identity
echo ": ok"

#
# price-01
#
# test: 6ddcb0d4-7dc6-4909-a1b4-ecbb5a33a186
# desc: strict, txn-time
# balance: 1.001 + 12.001 + 24.001 + 31.001 + \
#          1000.001 + 1200.001 + 2400.001 + 2900.001 + \
#          100000.001 + 120000.001 + 240000.001 + 310000.001
#        = 777568.012
# bal-grp:
#      g1: 1.001 + 12.001 + 24.001 + 31.001                  = 68.004
#      g2: 1000.001 + 1200.001 + 2400.001 + 2900.001         = 7500.004
#      g3: 100000.001 + 120000.001 + 240000.001 + 310000.001 = 770000.004
rm -f $OUTPUT_DIR/*
test_name=price-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type txn-time

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity

cmp_result_ref $module price $test_name txn identity
echo ": ok"

#
# price-02
#
# test: af19f8e7-a6f2-4f64-8c5a-b774dba4f58c
# desc: strict, last-time
rm -f $OUTPUT_DIR/*
test_name=price-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type last-price

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity

cmp_result_ref $module price $test_name txn identity
echo ": ok"

#
# price-03
#
# test: 02057635-5436-4457-b597-fa3f3e2e9df3
# desc: price and filter metadata with console
# test: 4075e741-605b-4e67-ab7d-0d13f38956ca
# desc: strict, given-time, filter
rm -f $OUTPUT_DIR/*
test_name=price-03
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type given-time \
    --price.before 2024-04-01 \
    --api-filter-def \
    '{ "txnFilter": { "TxnFilterTxnTSEnd": { "end": "2024-04-01T00:00:00Z" }}}' \
     > "${OUTPUT_DIR}/${test_name}.stdout.txt" \
     2> "${OUTPUT_DIR}/${test_name}.stderr.txt"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type given-time \
    --price.before 2024-04-01 \
    --api-filter-def \
    '{ "txnFilter": { "TxnFilterTxnTSEnd": { "end": "2024-04-01T00:00:00Z" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg

cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity

cmp_result $module $test_name txt stdout
cmp_result $module $test_name txt stderr
echo ": ok"

#
# price-04
#
# test: 0c600957-5c15-42d0-9bb9-b618ad7d597f
# desc: strict, given-time, filter
rm -f $OUTPUT_DIR/*
test_name=price-04
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type given-time \
    --price.before 2024-03-31 \
    --api-filter-def \
    '{ "txnFilter": { "TxnFilterTxnTSEnd": { "end": "2024-03-31T00:00:00Z" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#
# price-05
#
# test: 39c60c8b-b999-4305-8c57-5c06e99cc919
# desc: strict, given-time
rm -f $OUTPUT_DIR/*
test_name=price-05
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type "given-time" \
    --price.before "2024-04-01"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity

cmp_result_ref $module price $test_name txn identity
echo ": ok"

#
# price-06
#
# test: f7211562-c69d-4bab-a768-65b15803efdf
# desc: strict, given-time
rm -f $OUTPUT_DIR/*
test_name=price-06
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type "given-time" \
    --price.before "2024-03-31"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity

cmp_result_ref $module price $test_name txn identity
echo ": ok"

#
# price-07
#
# test: e1754c8d-ac71-4344-942f-0c78d69bb1b3
# desc: strict, given-time == last-price
rm -f $OUTPUT_DIR/*
test_name=price-07
echo "test: $module/$test_name: $mode"

# Result is same as with price-02
$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix price-07 \
    --config $SUITE_PATH/$module/price-strict.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --price.lookup-type "given-time" \
    --price.before "2025-01-01"

echo -n "check:"
# Result is same as with price-02
cmp_result_ref $module price-02 $test_name txt bal
cmp_result_ref $module price-02 $test_name txt balgrp
cmp_result_ref $module price-02 $test_name txt reg
cmp_result_ref $module price-02 $test_name json bal
cmp_result_ref $module price-02 $test_name json balgrp
cmp_result_ref $module price-02 $test_name json reg
cmp_result_ref $module price-02 $test_name txn equity

# identity should be same as basic price identity
cmp_result_ref $module price $test_name txn identity
echo ": ok"
