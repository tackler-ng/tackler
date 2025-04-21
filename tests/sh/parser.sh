#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh


###
### PARSER
###
module=parser

#####################################################################
#
# acc-names-01
#
# test: 00d410e8-f627-4bb1-a403-0dbe1d64a73a
rm -f $OUTPUT_DIR/*
test_name=acc-names-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/acc-names.toml \
    --input.file $SUITE_PATH/$module/ok/acc-names-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# acc-names-02
#
# test: bf61d3a8-8a77-4b8a-917e-1d58acfde25a
rm -f $OUTPUT_DIR/*
test_name=acc-names-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/acc-names.toml \
    --input.file $SUITE_PATH/$module/ok/acc-names-02.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# code-01
#
# test: 8ab8a08b-2348-4c20-8971-b357b2a8b6e1
rm -f $OUTPUT_DIR/*
test_name=code-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/code.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} json reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# basic
#
# test: ba71f292-cc3e-406f-991e-8dd82a7cf87c
rm -f $OUTPUT_DIR/*
test_name=basic
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/basic.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# par-02
#
# test: 2564946b-3afb-4edc-9ee5-3c0f37baa1a1
rm -f $OUTPUT_DIR/*
test_name=par-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/par-02.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} json reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# par-03
#
# test: 39902850-63aa-4c88-a9c3-4065da8b181e
rm -f $OUTPUT_DIR/*
test_name=par-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/par-03.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# id-01
#
# test: e997a1a0-4f96-4212-a565-31565c87a7a2
rm -f $OUTPUT_DIR/*
test_name=id-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/id-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# shard-glob
#
# test: 31a128f5-d1a2-4d93-beee-6599344ece57
rm -f $OUTPUT_DIR/*
test_name=shard-glob
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.fs.dir $SUITE_PATH/$module/ok/txns \
    --input.fs.ext txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
echo ": ok"

#####################################################################
#
# txn-comments-01.txn
#
# test: 5f80093d-6e90-4108-a18c-d76c91cacb55
rm -f $OUTPUT_DIR/*
test_name=txn-comments-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/txn-comments-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} json reg
echo ": ok"

#####################################################################
#
# post-comments-01
#
# test: 0f06fe78-83ca-46a9-a5cd-b873ab6be807
rm -f $OUTPUT_DIR/*
test_name=post-comments-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/post-comments-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --reports balance balance-group register \
    --group-by "month" \


echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} json reg
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# id-chars-01
#
# test: bc8f6399-fd7a-420a-8deb-9cc0ae97c9f6
rm -f $OUTPUT_DIR/*
test_name=id-chars-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/id-chars.toml \
    --input.file $SUITE_PATH/$module/ok/id-chars.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# no-coa
#
# test: 8a046e2a-2bed-4ac3-aa42-1dbc31fd78fd
rm -f $OUTPUT_DIR/*
test_name=no-cao
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/no-cao.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txn identity
echo ": ok"

