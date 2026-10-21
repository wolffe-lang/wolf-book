# bs70 — the prediction, committed before the new archives are downloaded

Crawl 54. Written 2026-10-21 against wolf-book `0a8f04f` (the contract,
an empty commit on trunk `8c87534`), **before** `wolf-0.2.34-*.tar.gz`
or `lupin-0.1.55-*` was downloaded on any host and before any change to
the book. DERIVED means computed from the upstream git histories at the
tags (the org checkouts' objects on nomad-1, read only:
`bs70-evidence/inputs.sh`, `inputs.txt`) and the release API; PREDICTED
is a claim about what the release binaries will do, with what falsifies
it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.33 / lupin 0.1.54), fetched by digest
from the release API and read twice on kasumi: linux x86-64 `4f955e8d…`
and `175df238…`, each `MATCH`, `sha256sum -c` OK on a second download,
the unpacked trees identical. The control's gates on the unedited head
`0a8f04f`, inside the container (`ctr.sh`: rootless podman, its own pid
namespace, `--network none`, RLIMIT_CORE 0; SigBlk and SigIgn 0):
**samples 531 / 528 passed / 3 pending / 0 failed / 0 flips, 500 of 520
console blocks**, 4 SKIP lines; `verify-docs` ok (corpus count 268, 36
version literals audited against 0.2.33 / 0.1.54); `ledger --check` 129
open (119 filed, 10 waived, 0 unfiled), 110 closed, chapter 34 3 waived
and 1 closed; `diagrams --check` 6 from 4; all exit 0. `allprogs.py`
extracted **596 programs**; the sweep, twice: 596 records each, the
noise **6 release moves** (B-11, ex9-4, ch17:29, :113, :325, ex30-5) and
0 on lupin, native and checked. The drafts (§7) were compiled by nobody:
every one calls a name the control does not have.

Where everything runs: kasumi (linux x86-64, 16 cpus), every program the
book executes inside the container. macOS and windows are the book's CI
samples lanes (tl30: windows at `tier full`, macOS at `tier all`, the
nightly's and a toolchain release's). hasu is not this lane's.

## 1. The pin (DERIVED)

```
[wolf]         rev 85dab5d7e597680d6fd7034f1d7ed2acb9a92f97 -> 9da7bdcf2aea9adf48df6570bedc58e16e9ef853
               impl_version 0.2.33 -> 0.2.34
[lupin]        rev 0e536e8fb4518eaa798ac426901ab81275db5c9f -> e642f2630ad4c7f53740428d5cca705f2e0d295b
               version "lupin 0.1.55 (wolf-interp, reference interpreter at pin 294d626)"
[lupin-trace]  rev 0e536e8f… -> e642f263… (follows [lupin]: src/eval/placetrace.rs byte-identical v0.1.54..v0.1.55)
```

One compiler release (0.2.34, release 419552903; 112 commits
`v0.2.33..v0.2.34`) and one interpreter release (0.1.55, release
418932753; 28 commits `v0.1.54..v0.1.55`, `src/` moving in
`eval/builtin.rs` and `eval/os.rs` only). PAIRING names 0.1.55 at the
same pin `294d626`.

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.34 (wolfgang, pin 9da7bdc)
paired with lupin 0.1.55 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.55 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `9da7bdcf`, D57 and wolf-interp's `vendor/upstream/PIN`
(unchanged `294d626d`). The interpreter's clause is now **TEN releases
behind**: `294d626` is v0.2.24's tag, an ancestor of `9da7bdcf`, **1067**
commits and ten releases (v0.2.25 … v0.2.34) back. ch01 §1.2's "nine
releases … 955" and the colophon's sentence move to ten and 1067. A
plain `cargo build` at the tag names itself `wolf 0.2.34+dev.unknown`.
Falsified by any character that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 664 → 672**, none dropped, none retargeted. Added (8):
  `os.clock.ns`, `os.editor.calls.note`, `os.fs.realpath`, `os.fs.sync`,
  `os.proc.fork`, `os.proc.fork.note`, `os.signal.wait.fds`,
  `os.term.restore`.
- `spec/grammar.ebnf`, `crates/wolf_driver/src/help.rs` and
  `docs/diagnostics.md` have no diff: Appendix A, the catalogue
  (`diagnostic-codes.txt`) and Appendix E's `wolf --help` do not move.
- Seven prelude names (`os_fork`, `os_term_restore_at_exit`,
  `os_signal_wait_fds`, `fs_sync`, `fs_sync_dir`, `fs_realpath`,
  `os_clock_ns`) and the mark `busy` (`spec/prelude.json` 205 → 213
  entries). No book or corpus program declares a function of any of the
  seven names (grep), and `busy` appears in no program, so W0304 and
  W0603 move nothing.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.33's member names. **`std/`: 58 files, 54 `.lu`**,
  `STD-REV` `e036bec318ad81ecd5a025612bc2c2adb17527f3`, differing from
  0.2.33's in `net/net.lu`, `os/users/users.lu`, `process/process.lu`,
  `signal/signal.lu` and `STD-REV` only (the four files sc66 and sc67
  touched; nothing under `std/` is removed or renamed, and every change
  to a public item is an addition: `Addr`, `wait_status_core`,
  `status_from_core`, `dumped_core`, `CoreChange`, `poll_core`,
  `exit_signal`, `ignore_one`, `reset_one`). **The manifest, computed
  here from wolf-std's objects with CI's own recipe** (the method
  reproduces 0.2.33's `4382865a…` exactly): **`5d6253b63c97c050e3bf068cfb15c8515453b7ec5b98cd62a91a955bd2a9fc9a`**.
  The book's CI staging step needs it before the first push.
- Byte-identical to 0.2.33's (a guess, as before): the licences, `_wolf`,
  `wolf.bash`, `wolf.fish`. Move: `wolf`, `wolf.1`, `wolf-cimport-worker`,
  `libwolf_rt.a` (the fork, the six calls), `libwolf_rt_none.a`, and
  `README.md` if it names the version.
- linux x86-64 `wolf`: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

In the container, against the archive whole: **0 flips. 531 samples:
528 passed, 3 pending, 0 failed; 496 of 520 console blocks replayed and
4 drifted**, text only, no verdict:

1. `book/ch01.md:153` `--version`; 2. `book/back/colophon.md:7`
   `--version`;
3. `book/ch22.md:288` `wolf interface`: `toolchain 0.2.33` → `0.2.34` on
   both stamp lines, **every hash held** (the listing reaches `std.range`,
   and `std/range` has no diff `cceda63..e036bec`);
4. `principles/exercises/ch22/EXERCISES.md:218`, 22-13: the toolchain
   line.

`error: 4 failure(s), 0 flip(s)`, exit 1; the same under
`taskset -c 0-3`. Pending 3 → 3. SKIP lines 4 → 4.

What does **not** fail, and why: the busy rule (no book program forks);
`wolf test`'s discovery (ch18's four `wolf test` blocks each replay one
file alone in its own directory); the mode's bits 5–7 (no book program
sets a mode on a terminal: CI's descriptors are pipes and `/dev/null`, so
every terminal call is `io` before a bit is written); the hub before
`main` for `os_signal_wait_fds` (no book program names it); the seven
names (declared by none); the std's additions (new names, no change to
an existing one); lupin 0.1.55's new answers (every book and corpus
program that calls a signal builtin is in chapter 34 and imports std,
which the interpreter's archive does not carry, so lupin stops at the
`use` as before).

`verify-docs` at the pin commit: **red on the version-literal ledger**
(the 36 literals were audited at 0.2.33 / 0.1.54) until it is re-read in
its own commit; everything else ok. `diagrams --check` with the 0.1.55
trace lupin: 6 from 4, byte-identical traces. `ledger --check`
unchanged. `backmatter --check` (with a v0.2.34 sibling): stale only on
22-13's toolchain line in `solutions.md` until it is regenerated (one
line), and on `vendor/spec/anchors.json` until it is re-vendored (the
pin commit does both: the anchors there, `solutions.md` after the
consoles).

## 6. The move table (PREDICTED), 596 programs, each pair swept twice

| machine | moves 0.2.33 / 0.1.54 → 0.2.34 / 0.1.55 | outside the noise |
|---|---|---|
| lupin | 0 | 0 |
| native (`conform-run --json`) | 0 | 0 |
| checked (`conform-run --json --checked`) | 0 | 0 |
| release | noise only (the control's 6, give or take) | 0 |

Falsified by any lupin, native or checked move, or a release move
outside both pairs' own noise.

## 7. The new pages (PREDICTED; drafts in `bs70-evidence/draft/`)

**The fork's note is §34.7, "A child that keeps running"**, a section of
its own at the chapter's end: §34.1–§34.6 keep their anchors and their
twelve programs keep their sample ids (`book/ch34/s1` … `s12`, which
samples-os.toml's windows rows name), and the fork follows the children
(§34.5) and an end (§34.6) as the third way a process makes or ends a
process. **The editor's calls** go where the terminal is, §34.3 (the
mode's bits 5–7 and the restore at exit, in prose: the sentences "Every
other setting is left as it is found" and "Nothing restores it for you at
exit" are false at 0.2.34 as written), and the rest, which have no page
(the book has no page on the file tree's durability and none on a clock),
into **§34.8, "What an editor asks of the host"**, again at the chapter's
end. §34.4's table and "What your machine does" take the new rows and
the sentences 0.1.55 makes false.

On the four machines of the new pair, inside the container (also `wolf
run` under `taskset -c 0-3`, identical):

- **`fork.lu`** (s13): native, `conform-run --json` and the release
  binary print
  `the child: answered (0, -1), its count 2` /
  `the parent: its count 1, the child's status 3` /
  `the parent was answered a pid and a handle: true`, exit 0. Checked:
  `error: unsupported`, exit 1 (ruling #95 R4 C, the row at the call).
  lupin 0.1.55: `fork.lu: unsupported: \`os_fork\` does not resolve`,
  exit 4 (wolf-interp#269).
- **`busy.lu`** (s14): native and release `a task live: busy` /
  `every task joined: a child, its status 0`, exit 0. **Checked:
  `a task live: unsupported` / `every task joined: unsupported`, exit 0**
  (the machine answers `unsupported` whatever is live: medium
  confidence). lupin: `os_fork` does not resolve, exit 4.
- **`wait.lu`** (s15): every compiled machine and checked:
  `nothing came: (-1, 0), and 25 ms passed: true` /
  `a byte waits in the pipe: (0, 0)` /
  `a resize beside it comes first: true`, exit 0. lupin: the `use
  std.signal` refusal, exit 4.
- **`save.lu`** (s16): every compiled machine and checked:
  `the temporary name, made again: exists` / `saved: new` /
  `drafts/.. is this directory: true` / `a name nothing holds: not_found`,
  exit 0. lupin: `fs_sync` does not resolve (or the first of the six it
  meets), exit 4.

**Windows** (the windows samples lane, `tier full`): `fork.lu` exit 1,
`error: unsupported` (low confidence that `os_pipe` passes there first);
`busy.lu` exit 0 (both lines `unsupported`, low confidence on the
first), so no row; `wait.lu` exit 1, `error: io` (a listen for
`resize()` is `io` there); `save.lu` exit 1,
`the temporary name, made again: exists` then `error: unsupported` at
`fs_sync_dir` (low confidence). Each row written from that lane's own
log after it is seen red, never from this list.

**wolf-book#87.** `run_capture` answers `status.code().unwrap_or(-1)`,
so a command a signal ended replays as `-1`. The decision: **`128 + N`**,
as a shell's `$?` prints it, because a console block is a shell
transcript (POSIX XCU 2.8.2: the exit status of a command terminated by a
signal is greater than 128; dash, bash and zsh print `128 + N`). The row:
§34.6's `again.lu`, built and run with its marker argument, then
`echo $?` — **`143`** on linux and macOS (SIGTERM is 15 on both; the
console replay is the unix lanes'). A test that `run_capture` of
`sh -c 'kill -TERM $$'` answers 143 is red before the fix (`-1`), and the
console row is red before it on ubuntu (`-1` where the page says `143`).

**The plant**, the PR's own `tier pr` run (ubuntu): at the head carrying
the pin, the CI manifest, #87's test and #87's console row, the text
otherwise unedited: samples red on exactly the four consoles of §5 and
§34.6's new row; the test job red on exactly #87's test.

## 8. What is not shown (DERIVED)

- No std spelling for the six calls, the bits or the fork: wolf-std
  `e036bec` has none (sc69's `3bacc5f` carries them, the next STD-PIN); a
  dated note in chapter 34's ledger says so, and one sentence there that
  wolf-book#89 (the `term.Mode` literal at that pin) is 0.2.35's.
- §34.6 keeps its five builtins: `e036bec` now wraps them (sc66), and
  moving §34.6 to the wrappers is not this contract's (an issue to file).
- lupin 0.1.55's "does not resolve" for the seven names is a lag
  (wolf-interp#268, #269), not the interpreter's designed refusal: it is
  recorded in the evidence and the ledger, and §34.4 says in one present
  tense sentence that the interpreter does not resolve them, as bs69
  said of the five; no page prints the lag's line (TONE.md §Tense).

## 9. Counts at the head (PREDICTED)

Samples 531 + 4 = **535** (four new book blocks), **532 passed**, 3
pending, 0 failed, 0 flips; console blocks 520 + 5 = **525** (four new
`wolf run` transcripts and #87's row; §34.7's checked transcript is one
more command in `busy.lu`'s block) and **505 of 525 replayed**; on
windows 535 / 3 pending and four declared refusals more than trunk's (if
`busy.lu` needs none, three). Ledger 129 / 10 waived / 110 closed
(chapter 34's rows are re-read and extended, none added open). TOC gains
§34.7 and §34.8. `solutions.md` moves by exactly one line (22-13).
