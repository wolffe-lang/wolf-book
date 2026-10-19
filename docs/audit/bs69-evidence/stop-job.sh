#!/usr/bin/env bash
# bs69: stop one of this lane's jobs by its RECORDED pid (logs/NAME.pid) and the pids of its direct
# children read now from that pid (recorded in logs/NAME.killed); never by name, pattern or group.
# A job under ns.sh is `unshare` (which ignores SIGTERM) and its child, the namespace's pid 1 (which
# takes no SIGTERM from outside without a handler): those get SIGKILL, which ends the whole namespace.
L=$HOME/lanes/bs69; n=$1; p=$(cat $L/logs/$n.pid)
kids=$(ps -o pid= --ppid $p | tr -d ' ' | tr '\n' ' ')
{ date -u +%FT%TZ; echo "leader $p: $(ps -o args= -p $p | cut -c1-120)"; for k in $kids; do echo "child $k: $(ps -o args= -p $k | cut -c1-120)"; done; } > $L/logs/$n.killed
case "$(ps -o comm= -p $p)" in unshare) kill -KILL $kids $p;; *) kill $p $kids;; esac 2>&1; sleep 2
for q in $p $kids; do ps -p $q > /dev/null && echo "still alive $q" || echo "gone $q"; done | tee -a $L/logs/$n.killed
