#!/bin/bash
# bs69, INSIDE the container (ctr.sh), on ~/lanes/bs69/wb-work (an rsync of the lane's working tree): the
# rig's own CI lane — fmt, clippy -D warnings, the xtask tests with --nocapture — full logs. Arg: tag.
set -u
L=$HOME/lanes/bs69; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp CARGO_TARGET_DIR=$L/target-work CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
T=${1:-work}; G=$L/logs/rig-$T; mkdir -p $G
cargo fmt --check > $G/fmt.log 2>&1; echo "EXIT $?" >> $G/fmt.log
cargo clippy --offline --all-targets -- -D warnings > $G/clippy.log 2>&1; echo "EXIT $?" >> $G/clippy.log
cargo test --offline -p xtask -- --nocapture > $G/xtasktest.log 2>&1; echo "EXIT $?" >> $G/xtasktest.log
for f in $G/*.log; do echo "$(basename $f): $(tail -n 1 $f)"; done > $G/summary.txt
echo DONE > $G/rig.done
