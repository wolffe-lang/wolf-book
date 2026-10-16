#!/bin/bash
# bs67 control, INSIDE the container (ctr.sh): the unedited contract head on the book's pin
# (c29 = 0.2.29 / 0.1.52): the repo's gates through the xtask binary built on the host, then the
# program sweep twice for the noise floor. The checkout is updated on the host first (control-host.sh).
set -u
L=$HOME/lanes/bs67; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
[ "$(git rev-parse --abbrev-ref HEAD)" = bs67 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
git rev-parse HEAD > $L/logs/control-head.txt
X=$L/target/debug/xtask; W=$(ls $L/c29/wolf-*/wolf); U=$(ls $L/c29/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c29-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
done
git status --porcelain > $L/logs/control-c29-status.txt
git clean -qfdX; git checkout -q -- .
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/wolf-book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c29; mv $L/logs/sweep-c29-progs.jsonl $L/logs/sweep-c29a.jsonl
bash $L/ev/sweep.sh c29; mv $L/logs/sweep-c29-progs.jsonl $L/logs/sweep-c29b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c29a.jsonl $L/logs/sweep-c29b.jsonl > $L/logs/sweep-c29-noise.txt 2>&1
echo DONE > $L/logs/control.done
