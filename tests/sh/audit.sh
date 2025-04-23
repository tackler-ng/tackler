#
# Tackler-NG 2024-2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### AUDIT
###
module=audit

# test: 93651962-6b61-4fd6-941a-339abd87ec73
rm -f $OUTPUT_DIR/*
test_name=audit-1E1-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/acc-selectors.toml \
    --input.git.ref txns-1E1

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: f8c0fe2b-f189-4338-b75e-3c8e68a8c7e2
rm -f $OUTPUT_DIR/*
test_name=audit-1E1-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/fs-audit.toml \

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 5c34d752-8d17-40df-be91-5dc1b107478e
rm -f $OUTPUT_DIR/*
test_name=audit-1E1-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --accounts "e" "a"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
echo ": ok"


#
# audit-1E2-01
#
# test: 4e8e1d79-bbb5-4e6f-9072-d7e3c5b8c7ea
#
# test: cdd2d0a0-3e6d-42e3-9cae-b4797a23fe66
# desc: Flat Balance compatibility guardian
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR/ \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns/2016/01/11" \
    --input.git.ref "txns-1E2"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 2250f5f5-7eb4-456b-a693-3ea63c219584
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns/2016/01/11" \
    --input.git.ref "txns-1E2" \
    --accounts '.*'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
echo ": ok"


#####################################################################
#
# test: b2ea4102-40a2-46e5-aca3-398cf4849058
# plain filter definition
# test: 515ba0be-b571-4a7f-a2a3-28dc1e545228
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 41a9479b-1907-44bb-88bc-48c3cbe8c00f
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-04
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts '.*' \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 7a887956-a350-4663-9638-715bfa3c9040
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-05
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts 'none-matching' \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 224709cb-c96c-47f5-83e1-6e94c333e5c6
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-06-step1
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts '.*' \
    --reports balance \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
# the equity export will be tested in step-2 by identity export
echo ": ok"

test_name=audit-1E2-06-step2
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/fs-non-audit.toml \
    --input.file $OUTPUT_DIR/audit-1E2-06-step1.equity.txn \
    --accounts '.*'

echo -n "check:"
cmp_result $module audit-1E2-06-step2 txn identity
echo ": ok"

#####################################################################
#
# test: 20ce2b43-e433-4edb-894a-48e955cdcd01
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-07
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit-sha3_512.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts '.*' \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"


#####################################################################
#
# test: 17d027aa-28d0-4887-b2dd-f342dccb61d1
# test-ref: 85d16b5a-cde9-40d3-9a37-3b7ba7ee7049
# base64 filter definition
# test-ref: 8bbe1d2a-8548-49cf-9d8b-942242b882bd
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-08
echo "test: $module/$test_name: "

# base64 armored filter definition
filter_def=$(make_filter_with_time_span "account_flt_body" '^e:.*' "2016-01-01T00:00:00Z[UTC]" "2016-02-01T00:00:00Z[UTC]")
#echo "filter: $fltdef"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2" \
    --accounts "e:.*" \
    --api-filter-def "$filter_def"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
cmp_result $module $test_name txn equity
echo ": ok"

#####################################################################
#
# test: 82fe2414-9e20-45da-8f7f-6f21ae8c71f2
# test-ref: 3ef0b17b-3e0f-4033-962b-0ca1de8d2ca4
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-09
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --reports balance \
    --accounts "^a:.*" \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "txns-1E2^^"

echo -n "check:"
cmp_result $module $test_name txt bal
echo ": ok"

#####################################################################
#
# test: 248707bc-7c58-4bab-a4a6-4cc1471fd936
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-10
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --reports balance \
    --accounts "^a:.*" \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "2da8c6a30fed6e65c06070d6c12e7dcaff84b599"

echo -n "check:"
cmp_result $module $test_name txt bal
echo ": ok"

#####################################################################
#
# audit-1E2-11, use abbreviated commit id, this is same
#               (with the same reference) as audit-1E2-10
#
# test: 0ae6dfb6-0975-49bc-8744-f7a4143a6ead
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-10
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --reports balance \
    --accounts "^a:.*" \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ref "2da8c6a3"

echo -n "check:"
cmp_result $module $test_name txt bal
echo ": ok"

