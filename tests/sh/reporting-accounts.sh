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
rm -f $OUTPUT_DIR/*
test_name=basic-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.toml \
    --input.file $SUITE_PATH/$module/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --reports "" \
    --exports "accounts" \
    --strict.mode false \

echo -n "check:"
cmp_result $module ${test_name} toml accounts "."
echo ": ok"


#####################################################################
#
# strict-01
#
# test: 5cf98224-cf9a-4455-90ea-9a0916c5ee2e
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

$TACKLER_SH \
    --config $SUITE_PATH/$module/strict-by-accounts.toml \
    --input.file $SUITE_PATH/$module/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \

echo -n "check:"
cmp_result $module ${test_name} txn identity "."
echo ": ok"
