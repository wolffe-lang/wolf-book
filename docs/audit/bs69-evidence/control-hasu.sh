#!/usr/bin/env bash
# bs69 control on hasu, inside ns.sh (its own pid namespace; no container runtime here): the unedited contract head on the book's
# pin (c32 = wolf 0.2.32 / lupin 0.1.54), the samples gate and the doc gates, under RLIMIT_CORE 0 (hasu
# pipes cores to systemd-coredump with `ulimit -c` unlimited by default). The book's programs signal only
# themselves and the child they start; every group send carries meaning 0.
set -u
ulimit -c 0
L=$HOME/lanes/bs69; cd $L/wolf-book || exit 1; export TMPDIR=$L/tmp; mkdir -p $TMPDIR
[ "$(git rev-parse --abbrev-ref HEAD)" = bs69 ] || { echo WRONG BRANCH > $L/logs/control.done; exit 1; }
{ echo "host $(hostname) core $(ulimit -c)/$(ulimit -Hc)"; grep -E '^Sig(Blk|Ign)' /proc/self/status; git rev-parse HEAD; } > $L/logs/control-head.txt
X=$L/target/debug/xtask; W=$(ls $L/c32/wolf-*/wolf); U=$(ls $L/c32/lupin-*/lupin)
for t in "samples" "verify-docs" "ledger --check" "diagrams --check"; do
  n=control-c32-$(echo $t | tr -d " -").log
  WOLF=$W LUPIN=$U LUPIN_TRACE=$U $X $t > $L/logs/$n 2>&1; echo "EXIT $?" >> $L/logs/$n
  git clean -qfdX
done
taskset -c 0-3 env WOLF=$W LUPIN=$U $X samples > $L/logs/control-c32-samples-taskset4.log 2>&1; echo "EXIT $?" >> $L/logs/control-c32-samples-taskset4.log
git status --porcelain > $L/logs/control-c32-status.txt
git clean -qfdX; git checkout -q -- .
echo DONE > $L/logs/control.done
