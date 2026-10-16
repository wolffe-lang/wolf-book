# bs67 — results against the prediction

The book at wolf 0.2.30 (wolf-lang `1b37f368`, release 413582739) / lupin
0.1.53 (wolf-interp `7896edd`, release 413375414), from 0.2.29 / 0.1.52,
and chapter 34 (wolf-book#79) and wolf-book#80. Prediction `27d6dff`,
committed before either archive was downloaded and before the first
change. One host measured every program locally: kasumi (linux x86-64,
16 cpus), from the release archives by digest, each digest read twice
(`kasumi/subject-download.log`, `subject-download-twice.log`), **every
program run inside a container** (`ctr.sh`: rootless podman, the lane's
uid, its own pid namespace, `--network none`; SigBlk and SigIgn 0 at the
head of every container log). macOS and windows are the book's CI
samples lanes, which run in full. Evidence under `bs67-evidence/`.

## The move table: every program, 0.2.29 / 0.1.52 against 0.2.30 / 0.1.53

580 programs (`allprogs.py`), each alone in its own directory, on lupin,
`conform-run --json` (native), `conform-run --json --checked`, and a
`--release` build and run for every native `pass` with a `main` (371).
Each pair swept twice (`kasumi/sweep-{c29,new}{a,b}.jsonl`), noise in
`sweep-*-noise.txt`, the comparison in `sweep-c29-vs-new.txt`: **lupin 0,
native 0, checked 0, release 7.**

| program(s) | machine | 0.2.29 / 0.1.52 | 0.2.30 / 0.1.53 | why |
|---|---|---|---|---|
| every program | lupin, native, checked | — | — | no exit status, verdict, stdout, code or refusal moves |
| **ex33-7** | checked | no record, killed at 120 s | **no record, killed at 120 s** (both sweeps) | wolf-lang#720, open |
| B-11, ex9-4 | release | an exit status after UB | another | UB's; inside the control's own noise |
| ex11-5, `ch17.md:29`, `ch17.md:113`, ex30-4, ex30-5 | release | one schedule's order or balance | another's | the scheduler's: each prints arrival order or shows a race; all in bs66's named set |

No sample moved its verdict, stdout or diagnostic on any of the four
machines. s233's must-join rule (the row I was least sure of) refused no
book program: no new E1010, and every `fail(E1010)` block's diagnostic
held. Four console transcripts moved their text.

## Samples runner

| run | samples | console blocks |
|---|---|---|
| control (0.2.29 pair, unedited `86db060`), kasumi | 515 / 512 / 3 pending / 0 failed / 0 flips | 488 of 508 |
| subject (0.2.30 pair, unedited `27d6dff`), archive whole | 515 / 512 / 3 / **4 failed** / 0 flips | 484 of 508 |
| subject, bare `wolf` | 515 / 512 / 3 / **4 failed** / 0 flips | 484 of 508 |
| head `fdfc50a`, archive whole, kasumi, also under `taskset -c 0-3` | **524 / 521 / 3 / 0 / 0 flips** | **496 of 516** |
| head `94ee637` (the book tree of record), archive whole, kasumi, also under `taskset -c 0-3` | **524 / 521 / 3 / 0 / 0 flips** | **495 of 515** (`window.lu`'s console folded into its fence) |
| head `94ee637`, bare (for the record) | 524 / 514 / 3 / 14 failed | 488 of 515 |

The four are the four predicted: `book/ch01.md:153` and
`book/back/colophon.md:7` (`--version`), `book/ch22.md:288` and
`principles/exercises/ch22/EXERCISES.md:218` (`wolf interface`, the
`toolchain` line and nothing else; every hash held). Spliced from the
run's `actual:` by `splice.py` (`kasumi/subject-samples.log`), never
typed. The bare `wolf` at head fails every chapter-34 program that
imports std (E0301, exit 2: no `std/` beside it), their console blocks,
and ch22's listing; CI stages the archive's `std/`, so the archive whole
is the toolchain of record, as bs65 ruled.

Head gate at `94ee637` (`kasumi/gate-94ee637/`, `gate-summary.txt`; and
before the windows rows at `fdfc50a`, `kasumi/gate-fdfc50a/`), in
the container, against the archive whole: `samples`, `samples
--self-test`, `verify-docs`, `ledger --check` (130 open: 119 filed, 11
waived, 0 unfiled; 109 closed), `backmatter --check` against wolf-lang at
v0.2.30, `contrast`, `tiers --check`, `render web`, `diagrams --check`
with lupin 0.1.53 as `LUPIN_TRACE`, `samples` under `taskset -c 0-3`,
and `cargo test -p xtask -- --nocapture` (203 passed): every one exit 0.
SKIP lines: samples 4 (the `solutions.md` extraction, two console
blocks that need a shell, the REPL blocks), self-test 1, every other
gate 0.

## Predicted, and what happened

| prediction | result |
|---|---|
| stamps to the character | as predicted (`kasumi/subject-members.txt`) |
| 717 commits, six releases | as predicted; ch01, colophon |
| anchors 640 → 648, eight named, none dropped; grammar unchanged; catalogue 187 | as predicted (`anchors-diff.txt`; `backmatter --check` clean) |
| members: 0.2.29's names; `std/` 57 files, 53 `.lu`, `fd6bcdcb`; only `env`, `signal`, `term` and `STD-REV` differ; licences and (a guess) the three completions identical | as predicted, the guess included; manifest `ab2576df…` |
| GLIBC ≤ 2.34 | 2.34 on both |
| the CI std step red at the bare pin until the manifest is written | **not seen**: the measured manifest was committed (`80e48eb`) before the first push, as at bs66 |
| subject: 4 console drifts, 512 / 3 / 4, 484 of 508, the same four on bare | as predicted |
| lupin 0, native 0, checked 0, release noise; ex33-7 still killed | as predicted; release 7 rows, every one in the named set |
| chapter 34's nine programs on four machines (§7) | **as predicted in every cell** (`kasumi/probes.txt`; the draft run `probes-draft.txt`): native, checked (`conform-run --json --checked`), release and `taskset -c 0-3` print the predicted lines; lupin answers `unsupported` exit 4 on all nine, by name at `os_signal_send` and `os_term_size` and at the first unresolved std name elsewhere |
| the lupin refusal's words | 0.1.52's words plus a clause suffix: «… std has no `kill(pid, sig)` ([os.signal.send], [os.term.note] R8)»; spliced from the run (`kasumi/work-samples-splice.log`) |
| head 524 samples, 521 passed, 0 failed | as predicted; console blocks 515, 495 replayed on kasumi (516 and 496 before `window.lu`'s console folded into its fence) |
| §13.1 on kasumi: 0.2.15's allocating `par` slower, 0.2.30's at least 3x faster, the other rows faster on both | as predicted: 0.27 s serial against 0.90 s `par` (8.41 s system) on 0.2.15; 0.26 s against 0.03 s on 0.2.30; the other two rows 7x and 10x on both (`kasumi/par-median.txt`) |
| windows: `reload.lu`, `sets.lu`, `modes.lu` pass; the others answer a row | as predicted for the six blocks and 34-7, with `ex34-1` passing too (its drafting change, below); **one miss**: `window.lu`, then claiming no stdout, was a FLIP on windows, because there `lupin` answers the `[os.term]` calls' `unsupported` row itself (exit 0) instead of refusing by name (run 38956581744, job 117160860322). The fence now claims its one line, and windows declares it. The windows `lupin alive.lu` console prints `error: unsupported`, which I had not predicted at all |
| CI on the unedited book at the pin: red on three hosts on exactly the four | as predicted: run 38952349140 at `80e48eb`, ubuntu and macOS 512 / 3 / 4 and 484 of 508, windows 511 / 3 / 4 and 421 of 508, each on exactly the four; the staged `std/` held to `ab2576df…` on all three |

One prediction was changed by a drafting decision before the first
change, and is recorded here rather than hidden: exercise 34-1 listens
for `terminate()` and `quit()` (predicted with `interrupt()` and
`terminate()`), so that windows, where a listen for `interrupt()` is
`io`, runs it too. It prints `arrived: quit`.

## Chapter 34 (wolf-book#79)

`book/ch34.md`, *Signals and the process*, at the end of Part 4 by
chapter 33's rule. Four sections and seven exercises:

- **34.1 A signal is an event**: listen, raise, wait and poll; ignore and
  reset; what a child inherits; the twenty-eight names in a table (name,
  what happened, the POSIX name; no bit, no number); the shape rule; a
  published bit's permanence. Programs: the cold open (`reload.lu`,
  `wolf-run(exit=0, stdout="heard reload: true")`) and `sets.lu`
  (`wolf-run(exit=0)` and its console).
- **34.2 Sending, numbering, and what was ignored**: `signal.send` and its
  rows in order, `signal.number` and the three places a number is the
  interface, `signal.ignored` and `nohup`; the interpreter's refusal of
  `os_signal_send`, by name, replayed (`alive.lu` and its `lupin`
  console). Programs: `rows.lu` and its console, `alive.lu`.
- **34.3 Who this process is, and its terminal**: `env.pid`, `env.ppid`;
  `os_uid`, `os_euid`, `os_gid`, `os_egid`, `os_groups` as builtins;
  `fs_set_owner`, `fs_set_owner_fd` and `os_set_ids` in prose (no program
  changes an id or an owner); `std.term`'s `Mode` with `flow` and
  `extended`; `os_term_size`, `os_term_chars`, `os_signal_wait_fd` as
  builtins. The page says plainly, twice, that the standard library this
  toolchain ships has no wrappers for those calls and that they ship with
  the next release (the contract's sentence; the chapter's ledger records
  its tension with TONE.md). Programs: `who.lu` and `modes.lu` with
  their consoles, `window.lu` with its one line claimed in the fence.
- **34.4 What a call charges, and where it is refused**: the capability
  table, read from the compiler at v0.2.30 (`wolf_sema`'s sandbox
  categories, `wolf_driver`'s `builtin_cap`): `exec` for every disposition,
  wait, send and the drop; `env` for this process's ids, its parent, what
  it ignores and its group; `fs` for the owner calls; nothing for the
  numbering and the terminal reads. What lupin and windows decline.

| program | native | checked | release | lupin 0.1.53 | windows (CI) |
|---|---|---|---|---|---|
| `reload.lu` | `heard reload: true` | the same | the same | `unsupported`, no std | passes |
| `sets.lu` | `true`, `true`, `io`, `io` | the same | the same | `unsupported`, no std | passes |
| `rows.lu` | `ok`, `invalid`, `not_found`, `true`, `true`, `false` | the same | the same | `unsupported`, no std | declared: exit 1, `unsupported`, `invalid`, `unsupported`, then `error: unsupported` |
| `alive.lu` | `this process may be signalled` | the same | the same | **refused by name** | declared: exit 1, `error: unsupported`; `lupin` prints the same row |
| `who.lu` | `true, true`, `false`, `false` | the same | the same | `unsupported`, no std | declared: exit 1, `error: unsupported` (`env.ppid`) |
| `modes.lu` | `false false true / false false 1 / true` | the same | the same | `unsupported`, no std | passes |
| `window.lu` | `size: io, keys: io` (standard output a pipe) | the same | the same | **refused by name** | declared: exit 0, `size: unsupported, keys: unsupported` |
| `ex34-1.lu` | `arrived: quit`, `still queued: 0` | the same | the same | `unsupported`, no std | passes |
| `ex34-7.lu` | `28 meanings, 28 numbered here`, `20 can be heard`, `two at once: invalid` | the same | the same | `unsupported`, no std | declared: exit 1, `error: unsupported` (`signal.number`) |

Every program signals only itself; the send to an id nobody holds sends
nothing (meaning 0). Each was also run under `taskset -c 0-3`, with the
same output.

## wolf-book#80

- **`net_wait`'s wording.** Exercise 33-7's solution said `net_wait`
  "neither parks a task nor disturbs the scheduler"; since 0.2.29 its poll
  runs compensated (s231, wolf-lang `3e242b81`): the pool is told its
  worker is blocked and keeps the other tasks running. Rewritten to say
  so (`2370779`). §33.3's parenthesis matched `[os.net.wait]` at v0.2.30
  word for word ("neither parks a task nor disturbs the reactor's
  registrations"); it keeps that and gains the pool's clause
  (`f161d59`). No program's answer moved.
- **§13.1's table.** It cannot be re-measured where it was measured (the
  book's machine, nomad-1, runs no builds), so it is **dated** (17
  September 2026, wolf 0.2.15). Its mechanism sentence ("the native root
  arena serializes allocation behind one lock (`[conc.task.par.cost]`)")
  had been false of the clause it cites since 0.2.27 (s222); it is in the
  past tense now, at 0.2.15. And the programs were re-measured on a
  second machine with a same-machine control: kasumi, the printed program
  and two rebuilt from wolf-lang#421's description, built by 0.2.15 and
  0.2.30, seven alternating rounds (`par.sh`, `par/`, `kasumi/par.log`,
  `kasumi/par-median.txt`; load 6–7 on 16 cpus throughout):

  | program | 0.2.15 serial | `par` | 0.2.30 serial | `par` |
  |---|---|---|---|---|
  | the printed counter | 0.27 s | 0.90 s (sys 8.41 s) | 0.26 s | 0.03 s (sys 0.02 s) |
  | one allocation removed | 0.14 s | 0.02 s | 0.14 s | 0.02 s |
  | computes only | 1.53 s | 0.15 s | 1.52 s | 0.15 s |

  That table is printed beside the dated one (`0f82ba3`). wolf-lang#421 is
  still open; at 0.2.30 its first row is the good case.

## Sentences fixed (each where an answer changed)

- ch01 §1.2: the stamps, `pin 1b37f36` twice, `lupin 0.1.53`, six
  releases and 717 commits, the two Windows archive names, the unstamped
  build's `0.2.30+dev.unknown`. Colophon: `lupin 0.1.53`, six releases,
  717 commits. ch22 §22.2: `toolchain 0.2.30`, and its ledger row.
- The version-literal ledger re-read; `solutions.md` regenerated
  (22-13's toolchain line, 33-7's wording, chapter 34's solutions); four
  consoles spliced; `wolf-toolchain.toml` and the CI manifest.
- What a new chapter must touch: SUMMARY, TOC.md (entry and §Deltas),
  how-to-read's late-chapter sentence, README's count, the exercise index
  (tiers regenerated) and the corpus counts in EXERCISES.md and
  EXERCISES-PENDING.md, and `backmatter.rs`'s chapter range (1..=34).

## Findings for the orchestrator (the lane files and closes nothing)

- **Close at merge:** wolf-book#79 (the page, and the spelling question
  answered by the release's std) and wolf-book#80 (both spots).
- wolf-book#45: chapter 34 is its third way out (a chapter grows a
  compiler sample needing a module the compiler serves); the issue's other
  two blockers (no wolf-std release to pin; lupin has no std) stand. Not
  closed.
- wolf-lang#421 is still open and its premise is gone at 0.2.27; the
  book's kasumi table is the before-and-after.
- wolf-lang#720 (ex33-7 on the checked machine) and #721 stay open and
  true at 0.2.30.
- At the next STD-PIN (wolf-std `f50fe73` or later), chapter 34's two
  "ship with the next release" sentences come out and the builtins in
  §34.3 can be taught through `env.uid` … `term.size`, `term.chars`,
  `signal.wait_fd`.

## Seen red

- **Run 38952349140** (PR #81) at `80e48eb`, the pin and the CI manifest
  with the book's text unedited: `samples (ubuntu-latest)` 117146592917,
  `samples (macos-latest)` 117146592589 and `samples (windows-latest)`
  117146592674 each red at "run every sample" with exactly four
  failures, `book/back/colophon.md:7`, `book/ch01.md:153`,
  `book/ch22.md:288` and `principles/exercises/ch22/EXERCISES.md:218`,
  each a console block drifted, 0.2.29's text expected and 0.2.30's
  actual (`red-38952349140.txt`). Every other job of that run was green.
- **Run 38956581744** at `8e4a444` (chapter 34 in, no windows rows yet):
  ubuntu and macOS samples green; `samples (windows-latest)` 117160860322
  red on exactly what samples-os.toml now declares (four programs
  exiting 1, the `lupin alive.lu` console's text) and one FLIP
  (`window.lu`). The next run held all seven windows rows
  (38959606063, job 117171053049: `7 row(s) apply to windows — 6
  declared refusal(s) and 1 declared transcript(s) held`, 515 / 3 / 0, 0
  flips, 426 of 515).

## Not verified

- macOS and windows locally: neither was measured outside CI's samples
  lanes (the 580-program sweep and chapter 34's probes are kasumi's
  alone). hasu was not used.
- Windows was never run by this lane outside CI: its rows are what the
  windows samples lane printed and holds. The `wolf run` console blocks
  of chapter 34 are not replayed there (that lane links no binary for a
  console), so they are linux's and macOS's.
- §13.1's dated table on its own machine (nomad-1 runs no builds), and
  the second and third rows' programs as the original lane wrote them:
  those are rebuilt from wolf-lang#421's description.
- A delivered signal from another process, `os_signal_wait_fd`, and
  `os_set_ids` / `fs_set_owner`: no program here exercises them.
- The cause of ex33-7's hang on the checked machine.
