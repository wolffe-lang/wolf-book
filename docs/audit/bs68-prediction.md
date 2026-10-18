# bs68 — the prediction, committed before either new archive was downloaded

Crawl 54. Written 2026-10-18 against wolf-book `0070509` (the contract,
an empty commit on bs67's merged trunk `95d68ad`), **before**
`wolf-0.2.32-*.tar.gz` or `lupin-0.1.54-*.tar.gz` was downloaded on any
host and before any change to the book. DERIVED means computed from the
upstream git histories at the tags (the org checkouts' objects on
nomad-1, read only: `bs68-evidence/inputs.sh`, `inputs.txt`); PREDICTED
is a claim about what the release binaries will do, with what falsifies
it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.30 / lupin 0.1.53), fetched by digest from
the release API on kasumi and read twice (`bs68-evidence/setup.sh`:
linux x86-64 `4e55b39e…` and `5db55332…`, each `MATCH`, `sha256sum -c`
OK on a second download, the unpacked trees identical). The control's
gates on the unedited head `0070509`, inside the container (`ctr.sh`),
had finished when this was written: **samples 524 / 521 passed / 3
pending / 0 failed / 0 flips, 495 of 515 console blocks**; `verify-docs`,
`ledger --check` (130 open: 119 filed, 11 waived, 0 unfiled; 109
closed), `diagrams --check` (6 from 4) exit 0. `allprogs.py` extracted
**589 programs**. Its sweep was still running. The new pages' drafts
(§7) were compiled by nobody: they call builtins the control does not
have.

Where everything runs: kasumi (linux x86-64, 16 cpus). Every program the
book executes runs **inside a container** (`ctr.sh`: rootless podman,
the lane's own uid, its own pid namespace, `--network none`, the host's
`/usr` read only), because the new pages' programs signal a child they
start. macOS and windows are the book's CI samples lanes, which run in
full.

## 1. The pin (DERIVED)

```
[wolf]         rev 1b37f368228db92eca505c8c4cd3c531ea60be48 -> 72de141f624662fdff522e8beba459fab7c5cf66
               impl_version 0.2.30 -> 0.2.32
[lupin]        rev 7896eddc17fd9e3e015b025b354ccf62a766734b -> 0e536e8fb4518eaa798ac426901ab81275db5c9f
[lupin-trace]  rev 7896eddc17fd9e3e015b025b354ccf62a766734b -> 0e536e8fb4518eaa798ac426901ab81275db5c9f
```

Two compiler releases (0.2.31, 0.2.32; release 416327027; 196 commits
`v0.2.30..v0.2.32`, 170 and 26) and one interpreter release (0.1.54,
release 415937354; 43 commits). The trace pin stays collapsed onto
`[lupin].rev`: `src/eval/placetrace.rs` is byte-identical between v0.1.53
and v0.1.54.

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.32 (wolfgang, pin 72de141)
paired with lupin 0.1.54 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.54 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `72de141f` (`lupin-version = 0.1.54`, `lupin-pin =
294d626`), `vendor/upstream/PIN` at `v0.1.54` (`294d626d…`) and D57. The
interpreter's clause is now **EIGHT releases behind**: `294d626` is
v0.2.24's tag, **913** commits and eight releases (v0.2.25 … v0.2.32).
ch01 §1.2's "six releases … 717" and the colophon's matching sentence
move to eight and 913. Falsified by any character of the stamps that
differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 648 → 661**, none dropped, none retargeted. Added (13):
  `os.net.error`, `os.net.reactor`, `os.net.reactor.note`,
  `os.proc.poll`, `os.proc.wait.note`, `os.signal.group.note`,
  `os.signal.host`, `os.signal.hub`, `os.signal.send.all`,
  `os.signal.send.group`, `os.signal.text`, `os.term.group`,
  `type.int.shift`.
- **`grammar.ebnf` does not move.** Appendix A does not regenerate.
- **Catalogue 187 → 187**; `docs/diagnostics.md` moves one `Fixtures:`
  line under E0706 and adds or removes no code, so
  `vendor/spec/diagnostic-codes.txt` does not move.
- `help.rs` has no diff: Appendix E's `wolf --help` holds.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.30's member names. `std/`: **58 files, 54 `.lu`**
  (one new module, `std/os/users/users.lu`), `STD-REV` =
  `d37c911f12206c4bf5c8d3281a649bf31da3beef`; it differs from 0.2.30's in
  `env`, `fs`, `process`, `signal`, `term`, the new `os/users` and
  `STD-REV`, so the manifest moves off `ab2576df…` and the book's CI
  staging step needs the archive's new manifest before the first push.
  Byte-identical to 0.2.30's: the four licences and (a guess, as the
  last three times) `_wolf`, `wolf.bash`, `wolf.fish`. Move: `wolf`,
  `wolf.1`, `README.md`, `wolf-cimport-worker`, `libwolf_rt.a`,
  `libwolf_rt_none.a`.
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE`, `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 524 samples: 517 passed, 3 pending, 4 failed; 491 of 515
console blocks**, against the archive whole, in the container. The four
are console blocks, text only, no verdict:

1. `book/ch01.md` `--version`; 2. `book/back/colophon.md` `--version`;
3. `book/ch22.md` `wolf interface`: `toolchain 0.2.30` → `0.2.32` on
   both stamp lines, **every hash held** (the listing reaches `std.range`
   and `std/range` has no diff `fd6bcdc..d37c911`; wolf-lang#721 says the
   hashes follow the std beside `wolf`, and at bs67 a std move outside
   `range` held them);
4. `principles/exercises/ch22/EXERCISES.md`, 22-13: the toolchain line.

On a bare `wolf` (no `std/` beside it): the four, and every chapter-34
program that imports std (as at bs67's bare run).

What does **not** fail, and why: the shift rule (no book program shifts:
a grep for `<<` and `>>` in every `.lu` and every wolf fence finds none);
the hub and the readiness set made before `main` (no book program runs
at a descriptor limit; the programs that listen or park start as before);
a closed standard descriptor (no book program closes 0–2); `-N` as a
host signal (no book program sends a negative meaning); the seven new
prelude names (no book program declares one: the 0.2.31 CHANGELOG's own
grep of the book at `95d68ad0`, and mine); `os_signal_wait`'s lost timer
(no output depends on it). Pending 3 → 3 in the same words.

## 6. The sweep: subject against control (PREDICTED)

589 programs (`allprogs.py`), each alone, on lupin, `conform-run --json`
(native), `conform-run --json --checked`, and a `--release` build and run
for every native `pass` with a `main`; each pair twice, inside the
container.

- **lupin: 0 rows** outside the control's own noise. The row I am least
  sure of: is76's literal-term rule (wolf-interp `f6ec48b`, a literal meets
  its declared integer type before its operators run) could move a
  program whose literal arithmetic leaves its declared type; I found
  none by reading.
- **native: 0 rows.**
- **checked: exactly 1 row, ex33-7** (`principles/exercises/ch33/ex33-7.lu`):
  0.2.30 no record, killed at 120 s (wolf-lang#720); 0.2.32 **a record,
  `exit(0)`, stdout `answered 4 questions on 2 connections`**, as lupin
  and native answer it (s244: `net_wait` beside other tasks polls a
  millisecond at a time and hands over). s237's two refusals retired
  (a plain shift, a home-module method call) name no book program: the
  checked machine's other `unsupported` reasons at 0.2.30 (bs67's sweep:
  no `main`, loop values, borrow expressions, indexing / generic
  application, module items, methods on builtin types, trait dispatch,
  interpolation in a multiline string, fd inheritance, an item without a
  type) are not those two. Falsified by any other checked row.
- **release: noise only** (bs67's set: B-11 and ex9-4's exit after UB,
  ch17's balances, ex11-5, ex30-4/ex30-5, and the like).

## 7. The new pages' programs on four machines (PREDICTED from the spec at v0.2.32, std `d37c911` and lupin `v0.1.54`'s `builtin.rs`)

Drafted before any archive (`~/lanes/bs68/draft/progs/` on nomad-1,
committed with the evidence; the page may change a line, and a change is
recorded in the results):

| program | native (`wolf run`, `conform-run --json`) | checked (`conform-run --json --checked`) | release | lupin 0.1.54 (its own archive, no std) |
|---|---|---|---|---|
| `shift.lu` (§3.3): `200` at `u8`, `>> 1`, `>> 8`, then `<< 1` | `100 0`, then `trap(overflow)` at the `<<` (134, `wolf-trap: overflow`) | the same kind | the same | `100 0`, then `trap(overflow)`, exit 3, one trap line naming `<<`, 400, `u8` and `[type.int.shift]` |
| `rotate.lu` (§3.3): `200` at `wrapping[u8]`, `<< 1`, `>> 7`, `<< 8` | `144 1 200` | the same | the same | the same |
| `nochild.lu` (§34.5): `os_wait_poll()` with no child | `nothing to report: -1, 0 live` | the same | the same | **the same** (lupin decides `(-1, 0)` without the host) |
| `jobs.lu` (§34.5): listen `child()`, spawn `/bin/sleep 10`, send it `stop()`, poll until it reports, send `kill()`, poll until it ends, poll once more | `a stop: true, by stop(): true` / `an end: true, by kill(): true` / `then: -1, 0 live` | the same | the same | `unsupported` (no std), exit 4 |
| `groups.lu` (§34.5): `os_signal_send_group` with meaning 0 to this process's group (`term.group()`), to group 1, to a group nobody leads, with two meanings; `os_signal_last()? > 28` | `ok`, `invalid`, `not_found`, `invalid`, `true` | the same | the same | `unsupported` (no std), exit 4 |
| `limit.lu` (probe only): names `net_accept`, run with `ulimit -n 3` | no line of `main`; stderr one line, `wolf: the runtime could not make its readiness set (… (os error 24)); the program did not start`; **exit 71** | (not run at a limit: the checked machine has no readiness set) | the same | — |
| `hublimit.lu` (probe only): names `signal.listen` and `wait`, `ulimit -n 3` | no line; `wolf: the runtime could not make its signal hub (… (os error 24)); the program did not start`; **exit 71** | (not run at a limit) | the same | — |

Every executed block is a `run(…)` (shift, rotate, nochild) or a
`wolf-run(…)` (jobs, groups). In CI's three samples lanes: ubuntu and
macOS pass all five (macOS: `os_signal_last()` is 31, stop and kill are
the host's 17 and 9, and the poll reports the stop as `256 + 17`; the
program prints only comparisons, so its lines are the same); **windows**
passes `shift.lu`, `rotate.lu` and `nochild.lu`, and answers the other
two with a row: `jobs.lu` exits 1 at `signal.listen(signal.child())?`
(`io`: windows hears only the console's meanings and a program's own
first four), `groups.lu` exits 1 at `term.group()?` (`unsupported`).
Those become `[[refusal]]` rows in `samples-os.toml`, each written from
that lane's own log. Falsified by any cell that answers otherwise.

## 8. Sentences the release makes false (found by reading, before any archive)

1. ch01 §1.2 and the colophon: the stamps; "six releases … 717" → eight,
   913; the Windows archive names; the `pin 1b37f36` sentences; the
   `0.2.30+dev.unknown` example.
2. ch22 §22.2: "`toolchain 0.2.30` is which compiler printed the page".
3. The version-literal ledger: every `audited-at` re-read at 0.2.32 /
   0.1.54.
4. `wolf-toolchain.toml`'s `[wolf-std]` note and the CI step's comment
   and `want`.
5. **ch34 §34.3, twice**: "the wrappers ship with the next release" (the
   ids; the terminal's size, characters and wait). wolf-std `d37c911`,
   the std 0.2.31 and 0.2.32 ship, wraps every call those sentences name
   (`env.uid`, `euid`, `gid`, `egid`, `groups`; `term.size`,
   `term.chars`; `signal.wait_fd`; `process.set_ids`; `fs.set_owner`,
   `set_file_owner`). The contract's DRIFT, in the contract commit: the
   sentences name a release that has passed, so they are re-worded to
   name the wrappers, and the waiver moves to the seven calls `d37c911`
   does not wrap.
6. ch34 §34.4's table, column "in `std`": the same wrappers; and the
   seven new builtins' rows (the three sends and `os_set_pgid` and the
   poll `exec`; the last number, the range and the words charge
   nothing).

## 9. Back matter and counts at head (PREDICTED)

- No snapshot re-blessed. Four console blocks spliced from the run;
  `solutions.md` regenerated by `backmatter` (22-13's toolchain line).
- `backmatter --check` against wolf-lang at v0.2.32: `anchors.json`
  re-vendored (661); codes and grammar unchanged.
- `diagrams --check` 6 from 4 with lupin 0.1.54 as `LUPIN_TRACE`:
  byte-identical.
- `ledger --check`: 130 open (119 filed, 11 waived) plus the new pages'
  rows, every one filed or waived; 109 closed.
- Head, kasumi, in the container, archive whole: **529 samples** (524 +
  five new blocks), 526 passed, 3 pending, 0 failed, 0 flips; console
  blocks 515 + 3 (`shift.lu`'s lupin trap, `jobs.lu` and `groups.lu`'s
  `wolf run`), **498 of 518** replayed. Also under `taskset -c 0-3`.
- The sweep at head: 589 + 5 programs; the five as §7 says.
- CI at head: green on three hosts; windows with the two `[[refusal]]`
  rows of §7.
- CI on the **unedited book at the pin** (the pin and the staged std's
  manifest, nothing else): samples **red on three hosts** on exactly the
  four blocks of §5. That run is the planted break this lane's rules ask
  for: the pin is the plant, the four consoles are what must go red.

## 10. Method

Control and subject on the same unedited tree, the toolchain the one
variable, on kasumi inside the container; the sweep twice per pair. Then
the pin (own commit), the CI manifest, a push of the unedited book for
the red run, the consoles spliced from that run, then the edits: the
pin's sentences, chapter 34's re-worded sentences and table, the new
pages (§3.3's shift rule, exit 71 in §33.4 and §34.1, §34.5 the poll and
the groups), what they must touch, the windows rows from CI's own log.
Then the head gate (`samples`, also under `taskset -c 0-3` and on the
bare `wolf`, `--self-test`, `verify-docs`, `ledger --check`, `backmatter
--check`, `contrast`, `diagrams --check`, `render web`, `tiers --check`,
xtask's tests with `--nocapture`), every log full `2>&1`, SKIPs counted.
