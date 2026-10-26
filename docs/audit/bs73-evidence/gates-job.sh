#!/usr/bin/env bash
set -u
base=/home/mfwolffe/lanes/bs73
echo $$ > "$base/gates.pid"
"$base/ctr.sh" bash "$base/gates.sh" > "$base/gates-launch.log" 2>&1
rc=$?;echo "$rc" > "$base/gates.exit";exit "$rc"
