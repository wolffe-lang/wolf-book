#!/bin/bash
# bs65 head gate on a linux host: every gate the repo's CI runs, against the BARE 0.2.28 wolf
# (CI's shape: no std/ beside it), full 2>&1 logs, SKIPs counted after; then the samples gate
# against the archive WHOLE (the reader's toolchain), and under taskset -c 0-3.
set -u
L=$HOME/lanes/bs65; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp
[ -n "${EXTRA_PATH:-}" ] && export PATH=$EXTRA_PATH:$PATH
git checkout -q -- . ; git fetch -q origin bs65 && git merge -q --ff-only origin/bs65 || { echo CHECKOUT FAILED > $L/logs/gate.done; exit 1; }
[ "$(git rev-parse --abbrev-ref HEAD)" = bs65 ] || { echo WRONG BRANCH > $L/logs/gate.done; exit 1; }
H=$(git rev-parse --short=7 HEAD); G=$L/logs/gate-$H; mkdir -p $G; rm -f $G/*; git rev-parse HEAD > $G/gate-head.txt
U=$(ls $L/new/lupin-*/lupin)
export CARGO_BUILD_JOBS=8 CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 WOLF=$L/bare/wolf LUPIN=$U WOLF_LANG_PATH=$L/wolf-lang-0228
[ -d $WOLF_LANG_PATH ] || git clone -q --depth 1 --branch v0.2.28 https://github.com/wolffe-lang/wolf-lang.git $WOLF_LANG_PATH
for t in "samples" "samples --self-test" "verify-docs" "ledger --check" "backmatter --check" "contrast" "tiers --check" "render web"; do
  n=gate-$(echo $t | tr -d " -").log
  cargo xtask $t > $G/$n 2>&1; echo "EXIT $?" >> $G/$n
done
LUPIN_TRACE=$U cargo xtask diagrams --check > $G/gate-diagramscheck.log 2>&1; echo "EXIT $?" >> $G/gate-diagramscheck.log
taskset -c 0-3 cargo xtask samples > $G/gate-samples-taskset4.log 2>&1; echo "EXIT $?" >> $G/gate-samples-taskset4.log
WOLF=$(ls $L/new/wolf-*/wolf) cargo xtask samples > $G/gate-samples-archivewhole.log 2>&1; echo "EXIT $?" >> $G/gate-samples-archivewhole.log
cargo test -p xtask -- --nocapture > $G/gate-xtasktest.log 2>&1; echo "EXIT $?" >> $G/gate-xtasktest.log
git status --porcelain > $G/gate-status.txt
{ for f in $G/*.log; do echo "$(basename $f): $(tail -n 1 $f) SKIP=$(grep -c 'SKIP' $f)"; done; } > $G/gate-summary.txt
echo DONE > $L/logs/gate.done
