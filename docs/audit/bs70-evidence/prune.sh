#!/usr/bin/env bash
# bs70: prune this lane's build outputs and archives on this host (only under ~/lanes/bs70), keep logs.
L=$HOME/lanes/bs70; cd $L || exit 1
du -sh $L 2>/dev/null | sed 's/^/before: /'
for d in target target-work progs progs-head tmp wolf-lang-0234 wb-work c33 new bare; do [ -e "$L/$d" ] && rm -rf "$L/$d" && echo "removed $d"; done
du -sh $L 2>/dev/null | sed 's/^/after: /'; df -h $HOME | tail -1
