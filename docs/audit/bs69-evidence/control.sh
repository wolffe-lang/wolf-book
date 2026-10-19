#!/bin/bash
# bs69 control, INSIDE the container (ctr.sh): the unedited contract head on the book's pin
# (c32 = wolf 0.2.32 / lupin 0.1.54): the repo's gates through the xtask binary built on the host, then
# the program sweep twice for the noise floor.
set -u
L=$HOME/lanes/bs69; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
[ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
git rev-parse HEAD > $L/logs/control-head.txt
X=$L/target/debug/xtask; W=$(ls $L/c32/wolf-*/wolf); U=$(ls $L/c32/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c32-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
done
git status --porcelain > $L/logs/control-c32-status.txt
git clean -qfdX; git checkout -q -- .
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/wolf-book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c32; mv $L/logs/sweep-c32-progs.jsonl $L/logs/sweep-c32a.jsonl
bash $L/ev/sweep.sh c32; mv $L/logs/sweep-c32-progs.jsonl $L/logs/sweep-c32b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c32a.jsonl $L/logs/sweep-c32b.jsonl > $L/logs/sweep-c32-noise.txt 2>&1
echo DONE > $L/logs/control.done
