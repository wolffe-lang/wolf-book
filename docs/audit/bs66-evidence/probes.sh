#!/bin/bash
# bs66 probes on kasumi, both pairs: the five builtins 0.2.29 adds (os_pid, os_ppid,
# os_signal_number, os_signal_send, os_signal_ignored), the shape rule on a listen, and the BUNDLED
# std's std.signal / std.env (the archive whole, no WOLF_STD) — each program alone in its own
# directory, on lupin, `wolf run` (native), `conform-run --json` (native record),
# `conform-run --json --checked`, and a `--release` build and run. The book has no page for any of
# these; this is the measurement behind the no-page list, not a sample.
set -u
L=$HOME/lanes/bs66; export TMPDIR=$L/tmp; unset WOLF_STD LUPIN_STD
for P in c28 new; do
  W=$(ls $L/$P/wolf-*/wolf); U=$(ls $L/$P/lupin-*/lupin)
  echo "########## $P: $($W --version | head -1) / $($U --version)"
  for p in ids number send ignored listen stdnine stduser stdsend stdpid; do
    T=$(mktemp -d); mkdir $T/$p; cp $L/ev/probes/$p.lu $T/$p/; cd $T/$p
    echo "=== $p.lu ($P)"; cat $p.lu
    echo "\$ lupin $p.lu"; timeout 60 $U $p.lu 2>&1 < /dev/null; echo "[exit $?]"
    echo "\$ lupin conform-run --json $p.lu"; timeout 60 $U conform-run --json $p.lu 2>&1 < /dev/null | cut -c1-1500; echo "[exit ${PIPESTATUS[0]}]"
    echo "\$ wolf run $p.lu"; timeout 60 $W run $p.lu 2>&1 < /dev/null; echo "[exit $?]"
    echo "\$ wolf conform-run --json $p.lu"; timeout 60 $W conform-run --json $p.lu 2>&1 < /dev/null | cut -c1-1500; echo "[exit ${PIPESTATUS[0]}]"
    echo "\$ wolf conform-run --json --checked $p.lu"; timeout 60 $W conform-run --json --checked $p.lu 2>&1 < /dev/null | cut -c1-1500; echo "[exit ${PIPESTATUS[0]}]"
    echo "\$ wolf build --release $p.lu && ./$p"; timeout 300 $W build --release $p.lu 2>&1 < /dev/null; b=$?; echo "[build exit $b]"; if [ $b = 0 ]; then timeout 60 ./$p 2>&1 < /dev/null; echo "[exit $?]"; fi
    cd $L; rm -rf $T
  done
done
