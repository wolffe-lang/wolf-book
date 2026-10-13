# bs66 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-13 against wolf-book `307305d` (the contract, an
empty commit on bs65's merged trunk `b845e07`), **before**
`wolf-0.2.29-*.tar.gz` or `lupin-0.1.52-*.tar.gz` was downloaded on any
host and before any change to the book. DERIVED means computed from the
upstream git histories at the tags (private blobless clones of wolf-lang,
wolf-interp and wolf-std under `~/lanes/bs66/`); PREDICTED is a claim
about what the release binaries will do, with what falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.28 / lupin 0.1.51), fetched by digest from
the release API on kasumi (`bs66-evidence/setup.sh`): linux x86-64
`3215f321…` / `d8f62dd1…`, both `MATCH`. Its gates and its program sweep
on the unedited head `307305d` (`control.sh`) were still running when
this was written, so no control number is quoted here; bs65's head gate
at the same pin and the same book tree read **515 samples, 512 passed, 3
pending, 0 failed, 0 flips; 488 of 508 console blocks**; ledger **126
open, 109 closed**.

One host measures locally this time: kasumi (linux x86-64), gates and
sweep. The lane's brief keeps every build and every sample run off
nomad-1, so bs65's macOS sweep from the archives is not repeated; macOS
and windows are measured by the book's CI samples lanes, which run in
full. That is a narrower local measurement than bs65's and is said in
the results.

## 1. The pin (DERIVED)

```
[wolf]         rev f71ece8e1ad2234940040d300359888932ad67af -> 6ffe7c6056eeb8c661b37e38d99b91166e8c1add
               impl_version 0.2.28 -> 0.2.29
[lupin]        rev f72e78d7a9d0ceb39d80380389ad6a2072991c63 -> 025428b725a35bfbc53745d4166817ff3053c0fd
[lupin-trace]  rev f72e78d7a9d0ceb39d80380389ad6a2072991c63 -> 025428b725a35bfbc53745d4166817ff3053c0fd
```

One compiler release (0.2.29, release 411040788; 70 commits
`v0.2.28..v0.2.29`) and one interpreter release (0.1.52, release
410845419; 13 commits). The trace pin stays collapsed onto
`[lupin].rev`: `src/eval/placetrace.rs` is byte-identical between
v0.1.51 and v0.1.52 (the release touches `src/eval/builtin.rs` and
tests).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.29 (wolfgang, pin 6ffe7c6)
paired with lupin 0.1.52 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.52 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `6ffe7c60` (`lupin-version = 0.1.52`, `lupin-pin =
294d626`), `vendor/upstream/PIN` at `v0.1.52` (`294d626d…`) and D57.
**The interpreter's clause is now FIVE releases behind**: `294d626` is
v0.2.24's tag, an ancestor of `6ffe7c60`, **611** commits and five
releases (v0.2.25 … v0.2.29). ch01 §1.2's "from four releases back … you
get 541" and the colophon's matching sentence move to five and 611.
Falsified by any character of the stamps that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 634 → 640**, none dropped, none retargeted. Added (6):
  `os.proc.self`, `os.signal.ignored`, `os.signal.number`,
  `os.signal.send`, `os.signal.set.stable`, `os.signal.shell.note`.
  Appendix D's tags hold.
- **`grammar.ebnf` does not move** (byte-equal to the vendored copy at
  v0.2.29). Appendix A does not regenerate.
- **Catalogue 187 → 187**: `crates/wolf_diag` has no diff between the
  tags, so no `--explain` text moves either (ch24's E1504 block holds).
- `help.rs` moves only `wolf lsp`'s usage line and about paragraph
  (`--std-root`, s235). The top-level `wolf --help` (Appendix E) does
  not print either, so its block holds; the count of subcommands stays
  twenty-three.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.28's member names, `std/` included: 53 `.lu` files,
  57 files, `STD-REV` = `d7c2968cbba1a855a58b4aabf3e55edcf7282992`. The
  std tree differs from 0.2.28's in `std/env/env.lu`, `std/fs/fs.lu` and
  `STD-REV` only (wolf-std `98e1b7bc..d7c2968c`), so its manifest moves
  off `98e9a178…` and the book's CI staging step, which holds the staged
  tree to the archive's manifest, is **red at the bare pin** until the
  new manifest is measured from the archive and written. Byte-identical
  to 0.2.28's: `LICENSE`, `LICENSE-EXCEPTION`, the two std licences, and
  (a guess, flagged as one: I did not find where completions are
  generated) `_wolf`, `wolf.bash`, `wolf.fish`. Move: `wolf`, `wolf.1`,
  `README.md`, `wolf-cimport-worker`, `libwolf_rt.a`, `libwolf_rt_none.a`.
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE`, `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 515 samples: 512 passed, 3 pending, 4 failed; 484 of 508
console blocks**, against the archive whole. No snapshot and no
`diagnostic,from` block moves. The four failures are console blocks,
every one a drift of text, no verdict:

1. `book/ch01.md` `--version` (§2's stamps).
2. `book/back/colophon.md` `--version`.
3. `book/ch22.md` `wolf interface`: `toolchain 0.2.28` → `0.2.29` on
   both stamp lines. **The hashes hold** (`c8e04b6d…` for the root,
   bs65's archive answer): the listing reaches `std.range` and nothing
   else in std, and `std/range` has no diff between the two bundled
   revisions. Falsified by any other line of that block moving.
4. `principles/exercises/ch22/EXERCISES.md`, 22-13's interface: the
   toolchain line only.

On a **bare** `wolf` (no `std/` beside it): the same four, ch22's block
then printing the bare listing (bs65 measured that parting; it is why CI
stages `std/`).

What does **not** fail, by category, and why:

- **The five new prelude names** (`os_signal_ignored`, `os_signal_send`,
  `os_signal_number`, `os_pid`, `os_ppid`): no book program declares or
  uses any (grep over `book/`, `principles/`, `samples*`: 0; the
  CHANGELOG's own grep at `b845e077`: 0), so no W0304 and no new
  capability charge (R8).
- **The signal set** (twenty listenable, eight send-only, the one shape
  rule): no book program calls an `os_signal_*` builtin or imports
  `std.signal` (grep: 0).
- **s231** (a worker per parked task, parked waits that do not poll, the
  dial outside the socket lock): no book program parks more than eight
  tasks a core, and nothing a program prints depends on the pool's size
  (ch10: "unobservable"). ch33's seven network programs each hold a
  handful of connections.
- **s235** (`wolf lsp` finds std): the book replays no `wolf lsp`
  session and prints no `wolf lsp --help`.
- **Appendix E's `wolf --help`**, **ch24's `--explain E1504`** and
  **exercise 5-8's lupin message**: their sources have no diff (§3;
  lupin's diff is `builtin.rs` and tests).
- Pending **3 → 3** (ex5-8, ex7-5, ex8-7) in the same words.

## 6. The sweep: subject against control (PREDICTED)

580 programs (`allprogs.py`), each alone, on lupin, `conform-run --json`
(native), `conform-run --json --checked`, and a `--release` build and
run for every native `pass` with a `main`; each pair twice for the noise
floor.

- **lupin: 0 rows** outside the control's own noise.
- **native: 0 rows** (verdict, stdout, codes). s231 changes how the pool
  grows and how a parked wait is woken, so a program whose output is the
  scheduler's arrival order may print another order: that is allowed
  only for a row already inside the control's own noise or in bs65's
  named set (ch17's balances, ex17-1, ex11-5, ex13-4a, ex30-4/ex30-5,
  ch10's first scope program). Falsified by any other native move.
- **checked: 0 rows.** In particular **ex33-7 still does not finish**
  (killed at the sweep's 120 s on both pairs; wolf-lang#720 is open and
  no 0.2.29 lane names it — though s231 makes `net_wait` and the dial
  tell the pool they wait, which is the checked machine's neighbour, so
  this is the row I am least sure of). Falsified by any checked row that
  moves, ex33-7 included.
- **release: noise only** (bs65's set: B-11 and ex9-4's exit after UB,
  ch17's balances, ex17-1, ex13-4a, ex11-5, ex30-4/ex30-5's arrival
  order, ch10's first scope program).

## 7. The five builtins on four machines (PREDICTED from `spec/11-os.md` at v0.2.29 and the CHANGELOG; probes, not book samples)

The book has no page that teaches a signal or a process id (contract
commit, DRIFT), so these are measured for the no-page list and no sample
is added. On kasumi, each program alone in its own directory:

| call | lupin 0.1.52 | native | checked (`conform-run --json --checked`) | release |
|---|---|---|---|---|
| `os_pid()` | a positive id | a positive id | a positive id | a positive id |
| `os_ppid()?` | a positive id | a positive id | a positive id | a positive id |
| `os_signal_number(512)?` (`USER`) | `10` | `10` | `10` | `10` |
| `os_signal_number(3)` (two bits) | `invalid` | `invalid` | `invalid` | `invalid` |
| `os_signal_send(os_pid(), 0)` | **refused by name** (`unsupported`, the record names `os_signal_send`) | ok | ok | ok |
| `os_signal_send(2147483647, 0)` | refused by name | `not_found` | `not_found` | `not_found` |
| `os_signal_ignored()?` | **refused by name** | a set (0 under `setsid nohup`? `nohup` ignores `RELOAD`, so bit 1 may be set: printed, not predicted) | the same set | the same set |
| `os_signal_listen(512)` (`USER`) | a shape row passes, then lupin never listens (refused by name) | ok | ok | ok |
| `os_signal_listen(1048576)` (`KILL`, send-only) | `io` | `io` | `io` | `io` |

And against the **bundled std** (the archive whole, no `WOLF_STD`):
`use std.signal` resolves; `signal.reload()` answers `1`;
**`signal.user()` does not compile** (the bundled `std.signal` has the
nine names of wolf-std `d7c2968`, not trunk's twenty), nor does
`signal.send` or `env.pid`. On the **control** (0.2.28) each of the five
builtin names is a compile error (E0301, nothing with this name in
scope). Falsified by any cell that answers otherwise.

## 8. Sentences the release makes false (found by reading, before any archive)

1. **ch01 §1.2** and **the colophon**: the stamps; "from four releases
   back … 541" → five releases, 611.
2. **ch01 §1.2**'s Windows archive name in the text block
   (`wolf-0.2.28-x86_64-pc-windows-msvc`), the `pin f71ece8` sentences
   and the `0.2.28+dev.unknown` example: the version and the pin.
3. **ch22 §22.2**: "`toolchain 0.2.28` is which compiler printed the
   page".
4. The version-literal ledger (`audit/version-literals.txt`): every
   `audited-at` stamp re-read at 0.2.29 / 0.1.52 (`verify-docs` forces
   it).
5. `wolf-toolchain.toml`'s `[wolf-std]` posture note and the CI step's
   comment and `want` (the archive's std is `d7c2968` now).

Sentences I checked and predict **hold**: ch01 §1.2's Windows paragraph
("what no process can do on that host is send one to another":
`os_signal_send` is `unsupported` on windows, `[os.signal.platform]`);
ch10 "What a task costs"; ch12 "The million idle connections"; ch25's
dated bs65 re-check (history at a named pin); every chapter ledger row
dated at an older pin.

## 9. Back matter and counts at head (PREDICTED)

- No snapshot re-blessed. Four console blocks spliced from the run's
  `actual:`; `solutions.md` regenerated by `backmatter` (22-13's
  toolchain line).
- `ledger --check`: **126 open, 109 closed**, unchanged.
- `verify-docs` ok after the version-literal re-read; tiers unchanged;
  Appendix C 187.
- `backmatter --check` against wolf-lang at v0.2.29: `anchors.json`
  re-vendored (640); `diagnostic-codes.txt` and `grammar.ebnf` unchanged.
- `diagrams --check` 6 from 4 with the release lupin 0.1.52 as
  `LUPIN_TRACE`: byte-identical.
- Head: **515 / 512 / 3 / 0 / 0, 488 of 508** on the archive whole on
  kasumi, also under `taskset -c 0-3`.
- CI at head: samples green on three hosts, ubuntu and macOS **512 / 3 /
  0, 488 of 508**, windows **511 / 3 / 0, 425 of 508** (bs65's counts;
  no sample added or removed).
- CI on the **unedited book at the pin** (the pin and the staged std's
  manifest, nothing else): samples **red on three hosts**, ubuntu and
  macOS on exactly the four blocks of §5 (512 / 3 / 4, 484 of 508);
  windows on those of the four its lane replays, `ch01.md`'s `--version`
  among them. That run is this lane's seen-red: the console replay
  against a 0.2.29 build, red on the stale stamps and nothing else.

## 10. Method

Control and subject on the same unedited tree, the toolchain the one
variable, on kasumi; the sweep on each pair twice. Then the pin (own
commit), the CI manifest, a push of the unedited book for the red run,
then the edits (console blocks spliced from the run by `splice.py`,
never typed), then the head gate (`samples`, also under `taskset -c 0-3`
and on the bare `wolf`, `--self-test`, `verify-docs`, `ledger --check`,
`backmatter --check`, `contrast`, `diagrams --check` with `LUPIN_TRACE`,
`render web`, `tiers --check`, xtask's tests with `--nocapture`), every
log full `2>&1` with SKIPs counted.
