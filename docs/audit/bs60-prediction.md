# bs60 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-04 against wolf-book `a23a061` (the contract,
an empty commit on bs59's merged trunk `f1d894ec`), **before**
`wolf-0.2.23-*.tar.gz` or `lupin-0.1.46-*.tar.gz` was downloaded, and
before any file in this tree other than this one and the files under
`bs60-evidence/` (`scan.py`, `oneprog.py`, `probe.sh`, `extra/`) was
written. DERIVED means computed from the two upstream git histories at
the tags; PREDICTED is a claim about what the release binaries will do,
with the number or name that falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book already carries — wolf 0.2.22 `df0f2fea…` and lupin 0.1.45
`907cfb1a…`, downloaded on kasumi and read against the release API the
same minute (19:30Z). On the unedited head `a23a061`: **509 samples, 506
passed, 3 pending, 0 failed, 0 flips in 29.9 s, 482 of 502 console
blocks, export 377, 125 held back, 4 SKIP lines** (bs59's own head
numbers, so the empty contract commit moved nothing). `ledger --check`:
**132 open (125 filed, 7 waived), 101 closed**. `verify-docs`: 345
exercises (296 printed), tiers 191 / 9 / 31 / 28 / 78 / 8. Every
one-machine fence, every pending row, the named programs below and five
hand-written witnesses (`bs60-evidence/extra/`) were probed one at a
time on the control pair (`lupin`, `wolf run`, `wolf conform-run --json`
with and without `--checked`): `bs60-evidence/probes-control.txt`.

## 1. The pin (DERIVED)

```
[wolf]   rev 8e36bc1a0f92bbbbc6861b10d5b2638f76412d6a -> 8edac3eeb48632b32f02ef41ea87d484d1423492
         impl_version 0.2.22 -> 0.2.23
[lupin]  rev 9f4e4a175da109bc4dc0acec3e8a4bc54358b941 -> f9269e3364169585fdfe92407639418220be373b
```

One compiler release (wolf-lang release 403069562), one interpreter
release (wolf-interp release 403040421).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.23 (wolfgang, pin 8edac3e)
paired with lupin 0.1.46 (reference interpreter), pin 8e36bc1
$ lupin --version
lupin 0.1.46 (wolf-interp, reference interpreter at pin 8e36bc1)
```

From `crates/wolf_driver/PAIRING` at `8edac3ee` (`lupin-version =
0.1.46`, `lupin-pin = 8e36bc1`), `vendor/upstream/PIN` at `v0.1.46`
(`8e36bc1a…`, v0.2.22's tag) and D57. The compiler's half is **exact**.
The interpreter's half is a distance again: `8e36bc1` is **v0.2.22's
tag**, an ancestor of `8edac3ee`, **200** commits and **one** release
behind (`git rev-list --count 8e36bc1a..8edac3ee`). §1.2's and the
colophon's "222" become "200"; "one release" and "the compiler's
previous release tag" hold. Falsified by any character that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 569 → 586**, key sets diffed both ways: added (17)
  `abi.interrupt`, `abi.layout.align`, `abi.layout.packed`,
  `abi.layout.query`, `abi.link`, `abi.link.extern`, `abi.link.script`,
  `abi.link.section`, `mem.prov.device`, `mem.static`, `mem.static.1`,
  `mem.static.2`, `mem.static.3`, `mem.unsafe.volatile`,
  `mem.unsafe.volatile.1`, `mem.unsafe.volatile.2`,
  `mem.unsafe.volatile.3`; **none dropped**. Both namespaces exist, so
  Appendix D's thirteen hold.
- **`grammar.ebnf` MOVES**: `910ff9d5…` → `67816f8f…`, two hunks, both
  kw09's `extern "c" let`: `bare_item` gains `| extern_let_item`, and
  `extern_let_item ::= 'extern' STRING 'let' IDENT ':' type TERM` is
  new. **Appendix A regenerates** (`cargo xtask backmatter`) with
  exactly those two changes.
- **Catalogue 181 → 185**: E0819, E0820, E0821, E1307 added, nothing
  removed (`## [EW]nnnn` heads of `docs/diagnostics.md`, the derivation
  `backmatter.rs` uses). Appendix C's "The catalog holds 181 codes"
  becomes 185. **No page prints any of the four**, so "these 60" holds
  and the index gains nothing.
- Registry text changed for E0201 (fixtures line only), E0301, E0403,
  E0705, E0708, E0817, E1301, E1401, W0601. Of the twelve codes the book
  runs `wolf --explain` on, only E0201 and E1401 are in that list:
  E0201's change is the fixtures line `--explain` does not print, and
  E1401's (`L1, L2` → `L1-L3`) is in no printed transcript (ch09's
  ledger quotes only its last sentence). **No `--explain` transcript
  drifts.**

## 4. Members and floors (PREDICTED)

- wolf archive: the same ten members by name as 0.2.22 (`wolf`,
  `wolf-cimport-worker`, `libwolf_rt.a`, `wolf.1`, `_wolf`,
  `wolf.bash`, `wolf.fish`, `LICENSE`, `LICENSE-EXCEPTION`,
  `README.md`); still no std tree (s204 is not in 0.2.23's CHANGELOG).
  `_wolf` `2d1e4801…`, `wolf.bash` `29eb2d75…`, `wolf.fish` `51a1b6a8…`,
  `LICENSE` `3972dc97…`, `LICENSE-EXCEPTION` `a0eec20d…`
  **byte-identical** (`help.rs` untouched between the tags); `wolf.1`
  and `README.md` **move** (each carries the version string; the
  README's two `v0.2.22` lines become `v0.2.23`).
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE` (`3972dc97…`),
  `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 3 program failures and 7 console failures.**

| sample | directive | why it fails at 0.2.23 / 0.1.46 |
|---|---|---|
| `ch18/ex18-4` | `fail(E0708)` | the verdict holds (`Vec2` has the native layout) but the snapshot drifts: the headline becomes «the **layout** of `Vec2` is not resolved until codegen lays it out» and the note is rewritten («`size_of`, `align_of` and `offset_of` answer at comptime for scalars and `#[repr(c)]` structs … mark the struct `#[repr(c)]` to make its layout a fact», kw08). Control: the 0.2.22 text, byte-identical to the snapshot. |
| `ch20/ex20-5` | `fail(E0817)` | the verdict holds; the snapshot drifts in its last line: «The attributes this compiler implements: `trusted`, `consttime`, `allow`, `index`, `budget`, `repr(c)`, **`repr(c, packed)`, `repr(c, align(N))`, `section(".name")`** and `cfg(target = "…")`» (`attrs.rs`'s `IMPLEMENTED`, kw08 and kw09). |
| `ch20/ex20-8` | `fail(E0817)` | the same drift. |

Console failures, by name:

1. the four stamp sites (`book/back/colophon.md:7`, `book/ch01.md:153`,
   `book/ch22.md:288`, `principles/exercises/ch22/EXERCISES.md:218`);
   the two `wolf interface` blocks fail on `toolchain 0.2.22` alone.
   **The export hash holds** (`b7a67a58…`): `interface.rs` is untouched
   between the tags.
2. `principles/exercises/ch18/EXERCISES.md`, 18-4's `$ wolf conform-run
   ./ex18-4.lu` (the E0708 text above).
3. `principles/exercises/ch20/EXERCISES.md`, 20-5's `$ lupin ex20-5.lu`
   → `3` / `$ echo $?` → `0`: lupin 0.1.46 reads the closed attribute
   set (is70, wolf-interp#174) and refuses `#[noalloc]` **E0817**, «`#[noalloc]`
   is not an attribute wolf implements: the set is closed (…), and an
   attribute nothing reads is refused, never ignored
   ([gram.item.attr.set])», **exit 2**, nothing printed on stdout.
4. 20-8's `$ lupin ex20-8.lu` → `13`: the same refusal.

Runner arithmetic: **509 samples, 503 passed, 3 pending, 10 failed
(3 programs + 7 consoles), 0 flips; 475 of 502 console blocks.**

What does **not** fail, each checked against the control probes and
`bs60-evidence/scan.py` over all 577 program texts:

- **No flip.** Every `lupin-run(exit=N)` fence the compiler declines
  does so for a construct 0.2.23 does not touch: multiline
  interpolation (ch02), `break` with a value (ch03, ex3-13), an item
  without a declared type (ch06's wordcount: `let USAGE` is UNTYPED,
  so kw09's module `let` does not reach it; still «an item without a
  declared type (E0407 upstream)» at `check.rs:5235`), std methods on
  builtins (ch08 ×2), «indexing / generic application» (ch09:487,
  ex9-7a), «row payload slots» (ex27-3), «`copy` of a value nested this
  deep» (ch28, ex28-*), and static refusals E0301, E0403, E0415, E0604,
  E1010. The two cast fences whose recorded words move (ex9-4, B-11)
  are `lupin-run(exit=trap(ub))`, which carries **no graduation probe**
  by design, so they cannot report a flip (§7).
- **Ruling #34**: no program in the book has an else-less `if` at a
  fallible fn's tail (scan: 0); the one else-less-`if` value in the
  book is a ledger row's program, not a sample (§7, ch03).
- **Ruling #35**: no behaviour moves (spec text only). The book's
  `str`-copy sentences quote E1002's explain text, which 0.2.23 left
  untouched.
- **kw06–kw10** surface: no program uses prefix `*p`, the provenance
  methods, `size_of`/`align_of`/`offset_of` beyond ex18-4, `#[repr]`,
  `#[section]`, `extern "c" let`, volatile, or module `var`. The five
  module-level `const`s are all ch18's bare `compile` / `fail(E0710)`
  fences, whose initializers are comptime calls `[mem.static.3]`
  admits; `compile` passes on `pass` as it did on `unsupported`, and
  the two E0710 snapshots hold.
- **lupin 0.1.46's other changes**: both `errdefer`s (ch06 §6.3, ex6-5)
  sit in fallible fns (E0607 does not reach them); no annotated `!T`
  binding (#180's E0602); no region-block `return` on a lupin-run fence
  (#178); no `cfg`, no `--target`. The lupin transcripts that print a
  parse error (ch01 `almost.lu`, ex1-6's E0202 at end of file; ch12,
  ch14, ex12-9's E0201) **hold**: #175 reorders which diagnostic is
  first only where two tiers both report, and each of these has one.
  This is the prediction's weakest line.
- §9.2's `pub fn header_len` (`fail(E1302)`) holds: lupin 0.1.46
  refuses a `pub` `*T` signature too, and a `fail(…)` fence reads only
  the compiler.

**Pending: 3 → 3**, in the same words at the same spans on both pairs
(ex5-8 @300..303, ex7-5 @365..367, ex8-7 @1926..1929). **lupin-run: 19 of
19 non-trap fences hold**; **wolf-run: 6 of 6 hold** (lupin exit 4 on
the five `comptime fn` programs; ch33's prefork). **Trap rows**: B-5
still `trap(exclusivity)` on lupin. Falsified by any flip, any failure
not in the table, or a table row that passes.

## 6. The witnesses (PREDICTED, `bs60-evidence/extra/`)

| witness | control (measured) | subject (predicted) |
|---|---|---|
| `w_private_ptr_sig` (§9.2's s3 without `pub`) | lupin **E1302, exit 2**; wolf `pass`, exit 0 | lupin **exit 0** (#181: E1302 only on `pub`); wolf `pass` |
| `w_elseless_if_value` (ch03's #153 program) | lupin **E0401, exit 2** («its block is a unit context»); wolf E0401 | both E0401, exit 2 |
| `w_registry_module_const` (§22.3's table as a module `const`) | wolf «item-initializer lowering (globals)», exit 4 | wolf `pass`, `wolf run` prints `Ingest Report Purge`; lupin exit 4 (`comptime fn`) |
| `w_size_of_repr_c` (`#[repr(c)] struct Vec2`) | E0708 on both compiler lanes | `pass`, `wolf run` prints `16`, exit 0; lupin still declines (wolf-interp#188) |
| `w_module_let_typed` (`let USAGE: str = …`) | wolf «item-initializer lowering (globals)»; checked «module items in checked execution» | `pass`, `wolf run` prints the text; lupin prints it too |

## 7. Sentences the release makes false (found by reading, before any archive)

1. **ch09 §9.2**: "The reference interpreter still holds the old line:
   lupin 0.1.45 refuses the private `header_len` with E1302 too
   (wolf-interp#181), so a private `*T` signature is one of the places
   this edition's two machines disagree." lupin 0.1.46 draws the same
   line; the two machines agree again.
2. **ch09's ledger, bs59's #181 row**: closes (wolf-interp#181 closed
   2026-10-04), measured by `w_private_ptr_sig` on both pairs.
3. **ch09's ledger, the cast row**: exercise 9-4 and B-11 «answer raw
   casts that change the machine shape (integer/pointer round trips)».
   kw06 lowers every int/pointer cast; both compile natively now (the
   native lane has no UB detector, so `conform-run` says `pass`, as it
   does for 9-3, 9-5, 9-6, 9-9 and 9-13) and the checked machine names
   `ub(mem.ub)` as before. The row narrows; it stays open on what is
   left (indexing / generic application ×3, E0415 ×1).
4. **ch18 §18.2**: "Ask for a size and the compiler declines, because
   layout (offsets, padding, the target's ABI) belongs to the code
   generator". True of a native-layout struct only; a `#[repr(c)]`
   struct's layout is the clause's, and `size_of` answers it at
   comptime (kw08, `[abi.layout.query]`). One sentence added.
5. **ex18-4's solution** (`ch18/EXERCISES.md`): the transcript, and
   "make aggregate layout a codegen fact" names the `#[repr(c)]` way
   out the new note offers.
6. **ch20's contracts corpus**: the set header ("lupin still reads no
   attribute at all (wolf-interp#174)"), the honesty paragraph ("lupin
   reads no attribute and runs it as before"), 20-5's transcript and
   "lupin executes the program — attributes are inert in the dynamic
   tier", 20-8's transcript and "The two machines claim different
   things … lupin prints 13: it reads no attribute", and the two `.lu`
   headers ("lupin reads no attribute (wolf-interp#174) and still
   prints 3" / "… and prints 13"). Both machines refuse by name now;
   20-8's checker stays `wolf + lupin` (both are run, both refuse), so
   the tiers hold at **191 / 9 / 31 / 28 / 78 / 8**.
7. **ch20's and ch18's ledgers** ("lupin 0.1.45 still reads no
   attribute", "lupin still ignores them"): re-measured notes; both
   rows stay open on #180 (refused is not verified).
8. **EXERCISES-PENDING.md**'s 20-5 row ("lupin 0.1.45 reads no
   attribute and prints `3`"); **25-5 (c)** keeps its dated sentence
   and gains the interpreter's half.
9. **ch22's ledger, the contract-delta row**: "a module-level `const`
   with a comptime initializer is refused with `item-initializer
   lowering (globals)`, so the table has to be a `const` inside
   `main`". kw09 compiles module state (`[mem.static.3]`, evaluated at
   compile time); measured by `w_registry_module_const`. The row stays
   open on its other half (a `comptime fn` still cannot build a
   `List`). §22.3's "Wolf has no code that runs before `main`" holds,
   and gains one sentence saying why module state does not break it.
10. **The stamps** (ch01 ×2 blocks and the 222/distance paragraph, the
    Windows archive name and the `+dev` line; the colophon; ch22's and
    22-13's `toolchain` line; `solutions.md` regenerated).

Found while reading, NOT made false by this release (stale at an
earlier pin; fixed here because each sits in this release's
territory, measured on both pairs):

11. **ch03's ledger, the #153 row**: "lupin still runs the same program
    clean (prints `*`, exit 0). The divergence stands." lupin 0.1.45
    already refuses it E0401 (`probes-control.txt`), the message naming
    `[type.unit.context]`; predicted to hold at 0.1.46 (ruling #34's
    mirror keeps an else-less `if` `()`). The row closes.
12. **ch32's ledger, the papercut row**: "`wolf build allocator.lu`
    produces no binary". The coda's final fence is `run(exit=0)`, both
    machines, green at the control (and since s173). The row closes.

## 8. Back matter and counts at head (PREDICTED)

- Head run: **509 samples, 506 passed, 3 pending, 0 failed, 0 flips;
  482 of 502 console blocks**; snapshots re-blessed for ex18-4, ex20-5
  and ex20-8, each reviewed against the CHANGELOG's wording.
- `ledger --check`: **132 → 129 open**, re-counted (#181 row, ch03's
  #153 row, ch32's papercut close; none added); closed **101 → 104**.
- `verify-docs`: 345 exercises (296 printed), tiers unchanged; Appendix
  C read at **185 / 60**.
- Export: **377**.
- `backmatter`: Appendix A regenerates (the grammar moved);
  `solutions.md` drifts on 22-13's `toolchain` line and on whatever
  solution prose this lane edits; `--check` against a wolf-lang checkout
  at v0.2.23 is clean after the re-vendor.

## 9. Open issues the pin moves

None of wolf-book #60, #58, #57, #48, #45, #43, #29: their programs
answer in the same words (§5). #48 is touched in passing (the 20-8
quote stays byte-identical to its file).

## 10. Method

Control (above) and subject on the same unedited tree, the toolchain
the one variable; the probes repeated on the subject pair; then the
edits; then the head gate (`samples`, `--self-test`, `verify-docs`,
`ledger --check`, `backmatter --check`, `contrast`, `render web`, and
xtask's own tests with `--nocapture`). Every gate log is the full
`2>&1` output with its SKIP lines counted (wolf-lang#571). All on
kasumi from the release archives by digest; CI builds both tools from
source at the pinned revs on three hosts.
