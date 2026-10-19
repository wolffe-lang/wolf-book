#!/usr/bin/env bash
# bs69 (hasu, host half): move the checkout to the head named and build xtask from it. No program runs.
set -u
L=$HOME/lanes/bs69; cd $L/wolf-book || exit 1; c=$1
git fetch -q origin bs69 && git checkout -q -- . && git merge -q --ff-only origin/bs69 && [ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] && [ "$(git rev-parse HEAD)" = "$c" ] || { echo "SYNC FAILED $(git rev-parse HEAD)"; exit 1; }
export CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
cargo build -q -p xtask && echo "xtask built at $(git rev-parse --short=7 HEAD)"
