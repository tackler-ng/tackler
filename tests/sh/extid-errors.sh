#
# Tackler-NG 2026
# SPDX-License-Identifier: Apache-2.0
#

set -eu

source $TEST_DIR/lib/utils.sh

###
### EXT-ID: ERRORS
###
module=extid
mode="error"

#####################################################################
#
# test: 22a82798-0fb5-45de-b31b-1755baa3e622
# desc: unique with duplicates
test_name=duplicates
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/extid-uniq-true.toml \
    --input.fs.dir $SUITE_PATH/$module/ok/txns-dups \
    2>&1 | grep 'Tackler error:.* 4 duplicate external'

echo "check: ok"
