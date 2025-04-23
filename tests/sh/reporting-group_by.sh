#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh


###
### REPORTING / GROUP-BY
###
module=reporting/group-by

#####################################################################
#
# date
#
# test: ed89d638-70fa-474c-b88f-be03df6231df
rm -f $OUTPUT_DIR/*
test_name=date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/plain \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/date.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
echo ": ok"

#####################################################################
#
# iso-week-date
#
# test: 66692492-2b9f-46c2-8e09-a49573ba0cec
rm -f $OUTPUT_DIR/*
test_name=iso-week-date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/plain \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week-date.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# iso-week
#
# test: 053ca6b0-6be6-4fc1-9dd8-6abc754d98d1
rm -f $OUTPUT_DIR/*
test_name=iso-week
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/plain \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# month
#
# test: a52914e4-0887-4255-abea-c0b457802f56
rm -f $OUTPUT_DIR/*
test_name=month
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/plain \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/month.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# year
#
# test: 9fff9c03-c94b-4d67-9180-ac7d22edf0c7
rm -f $OUTPUT_DIR/*
test_name=year
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/plain \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/year.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# zoned-date
#
# test: b5fd85fe-94b1-4ef8-8ab7-e8b22acb4d63
rm -f $OUTPUT_DIR/*
test_name=zoned-date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/zoned \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/date.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# zoned-iso-week-date
#
# test: c0febfbd-218e-41dd-8c65-a1851b21434e
rm -f $OUTPUT_DIR/*
test_name=zoned-iso-week-date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/zoned \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week-date.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# zoned-iso-week
#
# test: 8ffe3683-3934-4358-840d-fb00fe8fd788
rm -f $OUTPUT_DIR/*
test_name=zoned-iso-week
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/zoned \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# zoned-month
#
# test: 94e0bddc-4d1b-4133-992d-e314134d0fcc
rm -f $OUTPUT_DIR/*
test_name=zoned-month
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/zoned \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/month.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

#####################################################################
#
# zoned-year
#
# test: 29f80e87-070a-4707-a5f7-770a7731c9ec
rm -f $OUTPUT_DIR/*
test_name=zoned-year
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.fs.dir $SUITE_PATH/$module/txns/zoned \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/year.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name json balgrp "."
echo ": ok"

