#!/bin/bash
# bs64 kasumi setup: clone branch bs64, fetch the CONTROL pair (the book's pin, 0.2.25 / 0.1.48) by digest.
set -u
L=$HOME/lanes/bs64; mkdir -p $L/logs $L/c25; cd $L; . $L/fetch.sh
[ -d wolf-book ] || git clone -q --branch bs64 https://github.com/wolffe-lang/wolf-book.git wolf-book
{ date -u +%FT%TZ
  fetch c25 wolf-lang v0.2.25 wolf-0.2.25-x86_64-unknown-linux-gnu.tar.gz
  fetch c25 wolf-interp v0.1.48 lupin-0.1.48-x86_64-unknown-linux-gnu.tar.gz
  for b in $L/c25/wolf-*/wolf $L/c25/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
echo DONE > $L/logs/setup.done
