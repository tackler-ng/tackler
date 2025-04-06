#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### TEP-1016: FLAT BALANCE
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
cmp_result $module $test_name txt balgrp
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
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
echo ": ok"
