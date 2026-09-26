# bs53 — the prediction, committed before either archive was downloaded

Wave 48. Written 2026-09-26 against wolf-book trunk `b91b75d`, after
the contract (`docs/audit/bs53-contract.md`) and **before**
`wolf-0.2.17-*.tar.gz` or `lupin-0.1.40-*.tar.gz` was downloaded and
before any other file in this tree was edited. DERIVED means computed
from the two upstream git histories; PREDICTED is a claim about what the
release binaries will do, with the number or name that falsifies it.

bs50 said *predict the interpreter*; bs51 inverted it and predicted the
compiler, and was wrong on its biggest row (ch28) because it read a
commit subject for a construct. This bump is the interpreter's again, but
for a reason neither of those lanes had: the book skipped lupin 0.1.39,
and 0.1.39 is where `Scope` and `Proc[T]` reached the interpreter
(wolf-interp#130). The compiler's release is mostly rulings the book
does not exercise.

## 1. The pin (DERIVED)

```
[wolf]   rev 93a5fe504593ca7642b78ba83b4986e7a03cfe71 -> 02afce84f05c7841856a10671b6d7924f79193cc
         impl_version 0.2.16 -> 0.2.17
[lupin]  rev ba357aa6a2e32040d4f089d2cefe05bab86c4f86 -> 54f85e694d4c03e5cd40bef461f85ca0ac373332
```

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.17 (wolfgang, pin 02afce8)
paired with lupin 0.1.40 (reference interpreter), pin 93a5fe5
$ lupin --version
lupin 0.1.40 (wolf-interp, reference interpreter at pin 93a5fe5)
```

From `crates/wolf_driver/PAIRING` at `02afce84`, `vendor/upstream/PIN`
at `v0.1.40`, and D57 (a build at its own tag prints the bare version
and a seven-character pin). The compiler's half is **exact** again (it
names 0.1.40; the book pins 0.1.40). The interpreter's half is a
**distance** again: `93a5fe5` is v0.2.16's tag, an ancestor of
`02afce84`, **72** commits and exactly one release behind. §1.2's and
the colophon's "148 commits" become "72". Falsified by any character
that differs, or a count other than 72.

## 3. Spec artifacts (DERIVED both ways)

- Anchors **539 → 541**, `mem.model.place.rhs` and `os.fs.path.domain`
  added, none dropped, none remapped. Both land in namespaces that
  already exist, so namespaces hold at 13 and documents at 11; Appendix
  D moves in its counts only.
- `grammar.ebnf` **moves** (`2b6269d7…` → `910ff9d5…`), two lines: the
  moded store alternative and the `index_place` rule. Appendix A
  regenerates with exactly those two lines.
- Catalogue **176 → 176**, same code set. Appendix C does not move and
  no `wolf --explain` transcript moves (the only diff in
  `docs/diagnostics.md` is `Fixtures:` lines).

## 4. Flips at the subject run — **1**, the interpreter's (PREDICTED)

The subject run is the unedited tree with the new pair. The runner
reports a flip only where a probe can see one (TWO-MACHINES §6): a
`lupin-run` whose compiler verdict became `pass`, or a `wolf-run` the
interpreter now meets.

1. `book/ch14.md:146`, `part(watch)`,
   `wolf-run(exit=0, stdout="3")`: `fn watch(p: Proc[int])` and
   `p.join()`. lupin 0.1.38 answered `E0301: nothing named `Proc``;
   0.1.39 made `Proc[T]` a prelude type and `p.join()` the typed join
   (`78395fb`, `7dc4982`). **FLIPS** to `run(exit=0, stdout="3")`.

Predicted **not** to flip, each with its reason:

- The other six `wolf-run` fences: ex18-3, 18-5, 18-11, 18-15 and
  ex22-7 are `comptime fn`, which lupin still declines; ch33's prefork
  is `os_spawn_with`'s inherit set, declined by name. Nothing in
  0.1.39/0.1.40 touches either.
- Every `lupin-run(…)` non-trap fence (ch02, ch03 ×2, ch06, ch08 ×3,
  ch09, ch11 ×2, ch14 ×2, ch15, ch16, ch28, ex3-13, ex9-7a, ex9-15,
  ex11-1, ex27-3, ex28-1/2/3/6): the compiler's changes across the span
  are #449's narrower claim, #431's native env copy (invisible to a
  verdict that never executes), #438's store lowering and #437's record
  keys. None names a construct any of these rows is refused for —
  «`str` methods outside the builtin set», «indexing / generic
  application», «`copy` of a value nested this deep», D28's E0501 on an
  unbounded `[S]`/`[P]`. **ch28's six are the named risk**: s180's
  commit says "a plain index store deep-copies a place", and bs51 was
  burned by reading exactly this kind of subject line; the wordtree
  has no index store (its deep copy is `walk(n.left[0])`, a read), so
  it is predicted to hold.
- Trap rows carry no automatic flip.

**Falsified by any flip count other than 1, or a different name.**

## 5. Failures at the subject run — **0** (PREDICTED)

- #438: no sample stores a *place* into an index (contract §2, drift 1,
  the search shown to fire), so neither machine's move-to-copy change
  reaches a verdict or a byte.
- #449's fix only removes E1002s; bs51 measured the book's #449 sites at
  zero, so no `fail(E1002)` loses its refusal.
- #437 adds record keys the runner never reads and the console lane
  drops with the JSON line.
- #386 / `lupin conform-run`'s cwd: the book's 12 `lupin conform-run`
  blocks are all `--explore=N`, and the explorer keeps its private root
  (lupin CHANGELOG 0.1.40).
- lupin 0.1.39 retired `EXIT_REJECTED = 65` for `lupin lex`/`parse`;
  the book replays neither.

**Falsified by any sample failing at the subject run.** Totals at the
subject run: **506 samples, 502 passed + 1 FLIP (a hard error until
graduated), 3 pending**, then 503 passed after the graduation.

## 6. Graduations by respelling — **5**, measured not inferred (PREDICTED)

The probe cannot see these: the fences are `lupin-run` because the
compiler refuses the *generic* spelling (E0501 by D28), and it still
will. What changed is that the named spelling now works on both
machines — `Scope`/`Proc[T]` on lupin since 0.1.39, and the native
scope-handle hang (wolf-lang#431) fixed in wolf 0.2.17.

| # | site | respelled | predicted verdict, both machines |
|---|---|---|---|
| G1 | `book/ch11.md:20` §11.1 capability | `fn tally_into(s: Scope, …)` | `run(exit=0, stdout="2740 words, three readers, one scope")` |
| G2 | `book/ch11.md:124` §11.2 refresher | `fn refresh_into(s: Scope, …)` | `run(exit=0)` |
| G3 | `ch11/ex11-1.lu` | `fn launch(s: Scope, …)` | `run(exit=0)` (prints `60`) |
| G4 | `book/ch14.md:79` §14.1 three ways to stop | `fn reason_of(p: Proc[int], …)` | `run(exit=0)`, transcript `returned: normal(3)` / `failed: error(Corrupt)` / `killed: killed` on both |
| G5 | `book/ch14.md:357` §14.3 kill vs cancel | `fn reason_of(p: Proc[int], …)` | `run(exit=0)`, same transcript on both |

Plus one respelling that does **not** graduate, by design:
`ch11/ex11-2.lu` takes `Scope` and stays
`lupin-run(exit=trap(deadlock))`; its row stops being a debt (E0501)
and is the permitted difference `[conc.deadlock.trap]` names.

G4/G5 are the riskiest: bs51 only ever reached E0501 on them, so a
second refusal behind it (`p.monitor()`'s exit arm, `bad() -> !int` as
`Proc[int]`) has never been asked. **Falsified per row**: any of G1–G5
refused or disagreeing on either machine.

After §4 and §6: one-machine non-trap fences **31 → 25**
(six leave: the watch flip and G1–G5).

## 7. Pending: **3 → 3** (PREDICTED)

- `ch05/ex5-8`: unchanged (no std root; `take` unspellable, #58).
- `ch07/ex7-5`: unchanged (E1003 reaches no verdict).
- `ch08/ex8-7`: **both halves hold, in the same words**. lupin 0.1.40
  still reads `pool[h].next = k` as a map index (`[type.map.key]`,
  exit 4) — 0.1.39's census put `memory/pool_place_write.lu` out of
  scope for exactly that — and wolf 0.2.17 still has no `Eq` for a
  handle (nothing in the span's log touches equality).

## 8. Exercise 7-23 (PREDICTED)

- `ex7-23.lu`: wolf 0.2.17 `fail(E1002)` at `mem`, the rendered
  diagnostic **byte-identical** to the page (the E1002 catalogue entry
  moved only in its fixture list). lupin 0.1.40 runs it to exit 0,
  `moves, and the shelf still holds 3` — the lend rule is still static
  only, and neither lupin release touched it.
- `ex7-23b.lu`: `run(exit=0)` on both, same line.

## 9. Console and literal re-records, by name (PREDICTED)

Moves: §1.2's and the colophon's `--version` blocks (2); §1.2's two
Windows archive names and `0.2.16+dev.unknown`; ch22's
`toolchain 0.2.16` → `0.2.17` at 3 sites (ch22, its exercises page,
the solutions page). Holds: ch22's two export hashes and ch25's
`tree=`/`manifest=`/`interface=` (`crates/wolf_pkg/` unchanged); Appendix
E's `wolf --help` (`help.rs` unchanged); every `wolf conform-run` and
`lupin conform-run --explore` transcript. Exercise index tiers
193 / 31 / 25 → **192 / 31 / 26** (ex11-1 becomes wolf + lupin).

## 10. Method

Control and subject, as bs50 and bs51 ran it: the unedited tree at
`b91b75d` with the OLD pair first, then the same unedited tree with the
NEW pair, so the toolchain is the one variable. Both on kasumi (linux
x86-64), from release archives verified by digest, never a source build.
The respellings are measured one program at a time on both machines
before any fence changes.
