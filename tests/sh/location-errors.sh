#
# Tackler-NG 2025-2026
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### LOCATION: ERRORS
###
module=location
mode="error"

#####################################################################
#
# test: d948d0cd-d06c-4772-be5c-46e8875e4910
# desc: cli: error propagation: invalid value
test_name=invalid-value
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ex/values.txn \
    2>&1 | grep 'semantic error - Value .* Latitude: 90.1'

echo "check: ok"

#####################################################################
#
# test: f1951382-c746-4be0-b367-f903c8c9fb18
# desc: cli: error propagation: invalid format
test_name=invalid-format
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ex/format.txn \
    2>&1 | grep 'invalid txn metadata location'

echo "check: ok"
