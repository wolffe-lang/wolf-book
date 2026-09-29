# bs54 — the prediction, committed before either archive was downloaded

Wave 50. Written 2026-09-28 against wolf-book trunk `f2f4280`, after
the contract (`docs/audit/bs54-contract.md`, `974bf4f`) and **before**
`wolf-0.2.18-*.tar.gz` or `lupin-0.1.41-*.tar.gz` was downloaded and
before any other file in this tree was edited. DERIVED means computed
from the two upstream git histories; PREDICTED is a claim about what the
release binaries will do, with the number or name that falsifies it.

This is a point release on both sides. Every change in it is about
**moved places**: an element as a place (EG1), a store that does not
revive a moved sibling (#460), a `mut` parameter that must hold a value
at return (#464), index-then-value order on the checked machine (#452),
and on lupin, a read of a moved element traps (wolf-interp#141). The
book's programs move almost nothing out of a container, so the
prediction is that the release reaches the book through its **words**
— a catalogue paragraph, the stamps, two prose claims — and through no
verdict.

## 1. The pin (DERIVED)

```
[wolf]   rev 02afce84f05c7841856a10671b6d7924f79193cc -> ec56a08f04ff318ea659fd58683f7ae4f22dc7a5
         impl_version 0.2.17 -> 0.2.18
[lupin]  rev 54f85e694d4c03e5cd40bef461f85ca0ac373332 -> 0cfc0cfc89af5fd2aeb71d46c86742745b902869
```

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.18 (wolfgang, pin ec56a08)
paired with lupin 0.1.41 (reference interpreter), pin 93a5fe5
$ lupin --version
lupin 0.1.41 (wolf-interp, reference interpreter at pin 93a5fe5)
```

From `crates/wolf_driver/PAIRING` at `ec56a08f`,
`vendor/upstream/PIN` at `v0.1.41`, and D57. The compiler's half is
**exact** (it names 0.1.41; the book pins 0.1.41). The interpreter's
half is a **distance** and it has grown: `93a5fe5` is v0.2.16's tag,
an ancestor of `ec56a08f`, **152** commits and **two** releases behind
(lupin 0.1.41 kept 0.1.40's pin). §1.2's and the colophon's "72" and
"one release" become "152" and "two releases", and "the compiler's
*previous* release tag" becomes the tag two releases back. Falsified
by any character that differs, or a count other than 152.

## 3. Spec artifacts (DERIVED both ways)

- Anchors **541 → 542**, `mem.model.place.elem` added, none dropped,
  none remapped. `mem.model` already exists, so namespaces hold at 13
  and documents at 11; Appendix D moves in its counts only.
- `grammar.ebnf` **holds** (`910ff9d5…` at both tags); Appendix A does
  not move.
- Catalogue **176 → 176**, same code set; Appendix C does not move.
  E1001's prose gains one paragraph (seven lines) before its
  `Fixtures:` line.

## 4. Flips at the subject run — **0** (PREDICTED)

The subject run is the unedited tree with the new pair. The probe sees
a flip only where the compiler stops declining a `lupin-run` program or
lupin starts serving a `wolf-run` one (TWO-MACHINES §6).

- **`lupin-run` non-trap fences (25).** Their compiler refusals are
  «`str` methods outside the builtin set», «indexing / generic
  application», «`copy` of a value nested this deep» (ch28's six,
  push's plain copy of a `Node`, wolf-book#57), and the like. 0.2.18
  relaxes exactly one static rule — two literal indices are distinct
  places for **moves** (EG1) — and no one-machine row is refused for a
  move of an element. `mut` claims still treat elements as one place
  (EG2 is not in this release), so ch28's `add(mut n.left[0], w)`
  meets nothing new either.
- **`wolf-run` fences (the `comptime fn` five, ch33's prefork, the ch01
  family).** Nothing in lupin 0.1.41 touches `comptime` or
  `os_spawn_with`; it only adds traps.
- Trap rows carry no automatic flip.

**Falsified by any flip.**

## 5. Failures at the subject run — **6**, all console transcripts (PREDICTED)

By name:

1. `book/back/colophon.md:7` — the `--version` block (§2).
2. `book/ch01.md:153` — the same block in §1.2.
3. `book/ch22.md:288` — `wolfi v0 · toolchain 0.2.17` → `0.2.18`.
4. `principles/exercises/ch22/EXERCISES.md:218` — the same line.
5. `principles/exercises/ch07/EXERCISES.md` — exercise 7-10's
   `wolf --explain E1001` gains the `mut`-parameter paragraph.
6. `principles/exercises/appx/EXERCISES.md` — exercise C-1's
   `wolf --explain E1001`, the same paragraph.

The first four are bs53's four with the next number; the last two are
new and come from the one catalogue entry that moved.

**Zero program failures**, for these reasons, each a static search over
all 573 program texts that is shown to fire on wolf-lang's own
witnesses before its zero is believed (`bs54-evidence/detect.py`):

- **#464** (a `mut` parameter left moved-out at return is E1001): the
  seven callee moves out of a `mut` parameter in the book all store
  back before every return (`swap` ×2 and `retitle` on ch07's page,
  ex7-6's `grow` through `b = bigger`, ex7-21's `remove` through
  `s.docs = kept` before both returns) or move a `Copy` value (ex7-7's
  `int` swap, ch32's `h.head`). The detector hits 5 of 5
  `mut_param_moveout_*.lu`.
- **#460** (a store no longer revives a moved sibling) and **EG1**:
  the book moves no non-`Copy` element anywhere. All 27 element
  bindings read an `int`, `char`, handle, raw pointer or `str` (a
  slice or an element) — `Copy` on the compiler and in lupin's
  `is_copy`. The detector hits 10 of 10 element-move witnesses.
- **#452** (checked machine: index, then value): no index store in the
  book has a call in its index, and none has a `mut` argument in its
  value, so no order is observable; the nested stores of ex7-15/16/20
  are pure arithmetic, which also keeps wolf-interp#145 (outer
  operands evaluated twice) invisible.
- **wolf-interp#141** (lupin traps a read of a moved element): with no
  element moved, no read can meet one. The only lupin programs that
  bind elements (ch16's ring, ex8-6, ex8-7, ex12-10, ex16-7/8) bind
  handles, `char`s and `int`s.
- **W1002 retiring beside E1001**: the book's one W1002 row is
  wolf-lang#325, closed; no transcript prints a W1002.
- **Element names in diagnostics** (`xs[_]` → `xs[0]`): no rendered
  diagnostic block or `snapshots/diagnostics/` file names an element
  (searched for `[_]`: hits only in ledger prose, ch16 and ch31).

**Falsified by any program failing at the subject run, or any console
failure not in the six above.** Totals at the subject run: **506
samples, 503 passed, 3 pending, 6 failed, 0 flips**; console **475 of
501** replayed (bs53's 481 less the six), then 481 of 501 after the
re-records. Export **374** unchanged.

## 6. Graduations by respelling — **0** (PREDICTED)

bs53 graduated five programs the probe could not see. The analogue here
would be a program the book respelled to dodge element one-place-ness
(wolf-lang#446's class), which EG1 would now let it spell naturally.
There is none: the book's `copy xs[0]` sites (§5.3's two `best`s,
ex5-7) are generic over `T` and also read `xs[i]` at a **run-time**
index, which EG1 item 2 keeps one place with every other index, so the
`copy` is still what makes them legal.

## 7. Pending: **3 → 3** (PREDICTED)

- `ch05/ex5-8`: unchanged (no std root; `take` unspellable, #58).
- `ch07/ex7-5`: unchanged, «borrow expressions» at `resolve`.
- `ch08/ex8-7`: both halves hold in the same words. eg01b's pool-handle
  revival and `pool[h]` naming are about moves; the compiler's half is
  «`==` on a container, tuple or handle», which nothing in the span
  touches, and lupin's is `[type.map.key]`'s «handle is not a `Map`
  key», which 0.1.41 does not touch.

## 8. The two element-`len` claims (PREDICTED; witnesses, not samples)

EG1 item 1 says an element move leaves the container's `len`
readable, and the book says twice that it does not.

**E1. §5.3's `best` without `copy`** (the paragraph at `book/ch05.md`
~420: "`var b = xs[0]` moves it out of the list, and the next line's
`xs.len` reads a value that moved away … a refusal at compile time, at
that `xs.len`"). Witness: §5.3's block with both `copy`s removed.
- wolf 0.2.17: refused, and the first E1001 is at `xs.len`.
- wolf 0.2.18: **still refused**, but **no diagnostic at `xs.len`**;
  the refusal lands on the run-time read `xs[i]` (one place with the
  moved `xs[0]`).
- lupin, both: runs, `340 espresso` (`int` and `str` are `Copy`).
Falsified if 0.2.18 still names `xs.len`, or accepts the program. If it
holds, §5.3's paragraph is rewritten to say where the refusal lands.

**E2. ch31's shard papercut** (open row, wolf-lang#153): `let s =
shards[i]` in a `while i < shards.len` loop over a `List` of structs.
- wolf 0.2.17: two E1001s, `shards.len` and `shards[_]` (as bs13
  recorded).
- wolf 0.2.18: **one** E1001, the loop-carried re-read of `shards[i]`,
  with the move site named `shards[i]`; no `shards.len`.
- lupin 0.1.40 and 0.1.41: runs, prints both names, exit 0 — each
  iteration moves a different element and the next reads another.
Falsified per line. The row stays open (the binding still moves).

## 9. Planted witnesses on the subject archive (PREDICTED)

The zero in §5 is believed only if the release refuses what it says it
refuses on this host: wolf-lang's `elem_const_store_no_revive_heap.lu`
and `mut_param_moveout_whole.lu` from `v0.2.18` — `fail(E1001)` on
`wolf conform-run --json` and on `--checked`; lupin 0.1.41 traps
`use-after-move` on the first and prints `1` on the second (#146, still
open); both run on the 0.2.17 archive.

## 10. Exercise 7-23 (PREDICTED)

- `ex7-23.lu`: wolf 0.2.18 `fail(E1002)` at `mem`, rendered diagnostic
  **byte-identical** to the page (no element named in it; the E1002
  entry moved only in `Fixtures:`). lupin 0.1.41 runs it to exit 0,
  `moves, and the shelf still holds 3` — the returned `s.docs[best]` is
  a crossing lupin copies, and `shelf.docs.len` reads no element.
- `ex7-23b.lu`: `run(exit=0)` on both.

## 11. Re-records and literals, by name (PREDICTED)

Moves: §5's six console blocks; §1.2's two Windows archive names and
`0.2.17+dev.unknown`; §1.2's and the colophon's distance sentences
(152, two releases, "previous" tag); `book/back/solutions.md`
regenerated for 7-10, C-1 and 22-13's stamp. Holds: ch22's two export
hashes and ch25's `tree=`/`manifest=`/`interface=` (`crates/wolf_pkg/`
unchanged); Appendix E's `wolf --help` (`help.rs` unchanged); every
`wolf conform-run` and `lupin conform-run --explore` transcript;
ex11-2's deadlock roster (no byte of the file moves). Exercise index
tiers **192 / 31 / 26** unchanged. Ledger **132 open** unchanged (re-
measure notes close nothing; E2's row stays open).

## 12. Method

Control and subject, as bs50–bs53 ran it: the unedited tree at
`f2f4280` with the OLD pair (0.2.17 / 0.1.40, bs53's archives re-hashed
against their release API) first, then the same unedited tree with the
NEW pair, so the toolchain is the one variable. Both on kasumi (linux
x86-64), from release archives verified by digest. Witnesses are run
one program at a time on both pairs before any page changes.
