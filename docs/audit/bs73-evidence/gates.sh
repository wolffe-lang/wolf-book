#!/usr/bin/env bash
set -euo pipefail
cd /home/mfwolffe/lanes/bs73/wolf-book
evidence=docs/audit/bs73-evidence
set +e
cargo xtask samples > "$evidence/wrappers-before-record.log" 2>&1
rc=$?
set -e
printf 'draft samples exit %s\n' "$rc"
if [ "$rc" -ne 1 ]; then echo 'expected exactly the changed console to drift'; exit 1; fi
python3 -I - <<'PY'
from pathlib import Path
s=Path('docs/audit/bs73-evidence/wrappers-before-record.log').read_text()
assert s.count('samples: FAIL ')==1,s[-4000:]
assert 'console block drifted from the real run' in s
assert '1 failure(s), 0 flip(s)' in s
PY
python3 -I docs/audit/bs72-evidence/record-consoles.py "$PWD" "$PWD/$evidence/wrappers-before-record.log" > "$evidence/console-import.log"
cat "$evidence/console-import.log"
{
 cargo xtask tiers
 cargo xtask tiers --check
 cargo xtask backmatter
 cargo xtask verify-docs
 cargo test --offline -p xtask
 cargo xtask samples --self-test
 python3 -I ci/tiers-test.py
} > "$evidence/static-gates.log" 2>&1
cargo xtask samples > "$evidence/final-samples.log" 2>&1
grep '^samples: [0-9]\|^samples: of those' "$evidence/final-samples.log"
