# bs59 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-04 against wolf-book `731f966` (the contract,
an empty commit on bs58's merged trunk `fba6210a`), **before**
`wolf-0.2.22-*.tar.gz` or `lupin-0.1.45-*.tar.gz` was downloaded, and
before any file in this tree other than this one and the three
scripts beside it (`bs59-evidence/scan.py`, `oneprog.py`, `probe.sh`)
was written. DERIVED means computed from the two upstream git
histories at the tags; PREDICTED is a claim about what the release
binaries will do, with the number or name that falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book already carries — wolf 0.2.21 `09e0a6f5…` and lupin 0.1.44
`e44aae06…`, downloaded on kasumi and read against the release API the
same minute. On the unedited head `731f966`: **509 samples, 506 passed,
3 pending, 0 failed, 0 flips in 36.6 s, 482 of 502 console blocks,
export 377, 125 held back** — bs58's own head numbers, so the empty
contract commit moved nothing. `ledger --check`: **131 open (124 filed,
7 waived), 101 closed**. `verify-docs`: 345 exercises (296 printed),
tiers 192 / 9 / 31 / 27 / 78 / 8. Every one-machine fence, every
pending row, and the four programs this file names were probed one at a
time on the control pair (`lupin`, `wolf run`, `wolf conform-run
--json` with and without `--checked`): `bs59-evidence/probes-control.txt`.

## 1. The pin (DERIVED)

```
[wolf]   rev dfcc2f13e7c73182bdd41fc9bec2802c7da3b024 -> 8e36bc1a0f92bbbbc6861b10d5b2638f76412d6a
         impl_version 0.2.21 -> 0.2.22
[lupin]  rev ba4762714d75c37bb14b300610e2d52630665a05 -> 9f4e4a175da109bc4dc0acec3e8a4bc54358b941
```

One compiler release, one interpreter release.

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.22 (wolfgang, pin 8e36bc1)
paired with lupin 0.1.45 (reference interpreter), pin dfcc2f1
$ lupin --version
lupin 0.1.45 (wolf-interp, reference interpreter at pin dfcc2f1)
```

From `crates/wolf_driver/PAIRING` at `8e36bc1a` (`lupin-version =
0.1.45`, `lupin-pin = dfcc2f1`), `vendor/upstream/PIN` at `v0.1.45`
(`dfcc2f13…`), and D57. The compiler's half is **exact** (it names the
lupin the book pins). The interpreter's half is a distance again:
`dfcc2f1` is **v0.2.21's tag**, an ancestor of `8e36bc1a`,
**222** commits and **one** release behind
(`git rev-list --count dfcc2f13..8e36bc1a`). §1.2's and the colophon's
"91" become "222"; "one release" and "the compiler's previous release
tag" hold. Falsified by any character that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 546 → 569**, key sets diffed both ways: added (23)
  `abi.asm`, `abi.asm.inline`, `abi.asm.link`, `abi.asm.machines`,
  `abi.asm.roster`, `abi.c.export`, `abi.c.import`, `abi.layout.c`,
  `abi.target`, `abi.target.entry`, `abi.target.none`,
  `abi.target.none.alloc`, `abi.target.none.codegen`,
  `abi.target.none.hooks`, `gram.item.attr.cfg`, `gram.item.attr.set`,
  `mem.unsafe.sig`, `os.fs.read_at`, `os.fs.seek`, `os.fs.std`,
  `os.fs.tell`, `proto.record.first`, `type.numlit.cast.narrow`;
  **none dropped**. Every one sits in a namespace that already exists
  (`abi`, `gram`, `mem`, `os`, `proto`, `type`), so Appendix D's
  thirteen namespaces hold.
- **`grammar.ebnf` holds**: `910ff9d5…` at v0.2.21 and v0.2.22.
  Appendix A does not move.
- **Catalogue 178 → 181**: E0817, E0818, E1306 added (the `## Ennnn`
  heads of `docs/diagnostics.md` at each tag, the derivation
  `backmatter.rs` uses). Appendix C's "The catalog holds 178 codes"
  becomes 181. The registry entries that changed text are E1302's
  and the three new ones; no page runs `wolf --explain` on any of
  them (the book explains E0201, E0202, E0701, E1001, E1002, E1010,
  E1011, E1012, E1401, E1503, E1504, E1506), so **no `--explain`
  transcript drifts**.

## 4. Members and floors (PREDICTED)

- wolf archive: the same ten members by name as 0.2.21 (`wolf`,
  `wolf-cimport-worker`, `libwolf_rt.a`, `wolf.1`, `_wolf`,
  `wolf.bash`, `wolf.fish`, `LICENSE`, `LICENSE-EXCEPTION`,
  `README.md`); no std tree yet (s204 is not in 0.2.22's CHANGELOG).
  `_wolf` `2d1e4801…`, `wolf.bash` `29eb2d75…`, `wolf.fish`
  `51a1b6a8…` and `LICENSE` `3972dc97…` **byte-identical**
  (`help.rs`'s `completions` is untouched between the tags); `wolf.1`
  **moves** (the man page carries the command table, and `build`'s
  usage gains `[--target x86_64-unknown-none]`).
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE` (`3972dc97…`),
  `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 4 program failures and 5 console failures.**

| sample | directive | why it fails at 0.2.22 / 0.1.45 |
|---|---|---|
| `ch20/ex20-5` | `run(exit=0, stdout="3")` | `wolf run` refuses: **E0817** «`noalloc` is not implemented yet», label «known, but nothing implements it», the note naming I15 and wolf-lang#180 (`attrs.rs`'s `not_yet`), exit 2. lupin 0.1.45 still prints `3`: it reads no attribute (wolf-interp#174, kept by r27's pairing). Control: `3` on both. |
| `ch20/ex20-8` | `run(exit=0, stdout="13")` | the same E0817 on `wolf run`; lupin `13`. Control: `13` on both. |
| `book/ch09/s3` | `fail(E1302)` | the compiler's verdict becomes **`pass`**: `header_len` is module-private, and K9(b) = B (`[mem.unsafe.sig]`, kw02) lets a module-private fn carry `*T`. lupin 0.1.45, pinned at v0.2.21, still refuses it E1302 (wolf-interp#181), but a `fail(…)` fence reads only the compiler. Control: `fail(E1302)` on both compiler lanes, lupin E1302. |
| `book/ch06/part-wordcount` | `lupin-run(exit=0)` | lupin 0.1.45 no longer trims a cutset (is68, wolf-interp#125): `word.trim(".,;!?")` is **E0402** at resolve where the receiver is visibly a `str` (exit 2), or a by-name decline (exit 4) where it is not. Either way not exit 0. The compiler still declines the program for the top-level `let USAGE` («an item without a declared type (E0407 upstream)», control and subject alike). |

Console failures, by name: the four stamp sites (`book/back/colophon.md:7`,
`book/ch01.md:153`, `book/ch22.md:288`,
`principles/exercises/ch22/EXERCISES.md:218`) and the wordcount
transcript (`book/ch06.md:900`, `$ lupin wordcount.lu`). The ch20
corpus transcripts (`$ lupin ex20-5.lu` → `3`, `$ lupin ex20-8.lu` →
`13`) **hold**, for #174's reason.

Runner arithmetic: **509 samples, 502 passed, 3 pending, 9 failed
(4 programs + 5 consoles), 0 flips; 477 of 502 console blocks.**

What does **not** fail, each checked by `bs59-evidence/scan.py` over all
577 program texts:

- **E0817** beyond the two: the only attributes outside the implemented
  set are the two `#[noalloc]`s. **E0818**: no `extern` ABI string.
  No `cfg`, `export fn`, `extern "c"`, `asm`.
- **`[mem.unsafe.sig]`**: one module-private fn with a `*T` in its
  signature, s3. `pack` (§9.5, §9.7) takes and returns integers.
- **`[type.numlit.cast.narrow]`**: seven integer casts, all in ch32's
  allocator, all of non-negative values into `i64` or `uint`; none can
  trap. `int as byte` (§2.3) is `[type.byte.cast]`'s own truncating
  bridge, which the clause leaves alone.
- **E1010 for a `for` piece (#540)**: 22 `for` loops over
  `words`/`lines`/`split` in programs with a region; every iterable is
  a literal, a parameter or a `str` built outside any region, which
  s207's rule lets leave freely. None is refused.
- **Handles**: no program prints an `fs_open` handle (the first is 3
  now); no program calls `fs_open` at all.
- **First diagnostic (`[proto.record.first]`)**: the four multi-error
  snapshots (ch13 s5, ex7-9b, ex12-9, ex13-3) are one code each, so
  "which code is first" cannot move them.
- **lupin 0.1.45's other changes**: no nested fn with a moded parameter
  (#169), no unit-context tail the 19 lupin-run fences carry (#103),
  no view out of a region on a lupin-run fence (#126; ch16's ring
  copies a title inside the same region), one cutset `trim` (#125,
  above).

**Pending: 3 → 3**, in the same words at the same spans on both pairs
(ex5-8 @300..303, ex7-5 @365..367, ex8-7 @1926..1929). **lupin-run:
18 of 19 non-trap fences hold their refusal** (the 19th is wordcount,
above); **wolf-run: 6 of 6 hold** (lupin exit 4 on the five `comptime
fn` programs; ch33's prefork). **Trap rows**: B-5 still
`trap(exclusivity)` on lupin. Falsified by any flip, any failure not in
the table, or a table row that passes.

## 6. The `#[noalloc]` sites, decided (before the archive; §7 of the results measures them)

- **`ex20-5.lu` becomes `fail(E0817)`.** The exercise is "a broken
  promise": the attribute IS the subject, and without it there is no
  promise to break, so dropping it would delete the exercise. Under
  E0817 the program is refused before any checker could run, which
  is the honest verdict today and the stronger one: the compiler that
  used to run a lie now will not compile it. It stays the pending row
  in EXERCISES-PENDING.md (the verifying rejection, with its own code,
  is still #180's), with the directive column reading what the tools
  do today.
- **`ex20-8.lu` becomes `fail(E0817)` too.** The stem asks the reader
  to "state precisely what today's toolchain claimed about your
  attribute". At 0.2.21 the answer was "nothing"; at 0.2.22 the
  compiler refuses the attribute by name and lupin still ignores it.
  The exercise's point is exactly the question that moved, so the
  sample keeps the attribute and the solution answers the new way,
  per machine. Without the attribute it would be a dot product and no
  exercise.
- **EXERCISES.md's 20-8 block** quotes the file and keeps quoting it
  (wolf-book#48's rule, by hand); its `$ lupin ex20-8.lu` transcript
  holds and the prose beside it says what each machine does.

## 7. Sentences the release makes false (found by reading, before any archive)

1. **ch09 §9.2**, "A function's signature is safe or the program does
   not compile", and s3 itself: module-private signatures and the C
   membrane may carry `*T` now. s3 becomes `pub fn header_len`, which
   keeps E1302; its snapshot takes the 0.2.22 note («unsafety never
   appears in types crossing a module's boundary — … A `*T` may stand
   in a module-private fn's signature or at the C membrane …»); the
   paragraph after it is rewritten around the module as the granule.
2. **ch09 §9.5**, "`pack`'s signature is fully safe. It has to be;
   §9.2's E1302 saw to that." `pack` is module-private; nothing forces
   it now. It is safe by choice.
3. **ch32 §32.2**, "no `*T` crosses a function signature".
4. **ch06 §6.5** (the capstone), "`trim(".,;!?")` removes any of those
   characters from both ends": the spelling `[mem.str.ws]` has ruled
   out since 0.2.15 and that lupin now refuses too. `count` takes a
   small helper that slices the punctuation off; the transcript is
   predicted byte-identical (`17 words, 11 distinct`, `the 5`, `wolf
   2`, `moon 2`, `runs 1`). The ch06 ledger row that said the exercise
   "wants rewriting when the interpreter catches up" gets its note.
5. **ch20's corpus prose** (EXERCISES.md's set header, 20-5, 20-8,
   20-10): "the four attributes parse and are verified by nothing",
   "`wolf conform-run` reports `verdict=unsupported`" (already `pass`
   at the control), and 20-10's quotation of `corpus/comptime.lu`,
   whose comment at v0.2.22 says the opposite (the attribute is
   refused by name, E0817).
6. **ch20 and ch18 audit ledgers**: "parse and are verified by
   NOTHING" — re-measured notes, rows stay open (#180 is open).
7. **EXERCISES-PENDING.md** (20-5's row and "19-1: `#[noalloc]`
   parses"), **samples-pending.toml**'s header ("20-5's program runs
   green today"), and **25-5**'s (c) (adding `#[noalloc]` does not
   compile today).
8. **ch20 §20.1** gains one sentence: the names such annotations would
   take are refused by name, E0817, until a checker exists. That is
   the one new code a page prints, so **Appendix C gains E0817** and
   "these 59" becomes **60**; the index gains `E0817 20.1`.

## 8. Back matter and counts at head (PREDICTED)

- Head run: **509 samples, 506 passed, 3 pending, 0 failed, 0 flips;
  482 of 502 console blocks**; snapshots blessed for ex20-5, ex20-8
  (new) and s3 (moved).
- `ledger --check`: **131 → 132 open** (re-counted): one row added in
  chapter 9 for the new two-machine split on a module-private `*T`
  signature (wolf runs it, lupin refuses E1302, wolf-interp#181);
  none closed (#180 and the ch06 row stay open).
- `verify-docs`: 345 exercises (296 printed); 20-8's checker becomes
  `wolf + lupin`, so tiers **191 / 9 / 31 / 28 / 78 / 8**.
- Export: **377** (a guess at the export rule, stated as one: `fail(…)`
  files are exported like `run(…)` ones).
- `backmatter --check` against a wolf-lang checkout at v0.2.22: the
  vendored trio matches after the re-vendor; solutions.md drifts on
  22-13's `toolchain` line, as at every bump.

## 9. Open issues the pin moves

None of wolf-book #60, #58, #57, #48, #45, #43, #29 moves: their
programs answer in the same words (§5). #48 is touched in passing (the
20-8 quote is edited with its file).

## 10. Method

Control (above) and subject on the same unedited tree, the toolchain
the one variable; the probes repeated on the subject pair; then the
edits; then the head gate (`samples`, `--self-test`, `verify-docs`,
`ledger --check`, `backmatter --check`, `contrast`, `render web`). All
on kasumi from the release archives by digest; CI builds both tools
from source at the pinned revs on three hosts.
