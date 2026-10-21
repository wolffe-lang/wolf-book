#!/bin/bash
# bs70 (kasumi): make ~/lanes/bs70/wb-work a real clone (a worktree's .git is a file that points at
# nomad-1), at the commit named, then build xtask from it on the host (no program runs here).
# Args: commit (must be on origin/bs70, or present in ~/lanes/bs70/wolf-book).
set -u
L=$HOME/lanes/bs70; c=$1
[ -d $L/wb-work/.git ] && [ -d $L/wb-work/.git/objects ] || { rm -rf $L/wb-work; git clone -q $L/wolf-book $L/wb-work; }
cd $L/wb-work && git fetch -q https://github.com/wolffe-lang/wolf-book.git bs70 && git checkout -q -f "$c" && git clean -qfdx -e target && git rev-parse HEAD
export CARGO_TARGET_DIR=$L/target-work CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
cargo build -q -p xtask && echo "xtask built $(ls -la $CARGO_TARGET_DIR/debug/xtask | awk '{print $6,$7,$8}')"
