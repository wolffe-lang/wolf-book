# bs63 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-07 against wolf-book `beedaa8` (the contract,
an empty commit on bs62's merged trunk `d0213d07`), **before**
`wolf-0.2.25-*.tar.gz` or `lupin-0.1.48-*.tar.gz` was downloaded. DERIVED
means computed from the two upstream git histories at the tags; PREDICTED
is a claim about what the release binaries will do, with what falsifies it.

What ran before this file, and is allowed to: two **control** pairs, both
downloaded on kasumi by digest against the release API in the same minute
(`bs63-evidence/control-gates.txt`, 22:55Z):

- **c23**, the pin the book carries: wolf 0.2.23 `6f505eb5…`, lupin 0.1.46
  `d13a0379…`. On the unedited head `beedaa8`: **512 samples, 509 passed,
  3 pending, 0 failed, 0 flips; 485 of 505 console blocks**; ledger **128
  open (121 filed, 7 waived), 107 closed**; verify-docs 345 exercises,
  tiers 6 / 78 / 9 / 38 / 173 / 41; diagrams 6 from 4 traces, checked
  (trace half SKIPped: 0.1.46 has no trace).
- **c24**, the release the contract names as the "before" column: wolf
  0.2.24 `501d6d3f…`, lupin 0.1.47 `0ddc4ff3…`. Same tree: **507 passed,
  3 pending, 9 failed, 0 flips; 480 of 505 consoles** (the nine named in §5).
- A sweep of **all 577 programs** the runner executes (every book fence
  outside `back/`, `part()` fences assembled, every corpus `.lu`), each alone
  in its own directory, on lupin, `wolf conform-run --json` (native),
  `--checked`, and for every native `pass` with a `main` a `wolf build
  --release` and run (368 programs): `bs63-evidence/sweep-c23.jsonl`,
  `sweep-c24.jsonl`, compared by `cmp.py`. c23 run twice for the noise floor
  (`sweep-c23-noise.txt`): 5 release rows move between two identical runs
  (ex9-4 and B-11 exit codes after UB; ch17 §17.1's schedule program; ex30-4
  and ex30-5's arrival order). c23 → c24 (`sweep-c23-vs-c24.txt`): **ch18's
  two module-`const` witnesses go from `["E0710","E0710"]` to `["E0710"]` on
  native and checked (#584)**, nothing else outside the release noise set
  (ex11-5, ex17-1 and §17.1 move with the schedule, the same family as the
  noise rows).

## 1. The pin (DERIVED)

```
[wolf]         rev 8edac3eeb48632b32f02ef41ea87d484d1423492 -> 6710f9e0cbc3a7264349093751ce7a46a407e473
               impl_version 0.2.23 -> 0.2.25
[lupin]        rev f9269e3364169585fdfe92407639418220be373b -> 531bf0581dea6bba4b1247edb2abada5214c18ab
[lupin-trace]  rev b7b7ddb27f9f442086fdf1fdfc4827587c2434b9 -> 531bf0581dea6bba4b1247edb2abada5214c18ab
```

Two compiler releases (0.2.24 release 404332628, 0.2.25 release 406122367)
and two interpreter releases (0.1.47 404283632, 0.1.48 405340127). The
trace pin collapses onto `[lupin].rev`: `b7b7ddb` is an ancestor of
`v0.1.48` (`git merge-base --is-ancestor`), and `src/eval/placetrace.rs` is
byte-identical between them. CI's "build the trace lupin" step then prints
`trace lupin = the pinned lupin` and builds nothing extra.

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.25 (wolfgang, pin 6710f9e)
paired with lupin 0.1.48 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.48 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `6710f9e0` (`lupin-version = 0.1.48`, `lupin-pin =
294d626`), `vendor/upstream/PIN` at `v0.1.48` (`294d626d…`, v0.2.24's tag)
and D57. The interpreter's clause is a distance again: `294d626` is
**v0.2.24's tag**, an ancestor of `6710f9e0`, **50** commits and **one**
release behind (`git rev-list --count 294d626..6710f9e0`; the CHANGELOG's
"49 since v0.2.24" counts lanes' commits, the merge is the 50th). §1.2's
and the colophon's "200" become "50"; "one release" and "the compiler's
previous release tag" hold. Falsified by any character that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 586 → 595**, key sets diffed both ways: added (9)
  `conc.mm.atomic.order`, `conc.mm.atomic.raw`, `conc.mm.atomic.raw.1`–`.5`,
  `conc.mm.fence`, `mem.unsafe.raw.4`; **none dropped**. Appendix D's tags hold.
- **`grammar.ebnf` does not move**: `67816f8f…` at v0.2.23 and v0.2.25
  (the vendored copy is byte-equal to v0.2.25's). **Appendix A does not
  regenerate.**
- **Catalogue 185 → 187**: E1308, E1309 (kw11's atomics), nothing removed.
  Appendix C's "The catalog holds 185 codes" becomes 187. No page prints
  either, so "these 60" holds.
- Registry text: between the tags only E1301's entry changes besides the
  two new codes, and no `--explain` transcript in the book prints E1301
  (c24 replays every `--explain` block green). v0.2.24 → v0.2.25 changes no
  file under `docs/`; `spec/02-memory-model.md`'s `[mem.static.2]` text
  moves (s214) and the book quotes only `[mem.static.3]`.

## 4. Members and floors (PREDICTED)

- wolf archive: **eleven** members, 0.2.24's set: `wolf`,
  `wolf-cimport-worker`, `libwolf_rt.a`, `libwolf_rt_none.a` (new since
  0.2.24, kw12), `wolf.1`, `_wolf`, `wolf.bash`, `wolf.fish`, `LICENSE`,
  `LICENSE-EXCEPTION`, `README.md`. **Byte-identical to 0.2.24's**: `_wolf`
  `a368c8ec…`, `wolf.bash` `1b1d8853…`, `wolf.fish` `180de0ec…` (no driver
  source moved between v0.2.24 and v0.2.25), `LICENSE` `3972dc97…`,
  `LICENSE-EXCEPTION` `a0eec20d…`. **Move**: `wolf`, `wolf.1`, `README.md`
  (version strings), `wolf-cimport-worker`, `libwolf_rt.a` and
  `libwolf_rt_none.a` (no runtime source moved, but the workspace version
  0.2.24 → 0.2.25 changes every crate's metadata hash, so the archives'
  symbol names move; the weakest line here).
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE` (`3972dc97…`), `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34** (both
  controls measured 2.34).

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 507 passed, 3 pending, 11 failed; 478 of 505 console blocks.**

The nine c24 already shows, which 0.2.25 keeps (0.2.24 → 0.2.25 moves no
driver, help or `--explain` source and no comptime code):

1. `book/ch18/s2` and `book/ch18/s10`: verdict `fail(E0710)` holds; the
   snapshot drifts from **two records to one** (#584, fixed in 0.2.24 by
   s210: the module-state loop skips an initializer whose call site already
   faulted). The surviving record is the call site's ("while evaluating
   `SIXTEEN_SHARDS`", the witness note); the second record and its
   `[mem.static.3]` note go. Measured on c24.
2. The two `diagnostic,from(book/ch18/s2)` / `(…/s10)` blocks in ch18.md,
   for the same reason.
3. `book/back/appendix-e.md:13`, `wolf --help`: one new line under
   "Inspecting and reporting", `prelude        every name a program uses
   without an import`, between `audit-surface` and `profile` (s212's
   `wolf prelude`). The page's "twenty-two subcommands" and "Three of the
   twenty-two" become twenty-three.
4. The four stamp sites: `book/back/colophon.md:7`, `book/ch01.md:153`,
   `book/ch22.md:288` and `principles/exercises/ch22/EXERCISES.md:218`
   (`toolchain 0.2.23` → `0.2.25`; **both export hashes hold**,
   `b7a67a58…`: wolf-lang's own interface snapshot moves only its toolchain
   line between the tags).

And two that only 0.1.48 makes, both console blocks in
`principles/exercises/ch20/EXERCISES.md`:

5. 20-5's and 20-8's `$ lupin exN.lu` lines: E0817's closed set on lupin
   grows (is73, wolf-interp#188/#190) from «(`trusted`, `consttime`,
   `allow`, `index`, `budget`, `repr(c)`, `cfg(target = "…")`)» to
   «(`trusted`, `consttime`, `allow`, `index`, `budget`, `repr(c)` with
   `packed` or `align(N)`, `section`, `cfg(target = "…")`)», same code,
   span and exit 2.

What does **not** fail:

- **No flip, no verdict moves.** c24 already ran the tree with 0 flips;
  0.2.25 is s214 alone (a reload after a call that may write foreign
  memory; rangeopt's unsigned bound), and no book program has a module
  `var`, an `extern "c" let`, an atomic, a declared `fence`, or a `>>` in
  wolf code (grep over all 577 programs). Raw-pointer programs (ch09, B-11,
  ch32's allocator) keep their answers.
- **The sweep** (subject against c24 and against c23): native and checked
  move **nothing** against c24 (and only #584's code count against c23);
  release moves only inside the noise set above; lupin moves on **ex18-4
  alone** in exit/stdout — `size_of(Vec2)` was `unsupported` (exit 4,
  «`size_of` does not resolve») and becomes **E0708, exit 2** on 0.1.48
  (is73's `layout.rs`), the compiler's code. Its fence is `fail(E0708)`,
  which reads only the compiler, so no sample moves; the exercise's
  interpreter half is new information, not a failure. lupin's stderr moves
  on 20-5 and 20-8 (item 5). **No program that printed a wrong answer**:
  #598/#601 need a module `var`, `extern let` storage or a raw pointer
  written by a callee and re-read after the call, and #600 needs a `>>`;
  the book has none of the first two and no wolf `>>`. Falsified by any
  native, checked or release stdout that moves outside the noise set.
- **lupin 0.1.48's module-state refusals** do not reach ch06's capstone:
  `let USAGE = """…"""` has no written type, and `statics::not_static_data`
  judges only a written one; its initializer reads no module `var`, so
  `init_check` (E0705) passes it. ch18's module `const`s are `bool`
  (static data) and their fences read only the compiler.
- Pending **3 → 3** (ex5-8, ex7-5, ex8-7) in the same words.

## 6. Chapter 7 and the diagrams (PREDICTED)

- Chapter 7 is on trunk (bs62, `d0213d0`) and passes on both controls.
  At 0.2.25 / 0.1.48 every chapter-7 sample, console and `diagnostic,from`
  block holds; no chapter-7 file is edited by this lane except if a
  re-take says otherwise.
- `cargo xtask diagrams --check` with `LUPIN_TRACE` = the **release**
  lupin 0.1.48 re-takes the four traces **byte-identical** to the
  checked-in `snapshots/traces/ch07/*` (placetrace.rs unchanged since
  `b7b7ddb`; the four programs use no module state or layout query), so
  the six SVGs and their texts hold byte for byte. Falsified by any trace
  byte that moves.

## 7. Sentences the release makes false (found by reading, before any archive)

1. **ch18 §18.1** — "The second report is the same failure again … at this
   printing it evaluates that initializer twice and reports each fault …
   wolf-lang#584 is the compiler folding the two records into one." False
   since 0.2.24; the paragraph goes, as its own ledger row said it would.
2. **ch18's ledger, the bs60 row "ONE FAILED ASSERT, TWO RECORDS"**: closes
   (wolf-lang#584 closed), measured on both 0.2.24 and 0.2.25.
3. **ch22's ledger, the contract-delta row**: "Two cautions found on the
   way, both filed" — both fixed in 0.2.24 (#584, #585); the row gains the
   re-measure and stays open on its other gap (a `comptime fn` cannot build
   a `List`).
4. **ch20's corpus page, 20-8**: "the compiler's list … has grown
   `repr(c, packed)`, `repr(c, align(N))` and `section(".name")`, which the
   interpreter does not implement yet (wolf-interp#188, #190)". At 0.1.48
   both lists name the same set, spelled differently; the sentence is
   rewritten to say so. 20-5's prose is checked the same way.
5. **Appendix E**: "twenty-two subcommands" (twice) and the reference
   sentence naming the four commands no chapter teaches gains `prelude`.
6. **Appendix C**: 185 → 187 (regenerated by `backmatter`).
7. **The stamps**: ch01 (two blocks, the 200 → 50 distance, the Windows
   archive name, the `+dev` line), the colophon, ch22 and 22-13's
   `toolchain` line and §22.3's prose about it; `solutions.md` regenerated.
8. `principles/EXERCISES-PENDING.md`'s 20-5 row and the ch18 / ch20 ledgers'
   dated lupin sentences are re-read; each stays true of the version it
   names or is re-measured.

## 8. Back matter and counts at head (PREDICTED)

- Head run: **512 samples, 509 passed, 3 pending, 0 failed, 0 flips; 485 of
  505 console blocks**; snapshots re-blessed for ch18 s2 and s10 only.
- `ledger --check`: **128 → 127 open**, closed **107 → 108** (ch18's #584
  row), re-counted.
- `verify-docs`: 345 exercises (296 printed), tiers unchanged; Appendix C
  read at **187 / 60**.
- Export: unchanged count (c23's).
- `backmatter --check` against a wolf-lang checkout at v0.2.25 clean after
  the re-vendor (anchors.json and diagnostic-codes.txt move,
  grammar.ebnf does not).
- `diagrams --check` with `LUPIN_TRACE` = the release lupin: 6 diagrams from
  4 traces, byte-identical.

## 9. Open issues the pin moves

wolf-lang#584 and #585 are closed upstream; the book's rows naming them are
§7's 2 and 3. None of wolf-book #60, #58, #57, #45, #29 moves (their
programs answer in the same words on c24).

## 10. Method

Control (above) and subject on the same unedited tree, the toolchain the
one variable; the sweep repeated on the subject pair twice (noise); then
the edits (snapshots by `--bless`, console and `diagnostic,from` blocks
spliced from the run's own `actual:` output by `bs63-evidence/splice.py`,
never typed); then the head gate (`samples`, `--self-test`, `verify-docs`,
`ledger --check`, `backmatter --check`, `contrast`, `diagrams --check` with
`LUPIN_TRACE`, `render web`, `tiers --check`, xtask's tests with
`--nocapture`). Every gate log is full `2>&1` with its SKIP lines counted
(wolf-lang#571). All on kasumi from the release archives by digest; CI
builds both tools from source at the pinned revs on three hosts.
