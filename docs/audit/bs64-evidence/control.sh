#!/bin/bash
# bs64 control: the unedited contract head on the book's pin (c25 = 0.2.25 / 0.1.48): gates, then the program sweep twice.
set -u
L=$HOME/lanes/bs64; cd $L/wolf-book || exit 1
git fetch -q origin bs64 && git merge -q --ff-only origin/bs64 || { echo FETCH FAILED > $L/logs/control.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs64 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
git rev-parse HEAD > $L/logs/control-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
cargo build -q -p xtask > $L/logs/build-xtask.log 2>&1; echo "build exit $?" >> $L/logs/build-xtask.log
W=$(ls $L/c25/wolf-*/wolf); U=$(ls $L/c25/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c25-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U cargo xtask $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
done
git status --porcelain > $L/logs/control-c25-status.txt
git checkout -q -- .
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/wolf-book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c25; mv $L/logs/sweep-c25.jsonl $L/logs/sweep-c25a.jsonl
bash $L/ev/sweep.sh c25; mv $L/logs/sweep-c25.jsonl $L/logs/sweep-c25b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c25a.jsonl $L/logs/sweep-c25b.jsonl > $L/logs/sweep-c25-noise.txt 2>&1
echo DONE > $L/logs/control.done
