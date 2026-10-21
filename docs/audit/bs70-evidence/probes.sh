#!/bin/bash
# bs70, INSIDE the container (ctr.sh): chapter 34's programs, each alone in its own directory, on
# the four machines of the new pair: lupin (its own archive, no std beside it), `wolf run` (native),
# `conform-run --json` (the native record), `conform-run --json --checked` (THE checked machine;
# `wolf run --checked` is the native build), and a `--release` build and its binary; then `wolf run`
# again under taskset -c 0-3. The container's SigBlk/SigIgn are printed by ctr.sh at the head of the
# log, and each program's own are printed here before it runs. Args: [progs dir, default ev/ch34].
set -u
L=$HOME/lanes/bs70; export TMPDIR=$L/tmp; unset WOLF_STD LUPIN_STD
SRC=${1:-$L/ev/ch34}; P=${PAIR:-new}; W=$(ls $L/$P/wolf-*/wolf); U=$(ls $L/$P/lupin-*/lupin)
echo "########## $($W --version | head -1) / $($U --version)"
grep -E '^Sig(Blk|Ign)' /proc/self/status
for f in $SRC/*.lu; do p=$(basename $f .lu)
  T=$(mktemp -d -p $TMPDIR); mkdir $T/$p; cp $f $T/$p/; cd $T/$p
  echo; echo "=== $p.lu"; cat $p.lu
  echo "\$ lupin $p.lu"; timeout 60 $U $p.lu 2>&1 < /dev/null; echo "[exit $?]"
  echo "\$ wolf run $p.lu"; timeout 120 $W run $p.lu 2>&1 < /dev/null; echo "[exit $?]"
  echo "\$ wolf conform-run --json $p.lu"; timeout 120 $W conform-run --json $p.lu 2>&1 < /dev/null | cut -c1-1500; echo "[exit ${PIPESTATUS[0]}]"
  echo "\$ wolf conform-run --json --checked $p.lu"; timeout 120 $W conform-run --json --checked $p.lu 2>&1 < /dev/null | cut -c1-1500; echo "[exit ${PIPESTATUS[0]}]"
  echo "\$ wolf build --release $p.lu && ./$p"; timeout 300 $W build --release $p.lu 2>&1 < /dev/null; b=$?; echo "[build exit $b]"
  if [ $b = 0 ]; then timeout 60 ./$p 2>&1 < /dev/null; echo "[exit $?]"; fi
  echo "\$ taskset -c 0-3 wolf run $p.lu"; timeout 120 taskset -c 0-3 $W run $p.lu 2>&1 < /dev/null; echo "[exit $?]"
  cd $L; rm -rf $T
done
