#!/bin/sh
# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2019-2024
# SPDX-License-Identifier: Apache-2.0
#
#############################################################################
#
#
# Tackler example script: verify txn set checksum
#
#   1. Collect all Transcation UUIDs
#   2. Normalize UUIDs to lovercase form
#   3. Sort UUIDs
#   4. Compute SHA-256
#   5. Print out plain hash
#
if [ $# -ne 1 ]; then
    echo "Usage: $0 <txns dir>"
    exit 1
fi

txns_dir="$1"

(
    find "$txns_dir"  -type f -name '*.txn' |\
    xargs -n10 grep -E -h '^[[:space:]]+#[[:space:]]+uuid:[[:space:]]+'
) |\
    sed -E 's/^[[:space:]]+#[[:space:]]+uuid:[[:space:]]+([a-fA-F0-9-]+)[[:space:]]*/\1/' |\
    tr 'A-F' 'a-f' |\
    sort |\
    sha256sum |\
    sed -E 's/([a-f0-9]+) +-/\1/'

