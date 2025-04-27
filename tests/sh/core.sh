#
# Tackler-NG 2024-2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh


###
### CORE
###
module=core

#####################################################################
#
# master
#
# test: 292f250d-7cb2-4114-92e1-10f9a8d5b381
rm -f $OUTPUT_DIR/*
test_name=git-master-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# git-shard-01
#
# test: 1d2c22c1-e3fa-4cd4-a526-45318c15d13e
# test-ref: 9f0d3896-ce0b-415b-8d96-ad40e5177b45
rm -f $OUTPUT_DIR/*
test_name=git-shard-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --exports identity equity \
    --reports balance balance-group register \
    --input.git.ref shard-01

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# git-tag-01
#
# test: 80d3c14d-64ee-46d3-b9fd-37f82384a562
rm -f $OUTPUT_DIR/*
test_name=git-tag-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --exports identity equity \
    --reports balance balance-group register \
    --input.git.ref tag-test-01

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# git-and-filter-01
#
# test: a6031106-1bf7-47cf-8837-0c77566db79d
rm -f $OUTPUT_DIR/*
test_name=git-and-filter-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit  3ebf14112c0ba2b4263a4b7388c589b10db27b1e \
    --exports identity equity \
    --reports balance balance-group register \
    --api-filter-def '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "shard01" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# git-utf8-01
#
# test: c2f39ef7-c085-4ff4-af4d-85a50d0ee203
rm -f $OUTPUT_DIR/*
test_name=git-utf8-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref utf8-01

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"



#####################################################################
#
# git-commit-01
#
# test: ede5d6b5-1885-4e02-8f9d-e2e1034fb6e3
rm -f $OUTPUT_DIR/*
test_name=git-commit-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit 4b0ea73bef9bd996102eed35712e80fe1c81207e

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# git-commit-02
#
# test: 7dfebf19-480c-4bf5-806a-4d560a20a1d4
rm -f $OUTPUT_DIR/*
test_name=git-commit-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit 3a1622d0c81cd1520f42ee357315d1db8c0e0771

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# git-commit-03
#
# test: f44faf05-7019-4c34-b0af-3345feb4ad37
rm -f $OUTPUT_DIR/*
test_name=git-commit-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit 3ebf14112c0ba2b4263a4b7388c589b10db27b1e

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"


#####################################################################
#
# git-commit-04
#
# test: 7752eee6-d3cf-4084-93c0-cb43a093fdd0
rm -f $OUTPUT_DIR/*
test_name=git-commit-04
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.commit 3a1622d0 \
    --exports identity equity \
    --reports balance balance-group register

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"


#####################################################################
#
# git-dirtxn-01
#
# test: eb74f8a7-8f70-413f-a431-15b0a5b09aad
rm -f $OUTPUT_DIR/*
test_name=git-dirtxn-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-ok.toml \
    --input.git.ref dirtxn-01

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# git-dirtxn-02
#
# test: f865d983-2f48-40f8-9010-d326a83e959e
rm -f $OUTPUT_DIR/*
test_name=git-dirtxn-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-dir.toml

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# git-dirtxn-03
#
# note-ng: input.git.dir alone is not supported
# test: f25e0542-ef74-4131-99eb-e76a5b04b263
rm -f $OUTPUT_DIR/*
test_name=git-dirtxn-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/git-dir.toml \
    --input.git.repository $SUITE_PATH/$module/test-data.git \
    --input.git.dir git-txns/2019 \
    --input.git.ref dirtxn-02

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# identity-01
#
# test: 1d731709-1e85-4fe6-b209-aad46cca5551
rm -f $OUTPUT_DIR/*
test_name=id2id-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/id2id.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \

$TACKLER_SH \
    --config $SUITE_PATH/$module/id2id.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn \

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# shard-glob-01
#
# test: b6dee458-ac8a-461b-a3a3-e598f85d869c
rm -f $OUTPUT_DIR/*
test_name=shard-glob-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.fs.dir $SUITE_PATH/$module/ok/txns-as-txt \
    --input.fs.ext 'txt'

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"


#####################################################################
#
# shard-link-01
#
# test: 4c62d3a6-da8e-441a-a459-6aca5edceb6a
rm -f $OUTPUT_DIR/*
test_name=shard-link-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.fs.dir $SUITE_PATH/$module/ok/txns-by-link \
    --input.fs.ext 'txn'

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"


#####################################################################
#
# strg-selector-01
#
# test: d844f381-94dd-42ad-a3a5-dbf08974f7fa
rm -f $OUTPUT_DIR/*
test_name=strg-selector-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/git-ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.storage fs

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# strg-selector-02
#
# test: af6d111d-5b01-40ad-8da2-98f26dacc4f3
rm -f $OUTPUT_DIR/*
test_name=strg-selector-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/id2id.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.storage git

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# time-conf-01
#
# test: 51fad435-682f-42fc-bb66-7d7d5d710042
rm -f $OUTPUT_DIR/*
test_name=time-conf-01
echo "test: $module/$test_name: "


$TACKLER_SH \
    --config $SUITE_PATH/$module/zoneid.toml \
    --input.file $SUITE_PATH/${module}/ok/time-plain.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# time-conf-02
#
# test: 2c4566b8-0424-4561-9265-087215c2d243
rm -f $OUTPUT_DIR/*
test_name=time-conf-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/zoneid-helsinki.toml \
    --input.file $SUITE_PATH/${module}/ok/time-plain.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# time-dst-01
#
# test: 203c42c0-6660-4c0a-b991-e55c86e06da3
rm -f $OUTPUT_DIR/*
test_name=time-dst-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/zoneid.toml \
    --input.file $SUITE_PATH/${module}/ok/time-dst.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# time-dst-02
#
# test: 23ee5075-ca27-4f08-876e-e29de77cfc59
rm -f $OUTPUT_DIR/*
test_name=time-dst-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/zoneid-helsinki.toml \
    --input.file $SUITE_PATH/${module}/ok/time-dst.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# time-nano-01
#
# test: de406118-eb73-4621-9b9b-6e26d512cf05
rm -f $OUTPUT_DIR/*
test_name=time-nano-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/${module}/ok/time-nano.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --reports register

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name json reg
cmp_result $module $test_name txn identity
echo ": ok"
