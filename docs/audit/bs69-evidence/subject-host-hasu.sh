#!/usr/bin/env bash
# bs69 subject, the HOST half (nothing executes a program here but `--version`; hasu: curl, no gh): download wolf
# 0.2.33 by digest and read it a second time; lupin 0.1.54 is the pinned archive, fetched again into new/ by
# digest; hash every member by name (std/ as a file count and a sorted-manifest digest, the way the book's
# CI computes it); make the BARE wolf (the binary and its runtime beside it, no std/); update the checkout
# to the prediction head.
set -u
L=$HOME/lanes/bs69; mkdir -p $L/new $L/bare; cd $L; . $L/ev/fetch-curl.sh
{ date -u +%FT%TZ; echo "host $(hostname)"
  fetch new wolf-lang v0.2.33 wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz
  fetch new wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz
} > $L/logs/subject-download.log 2>&1
{ date -u +%FT%TZ
  fetch2 new wolf-lang v0.2.33 wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz $L/new/wolf-0.2.33-x86_64-unknown-linux-gnu
  fetch2 new wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz $L/new/lupin-0.1.54-x86_64-unknown-linux-gnu
} > $L/logs/subject-download-twice.log 2>&1
grep -q 'MISMATCH\|DIFFERS\|FAILED' $L/logs/subject-download*.log && { echo MISMATCH > $L/logs/subject-host.done; exit 1; }
WD=$(ls -d $L/new/wolf-*/); W=$WD/wolf; U=$(ls $L/new/lupin-*/lupin)
for f in wolf wolf-cimport-worker libwolf_rt.a libwolf_rt_none.a; do cp $WD/$f $L/bare/; done
man() { (cd $1 && find std -type f | LC_ALL=C sort | while IFS= read -r f; do printf '%s  %s\n' "$(sha256sum "$f" | cut -c1-64)" "$f"; done | sha256sum | cut -c1-64); }
{ for x in $L/new/wolf-*/ $L/new/lupin-*/ $L/c32/wolf-*/; do echo "== $(basename $x)"; (cd $x && command ls -1 | while read f; do [ -f "$f" ] && sha256sum "$f"; done)
    [ -d $x/std ] && echo "std/: $(cd $x && find std -type f | wc -l) files, $(cd $x && find std -name '*.lu' | wc -l) .lu, STD-REV $(cat $x/std/STD-REV), manifest $(man $x)"; done
  echo "== std/ files that differ, 0.2.32 -> 0.2.33"; diff -rq $L/c32/wolf-*/std $WD/std
  echo "== members that differ, 0.2.32 -> 0.2.33 (top level)"; for f in $(cd $WD && command ls -1); do [ -f $WD/$f ] || continue; cmp -s $WD/$f $L/c32/wolf-0.2.32-*/$f && echo "same  $f" || echo "moved $f"; done
  echo "== lupin new/ vs c32/"; diff -r $L/c32/lupin-0.1.54-* $L/new/lupin-0.1.54-* && echo "lupin trees IDENTICAL"
  echo "== versions"; $W --version; $U --version; echo "== bare"; $L/bare/wolf --version
} > $L/logs/subject-members.txt 2>&1
cd $L/wolf-book && git fetch -q origin bs69 && git checkout -q -- . && git merge -q --ff-only origin/bs69 || { echo FETCH FAILED > $L/logs/subject-host.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] || { echo WRONG BRANCH > $L/logs/subject-host.done; exit 1; }
echo DONE > $L/logs/subject-host.done
