# bs69 — the prediction, committed before the new archive is downloaded

Crawl 54. Written 2026-10-19 against wolf-book `eb657f2` (the contract,
an empty commit on bs68's merged trunk `e9862f6`), **before**
`wolf-0.2.33-*.tar.gz` was downloaded on any host and before any change
to the book. DERIVED means computed from the upstream git histories at
the tags (the org checkouts' objects on nomad-1, read only:
`bs69-evidence/inputs.sh`, `inputs.txt`) and the release API; PREDICTED
is a claim about what the release binaries will do, with what falsifies
it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.32 / lupin 0.1.54), fetched by digest from
the release API and read twice on kasumi (`gh`) and on hasu (`curl`
against the public API: hasu's `gh` has no valid login): linux x86-64
`4c37f9c5…` and `175df238…`, each `MATCH`, `sha256sum -c` OK on a second
download, the unpacked trees identical. The control's gates on the
unedited head `eb657f2` on kasumi, inside the container (`ctr.sh`, now
with `--ulimit core=0:0`): **samples 529 / 526 passed / 3 pending / 0
failed / 0 flips, 498 of 518 console blocks**; `verify-docs` ok (corpus
count 268); `ledger --check` 130 open (119 filed, 11 waived, 0
unfiled), 109 closed, chapter 34 4 waived; `diagrams --check` 6 from 4;
all exit 0. `allprogs.py` extracted **594 programs**. The sweep and
hasu's control were still running. The chapter-34 drafts (§7) were
compiled by nobody: five of them call names the control does not have.

Where everything runs: kasumi (linux x86-64, 16 cpus; every program the
book executes runs inside the container, rootless podman, its own pid
namespace, `--network none`, RLIMIT_CORE 0) and hasu (linux x86-64,
NixOS, 20 cpus; no container runtime, so the samples gate runs on the
host under `ulimit -c 0`, its programs signalling nothing but themselves
and the child they start). macOS and windows are the book's CI samples
lanes, which run in full.

## 1. The pin (DERIVED)

```
[wolf]         rev 72de141f624662fdff522e8beba459fab7c5cf66 -> 85dab5d7e597680d6fd7034f1d7ed2acb9a92f97
               impl_version 0.2.32 -> 0.2.33
[lupin]        rev 0e536e8fb4518eaa798ac426901ab81275db5c9f (unchanged: 0.2.33 pairs 0.1.54)
[lupin-trace]  rev 0e536e8fb4518eaa798ac426901ab81275db5c9f (unchanged)
```

One compiler release (0.2.33, release 417610226; 42 commits
`v0.2.32..v0.2.33`); no interpreter release (PAIRING still names 0.1.54
at pin `294d626`). The lupin archive is the one already pinned
(`175df238…`, release 415937354).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.33 (wolfgang, pin 85dab5d)
paired with lupin 0.1.54 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.54 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `85dab5d7` and D57. The interpreter's clause is now
**NINE releases behind**: `294d626` is v0.2.24's tag, **955** commits
and nine releases (v0.2.25 … v0.2.33) back. ch01 §1.2's "eight releases
… 913" and the colophon's matching sentence move to nine and 955. A
plain `cargo build` at the tag names itself `wolf 0.2.33+dev.unknown`
(`[workspace.package] version = "0.2.33"`). Falsified by any character
of the stamps that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 661 → 664**, none dropped, none retargeted. Added (3):
  `os.signal.disp.one`, `os.signal.exit`, `os.signal.gaps.note`.
- **`grammar.ebnf` does not move.** Appendix A does not regenerate.
- **Catalogue 187 → 187**: `docs/diagnostics.md` has no diff, so
  `vendor/spec/diagnostic-codes.txt` does not move.
- `help.rs` has no diff: Appendix E's `wolf --help` holds.
- Five new prelude names: `os_signal_ignore_one`, `os_signal_default_one`,
  `os_exit_signal`, `os_wait_status_core`, `os_wait_poll_core`. No book
  program declares a function of any of those names (grep), so W0304
  moves nothing.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.32's member names. `std/`: **58 files, 54 `.lu`**
  (no module added), `STD-REV` = `cceda63151430eac07df895fdfffddfa4a0b92c1`;
  it differs from 0.2.32's in **`process/process.lu`, `signal/signal.lu`
  and `STD-REV` only**, so the manifest moves off `1f7389b7…` and the
  book's CI staging step needs the archive's new manifest before the
  first push. Byte-identical to 0.2.32's: the four licences and (a guess,
  as the last four times) `_wolf`, `wolf.bash`, `wolf.fish`. Move:
  `wolf`, `wolf.1`, `wolf-cimport-worker`, `libwolf_rt.a`,
  `libwolf_rt_none.a`, and `README.md` if it names the version.
- linux x86-64 `wolf`: no `GLIBC_` symbol above **2.34** (r38 read the
  same).

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

On kasumi in the container and on hasu on the host, against the archive
whole: **0 flips. 529 samples: 526 passed, 3 pending, 0 failed; 494 of
518 console blocks replayed and 4 drifted**, text only, no verdict:

1. `book/ch01.md:153` `--version`; 2. `book/back/colophon.md:7`
   `--version`;
3. `book/ch22.md:288` `wolf interface`: `toolchain 0.2.32` → `0.2.33` on
   both stamp lines, **every hash held** (the listing reaches `std.range`
   and `std/range` has no diff `d37c911..cceda63`; at bs67 and bs68 a std
   move outside `range` held them);
4. `principles/exercises/ch22/EXERCISES.md:218`, 22-13: the toolchain
   line.

`error: 4 failure(s), 0 flip(s)`, exit 1. Also under `taskset -c 0-3`
on kasumi. On a bare `wolf` (no `std/` beside it), kasumi only: bs68's
head bare set (18 failures at 0.2.32) plus ch01, the colophon and 22-13
(ch22:288 already fails bare): **21 failures, 0 flips** (low
confidence: I have no bare control at 0.2.32 of my own).

What does **not** fail, and why: `os_signal_text(0)` answering words (no
book program asks for 0's words); a SENT SEGV ending a program (no book
program sends one); the descriptor waits' shared wake pipe (no book
program names `os_signal_wait_fd`); the five prelude names (no book
program declares one); the bundled std's new functions (std functions,
not prelude names: no collision with a book program's own). Pending 3 → 3
in the same words.

## 6. The sweep: subject against control (PREDICTED)

594 programs (`allprogs.py`), each alone, on lupin,
`conform-run --json` (native), `conform-run --json --checked`, and a
`--release` build and run for every native `pass` with a `main`; each
pair twice, inside the container.

- **lupin: 0 rows** outside the control's own noise: it is the same
  binary (`175df238…`), so any row there is noise, not a move.
- **native: 0 rows.** s246's runtime changes reach no book program: no
  program is sent a SEGV or BUS, names `os_signal_wait_fd`, or reads
  `text(0)`; the guard's `SIGBUS` handler on linux changes no output.
- **checked: 0 rows.** s246's checked changes are the five new calls,
  `text(0)` and the wait_fd pipe (wolf-lang#791, unchanged), none of
  which a book program reaches.
- **release: noise only** (bs68's set: B-11 and ex9-4's exit after UB,
  ch17's balances, ex11-5, ex30-4/ex30-5 and the like). Falsified by any
  row outside the control's two-sweep noise on native, checked or
  lupin.

## 7. Chapter 34 on the std, and the five calls (PREDICTED from the spec at v0.2.33, std `cceda63` and lupin `v0.1.54`)

Drafted before any archive (`bs69-evidence/draft/`, committed with this
file; the page may change a line, and a change is recorded in the
results). **What moves to the std spelling:** every program in chapter
34 that calls a builtin the bundled std now wraps, except the two that
exist to show the builtin underneath on both machines (§34.2's
`alive.lu`, lupin's by-name refusal; §34.5's `nochild.lu`, the one
program both machines run). So: §34.3's `who.lu` (`env.uid` … `egid`)
and `window.lu` (`term.size`, `term.chars` on `fs.stdout()`), §34.5's
`groups.lu` (`process.lead_group`, `own_group`, `Group`,
`signal.send_group`, `signal.last`) and `jobs.lu` (`process.command`,
`start`, `pid`, `poll`, `kill`, `Status`'s accessors; and `sleep` by
name on the `PATH`, where the draft at bs68 named `/bin/sleep`, which
hasu (NixOS) does not have). New, in a §34.6 at the chapter's end (so
§34.1–§34.5 keep their anchors and sample ids s1–s10 keep their
numbers): `one.lu` (the one-meaning disposition and the sent-fault rule)
and `again.lu` (an end by a signal, read by the parent through the core
sibling). `text0.lu` is a probe only.

| program | native (`wolf run`, `conform-run --json`) | checked (`conform-run --json --checked`) | release | lupin 0.1.54 (its own archive, no std) |
|---|---|---|---|---|
| `who.lu` (§34.3, s5) | `an id: true, a parent: true` / `started set-user-id: false` / `started set-group-id: false` | the same | the same | `unsupported` (no std), exit 4 |
| `window.lu` (§34.3, s7) | `size: io, keys: io` (stdout not a terminal) | the same | the same | `unsupported` (no std) |
| `groups.lu` (§34.5, s8) | `leads a group of its own: true`, `ok`, `invalid`, `not_found`, `invalid`, `true` | the same | the same | `unsupported` (no std) |
| `nochild.lu` (§34.5, s9, unchanged) | `nothing to report: -1, 0 live` | the same | the same | **the same** |
| `jobs.lu` (§34.5, s10) | `a stop: true, by stop(): true` / `an end: true, by kill(): true` / `then: changed false, 0 live` | the same | the same | `unsupported` (no std) |
| `one.lu` (§34.6, s11) | `kill: invalid, stop: invalid` / `nothing: invalid, two at once: invalid` / `a fault meaning: ok` / `lived through the segment_fault() it sent itself`, exit 0 | **refused by name** at `os_signal_ignore_one(SEGMENT_FAULT)` after the first two lines (`[os.signal.checked]`: the `wolf` process's own guard holds SEGV) | the same as native | `unsupported` (no std) |
| `again.lu` (§34.6, s12) | `ended by terminate(): true` / `and with a core: false` | the parent's `os_exe()` is the `wolf` process, so the child is not the program: `false` / `false` (low confidence: it may instead be refused) | the same as native | `unsupported` (no std) |
| `text0.lu` (probe) | `words for 0: Unknown signal 0` / `words for terminate(): Terminated` / `nothing to report, core sibling: -1, 0 live` | the same | the same | `unsupported` (no std) |

Each also under `taskset -c 0-3`, the same. Every executed block is a
`wolf-run(…)` block but `nochild.lu`'s `run(…)`. In CI's three samples
lanes: ubuntu and macOS pass all seven (macOS: `signal.last()` is 31,
stop and kill are the host's 17 and 9, a self-sent SEGV is the announced
send the guard passes to the program's ignore, TERM is 15 on both; every
program prints only comparisons). **Windows**: the five std moves answer
the rows bs67 and bs68 declared, unchanged (`who.lu` at `env.ppid`,
`error: unsupported`, exit 1; `window.lu` `size: unsupported, keys:
unsupported`, exit 0; `groups.lu` at `process.lead_group`, `error:
unsupported`, exit 1; `jobs.lu` at the `child()` listen, `error: io`,
exit 1; `nochild.lu` passes); the two new programs are red until rows are
declared from that lane's own log: `one.lu` prints its first two lines,
`a fault meaning: unsupported`, then `error: unsupported` at the send,
exit 1; `again.lu`'s child prints `error: unsupported` at
`os_exit_signal` and exits 1, and the parent ends at `signal.number`
with `error: unsupported`, exit 1. Falsified by any cell that answers
otherwise.

No program on any page ends a child by a signal that dumps core
(`again.lu`'s child ends by `terminate()`, whose default writes none), so
the core reading (`-(256 + N)`) is taught from `[os.proc.status]` and
s246's measurement (wolf-lang#789's core.lu rows), not re-measured here.

## 8. Sentences the release makes false (found by reading, before any archive)

1. ch01 §1.2 and the colophon: the stamps; "eight releases … 913" →
   nine, 955; the Windows archive names; the `pin 72de141` sentences; the
   `0.2.32+dev.unknown` example.
2. ch22 §22.2: "`toolchain 0.2.32` is which compiler printed the page";
   its ledger row gains the bs69 re-measure.
3. The version-literal ledger: every `audited-at` re-read at 0.2.33 /
   0.1.54.
4. `wolf-toolchain.toml`'s `[wolf-std]` note and the CI step's comment
   and `want`.
5. **ch34 §34.5's waiver sentence**: "The standard library this
   toolchain ships has no wrappers for them, or for the poll later in
   this section" — false at `cceda63`, which carries ruling #85's
   `process.Status`, `process.poll`, `Group`, `group_of`, `own_group`,
   `lead_group`, `join_group`, `signal.send_group`, `send_all`, `last`,
   `Realtime`/`realtime`, `text`, `host`. It comes out; the section
   teaches the std and names the builtins underneath; the ledger's
   waiver row closes (chapter 34 waived 4 → 3, closed 0 → 1).
6. ch34 §34.4's table: the "in `std`" column gains the group, numbering
   and poll wrappers; the `exec` row gains the five builtins; its prose
   ("What your machine does") gains lupin's answer to the five (it does
   not resolve them) and windows' (`unsupported` for the disposition and
   the exit; no core ever).
7. Not false by the release but by the std move: §34.3's "the program
   below calls the builtins directly" and §34.5's `os_…`-first prose.

## 9. #83 and #84 (PREDICTED)

- **#83**: `console.rs` `run_capture` spawns with `cmd.output()` and no
  process group, where the samples runner's `run_within` gives each
  program `process_group(0)`. A new unit test (unix) replays `sh -c` and
  asserts the shell leads its own group: **red** before the fix (the
  shell is in `cargo test`'s group), green after. The fix moves no
  console block (§34.5's `groups.lu` leads its own group already).
- **#84**: `printed_exercises` keeps the last numbered section across an
  unnumbered `## Exercises` heading, so 33-5 … 33-7 link §33.4 and 34-5 …
  34-7 §34.5. A new unit test: **red** before the fix, green after; the
  fix makes the six link their chapter (`#33`, `#34`, the chapter
  headings' anchors and typst labels), and `solutions.md` regenerates by
  exactly those six `<summary>` lines (plus 22-13's toolchain line). The
  other unnumbered closing headings (`The shelf, and what it cannot do`
  and the like) are not batch sections and keep linking the section
  before them.
- Both tests' reds are seen in CI on the PR run that carries them before
  their fixes.

## 10. Back matter and counts at head (PREDICTED)

- No snapshot re-blessed (§34.6 is after every diagnostic block in
  chapter 34; no fence moves before one). Four console blocks spliced
  from the run; two new consoles (`one.lu`, `again.lu`) copied from the
  probe logs and replayed by the gate; `solutions.md` regenerated by
  `backmatter`.
- `backmatter --check` against wolf-lang at v0.2.33: `anchors.json`
  re-vendored (664); codes and grammar unchanged.
- `diagrams --check` 6 from 4 with lupin 0.1.54 as `LUPIN_TRACE`:
  byte-identical (the trace lupin did not move).
- `ledger --check`: **129 open (119 filed, 10 waived), 110 closed** (the
  waiver row closes; the new pages' rows join the existing lupin and
  windows waivers rather than adding any).
- Head, kasumi in the container and hasu on the host, archive whole:
  **531 samples** (529 + `one.lu`, `again.lu`), 528 passed, 3 pending, 0
  failed, 0 flips; console blocks 518 + 2, **500 of 520** replayed. Also
  under `taskset -c 0-3`.
- The sweep at head: 594 + 2 programs; the seven as §7 says.
- CI at head: green on three hosts; windows with two new `[[refusal]]`
  rows (s11, s12); s5, s7, s8, s10 unchanged.
- CI on the **unedited book at the pin** (the pin and the staged std's
  manifest, with #83's and #84's tests before their fixes): samples
  **red on three hosts** on exactly the four blocks of §5, and the test
  job red on exactly the two new tests (#84's on three hosts, #83's on
  ubuntu and macOS; windows does not build it). That run is this lane's
  planted break.

## 11. Method

Control and subject on the same unedited tree, the toolchain the one
variable, on kasumi inside the container and on hasu; the sweep twice per
pair on kasumi. Then the pin (own commit), the CI manifest, #83's and
#84's tests, a push for the red run, the consoles spliced from kasumi's
run, then the edits: the pin's sentences, chapter 34 on the std (§34.3,
§34.5) and the waiver out, §34.6 the five calls, §34.4's table, TOC and
§Deltas, the ledger rows; #83's fix and #84's fix, each its own commit;
the windows rows from CI's own log. Then the head gate on kasumi
(`samples`, also under `taskset -c 0-3` and on the bare `wolf`,
`--self-test`, `verify-docs`, `ledger --check`, `backmatter --check`,
`contrast`, `diagrams --check`, `render web`, `tiers --check`, xtask's
tests with `--nocapture`) and the samples gate on hasu, every log full
`2>&1`, SKIPs counted.
