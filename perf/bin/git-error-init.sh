#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#############################################################################
#
# Add error cases to the test repository
#
usage () {
    echo "Add error case to the test repository"
    echo
	echo "Usage: $0 <repo name>"
}

if [ $# != 1 ]; then
    usage
    exit 1
fi

repo_name="$1"
name="err-1e2"

if [ ! -d "$repo_name" ]; then
    echo "Repository must exists with set-1e2 branch"
    exit 1
fi

cd "$repo_name"

git checkout main

echo " * $name" >> readme.txt
git add readme.txt
ts="2015-12-31T14:22:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "main: $name" readme.txt
git push

git checkout -b "$name" set-1e2

echo "Tackler test repository for git storage backend" > readme.txt
echo >> readme.txt
echo "This is error branch: $name" >> readme.txt
echo >> readme.txt
echo "Available errors" >> readme.txt
echo " * Missing uuid" >> readme.txt
echo "   * tag: e_01_miss_uuid" >> readme.txt
echo "   * dir: txns/2016/04" >> readme.txt
echo "   * txn: txns/2016/04/01/20160401T120000-26.txn" >> readme.txt
echo " * Duplicate UUIDs" >> readme.txt
echo "   * tag: e_02_dup_uuid" >> readme.txt
echo "   * dir: txns/2016/05" >> readme.txt
echo "   * txn: txns/2016/05/04/20160504T103336-35.txn" >> readme.txt
echo "   * txn: txns/2016/05/08/20160508T022400-36.txn" >> readme.txt
echo "   * txn: txns/2016/05/19/20160519T015512-39.txn" >> readme.txt
echo "   * txn: txns/2016/05/30/20160530T012624-42.txn" >> readme.txt
echo "   * dup: d38ad90e-a83d-561c-a5bd-546f8ebb2472" >> readme.txt
git add readme.txt

echo "branch: $name" > info.txt
git add info.txt

ts="2016-12-31T12:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "init ($name)" readme.txt info.txt

#
# remove uuid
#
sed -i '/^.*8cb3cb3d-c61c-55a7-b459-327bd5aa7dd5.*$/d'          txns/2016/04/01/20160401T120000-26.txn
git add txns/2016/04/

ts="2016-12-31T15:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "$name: error - missing uuid"

GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git tag -a -m "$name: missing uuid" "e_01_miss_uuid"

#
# duplicate uuid with txns/2016/05/04/20160504T103336-35.txn
#
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/08/20160508T022400-36.txn
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/19/20160519T015512-39.txn 
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/30/20160530T012624-42.txn
git add txns/2016/05/

ts="2016-12-31T18:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "$name: error - duplicate uuid"

GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git tag -a -m "$name: duplicate uuid" "e_02_dup_uuid"


echo "step: git gc --prune=now"
git gc --prune=now

git push --set-upstream origin err-1e2
