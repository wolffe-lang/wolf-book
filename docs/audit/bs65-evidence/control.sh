#!/bin/bash
# bs65 control on a linux host (kasumi, hasu): the unedited contract head on the book's pin
# (c26 = 0.2.26 / 0.1.49): the repo's gates, then the program sweep twice for the noise floor.
set -u
L=$HOME/lanes/bs65; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
git fetch -q origin bs65 && git merge -q --ff-only origin/bs65 || { echo FETCH FAILED > $L/logs/control.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs65 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
git rev-parse HEAD > $L/logs/control-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
cargo build -q -p xtask > $L/logs/build-xtask.log 2>&1; echo "build exit $?" >> $L/logs/build-xtask.log
W=$(ls $L/c26/wolf-*/wolf); U=$(ls $L/c26/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c26-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U cargo xtask $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
done
git status --porcelain > $L/logs/control-c26-status.txt
git checkout -q -- .
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/wolf-book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c26; mv $L/logs/sweep-c26.jsonl $L/logs/sweep-c26a.jsonl
bash $L/ev/sweep.sh c26; mv $L/logs/sweep-c26.jsonl $L/logs/sweep-c26b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c26a.jsonl $L/logs/sweep-c26b.jsonl > $L/logs/sweep-c26-noise.txt 2>&1
echo DONE > $L/logs/control.done
