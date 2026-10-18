#!/bin/bash
# bs68, INSIDE the container (ctr.sh), on wb-work: `samples --bless` against the 0.2.32 archive whole,
# so a diagnostic snapshot whose sample id moved (ch03's s16 -> s18, two fences inserted before it in
# §3.3) is re-captured from the run, never typed. The full log is kept; the snapshot diff is read by hand.
set -u
L=$HOME/lanes/bs68; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp
X=$L/target-work/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
WOLF=$W LUPIN=$U $X samples --bless > $L/logs/work-bless.log 2>&1; echo "EXIT $?" >> $L/logs/work-bless.log
git status --porcelain > $L/logs/work-bless-status.txt
git diff -- snapshots > $L/logs/work-bless-snapshots.diff
