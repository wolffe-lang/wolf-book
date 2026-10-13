#!/bin/bash
# bs66 per-host setup (kasumi, hasu, nomad-1): fetch the CONTROL pair (the book's pin,
# 0.2.28 / 0.1.51) by digest for this host's triple; on linux also clone branch bs66.
set -u
L=$HOME/lanes/bs66; mkdir -p $L/logs $L/c28 $L/tmp; cd $L; . $L/ev/fetch.sh; T=$(triple)
[ "$(uname -s)" = Linux ] && { [ -d wolf-book ] || git clone -q --branch bs66 https://github.com/wolffe-lang/wolf-book.git wolf-book; }
{ date -u +%FT%TZ; echo "host $(hostname) triple $T"
  fetch c28 wolf-lang v0.2.28 wolf-0.2.28-$T.tar.gz
  fetch c28 wolf-interp v0.1.51 lupin-0.1.51-$T.tar.gz
  for b in $L/c28/wolf-*/wolf $L/c28/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
echo DONE > $L/logs/setup.done
