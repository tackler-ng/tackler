#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### TEP-1016: ERRORS
###
module=audit
mode="error"

#####################################################################
#
# test: 9e41344d-6561-4641-bbb8-faf636ed9e7d
# desc: conf: balance, unknown type
test_name=bal-type-bal-unknown-type
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/tep1016-01-err.toml \
    --input.git.repository $SUITE_PATH/$module/audit-repo.git \
    --input.git.ref errs-1E2 \
    --input.git.dir txns/2016/04 \
    2>&1 | grep 'Tackler error: Configuration .*/tep1016-01-err.toml.*Unknown .* flatter'

echo "check: ok"

#####################################################################
#
# test: a597e639-2f2c-42ae-b453-9cafedc7150c
# desc: conf: balance-group, unknown type
test_name=bal-type-balgrp-unknown-type
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/tep1016-02-err.toml \
    --input.git.repository $SUITE_PATH/$module/audit-repo.git \
    --input.git.ref errs-1E2 \
    --input.git.dir txns/2016/04 \
    2>&1 | grep 'Tackler error: Configuration .*/tep1016-02-err.toml.*Unknown .* bruce'

echo "check: ok"

