#!/bin/bash
# bs65 (from bs64): run sweep1.sh over every program for one pair, N at a time.
# Args: pair (c26|new) [jobs, default 8]. Scratch under $L/tmp (TMPDIR), never /tmp.
set -u
L=$HOME/lanes/bs65; P=$L/$1; J=${2:-8}; OUT=$L/logs/sweep-$1.jsonl; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
ls -d $L/progs/*/ | sed 's:/$::' | xargs -P $J -I{} bash $L/ev/sweep1.sh $P {} > $OUT.tmp 2> $L/logs/sweep-$1.err
sort $OUT.tmp > $OUT; rm -f $OUT.tmp
echo "$(wc -l < $OUT) records" > $L/logs/sweep-$1.done
