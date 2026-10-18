#!/bin/bash
# bs68, INSIDE the container (ctr.sh): the new pages' programs on the four machines (probes.sh over
# ev/<dir>), then exit 71 at a descriptor limit (limits.sh). Args: [dir, default new] [tag].
L=$HOME/lanes/bs68; D=${1:-new}; T=${2:-$D}
bash $L/ev/probes.sh $L/ev/$D > $L/logs/probes-$T.txt 2>&1
bash $L/ev/limits.sh > $L/logs/limits-$T.txt 2>&1
echo DONE > $L/logs/probe-$T.done
