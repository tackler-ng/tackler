#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### PRICE: STRICT = FALSE
###
module=price
mode="strict=false"

#
# price-00
#
# test: 42297bb3-c816-496f-83a3-924f52702e42
# desc: cli: lookup-type none
rm -f $OUTPUT_DIR/*
test_name=price-00
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
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
# price-00-00
#
# test: e9e2bc45-94d7-4eaa-9e0d-20667ac8a365
# desc: no target commodity match
rm -f $OUTPUT_DIR/*
test_name=price-00-00
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
    --accounts "e:conv" \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity "€"


echo -n "check:"
cmp_result_ref $module price-00 $test_name txt bal
cmp_result_ref $module price-00 $test_name txt balgrp
# output is different when price conv is activated, even with no match
cmp_result $module $test_name txt reg
cmp_result_ref $module price-00 $test_name json bal
cmp_result_ref $module price-00 $test_name json balgrp
cmp_result_ref $module price-00 $test_name json reg
cmp_result_ref $module price-00 $test_name txn equity
cmp_result_ref $module price    $test_name txn identity

echo ": ok"

#
# price-01
#
# test: 29b8ecea-bb2b-4a66-b0a2-178cb7a9f1b4
# desc: txn-time
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
    --config $SUITE_PATH/$module/price.toml \
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
# test: 8a29b198-6e87-4d4c-885d-a888be485d92
# desc: last-price
rm -f $OUTPUT_DIR/*
test_name=price-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
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
# price-05
#
# test: e767afa7-cd1e-4979-a76a-a1371a5b2a90
# desc: given-time
rm -f $OUTPUT_DIR/*
test_name=price-05
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
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
