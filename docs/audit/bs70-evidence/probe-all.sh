#!/bin/bash
# bs70 (from bs69), INSIDE the container (ctr.sh): the new pages' drafts on the four machines of the new
# pair (probes.sh over the draft dir), the lupin probes behind §34.4's sentences on BOTH pairs (the
# control's lupin 0.1.54 and the subject's 0.1.55, and the compiled machines of each), and #87's row as a
# shell reads it (shell/again.sh). Args: [draft dir, default docs/audit/bs70-evidence/draft of the checkout] [tag].
set -u
L=$HOME/lanes/bs70; D=${1:-$L/wolf-book/docs/audit/bs70-evidence/draft}; T=${2:-draft}; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
bash $L/ev/probes.sh $D > $L/logs/probes-$T.txt 2>&1
PAIR=new bash $L/ev/probes.sh $D/lupin > $L/logs/probes-$T-lupin-new.txt 2>&1
PAIR=c33 bash $L/ev/probes.sh $D/lupin > $L/logs/probes-$T-lupin-c33.txt 2>&1
for P in new c33; do X=$(mktemp -d -p $TMPDIR); cd $X
  W=$(ls $L/$P/wolf-*/wolf) bash $D/shell/again.sh $D/shell/again.lu > $L/logs/probes-$T-again-$P.txt 2>&1
  cd $L; rm -rf $X; done
echo DONE > $L/logs/probe-$T.done
