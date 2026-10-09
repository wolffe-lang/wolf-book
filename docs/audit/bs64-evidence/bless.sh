#!/bin/bash
# bs64: snapshots by --bless on the new pair, then a samples run whose drifted console and
# diagnostic,from blocks splice.py regenerates from the run's own actual:. Output: a patch.
set -u
L=$HOME/lanes/bs64; cd $L/wolf-book || exit 1
git checkout -q -- . ; git fetch -q origin bs64 && git merge -q --ff-only origin/bs64 || { echo FETCH FAILED > $L/logs/bless.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs64 ] || { echo WRONG BRANCH > $L/logs/bless.done; exit 1; }
git rev-parse HEAD > $L/logs/bless-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 WOLF=$(ls $L/new/wolf-*/wolf) LUPIN=$(ls $L/new/lupin-*/lupin)
cargo xtask samples --bless > $L/logs/bless.log 2>&1; echo "EXIT $?" >> $L/logs/bless.log
git diff --stat > $L/logs/bless-stat.txt
cargo xtask samples > $L/logs/bless-after.log 2>&1; echo "EXIT $?" >> $L/logs/bless-after.log
python3 $L/ev/splice.py $L/wolf-book $L/logs/bless-after.log book/ch08/s3 > $L/logs/splice.log 2>&1; echo "EXIT $?" >> $L/logs/splice.log
cargo xtask samples > $L/logs/bless-final.log 2>&1; echo "EXIT $?" >> $L/logs/bless-final.log
git diff > $L/logs/bless.patch
git status --porcelain > $L/logs/bless-status.txt
echo DONE > $L/logs/bless.done
