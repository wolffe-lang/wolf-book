# bs67 — the prediction, committed before either new archive was downloaded

Crawl 54. Written 2026-10-16 against wolf-book `86db060` (the contract,
an empty commit on bs66's merged trunk `0a5fecc`), **before**
`wolf-0.2.30-*.tar.gz` or `lupin-0.1.53-*.tar.gz` was downloaded on any
host and before any change to the book. DERIVED means computed from the
upstream git histories at the tags (blobless clones of wolf-lang,
wolf-interp and wolf-std under `~/lanes/bs67/` on nomad-1, read only);
PREDICTED is a claim about what the release binaries will do, with what
falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.29 / lupin 0.1.52), and wolf 0.2.15 (the
toolchain §13.1's table names), each fetched by digest from the release
API on kasumi and read twice (`bs67-evidence/setup.sh`: linux x86-64
`ae466e85…`, `f5194b74…`, `dd28150c…`, each `MATCH`, `sha256sum -c` OK on
a second download, the unpacked trees identical). The control's gates on
the unedited head `86db060`, inside the container (`ctr.sh`), had
finished when this was written: **samples 515 / 512 passed / 3 pending /
0 failed / 0 flips, 488 of 508 console blocks**; `verify-docs`, `ledger
--check`, `diagrams --check` exit 0. Its sweep was still running. The
seven chapter-34 drafts (below, §7) were compiled by nobody: they use
names the control's std does not have. §13.1's six timing programs were
type-checked by the control's `conform-run` (all `pass`) and not timed.

Where everything runs: kasumi (linux x86-64, 16 cpus). Every program the
book executes runs **inside a container** (`ctr.sh`: rootless podman,
the lane's own uid, its own pid namespace, `--network none`, the host's
`/usr` read only), because the new chapter's programs signal themselves
and the lane's rule is that no signal is sent on a host. macOS and
windows are the book's CI samples lanes, which run in full.

## 1. The pin (DERIVED)

```
[wolf]         rev 6ffe7c6056eeb8c661b37e38d99b91166e8c1add -> 1b37f368228db92eca505c8c4cd3c531ea60be48
               impl_version 0.2.29 -> 0.2.30
[lupin]        rev 025428b725a35bfbc53745d4166817ff3053c0fd -> 7896eddc17fd9e3e015b025b354ccf62a766734b
[lupin-trace]  rev 025428b725a35bfbc53745d4166817ff3053c0fd -> 7896eddc17fd9e3e015b025b354ccf62a766734b
```

One compiler release (0.2.30, release 413582739; 106 commits
`v0.2.29..v0.2.30`) and one interpreter release (0.1.53, release
413375414; 26 commits). The trace pin stays collapsed onto `[lupin].rev`:
`src/eval/placetrace.rs` is byte-identical between v0.1.52 and v0.1.53
(the release touches `builtin.rs`, `fs.rs`, CI and tests).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.30 (wolfgang, pin 1b37f36)
paired with lupin 0.1.53 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.53 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `1b37f368` (`lupin-version = 0.1.53`, `lupin-pin =
294d626`), `vendor/upstream/PIN` at `v0.1.53` (`294d626d…`) and D57. The
interpreter's clause is now **SIX releases behind**: `294d626` is
v0.2.24's tag, an ancestor of `1b37f368`, **717** commits and six
releases (v0.2.25 … v0.2.30). ch01 §1.2's "five releases … 611" and the
colophon's matching sentence move to six and 717. Falsified by any
character of the stamps that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 640 → 648**, none dropped, none retargeted. Added (8):
  `os.fs.owner`, `os.proc.ids`, `os.proc.ids.note`, `os.proc.ids.set`,
  `os.signal.wait.fd`, `os.term.chars`, `os.term.editor.note`,
  `os.term.size`.
- **`grammar.ebnf` does not move.** Appendix A does not regenerate.
- **Catalogue 187 → 187**; `vendor/spec/diagnostic-codes.txt` does not
  move. E1010's explanation grows by one paragraph (s233); the book
  prints no `--explain E1010`, so no block moves for it.
- `help.rs` has no diff: Appendix E's `wolf --help` holds.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.29's member names. `std/`: 57 files, 53 `.lu`,
  `STD-REV` = `fd6bcdcb1dfe03ea5092f10316b7abe81f2df41e`; it differs from
  0.2.29's in `std/env/env.lu`, `std/signal/signal.lu`,
  `std/term/term.lu` and `STD-REV` only, so its manifest moves off
  `229202f5…` and **the book's CI staging step is red at the bare pin**
  until the archive's new manifest is measured and written. Byte-identical
  to 0.2.29's: the four licences and (a guess, as last time) `_wolf`,
  `wolf.bash`, `wolf.fish`. Move: `wolf`, `wolf.1`, `README.md`,
  `wolf-cimport-worker`, `libwolf_rt.a`, `libwolf_rt_none.a`.
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE`, `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 515 samples: 512 passed, 3 pending, 4 failed; 484 of 508
console blocks**, against the archive whole, in the container. The four
are console blocks, text only, no verdict:

1. `book/ch01.md` `--version`; 2. `book/back/colophon.md` `--version`;
3. `book/ch22.md` `wolf interface`: `toolchain 0.2.29` → `0.2.30` on
   both stamp lines, **every hash held** (the listing reaches `std.range`
   and `std/range` has no diff `d7c2968..fd6bcdc`);
4. `principles/exercises/ch22/EXERCISES.md`, 22-13: the toolchain line.

On a bare `wolf` (no `std/` beside it): the same four.

What does **not** fail, and why: the eleven new prelude names (no book
program declares or calls one; grep 0, and the CHANGELOG's own grep of
`0a5fecc0`); s233's must-join rule (E1010 at a call that hands a
region's bytes to a function that may leave a proc running): the book's
71 `spawn proc` sites are each joined or monitored in the frame that
spawns them, as far as I read — **the row I am least sure of**,
falsified by any new E1010 or any `fail(E1010)` block whose diagnostic
grows a line; s234's mode bits (no book program sets a terminal mode);
s755 (no book program panicked on 0.2.29: bs66's sweep has no such row);
pending 3 → 3 in the same words.

## 6. The sweep: subject against control (PREDICTED)

580 programs (`allprogs.py`), each alone, on lupin, `conform-run --json`
(native), `conform-run --json --checked`, and a `--release` build and run
for every native `pass` with a `main`; each pair twice, inside the
container.

- **lupin: 0 rows** outside the control's own noise.
- **native: 0 rows.** **checked: 0 rows**, ex33-7 still killed at 120 s
  (wolf-lang#720, open; nothing in 0.2.30 names it).
- **release: noise only** (bs66's set: B-11 and ex9-4's exit after UB,
  ch17's balances, ex17-1, ex11-5, ex13-4a, ex30-4/ex30-5, ch10's first
  scope program).

## 7. The new chapter's programs on four machines (PREDICTED from the spec at v0.2.30, std `fd6bcdc` and lupin `v0.1.53`'s `builtin.rs`)

Chapter 34, *Signals and the process*, at the end of Part 4 (the ch33
precedent: the next free number). Its programs, as drafted (the page may
change a line; a change is recorded in the results):

| program | native (`wolf run`, `conform-run --json`) | checked (`conform-run --json --checked`) | release | lupin 0.1.53 (its own archive, no std) |
|---|---|---|---|---|
| `reload.lu`: listen `reload()`, raise it, wait on `reload() \| terminate()` | `heard reload: true`, exit 0 | the same (the in-machine queue) | the same | `unsupported` (no std), exit 4 |
| `sets.lu`: `quit()` in a set of three, `reload()` not, a listen of a set holding `kill()`, of the empty set | `true`, `true`, `io`, `io` | the same | the same | `unsupported` (no std) |
| `rows.lu`: `signal.send` to this process with 0, with two meanings, to an id nobody holds; `signal.number(user())? > 0`; `ignore`, `ignored`, `reset` | `ok`, `invalid`, `not_found`, `true`, `true`, `false` | the same | the same | `unsupported` (no std) |
| `alive.lu`: `os_signal_send(os_pid(), 0)?` | `this process may be signalled` | the same | the same | **refused by name**, exit 4: «`os_signal_send` signals a process by id: this machine admits no `unsafe` and std has no `kill(pid, sig)`» (0.1.52's words, unchanged in 0.1.53's source) |
| `who.lu`: `env.pid() > 0`, `env.ppid()? > 0`, `os_uid()? != os_euid()?`, `os_gid()? != os_egid()?` | `true`, `true`, `false`, `false` | the same | the same | `unsupported` (no std) |
| `modes.lu`: `term.raw` of a cooked `term.Mode` with `flow` and `extended` on; `pack(unpack(pack(raw)))` | `false`, `false`, `true` / `false`, `false`, `1` / `true` | the same | the same | `unsupported` (no std) |
| `window.lu`: `os_term_size(1)`, `os_term_chars(1)` with standard output captured | `io`, `io` | `io`, `io` | `io`, `io` | refused by name at `os_term_size` |
| `ex34-1.lu`: listen `interrupt() \| terminate()`, raise `terminate()`, wait, poll | `arrived: terminate`, `still queued: 0` | the same | the same | `unsupported` (no std) |
| `ex34-7.lu`: all 28 names numbered, how many a listen takes, two at once | `28 meanings, 28 numbered here`, `20 can be heard`, `two at once: invalid` | the same | the same | `unsupported` (no std) |

So every chapter-34 program is a `wolf-run(…)` block. In CI's three
samples lanes (a source-built `wolf` with the archive's `std/` staged
beside it): ubuntu and macOS pass all of them; **windows** passes
`reload.lu` (a listen for `reload()` is served there and a self-raise is
delivered in process), `sets.lu` (the shape rule comes first) and
`modes.lu` (pure), and answers the others with a row: `unsupported` for
`send`, `number`, `ignored`, `ignore`, `ppid`, the four id reads and the
two terminal calls, `io` for a listen of `interrupt()`. Those become
`[[refusal]]` rows in `samples-os.toml`, each written from that lane's
own log. Falsified by any cell that answers otherwise.

## 8. Sentences the release makes false (found by reading, before any archive)

1. ch01 §1.2 and the colophon: the stamps; "five releases … 611" → six,
   717; the Windows archive names; the `pin 6ffe7c6` sentences; the
   `0.2.29+dev.unknown` example.
2. ch22 §22.2: "`toolchain 0.2.29` is which compiler printed the page".
3. The version-literal ledger: every `audited-at` re-read at 0.2.30 /
   0.1.53.
4. `wolf-toolchain.toml`'s `[wolf-std]` note and the CI step's comment
   and `want`.
5. **#80, not the release:** exercise 33-7's "`net_wait` neither parks a
   task nor disturbs the scheduler" (since 0.2.29 its poll tells the
   pool its worker is waiting); §33.3's parenthesis still matches
   `[os.net.wait]` at v0.2.30 word for word ("neither parks a task nor
   disturbs the reactor's registrations") and gets one clause about the
   pool. **§13.1:** "the native root arena serializes allocation behind
   one lock (`[conc.task.par.cost]`)" is false of that clause since
   0.2.27 (s222).

## 9. §13.1 on kasumi (PREDICTED)

The printed program (72 pieces of 379,900 characters) and the two
reconstructions, built by 0.2.15 and by 0.2.30, timed in alternating
rounds (`par.sh`). Predicted: at **0.2.15** the allocating counter's
`par` is **slower** than its serial twin (s222's own kasumi measurement
before its change read 0.77 s against 0.18–0.21 s), and the other two
rows are faster with `par`; at **0.2.30** all three rows are faster with
`par`, the first by at least 3x. Every program prints the same line on
both toolchains. Falsified by a 0.2.30 first row that is not faster, or a
0.2.15 first row that is. The page's table stays as measured (dated, on
the machine it names); the kasumi numbers go beside it.

## 10. Back matter and counts at head (PREDICTED)

- No snapshot re-blessed. Four console blocks spliced from the run;
  `solutions.md` regenerated by `backmatter` (22-13's toolchain line,
  33-7's wording, chapter 34's solutions).
- `backmatter --check` against wolf-lang at v0.2.30: `anchors.json`
  re-vendored (648); codes and grammar unchanged.
- `diagrams --check` 6 from 4 with lupin 0.1.53 as `LUPIN_TRACE`:
  byte-identical.
- `ledger --check`: 126 open (119 filed, 7 waived) plus chapter 34's rows,
  every one filed or waived; 109 closed.
- Head, kasumi, in the container, archive whole: **524 samples** (515 +
  seven chapter blocks + two exercise programs), 521 passed, 3 pending, 0
  failed, 0 flips; console blocks 508 + chapter 34's (one, `lupin
  alive.lu`, spliced from the run), every one replayed. Also under
  `taskset -c 0-3`.
- CI at head: green on three hosts; windows with the `[[refusal]]` rows of
  §7.
- CI on the **unedited book at the pin** (the pin and the staged std's
  manifest, nothing else): samples **red on three hosts** on exactly the
  four blocks of §5.

## 11. Method

Control and subject on the same unedited tree, the toolchain the one
variable, on kasumi inside the container; the sweep twice per pair. Then
the pin (own commit), the CI manifest, a push of the unedited book for
the red run, the consoles spliced from that run, then the edits: the
pin's sentences, #80, chapter 34 and what a chapter must touch, its
windows rows from CI's own log. Then the head gate (`samples`, also under
`taskset -c 0-3` and on the bare `wolf`, `--self-test`, `verify-docs`,
`ledger --check`, `backmatter --check`, `contrast`, `diagrams --check`,
`render web`, `tiers --check`, xtask's tests with `--nocapture`), every
log full `2>&1`, SKIPs counted.
