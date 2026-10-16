#!/bin/bash
# bs67 subject, the HOST half (nothing executes a program here but `--version`): download wolf 0.2.30
# / lupin 0.1.53 by digest and read each a second time; hash every member by name (std/ as a file
# count and a sorted-manifest digest, the way the book's CI computes it); make the BARE wolf (the
# binary and its runtime beside it, no std/); update the checkout to the prediction head.
set -u
L=$HOME/lanes/bs67; mkdir -p $L/new $L/bare; cd $L; . $L/ev/fetch.sh
{ date -u +%FT%TZ; echo "host $(hostname)"
  fetch new wolf-lang v0.2.30 wolf-0.2.30-x86_64-unknown-linux-gnu.tar.gz
  fetch new wolf-interp v0.1.53 lupin-0.1.53-x86_64-unknown-linux-gnu.tar.gz
} > $L/logs/subject-download.log 2>&1
{ date -u +%FT%TZ
  fetch2 new wolf-lang v0.2.30 wolf-0.2.30-x86_64-unknown-linux-gnu.tar.gz $L/new/wolf-0.2.30-x86_64-unknown-linux-gnu
  fetch2 new wolf-interp v0.1.53 lupin-0.1.53-x86_64-unknown-linux-gnu.tar.gz $L/new/lupin-0.1.53-x86_64-unknown-linux-gnu
} > $L/logs/subject-download-twice.log 2>&1
grep -q 'MISMATCH\|DIFFERS\|FAILED' $L/logs/subject-download*.log && { echo MISMATCH > $L/logs/subject-host.done; exit 1; }
WD=$(ls -d $L/new/wolf-*/); W=$WD/wolf; U=$(ls $L/new/lupin-*/lupin)
for f in wolf wolf-cimport-worker libwolf_rt.a libwolf_rt_none.a; do cp $WD/$f $L/bare/; done
man() { (cd $1 && find std -type f | LC_ALL=C sort | while IFS= read -r f; do printf '%s  %s\n' "$(sha256sum "$f" | cut -c1-64)" "$f"; done | sha256sum | cut -c1-64); }
{ for x in $L/new/wolf-*/ $L/new/lupin-*/ $L/c29/wolf-*/; do echo "== $(basename $x)"; (cd $x && command ls -1 | while read f; do [ -f "$f" ] && sha256sum "$f"; done)
    [ -d $x/std ] && echo "std/: $(cd $x && find std -type f | wc -l) files, $(cd $x && find std -name '*.lu' | wc -l) .lu, STD-REV $(cat $x/std/STD-REV), manifest $(man $x)"; done
  echo "== std/ files that differ, 0.2.29 -> 0.2.30"; diff -rq $L/c29/wolf-*/std $WD/std
  echo "== versions"; $W --version; $U --version; echo "== bare"; $L/bare/wolf --version
  for b in $W $U; do echo "$(basename $b) highest $(objdump -T $b | grep -oE 'GLIBC_[0-9.]+' | sort -uV | tail -1)"; done
} > $L/logs/subject-members.txt 2>&1
cd $L/wolf-book && git fetch -q origin bs67 && git checkout -q -- . && git merge -q --ff-only origin/bs67 || { echo FETCH FAILED > $L/logs/subject-host.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs67 ] || { echo WRONG BRANCH > $L/logs/subject-host.done; exit 1; }
echo DONE > $L/logs/subject-host.done
