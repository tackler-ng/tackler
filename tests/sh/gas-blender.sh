#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### COMMODITY: Gas Blender
###
module=commodity

#####################################################################
#
# Gas Blender: Mix logs
#
# test: 6cc57a39-49ad-4563-b349-5fa28766b575
rm -f $OUTPUT_DIR/*
test_name=gas-blender-mix-log
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/gb-stock.toml \
    --accounts '^User:.*:.*' \
    --reports "register"

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name json reg
echo ": ok"

#####################################################################
#
# Gas Blender: Tank pressures
#
# test: 7f847843-d14d-4bea-8f8d-beb94ea675b4
rm -f $OUTPUT_DIR/*
test_name=gas-blender-tanks
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/gb-stock.toml \
    --accounts '^Gas:.*:Tank.*' \
    --reports "balance"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
echo ": ok"

#####################################################################
#
# Gas Blender: Saldo
#
# test: 334203d1-a147-48ab-8ec2-50ce006cc9cc
rm -f $OUTPUT_DIR/*
test_name=gas-blender-saldo
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/gb-stock.toml \
    --accounts '^User:.*:.*' \
    --reports "balance"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
echo ": ok"

#####################################################################
#
# Gas Blender: Billing
#
# test: 7b8073e2-8264-4eba-b282-5bfb8e1718d2
rm -f $OUTPUT_DIR/*
test_name=gas-blender-billing
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/gb-billing.toml

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
echo ": ok"

