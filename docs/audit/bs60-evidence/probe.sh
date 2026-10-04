#!/bin/bash
# bs60 probes (bs59's probe.sh, lane dir renamed): each program alone in its own directory; pair = old|new.
# lupin <file>; wolf run (bs59 added it); wolf conform-run --json (default lane); --checked.
set -u
L=$HOME/lanes/bs60; pair=$1; shift
W=$(ls $L/$pair/wolf-*/wolf); U=$(ls $L/$pair/lupin-*/lupin)
[ -x "$W" ] && [ -x "$U" ] || { echo "NO TOOLS for $pair"; exit 1; }
echo "## pair=$pair wolf=$($W --version | head -1) lupin=$($U --version)"
cd $L/probes/one || exit 1
dirs=("$@"); [ ${#dirs[@]} -eq 0 ] && dirs=($(command ls $L/probes/one))
for d in "${dirs[@]}"; do
  [ -d "$L/probes/one/$d" ] || { echo "MISSING $d"; continue; }
  cd $L/probes/one/$d; f=$d.lu
  echo "=== $pair $(cat DIRECTIVE)"
  out=$(timeout 60 $U $f 2>&1); echo "lupin exit=$?"; echo "$out" | head -8 | sed 's/^/  out| /'
  out=$(timeout 60 $W run $f 2>&1); echo "wolf run exit=$?"; echo "$out" | grep -v "^\s*$" | head -4 | sed 's/^/  out| /'
  out=$(timeout 60 $W conform-run --json $f 2>&1); echo "wolf conform-run exit=$?"; echo "$out" | grep -E '^\{' | python3 -c 'import json,sys
for l in sys.stdin:
    r=json.loads(l); print("  verdict=",r.get("verdict"),"|",r.get("x-unsupported-construct",""),r.get("x-unsupported-span",""),"| diags=",[x["code"] for x in r.get("diagnostics",[])])'
  out=$(timeout 60 $W conform-run --json --checked $f 2>&1); echo "wolf conform-run --checked exit=$?"; echo "$out" | grep -E '^\{' | python3 -c 'import json,sys
for l in sys.stdin:
    r=json.loads(l); print("  verdict=",r.get("verdict"),"|",r.get("x-unsupported-construct",""),"| stdout=",(r.get("stdout_inline") or "")[:80].replace("\n","/"))'
  echo
done
