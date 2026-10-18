# bs68 — results against the prediction

The book at wolf 0.2.32 (wolf-lang `72de141f`, release 416327027) / lupin
0.1.54 (wolf-interp `0e536e8`, release 415937354), from 0.2.30 / 0.1.53;
ex33-7's checked record; the shift rule, exit 71, the poll and the group
calls, shown as far as the released compiler and its bundled std
(wolf-std `d37c911`) carry them. Prediction `b42f6a9`, committed before
either archive was downloaded and before the first change. One host
measured every program locally: kasumi (linux x86-64, 16 cpus), from the
release archives by digest, each digest read twice
(`kasumi/subject-download.log`, `subject-download-twice.log`), **every
program run inside a container** (`ctr.sh`: rootless podman, the lane's
uid, its own pid namespace, `--network none`; SigBlk and SigIgn 0 at the
head of every container log). macOS and windows are the book's CI
samples lanes, which run in full. Evidence under `bs68-evidence/`.

## The move table, by machine: every program, 0.2.30 / 0.1.53 against 0.2.32 / 0.1.54

589 programs (`allprogs.py`), each alone in its own directory, on lupin,
`conform-run --json` (native), `conform-run --json --checked`, and a
`--release` build and run for every native `pass` with a `main` (380).
Each pair swept twice (`kasumi/sweep-{c30,new}{a,b}.jsonl`), noise in
`sweep-*-noise.txt`, the comparison in `sweep-c30-vs-new.txt`: **lupin 0,
native 0, checked 1, release 8.**

| machine | rows | program(s) | 0.2.30 / 0.1.53 | 0.2.32 / 0.1.54 | why |
|---|---|---|---|---|---|
| lupin | 0 | every program | — | — | no exit status, stdout or refusal moves |
| native (`conform-run --json`) | 0 | every program | — | — | no verdict or diagnostic moves |
| **checked** (`conform-run --json --checked`) | **1** | **ex33-7** | no record, killed at 120 s (both control sweeps) | **`exit(0)`, stdout `answered 4 questions on 2 connections`** (both subject sweeps) | s244 (wolf-lang#720, closed): `net_wait` beside other tasks polls a millisecond at a time and hands over; lupin and native answer the same |
| release | 8 | B-11, ex9-4 | an exit status after UB | another | UB's; both in this lane's control noise and subject noise |
| release | | ch10 `md_84`, ex11-5, ex17-1, ex17-3, `ch17.md:113`, ex30-5 | one schedule's order or balance | another's | the scheduler's: each prints arrival order or shows a race; each is in this lane's control or subject noise (`ch17.md:113` in bs66's and bs67's) |

No sample moved its verdict, stdout or diagnostic on any of the four
machines. Four console transcripts moved their text.

## Samples runner

| run | samples | console blocks |
|---|---|---|
| control (0.2.30 pair, unedited `0070509`), kasumi | 524 / 521 / 3 pending / 0 failed / 0 flips | 495 of 515 |
| subject (0.2.32 pair, unedited `b42f6a9`), archive whole | 524 / 521 / 3 / **4 failed** / 0 flips | 491 of 515 |
| subject, bare `wolf` | 524 / 514 / 3 / 17 failed / 0 flips | 485 of 515 |
| head `3b40fde` (the book tree of record), archive whole, kasumi, also under `taskset -c 0-3` | **529 / 526 / 3 / 0 / 0 flips** | **498 of 518** |
| head `3b40fde`, bare (for the record) | 529 / 517 / 3 / 18 failed | 489 of 518 |

The four are the four predicted: `book/ch01.md:153` and
`book/back/colophon.md:7` (`--version`), `book/ch22.md:288` and
`principles/exercises/ch22/EXERCISES.md:218` (`wolf interface`, the
`toolchain` line and nothing else; every hash held). Spliced from the
run's `actual:` by `splice.py` (`kasumi/subject-samples.log`), never
typed. The bare `wolf` fails every program that imports std (E0301: no
`std/` beside it), their consoles and ch22's listing; CI stages the
archive's `std/`, so the archive whole is the toolchain of record.

Head gate at `3b40fde` (`kasumi/gate-3b40fde/`, `gate-summary.txt`; and
at `a23aa07` before the windows rows, on kasumi, not committed), in the
container, against the archive whole: `samples`, `samples --self-test`,
`verify-docs`, `ledger --check` (130 open: 119 filed, 11 waived, 0
unfiled; 109 closed), `backmatter --check` against wolf-lang at v0.2.32,
`contrast`, `tiers --check`, `render web`, `diagrams --check` with lupin
0.1.54 as `LUPIN_TRACE` (6 from 4), `samples` under `taskset -c 0-3`, and
`cargo test -p xtask -- --nocapture` (203 passed): every one exit 0.
SKIP lines: samples 4, self-test 1, every other gate 0.

## The new pages' programs on four machines (`kasumi/probes-new.txt`, `limits-new.txt`)

| program | native (`wolf run`) | checked (`conform-run --json --checked`) | release | lupin 0.1.54 | windows (CI) |
|---|---|---|---|---|---|
| `shift.lu` (§3.3) | `100 0`, then `wolf-trap: overflow` at 4:5, 134 | `trap(overflow)`, stdout `100 0` | the same as native | `100 0`, then `trap(overflow)` at 4:13 citing `[arith.checked]`, exit 3 | passes |
| `rotate.lu` (§3.3) | `144 1 200` | `exit(0)`, the same | the same | the same | passes |
| `nochild.lu` (§34.5) | `nothing to report: -1, 0 live` | the same | the same | **the same** | passes |
| `jobs.lu` (§34.5) | `a stop: true, by stop(): true` / `an end: true, by kill(): true` / `then: -1, 0 live` | the same | the same | `unsupported: signal.listen does not resolve`, exit 4 | declared: exit 1, `error: io` |
| `groups.lu` (§34.5) | `leads a group of its own: true` / `ok` / `invalid` / `not_found` / `invalid` / `true` | the same | the same | refused by name at `os_set_pgid` (`[os.term.group]`, `[os.signal.group.note] R8`), exit 4 | declared: exit 1, `error: unsupported` |
| `limit.lu` (probe), `prlimit --nofile=4:4` | no line; `wolf: the runtime could not make its readiness set (Too many open files (os error 24)); the program did not start`; **exit 71** | — | the same | — | — |
| `hublimit.lu` (probe), `prlimit --nofile=4:4` | no line; `wolf: the runtime could not make its signal hub (Too many open files (os error 24)); the program did not start`; **exit 71** | — | the same | — | — |

Every `wolf run` row was run again under `taskset -c 0-3` with the same
output, and both limit programs too. Without the limit, both print
their lines and exit 0. ubuntu and macOS CI pass all five blocks and
replay the two `wolf run` consoles (run 39176210454: 526 / 3 / 0, 498 of
518 on ubuntu, job 117819301640).

## Predicted, and what happened

| prediction | result |
|---|---|
| stamps to the character | as predicted (`kasumi/subject-members.txt`) |
| 913 commits, eight releases | as predicted; ch01, colophon |
| anchors 648 → 661, thirteen named, none dropped; grammar unchanged; catalogue 187 | as predicted (`anchors-diff.txt`; `backmatter --check` clean) |
| members: 0.2.30's names; `std/` 58 files, 54 `.lu`, `d37c911f`; the completions identical (a guess) | as predicted, the guess included; manifest `1f7389b7…` |
| GLIBC ≤ 2.34 | 2.34 on both |
| subject: the four console drifts, every hash held, 491 of 515 | as predicted |
| subject: "517 passed, 3 pending, 4 failed" | **miss, my arithmetic**: 521 passed. The four failures are console blocks, counted apart from the 524 samples, exactly as at bs67 (515 / 512 / 3 / 4); the prediction subtracted them |
| lupin 0, native 0, checked 1 (ex33-7 alone, `answered 4 questions on 2 connections`), release noise only | **as predicted in every row** |
| the lupin row I was least sure of (is76's literal-term rule) | moved nothing |
| `shift.lu`, `rotate.lu`, `nochild.lu` on four machines | as predicted, with one miss in the words: lupin's trap line cites `[arith.checked]`, not `[type.int.shift]` |
| `jobs.lu` on four machines | as predicted, **after a drafting fix**: the draft did not compile (E0401: `signal.wait(…)?` is an `int` in a loop body that must be `()`); `let _ =` before it |
| `groups.lu` on four machines | as predicted under `wolf run` (the draft's five lines), **then changed**: in the samples runner's console replay inside the container the draft's first line answered `invalid`, because a console replay inherits the container's process group 1 and group 1 cannot be named (`pgid.sh`, `kasumi/pgid.txt`: pgid 1 for a bare `wolf run`, its own pid's under `timeout` or the runner's sample spawn). The page's program now leads a group of its own first (`os_set_pgid(0)`, and a new first line), which is what the section teaches anyway. lupin then refuses it by name at `os_set_pgid` rather than at the std import |
| exit 71 at `ulimit -n 3` | **miss in the method**: at three descriptors the dynamic loader cannot open `libm.so.6` (exit 127, `Error 24`), and a shell cannot redirect at its own limit. Measured with `prlimit --nofile=4:4` (one descriptor to give where the set wants two and the hub four): exit 71 and the predicted line, on both tiers, as predicted |
| windows: three pass, `jobs.lu` exits 1 at the `child()` listen (`io`), `groups.lu` exits 1 (`unsupported`) | as predicted (`windows-rows.txt`): run 39175460332, job 117817084098, red on exactly `book/ch34/s8` and `book/ch34/s10`; the next run 39176210454, job 117819301817, `9 row(s) apply to windows — 8 declared refusal(s) and 1 declared transcript(s) held`, 518 / 3 / 0, 427 of 518. `groups.lu`'s row is at `os_set_pgid` now, not `term.group`, the same words |
| §8: the sentences the release makes false | as listed, and **one I did not list**: §33.3's "the reactor is lazy and a loop that never parks never asks for one", false since 0.2.31 (found while placing exit 71), now in the past |
| head 529 samples, 526 passed, 498 of 518 consoles | as predicted, also under `taskset -c 0-3` |
| backmatter: solutions.md (22-13's toolchain line) | as predicted, and exercises 34-5 … 34-7 now link §34.5 (the chapter's last section, by backmatter's rule) |
| CI on the unedited book at the pin: red on three hosts on exactly the four | as predicted: run 39174239570 at `9e04c5f` (`red-39174239570.txt`) |
| exit 71 placed in §33.4 | it went in §33.3, beside the reactor sentence it corrects |

Two smaller drafting differences, recorded: the committed draft of
`rotate.lu` shifted `<< 9` where the prediction's table (and the page)
says `<< 8`; the page's program is the table's.

## ex33-7's checked record (item 2)

`principles/exercises/ch33/ex33-7.lu` on `wolf conform-run --json
--checked`: at 0.2.30 no record in either control sweep (`timeout 120`
ended it: `sweep-c30a.jsonl`, `sweep-c30b.jsonl`); at 0.2.32 `verdict
exit(0)`, `stdout_inline "answered 4 questions on 2 connections\n"` in
both subject sweeps (`sweep-newa.jsonl`, `sweep-newb.jsonl`), the line
the page prints from lupin. The book's text names no checked-machine
answer for it, so no page sentence moves; the record is this table's
row and the `[wolf]` note in `wolf-toolchain.toml`.

## The new pages (item 3)

- **§3.3, the shift rule** (`[type.int.shift]`): a plain shift is
  arithmetic on its value and traps `overflow` like `*`; `>>` never
  traps on a count of zero or more; a negative count traps; the count
  has the operand's type; `wrapping[T]` is the machine's shift. Two
  `run(…)` programs and the lupin trap replayed.
- **Exit 71** (`[os.net.reactor]`, `[os.signal.hub]`): §33.3 (the
  readiness set, a `text` block of the measured line) and §34.1 (the
  hub). Both say the program does not start and why 71 is not a trap.
- **§34.5, groups and the children's changes**: `os_signal_send_group`,
  `os_signal_send_all`, `os_set_pgid`, host numbers `-N`,
  `os_signal_last`, `os_signal_realtime`, `os_signal_text`, and
  `os_wait_poll` (`256 + N` a stop, `(-1, n)`, a continue not reported,
  the wait a signal cuts short), as builtins. §34.4's table and its
  machine notes carry the seven. TOC's entries and §Deltas follow.

## The chapter 34 waiver, against the released std (item 4)

Read at wolf-std `d37c911` (the std 0.2.31 and 0.2.32 bundle,
`kasumi/subject-members.txt`'s `STD-REV`): the two §34.3 sentences that
said the wrappers "ship with the next release" were about the ids and
the terminal calls, and `d37c911` wraps every one of them (`env.uid` …
`env.groups`, `term.size`, `term.chars`, `signal.wait_fd`,
`process.set_ids`, `fs.set_owner` and two siblings). They named a
release that has passed and were false, so by the contract's clause they
are re-worded to name the wrappers (the contract's own premise, that the
sentence was about ruling #85's wrappers, was the drift recorded in
`0070509`). The waiver row stays, re-dated 2026-10-18, on §34.5's one
sentence, which is true of `d37c911`: no wrapper for the group calls or
the poll, a program calls the builtins. It names no release, and nothing
from wolf-std trunk (`dcf3e89`) appears on any page.

## Seen red

- **Run 39174239570** (PR #82) at `9e04c5f`, the pin and the CI manifest
  with the book's text unedited (the planted break is the pin itself):
  `samples (ubuntu-latest)` 117813497590, `samples (macos-latest)`
  117813497762 and `samples (windows-latest)` 117813497682 each red at
  "run every sample" with exactly four failures, `book/back/colophon.md:7`,
  `book/ch01.md:153`, `book/ch22.md:288` and
  `principles/exercises/ch22/EXERCISES.md:218`, each a console block
  drifted; 521 / 3 / 4 and 491 of 515 on ubuntu and macOS, 515 / 3 / 4
  and 422 of 515 on windows; the staged `std/` held to `1f7389b7…` on
  all three (`red-39174239570.txt`). Every other job of that run was
  green.
- **Run 39175460332** at `a23aa07` (§34.5 in, no windows rows yet):
  ubuntu and macOS samples green; `samples (windows-latest)` 117817084098
  red on exactly `book/ch34/s8` and `book/ch34/s10`, `exited Some(1),
  expected 0` (`windows-rows.txt`). The next run 39176210454 held both
  rows.

## Not verified

- macOS and windows locally: neither was measured outside CI's samples
  lanes (the 589-program sweep and the probes are kasumi's alone). hasu
  was not used.
- Windows was never run by this lane outside CI: its rows are what the
  windows samples lane printed and holds. The `wolf run` consoles of
  §34.5 are not replayed there (that lane links no binary for a
  console), so they are linux's and macOS's.
- exit 71 on macOS and windows, and on the checked machine (which has no
  readiness set or hub to make): measured on linux only.
- A stop by `suspend()`, `bg_read()` or `bg_write()`, a `continued()`,
  `os_signal_send_all` with a real meaning, `os_set_pgid` to another
  group, a host number `-N` sent, and `os_signal_text`'s words: taught
  in prose from the clauses, not run (no program here sends a signal
  outside the child it started).
