#!/bin/bash
# bs62 probe runner: each probe alone in its own dir, on lupin 0.1.46 and wolf 0.2.23
R=$HOME/lanes/bs62/rel
W=$R/wolf-0.2.23-x86_64-unknown-linux-gnu/wolf
L=$R/lupin-0.1.46-x86_64-unknown-linux-gnu/lupin
P="$(cd "$(dirname "$0")" && pwd)"
$W --version; $L --version
for f in "$P"/*.lu; do
  b=$(basename $f); d=$HOME/lanes/bs62/work/${b%.lu}; rm -rf $d; mkdir -p $d; cp $f $d/; cd $d
  echo "=== $b"
  echo "--- lupin"; timeout 30 $L $b 2>&1; echo "exit $?"
  echo "--- wolf run"; timeout 60 $W run $b > out.txt 2>&1; e=$?; grep -v '^ *$' out.txt | grep -E '^(error|warning)|^[^ |-]' | head -8; echo "exit $e"
  echo "--- conform-run --checked"; timeout 60 $W conform-run --json --checked ./$b 2>/dev/null | python3 -c 'import json,sys; d=json.loads(sys.stdin.read()); print(d.get("verdict"), d.get("phase_reached"), repr((d.get("stdout") or "")[:80]))'
done
