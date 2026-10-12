#!/bin/bash
# bs65 subject on nomad-1 (macOS arm64): the new pair by digest, members by name, the sweep twice
# (no cargo build here), the comparison against c26.
set -u
L=$HOME/lanes/bs65; mkdir -p $L/new; cd $L; . $L/ev/fetch.sh; export TMPDIR=$L/tmp; T=$(triple)
{ date -u +%FT%TZ
  fetch new wolf-lang v0.2.28 wolf-0.2.28-$T.tar.gz
  fetch new wolf-interp v0.1.51 lupin-0.1.51-$T.tar.gz
} > $L/logs/subject-download.log 2>&1
grep -q MISMATCH $L/logs/subject-download.log && { echo MISMATCH > $L/logs/subject.done; exit 1; }
W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
{ for x in $L/new/wolf-*/ $L/new/lupin-*/; do echo "== $(basename $x)"; (cd $x && command ls -1 | while read f; do [ -f "$f" ] && shasum -a 256 "$f"; done)
    [ -d $x/std ] && (cd $x && echo "std/: $(find std -type f | wc -l | tr -d ' ') files, $(find std -name '*.lu' | wc -l | tr -d ' ') .lu, STD-REV $(cat std/STD-REV), manifest $(find std -type f | LC_ALL=C sort | xargs shasum -a 256 | shasum -a 256 | cut -c1-16)…"); done
  echo "== versions"; $W --version; $U --version
} > $L/logs/subject-members.txt 2>&1
bash $L/ev/sweep.sh new 4; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newa.jsonl
bash $L/ev/sweep.sh new 4; mv $L/logs/sweep-new.jsonl $L/logs/sweep-newb.jsonl
python3 $L/ev/cmp.py $L/logs/sweep-newa.jsonl $L/logs/sweep-newb.jsonl > $L/logs/sweep-new-noise.txt 2>&1
python3 $L/ev/cmp.py $L/logs/sweep-c26a.jsonl $L/logs/sweep-newa.jsonl > $L/logs/sweep-c26-vs-new.txt 2>&1
echo DONE > $L/logs/subject.done
