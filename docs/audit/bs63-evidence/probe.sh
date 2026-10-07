#!/bin/bash
# bs63 probes: each witness alone in its own directory, on one pair (c23|c24|new):
# lupin; wolf run; wolf conform-run --json (native); --checked.
set -u
L=$HOME/lanes/bs63; P=$L/$1; W=$(ls $P/wolf-*/wolf); U=$(ls $P/lupin-*/lupin)
echo "## pair=$1 $($W --version | head -1) / $($U --version)"
for d in $L/extra/*/; do d=${d%/}; n=$(basename $d); cd $d
  echo "=== $n"
  out=$(timeout 60 $U $n.lu 2>&1); echo "lupin exit=$?"; echo "$out" | head -6 | sed 's/^/  | /'
  out=$(timeout 120 $W run $n.lu 2>&1); echo "wolf run exit=$?"; echo "$out" | head -6 | sed 's/^/  | /'
  for m in "" "--checked"; do out=$(timeout 120 $W conform-run --json $m $n.lu 2>/dev/null | grep '^{'); echo "conform-run $m: $(echo "$out" | python3 -c 'import json,sys
r=json.loads(sys.stdin.read() or "{}"); print(r.get("verdict"),[d["code"] for d in r.get("diagnostics",[])],r.get("x-unsupported-construct",""),repr((r.get("stdout_inline") or "")[:80]))')"; done
done
