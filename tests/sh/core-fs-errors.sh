#
# Tackler-NG 2025-2026
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### CORE: FS: ERRORS
###
module=core
mode="error"

#####################################################################
#
# test: 8722a7e2-04fd-4b32-a259-e16ca882a725
# desc: empty transaction set
test_name=fs-empty-txn-set
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.fs.dir $SUITE_PATH/$module/ex/empty-txns \
    --input.fs.ext txn \
    2>&1 | grep 'Tackler error: Txn Data: .* (txn set is empty)'

echo "check: ok"

#####################################################################
# test: 71d06779-dce9-4499-b54a-0af85b33ac44
# desc: zero sum posting
test_name=fs-zero-sum-posting
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ex/zero-posting.txn \
    2>&1 | grep 'semantic error - Zero sum'

echo "check: ok"

#####################################################################
#
# test: ba7e3b73-c1f7-4a82-832d-c75dc866149b
# desc: fs directory not found
test_name=fs-dir-not-found
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.fs.dir $SUITE_PATH/$module/txns-not-found \
    --input.fs.ext txn \
    2>&1 | grep 'Tackler error:.* IO .*/core/txns-not-found: No such file or directory'

echo "check: ok"
