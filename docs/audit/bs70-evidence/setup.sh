#!/bin/bash
# bs70 setup on kasumi (from bs69): fetch the CONTROL pair (the book's pin, wolf 0.2.33 / lupin 0.1.54) by
# digest, each read twice; clone branch bs70; build xtask on the host (no program runs here) and fetch its
# crates so the container needs no network. The 0.2.34 / 0.1.55 subject is NOT fetched here: the
# prediction is committed first.
set -u
L=$HOME/lanes/bs70; mkdir -p $L/logs $L/c33 $L/tmp; cd $L; . $L/ev/fetch.sh
[ -d wolf-book ] || git clone -q --branch bs70 https://github.com/wolffe-lang/wolf-book.git wolf-book
{ date -u +%FT%TZ; echo "host $(hostname)"
  fetch c33 wolf-lang v0.2.33 wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c33 wolf-lang v0.2.33 wolf-0.2.33-x86_64-unknown-linux-gnu.tar.gz $L/c33/wolf-0.2.33-x86_64-unknown-linux-gnu
  fetch c33 wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c33 wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz $L/c33/lupin-0.1.54-x86_64-unknown-linux-gnu
  for b in $L/c33/wolf-*/wolf $L/c33/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
cd $L/wolf-book && [ "$(git rev-parse --abbrev-ref HEAD)" = bs70 ] || { echo WRONG BRANCH > $L/logs/setup.done; exit 1; }
export CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
{ cargo fetch && cargo build -q -p xtask && cargo test -q -p xtask --no-run; echo "build exit $?"; } > $L/logs/build-xtask.log 2>&1
echo DONE > $L/logs/setup.done
