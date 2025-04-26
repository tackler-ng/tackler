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

if [ ! -d "$repo_name" ]; then
    echo "Repository must exists with set-1e2 branch"
    exit 1
fi

cd "$repo_name"

git checkout main

echo " * err-1e2" >> readme.txt
git add readme.txt
ts="2015-12-31T14:22:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "readme: err-1e2" readme.txt
git push

git checkout -b err-1e2 set-1e2

echo "Tackler test repository for git storage backend" > readme.txt
echo >> readme.txt
echo "This is error case branch" >> readme.txt
echo >> readme.txt
echo "Available errors" >> readme.txt
echo " * Missing uuid" >> readme.txt
echo "   * dir: txns/2016/04" >> readme.txt
echo "   * txn: txns/2016/04/01/20160401T120000-26.txn" >> readme.txt
echo " * Duplicate UUIDs (d38ad90e-a83d-561c-a5bd-546f8ebb2472)" >> readme.txt
echo "   * dir: txns/2016/05" >> readme.txt
echo "   * txn: txns/2016/05/04/20160504T103336-35.txn" >> readme.txt
echo "   * txn: txns/2016/05/08/20160508T022400-36.txn" >> readme.txt
echo "   * txn: txns/2016/05/19/20160519T015512-39.txn" >> readme.txt
echo "   * txn: txns/2016/05/30/20160530T012624-42.txn" >> readme.txt
git add readme.txt

echo "set: err-1e2" > info.txt
git add info.txt

ts="2016-12-31T12:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "Error readme" readme.txt info.txt

# remove uuid
sed -i '/^.*8cb3cb3d-c61c-55a7-b459-327bd5aa7dd5.*$/d'          txns/2016/04/01/20160401T120000-26.txn
# duplicate uuid with txns/2016/05/04/20160504T103336-35.txn
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/08/20160508T022400-36.txn
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/19/20160519T015512-39.txn 
sed -i 's/uuid: .*/uuid: d38ad90e-a83d-561c-a5bd-546f8ebb2472/' txns/2016/05/30/20160530T012624-42.txn
git add txns/2016/04/
git add txns/2016/05/

ts="2016-12-31T18:00:00+0000"
GIT_AUTHOR_DATE="$ts" GIT_COMMITTER_DATE="$ts" \
    git commit -m "Errors"


echo "step: git gc --prune=now"
git gc --prune=now

git push --set-upstream origin err-1e2
