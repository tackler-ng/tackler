#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### REPORTING: ERRORS
###
module=reporting
mode="error"

#####################################################################
#
# test: 88ef188c-b8f8-47f0-88b6-7af3577aa133
# desc: conf: report.targets unknown report
test_name=unknown-report
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex-unknown-report.toml \
    2>&1 | grep "Tackler error: Configuration .*/reporting/ex-unknown-report.toml': Invalid .* does-not-exists"

echo "check: ok"

#####################################################################
# test: 31e0bd80-d4a9-4d93-915d-fa2424aedb84
# desc: conf: Unknown group-by operator
test_name=unknown-group-by
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex-unknown-group-by.toml \
    2>&1 | grep "Tackler error: Configuration .*/reporting/ex-unknown-group-by.toml': Unknown group-by selector"

echo "check: ok"

#####################################################################
#
# test: f932155c-5953-4b24-8c33-cf4d49c22fa3
# desc: unknown equity account (not defined in CoA)
test_name=unknown-equity-account
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex-equity-tep1014.toml \
    --input.file $SUITE_PATH/$module/ex/eq-tep1014.txn \
    2>&1 | grep 'Tackler error: Unknown `equity.equity-account` and `strict` mode is on'

echo "check: ok"

#####################################################################
#
# test: 33c4fd0b-03c7-48bc-b4cd-0aac72e11beb
# desc: console output with export
rm -f $OUTPUT_DIR/*
test_name=console-output-with-export
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --exports identity \
    2>&1 | grep 'error: .* required arguments were not provided'

echo "check: ok"
