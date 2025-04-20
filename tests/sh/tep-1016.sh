#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### TEP-1016: FLAT BALANCE / AUDIT
###
module=audit

#####################################################################
#
# test: 8d9b68a7-32f6-451d-ab74-08f03016f7c7
# desc: bal.type = flat, balgrp.type = tree
rm -f $OUTPUT_DIR/*
test_name=tep1016-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/tep1016-01.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts '.*' \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name json balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 4bf2653c-7772-45fe-a613-11617215ad2b
# desc: bal.type = tree, balgrp.type = flat
rm -f $OUTPUT_DIR/*
test_name=tep1016-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/tep1016-02.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts '.*' \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name json balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
echo ": ok"

###
### TEP-1016: FLAT BALANCE / COMMODITY
###
module=commodity

tep1016_commodity_test () {
    local test_name=$1

    echo "test: $module/tep1016-${test_name}: "

    rm -f $OUTPUT_DIR/*
    $TACKLER_SH \
        --config $SUITE_PATH/$module/tep1016.toml \
        --output.dir $OUTPUT_DIR \
        --output.prefix "tep1016-${test_name}" \
        --input.file $SUITE_PATH/$module/ok/$test_name.txn

    echo -n "check:"
    cmp_result $module "tep1016-${test_name}" txt bal
    cmp_result $module "tep1016-${test_name}" json bal
    cmp_result $module "tep1016-${test_name}" txt balgrp
    cmp_result $module "tep1016-${test_name}" json balgrp
    cmp_result $module "tep1016-${test_name}" txn equity
    echo ": ok"
}

#####################################################################
#
# valpos
#

# test: 895d913f-649d-4779-82dd-5da27ff48423
# desc: flat with commodity (valpos-01)
tep1016_commodity_test valpos-01

# test: a5d7770b-1680-4f90-a7cd-7de38dbb1487
# desc: flat with commodity (valpos-02)
tep1016_commodity_test valpos-02

# test: e2eadf49-1ac4-4b7a-84b3-fa600d628ad6
# desc: flat with commodity (valpos-03)
tep1016_commodity_test valpos-03

