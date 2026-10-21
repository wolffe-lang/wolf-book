# bs70 — results against the prediction

Crawl 54, 2026-10-21. The prediction is `0166b29`
(`docs/audit/bs70-prediction.md`), committed before either archive was
downloaded and before the first change. Everything measured on kasumi ran
inside the container (`ctr.sh`: rootless podman, its own pid namespace,
`--network none`, RLIMIT_CORE 0 soft and hard; SigBlk and SigIgn 0 at
every job's head); macOS and windows are the book's CI samples lanes.
Logs under `docs/audit/bs70-evidence/kasumi/`.

## Held

- **The pin** (§1): `[wolf]` `9da7bdcf`, `[lupin]` and `[lupin-trace]`
  `e642f26` (0.1.55), placetrace.rs byte-identical, the traces re-taken
  byte-identical (`diagrams --check`: 6 from 4).
- **The stamps, to the character** (§2): `wolf 0.2.34 (wolfgang, pin
  9da7bdc)` / `paired with lupin 0.1.55 (reference interpreter), pin
  294d626` / `lupin 0.1.55 (wolf-interp, reference interpreter at pin
  294d626)`; ten releases and 1067 commits.
- **Spec** (§3): anchors 664 → 672, the eight predicted, none dropped or
  retargeted (`anchors-diff.txt`); grammar, catalogue and `--help`
  unchanged; no W0304 or W0603 move.
- **Members** (§4): both archives `MATCH` on the API digest and
  `sha256sum -c` OK on a second download, the unpacked trees identical
  (`subject-download*.log`); `std/` 58 files, 54 `.lu`, STD-REV
  `e036bec3…`, differing from 0.2.33's in exactly the five predicted
  files; **the manifest `5d6253b6…`, as computed beforehand from wolf-std's
  objects**; `_wolf`, `wolf.bash`, `wolf.fish` and the licences identical;
  `wolf`, `wolf.1`, `wolf-cimport-worker`, both runtimes and `README.md`
  moved; highest `GLIBC_2.34` for `wolf` and `lupin`
  (`subject-members.txt`).
- **The subject run** (§5), the unedited tree on the new pair: **531
  samples, 528 passed, 3 pending, 4 failed, 0 flips; 496 of 520 console
  blocks**, the four drifts exactly ch01:153, colophon:7, ch22:288 and
  22-13 (EXERCISES.md:218), text only, every ch22 hash held; the same
  under `taskset -c 0-3`; 4 SKIP lines as the control
  (`subject-samples*.log`). `verify-docs`, `ledger --check` (129 / 119 /
  10, 110 closed) and `diagrams --check` exit 0 on the unedited tree;
  `backmatter --check` red on `anchors.json` alone until the pin commit
  re-vendored it.
- **The move table** (§6): below; lupin, native and checked 0.
- **The new pages' programs** (§7), every line on every machine as
  predicted (`probes-draft.txt`): `fork.lu` and `busy.lu` print the
  pages' lines natively, released and under `taskset -c 0-3`; the checked
  machine answers `unsupported` at each fork (`fork.lu` `error:
  unsupported`, `exit(1)`; `busy.lu` both lines `unsupported`, `exit(0)`:
  the medium-confidence call held); `wait.lu` and `save.lu` print the
  same lines on native, release and checked; lupin stops at the first
  name it does not resolve (`os_fork`, `fs_sync`, the `std.signal`
  import). On the control pair every one of the four is E0301
  (`probes-draft-c33.txt`).
- **wolf-book#87** (§7): `128 + N`. Measured in five shells on two
  images: `$?` reads 143 after an end by TERM in bash, sh, dash and zsh
  (arch) and dash and bash (ubuntu 24.04); bash and dash also print a
  `Terminated` notice, sh and zsh `-c` none (`probes-again-*.txt`,
  `shells-ubuntu.txt`). The test was red before the fix on kasumi and in
  the plant run, the console row likewise (below).
- **solutions.md** moved by exactly one line (22-13's toolchain).
- **Ledger** 129 open (119 filed, 10 waived), 110 closed: chapter 34's
  rows extended, none added open.

## Missed

- **§9's console count**: predicted 525 blocks / 505 replayed, measured
  **526 / 506**: §34.7's checked transcript is a block of its own, not a
  second command in `busy.lu`'s block, because `wolf conform-run
  --checked` without `--json` prints the verdict line alone, which the
  console replay drops (`checked-console-busy.txt`): a block of that
  command would show the reader nothing. The page prints the
  `--json` record instead. I had not measured the reader's half before
  predicting the block's shape.
- **Release noise**: predicted "the control's 6, give or take"; measured
  7 moves in each of the two cross-pair comparisons, two of them (ex11-5,
  ex17-1) outside both pairs' own two-sweep noise. Forty release runs of
  each, per pair, plain and under `taskset -c 0-3`
  (`noise-release-races.txt`), put both inside one distribution on both
  pairs (ex17-1 `balance=50` 23/40 vs 19/40 plain; ex11-5's most common
  order 22/40 vs 24/40): races the chapters teach as races, not moves.
- **Windows' `save.lu`**: predicted `the temporary name, made again:
  exists` then `error: unsupported` at `fs_sync_dir` (low confidence);
  it ends one call sooner, at `fs_open_perm`, which windows answers
  `unsupported` by name (no permission bits, `[os.fs.create.mode]`), with
  no line printed. I had read the save's new calls' windows rows and not
  the old call's. The row says so, and §34.8 gains one parenthesis for
  the exclusive open without permission bits (`fs_open_mode(path, 4)`).
  `fork.lu` (`error: unsupported`), `wait.lu` (`error: io`) and `busy.lu`
  (exit 0, no row) as predicted.
- **A probe script's bug**, not a prediction: the first `again.sh` read
  `$?` after an `echo` and printed 0; fixed (`88f2624`) before any
  record was taken from it.

## The move table (0.2.33 / 0.1.54 → 0.2.34 / 0.1.55, the unedited tree, 596 programs, each pair swept twice)

| machine | moves (a, b) | outside the noise |
|---|---|---|
| lupin | 0 / 0 | 0 |
| native (`conform-run --json`) | 0 / 0 | 0 |
| checked (`conform-run --json --checked`) | 0 / 0 | 0 |
| release | 7 / 7 | 0 (B-11's exit, ex9-4's exit, ex11-5, ex17-1, ch17:29, :113, :325, ex30-5, ch10:84: each a race inside its own distribution on both pairs) |

## The head

- **Gate at `2ffebe3`** on kasumi, inside the container, against the
  archive whole (`kasumi/gate-2ffebe3/`): every gate exit 0 — samples
  **535 / 532 passed / 3 pending / 0 failed / 0 flips, 506 of 526**
  console blocks (249 of 251 book, 257 of 275 corpus), the same under
  `taskset -c 0-3`; `samples --self-test`, `verify-docs`, `ledger
  --check` (129 open: 119 filed, 10 waived; 110 closed), `backmatter
  --check` (the vendored spec matching the v0.2.34 sibling), `contrast`,
  `tiers --check`, `render web`, `diagrams --check` with the 0.1.55 trace
  lupin; `cargo test -p xtask -- --nocapture` 206 passed. SKIP lines
  counted: samples 4 (as trunk's), self-test 1, the rest 0. A bare `wolf`
  (no `std/` beside it), for the record: 26 failures, every one a
  `use std.…` program or its console (`gate-samples-bare.log`).
- **The rig** (`kasumi/rig-work/`): fmt, clippy `-D warnings` and the
  xtask tests (206) green with #87's fix; at the plant head the one new
  test red (`plant-4deb627/`).
- **The head sweep** (600 programs, the new pair) against the unedited
  tree's: matched by program text, 586 identical records, the five races
  above moved, the four new programs only at the head, none gone
  (`sweep-newa-vs-head-2ffebe3-bytext.txt`); by slug, chapter 34's
  programs below §34.3 show as moved lines, not moved answers.
- **Gate at `9ed3f17`** (the content head: `2ffebe3` plus §34.8's windows
  parenthesis, the three windows rows and the ledger's citation):
  every gate exit 0 again (`kasumi/gate-9ed3f17/`), samples 535 / 532 /
  3 / 0 / 0 flips and 506 of 526, the same under `taskset -c 0-3`, the
  xtask tests 206, SKIPs as at `2ffebe3`. The final head adds only
  `docs/audit/` (this file and the evidence), which no gate reads.

## CI

- **Plant**, run 39685261147 at `4deb627` (`tier pr`): samples (ubuntu,
  job 119517678415) red on exactly the four predicted consoles and
  §34.6's #87 row (`-1` where the page says `143`); test (ubuntu, job
  119517678473) red on exactly #87's test; the staged std at
  `5d6253b6…` (`red-39685261147.txt`, `ci/plant-*`). Cancelled by my next
  push after both reds had answered, its contents jobs still running.
- **`tier all`** dispatch, run 39687320723 at `2ffebe3` (every job on
  every host, for the windows rows and the macOS samples the merge
  leaves to the nightly): samples ubuntu and **macOS 535 / 532 / 3 / 0 /
  0 flips, 506 of 526** (fork, busy, wait, save and #87's `143` on
  macOS); **windows red on exactly `book/ch34/s13`, `s15` and `s16`**
  (exit 1 each; 519 passed, 428 of 526 consoles, the checked record
  among them), before their rows (`ci/all-39687320723-*`); the rig's
  tests green on all three hosts.
- At the final head: `tier pr` and the labelled `tier full` (the PR
  body has the ids).
