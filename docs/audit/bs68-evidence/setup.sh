#!/bin/bash
# bs68 setup on kasumi: fetch the CONTROL pair (the book's pin, 0.2.30 / 0.1.53) by digest, each read
# twice; clone branch bs68; build xtask on the host (no program runs here) and fetch its crates so
# the container needs no network.
set -u
L=$HOME/lanes/bs68; mkdir -p $L/logs $L/c30 $L/tmp; cd $L; . $L/ev/fetch.sh
[ -d wolf-book ] || git clone -q --branch bs68 https://github.com/wolffe-lang/wolf-book.git wolf-book
{ date -u +%FT%TZ; echo "host $(hostname)"
  fetch c30 wolf-lang v0.2.30 wolf-0.2.30-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c30 wolf-lang v0.2.30 wolf-0.2.30-x86_64-unknown-linux-gnu.tar.gz $L/c30/wolf-0.2.30-x86_64-unknown-linux-gnu
  fetch c30 wolf-interp v0.1.53 lupin-0.1.53-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c30 wolf-interp v0.1.53 lupin-0.1.53-x86_64-unknown-linux-gnu.tar.gz $L/c30/lupin-0.1.53-x86_64-unknown-linux-gnu
  for b in $L/c30/wolf-*/wolf $L/c30/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
cd $L/wolf-book && export CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
{ cargo fetch && cargo build -q -p xtask && cargo test -q -p xtask --no-run; echo "build exit $?"; } > $L/logs/build-xtask.log 2>&1
echo DONE > $L/logs/setup.done
