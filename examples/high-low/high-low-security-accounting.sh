#!/bin/sh
# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2024-2026
# SPDX-License-Identifier: Apache-2.0
#
set -e

exe_dir=$(dirname $(realpath $0))
setup_dir="$exe_dir/setup"


if  [ -e "$exe_dir/acme-inc/hq.git" ] || \
    [ -e "$exe_dir/acme-inc/store-a.git" ] || \
    [ -e "$exe_dir/acme-inc/store-b.git" ]; \
then
    echo "Error: acme-inc directory is not empty"
    exit 1
fi

rm -f  "$exe_dir/reports/"*.txt

cd "$exe_dir/acme-inc"

echo
echo "#################################################"
echo "### Init HQ" 
echo "#################################################"
(
    ts="2024-12-01T10:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/hq/init.sh" \
        "$setup_dir/hq/conf/hq-conf.toml" \
        "$setup_dir/hq/conf/hq-accounts.toml" \
        "$setup_dir/hq/conf/hq-commodities.toml" \
        "$setup_dir/hq/conf/hq-tags.toml"
)

echo
echo "#################################################"
echo "Init Store A" 
echo "#################################################"
(
    ts="2024-12-01T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-a/init.sh" \
        "$setup_dir/store-a/conf/store-a-conf.toml" \
        "$setup_dir/store-a/conf/store-a-accounts.toml" \
        "$setup_dir/store-a/conf/store-a-commodities.toml" \
        "$setup_dir/store-a/conf/store-a-tags.toml"
)

echo
echo "#################################################"
echo "Init Store B" 
echo "#################################################"
(
    ts="2024-12-01T14:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-b/init.sh"
)

echo
echo "#################################################"
echo "Setup connection between HQ and stores" 
echo "#################################################"
(
    cd hq
    sh "$setup_dir/hq/remotes.sh"
)

echo
echo "#################################################"
echo "Store A: sales #001" 
echo "#################################################"
(
    cd store-a
    ts="2024-12-17T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-a/sales.sh" \
        "001" "$setup_dir/store-a/sales-001.txn"
)

echo
echo "#################################################"
echo "Store B: sales #001" 
echo "#################################################"
(
    cd store-b
    ts="2024-12-17T14:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-b/sales.sh" \
        "001" "$setup_dir/store-b/sales-001.txn"
)

echo
echo "#################################################"
echo "HQ: Syncronize journals and report" 
echo "#################################################"
(
    cd hq
    ts="2024-12-17T16:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/hq/update.sh"

    sh "$setup_dir/hq/report.sh" \
        --output.dir "$exe_dir/reports" \
        --output.prefix hq-begin
)

echo
echo "#################################################"
echo "Store B: sales #002" 
echo "#################################################"
(
    cd store-b
    ts="2024-12-18T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-b/sales.sh" \
        "002" "$setup_dir/store-b/sales-002.txn"
)

echo
echo "#################################################"
echo "Store B: sales #003" 
echo "#################################################"
(
	cd store-b
    ts="2024-12-19T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
	sh "$setup_dir/store-b/sales.sh" \
        "003" "$setup_dir/store-b/sales-003.txn"
)

echo
echo "#################################################"
echo "Store A: sales #002" 
echo "#################################################"
(
    cd store-a
    ts="2024-12-20T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/store-a/sales.sh" \
        "002" "$setup_dir/store-a/sales-002.txn"
)

echo
echo "#################################################"
echo "HQ: Syncronize journals and report" 
echo "#################################################"
(
    cd hq
    ts="2024-12-20T16:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts"
    export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
    sh "$setup_dir/hq/update.sh"

    sh "$setup_dir/hq/report.sh" \
        --output.dir "$exe_dir/reports" \
        --output.prefix hq-final
)

echo
echo "#################################################"
echo "HQ: Report only Store A" 
echo "#################################################"
(
    cd hq
    sh "$setup_dir/hq/filter.sh" \
        --output.dir "$exe_dir/reports" \
        --output.prefix hq-filter
)

echo
echo "#################################################"
echo "Store A: Report" 
echo "#################################################"
(
    cd store-a
    sh "$setup_dir/store-a/report.sh" \
        --output.dir "$exe_dir/reports" \
        --output.prefix store-a
)

echo
echo
echo "High-Low Security Accounting setup is located under: $(pwd)"
