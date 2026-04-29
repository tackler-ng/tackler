#
# Tackler-NG 2025-2026
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### TAGS: ERRORS
###
module=tags
mode="error"

#####################################################################
#
# test: a79e871f-0b68-4172-9337-25c719b9ac11
# desc: unknown tag
test_name=unknown-tag
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-tag.txn \
    2>&1 | grep 'semantic error - Unknown tag:.*saunatonttu2'

echo "check: ok"
