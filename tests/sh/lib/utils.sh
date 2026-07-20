# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2024-2025
# SPDX-License-Identifier: Apache-2.0
#

cmp_result () {
    local module=$1
    local test_name=$2
    local suffix=$3
    local target=$4
    local ok_path="$5"

    if [ "x$ok_path" == "x" ]; then
        ok="ok"
    else
        ok=$ok_path
    fi
    ref=$SUITE_PATH/$module/$ok/$test_name.ref.$target.$suffix
    out=$OUTPUT_DIR/$test_name.$target.$suffix

    if [ "$UPDATE_REF" == "true" ]; then
        cp -f $out $ref
    else
        cmp $ref $out || {
            diff -u $ref $out
            echo
            echo "ref: $ref"
            echo "out: $out"
            exit 1
        }
    fi
    echo -n " $target"
}

cmp_result_ref () {
    local module=$1
    local ref_name=$2
    local test_name=$3
    local suffix=$4
    local target=$5
    local ok_path="$6"

    if [ "x$ok_path" == "x" ]; then
        ok="ok"
    else
        ok=$ok_path
    fi
    ref=$SUITE_PATH/$module/$ok/$ref_name.ref.$target.$suffix
    out=$OUTPUT_DIR/$test_name.$target.$suffix

    if [ "$UPDATE_REF" == "true" ]; then
        cp -f $out $ref
    else
        cmp $ref $out || {
            diff -u $ref $out
            echo
            echo "ref: $ref"
            echo "out: $out"
            exit 1
        }
    fi
    echo -n " $target"
}

check_suite () {
    if [ ! -e "$SUITE_PATH/data/repos/audit-repo.git" ] || [ ! -d "$SUITE_PATH/data/repos/audit-repo.git" ]; then
        echo ""
        echo "Tackler-NG test setup error"
        echo ""
        echo "Tackler-NG test suite is missing at the location '$SUITE_PATH'"
        echo "The test suite is git submodule, so you must update it by running:"
        echo ""
        echo "   git submodule init"
        echo "   git submodule update"
        echo ""
        exit 1
    fi
}
