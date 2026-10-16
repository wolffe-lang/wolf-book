#!/bin/bash
# bs67, wolf-book#80: §13.1's three rows re-measured on kasumi (linux x86-64, 16 cpus): the book's
# printed program and two reconstructions from wolf-lang#421's description (the second row "counts
# the words() view, not a lowercased copy"; the third is "acc = (acc * 1103515245 + 12345) %
# 2147483647, 4M iterations" per piece), 72 pieces of 379,900 characters (the three sentences, 2,900
# times). Each program built with `wolf build` by each toolchain named, and the BINARY timed (no
# compile counted); 7 rounds, every binary of every toolchain once per round, so load moves both
# toolchains alike; medians after. No signals: runs on the host. Args: pair... (c15 new).
set -u
L=$HOME/lanes/bs67; O=$L/logs/par.log; PROGS="alloc_serial alloc_par view_serial view_par spin_serial spin_par"
{ echo "== start $(date -u +%FT%TZ); load $(cut -d' ' -f1-3 /proc/loadavg); nproc $(nproc)"
  for P in "$@"; do W=$(ls $L/$P/wolf-*/wolf); D=$L/par-$P; rm -rf $D; mkdir -p $D; echo "== $P: $($W --version | head -1)"
    for p in $PROGS; do mkdir -p $D/$p; cp $L/ev/par/$p.lu $D/$p/
      (cd $D/$p && $W build $p.lu > build.log 2>&1; echo "build $P/$p exit $? size $(stat -c %s $p 2>/dev/null)"; cat build.log); done; done
  for i in 1 2 3 4 5 6 7; do for P in "$@"; do for p in $PROGS; do
    (cd $L/par-$P/$p && /usr/bin/time -f "$P/$p run$i real %e user %U sys %S" ./$p > out.txt 2> time.txt; tail -1 time.txt; echo "   out $(cat out.txt)")
  done; done; echo "-- round $i done, load $(cut -d' ' -f1-3 /proc/loadavg)"; done
  echo "== end $(date -u +%FT%TZ); load $(cut -d' ' -f1-3 /proc/loadavg)"
} > $O 2>&1
python3 - $O > $L/logs/par-median.txt <<'PY'
import sys,re,statistics as st
rows={}
for l in open(sys.argv[1]):
    m=re.match(r'(\S+) run\d+ real ([\d.]+) user ([\d.]+) sys ([\d.]+)',l)
    if m: rows.setdefault(m.group(1),[]).append(tuple(map(float,m.group(2,3,4))))
for k,v in rows.items():
    r=[x[0] for x in v]
    print(f"{k:18s} n={len(v)} real median {st.median(r):.2f} (min {min(r):.2f} max {max(r):.2f}) user {st.median(x[1] for x in v):.2f} sys {st.median(x[2] for x in v):.2f}")
PY
for P in "$@"; do rm -rf $L/par-$P; done; echo DONE > $L/logs/par.done
