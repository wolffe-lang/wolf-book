# bs58 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-03 against wolf-book `307a932` (the contract,
an empty commit on bs57's head `280ff370`), **before**
`wolf-0.2.21-*.tar.gz` or `lupin-0.1.44-*.tar.gz` was downloaded,
before any file in this tree other than this one was edited. DERIVED
means computed from the two upstream git histories; PREDICTED is a
claim about what the release binaries will do, with the number or name
that falsifies it.

What ran before this file, and is allowed to: the **control** pair
(wolf 0.2.19 `9f3873d8…`, lupin 0.1.42 `9856335a…`, both re-hashed
against the live release API on kasumi), which is the pin the book
already carries. On the unedited head it gave **509 samples, 503
passed, 6 pending, 0 failed, 0 flips, 481 of 502 console blocks,
export 377, 125 held back** — bs57's own head numbers, so the carried
branch is unchanged by the empty contract commit. Every one-machine
fence and every pending row was also probed one program at a time on
the control pair (`wolf conform-run --json`, `--checked`, `lupin`),
which is what names the refusal each prediction below leans on.

## 1. The pin (DERIVED)

```
[wolf]   rev c2401f05f37794a078d2acf62f837dad98e5950d -> dfcc2f13e7c73182bdd41fc9bec2802c7da3b024
         impl_version 0.2.19 -> 0.2.21
[lupin]  rev 8e2516dc47bf808512388cc687e070981d331d98 -> ba4762714d75c37bb14b300610e2d52630665a05
```

The book never pinned 0.2.20: the span is two compiler releases
(246 commits) and two interpreter releases (90 commits).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.21 (wolfgang, pin dfcc2f1)
paired with lupin 0.1.44 (reference interpreter), pin cdde128
$ lupin --version
lupin 0.1.44 (wolf-interp, reference interpreter at pin cdde128)
```

From `crates/wolf_driver/PAIRING` at `dfcc2f13` (`lupin-version =
0.1.44`, `lupin-pin = cdde128`), `vendor/upstream/PIN` at `v0.1.44`
(`cdde128a…`), and D57. The compiler's half is **exact** again (it names
the lupin the book pins). The interpreter's half is a distance:
`cdde128` is **v0.2.20's tag**, an ancestor of `dfcc2f13`, **91**
commits and **one** release behind (bs55 wrote 105 and one). §1.2's and
the colophon's "105" become "91"; "one release" and "the compiler's
previous release tag" hold. Falsified by any character that differs or
a count other than 91.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 542 → 546**, key sets diffed both ways: added
  `mem.tier0.excl.4`, `type.row.else` (both 0.2.20), `type.row.defer`,
  `type.row.match` (0.2.21); **none dropped, none remapped**. All four
  sit in namespaces that exist (`mem.*`, `type.*`), so Appendix D's
  eleven documents and thirteen namespaces hold.
- **`grammar.ebnf` holds**: `910ff9d5…` at v0.2.19, v0.2.20 and
  v0.2.21. Appendix A does not move.
- **Catalogue 176 → 178**: E0611 and E0816 added; every other entry
  moves only in its `Fixtures:` line, which `--explain` does not print.
  `vendor/spec/diagnostic-codes.txt` gains two lines; Appendix C's
  sentence "The catalog holds 176 codes" becomes 178 (verify-docs reds
  on it the moment the vendored file moves, and that red is expected);
  "these 59" holds, because no page prints E0611 or E0816 (§6.6 names
  the E0816 rule in prose, without the code). E0801's *Sections* cell
  gains 6.6, which is accuracy, not a gate.

## 4. Flips at the subject run — **3**, all pending rows (PREDICTED)

The subject run is the unedited head with the new pair. The three rows
bs57 added to `samples-pending.toml` flip, by name:

| sample | directive | wolf 0.2.21 | lupin 0.1.44 |
|---|---|---|---|
| `book/ch06/part-rowmatch` | `run(exit=0)` | runs, exit 0, `340 cents` / `nothing owed` / `` `lots` at byte 7 is not a number `` / `no comma` on `wolf run`; the same four lines from `conform-run --json --checked` | runs, exit 0, the **same four lines** (is67: a call to a module fn is read by its declared row, so `no_comma` and `NotANumber(bad)` take only the row, `0` and `n` only the value) |
| `book/ch06/s7` | `fail(E0801)` | `fail(E0801)`, «this `match` does not cover `no_comma`», rendered **byte-identical** to `snapshots/diagnostics/book__ch06__s7.txt` (the note's text at `check.rs:11600` in v0.2.21 is the snapshot's, word for word); checked lane the same verdict | `fail(E0801)` at the resolve rung naming `no_comma`, exit 2 — not asked by the runner (a `fail(…)` fence reads the compiler's verdict only), probed for the record |
| `ch06/ex6-15` | `run(exit=0, stdout="7 -4 -99")` | `7 -4 -99`, exit 0, on `wolf run`; `--checked` still refuses `Weird` as a module item (`unsupported`, the checked machine's limit 6-6 shares), which the runner does not ask | `7 -4 -99`, exit 0 (`Weird` is a row value, so it skips `v` and lands in `_`) |

Runner arithmetic, from bs57's s197 summary (a flipped pending row
counts neither as passed nor as pending): **509 samples, 503 passed,
3 pending, 4 failed, 3 flips**.

**No `lupin-run(…)` fence flips (0 of 19 non-trap fences).** Each was
probed on the control compiler and each refusal is about a construct
neither CHANGELOG touches:

- «interpolation inside a multiline string» — `book/ch02/…:145`
- «`break` with a value (loop values)» — `ch03.md:407`, ex3-13
- E0301 — `ch03.md:550`
- «an item without a declared type (E0407 upstream)» — `ch06.md:880`
  (the wordcount part)
- «methods on builtin types (the std surface)» — `ch08.md:221`, `:433`
- E0403 — `ch08.md:823`
- «indexing / generic application» — `ch09.md:472`, ex9-7a
- E0415 ×8 + E0401 ×2 — ex9-15 (a STATIC refusal; s202's #458 makes
  more literals `i32`, never fewer, so it cannot heal here)
- E0604 — `ch15.md:86`; E1010 — `ch16.md:27` (the ring part)
- «row payload slots with conflicting types across tags (spilled
  union layout, c06)» — ex27-3. This is the one that sits near a
  release topic (rows), and it is about **layout**, not `match`:
  0.2.21's `match` over a fallible value changes nothing about how a
  row's payload slots are laid out
- «`copy` of a value nested this deep» — ch28's wordtree part and
  ex28-1/2/3/6 (push's plain copy, wolf-book#57)

**No `wolf-run(…)` fence flips (0 of 6):** ex18-3/5/11/15 and ex22-7
are `comptime fn` (lupin exit 4) and ch33's prefork is `os_spawn_with`
fd inheritance; nothing in lupin 0.1.43 or 0.1.44 touches either.

**Trap rows** carry no automatic flip, and none moves by hand: the one
`trap(exclusivity)` row, appendix B-5 (`wide(mut p.a, mut p.a.n)`), is
two `mut` claims of a path and its prefix — a second claim, which
ruling #17 keeps E1002 and lupin keeps trapping; two-phase only frees
a later argument's READ.

**The per-host row** (`samples-os.toml`, `book/ch33/part-prefork` on
windows) holds; wolf-lang#235's crossing is not in either release.

**Falsified by** any flip not in the table, or any table row that does
not flip.

## 5. Failures at the subject run — **4**, all console (PREDICTED)

The four stamp sites and nothing else: `book/back/colophon.md:7`,
`book/ch01.md:153` (the `--version` pair), `book/ch22.md:288` and
`principles/exercises/ch22/EXERCISES.md:218` (the interface's
`toolchain 0.2.19` line). Console blocks: **477 of 502** at the subject
run. **0 program failures**, for these reasons, each checked by a static
search over all 577 program texts (`bs58-evidence/scan.py`):

- **E0611** (`?` under `defer`/`errdefer`): 0 hits.
- **#458** (an unannotated binding's literal must fit `i32`): 0
  bindings whose literal or literal term leaves `i32` (ex9-15's
  E0415s are already there at the control).
- **#486** (field shorthand moves): 0 shorthand initializers; ex7-19's
  two hits are struct PATTERNS.
- **#484** (moded fn as a value), **#476/#487** (a write in a later
  argument): 0 hits.
- **#499** (a block's `errdefer`): the book's two `errdefer`s (§6.3,
  ex6-5) are function-level, which ruling #20 leaves alone.
- **#492** (`else` and a `?` in its scrutinee): 0 hits.
- **E1002 wording**: every book snapshot's phrase is in v0.2.21's
  source, and `overlap_note` is byte-identical at both tags, so the
  nine E1001/E1002 snapshots hold (§7.4's s13, §21.1's s2, ex7-8,
  ex7-9b, ex7-23 and the four E1001s).
- **E0801 wording** for §3.4's ordinary `match` (`book/ch03/s16`): the
  label and the closing note are unchanged; the row-match note only
  prints over a fallible value.
- **No `--explain` transcript drifts**: only `Fixtures:` lines moved in
  the entries the book explains.

## 6. Pending: **6 → 3** (PREDICTED)

The three bs57 rows leave as reported FLIPs (§4). The three standing
rows hold **in the same words at the same spans** on both pairs:
ex5-8 («member access on a value whose type is still being inferred»
@300..303; lupin's `std.list` refusal), ex7-5 («borrow expressions»
@365..367), ex8-7 («`==` on a container, tuple or handle» @1926..1929;
lupin «handle is not a `Map` key»). Nothing in either release is about
std roots, borrow expressions, `Eq` or map keys.

## 7. The #176 parting — **none in the book** (PREDICTED)

wolf-interp#176 is a `match` over a call whose row is INFERRED empty
(`fn f() -> !int`). The book has exactly three `match`es over a
fallible value (§6.6's two, 6-15), and all three rows are SPELLED:
`int ! {no_comma, NotANumber(Bad)}` (closed) and `int ! {Io(int), ..}`
(open). No book program matches over a `-> !T` fn. Nothing is pinned
by version. Falsified by any lupin E0801 on a book program other than
s7.

## 8. Rulings #19, #20, #33 — **no row to advance** (DERIVED)

The book has no pending row, ledger row or sample for `?` under a
defer (#19), a block's `errdefer` on a row value (#20), or an
unannotated binding's literal (#33). One existing open row sits next to
#33: ch05's `ba:diagnostic (bs43)` (wolf-lang#347, CLOSED 2026-09-15),
whose second half says the spec disagrees with itself because
`[type.numlit.value]` called `let n = 0` an `int`. s202 rewrote that
clause: `let n = 0` is fixed at the binding as `i32` unless a later use
decides it. So that half is RESOLVED at v0.2.21. Its first half is a
behaviour, re-probed on both pairs: `refund(340)` and `let cents = 340`
then `refund(cents)` under `impl Neg for int` — PREDICTED unchanged,
«E0502: `i32` does not implement `Neg`» on both compilers (a generic
call is not a use that decides the binding), lupin `-340`. The row
gets a re-measure note, not a close, and the closed issue is reported.

## 9. Ledger, index, back matter (PREDICTED)

- `ledger --check`: **133 → 132 open** (re-counted), closed 99 → 100.
  bs57's `ba:blocker` row on ch06 closes with the three rows.
- `verify-docs`: corpus 266 holds; EXERCISES-INDEX 345 (296 printed),
  tiers **192 / 9 / 31 / 27 / 78 / 8** (6-15 moves from pending to
  `run (wolf + lupin)`).
- `backmatter --check`: 296 / 331 holds; solutions.md regenerated only
  if 6-15's pending apparatus leaves its comment (the `.lu` header
  comment says "Pending"; it is rewritten).
- Export: **377 → 380** at head (the three graduated programs join;
  a guess at the export rule, stated as one).
- §13.1's timings (0.2.15 history sentence): re-read, not re-measured.

## 10. Members and floors (PREDICTED)

Both linux x86-64 binaries carry no `GLIBC_` symbol above **2.34**
(`objdump -T`). The archives' members, hashed by name, include `wolf`,
`_wolf` (the 1,913-byte zsh completion, `2d1e4801…` since 0.2.13) and
the runtime library.

## 11. Method

Control (above) and subject on the same unedited head, the toolchain
the one variable; then each §6.6 program and each standing pending
program probed one at a time on both pairs; then the edits; then the
head gate (`samples`, `--self-test`, `verify-docs`, `ledger --check`,
`backmatter --check` against a wolf-lang checkout at `v0.2.21`,
`contrast`, `render web`). All on kasumi from the release archives by
digest; CI builds both tools from source at the pinned revs on three
hosts.
