#!/bin/bash
# bs67: the host half of the head gate: build the xtask binary (and its tests) from wb-work, no
# program run. Then the container half (ctr.sh gate.sh).
set -u
L=$HOME/lanes/bs67; cd $L/wb-work || exit 1
export CARGO_TARGET_DIR=$L/target-work CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
{ cargo build -q -p xtask && cargo test -q -p xtask --no-run; echo "build exit $?"; } > $L/logs/gate-build.log 2>&1
