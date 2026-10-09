# bs64 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-09 against wolf-book `851b996` (the contract,
an empty commit on bs63's merged trunk `fde77b8`), **before**
`wolf-0.2.26-*.tar.gz` or `lupin-0.1.49-*.tar.gz` was downloaded. DERIVED
means computed from the two upstream git histories at the tags; PREDICTED
is a claim about what the release binaries will do, with what falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries, downloaded on kasumi by digest against the release
API (`bs64-evidence/control-gates.txt`, 18:51Z): wolf 0.2.25 `9d91f533…`,
lupin 0.1.48 `81cfd77a…`. On the unedited head `851b996`: **512 samples,
509 passed, 3 pending, 0 failed, 0 flips; 485 of 505 console blocks**;
ledger **127 open (120 filed, 7 waived), 108 closed**; verify-docs 345
exercises, tiers 6 / 78 / 9 / 38 / 173 / 41; diagrams 6 from 4 traces,
checked with the release lupin as `LUPIN_TRACE`. And the **577-program
sweep** (bs63's `allprogs.py`: every book fence outside `back/`, `part()`
fences assembled, every corpus `.lu`, each alone in its own directory) on
lupin, `wolf conform-run --json` (native), `--checked`, and a `--release`
build and run for every native `pass` with a `main`, twice for the noise
floor (`sweep-c25a.jsonl`, `sweep-c25b.jsonl`, `sweep-c25-noise.txt`).

## 1. The pin (DERIVED)

```
[wolf]         rev 6710f9e0cbc3a7264349093751ce7a46a407e473 -> 89dc139443da38078df6093568da87bc6d0ee6f9
               impl_version 0.2.25 -> 0.2.26
[lupin]        rev 531bf0581dea6bba4b1247edb2abada5214c18ab -> f516a5f4ea4341acd3a30f3e5cdd4327aede1912
[lupin-trace]  rev 531bf0581dea6bba4b1247edb2abada5214c18ab -> f516a5f4ea4341acd3a30f3e5cdd4327aede1912
```

One compiler release (0.2.26, release 408143286, 163 commits after
v0.2.25) and one interpreter release (0.1.49, release 408028965, 55
commits after v0.1.48). The trace pin stays collapsed onto `[lupin].rev`:
`src/eval/placetrace.rs` is byte-identical between v0.1.48 and v0.1.49
(`git diff --stat` empty).

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.26 (wolfgang, pin 89dc139)
paired with lupin 0.1.49 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.49 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `89dc1394` (`lupin-version = 0.1.49`, `lupin-pin =
294d626`, unchanged), `vendor/upstream/PIN` at `v0.1.49` (`294d626d…`)
and D57. **The interpreter's clause is now TWO releases behind**:
`294d626` is v0.2.24's tag, an ancestor of `89dc1394`, **213** commits
(`git rev-list --count 294d626..89dc1394`) and **two** releases (v0.2.25,
v0.2.26). So these sentences become false and are rewritten: ch01 §1.2
"`294d626`, which is the compiler's *previous* release tag … one release
behind … you get 50" and the colophon's "the compiler's previous release
tag … 50 commits, one release". Falsified by any character of the stamps
that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 595 → 607**, key sets diffed both ways: added (12)
  `mem.list.bytes`, `mem.region.copyout`, `mem.static.4`,
  `mem.unsafe.raw.5`, `os.fs.chdir`, `os.fs.copy`, `os.fs.error`,
  `os.fs.isatty`, `os.proc.fds`, `os.proc.pipe`, `type.fn.never`,
  `type.int.not`; **none dropped**. Appendix D's tags hold.
- **`grammar.ebnf` does not move** (`67816f8f…` at v0.2.25 and v0.2.26,
  byte-equal to the vendored copy; `copy region` adds no token or rule).
  **Appendix A does not regenerate.**
- **Catalogue 187 → 187**: `docs/diagnostics.md` gains no `## E…`/`## W…`
  heading between the tags. Appendix C's count holds.
- Registry text: only **E1010**'s entry changes (a `copy region`
  sentence); no `--explain` transcript in the book prints E1010, so no
  `--explain` block moves.

## 4. Members and floors (PREDICTED)

- wolf archive: **eleven** members, 0.2.25's set. **Byte-identical to
  0.2.25's**: `_wolf` `a368c8ec…`, `wolf.bash` `1b1d8853…`, `wolf.fish`
  `180de0ec…` (`main.rs` moved by a comment only, no subcommand or help
  text), `LICENSE` `3972dc97…`, `LICENSE-EXCEPTION` `a0eec20d…`. **Move**:
  `wolf`, `wolf.1`, `README.md` (version strings), `wolf-cimport-worker`,
  `libwolf_rt.a`, `libwolf_rt_none.a` (the runtime moved: s200's byte
  scan and copy, s215's pipe/chdir/isatty/spawn_fds).
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE` (`3972dc97…`),
  `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**
  (`copy_file_range` is 2.27).

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 506 passed, 3 pending, 12 failed; 477 of 505 console blocks.**

The **three samples** (verdict `fail(E1010)` holds; the snapshot drifts):
E1010's note gains one sentence at 0.2.26 (wolf_mem: «When the block's
own value is what must outlive it, `copy region { … }` copies that value
into the enclosing region before the free ([mem.region.copyout]).»),
re-wrapped:

1. `book/ch08/s3` (§8.2, `newest`), 2. `book/ch08/s4` (exercise 8-3's
fence in §8.2), 3. `ch08/ex8-3` (the corpus file).

The **one** `diagnostic,from` block: 4. `book/ch08.md`'s
`diagnostic,from(book/ch08/s3)`.

The **eight** console blocks:

5. `principles/exercises/ch08/EXERCISES.md`, 8-3's `$ wolf conform-run
   ./ex8-3.lu` (the same E1010 note).
6. `book/ch01.md` `--version` (§2's stamps).
7. `book/back/colophon.md` `--version`.
8. `book/ch22.md` `wolf interface`: `toolchain 0.2.25` → `0.2.26`, **both
   export hashes hold** (`b7a67a58…`; wolf-lang's own interface snapshot
   moves only its toolchain line between the tags).
9. `principles/exercises/ch22/EXERCISES.md`, 22-13's interface: the same.
10. `book/ch24.md` `pkg/acquired`, `wolf audit --dir app`: one line after
    `effective: [net]`, `  net: regex — declared (nothing in its code
    reaches it)` (s217's `render_audit`).
11. `book/ch24.md` `pkg/acquired`, `wolf audit --ci --dir app`: the same
    line, the ACQUIRES and refusal lines and exit 1 unchanged.
12. `book/back/appendix-e.md` `pkg/promote`, `wolf audit --dir tally`: one
    line after `effective: [fs]`, `  fs: local/tally (root) — declared;
    calls `fs_write_text` at tally/main.lu:L:5 (+1 more)` (the second
    site is `fs_read_text`; the line number L is the one prediction here
    not made to the character, 3 or 4, depending on whether `wolf init`
    keeps the leading `//!` doc line in `main.lu`).

Not replayed, regenerated from a run anyway (s217's fifth transcript):
`principles/exercises/ch24/EXERCISES.md:136` is a declined block
(`samples-declined.toml`); its `wolf audit --ci` gains the same `net:
regex` line, and `book/back/solutions.md` follows by `backmatter`.

What does **not** fail:

- **No flip, no verdict moves** (native, checked, lupin). By category:
  - **#618** (a call's `str` result held past a region, now E1010): no
    book program has a region block and a fn returning `str` (scan of
    the 577: 0 programs), so nothing is newly refused.
  - **s217** (capabilities from what code reaches): only `samples/pkg`
    has manifests; `shelf` and `acquired`'s code calls no host builtin,
    `promote` declares the `fs` it reaches. No E1504 appears.
  - **The ten prelude names**: no book program declares any of them
    (grep), so no W0304.
  - **s200** (descriptors 0–2): no book program calls `fs_read`/`fs_write`
    family on 0, 1 or 2 or opens `/dev/std*`.
  - **#575/checked bitwise**: no book program uses `&`, `|`, `^` or `!`
    on an integer (every `|` in the 577 is a pattern alternative), so the
    checked machine's widening moves nothing.
  - **#572** (`never` renders the bottom type): no book diagnostic prints
    a bottom type.
  - **#577/#579**: no book program stores a raw element's field or reads
    another module's `pub` item by its module's name (the 577 are single
    files).
  - lupin 0.1.49's halves (byte surface, papercuts, s215, s216, is74's
    volatile and atomics): no book program reaches them.
- **The sweep** (subject against c25): native and checked move **nothing**
  but the three E1010 programs' rendered note, which `cmp.py` does not
  compare (it compares verdict, stdout and codes), so **0 rows**; lupin
  **0 rows**; release only inside c25's own noise set. Falsified by any
  native, checked, lupin or release row that moves outside the noise set.
- Pending **3 → 3** (ex5-8, ex7-5, ex8-7) in the same words.

## 6. Chapter 7 and the diagrams (PREDICTED)

`cargo xtask diagrams --check` with `LUPIN_TRACE` = the release lupin
0.1.49 re-takes the four traces **byte-identical** (placetrace.rs did not
move; the four programs use no region, raw pointer or new builtin); the
six SVGs and texts hold. No chapter-7 sample, console or diagnostic block
moves. Chapter 7's `copy` section ("Copying is a decision") makes no claim
about regions, so `copy region` puts no sentence there out of date.

## 7. Sentences the release makes false (found by reading, before any archive)

1. **ch01 §1.2** and **the colophon**: "the compiler's *previous* release
   tag", "one release behind", "50" → v0.2.24's tag, two releases, 213.
2. **ch05 §5.6** (traits and bounds): "`!`, `&&` and `||` are `bool`'s alone"
   — `!` on an integer is its bitwise complement (ruling #51). The E0501
   note it quotes is unchanged at 0.2.26 (`wolf_sema/src/check.rs`) and
   still accurate for a type parameter (no trait covers `!`); the page's
   own sentence is rewritten.
3. **ch08 §8.2**: "Three repairs … `copy` inside the block does not help"
   — still true of a plain `copy`, but there is now a fourth repair for a
   block's own value, `copy region` (ruling #56), and the E1010 note the
   page prints says so.
4. **ch08's ledger**, the open `ba:diagnostic` row "`wolf --explain E1010`
   closes with … coming in later tiers": false already (the explain is
   present tense, "both are spellings this compiler takes today") and at
   0.2.26 it also names `copy region`; the row closes, measured.
5. **ch22 §22.3**: "`toolchain 0.2.25` is which compiler printed the
   page" → 0.2.26.
6. **ch24 §24.3**: the paragraph after the `pkg/acquired` audit reads the
   output line by line; it gains the reason line (`effective` is now
   what is declared united with what the code reaches, s217, rulings #53
   and #54). Appendix E's `pkg/promote` paragraph likewise.
7. The version-literal ledger (`audit/version-literals.txt`): every
   `audited-at` stamp re-read at 0.2.26 / 0.1.49 (bs63's miss: the ledger
   forces the re-read).

## 8. The new surfaces, placed (PREDICTED)

Each is a `run(…)` fence (lupin and `wolf run`), measured beside it on
all four machines (lupin, native, checked, release) and on CI's three
hosts:

- **`copy region` (ruling #56)** — ch08 §8.2, beside the repair sentence:
  a loop whose turn is computed in `copy region turn { … }` and keeps
  only the string it returns. Prose: the block's value is deep-copied into
  the region it was entered from, then the block's region is freed; any
  other way out is still E1010.
- **`-> never` (ruling #50)** — ch06 §6.2, beside `else |err| { … }`'s
  "the block's value becomes the expression's": a handler that ends the
  program is a call to a `-> never` fn, which fits where an `int` is
  wanted; a `-> never` body that can reach its end is E0401.
- **`!` on integers (ruling #51)** — ch02 §2.3 (bytes, where the page
  already names masks), with `n & !0xff` and `(!b) as byte`; and ch05's
  sentence (§7.2 above).

Head counts: **515 samples, 512 passed, 3 pending, 0 failed, 0 flips;
488 of 508 console blocks** (one console block per new fence).

## 9. Back matter and counts at head (PREDICTED)

- Snapshots re-blessed for ch08 s3, s4 and ex8-3 only.
- `ledger --check`: **127 → 126 open, 108 → 109 closed** (§7.4's row).
- `verify-docs`: 345 exercises, tiers unchanged; Appendix C 187 / 60.
- `backmatter --check` against a wolf-lang checkout at v0.2.26 clean after
  the re-vendor (anchors.json moves; diagnostic-codes.txt and
  grammar.ebnf do not); `solutions.md` regenerated (22-13's toolchain
  line, 8-3's note, 24-6's reason line).
- Export count unchanged; `diagrams --check` 6 from 4, byte-identical.

## 10. Method

Control (above) and subject on the same unedited tree, the toolchain the
one variable; the sweep on the subject pair twice; then the edits
(snapshots by `--bless`, console and `diagnostic,from` blocks spliced from
the run's own `actual:` by bs63's `splice.py`, which asserts the old block
equals the log's `expected:`, never typed); the declined 24-6 block and
the new fences' console blocks from runs on kasumi, logged; then the head
gate (`samples`, also under `taskset -c 0-3`, `--self-test`,
`verify-docs`, `ledger --check`, `backmatter --check`, `contrast`,
`diagrams --check` with `LUPIN_TRACE`, `render web`, `tiers --check`,
xtask's tests with `--nocapture`), every log full `2>&1` with SKIPs
counted. All on kasumi from the release archives by digest; CI builds both
tools from source at the pinned revs on three hosts.
