#!/bin/bash
# bs65 per-host setup (kasumi, hasu, nomad-1): fetch the CONTROL pair (the book's pin,
# 0.2.26 / 0.1.49) by digest for this host's triple; on linux also clone branch bs65.
set -u
L=$HOME/lanes/bs65; mkdir -p $L/logs $L/c26 $L/tmp; cd $L; . $L/ev/fetch.sh; T=$(triple)
[ "$(uname -s)" = Linux ] && { [ -d wolf-book ] || git clone -q --branch bs65 https://github.com/wolffe-lang/wolf-book.git wolf-book; }
{ date -u +%FT%TZ; echo "host $(hostname) triple $T"
  fetch c26 wolf-lang v0.2.26 wolf-0.2.26-$T.tar.gz
  fetch c26 wolf-interp v0.1.49 lupin-0.1.49-$T.tar.gz
  for b in $L/c26/wolf-*/wolf $L/c26/lupin-*/lupin; do echo "$b"; $b --version; done
} > $L/logs/setup.log 2>&1
echo DONE > $L/logs/setup.done
