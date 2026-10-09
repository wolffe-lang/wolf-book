#!/bin/bash
# bs64: each probe on lupin, wolf run (native), conform-run --checked, and a --release build, with the pair given. Args: pairdir
P=$1; W=$(ls $P/wolf-*/wolf); U=$(ls $P/lupin-*/lupin); D=$HOME/lanes/bs64/probes
echo "## pair=$(basename $P) $($W --version | head -1) / $($U --version)"
for f in $D/*.lu; do b=$(basename $f .lu); T=$(mktemp -d); cp $f $T/$b.lu; cd $T
  echo "### $b.lu"
  echo "\$ lupin $b.lu"; timeout 60 $U $b.lu 2>&1; echo "[exit $?]"
  echo "\$ wolf run $b.lu"; timeout 120 $W run $b.lu 2>&1; echo "[exit $?]"
  echo "\$ wolf conform-run --json --checked $b.lu"; timeout 120 $W conform-run --json --checked $b.lu 2>&1 | head -c 1500; echo "[exit ${PIPESTATUS[0]}]"
  echo "\$ wolf build --release $b.lu && ./$b"; timeout 300 $W build --release $b.lu > rb.out 2>&1; rb=$?
  if [ $rb = 0 ]; then timeout 60 ./$b 2>&1; echo "[exit $?]"; else head -c 1500 rb.out; echo "[build exit $rb]"; fi
  cd /; rm -rf $T
done
