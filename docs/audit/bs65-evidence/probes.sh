#!/bin/bash
# bs65 probes on a linux host, both pairs: (1) `use std.io` from the archive whole, from the bare
# wolf, and on lupin unpacked alone; (2) ex5-8 on every machine; (3) ex16-7 and ex16-8 on lupin,
# native, checked (unseeded and --seed=7) and release; (4) chapter 11's exercise programs on the
# checked machine (the EXERCISES.md header the release makes false).
set -u
L=$HOME/lanes/bs65; export TMPDIR=$L/tmp; [ -n "${EXTRA_PATH:-}" ] && export PATH=$EXTRA_PATH:$PATH
B=${B:-$L/wolf-book}; T=$(mktemp -d); R=$(mktemp -d)  # R: the release build alone (two .lu in one dir are one module, D32)
printf 'use std.io\n\nfn main() {\n    io.write_line("std answers")\n}\n' > $T/stdio.lu
for P in c26 new bare; do
  if [ $P = bare ]; then W=$L/bare/wolf; U=$(ls $L/new/lupin-*/lupin); else W=$(ls $L/$P/wolf-*/wolf); U=$(ls $L/$P/lupin-*/lupin); fi
  echo "########## $P: $($W --version | head -1) / $($U --version)"
  cd $T; echo '--- (1) stdio.lu'; cat stdio.lu
  echo '$ wolf run stdio.lu'; $W run stdio.lu 2>&1; echo "[exit $?]"
  echo '$ lupin stdio.lu'; $U stdio.lu 2>&1; echo "[exit $?]"
  cd $B/principles/exercises/ch05
  echo '--- (2) ex5-8'; echo '$ wolf conform-run --json ex5-8.lu'; $W conform-run --json ex5-8.lu 2>&1; echo '$ lupin ex5-8.lu'; $U ex5-8.lu 2>&1; echo "[exit $?]"
  cd $B/principles/exercises/ch16
  for x in ex16-7 ex16-8; do echo "--- (3) $x"
    echo "\$ lupin $x.lu"; $U $x.lu 2>&1; echo "[exit $?]"
    echo "\$ lupin run $x.lu --seed=7"; $U run $x.lu --seed=7 2>&1; echo "[exit $?]"
    echo "\$ wolf run $x.lu"; $W run $x.lu 2>&1; echo "[exit $?]"
    echo "\$ wolf conform-run --json $x.lu"; $W conform-run --json $x.lu 2>&1
    echo "\$ wolf conform-run --json --checked $x.lu"; $W conform-run --json --checked $x.lu 2>&1
    echo "\$ wolf conform-run --json --checked --seed=7 $x.lu"; $W conform-run --json --checked --seed=7 $x.lu 2>&1
    cp $x.lu $R/r.lu; echo "\$ wolf build --release $x.lu && ./$x"; (cd $R && $W build --release r.lu 2>&1 && ./r; echo "[exit $?]"; rm -f r r.lu)
  done
  cd $B/principles/exercises/ch11; echo '--- (4) chapter 11 on the checked machine'
  for f in *.lu; do printf '%s %s ' "$f" "$(grep -m1 '//! check:' $f)"; $W conform-run --json --checked $f 2>&1 | python3 -c 'import json,sys
for l in sys.stdin:
    if l.startswith("{"): r=json.loads(l); print(r.get("verdict"), r.get("x-unsupported-construct") or "", repr((r.get("stdout_inline") or "")[:60]))'; done
done
rm -rf $T $R
