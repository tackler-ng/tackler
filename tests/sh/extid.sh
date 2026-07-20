#
# Tackler-NG 2026
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### EXT-ID: UNIQUE = FALSE
###
module=extid

mode="kernel.ext-id missing"

#####################################################################
#
# ext-id: 00
#
# test: 4044420a-6df9-42c8-809a-ed3221b5e640
# desc: duplicates with missing kernel.ext-id conf
rm -f $OUTPUT_DIR/*
test_name=extid-00
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/extid-plain.toml \
    --input.fs.dir $SUITE_PATH/$module/ok/txns-dups \
    --reports ""

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

mode="unique = false"

#####################################################################
#
# ext-id: 01
#
# test: 8d5321fd-92af-4a4d-934f-9c35c41f7b00
# desc: no dups, unique false
rm -f $OUTPUT_DIR/*
test_name=extid-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/extid-uniq-false.toml \
    --input.fs.dir $SUITE_PATH/$module/ok/txns

echo -n "check:"
cmp_result_ref $module extid $test_name txt reg
cmp_result_ref $module extid $test_name json reg

cmp_result_ref $module extid $test_name txn identity
echo ": ok"

mode="unique = true"

#####################################################################
#
# ext-id: 02
#
# test: 4796a487-d903-4314-9fd5-73c9ebf9a86f
# desc: no dups, unique true
rm -f $OUTPUT_DIR/*
test_name=extid-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/extid-uniq-true.toml \
    --input.fs.dir $SUITE_PATH/$module/ok/txns

echo -n "check:"
cmp_result_ref $module extid $test_name txt reg
cmp_result_ref $module extid $test_name json reg

cmp_result_ref $module extid $test_name txn identity
echo ": ok"

#####################################################################
#
# identity-01
#
# test: 0d0bb5f1-4d20-48ff-af64-da80ef9ac039
rm -f $OUTPUT_DIR/*
test_name=identity-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/extid-uniq-true.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \
    --input.fs.dir $SUITE_PATH/$module/ok/txns \
    --reports ""

$TACKLER_SH \
    --config $SUITE_PATH/$module/extid-uniq-true.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn \

echo -n "check:"
cmp_result_ref $module extid $test_name txt reg
cmp_result_ref $module extid $test_name json reg

cmp_result_ref $module extid $test_name txn identity
echo ": ok"
