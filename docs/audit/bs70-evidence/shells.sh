#!/bin/bash
# bs70, on the HOST: the #87 row's binary (built inside the arch container by again.sh) run under dash in
# an ubuntu:24.04 container (dash is its /bin/sh), its own pid namespace, --network none, RLIMIT_CORE 0.
set -u
L=$HOME/lanes/bs70; B=$1
exec podman run --rm --network none --ulimit core=0:0 -v $B:/work:ro -w /work docker.io/library/ubuntu:24.04 \
  /bin/bash -c 'ulimit -c 0; for sh in "dash -c" "bash -c" "dash -ic"; do out=$($sh "./again again; echo status \$?" 2> /tmp/err.txt < /dev/null); echo "## $sh: stdout [$out] stderr [$(cat /tmp/err.txt)]"; done'
