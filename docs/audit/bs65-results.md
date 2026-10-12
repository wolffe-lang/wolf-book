# bs65 — results against the prediction

The book at wolf 0.2.28 (wolf-lang `f71ece8e`, release 410118732) / lupin
0.1.51 (wolf-interp `f72e78d7`, release 409991647), from 0.2.26 / 0.1.49.
Prediction `94acb67`, committed before either archive was downloaded on
any host. Three hosts measured every program: kasumi and hasu (linux
x86-64, the gates and the sweep) and nomad-1 (macOS arm64, the sweep
only, no cargo build). Evidence per host under `bs65-evidence/<host>/`.

## The move table: every program, 0.2.26 / 0.1.49 against 0.2.28 / 0.1.51

580 programs (`allprogs.py`: every book fence outside `back/`, `part()`
fences assembled, every corpus `.lu`), each alone in its own directory,
on lupin, `conform-run --json` native and `--checked`, and a `--release`
build and run for every native `pass` with a `main` (371). Each pair swept
twice on each host (`sweep-{c26,new}{a,b}.jsonl`), noise in
`sweep-*-noise.txt`, the comparison in `sweep-c26-vs-new.txt`, classed
against the prediction by `analyze.py` (`analyze-c26-vs-new.txt`). **The
three hosts agree row for row**: lupin 0, native 2, checked 96 on each.

| program(s) | machine | 0.2.26 / 0.1.49 | 0.2.28 / 0.1.51 | why |
|---|---|---|---|---|
| **ex16-7, ex16-8** | checked | `unsupported` «structured concurrency in checked execution (C1 deferred)» | **`exit(0)`, `distance=18` and `distance=10`**, unseeded and `--seed=7` (`"seeded": true`), lupin's and native's bytes | s226, ruling #63; as predicted (`probes.txt` on three hosts: lupin, `lupin run --seed=7`, `wolf run`, native, checked twice, release, all `distance=18` / `distance=10`) |
| 79 more C1 and closure rows (`predicted-checked.txt`) | checked | `unsupported` (C1 or «closures in checked execution») | lupin's verdict class and, on `exit(0)`, lupin's stdout byte for byte | s226; as predicted |
| ch15's three `run(exit=nonzero)` programs (§15.1 `ch15.md:37`, `:157`, ex15-2) | checked | `unsupported` (C1) | `exit(121)`; lupin exits 1 | ruling #63 **R7** (exit 121 when the root domain is killed; lupin 1), a parting the 0.2.28 notes name and my prediction did not list; the directive holds on every machine |
| **ex33-7** | checked | `unsupported` (C1) | **no record: killed at 120 s** (both sweeps, three hosts) and at 200 s (nomad-1 probe) | **not predicted, a finding**: the checked machine does not finish the program (`main` loops on `net_wait` while two spawned tasks are the callers); lupin and native answer `answered 4 questions on 2 connections` at once. Cause not isolated (Not verified) |
| 8 `freeze` rows (ch10 `:29`, ch13 `:321`, ch16 `:193`, ex16-4, ch30 `:260`, ex30-3/4/5) and 3 `List`-method rows (ch13 `:75`, ex13-1, ex13-6) | checked | `unsupported` «freeze of a non-region value» / «this List method» | lupin's verdict and bytes | not predicted: the 0.2.28 notes say `freeze region { … }` runs on the checked machine now and I read it as concurrency-only |
| ch16 §16.1's ring (`ch16.md:13`/`:27`, `part(ring)`, `lupin-run(exit=0)`) | native, checked | `fail(E1010)` ×1 | `fail(E1010)` ×2 | **the E1010 finding**: s229's spawn rule refuses `docs` at `spawn proc walker(…)` although the next line sends `store` to that proc; verdict and directive unmoved; ledger row (`bfc062e`) |
| ex5-8 (pending) | native, checked | `unsupported` «member access on a value whose type is still being inferred» | **from the archive whole**: `fail(E0401)` + E0403; **from a bare `wolf`**: unchanged | not predicted: the archive's `std/` resolves `sorted_by`, and the comparator answers `int` where `bool` is wanted (a third blocker); `samples-pending.toml` (`6a0821a`) |
| every other program | all four | — | — | no verdict, stdout, code or lupin exit moves |

Release rows that move are the scheduler's and UB's and each is noise on
some host or in bs64's list: B-11 and ex9-4's exit after UB, ch17's
balances (`ch17.md:113` moved outside kasumi's own noise, inside hasu's),
ex17-1, ex13-4a, ex30-4/5's arrival order, ch10's first scope program
(`ch10.md:84`, outside nomad-1's noise, inside kasumi's).

## Samples runner, both ways

| run | samples | console blocks |
|---|---|---|
| control (c26, unedited `f8e73fa`), kasumi | 515 / 512 / 3 pending / 0 failed / 0 flips | 488 of 508 |
| control, hasu, no `ld.lld` on PATH | 515 / 512 / 3 / **20 failed** | 468 of 508 — every drift the linker note, nothing else (`hasu/control-c26-samples.log`) |
| control, hasu, an lld already in the nix store on PATH | 515 / 512 / 3 / 0 / 0 | 488 of 508 (`hasu/control-c26-samples-lld.log`) |
| subject (unedited `94acb67`), archive whole, kasumi and hasu | 515 / 512 / 3 / **7 failed** / 0 flips | 481 of 508 |
| subject, bare `wolf` (CI's shape before the ruling), kasumi and hasu | 515 / 512 / 3 / **7 failed** / 0 flips | 481 of 508 |
| head `e6fc25f` (before the ruling), bare, kasumi and hasu, also under `taskset -c 0-3` | **515 / 512 / 3 / 0 / 0 flips** | **488 of 508** |
| head `e6fc25f` (before the ruling), archive whole, kasumi and hasu | 515 / 512 / 3 / **1 failed** (`ch22.md:288`, the archive's interface above) | 487 of 508 |
| **head `65a2ca3` (after the ruling), archive whole, kasumi and hasu, also under `taskset -c 0-3`** | **515 / 512 / 3 / 0 / 0 flips** | **488 of 508** |
| head `65a2ca3`, bare (for the record) | 515 / 512 / 3 / 1 failed (`ch22.md:288`, the bare listing) | 487 of 508 |

Head gate at `e6fc25f` on kasumi and on hasu (`<host>/gate-e6fc25f/`,
`gate-summary.txt`): `samples`, `samples --self-test`, `verify-docs`,
`ledger --check` (126 open, 119 filed, 7 waived, 0 unfiled; 109 closed),
`backmatter --check` against wolf-lang at v0.2.28, `contrast`, `tiers
--check`, `render web`, `diagrams --check` with the release lupin as
`LUPIN_TRACE` (6 from 4), `samples` under `taskset -c 0-3`, and `cargo
test -p xtask -- --nocapture` (203 passed): every one exit 0 against the
bare `wolf`. SKIP lines counted per log: samples 4 (the solutions.md
extraction, two console blocks that need a shell, the REPL blocks
pending is08), self-test 1, every other gate 0. The first head gate, at
`9447762`, was red on two lines (`ch11/EXERCISES.md:166`, `:167`): the
re-worded header had grown a line under a `samples-declined.toml` row
keyed by line; `e6fc25f` keeps the header to three lines.

The seven are the six predicted (ch01 and colophon `--version`, ch22 and
22-13's interface, ch24's `--explain E1504`, appendix E's `wolf --help`)
and **one not predicted: exercise 5-8's lupin transcript**, whose
std-root message gained s204's words (a `std/` beside the binary). Whole
and bare fail the same seven blocks, but **ch22's `wolf interface`
answers differently**: bare prints the old hashes with `toolchain
0.2.28`; the archive whole prints a `module pkg :: std.range` block and
moves both of the root's hashes `b7a67a58…` → `c8e04b6d…` (`xs.is_empty()`
resolves into the std; `probes2.txt`). Before the ruling the page printed CI's (bare) run;
the ch22 export-hash row records the archive's.

## Predicted, and what happened

| prediction | result |
|---|---|
| stamps to the character | as predicted, three hosts |
| 541 commits, four releases | as predicted; ch01, colophon (`540aabf`, `b4bd0f4`) |
| anchors 607 → 634, none dropped; grammar unchanged; catalogue 187 | as predicted (`anchors-diff.txt`; `backmatter --check` clean) |
| members: + `std/` (53 `.lu`, 57 files, `98e1b7bc`); `_wolf`, `wolf.bash`, `wolf.fish`, licences = 0.2.26's | as predicted |
| GLIBC ≤ 2.34 | 2.34 on both |
| subject: 6 console drifts, 512 / 3 / 6, 482 of 508, the same on bare | **7**, 481 of 508: the seventh is 5-8's lupin message (missed: I scanned lupin for builtins, not messages) |
| "no book program writes `use std`, so the archive's std reaches no sample" | **wrong**: a method call reaches it — ch22's interface and ex5-8 answer differently with `std/` beside `wolf` |
| E1010: no program newly refused | **held for verdicts, not for diagnostics**: §16.1's ring gains a second E1010 |
| checked: exactly the 83, lupin's class and bytes | 83 moved: 79 as predicted, ch15's three `exit(121)` (R7, a named parting I left off), ex33-7 past every timeout; **plus 13 not predicted** (11 `freeze`/`List`-method rows to lupin's bytes, ex5-8, the ring) |
| ex16-7, ex16-8 `exit(0)`, `distance=18` / `distance=10` | as predicted, every machine, three hosts |
| lupin 0 rows, native 0 rows, release noise | lupin 0; native **2** (ex5-8, the ring); release noise |
| traces byte-identical, diagrams 6 from 4 | as predicted (kasumi, hasu) |
| ledger 126 / 109 unchanged; verify-docs ok after re-read | as predicted |

## Sentences fixed (each where an answer changed)

- ch01 §1.2 and the colophon: the stamps, four releases and 541 commits,
  the folder holds `std/` (`540aabf`, `b4bd0f4`).
- ch22 §22.2: `toolchain 0.2.28`; the export-hash row records the
  archive's moved hashes (`e7c5ce9`).
- ch24 §24.3: the explain names every kind of reach now (`8878606`).
- exercises ch11 header: the checked machine runs the four programs and
  answers as lupin (`0a43019`).
- ch25: `use std.io` resolves from the archive; §25.2 stays held, the
  HOLD note re-measured (`fc4b738`).
- ch16 ledger: the ring's second E1010 (`bfc062e`); pending 5-8's row
  (`6a0821a`).
- The version-literal ledger re-read (`669998b`); `solutions.md`
  regenerated (`9447762`); seven consoles spliced from the bare run
  (`f0b14f2`).

## Findings for the orchestrator to file (the lane files nothing)

1. FILED as wolf-lang#721 by the orchestrator: `wolf interface` on an unedited file moves its
   `export_hash` and `pkg_hash` with the presence of the archive's
   `std/` (ch22's `summary.lu`, `b7a67a58…` vs `c8e04b6d…`; one
   `xs.is_empty()` is enough, and the std module is listed as
   `module pkg :: std.range`). `probes2.txt`.
2. FILED as wolf-lang#720 by the orchestrator: the checked machine does not finish ex33-7 (killed at
   120 s and 200 s on three hosts) where lupin and native answer at
   once; a `net_wait` loop in `main` with two spawned callers.
3. wolf-lang: s229's spawn rule refuses a spawn argument whose region is
   sent to the spawned proc on the next line (§16.1's ring) — within
   ruling #66's accepted refusals, a witness if a transfer-aware rule is
   ever written.
4. wolf-book: exercise 5-8's comparator answers `int` where std's
   `sorted_by` wants `bool` — a third blocker, visible only with a std.
5. Close: wolf-lang#639 (E1504's explain now names every reach; measured
   in ch24's regenerated block).

## Seen red

Plant `a283ac4` put ch01 §1.2's `--version` block back to the 0.2.26 /
0.1.49 stamps, a block only a console replay against a 0.2.28 build can
see. Run 38213474972 (PR #77): `samples (ubuntu-latest)` 114692705686,
`samples (macos-latest)` 114692705572 and `samples (windows-latest)`
114692705569 each red with exactly one failure, `book/ch01.md:153:
console block drifted from the real run` (`plant-red-38213474972.txt`).
Reverted at `9115f09`. The plant run also gives the CI counts with one
console failing: ubuntu and macOS 512 / 3 / 1, 487 of 508; windows 511 /
3 / 1, 424 of 508 (windows' declared prefork refusal and its off-lane
consoles, as at trunk). CI at `e6fc25f`, the head before the plant:
run 38209773182, samples green on all three hosts (ubuntu and macOS 512
/ 3 / 0, 488 of 508; windows 511 / 3 / 0, 425 of 508).

## Not verified

- The cause of ex33-7's hang on the checked machine (only that it does
  not finish in 120 s or 200 s, on three hosts).
- Whether ch22's moved export hash is a compiler defect or the format's design (wolf-lang#721; the book now prints the archive's listing either way).
- linux aarch64 and windows were not measured locally (CI's windows
  samples job only); hasu needed an lld already in its nix store on
  `PATH` (no package installed), without which every linking console
  block carries the linker note (20 drifts, `hasu/control-c26-samples.log`).
- The release machine on nomad-1 ran `wolf build --release` on book
  programs (the sweep), which is a wolf compile, not a cargo build.

## The ch22 ruling (the orchestrator, after the first report)

"Since 0.2.28 the archive ships std beside `wolf` and that is what a
reader installs, so the book must print what a reader sees." Done as
ruled:

- **CI stages `std/` beside its source-built `wolf`** exactly as `cargo
  xtask dist` stages the archive's (wolf-std at wolf-lang's
  `crates/wolf_driver/STD-PIN`, `98e1b7bc…`; the two licences;
  `STD-REV`), and the step fails unless the staged tree's manifest is
  the 0.2.28 archive's, `98e9a178…`, measured on three hosts
  (`a130fd3`, `65a2ca3`).
- **Seen to take effect**: run 38221392065 at `a130fd3` (the step, ch22
  still the bare listing): ubuntu (114718236024) and macOS
  (114718235945) staged `98e9a178…` and went red at exactly
  `ch22.md:288` with the `module pkg :: std.range` listing; windows
  (114718235921) staged the same 57 files and computed `ca3b6e76…`,
  because its `sha256sum` marks binary mode in each line;
  `65a2ca3` writes each manifest line by hand and windows' job
  114723072698 in run 38222853235 stages `98e9a178…`
  (`stage-red-38221392065.txt`).
- **ch22**: the interface block spliced from the archive whole
  (`2c44d2c`, from `kasumi/restage-samples-archivewhole.log`); §22.2's
  prose reads the listing — the first module is `std.range`, which
  `mean`'s `xs.is_empty()` reaches; the hash covers what the file
  exports and what it uses, a std module reached through a method call
  included, so a `wolf` with no `std/` beside it reads another number
  (`dbc97bb`); the export-hash ledger row records the ruling and
  wolf-lang#721. **22-13's block was already the archive's answer**
  (`tokens.lu` reaches no std module; identical whole and bare).
- The records that said CI has no std now say it stages one: the pin
  notes, ch25's HOLD note, pending 5-8, ch01 §1.2's build-from-source
  paragraph (`ba9c312`, `0acc217`).
- **Head gate at `65a2ca3`** on kasumi and hasu (`<host>/gate-65a2ca3/`)
  **against the archive whole**: every gate exit 0 — samples 515 / 512
  / 3 / 0 / 0, 488 of 508, also under `taskset -c 0-3`; self-test;
  verify-docs; ledger 126 open (119 filed, 7 waived, 0 unfiled) / 109
  closed; backmatter --check against v0.2.28; contrast; tiers --check;
  render web; diagrams 6 from 4 with the release lupin; xtask tests 203
  passed with `--nocapture`. SKIPs: samples 4, self-test 1, others 0.
  The bare `wolf` now fails exactly `ch22.md:288`, as it should.
  (hasu's `gate-0acc217` collided with a re-stage run in the same
  clone and is not evidence; it is not committed.)
