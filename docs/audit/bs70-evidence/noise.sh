#!/bin/bash
# bs70, INSIDE the container (ctr.sh): the release moves that sit outside both pairs' two-sweep noise
# (ex11-5, ex17-1) and the ch17 races beside them, each built --release once per pair and run 40 times,
# plain and under taskset -c 0-3, the outputs tallied: a racy program's distribution, not one draw.
set -u
L=$HOME/lanes/bs70; export TMPDIR=$L/tmp
for id in ch11_ex11_5_lu ch17_ex17_1_lu ch17_md_113 ch17_md_29 ch17_md_325 appx_exB_11_lu ch10_md_84; do
  for P in c33 new; do W=$(ls $L/$P/wolf-*/wolf); T=$(mktemp -d -p $TMPDIR); cp $L/progs/$id/$id.lu $T/r.lu; cd $T
    timeout 300 $W build --release r.lu > b.out 2>&1 < /dev/null || { echo "$id $P build failed"; cd $L; rm -rf $T; continue; }
    for mode in plain taskset; do
      for i in $(seq 40); do if [ $mode = plain ]; then timeout 30 ./r 2>&1 < /dev/null; else timeout 30 taskset -c 0-3 ./r 2>&1 < /dev/null; fi; echo "[exit $?]"; echo "@@"; done > runs.txt
      echo "== $id $P $mode: $(python3 -c "import collections,sys; c=collections.Counter(open('runs.txt').read().split('@@\n')[:-1]); print('; '.join(f'{n}x {k!r}' for k,n in c.most_common()))")"
    done; cd $L; rm -rf $T; done; done
