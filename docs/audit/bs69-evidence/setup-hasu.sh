#!/usr/bin/env bash
# bs69 setup on hasu (NixOS, no container runtime): the CONTROL pair (wolf 0.2.32 / lupin 0.1.54) by digest,
# each read twice (fetch-curl.sh); clone branch bs69; build xtask. Nothing executes a program here but
# `--version`.
set -u
L=$HOME/lanes/bs69; mkdir -p $L/logs $L/c32 $L/tmp; cd $L; . $L/ev/fetch-curl.sh
[ -d wolf-book ] || git clone -q --branch bs69 https://github.com/wolffe-lang/wolf-book.git wolf-book
{ date -u +%FT%TZ; echo "host $(hostname) $(uname -sm)"
  fetch c32 wolf-lang v0.2.32 wolf-0.2.32-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c32 wolf-lang v0.2.32 wolf-0.2.32-x86_64-unknown-linux-gnu.tar.gz $L/c32/wolf-0.2.32-x86_64-unknown-linux-gnu
  fetch c32 wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz
  fetch2 c32 wolf-interp v0.1.54 lupin-0.1.54-x86_64-unknown-linux-gnu.tar.gz $L/c32/lupin-0.1.54-x86_64-unknown-linux-gnu
  for b in $L/c32/wolf-*/wolf $L/c32/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
cd $L/wolf-book && [ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] || { echo WRONG BRANCH > $L/logs/setup.done; exit 1; }
export CARGO_TARGET_DIR=$L/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=8
{ cargo --version; rustc --version; cargo build -q -p xtask && cargo test -q -p xtask --no-run; echo "build exit $?"; } > $L/logs/build-xtask.log 2>&1
echo DONE > $L/logs/setup.done
