#!/bin/bash
# bs64 head gate on kasumi: every gate the repo's CI runs, full 2>&1 logs, SKIPs counted after.
set -u
L=$HOME/lanes/bs64; cd $L/wolf-book || exit 1
git checkout -q -- . ; git fetch -q origin bs64 && git merge -q --ff-only origin/bs64 || { echo CHECKOUT FAILED > $L/logs/gate.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs64 ] || { echo WRONG BRANCH > $L/logs/gate.done; exit 1; }
git rev-parse HEAD > $L/logs/gate-head.txt
mkdir -p $L/logs/gate; rm -f $L/logs/gate/*
W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 WOLF=$W LUPIN=$U WOLF_LANG_PATH=$L/wolf-lang-0226
for t in "samples" "samples --self-test" "verify-docs" "ledger --check" "backmatter --check" "contrast" "tiers --check" "render web"; do
  n=gate-$(echo $t | tr -d " -").log
  cargo xtask $t > $L/logs/gate/$n 2>&1; echo "EXIT $?" >> $L/logs/gate/$n
done
LUPIN_TRACE=$U cargo xtask diagrams --check > $L/logs/gate/gate-diagramscheck.log 2>&1; echo "EXIT $?" >> $L/logs/gate/gate-diagramscheck.log
taskset -c 0-3 cargo xtask samples > $L/logs/gate/gate-samples-taskset4.log 2>&1; echo "EXIT $?" >> $L/logs/gate/gate-samples-taskset4.log
cargo test -p xtask -- --nocapture > $L/logs/gate/gate-xtasktest.log 2>&1; echo "EXIT $?" >> $L/logs/gate/gate-xtasktest.log
git status --porcelain > $L/logs/gate/gate-status.txt
echo DONE > $L/logs/gate.done
