#!/usr/bin/env bash
set -euo pipefail
cd "$HOME/lanes/bs72/wolf-book"
trap 'cp "$HOME/lanes/bs72/r7-final-samples.rs" xtask/src/samples.rs' EXIT
for variant in byte host probe; do
    python3 -I - "$variant" <<'PY'
from pathlib import Path
import sys
s=Path.home().joinpath('lanes/bs72/r7-final-samples.rs').read_text()
variant=sys.argv[1]
if variant=='byte':
    for old in ['&& observed.stdout == stdout', '&& outcome.stdout == stdout']:
        assert s.count(old)==1;s=s.replace(old,'&& true /* planted: stdout was not compared */',1)
elif variant=='host':
    old=' || row.os != host';assert s.count(old)==1;s=s.replace(old,' || false /* planted: host was not compared */',1)
else:
    start=s.index('fn declared_host_graduation(')
    at=s.index('    if !matches!(check',start)
    s=s[:at]+'    if outcome.passed && outcome.graduates.is_none() { return true; } // planted\n'+s[at:]
Path('xtask/src/samples.rs').write_text(s)
PY
    case "$variant" in
        byte) test=host_graduation_rejects_one_byte_drift;;
        host) test=host_graduation_is_scoped_to_named_host;;
        probe) test=plain_passing_refusal_without_probe_still_flips;;
    esac
    printf 'R7 deliberate %s break: %s\n' "$variant" "$test"
    set +e
    cargo test --offline -p xtask "$test" -- --nocapture > "$HOME/lanes/bs72/logs/r7-red-$variant.log" 2>&1
    rc=$?
    set -e
    if [ "$rc" -eq 0 ]; then echo 'ERROR: planted break passed'; exit 1; fi
    grep -F "test samples::tests::$test ... FAILED" "$HOME/lanes/bs72/logs/r7-red-$variant.log"
    grep '^test result:' "$HOME/lanes/bs72/logs/r7-red-$variant.log"
done
cp "$HOME/lanes/bs72/r7-final-samples.rs" xtask/src/samples.rs
printf 'R7 final source: all tests, doc truth, and sample-rig self-test\n'
cargo test --offline -p xtask > "$HOME/lanes/bs72/logs/r7-green.log" 2>&1
cargo xtask verify-docs >> "$HOME/lanes/bs72/logs/r7-green.log" 2>&1
cargo xtask samples --self-test >> "$HOME/lanes/bs72/logs/r7-green.log" 2>&1
grep -E '^test result:|^verify-docs: ok|^samples: self-test ok' "$HOME/lanes/bs72/logs/r7-green.log"
