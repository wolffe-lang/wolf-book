#!/bin/bash
# bs69, INSIDE the container (ctr.sh): every program the runner executes at the head (wb-work, the commit
# named in SUBJECT_HEAD), swept once on the new pair, and compared program by program with the unedited
# tree's sweep (sweep-newa): the moves are the page edits and nothing else.
set -u
L=$HOME/lanes/bs69; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp
H=$(git rev-parse --short=7 HEAD); [ "$H" = "${SUBJECT_HEAD:-UNSET}" ] || { echo "WRONG HEAD $H" > $L/logs/head-sweep.done; exit 1; }
rm -rf $L/progs-head; python3 $L/ev/allprogs.py $L/wb-work $L/progs-head > $L/logs/allprogs-head.log 2>&1
bash $L/ev/sweep.sh new 8 progs-head; mv $L/logs/sweep-new-progs-head.jsonl $L/logs/sweep-head-$H.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-newa.jsonl $L/logs/sweep-head-$H.jsonl > $L/logs/sweep-newa-vs-head-$H.txt 2>&1
echo DONE > $L/logs/head-sweep.done
