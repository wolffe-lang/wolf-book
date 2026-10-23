"""Chapter 34's process/signal family, inside the container on CPUs 0-3."""
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys

root, work, wolf = map(Path, sys.argv[1:4])
print('affinity', sorted(os.sched_getaffinity(0)), flush=True)
assert os.sched_getaffinity(0) <= {0, 1, 2, 3}
# Python ignores these two for its own I/O. The programs under test must
# start with defaults; reset only this harness process before spawning them.
for sig in (signal.SIGPIPE, signal.SIGXFSZ):
    signal.signal(sig, signal.SIG_DFL)
status = Path('/proc/self/status').read_text().splitlines()
for line in status:
    if line.startswith(('SigBlk:', 'SigIgn:')):
        print(line, flush=True)
        assert int(line.split()[1], 16) == 0
for sample in ('s13', 's14', 's15', 's16'):
    for release in (False, True):
        case = work / (sample + ('-release' if release else '-native'))
        case.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(root / 'samples/extracted/ch34' / sample / (sample + '.lu'), case / 'main.lu')
        if release:
            build = subprocess.run([str(wolf), 'build', '--release', 'main.lu', '-o', 'program'],
                                   cwd=case, capture_output=True, text=True, timeout=40)
            assert build.returncode == 0, build.stderr
            command = [str(case / 'program')]
        else:
            command = [str(wolf), 'run', 'main.lu']
        p = subprocess.run(command, cwd=case, capture_output=True, text=True, timeout=30)
        print(json.dumps(dict(sample=sample, release=release, exit=p.returncode,
                              stdout=p.stdout, stderr=p.stderr)), flush=True)
        assert p.returncode == 0, (sample, p)
print('ALL EIGHT CONSTRAINED PROCESS/SIGNAL RUNS PASS', flush=True)
