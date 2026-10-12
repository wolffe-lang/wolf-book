#!/bin/bash
# bs65: (a) ch25's three §25.2 imports and std.io against the 0.2.28 archive whole and the bare wolf;
# (b) ch22's summary.lu interface, whole and bare, and the one call that pulls std.range in.
set -u
L=$HOME/lanes/bs65; export TMPDIR=$L/tmp; B=${B:-$L/wolf-book}; W=$(ls $L/new/wolf-*/wolf); WB=$L/bare/wolf
for m in io list os.path x.http; do T=$(mktemp -d); cd $T
  printf 'use std.%s\n\nfn main() {\n    print("ok")\n}\n' $m > p.lu
  for w in $W $WB; do echo "--- use std.$m — $w"; $w conform-run --json p.lu 2>&1 | grep -E '^error|^\{' | cut -c1-220; done
  cd /; rm -rf $T; done
T=$(mktemp -d); cp $B/samples/pkg/mods-iface/summary.lu $T/; cd $T
for w in $W $WB; do echo "--- wolf interface ./summary.lu — $w"; $w interface ./summary.lu 2>&1; done
printf '/// e\npub fn e(xs: List[int]) -> bool {\n    xs.is_empty()\n}\n' > summary.lu; cat summary.lu
for w in $W $WB; do echo "--- one is_empty — $w"; $w interface ./summary.lu 2>&1 | grep -E "^module|export_hash"; done
cd /; rm -rf $T
