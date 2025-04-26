#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Tackler-NG 2018-2025
# SPDX-License-Identifier: Apache-2.0
#############################################################################
#
# Initialize test repository for tackler git backend
#
# Test data is generated with pta-generator
# (https://github.com/tackler-ng/pta-generator/)
#
# This will produce stable git repository (e.g. commit ids) on repeatable runs.
#
# For historic reasons this data set is generated for year 2016:
#    pta-generator audit --path sets --shard-type txn \
#      --start 2016-01-01T00:00:00+0000[UTC] \
#      --stop  2017-01-01T00:00:00+0000[UTC] \
#      --set-size ...
#
usage () {
    echo "Initialize and populate git repository with test data"
    echo
	echo "Usage: $0 <repo name> <1e1 | 1e2 | 1e3 | 1e4 | 1e5 | 1e6>"
}

if [ $# != 2 ]; then
    usage
    exit 1
fi

repo_name="$1"

set="$2"
name="set-$set"
store="$(pwd)/sets/audit/${name}-txn"

if [ ! -d "$store" ]; then
	echo "Error: $name not found"
	exit 1
fi

if [ ! -d "$repo_name" ]; then
    git init --bare "$repo_name.git"
    touch "$repo_name.git/refs/heads/.keep"
    touch "$repo_name.git/refs/tags/.keep"
    touch "$repo_name.git/hooks/.keep"

    git clone "$repo_name.git"
    
    cd "$repo_name"
    
    git config user.name tackler
    git config user.email "accounting@example.com"
    git config commit.gpgSign no
    git config gc.autoDetach no

    ts="2015-12-31T10:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
        git commit --allow-empty -m "init"
    cd ..
fi

cd "$repo_name"

if [ ! -e readme.txt ]; then
    echo "Tackler test repository for git storage backend" > readme.txt
    echo >> readme.txt
    echo "Structure of the repository" >> readme.txt
    echo " * Each test set is own branch" >> readme.txt
    echo " * Transaction directory is 'txns'" >> readme.txt
    echo " * Txn shard is by year, month and day (YYYY/MM/DD)" >> readme.txt
    echo " * Each txn is separate file, named by timestamp and index" >> readme.txt
    echo >> readme.txt
    echo "Available git objects" >> readme.txt
    echo " * branch for each test set" >> readme.txt
    echo " * tags by month for each set" >> readme.txt
    echo " * commit time is the time of the last txn of the day" >> readme.txt
    echo >> readme.txt
    echo "Available test branches:" >> readme.txt

    git add readme.txt
    ts="2015-12-31T12:00:00+0000"
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
        git commit -m 'init (main)'
    git push --set-upstream origin main
fi

git checkout main


set_min=$(echo $set | tr -d 'e')

echo " * $name" >> readme.txt
git add readme.txt
ts="2015-12-31T14:${set_min}:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "readme: $name" readme.txt
git push

git checkout -b "$name"

echo "set: $name" > "info.txt"
git add "info.txt"
ts="2016-01-01T00:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "$name: init"
 
mkdir -p txns
mkdir -p txns/2016

for M in $(seq 1 12); do
    m=$(printf "%02d" $M)

    src="$store/txns/2016/$m"
    trg="txns/2016/$m"

    echo "step: $name, month: $m"

    # All sets does't have all months
    if [ ! -d "$src" ]; then
        echo "step: skip  $name, month: $m"
        echo
        continue
    fi
    cp -a "$src" "$trg"

    for D in $(seq 1 31); do
        d=$(printf '%02d' $D)
        trg_d="$trg/$d"

        if [ ! -d "$trg_d" ]; then
            echo "step: skip $name, month: $m, day: $d"
            continue
        fi
        git add "$trg_d"

        ts=$(ls $trg_d/*.txn | sort |tail -n1 | sed 's@.*/2016\(.*\)-[0-9]\+.txn@2016\1Z@')
        GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
            git commit -m "$name: 2016-$m-$d"
    done
    GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
        git tag -a -m "$name: 2016-$m" "s${set}_2026-$m"
    echo "step: git gc"
    git gc
    
    echo 
    echo "done $name, round: $m"
done

echo "step: git gc --prune=now"
git gc --prune=now

git push --set-upstream origin $name
