"""R-bs72-4/5/6 witnesses. Run inside the lane container, never on a host."""
import json
import os
from pathlib import Path
import pty
import subprocess
import sys

work = Path(sys.argv[1])
work.mkdir(parents=True, exist_ok=True)
wolf, lupin, control = map(Path, sys.argv[2:5])
records = []


def run(name, tool, source, terminal=False):
    case = work / name
    case.mkdir(exist_ok=True)
    program = case / 'main.lu'
    program.write_text(source)
    master = slave = None
    if terminal:
        master, slave = pty.openpty()
    try:
        cmd = [str(tool), 'run', 'main.lu'] if tool.name == 'wolf' else [str(tool), 'main.lu']
        p = subprocess.run(cmd, cwd=case, stdin=slave if terminal else subprocess.DEVNULL,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=30)
    finally:
        if terminal:
            os.close(master)
            os.close(slave)
    record = dict(name=name, tool=str(tool), exit=p.returncode, stdout=p.stdout, stderr=p.stderr)
    records.append(record)
    print(json.dumps(record), flush=True)
    return record


clock = run('clock', lupin, 'fn main() { print("{os_clock_ns() >= 0}") }\n')
assert clock['exit'] == 0 and clock['stdout'] == 'true\n'
for name, source, terminal in [
    ('fork', 'fn main() -> !int { let _ = os_fork()?; 0 }\n', False),
    ('restore', 'fn main() -> !int { os_term_restore_at_exit(0, 287, List[byte]())?; 0 }\n', True),
    ('wait-fds', 'fn main() -> !int { let p = os_pipe()?; let _ = os_signal_wait_fds([p.0], 16384, 25)?; 0 }\n', False),
]:
    rec = run(name, lupin, source, terminal)
    assert rec['exit'] == 4 and 'unsupported:' in rec['stderr'] and 'does not resolve' not in rec['stderr'], rec

save = '''fn main() -> !int {
    fs_write_text("note", "new")?
    let f = fs_open_mode("note", 1)?
    fs_sync(f)?
    fs_close(f)?
    fs_sync_dir(".")?
    fs_create_dir_all("drafts")?
    let here = fs_realpath(".")?
    let up = fs_realpath("drafts/..")?
    print("{here == up}")
    0
}
'''
for name, tool in [('save-lupin', lupin), ('save-wolf', wolf)]:
    rec = run(name, tool, save)
    assert rec['exit'] == 0 and rec['stdout'] == 'true\n', rec

old_mode = '''use std.term
fn main() {
    let cooked = term.Mode {
        canonical: true, echo: true, signals: true,
        flow: true, extended: true, min: 1, time: 0,
    }
    print("{term.pack(cooked)} {term.pack(term.raw(cooked))}")
}
'''
new_mode = old_mode.replace('flow: true, extended: true, min:',
                            'flow: true, extended: true, raw_cr: false, raw_output: false, strip8: false, min:')
for name, tool, source in [('mode-control', control, old_mode), ('mode-subject', wolf, new_mode)]:
    rec = run(name, tool, source)
    assert rec['exit'] == 0 and rec['stdout'] == '287 260\n', rec

print('ALL R-bs72-4/5/6 WITNESSES PASS', flush=True)
