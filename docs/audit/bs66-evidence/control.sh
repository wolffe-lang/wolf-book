#!/bin/bash
# bs66 control on a linux host (kasumi, hasu): the unedited contract head on the book's pin
# (c28 = 0.2.28 / 0.1.51): the repo's gates, then the program sweep twice for the noise floor.
set -u
L=$HOME/lanes/bs66; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
git fetch -q origin bs66 && git merge -q --ff-only origin/bs66 || { echo FETCH FAILED > $L/logs/control.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs66 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
git rev-parse HEAD > $L/logs/control-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
cargo build -q -p xtask > $L/logs/build-xtask.log 2>&1; echo "build exit $?" >> $L/logs/build-xtask.log
W=$(ls $L/c28/wolf-*/wolf); U=$(ls $L/c28/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c28-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U cargo xtask $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
done
git status --porcelain > $L/logs/control-c28-status.txt
git checkout -q -- .
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/wolf-book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c28; mv $L/logs/sweep-c28.jsonl $L/logs/sweep-c28a.jsonl
bash $L/ev/sweep.sh c28; mv $L/logs/sweep-c28.jsonl $L/logs/sweep-c28b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c28a.jsonl $L/logs/sweep-c28b.jsonl > $L/logs/sweep-c28-noise.txt 2>&1
echo DONE > $L/logs/control.done
