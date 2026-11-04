"""Splice reviewed diagnostic snapshots into their `diagnostic,from(ID)` blocks.

A block is touched only when its whole content is the one line PENDING, and it
is replaced only with the snapshot `cargo xtask samples --bless` wrote for ID,
byte for byte. Nothing is typed. bs74.
"""
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
pattern = re.compile(r'^```diagnostic,from\(([^)]+)\)\nPENDING\n```$', re.M)
missing = 0
for md in sorted((root / 'book').rglob('*.md')):
    text = md.read_text()

    def splice(match):
        global missing
        sid = match.group(1)
        snap = root / 'snapshots' / 'diagnostics' / (sid.replace('/', '__') + '.txt')
        if not snap.is_file():
            missing += 1
            print(f'{md.relative_to(root)}: {sid}: no snapshot at {snap.relative_to(root)}')
            return match.group(0)
        body = snap.read_text()
        assert body.endswith('\n'), snap
        print(f'{md.relative_to(root)}: {sid} <- {snap.relative_to(root)}')
        return f'```diagnostic,from({sid})\n{body}```'

    new = pattern.sub(splice, text)
    if new != text:
        md.write_text(new)
sys.exit(1 if missing else 0)
