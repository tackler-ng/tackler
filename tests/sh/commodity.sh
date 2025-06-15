#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### COMMODITY
###
module=commodity

commodity_test () {
    local test_name=$1

    echo "test: $module/$test_name: "

    rm -f $OUTPUT_DIR/*
    $TACKLER_SH \
        --config $SUITE_PATH/$module/ok.toml \
        --output.dir $OUTPUT_DIR \
        --output.prefix $test_name \
        --input.file $SUITE_PATH/$module/ok/$test_name.txn

    echo -n "check:"
    cmp_result $module $test_name txt bal
    cmp_result $module $test_name txt balgrp
    cmp_result $module $test_name txt reg
    cmp_result $module $test_name txn equity
    cmp_result $module $test_name txn identity
}

# test: a5318e06-309e-4c52-996b-b07aefac4c46
commodity_test basic-01
echo ": ok"

# test: 04af8049-355d-4c6c-ac63-c153b0141c48
commodity_test basic-02
echo ": ok"

# test: 8a7a8c60-aab9-4700-b081-9804c1ec537f
commodity_test basic-03
cmp_result $module basic-03 json bal
cmp_result $module basic-03 json balgrp
cmp_result $module basic-03 json reg
echo ": ok"

# test: c5493e99-d0f3-49b3-8450-3ae6ef970b10
commodity_test basic-04
cmp_result $module basic-04 json bal
cmp_result $module basic-04 json balgrp
cmp_result $module basic-04 json reg
echo ": ok"

# test: aeffbcc2-bcc4-40f3-9b37-03dbca3bff88
commodity_test basic-05
echo ": ok"

# test: 0dae7dfd-c816-4a77-afff-a372e1802026
commodity_test basic-06
cmp_result $module basic-06 json bal
cmp_result $module basic-06 json balgrp
cmp_result $module basic-06 json reg
echo ": ok"

#####################################################################
#
# basic-03-01
#
# test: 7616a6e8-8492-4fe7-9921-ae00f86af2cd
rm -f $OUTPUT_DIR/*
test_name=basic-03-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/basic-03.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name json bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name json balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# basic-03-02
#
# test: c7d115a7-664c-479d-8bf4-c1a85a06d5ba
rm -f $OUTPUT_DIR/*
test_name=basic-03-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/basic-03.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.equity.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# valpos
#

# test: 54cb6baa-6c3d-4196-94bc-b6fd37ed0b57
commodity_test valpos-01
echo ": ok"

# test: 2501f96c-46d8-4711-9e7e-dc70e4af329d
commodity_test valpos-02
echo ": ok"

# test: d0dd6b75-238e-49cf-a048-a77baafba9bd
commodity_test valpos-03
echo ": ok"

#####################################################################
#
# valpos-01-01
#
# test: 773106ca-d6e5-4514-b19b-3e4d34283152
rm -f $OUTPUT_DIR/*
test_name=valpos-01-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/valpos-01.txn

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# valpos-02-01
#
# test: d37de2c1-61bf-4ffc-8f7a-933ce8380b2f
rm -f $OUTPUT_DIR/*
test_name=valpos-02-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/valpos-02.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"



commodity_filter_test () {
    local test_name=$1

    echo "test: $module/filter-$test_name: "

    rm -f $OUTPUT_DIR/*
    $TACKLER_SH \
        --config $SUITE_PATH/$module/ok.toml \
        --output.dir $OUTPUT_DIR \
        --output.prefix filter-$test_name \
        --input.file "$SUITE_PATH/$module/ok/basic-$test_name".txn \
        --accounts '^a.*'

    echo -n "check:"
    cmp_result $module filter-$test_name txt bal
    cmp_result $module filter-$test_name txt balgrp
    cmp_result $module filter-$test_name txt reg
    cmp_result $module filter-$test_name txn equity
    cmp_result $module filter-$test_name txn identity
    echo ": ok"
}

# test: 1df0fb31-6604-4679-9ec4-7a1d65e49574
commodity_filter_test "01"
echo ": ok"

# test: 633da495-ef30-4484-8b14-fbd1fdcc9684
commodity_filter_test "02"
echo ": ok"

# test: 09e9a673-4730-4a03-b73a-5959730450f6
commodity_filter_test "03"
echo ": ok"

#####################################################################
#
# unknown-01
#
# test: 779fdbd7-93f8-48da-9860-8ce7a5c66e6e
rm -f $OUTPUT_DIR/*
test_name=unknown-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/no-commodities.toml \
    --input.file $SUITE_PATH/$module/ok/unknown-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --strict.mode false

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# test: 604ca9e5-94d4-40b6-89cb-afac2460fd12
# desc: inverted
rm -f $OUTPUT_DIR/*
test_name=inverted-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/no-commodities.toml \
    --input.file $SUITE_PATH/$module/ok/basic-03.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --strict.mode false \
    --formats txt json \
    --accounts "a" \
    --invert

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg

cmp_result $module $test_name json bal
cmp_result $module $test_name json balgrp
cmp_result $module $test_name json reg

cmp_result $module $test_name txn equity
echo ": ok"

