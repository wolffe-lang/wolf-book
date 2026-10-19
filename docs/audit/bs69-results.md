# bs69 — results against the prediction

Crawl 54, 2026-10-19. The prediction is `d9b804b`
(`docs/audit/bs69-prediction.md`), committed before the 0.2.33 archive
was downloaded and before the first change; this file measures it. Every
program ran inside a container on kasumi (rootless podman, its own pid
namespace, `--network none`, RLIMIT_CORE 0 soft and hard) or inside its
own pid namespace on hasu (`ns.sh`: an unprivileged user and pid
namespace, RLIMIT_CORE 0, an lld already in the nix store on `PATH`);
macOS and windows are the book's CI samples lanes. Evidence:
`docs/audit/bs69-evidence/` (`kasumi/`, `hasu/`, the scripts beside
them).

## The move table, by machine (unedited tree, 0.2.32 → 0.2.33)

594 programs (`allprogs.py`), each swept twice per pair on kasumi
(`kasumi/sweep-{c32,new}{a,b}.jsonl`; `cmp.py`):

| machine | control noise (c32a vs c32b) | new noise (newa vs newb) | **c32 → 0.2.33** (a vs a / b vs b) | outside both noise sets |
|---|---|---|---|---|
| lupin 0.1.54 (the same binary) | 0 | 0 | **0 / 0** | 0 |
| native (`conform-run --json`) | 0 | 0 | **0 / 0** | 0 |
| checked (`conform-run --json --checked`) | 0 | 0 | **0 / 0** | 0 |
| release (`build --release` and run) | 6 | 7 | **5 / 7** | 0 |

The release rows are B-11 and ex9-4 (exits after undefined behaviour),
ex11-5, ex17-1, ch17's balances (`ch17.md:29`, `:113`), ex30-5 and
`ch10.md:84`: every one in the control's or the subject's own noise
(`kasumi/sweep-c32-vs-new.txt`, `-b.txt`, `sweep-c32-noise.txt`,
`sweep-new-noise.txt`). **No program moved on any machine.** As
predicted (§6).

The head sweep (596 programs at `7dcbead`, once, against the unedited
tree's `newa`, `kasumi/sweep-newa-vs-head-7dcbead.txt`): lupin 0, native
0, checked 0, release 7, all in the noise sets; the only other rows are
chapter 34's programs whose page lines moved (`MISSING` by slug) and the
two new ones, each answered as §7 says (below).

## The gates

| run | samples (samples / passed / pending / failed / flips) | consoles |
|---|---|---|
| control `eb657f2`, kasumi, archive whole | 529 / 526 / 3 / 0 / 0 | 498 of 518 |
| control `eb657f2`, hasu (namespace), and under `taskset -c 0-3` | 529 / 525 / 3 / **2** / 0 | 497 of 518 |
| subject `d9b804b` (unedited), kasumi, and under `taskset -c 0-3` | 529 / 526 / 3 / **4** / 0 | 494 of 518 |
| subject `d9b804b`, kasumi, bare `wolf` | 529 / 517 / 3 / 21 / 0 | 486 of 518 |
| subject `d9b804b`, hasu, and under `taskset -c 0-3` | 529 / 525 / 3 / **6** / 0 | 493 of 518 |
| **head `7dcbead`, kasumi, and under `taskset -c 0-3`** | **531 / 528 / 3 / 0 / 0** | **500 of 520** |
| head `7dcbead`, kasumi, bare `wolf` (for the record) | 531 / 516 / 3 / 23 / 0 | 489 of 520 |
| **head `7dcbead`, hasu, and under `taskset -c 0-3`** | **531 / 528 / 3 / 0 / 0** | **500 of 520** |

The subject's four are exactly §5's: `book/back/colophon.md:7`,
`book/ch01.md:153`, `book/ch22.md:288`,
`principles/exercises/ch22/EXERCISES.md:218`, text only, every ch22 hash
held (kasumi and hasu print the same listing). hasu's two more at the
control and the subject are one program: bs68's `jobs.lu` named
`/bin/sleep`, which NixOS does not have (`error: not_found`, the sample
and its console); the std move starts `sleep` by name on the `PATH`, and
the head passes there. The bare runs fail every program that imports the
std (the archive's `std/` is the default root), as at bs68.

Head gate on kasumi (`kasumi/gate-7dcbead/gate-summary.txt`): every gate
exit 0 against the archive whole — samples (and `taskset -c 0-3`),
`samples --self-test`, `verify-docs` (ok, corpus count 268, 36 version
literals audited against wolf 0.2.33 / lupin 0.1.54), `ledger --check`
(**129 open: 119 filed, 10 waived, 0 unfiled; 110 closed**, chapter 34
3 waived and 1 closed), `backmatter --check` (the vendored spec matches
wolf-lang at v0.2.33), `contrast`, `tiers --check`, `diagrams --check`
(6 from 4), `render web`, and `cargo test -p xtask -- --nocapture` **205
passed** (203 + the two new tests); SKIPs counted (samples 4,
self-test 1, the rest 0). hasu (`hasu/gate-7dcbead/`): samples (and
`taskset -c 0-3`), `verify-docs`, `ledger --check`, `diagrams --check`,
exit 0.

## Against the prediction

| § | predicted | measured |
|---|---|---|
| 1 | `[wolf]` → `85dab5d7`, 0.2.33; lupin and the trace pin unchanged | as predicted |
| 2 | the stamps to the character; nine releases, 955 commits | byte-identical (`kasumi/subject-members.txt`, and hasu's) |
| 3 | anchors 661 → 664 (+3, none dropped); grammar, catalogue, help unchanged | as predicted (`anchors-diff.txt`) |
| 4 | 0.2.32's member names; `std/` 58 files, 54 `.lu`, `STD-REV` `cceda631…`, differing in `process.lu`, `signal.lu`, `STD-REV`; completions unchanged; GLIBC ≤ 2.34 | as predicted; `README.md` moved too (the guess allowed for it); manifest `4382865a…` |
| 5 | subject 526 / 3 / 4 console drifts, 494 of 518; bare 21 | as predicted, to the count (hasu: +2, the `/bin/sleep` program, not predicted) |
| 6 | lupin 0, native 0, checked 0, release noise | as predicted |
| 7 | the seven programs on four machines | every cell as predicted but one (below) |
| 8 | the sentences the release makes false | as listed; nothing else found false |
| 9 | #83's and #84's tests red before their fixes, green after; the six links | as predicted (run 39314984062; 205 passed after) |
| 10 | head 531 / 528 / 3 / 0, 500 of 520; ledger 129 / 10 / 110; `solutions.md` by seven lines | as predicted |

**Missed:** the checked machine refuses `one.lu` before it runs (the
refusal is static, at `phase_reached: mem`, no line printed), not after
its first two lines; hasu could not run bs68's `jobs.lu` at all (no
`/bin/sleep` on NixOS), found by the control, not predicted.

## Chapter 34's programs on four machines (kasumi, the container; `kasumi/probes.txt`)

| program | `wolf run` | checked (`conform-run --json --checked`) | release | lupin 0.1.54 (no std) |
|---|---|---|---|---|
| `who.lu` | `an id: true, a parent: true` / `false` / `false` | the same | the same | `` `env.pid` does not resolve ``, exit 4 |
| `window.lu` | `size: io, keys: io` | the same | the same | `` `fs.stdout` does not resolve `` |
| `groups.lu` | `true`, `ok`, `invalid`, `not_found`, `invalid`, `true` | the same | the same | `` `process.lead_group` does not resolve `` |
| `nochild.lu` | `nothing to report: -1, 0 live` | the same | the same | **the same** |
| `jobs.lu` | `a stop: true, by stop(): true` / `an end: true, by kill(): true` / `then: changed false, 0 live` | the same | the same | `` `signal.listen` does not resolve `` |
| `one.lu` | `kill: invalid, stop: invalid` / `nothing: invalid, two at once: invalid` / `a fault meaning: ok` / `lived through the segment_fault() it sent itself` | `unsupported` by name: `` `os_signal_ignore_one`/`os_signal_default_one` of a fault meaning (SEGMENT_FAULT, BUS_ERROR: the `wolf` process's own stack guard holds them; [os.signal.disp.one]) `` | the same as `wolf run` | `` `signal.kill` does not resolve `` |
| `again.lu` | `ended by terminate(): true` / `and with a core: false` | `false` / `false` (`os_exe()` is `wolf`: `` `again` is not a wolf command ``) | the same as `wolf run` | `` `env.args` does not resolve `` |
| `text0.lu` (probe) | `words for 0: Unknown signal 0` / `words for terminate(): Terminated` / `nothing to report, core sibling: -1, 0 live` | the same | the same | `` `signal.text` does not resolve `` |

Each the same under `taskset -c 0-3`. The container's and every
program's `SigBlk` and `SigIgn` read 0 (the probe log's head). No core
was dumped: no program ends by a signal that writes one, and RLIMIT_CORE
is 0 wherever they ran. The page's programs are the drafts byte for byte
(`draft/`).

## Windows

The std moves answer bs67's and bs68's rows unchanged (s5, s7, s8, s10
held in every windows run). §34.6's two programs went red on the windows
lane of run 39317769517 (job 118283595542: `book/ch34/s11` and `s12`,
`exited Some(1), expected 0`, 531 / 518 / 3 / 2 failed, 427 of 520),
and their rows are in `samples-os.toml` (`a1ea1fc`), stdout as §7
predicted; the windows lane of run 39319348113 at `7dcbead` holds them
(job 118289027705: 531 / 518 / 3 / 0 failed / 0 flips, 427 of 520,
`11 row(s) apply to windows — 10 declared refusal(s) and 1 declared
transcript(s) held`, `std/` staged at `4382865a…`).

## Seen red

- Run 39314984062 at `422c175` (the pin, the CI manifest, #83's and
  #84's tests before their fixes, the text unedited): samples red on
  ubuntu (job 118273988594) and windows (118273988563) on exactly the
  four blocks of §5, each staging `std/` at `4382865a…`; the test job
  red on ubuntu (118273988350) and macOS (118273988206) on exactly the
  two new tests and on windows (118273988428) on #84's alone (#83's is
  unix-only). `red-39314984062.txt`.
- Run 39317769517 at `a99aa37`: windows samples red on exactly s11 and
  s12 before their rows (above).

## Process notes

- hasu has no container runtime, and RULES.md §5 (received mid-lane)
  asks for one. hasu's first control run (19:43Z) ran on the host's
  process table, without an lld on `PATH` (22 failures: 20 linker notes
  and the `/bin/sleep` program); it was stopped by its recorded pid and
  re-run inside `ns.sh`. Its logs are kept in `hasu/host-run/`. Its
  programs signalled nothing but themselves and the `sleep` they
  started; every group send carries meaning 0. A first namespace run was
  started with SIGINT and SIGQUIT ignored (`SigIgn 0x6`: `bg.sh`'s `&`)
  and a second overlapped it in the same checkout; both were stopped by
  recorded pid (the namespace's pid 1 takes SIGKILL only,
  `host-run/control-ns{1,2}.killed`) and the control re-run clean;
  `bg.sh` uses `setsid -f` since.
- The superseded PR runs at `b8dc287` (39317391421) and `a99aa37`
  (39317769517, after its windows lane had answered) were cancelled by
  me; both are this lane's.
