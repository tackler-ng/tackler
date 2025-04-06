#!/bin/bash
# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2024-2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

TEST_DIR="$(readlink -f $(dirname $(realpath $0)))"
export TEST_DIR

TACKLER_ROOT=$(realpath "$TEST_DIR/../..")
export TACKLER_ROOT

target="unknown"
UPDATE_REF="false"

if [ "$1" == "--debug" ]; then
    target="debug"
elif [ "$1" == "--update-ref" ]; then
    target="debug"
    UPDATE_REF="true"
else
    target="release"
fi
export UPDATE_REF

TACKLER_SH=$TACKLER_ROOT/target/$target/tackler
export TACKLER_SH

SUITE_PATH=$TACKLER_ROOT/suite
export SUITE_PATH

OUTPUT_DIR=$(mktemp -d $TACKLER_ROOT/target/tmp-XXX)
export OUTPUT_DIR

source $TEST_DIR/lib/utils.sh

check_suite

$TEST_DIR/tep-1016.sh
$TEST_DIR/tep-1016-errors.sh
$TEST_DIR/accumulator.sh
$TEST_DIR/audit.sh
$TEST_DIR/audit-errors.sh
$TEST_DIR/cli.sh
$TEST_DIR/cli-errors.sh
$TEST_DIR/commodity.sh
$TEST_DIR/commodity-errors.sh
$TEST_DIR/core.sh
$TEST_DIR/core-fs-errors.sh
$TEST_DIR/core-git-errors.sh
$TEST_DIR/gas-blender.sh
$TEST_DIR/location.sh
$TEST_DIR/location-errors.sh
$TEST_DIR/parser.sh
$TEST_DIR/parser-errors.sh
$TEST_DIR/reporting-equity.sh
$TEST_DIR/reporting-group_by.sh
$TEST_DIR/reporting-ok.sh
$TEST_DIR/reporting-time_zones.sh
$TEST_DIR/reporting-errors.sh
$TEST_DIR/tags.sh
$TEST_DIR/tags-errors.sh
$TEST_DIR/new-and-init.sh
$TEST_DIR/price.sh
$TEST_DIR/price-errors.sh
$TEST_DIR/price-strict.sh
$TEST_DIR/price-multi.sh
$TEST_DIR/price-multi-miss.sh

echo
if [ "$UPDATE_REF" = "true" ]; then
    echo "THIS IS NOT A REAL TEST - THIS IS TEST VECTOR UPDATE!"
    echo
    echo "exit status is always 1"
    exit 1
else
    echo "target: $target - All good"
fi

rm -rf $OUTPUT_DIR
exit 0
