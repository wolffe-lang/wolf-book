#!/bin/bash
# bs69 head gate, INSIDE the container (ctr.sh), on ~/lanes/bs69/wb-work (the lane's committed head,
# rsynced; gate-host.sh builds xtask from it): every gate the repo's CI runs, against the 0.2.32
# archive WHOLE (std/ beside wolf, as CI stages it), full 2>&1 logs, SKIPs counted after; the samples
# gate also under taskset -c 0-3, and once against a BARE wolf (no std/ beside it) for the record.
set -u
L=$HOME/lanes/bs69; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp
H=$(git rev-parse --short=7 HEAD); G=$L/logs/gate-$H; mkdir -p $G; rm -f $G/*; git rev-parse HEAD > $G/gate-head.txt
git status --porcelain > $G/gate-status-before.txt
X=$L/target-work/debug/xtask; U=$(ls $L/new/lupin-*/lupin); WD=$(ls -d $L/new/wolf-*/)
export CARGO_TARGET_DIR=$L/target-work CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8 WOLF=$WD/wolf LUPIN=$U WOLF_LANG_PATH=$L/wolf-lang-0233
for t in "samples" "samples --self-test" "verify-docs" "ledger --check" "backmatter --check" "contrast" "tiers --check" "render web"; do
  n=gate-$(echo $t | tr -d " -").log
  $X $t > $G/$n 2>&1; echo "EXIT $?" >> $G/$n
  git clean -qfdX
done
LUPIN_TRACE=$U $X diagrams --check > $G/gate-diagramscheck.log 2>&1; echo "EXIT $?" >> $G/gate-diagramscheck.log
taskset -c 0-3 $X samples > $G/gate-samples-taskset4.log 2>&1; echo "EXIT $?" >> $G/gate-samples-taskset4.log; git clean -qfdX
WOLF=$L/bare/wolf $X samples > $G/gate-samples-bare.log 2>&1; echo "EXIT $?" >> $G/gate-samples-bare.log; git clean -qfdX
cargo test --offline -p xtask -- --nocapture > $G/gate-xtasktest.log 2>&1; echo "EXIT $?" >> $G/gate-xtasktest.log
git status --porcelain > $G/gate-status.txt
{ for f in $G/*.log; do echo "$(basename $f): $(tail -n 1 $f) SKIP=$(grep -c 'SKIP' $f)"; done; } > $G/gate-summary.txt
sed -i "s#$L/wb-work/##g; s#$L/#~/lanes/bs69/#g" $G/*.log
echo DONE > $G/gate.done
