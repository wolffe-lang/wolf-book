#!/bin/bash
# bs66: extract the result lines of one run's three samples jobs into an evidence file.
# Args: out-file header-line job:host...
out=$1; shift; hdr=$1; shift
{ echo "$hdr"; echo "Extracted from each samples job's log (gh run view --job <id> --log), the workflow's echoed script lines dropped:"
  for jh in "$@"; do j=${jh%%:*}; h=${jh#*:}; echo; echo "== samples ($h), job $j"
    grep -h 'samples: FAIL\|samples: [0-9]* passed\|console block(s)\|staged std/\|STD-PIN:\|actual:\|       | wolf 0\.2\.\|       | lupin 0\.1\.\|       | paired\|toolchain 0\.2' cilogs/$PFX-$j.log | cut -f3- | cut -c30-250 | grep -v 'DECLINED\|\[36;1m'
  done; } > "$out"
