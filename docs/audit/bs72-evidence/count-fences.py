"""Count active check-bearing book fences and exercise headers at two heads."""
from collections import Counter
from pathlib import PurePosixPath
import re
import subprocess
import sys


def count(rev):
    paths = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', rev,
                                     '--', 'book', 'principles/exercises'], text=True).splitlines()
    counts = Counter()
    ub = 0
    for path in paths:
        p = PurePosixPath(path)
        if path.startswith('book/'):
            if p.suffix != '.md' or p.name in ('SUMMARY.md', 'solutions.md') or p.name.startswith('_'):
                continue
            text = subprocess.check_output(['git', 'show', rev + ':' + path], text=True)
            text = re.sub(r'<!--.*?-->', '', text, flags=re.S)
            directives = re.findall(r'^```wolf,([^\n]*)', text, flags=re.M)
        elif p.suffix == '.lu':
            text = subprocess.check_output(['git', 'show', rev + ':' + path], text=True)
            directives = re.findall(r'^//! check: (.*)', text, flags=re.M)
        else:
            continue
        for directive in directives:
            m = re.search(r'(?<![\w-])(lupin-run|wolf-run|run|fail)\(', directive)
            if m:
                counts[m[1]] += 1
                ub += m[1] == 'lupin-run' and 'trap(ub)' in directive
    print(rev, dict(sorted(counts.items())), 'lupin UB', ub,
          'eligible lupin-run', counts['lupin-run'] - ub)


for revision in sys.argv[1:]:
    count(revision)
