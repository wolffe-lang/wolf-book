#!/bin/bash
# bs67 (from bs66, bs64): run sweep1.sh over every program for one pair, N at a time. Runs INSIDE the
# container (ctr.sh). Args: pair (c29|new) [jobs, default 8] [progs dir, default progs].
set -u
L=$HOME/lanes/bs67; P=$L/$1; J=${2:-8}; G=$L/${3:-progs}; OUT=$L/logs/sweep-$1-${3:-progs}.jsonl; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
ls -d $G/*/ | sed 's:/$::' | xargs -P $J -I{} bash $L/ev/sweep1.sh $P {} > $OUT.tmp 2> $OUT.err
sort $OUT.tmp > $OUT; rm -f $OUT.tmp
echo "$(wc -l < $OUT) records" > $OUT.done
