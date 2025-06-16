#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### CLI: ERRORS
###
module=cli
mode="error"

#####################################################################
#
# test: 47485d81-ef64-4a6a-9cd6-5f1e15ac130b
# desc: unknown option
test_name=unknown-option
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --extra \
    2>&1 | grep 'unexpected argument'

echo "check: ok"

#####################################################################
#
# test: 7a20d61f-c1ab-4aaa-993c-73cc39978c6f
# desc: extra args
test_name=extra-argument
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    extra \
    2>&1 | grep 'the subcommand .* cannot'

echo "check: ok"

#####################################################################
#
# test: e00bd60a-dc39-4d36-ac1e-243efc8489b2
# desc: missing config with command
test_name=missing-config
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    report \
    2>&1 | grep 'config file is not provided'

echo "check: ok"

#####################################################################
#
# test: a11c536e-ce20-4e2b-8abf-cd8a47cdaf07
# desc: unknown account
test_name=unknown-account
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-acc-01.txn \
    --strict.mode true \
    2>&1 | grep "Semantic error: Unknown account: 'a'"

echo "check: ok"

#####################################################################
#
# test: 178a3269-76e9-4bb4-abcd-62806817573e
# desc: Detect unbalanced transaction
test_name=unbalanced-txn
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ex/unbalanced.txn \
    2>&1 | grep "Semantic error: .* zero: 2"

echo "check: ok"

#####################################################################
#
# test: c096f7a8-a87c-4b3c-ad37-b860775597a0
# desc: empty config
test_name=empty-config
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ex/empty.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    2>&1 | grep "Tackler error: Configuration .*/cli/ex/empty.toml': TOML parse error at line 1, column 1"

echo "check: ok"

#####################################################################
#
# test: f04577b6-49b2-460f-92f6-7886a4b49152
# desc: no output path
test_name=non-existing-output
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR/no/such/path \
    --output.prefix $test_name \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    --reports balance \
    2>&1 | grep "Tackler error: No .* file or directory .*: '.*/no/such/path/non-existing-output.bal.txt'"

echo "check: ok"

#####################################################################
#
# test: 39fed28a-1157-4cda-98de-cfe8fa1ce5b5
# desc: filter all transactions out of stream. Result should be an empty txns
test_name=filtered-empty-txn-set
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    --api-filter-def \
      '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "this-wont-be-matched" }}}' \
    2>&1 | grep 'Tackler error: Txn Data: no transactions (txn set is empty)'

echo "check: ok"


#####################################################################
# test: 5a80bd1d-118c-4f3c-b89b-142059bcb3be
# desc: invalid json at parse level
test_name=invalid-filter-json-level
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    --api-filter-def \
      '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "this-wont-be-matched" }}' \
    2>&1 | grep 'Tackler error: Txn Filter definition is not valid JSON: .* parsing .*'

echo "check: ok"

#####################################################################
#
# test: d2e7dd30-8ead-43b2-8986-4cb416167a28
# desc: invalid json, can not be deserialized correctly
test_name=invalid-filter-json-deseriliaze
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    --api-filter-def \
      '{ "txnFilter": { "TxnFilterWhichWontBeThere": { "regex": "a.*" }}}' \
    2>&1 | grep 'Tackler error: Txn Filter definition is not valid JSON: unknown variant'

echo "check: ok"

#####################################################################
#
# test: 66bc6ec3-991f-41b6-a347-b3f492bc6f65
# desc: invalid filter regex
test_name=invalid-filter-regex
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ok/filters.txn \
    --api-filter-def \
      '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "*" } } }' \
    2>&1 | grep 'Tackler error: Txn Filter definition is not valid JSON: regex parse error'

echo "check: ok"


#####################################################################
#
# test: 9c9b341f-2ffe-4cad-8c24-174334e39a17
# desc: unknown storage type from command line
test_name=unknown-storage-type
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage foo \
    2>&1 | grep 'invalid value'

echo "check: ok"


#####################################################################
#
# test: f2284891-82da-4efd-a705-9e8f612c88af
# desc: invalid input arg combinations
test_name=invalid-arg-combinations
echo "test: $module/$test_name: $mode"

###
### input.storage
###

### input.storage = git
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage git \
    --input.fs.path path \
    --input.fs.dir dir \
    --input.fs.ext ext\
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage git --input.fs.dir path \
    2>&1 | grep 'cannot be used with'
# test: a2ca374a-1323-413b-aaff-64bc3c8d4d30
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage git \
    --input.git.ref main \
    --input.git.commit abcdef \
    2>&1 | grep 'cannot be used with'

### input.storage = fs
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage fs \
    --input.git.repository path \
    --input.git.dir dir \
    --input.git.ext ext\
    --input.git.ref ref \
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.storage fs --input.git.dir path \
    2>&1 | grep 'cannot be used with'

###
### input.file
###
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.storage fs \
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.storage git \
    2>&1 | grep 'cannot be used with'

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.fs.path path \
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.fs.dir dir \
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.fs.ext txn \
    2>&1 | grep 'cannot be used with'

$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.git.repository path \
    2>&1 | grep 'cannot be used with'
# test: 1822f1b2-f749-4f63-be44-fa29c58c4fe2
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.git.ref main \
    2>&1 | grep 'cannot be used with'
# test: 97bf542e-55b5-437f-9878-7f436f50c428
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.git.commit abcdef \
    2>&1 | grep 'cannot be used with'
# test: 8afb22ac-8a52-4cba-9443-e6375e6fcf75
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.git.dir dir \
    2>&1 | grep 'cannot be used with'
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.file f.txn --input.git.ext txn \
    2>&1 | grep 'cannot be used with'

### cli dir vs. cli git
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.dir path --input.git.repository path \
    2>&1 | grep 'cannot be used with'
# test: 3eba26fe-821d-4d36-94cb-09427b1c004f
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.dir path --input.git.ref main \
    2>&1 | grep 'cannot be used with'
# test: 400bd1e9-6f7a-4e0c-913c-45401ee73181
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.dir path --input.git.commit abcdef \
    2>&1 | grep 'cannot be used with'
# test: f74a2252-d826-4176-945a-8895d4c7f1f7
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.dir path --input.git.dir path \
    2>&1 | grep 'cannot be used with'

### cli fs.ext vs. git
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.ext txn --input.git.repository path \
    2>&1 | grep 'cannot be used with'
# test: 7d4984c7-633f-4403-a2b7-5ea0cd4f07e8
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.ext txn --input.git.ref main \
    2>&1 | grep 'cannot be used with'
# test: 6ec6431e-a443-4633-8f26-df3218a8657c
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.ext txn --input.git.commit abcdef \
    2>&1 | grep 'cannot be used with'
# test: f150df09-dd9b-4240-9191-df1029c698e9
$TACKLER_SH \
    --config $SUITE_PATH/basic.toml \
    --input.fs.ext txn --input.git.dir path \
    2>&1 | grep 'cannot be used with'

echo "check: ok"
