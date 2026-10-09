#!/bin/bash
# bs64 subject: download wolf 0.2.26 / lupin 0.1.49 by digest, hash members by name, then run the
# unedited prediction head (6d90749) on them: samples, diagrams --check with LUPIN_TRACE, ledger,
# verify-docs; and the 577-program sweep twice.
set -u
L=$HOME/lanes/bs64; mkdir -p $L/new; cd $L; . $L/fetch.sh
{ date -u +%FT%TZ
  fetch new wolf-lang v0.2.26 wolf-0.2.26-x86_64-unknown-linux-gnu.tar.gz
  fetch new wolf-interp v0.1.49 lupin-0.1.49-x86_64-unknown-linux-gnu.tar.gz
} > $L/logs/subject-download.log 2>&1
grep -q MISMATCH $L/logs/subject-download.log && { echo MISMATCH > $L/logs/subject.done; exit 1; }
W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
{ for x in $L/new/wolf-*/ $L/new/lupin-*/; do echo "== $(basename $x)"; (cd $x && command ls -1 | while read f; do sha256sum "$f"; done); done
  echo "== versions"; $W --version; $U --version
  for b in $W $U; do echo "$(basename $b) highest $(objdump -T $b | grep -oE 'GLIBC_[0-9.]+' | sort -uV | tail -1)"; done
} > $L/logs/subject-members.txt 2>&1
cd $L/wolf-book || exit 1
git fetch -q origin bs64 && git merge -q --ff-only origin/bs64 || { echo FETCH FAILED > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs64 ] || { echo WRONG BRANCH > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --short=7 HEAD)" = "6d90749" ] || { echo WRONG HEAD > $L/logs/subject.done; exit 1; }
git rev-parse HEAD > $L/logs/subject-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
for t in "samples" "verify-docs" "ledger --check"; do n=subject-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U cargo xtask $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n; done
WOLF=$W LUPIN=$U LUPIN_TRACE=$U cargo xtask diagrams --check > $L/logs/subject-diagramscheck.log 2>&1; echo "EXIT $?" >> $L/logs/subject-diagramscheck.log
git status --porcelain > $L/logs/subject-status.txt
git checkout -q -- .
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newa.jsonl
bash $L/ev/sweep.sh new; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newb.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-newa.jsonl $L/logs/sweep-newb.jsonl > $L/logs/sweep-new-noise.txt 2>&1
python3 $L/ev/cmp.py $L/logs/sweep-c25a.jsonl $L/logs/sweep-newa.jsonl > $L/logs/sweep-c25-vs-new.txt 2>&1
echo DONE > $L/logs/subject.done
