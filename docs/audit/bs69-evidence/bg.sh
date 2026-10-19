#!/bin/bash
# bs69: start one job detached on this host, setsid, its pid recorded; kill only by that pid.
# `setsid -f` forks it, with no `&`: a non-interactive shell's `&` would start it with SIGINT and SIGQUIT
# ignored (measured on hasu: SigIgn 0x6), and an ignore inherited at start is what chapter 34 reads.
# Usage: bg.sh NAME command [args...]   -> ~/lanes/bs69/logs/NAME.pid, NAME.out
L=$HOME/lanes/bs69; mkdir -p $L/logs; n=$1; shift
setsid -f bash -c 'echo $$ > "$0"; ulimit -c 0; exec "$@"' $L/logs/$n.pid "$@" > $L/logs/$n.out 2>&1 < /dev/null
sleep 1; echo "started $n pid $(cat $L/logs/$n.pid)"
