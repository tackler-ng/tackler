#
# Tackler-NG 2025-2026
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### COMMODITY: ERRORS
###
module=commodity
mode="error"

#####################################################################
#
# test: 561ca500-ec5b-41b5-8466-eb82362a06ea
# desc: Empty posting commodity
test_name=empty-commodity
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex.toml \
    --input.file $SUITE_PATH/$module/ex/empty-postcomm.txn \
    2>&1 | grep 'semantic error - Empty commodity .*permit-empty-commodity'

echo "check: ok"

#####################################################################
# test: 40592a4d-54b4-4270-807f-8bd31b122a08
# desc: Unknown txn commodity
test_name=unknown-txn-commodity
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-txncomm.txn \
    2>&1 | grep 'semantic error - Unknown.*JPY'

echo "check: ok"

#####################################################################
#
# test: 429c3406-2b73-4b8b-8b62-bd27cd2bb351
# desc: Unknown posting commodity
test_name=unknown-posting-commodity
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-postcomm.txn \
    2>&1 | grep 'semantic error - Unknown.*AAPL'

echo "check: ok"
