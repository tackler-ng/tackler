#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### PRICE: MULTI / MISS
###
module=price
mode="miss"

#
# multi-miss-01
#
# test: ac8bb20d-8802-401d-9130-2349f0643aee
# desc: multi, miss: txn-time
rm -f $OUTPUT_DIR/*
test_name=multi-miss-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi-miss.toml \
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
# multi-miss-vp-01
#
# test: 9fa4a692-1e00-4eec-9773-886ef79cb740
# desc: multi-miss-vp: txn-time, unit-price
rm -f $OUTPUT_DIR/*
test_name=multi-miss-vp-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi-miss.toml \
    --input.file $SUITE_PATH/$module/ok/multi-vp-01.txn

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
# multi-miss-vp-04
#
# test: 616b5b97-ede1-4a8b-9767-d6c0ca5206d8
# desc: multi-miss-vp: txn-time, txn with multiple, missing comms
rm -f $OUTPUT_DIR/*
test_name=multi-miss-vp-04
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-multi-miss.toml \
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
