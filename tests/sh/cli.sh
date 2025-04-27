#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### CLI
###
module=cli

#
# test: cd54250a-8af2-4daa-9d8e-7870b5987da9
# desc: unknown account ok case
rm -f $OUTPUT_DIR/*
test_name=unknown-acc-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-acc-01.txn \
    --exports identity \

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#
# test: fd250432-9b13-4cdd-83a1-1aedff1593ed
# desc: UTF-8 output
rm -f $OUTPUT_DIR/*
test_name=console-03
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic-json.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $SUITE_PATH/$module/ok/console-txns/id-chars.txn \
    --reports balance balance-group register \
    --exports identity equity

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/console-txns/id-chars.txn \
    --reports balance balance-group register \
     > "${OUTPUT_DIR}/${test_name}.stdout.txt" \
     2> "${OUTPUT_DIR}/${test_name}.stderr.txt"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg

cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity

cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txt stdout
cmp_result $module $test_name txt stderr
echo ": ok"
