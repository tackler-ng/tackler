#
# Tackler-NG 2026
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh


###
### REPORTING: Accounts
###
module=reporting/accounts

#####################################################################
#
# basic-01
#
# test: 471c68df-734e-476c-80f1-aa7f2b170a81
# description:
#  - "basic accounts export functionality"
#  - "sorting order of accounts in reports"
rm -f $OUTPUT_DIR/*
test_name=basic-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.toml \
    --input.file $SUITE_PATH/$module/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --accounts "" \
    --exports "equity" "accounts" \
    --strict.mode false \

echo -n "check:"
cmp_result $module ${test_name} txt bal "."
cmp_result $module ${test_name} txt balgrp "."
cmp_result $module ${test_name} txt reg "."
cmp_result $module ${test_name} txn equity "."

cmp_result $module ${test_name} toml accounts "."
echo ": ok"


#####################################################################
#
# strict-01
#
# test: 5cf98224-cf9a-4455-90ea-9a0916c5ee2e
# description:
#  - "generated accounts.toml can be used as part of configuration"
#  - "accounts export by configuration"
rm -f $OUTPUT_DIR/*
rm -f $SUITE_PATH/$module/strict-01.accounts.toml
test_name=strict-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.toml \
    --input.file $SUITE_PATH/$module/basic-01.txn \
    --output.dir $SUITE_PATH/$module \
    --output.prefix ${test_name} \
    --reports "" \
    --exports "accounts" \
    --strict.mode false \

# test: 2fe1dd09-fefc-4a19-ad2b-03bf77c73bf5
# desc: account selector doesn't afffect the account export
$TACKLER_SH \
    --config $SUITE_PATH/$module/strict-by-accounts.toml \
    --input.file $SUITE_PATH/$module/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name

echo -n "check:"
cmp_result $module ${test_name} txn identity "."
cmp_result $module ${test_name} toml accounts "."
echo ": ok"
