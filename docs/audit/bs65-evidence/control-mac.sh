#!/bin/bash
# bs65 control on nomad-1 (macOS arm64): NO cargo build here (lane rule) — the program sweep
# alone, from the release archives, over the programs allprogs.py writes from this clone.
set -u
L=$HOME/lanes/bs65; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
git -C $L/book rev-parse HEAD > $L/logs/control-head.txt
rm -rf $L/progs; python3 $L/ev/allprogs.py $L/book $L/progs > $L/logs/allprogs.log 2>&1
bash $L/ev/sweep.sh c26 4; mv $L/logs/sweep-c26.jsonl $L/logs/sweep-c26a.jsonl
bash $L/ev/sweep.sh c26 4; mv $L/logs/sweep-c26.jsonl $L/logs/sweep-c26b.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-c26a.jsonl $L/logs/sweep-c26b.jsonl > $L/logs/sweep-c26-noise.txt 2>&1
echo DONE > $L/logs/control.done
