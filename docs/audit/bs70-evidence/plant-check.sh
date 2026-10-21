#!/bin/bash
# bs70, INSIDE the container (ctr.sh), on wb-work at the plant head: the samples gate against the 0.2.34
# archive whole, verify-docs, and the xtask tests (--nocapture), full logs, before the PR is opened.
set -u
L=$HOME/lanes/bs70; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp
H=$(git rev-parse --short=7 HEAD); G=$L/logs/plant-$H; mkdir -p $G
X=$L/target-work/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
export CARGO_TARGET_DIR=$L/target-work CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
WOLF=$W LUPIN=$U $X samples > $G/samples.log 2>&1; echo "EXIT $?" >> $G/samples.log; git clean -qfdX
WOLF=$W LUPIN=$U $X verify-docs > $G/verifydocs.log 2>&1; echo "EXIT $?" >> $G/verifydocs.log
cargo test --offline -p xtask -- --nocapture > $G/xtasktest.log 2>&1; echo "EXIT $?" >> $G/xtasktest.log
git clean -qfdX
echo DONE > $G/plant.done
