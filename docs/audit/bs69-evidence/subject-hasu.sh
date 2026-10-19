#!/usr/bin/env bash
# bs69 subject on hasu, inside ns.sh (its own pid namespace), RLIMIT_CORE 0: the unedited prediction head on
# the new pair: samples (archive whole, and under taskset -c 0-3), verify-docs, ledger --check,
# diagrams --check. No sweep here (kasumi's, in the container).
set -u
ulimit -c 0
L=$HOME/lanes/bs69; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp
[ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] || { echo WRONG BRANCH > $L/logs/subject.done; exit 1; }
[ "$(git rev-parse --short=7 HEAD)" = "${SUBJECT_HEAD:-PREDICTION_HEAD_UNSET}" ] || { echo WRONG HEAD > $L/logs/subject.done; exit 1; }
{ echo "host $(hostname) core $(ulimit -c)/$(ulimit -Hc)"; grep -E '^Sig(Blk|Ign)' /proc/self/status; git rev-parse HEAD; } > $L/logs/subject-head.txt
X=$L/target/debug/xtask; W=$(ls $L/new/wolf-*/wolf); U=$(ls $L/new/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check"; do n=subject-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U $X $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n; git clean -qfdX; done
taskset -c 0-3 env WOLF=$W LUPIN=$U $X samples > $L/logs/subject-samples-taskset4.log 2>&1; echo "EXIT $?" >> $L/logs/subject-samples-taskset4.log; git clean -qfdX
WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X diagrams --check > $L/logs/subject-diagramscheck.log 2>&1; echo "EXIT $?" >> $L/logs/subject-diagramscheck.log
git status --porcelain > $L/logs/subject-status.txt
git checkout -q -- .
echo DONE > $L/logs/subject.done
