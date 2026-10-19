#!/bin/bash
# bs69 working-tree check, INSIDE the container (ctr.sh), on ~/lanes/bs69/wb-work (an rsync of the
# lane's working tree, uncommitted edits included): the xtask binary built from that tree on the host
# (work-host.sh), the archive whole of the new pair. Args: gate names (default: the set below).
set -u
L=$HOME/lanes/bs69; cd $L/wb-work || exit 1; export TMPDIR=$L/tmp
X=$L/target-work/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
export WOLF=$W LUPIN=$U LUPIN_TRACE=$U WOLF_LANG_PATH=$L/wolf-lang-0233
G=${*:-"tiers backmatter verify-docs ledger--check tiers--check samples"}
for t in $G; do a=$(echo $t | sed 's/--/ --/'); n=$L/logs/work-$(echo $t | tr -d -).log
  echo "##### xtask $a"; $X $a > $n 2>&1; e=$?; echo "EXIT $e" >> $n; tail -${TAIL:-40} $n; done
git clean -qfdX
