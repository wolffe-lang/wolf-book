#!/bin/bash
# bs65: backmatter (solutions.md, appendices) against wolf-lang at v0.2.28, then verify-docs, tiers
# --check and ledger --check. Output: a patch and the logs; nothing is pushed from here.
set -u
L=$HOME/lanes/bs65; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp
git checkout -q -- . ; git fetch -q origin bs65 && git merge -q --ff-only origin/bs65 || { echo FETCH FAILED > $L/logs/regen.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs65 ] || { echo WRONG BRANCH > $L/logs/regen.done; exit 1; }
[ -d $L/wolf-lang-0228 ] || git clone -q --depth 1 --branch v0.2.28 https://github.com/wolffe-lang/wolf-lang.git $L/wolf-lang-0228
git -C $L/wolf-lang-0228 rev-parse HEAD > $L/logs/wolf-lang-0228-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 WOLF=$L/bare/wolf LUPIN=$(ls $L/new/lupin-*/lupin) WOLF_LANG_PATH=$L/wolf-lang-0228
git rev-parse HEAD > $L/logs/regen-head.txt
cargo xtask backmatter > $L/logs/regen-backmatter.log 2>&1; echo "EXIT $?" >> $L/logs/regen-backmatter.log
cargo xtask verify-docs > $L/logs/regen-verifydocs.log 2>&1; echo "EXIT $?" >> $L/logs/regen-verifydocs.log
cargo xtask tiers --check > $L/logs/regen-tiers.log 2>&1; echo "EXIT $?" >> $L/logs/regen-tiers.log
cargo xtask ledger --check > $L/logs/regen-ledger.log 2>&1; echo "EXIT $?" >> $L/logs/regen-ledger.log
git status --porcelain > $L/logs/regen-status.txt; git diff > $L/logs/regen.patch
echo DONE > $L/logs/regen.done
