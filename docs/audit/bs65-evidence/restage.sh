#!/bin/bash
# bs65 after the ch22 ruling: re-fetch the 0.2.28 / 0.1.51 pair by digest (pruned after the first
# gate), clone branch bs65, and run the samples gate against the ARCHIVE WHOLE so ch22's block can be
# spliced from the reader's toolchain. EXTRA_PATH (hasu): the nix-store lld.
set -u
L=$HOME/lanes/bs65; mkdir -p $L/new $L/tmp $L/logs; cd $L; . $L/ev/fetch.sh; export TMPDIR=$L/tmp
[ -n "${EXTRA_PATH:-}" ] && export PATH=$EXTRA_PATH:$PATH
{ date -u +%FT%TZ
  fetch new wolf-lang v0.2.28 wolf-0.2.28-x86_64-unknown-linux-gnu.tar.gz
  fetch new wolf-interp v0.1.51 lupin-0.1.51-x86_64-unknown-linux-gnu.tar.gz
} > $L/logs/restage-download.log 2>&1
grep -q MISMATCH $L/logs/restage-download.log && { echo MISMATCH > $L/logs/restage.done; exit 1; }
[ -d wolf-book ] || git clone -q --branch bs65 https://github.com/wolffe-lang/wolf-book.git wolf-book
cd wolf-book; git fetch -q origin bs65 && git checkout -q -- . && git merge -q --ff-only origin/bs65 || { echo FETCH FAILED > $L/logs/restage.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs65 ] || { echo WRONG BRANCH > $L/logs/restage.done; exit 1; }
git rev-parse HEAD > $L/logs/restage-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0
WOLF=$(ls $L/new/wolf-*/wolf) LUPIN=$(ls $L/new/lupin-*/lupin) cargo xtask samples > $L/logs/restage-samples-archivewhole.log 2>&1; echo "EXIT $?" >> $L/logs/restage-samples-archivewhole.log
echo DONE > $L/logs/restage.done
