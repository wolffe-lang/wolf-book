#!/bin/bash
# bs68, INSIDE the container (ctr.sh): exit 71. Each program in ev/limit/ is built by the new pair
# (`wolf build`, the dev tier, and `wolf build --release`), then each binary runs twice: with the
# container's own descriptor limit, and under `prlimit --nofile=4:4` (0, 1 and 2 open and one more for the loader: one descriptor to give, where the readiness set wants two and the hub four),
# its stdout, stderr and exit status printed. Also under taskset -c 0-3. The checked machine and
# lupin have no readiness set and no hub (`[os.net.reactor]`, `[os.signal.hub]`): not run at a limit.
set -u
L=$HOME/lanes/bs68; export TMPDIR=$L/tmp; unset WOLF_STD LUPIN_STD
W=$(ls $L/new/wolf-*/wolf)
echo "########## $($W --version | head -1)"
grep -E '^Sig(Blk|Ign)' /proc/self/status
echo "descriptor limit here: $(ulimit -n)"
for f in $L/ev/limit/*.lu; do p=$(basename $f .lu)
  T=$(mktemp -d -p $TMPDIR); cp $f $T/; cd $T
  echo; echo "=== $p.lu"; cat $p.lu
  for tier in dev release; do
    if [ $tier = dev ]; then timeout 300 $W build $p.lu > b.out 2>&1 < /dev/null; b=$?; else timeout 300 $W build --release $p.lu > b.out 2>&1 < /dev/null; b=$?; fi
    echo "\$ wolf build$([ $tier = release ] && echo ' --release') $p.lu  [exit $b]"; cat b.out
    [ $b = 0 ] || continue
    echo "\$ ./$p"; timeout 30 ./$p < /dev/null; echo "[exit $?]"
    echo "\$ prlimit --nofile=4:4 ./$p"; timeout 30 prlimit --nofile=4:4 ./$p < /dev/null; echo "[exit $?]"
    echo "\$ taskset -c 0-3 prlimit --nofile=4:4 ./$p"; timeout 30 taskset -c 0-3 prlimit --nofile=4:4 ./$p < /dev/null; echo "[exit $?]"
    rm -f $p
  done
  cd $L; rm -rf $T
done
