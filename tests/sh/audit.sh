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

# test: f0782d7f-1626-45ef-bbdc-86bf833eb105
# desc: Audit metadata with console
# test: 93651962-6b61-4fd6-941a-339abd87ec73
rm -f $OUTPUT_DIR/*
test_name=audit-1E1-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/audit/acc-selectors.toml \
    --input.git.ref set-1e1 \
     > "${OUTPUT_DIR}/${test_name}.stdout.txt" \
     2> "${OUTPUT_DIR}/${test_name}.stderr.txt"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/acc-selectors.toml \
    --input.git.ref set-1e1

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg

cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity

cmp_result $module $test_name txt stdout
cmp_result $module $test_name txt stderr
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
# test: 1151b193-8af2-4d1d-87b7-0002f1d20df5
# desc: fs: full overlay
rm -f $OUTPUT_DIR/*
test_name=cfg-fs-full-overlay
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-fs-invalid.toml \
    --input.fs.path txns \
    --input.fs.dir 1E1/2016/03/14 \
    --input.fs.ext txn

echo -n "check:"
cmp_result $module $test_name txt bal
echo ": ok"

#####################################################################
#
# test: 1a646767-9591-4ab1-8c14-fb11bff7d2b6
# desc: fs: full overlay, missing fs
rm -f $OUTPUT_DIR/*
test_name=cfg-fs-missing
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-fs-nothing.toml \
    --input.fs.path txns \
    --input.fs.dir 1E1/2016/03/14 \
    --input.fs.ext txn

echo -n "check:"
cmp_result_ref $module cfg-fs-full-overlay $test_name txt bal
echo ": ok"


#####################################################################
#
# test: e301910e-f8f9-43ac-9010-d74558efe829
# desc: fs: partial overlay, --input.fs.dir
rm -f $OUTPUT_DIR/*
test_name=cfg-fs-partial-dir
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-fs-git-active.toml \
    --input.storage fs \
    --input.fs.dir 1E1/2016/03/14

echo -n "check:"
cmp_result_ref $module cfg-fs-full-overlay $test_name txt bal
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
cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg
echo ": ok"


#
# audit-1E2-01
#
# test: 4e8e1d79-bbb5-4e6f-9072-d7e3c5b8c7ea
#
# test: cdd2d0a0-3e6d-42e3-9cae-b4797a23fe66
# desc: Flat Balance compatibility guardian
# test: 649a72b6-e6cf-467c-9f7c-e3f49bb0d98c
# desc: git: full overlay (especially git dir)
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR/ \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns/2016/01/11" \
    --input.git.ext "txn" \
    --input.git.ref "set-1e2"

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
# desc: lorem ipsum
# test: b46c8b92-714a-4ccb-b93d-8fc9f91c8c02
# desc: git: partial overlay (git dir + git ref)
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.dir "txns/2016/01/11" \
    --input.git.ref "set-1e2" \
    --accounts '.*'

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
# test: 69502e1d-0c51-44bf-89e7-5f559c65f147
# desc: audit and filter metadata with console
# test: b2ea4102-40a2-46e5-aca3-398cf4849058
# desc: plain filter definition
# test: 515ba0be-b571-4a7f-a2a3-28dc1e545228
# desc: lorem ipsum
# test: 53fd2388-c7f0-430c-8f8b-c9cf6e3334f5
# desc: git: partial overlay (git.ref); full overlay, with no git cfg
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/audit/audit.toml \
    --input.git.ref "set-1e2" \
    --api-filter-def \
        '{ "txnFilter": { "TxnFilterTxnDescription": { "regex": "^1E2 txn-(1|17|100)$" }}}' \
     > "${OUTPUT_DIR}/${test_name}.stdout.txt" \
     2> "${OUTPUT_DIR}/${test_name}.stderr.txt"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-git-nothing.toml \
    --input.git.repository $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ext "txn" \
    --input.git.ref "set-1e2" \
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

cmp_result $module $test_name txt stdout
cmp_result $module $test_name txt stderr
echo ": ok"

#####################################################################
#
# test: 41a9479b-1907-44bb-88bc-48c3cbe8c00f
# desc: --input.git.repo alias
# test: 66197e67-35b5-425f-bad4-dd510d48dbee
# desc: git: full overlay and invalid base configuration (git)
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-04
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-git-invalid.toml \
    --input.git.repo $SUITE_PATH/audit/audit-repo.git \
    --input.git.dir "txns" \
    --input.git.ext "txn" \
    --input.git.ref "set-1e2" \
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
# desc: lorem ipsum
# test: 1292763d-9828-4998-ba2b-84fe2968cdf7
# desc: git: partial overlay, cfg:storage = fs, and  --input.storage = git
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-05
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-git-fs-active.toml \
    --input.storage git \
    --input.git.ref "set-1e2" \
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
# desc: lorem ipsum
# test: 12c49a4f-b403-4f19-b167-cae6e38d4d0a
# desc: file: overlay --input.file
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-06-step1
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/audit.toml \
    --input.git.ref "set-1e2" \
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
    --input.git.ref "set-1e2" \
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
    --input.git.ref "set-1e2" \
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
    --input.git.ref "set-1e2^^"

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
    --input.git.ref "3e8aa1c9cfc11a8a24afa33da76b7973b93fd455"

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
    --input.git.ref "3e8aa1c9"

echo -n "check:"
cmp_result $module $test_name txt bal
echo ": ok"

#####################################################################
#
# test: 07a5ec54-d278-4769-a916-1ddfe220b67e
# desc: check that input.git.dir doesn't act as wildcard
rm -f $OUTPUT_DIR/*
test_name=audit-1E2-11
echo "test: $module/$test_name: "

set +e
$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/audit/cfg-fs-git-active.toml \
    --input.git.ref "set-1e2" \
    --input.git.dir "txns/2016/1" \
    2>&1 | grep 'txn set is empty'
set -e

echo -n "check:"
echo "special test - error is fine"
echo ": ok"
