#!/bin/bash
# bs67 setup on kasumi: fetch the CONTROL pair (the book's pin, 0.2.29 / 0.1.52) and wolf 0.2.15 (the
# toolchain §13.1's table names) by digest, each read twice; clone branch bs67; build xtask on the
# host (no program runs here) and fetch its crates so the container needs no network.
set -u
L=$HOME/lanes/bs67; mkdir -p $L/logs $L/c29 $L/c15 $L/tmp; cd $L; . $L/ev/fetch.sh
[ -d wolf-book ] || git clone -q --branch bs67 https://github.com/wolffe-lang/wolf-book.git wolf-book
{ date -u +%FT%TZ; echo "host $(hostname)"
  fetch c29 wolf-lang v0.2.29 wolf-0.2.29-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c29 wolf-lang v0.2.29 wolf-0.2.29-x86_64-unknown-linux-gnu.tar.gz $L/c29/wolf-0.2.29-x86_64-unknown-linux-gnu
  fetch c29 wolf-interp v0.1.52 lupin-0.1.52-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c29 wolf-interp v0.1.52 lupin-0.1.52-x86_64-unknown-linux-gnu.tar.gz $L/c29/lupin-0.1.52-x86_64-unknown-linux-gnu
  fetch c15 wolf-lang v0.2.15 wolf-0.2.15-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c15 wolf-lang v0.2.15 wolf-0.2.15-x86_64-unknown-linux-gnu.tar.gz $L/c15/wolf-0.2.15-x86_64-unknown-linux-gnu
  for b in $L/c29/wolf-*/wolf $L/c29/lupin-*/lupin $L/c15/wolf-*/wolf; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
cd $L/wolf-book && export CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
{ cargo fetch && cargo build -q -p xtask && cargo test -q -p xtask --no-run; echo "build exit $?"; } > $L/logs/build-xtask.log 2>&1
echo DONE > $L/logs/setup.done
