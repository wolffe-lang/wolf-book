#!/bin/bash
# bs70, INSIDE the container (ctr.sh): #87's row on both pairs; the new pair's binary kept in logs/again-bin
# for shells.sh.
set -u
L=$HOME/lanes/bs70; D=$L/wolf-book/docs/audit/bs70-evidence/draft/shell; export TMPDIR=$L/tmp
for P in new c33; do X=$(mktemp -d -p $TMPDIR); cd $X
  W=$(ls $L/$P/wolf-*/wolf) bash $D/again.sh $D/again.lu > $L/logs/probes-again-$P.txt 2>&1
  [ $P = new ] && { rm -rf $L/logs/again-bin; mkdir -p $L/logs/again-bin; cp again $L/logs/again-bin/; }
  cd $L; rm -rf $X; done
echo DONE > $L/logs/again-probe.done
