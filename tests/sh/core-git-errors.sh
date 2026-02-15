#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### CORE: GIT: ERRORS
###
module=core
mode="error"

#####################################################################
#
# test: 4569085c-04a8-4330-9c03-d07ea0935f22
# desc: repo not found
test_name=git-repo-not-found
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.repository $SUITE_PATH/$module/repo-not-found.git \
    --input.git.ref master \
    --input.git.dir git-txns \
    --input.git.ext txn \
    2>&1 | grep 'Tackler error: Txn Data: ".*/core/repo-not-found.git" .* not .* git repository'

echo "check: ok"

#####################################################################
#
# test: b0116db0-dd2b-40fb-976e-ee28118bc1e8
# desc: not git repository
test_name=git-not-git-repo
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.repository $SUITE_PATH/$module/ex/not-git-repo \
    --input.git.ref master \
    --input.git.dir git-txns \
    --input.git.ext txn \
    2>&1 | grep 'Tackler error: Txn Data: ".*/core/ex/not-git-repo" .* not .* git repository'

echo "check: ok"

#####################################################################
#
# test: 25452d77-aae5-414c-a6a6-bd60f090731e
# desc: parse error with shard data
test_name=git-parse-error
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref error-01 \
    2>&1 | grep 'parse error at line 2, column 3'

echo "check: ok"

#####################################################################
#
# test: c233295d-08b9-49b5-b384-634fc8432e64
# desc: commit not found (by ref)
test_name=git-unknown-commit-by-ref
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref ef88456ffae9eb546d115833f2ad66d48a8e268b \
    2>&1 | grep 'Tackler error: Txn Data: .* object .* ef8845.* could not be found'

echo "check: ok"

#####################################################################
#
# test: beed0077-33b6-441f-b7be-deca8080362d
# desc: commit not found (by commit)
test_name=git-unknown-commit-by-commit
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit ef88456ffae9eb546d115833f2ad66d48a8e268b \
    2>&1 | grep "Tackler error: Txn Data: Unknown commit id 'ef88456ffae9eb546d115833f2ad66d48a8e268b'"

echo "check: ok"

#####################################################################
#
# test: aeb11f77-ba35-400f-bdae-50d6ebb7e098
# desc: malformed sha1
test_name=git-malformed-sha1
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "//" \
    2>&1 | grep 'An error occurred while trying to find a reference'

echo "check: ok"

#####################################################################
#
# test: 7cb6af2e-3061-4867-96e3-ee175b87a114
# desc: can not resolve ref
test_name=git-ref-doesnt-resolve
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "^HEAD" \
    2>&1 | grep 'Tackler error: Txn Data: revspec ".*" did not .* single object'

echo "check: ok"

#####################################################################
#
# test: 4b507e08-b90e-4a6f-9c6b-4fef7c58d9fe
# desc: ref which is not found
test_name=git-ref-not-found
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "not-found-ref" \
    2>&1 | grep 'Tackler error: Txn Data: The ref .* "not-found-ref" .* not be found'

echo "check: ok"

#####################################################################
#
# test: 14b7e8eb-d168-48b2-86e7-8d922f306ad0
# desc: ref format is invalid
test_name=invalid-ref-format
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "^^^" \
    2>&1 | grep 'Tackler error: Txn Data: Tried to navigate .* anchor first'

echo "check: ok"

#####################################################################
#
# test: 5c0a2e2c-f82c-400a-a9bb-bc7b83010b3c
# desc: link inside repository, not supported
test_name=link-in-repository
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "linktxn-01" \
    2>&1 | grep 'Links .* not supported'

echo "check: ok"

#####################################################################
#
# test: a32df7bb-b499-4f7b-98c8-5103ceac82fb
# desc: empty transaction set
test_name=git-empty-txn-set
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref "empty" \
    2>&1 | grep 'Tackler error: Txn Data: .* (txn set is empty)'

echo "check: ok"
