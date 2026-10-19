#!/usr/bin/env bash
# bs69: summarize this host's lane logs (read only). Args: glob (default *.log) under ~/lanes/bs69/logs
cd $HOME/lanes/bs69/logs || exit 1
command ls -1 | tr '\n' ' '; echo
for f in ${1:-*.log}; do [ -f "$f" ] || continue; echo "== $f"
  grep -E '^samples: [0-9]+ (sample|passed|of)|^error:|^EXIT|^ledger: +(total|ch34)|^diagrams:|^verify-docs: (ok|FAIL)|programs$|records$' "$f" | tail -8; done
for d in *.done; do [ -f "$d" ] && echo "$d: $(cat $d)"; done
