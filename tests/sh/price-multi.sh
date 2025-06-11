#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### PRICE: MULTI
###
module=price
mode=""

#
# multi-00
#
# test: 83d831d5-5947-4906-9bf0-d561c6c48349
# desc: multi, lookup none
rm -f $OUTPUT_DIR/*
test_name=multi-00
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/multi.txn \
    --exports "identity" "equity" \
    --price.lookup-type "none"

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
# multi-01
#
# test: 75c3846c-c291-4957-a5dd-80d0f282e084
# desc: multi-value: txn-time
rm -f $OUTPUT_DIR/*
test_name=multi-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/multi.txn \

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity
echo ": ok"

#
# multi-02
#
# test: a060c24f-92cd-4b44-9b7a-7152cfa59eb7
# desc: multi-value: given-time
rm -f $OUTPUT_DIR/*
test_name=multi-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/multi.txn \
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
echo ": ok"

#
# multi-03
#
# test: 0c659e24-542e-43c9-9bc6-65b39dd0c611
# desc: multi-value: txn-filter, single commodity
rm -f $OUTPUT_DIR/*
test_name=multi-03
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/multi.txn \
    --price.lookup-type "given-time" \
    --price.before "2024-04-01" \
    --reports register \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterPostingCommodity": { "regex": "bbb" }}}'

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name json reg
echo ": ok"

#
# multi-04
#
# test: 06a2355b-3567-45ed-b071-dd397febc97f
# desc: multi-value: txn commodity is target commodity
rm -f $OUTPUT_DIR/*
test_name=multi-04
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/multi-in-tcklr.txn \
    --price.lookup-type "given-time" \
    --price.before "2024-04-01"

echo -n "check:"
# The multi-02 is same as this
cmp_result_ref $module multi-02 $test_name txt bal
cmp_result_ref $module multi-02 $test_name txt balgrp
cmp_result_ref $module multi-02 $test_name txt reg
cmp_result_ref $module multi-02 $test_name json bal
cmp_result_ref $module multi-02 $test_name json balgrp
cmp_result_ref $module multi-02 $test_name json reg

cmp_result_ref $module multi-02 $test_name txn equity
echo ": ok"


#
# multi-vp-01
#
# test: 6ad08423-c2d1-4667-9084-10920edfef4c
# desc: multi-vp: txn-time, unit-price
rm -f $OUTPUT_DIR/*
test_name=multi-vp-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/$test_name.txn

echo -n "check:"
cmp_result_ref $module multi-01 $test_name txt bal
cmp_result_ref $module multi-01 $test_name txt balgrp
cmp_result_ref $module multi-01 $test_name txt reg
cmp_result_ref $module multi-01 $test_name json bal
cmp_result_ref $module multi-01 $test_name json balgrp
cmp_result_ref $module multi-01 $test_name json reg

cmp_result_ref $module multi-01 $test_name txn equity
echo ": ok"

#
# multi-vp-02
#
# test: 83bf6ec0-a100-490b-9cff-e58775554963
# desc: multi-vp: txn-time, inverted unit-price
rm -f $OUTPUT_DIR/*
test_name=multi-vp-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/$test_name.txn

echo -n "check:"
cmp_result_ref $module multi-01 $test_name txt bal
cmp_result_ref $module multi-01 $test_name txt balgrp
cmp_result_ref $module multi-01 $test_name txt reg
cmp_result_ref $module multi-01 $test_name json bal
cmp_result_ref $module multi-01 $test_name json balgrp
cmp_result_ref $module multi-01 $test_name json reg

cmp_result_ref $module multi-01 $test_name txn equity
echo ": ok"

#
# multi-vp-03
#
# test: 34716ab3-3dd8-4873-8af9-868b50b32bbd
# desc: multi-vp: txn-time, txn with multiple comms
rm -f $OUTPUT_DIR/*
test_name=multi-vp-03
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/$test_name.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity
echo ": ok"

#
# multi-vp-04
#
# test: 12f7784d-6319-40d7-9942-f8be219106a8
# desc: multi-vp: txn-time, txn with multiple, missing comms
rm -f $OUTPUT_DIR/*
test_name=multi-vp-04
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --input.file $SUITE_PATH/$module/ok/$test_name.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity
echo ": ok"

#
# multi-odd-01
#
# test: cbfdf9ae-5f57-4e64-80b4-feec41b127f4
# desc: multi-odd: txn-time, txns with diff multiple comms, price times
rm -f $OUTPUT_DIR/*
test_name=multi-odd-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --pricedb $SUITE_PATH/$module/ok/multi-odd-times.db \
    --price.lookup-type "last-price" \
    --input.file $SUITE_PATH/$module/ok/multi-vp-04.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity
echo ": ok"

#
# timestamps-01
#
# test: 3029acb0-ff10-4095-895a-7fa0ba272fb5
# desc: multi-timestamps: last-price, prices with timestamps and offsets
rm -f $OUTPUT_DIR/*
test_name=multi-timestamps-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi.toml \
    --pricedb $SUITE_PATH/$module/ok/multi-timestamps.db \
    --price.lookup-type "last-price" \
    --input.file $SUITE_PATH/$module/ok/multi-vp-04.txn \
    --reports register

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name json reg
echo ": ok"
