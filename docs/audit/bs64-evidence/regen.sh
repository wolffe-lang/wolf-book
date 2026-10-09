#!/bin/bash
# bs64: backmatter (solutions.md, appendices) against wolf-lang at v0.2.26, then verify-docs and tiers --check.
set -u
L=$HOME/lanes/bs64; cd $L/wolf-book || exit 1
git checkout -q -- . ; git fetch -q origin bs64 && git merge -q --ff-only origin/bs64 || { echo FETCH FAILED > $L/logs/regen.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs64 ] || { echo WRONG BRANCH > $L/logs/regen.done; exit 1; }
[ -d $L/wolf-lang-0226 ] || git clone -q --depth 1 --branch v0.2.26 https://github.com/wolffe-lang/wolf-lang.git $L/wolf-lang-0226
git -C $L/wolf-lang-0226 rev-parse HEAD > $L/logs/wolf-lang-0226-head.txt
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 WOLF=$(ls $L/new/wolf-*/wolf) LUPIN=$(ls $L/new/lupin-*/lupin) WOLF_LANG_PATH=$L/wolf-lang-0226
git rev-parse HEAD > $L/logs/regen-head.txt
cargo xtask backmatter > $L/logs/regen-backmatter.log 2>&1; echo "EXIT $?" >> $L/logs/regen-backmatter.log
cargo xtask verify-docs > $L/logs/regen-verifydocs.log 2>&1; echo "EXIT $?" >> $L/logs/regen-verifydocs.log
cargo xtask tiers --check > $L/logs/regen-tiers.log 2>&1; echo "EXIT $?" >> $L/logs/regen-tiers.log
cargo xtask ledger --check > $L/logs/regen-ledger.log 2>&1; echo "EXIT $?" >> $L/logs/regen-ledger.log
git status --porcelain > $L/logs/regen-status.txt; git diff > $L/logs/regen.patch
echo DONE > $L/logs/regen.done
