"""Import complete actual console blocks from cargo xtask samples' drift log.

Never substitutes version strings. Only a real replay's full actual block
can replace the exact expected block, uniquely, in the named Markdown file.
"""
from pathlib import Path
import re
import sys

root, log = Path(sys.argv[1]), Path(sys.argv[2])
text = log.read_text()
pattern = re.compile(r'^samples: FAIL (.+?\.md):\d+: console block drifted from the real run\n'
                     r'     expected:\n((?:       \|[^\n]*\n)*)'
                     r'     actual:\n((?:       \|[^\n]*\n)*)', re.M)
for match in pattern.finditer(text):
    path = root / match[1].split('/wolf-book/', 1)[1]
    expected, actual = ['\n'.join(line[9:].lstrip(' ') if line == '       |' else line[9:]
                                 for line in part.splitlines()) + '\n' for part in match.group(2, 3)]
    source = path.read_text()
    old = expected + '```'
    assert source.count(old) == 1, (path, expected)
    path.write_text(source.replace(old, actual + '```', 1))
    print(path.relative_to(root))
