#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Tackler-NG 2018-2025
# SPDX-License-Identifier: Apache-2.0
#
#############################################################################


exe_path="$1"
storage="$2"
trg="$3"
report="$4"
formats="$5"
filter="$6"


build="$($exe_path --version | sed 's/^tackler //')"
version="$(echo $build | sed 's/\([^ ]\+\) .*/\1/')"

if [ "$storage" = "fs" ];then
	#inputSelector=" --input.fs.dir data/txns-$trg --input.fs.ext txn "
    # This is using check out from Git, see perf.toml
	inputSelector=""
elif [ "$storage" = "git" ]; then
	inputSelector="--input.git.ref set-$trg"
else
	echo "unkonwn storage: $storage"
	exit 1
fi



if [ -n "$filter" ]; then 
       flt="flt"
       fltOpts="--api-filter-def ${filter}"
else
       flt="all"
       fltOpts=
fi


report_file=results/hwXX/$version-perf-$flt-$storage-$trg-"$(echo $report | tr ' ' '_')"-"$(echo $formats | tr ' ' '_')".txt

(
echo "exe: $exe_path"
echo "build: $build"
echo "storage: $storage"
echo "set: $trg"
echo "version: $version"
echo "report: $report"
echo "formats: $formats"
echo "filter: $filter"
echo ""


for i in 1 2 3 4 5; do 

	rm -f out/perf-*
	/usr/bin/time -f "\nreal\t%es\nuser\t%Us\nsys\t%Ss\nmem\t%Mk (max)\ncpu\t%P" \
	"$exe_path" \
	--config perf.toml \
    --input.storage "$storage" \
	$inputSelector \
	--output.dir out \
	--output.prefix perf-$storage-$trg-$flt \
	--reports $report \
	$fltOpts
	echo
done
) > "$report_file"  2>&1

# clean up path prefix
sed -i 's@/.*perf/@perf/@' "$report_file"

