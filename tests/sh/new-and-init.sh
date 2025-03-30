#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### CMD: NEW and INIT
###
module=new-and-init

#
# new-01
#
# test: a405a6f4-be40-429d-a027-1ec80734dec2
rm -rf $OUTPUT_DIR/*
test_name=new-01
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name

$TACKLER_SH \
    new $journal_path

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml
echo ": ok"

#
# new-02
#
# test: 1586e27f-3bc2-44b9-b4b4-9a23f747c837
# desc: new with strict and equity
rm -rf $OUTPUT_DIR/*
test_name=new-02
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name

$TACKLER_SH \
    new $journal_path

echo "check:"
$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --config $journal_path/conf/tackler.toml \
    --exports equity \
    --strict.mode true
echo ": ok"


#
# new-03
#
# test: eddeb65b-630b-4d8b-a236-f9caa298a08b
# desc: new with price
rm -rf $OUTPUT_DIR/*
test_name=new-03
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name

$TACKLER_SH \
    new $journal_path

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml \
    --reports balance \
    --price.lookup-type last-price
echo ": ok"

#
# new-04
#
# test: ffe35948-b42d-4b43-a71a-483d1da048bc
# desc: new with price, strict
rm -rf $OUTPUT_DIR/*
test_name=new-04
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name

$TACKLER_SH \
    new $journal_path

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml \
    --reports balance \
    --price.lookup-type last-price \
    --strict.mode true
echo ": ok"


#
# init-01
#
# test: 720220ba-ba95-44f4-86d0-9390afa462de
rm -rf $OUTPUT_DIR/*
test_name=init-01
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name
(
	mkdir -p $journal_path
	cd $journal_path
	$TACKLER_SH init
)

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml
echo ": ok"

rm -rf $OUTPUT_DIR/*
