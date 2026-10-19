#!/usr/bin/env bash
# bs69 head gate on hasu, inside ns.sh (its own pid namespace, RLIMIT_CORE 0, an lld from the nix store on
# PATH): the checkout at the head named (SUBJECT_HEAD), the 0.2.33 archive whole: samples (and under
# taskset -c 0-3), verify-docs, ledger --check, diagrams --check; full 2>&1 logs, SKIPs counted after.
set -u
ulimit -c 0
L=$HOME/lanes/bs69; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp
H=$(git rev-parse --short=7 HEAD); [ "$H" = "${SUBJECT_HEAD:-UNSET}" ] || { echo "WRONG HEAD $H" > $L/logs/gate.done; exit 1; }
G=$L/logs/gate-$H; mkdir -p $G; rm -f $G/*
{ echo "host $(hostname) core $(ulimit -c)/$(ulimit -Hc)"; grep -E '^Sig(Blk|Ign)' /proc/self/status; git rev-parse HEAD; } > $G/gate-head.txt
X=$L/target/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check"; do n=gate-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U $X $t > $G/$n 2>&1; echo "EXIT $?" >> $G/$n; git clean -qfdX; done
taskset -c 0-3 env WOLF=$W LUPIN=$U $X samples > $G/gate-samples-taskset4.log 2>&1; echo "EXIT $?" >> $G/gate-samples-taskset4.log; git clean -qfdX
WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X diagrams --check > $G/gate-diagramscheck.log 2>&1; echo "EXIT $?" >> $G/gate-diagramscheck.log
git status --porcelain > $G/gate-status.txt
{ for f in $G/*.log; do echo "$(basename $f): $(tail -n 1 $f) SKIP=$(grep -c 'SKIP' $f)"; done; } > $G/gate-summary.txt
echo DONE > $G/gate.done; echo DONE > $L/logs/gate.done
