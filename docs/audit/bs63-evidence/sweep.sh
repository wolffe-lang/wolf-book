#!/bin/bash
# bs63: run sweep1.sh over every program for one pair, 8 at a time. Args: pair (c23|c24|new)
set -u
L=$HOME/lanes/bs63; P=$L/$1; OUT=$L/logs/sweep-$1.jsonl
ls -d $L/progs/*/ | sed 's:/$::' | xargs -P 8 -I{} bash $L/ev/sweep1.sh $P {} > $OUT.tmp 2> $L/logs/sweep-$1.err
sort $OUT.tmp > $OUT; rm -f $OUT.tmp
echo "$(wc -l < $OUT) records" > $L/logs/sweep-$1.done
