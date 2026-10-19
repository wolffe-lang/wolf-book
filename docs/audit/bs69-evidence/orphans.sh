#!/usr/bin/env bash
# bs69: list this user's processes whose command line or cwd is under ~/lanes/bs69 (read only).
for p in $(ps -u $(id -u) -o pid=); do c=$(readlink /proc/$p/cwd 2>/dev/null); a=$(tr '\0' ' ' < /proc/$p/cmdline 2>/dev/null)
  case "$c $a" in *lanes/bs69*) echo "$p ppid $(ps -o ppid= -p $p | tr -d ' ') cwd $c :: $a";; esac; done
