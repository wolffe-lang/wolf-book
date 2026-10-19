#!/usr/bin/env bash
unshare --user --map-current-user --pid --fork --mount-proc bash -c 'ulimit -c 0; echo "ns: pid $$ procs $(ls -d /proc/[0-9]* | wc -l) uid $(id -u) core $(ulimit -c)"; grep -E "^Sig(Blk|Ign)" /proc/self/status; kill -0 1 && echo "pid 1 is $(cat /proc/1/comm)"; sleep 1 & echo "child $!"; wait'
echo "exit $?"
