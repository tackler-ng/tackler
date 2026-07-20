#!/bin/bash
# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# This is a tool to check T3DB and test records
#
# Included checks are:
# - Check tests for missing ids (exec-files):
# - Check tests for duplicate ids (scalatest + exec-files):
# - Check T3DB YAML validity:
# - Check T3DB for duplicate test ids:
# - Check T3DB for non-exist refids:
# - Check T3DB for non-exist parents:
# - Cross check T3DB and tests ids:
# - Check JSON validity:
#
sh_pykwalify=pykwalify

exe_dir=$(dirname $(realpath $0))

db_dir="$exe_dir/../../tackler-t3db"

# good enough for know
t3db_00="$db_dir/tests.yml"
t3db_ng_00="$db_dir/tests-ng.yml"
t3db_01="$db_dir/tests-1001.yml"
t3db_02="$db_dir/tests-1002.yml"
# t3db_03="$db_dir/tests-1003.yml" not implemented yet
t3db_04="$db_dir/tests-1004.yml"
t3db_05="$db_dir/tests-1005.yml"
t3db_06="$db_dir/tests-1006.yml"
t3db_07="$db_dir/tests-1007.yml"
t3db_08="$db_dir/tests-1008.yml"
t3db_09="$db_dir/tests-1009.yml"
t3db_10="$db_dir/tests-1010.yml"
t3db_11="$db_dir/tests-1011.yml"
t3db_12="$db_dir/tests-1012.yml"
t3db_13="$db_dir/tests-1013.yml"
t3db_14="$db_dir/tests-1014.yml"
t3db_15="$db_dir/tests-1015.yml"
t3db_16="$db_dir/tests-1016.yml"
t3db_18="$db_dir/tests-1018.yml"


T3DBs="$t3db_00 $t3db_ng_00 $t3db_01 $t3db_02 $t3db_04 $t3db_05 $t3db_06 $t3db_07 $t3db_08 $t3db_09 $t3db_10 $t3db_11 $t3db_12 $t3db_13 $t3db_14 $t3db_15 $t3db_16 $t3db_18"

rgx_test=' +((test:)|(test-ref:)) +[[:xdigit:]]+-[[:xdigit:]]+-[[:xdigit:]]+-[[:xdigit:]]+-[[:xdigit:]]+ *$'
rgx_test_func=' +id_+[[:xdigit:]]+_[[:xdigit:]]+_[[:xdigit:]]+_[[:xdigit:]]+_[[:xdigit:]]+__'

if [ -e ../build.sbt ]; then
    tackler_version="SC"
elif [ -e ../Cargo.toml ]; then
    tackler_version="RS"
else
    echo "Can't detect tackler type"
    exit 1
fi

get_source_files () {
    if [ "$tackler_version" == "SC" ]; then
        find "$exe_dir/../api/src/" "$exe_dir/../core/src/" "$exe_dir/../cli/src/" -name '*.scala'
    elif [ "$tackler_version" == "RS" ]; then
        find \
            "$exe_dir/../tackler-rs/src/" \
            "$exe_dir/../tackler-api/src/" \
            "$exe_dir/../tackler-core/src/" \
            "$exe_dir/../tackler-core/tests/" \
            "$exe_dir/../tackler-cli/src/" \
            -name '*.rs'
    else
        echo "Unknown tackler type"
        exit 1
    fi
}

get_t3db_content () {
    grep -Ehv '^[[:space:]]*#' $T3DBs
}

get_t3db_feature_ids () {
    get_t3db_content | grep -EA1 ' feature:' |  grep -E '[[:space:]]+id:' | sed -E 's/[[:space:]]+id: +//'
}

get_t3db_test_ids () {
    get_t3db_content | grep -EA1 ' (error|test):' | grep -E '[[:space:]]+id:' | sed -E 's/[[:space:]]+id: +//'
}

get_rust_test_ids () {
    # shell-based tests
    find "$exe_dir/../tests/sh" -name '*.sh' |\
        xargs grep -Eh '^#'"$rgx_test"
}

get_scala_test_ids () {
    # exec-based tests
    find "$exe_dir" -name '*.exec' |\
        xargs grep -Eh '^#'"$rgx_test"
}

#sed -E 's/^# +test: +//'
get_test_ids () {
    (
        if [ "$tackler_version" == "SC" ]; then
            get_scala_test_ids
        elif [ "$tackler_version" == "RS" ]; then
            get_rust_test_ids
        else
            echo "Unknown tackler type"
            exit 1
        fi

        # unit and integration tests
        get_source_files |\
            xargs grep -Eh '(((\*)|(//))('"$rgx_test"'))|(fn'"$rgx_test_func"')'
    )|\
        sed -E 's/.* +((test:)|(test-ref:)) +//'
}

t3db_feature_id_lst=$(mktemp /tmp/t3db_feature_lst.XXXXXX)
trap "rm -f $t3db_feature_id_lst" 0

t3db_id_lst=$(mktemp /tmp/t3db_id_lst.XXXXXX)
trap "rm -f $t3db_id_lst" 0

test_id_lst=$(mktemp /tmp/test_id_lst.XXXXXX)
trap "rm -f $test_id_lst" 0

test_id_dups_lst=$(mktemp /tmp/test_id_dups_lst.XXXXXX)
trap "rm -f $test_id_dups_lst" 0

get_t3db_feature_ids | sort > $t3db_feature_id_lst
get_t3db_test_ids | sort > $t3db_id_lst
get_test_ids | sort > $test_id_dups_lst
cat $test_id_dups_lst | uniq > $test_id_lst

echo "Check tests for missing ids (exec-files):"
find "$exe_dir" -name '*.exec' | xargs grep -E '#'"$rgx_test" -L

# this is already checked by diff, but print dups again here
echo "Check tests for duplicate ids (unit and exec tests):"
cat $test_id_dups_lst | uniq -d


echo "Check T3DB YAML validity:"
for test_db in $T3DBs
do
    $sh_pykwalify -v -s  "$db_dir/tests-schema.yml" -d  "$test_db"
done

echo "Check T3DB for duplicate test ids:"
cat $t3db_id_lst | uniq -d

echo "Check T3DB for non-exist refids:"
get_t3db_content | grep ' refid:' | sed 's/.*refid: //' | while read refid;
do
    grep -Eq -L "$refid" $t3db_id_lst || echo $refid
done

echo "Check T3DB for non-exist parents:"
get_t3db_content | grep ' parent:' | sed 's/.*parent: //' | while read parent;
do
    grep -Eq -L "$parent" $t3db_feature_id_lst || echo $parent
done

echo "Cross check T3DB and tests ids:"

diff -u $t3db_id_lst $test_id_lst | grep -v -- '---' | grep '^-' |\
    while read raw_uuid; do
        uuid=$(echo $raw_uuid | sed 's/^-//')
        echo "In T3DB, no test: $(echo $uuid | sed 's/^-//')"
        # grep -E ' +id: +'"$uuid" $T3DBs
    done

diff -u $t3db_id_lst $test_id_lst | grep -v '+++' | grep '^+' |\
    while read raw_uuid; do
        uuid=$(echo $raw_uuid | sed 's/^+//')
        echo "In test, no T3DB: $uuid"
        # (
        #     find $exe_dir/ -name '*.exec'
        #     get_source_files
        # ) | xargs grep $uuid
    done

echo "T3DB test count: $(cat $t3db_id_lst | wc -l)"
echo "Impl Test count: $(cat $test_id_lst | wc -l)"

echo "Check JSON validity:"
find "$exe_dir" -name '*.json' -exec "$db_dir/json_lint.sh" {} \;

echo 
echo "Silence is gold"

