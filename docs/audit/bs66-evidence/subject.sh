#!/bin/bash
# bs66 subject on a linux host: download wolf 0.2.29 / lupin 0.1.52 by digest, hash members by
# name (std/ as a file count and a sorted-manifest digest), make the BARE wolf (the binary and its
# runtime beside it, no std/: what the book's CI builds from source), then run the unedited
# prediction head on: samples (archive whole, and bare), verify-docs, ledger --check,
# diagrams --check with LUPIN_TRACE; the 580-program sweep twice; the comparison against c28.
# EXTRA_PATH (hasu): an lld already in the nix store, since hasu has none on PATH.
set -u
L=$HOME/lanes/bs66; mkdir -p $L/new $L/bare; cd $L; . $L/ev/fetch.sh; export TMPDIR=$L/tmp
[ -n "${EXTRA_PATH:-}" ] && export PATH=$EXTRA_PATH:$PATH
{ date -u +%FT%TZ; echo "host $(hostname)"; command -v ld.lld && ld.lld --version
  fetch new wolf-lang v0.2.29 wolf-0.2.29-x86_64-unknown-linux-gnu.tar.gz
  fetch new wolf-interp v0.1.52 lupin-0.1.52-x86_64-unknown-linux-gnu.tar.gz
} > $L/logs/subject-download.log 2>&1
grep -q MISMATCH $L/logs/subject-download.log && { echo MISMATCH > $L/logs/subject.done; exit 1; }
WD=$(ls -d $L/new/wolf-*/); W=$WD/wolf; U=$(ls $L/new/lupin-*/lupin)
for f in wolf wolf-cimport-worker libwolf_rt.a libwolf_rt_none.a; do cp $WD/$f $L/bare/; done; WB=$L/bare/wolf
{ for x in $L/new/wolf-*/ $L/new/lupin-*/; do echo "== $(basename $x)"; (cd $x && command ls -1 | while read f; do [ -f "$f" ] && sha256sum "$f"; done)
    [ -d $x/std ] && (cd $x && echo "std/: $(find std -type f | wc -l) files, $(find std -name '*.lu' | wc -l) .lu, STD-REV $(cat std/STD-REV), manifest $(find std -type f | LC_ALL=C sort | xargs sha256sum | sha256sum | cut -c1-64)"); done
  echo "== versions"; $W --version; $U --version; echo "== bare"; $WB --version
  for b in $W $U; do echo "$(basename $b) highest $(objdump -T $b | grep -oE 'GLIBC_[0-9.]+' | sort -uV | tail -1)"; done
} > $L/logs/subject-members.txt 2>&1
cd $L/wolf-book || exit 1
git fetch -q origin bs66 && git checkout -q -- . && git merge -q --ff-only origin/bs66 || { echo FETCH FAILED > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs66 ] || { echo WRONG BRANCH > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --short=7 HEAD)" = "${SUBJECT_HEAD:-38b91a3}" ] || { echo WRONG HEAD > $L/logs/subject.done; exit 1; }
git rev-parse HEAD > $L/logs/subject-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
if [ -n "${EXTRA_PATH:-}" ]; then WOLF=$(ls $L/c28/wolf-*/wolf) LUPIN=$(ls $L/c28/lupin-*/lupin) cargo xtask samples > $L/logs/control-c28-samples-lld.log 2>&1; echo "EXIT $?" >> $L/logs/control-c28-samples-lld.log; fi
for t in "samples" "verify-docs" "ledger --check"; do n=subject-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U cargo xtask $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n; done
WOLF=$WB LUPIN=$U cargo xtask samples > $L/logs/subject-samples-bare.log 2>&1; echo "EXIT $?" >> $L/logs/subject-samples-bare.log
WOLF=$W LUPIN=$U LUPIN_TRACE=$U cargo xtask diagrams --check > $L/logs/subject-diagramscheck.log 2>&1; echo "EXIT $?" >> $L/logs/subject-diagramscheck.log
git status --porcelain > $L/logs/subject-status.txt
git checkout -q -- .
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newa.jsonl
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newb.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-newa.jsonl $L/logs/sweep-newb.jsonl > $L/logs/sweep-new-noise.txt 2>&1
python3 $L/ev/cmp.py $L/logs/sweep-c28a.jsonl $L/logs/sweep-newa.jsonl > $L/logs/sweep-c28-vs-new.txt 2>&1
echo DONE > $L/logs/subject.done
