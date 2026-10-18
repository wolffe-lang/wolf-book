#!/bin/bash
# bs68 subject, INSIDE the container (ctr.sh): the unedited prediction head on the new pair:
# samples (archive whole, and bare), verify-docs, ledger --check, diagrams --check with LUPIN_TRACE;
# the 580-program sweep twice; the comparison against c30.
set -u
L=$HOME/lanes/bs68; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp
[ "$(git rev-parse --abbrev-ref HEAD)" = bs68 ] || { echo WRONG BRANCH > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --short=7 HEAD)" = "${SUBJECT_HEAD:-PREDICTION_HEAD_UNSET}" ] || { echo WRONG HEAD > $L/logs/subject.done; exit 1; }
git rev-parse HEAD > $L/logs/subject-head.txt
X=$L/target/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check"; do n=subject-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U $X $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n; git clean -qfdX; done
WOLF=$L/bare/wolf LUPIN=$U $X samples > $L/logs/subject-samples-bare.log 2>&1; echo "EXIT $?" >> $L/logs/subject-samples-bare.log; git clean -qfdX
WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X diagrams --check > $L/logs/subject-diagramscheck.log 2>&1; echo "EXIT $?" >> $L/logs/subject-diagramscheck.log
git status --porcelain > $L/logs/subject-status.txt
git checkout -q -- .
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new-progs.jsonl $L/logs/sweep-newa.jsonl
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new-progs.jsonl $L/logs/sweep-newb.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-newa.jsonl $L/logs/sweep-newb.jsonl > $L/logs/sweep-new-noise.txt 2>&1
python3 $L/ev/cmp.py $L/logs/sweep-c30a.jsonl $L/logs/sweep-newa.jsonl > $L/logs/sweep-c30-vs-new.txt 2>&1
echo DONE > $L/logs/subject.done
