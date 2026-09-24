# bs51 — the prediction, committed before either archive was unpacked

Wave 46, subwave 3. Written 2026-09-24 against wolf-book trunk
`44195fc`, **before** `wolf-0.2.16-*.tar.gz` or `lupin-0.1.38-*.tar.gz`
was downloaded, and before any other file in this repository was
edited. Everything below that is labelled DERIVED was computed from the
two upstream git histories (a clone is not an archive); everything
labelled PREDICTED is a claim about what the release binaries will do,
and each one carries the number or the name that falsifies it.

bs50 predicted two flips and measured ten, and all eight it missed were
the interpreter's. Its closing line — **the interpreter is the thing to
predict** — is the rule this file is written under, and §7 below says
why this bump is expected to invert it.

## 0. Drift against the contract's Inputs line, reported not absorbed

The contract (`sprints/book/bs51-the-book-at-0216.md` §2) says
wolf-lang#431 "still deadlocks native **on linux** with a scope handle
as a parameter", and `sprints/wave-46.md`'s s174 row says "the **linux**
native deadlock".

**The issue says windows.** wolf-lang#431's title is "a Scope handle
passed as a parameter deadlocks the native lane **on windows**", and its
measured table reads `exit(0)` / stdout `1` on linux x86-64 (ubuntu-
latest and kasumi) and on macOS aarch64, and "no verdict — killed at the
60 s observation cap, twice" on windows-latest — CI runs 35415995192 and
35417514571, job "build + test (windows-latest)", step "windows native
rung — corpus lane coverage". wolf-lang's CHANGELOG entry at `2f8deb7f`
says the same thing in the same words.

This matters to this lane specifically, because the book's samples gate
runs on all three hosts and windows is one of them. The posture the
contract asks for — the program stays `lupin-run` and says why — is
unchanged by the correction; the host named in the book's prose and in
its ledger row is not. **The book will name windows.**

Everything else in the contract's §2 re-derived clean:

| claim | re-derived | result |
|---|---|---|
| wolf-book trunk `44195fc` | `git rev-parse origin/trunk` | `44195fcca59327045019d3b2ed1400ffa0fe1463` ✓ |
| book pins wolf `2e4ca769` / 0.2.15 | `git show origin/trunk:wolf-toolchain.toml` L808-810 | ✓ |
| book pins lupin `552786e` / 0.1.37 at pin `41695e7` | same file L1328-1330 | ✓ |
| wolf 0.2.16 = tag `v0.2.16` = `93a5fe50` | `git rev-parse v0.2.16^{commit}` | `93a5fe504593ca7642b78ba83b4986e7a03cfe71` ✓ |
| release 395302343, four assets, digests `b0431056…` `fe1966a4…` `f984c7a7…` `84e30c05…` | `gh release view v0.2.16 --json assets` | all four ✓, `isDraft:false`, `isPrerelease:false` |
| lupin 0.1.38 = `ba357aa`, five assets | `gh release view v0.1.38`, `git rev-parse v0.1.38^{commit}` | `ba357aa6a2e32040d4f089d2cefe05bab86c4f86` ✓, five assets ✓ |
| lupin 0.1.38 is pinned on the released line | `git show v0.1.38:vendor/upstream/PIN` | `2e4ca769b396219585a07ff18492529c944672d9` — v0.2.15's own revision ✓ |
| `2e4ca769` is an ancestor of `93a5fe50` | `git merge-base --is-ancestor` | true ✓ |

## 1. The pin (DERIVED, not predicted)

```
[wolf]   rev 2e4ca769b396219585a07ff18492529c944672d9 -> 93a5fe504593ca7642b78ba83b4986e7a03cfe71
         impl_version 0.2.15 -> 0.2.16
[lupin]  rev 552786e39c5660e2e58b7facadd54195df3243e8 -> ba357aa6a2e32040d4f089d2cefe05bab86c4f86
```

The four members of each archive will be hashed BY NAME, because
`_wolf` is the zsh completion script and hashes to the same constant at
every pin on every platform — a check that reads "the first binary it
finds" cannot go red at any release.

## 2. The stamps (PREDICTED)

```
$ wolf --version
wolf 0.2.16 (wolfgang, pin 93a5fe5)
paired with lupin 0.1.38 (reference interpreter), pin 2e4ca76
$ lupin --version
lupin 0.1.38 (wolf-interp, reference interpreter at pin 2e4ca76)
```

Derived from `crates/wolf_driver/PAIRING` at `93a5fe50`
(`lupin-version = 0.1.38`, `lupin-pin = 2e4ca76`), from
`vendor/upstream/PIN` at `v0.1.38`, and from D57's rule that a build
made exactly at a release tag prints the bare number with a seven-
character pin clause. **Falsified by any character that differs.**

## 3. Both halves of the pair, and what closes (PREDICTED)

- **The compiler's half is EXACT.** It names `lupin 0.1.38`, and this
  file pins 0.1.38. Second printing running.
- **The interpreter's half is a DISTANCE AGAIN, for the first time
  since 0.1.36.** `2e4ca76` is v0.2.15's own revision and an ancestor of
  the compiler's `93a5fe50` (checked in wolf-lang, not assumed), so the
  two clauses can be read as a count for the first time in three
  printings. `git rev-list --count 2e4ca769..93a5fe50` = **148**, and
  the interpreter's half is **exactly one release** behind.
- So §1.2's and the colophon's long paragraphs about "a printed
  revision that is not a place you can go", the rebased development
  head, and `refs/tags/lupin-0.1.37-conformance-pin` **retire**, and the
  sentence "do not read the two clauses as a distance" **inverts**. That
  is the interpreter's half of the book's pin closing.

**Falsified by** a count other than 148, or by the ancestry check
failing.

## 4. The spec artifacts (DERIVED both ways)

- **Anchors 524 → 539, +15, NONE dropped, NONE remapped**, key sets
  diffed both ways (the is30 lesson). Added:
  `conc.proc.arg`, `conc.proc.handle`, `conc.proc.join`, `conf.exit`,
  `conf.exit.class`, `conf.exit.collide`, `conf.exit.refused`,
  `conf.exit.static`, `conf.exit.test`, `mem.region.imm.ret`,
  `mem.region.root`, `proto.cmp.pass`, `proto.record.pass`,
  `proto.record.trap`, `type.err.alias.qualified`.
  Namespaces hold at **13**, documents at **11**, the namespace-to-
  document map is unchanged in both directions, so Appendix D
  regenerates to itself apart from its counts.
- **`grammar.ebnf` is BYTE-IDENTICAL** across the span — sha256
  `2b6269d7ce926d89ab537930580ace04417e2866dc544dcba9fa83336d1642ac` at
  `2e4ca769`, at `93a5fe50` and in `vendor/spec/grammar.ebnf` today.
  Appendix A regenerates to itself and the vendored copy is not
  touched.
- **The catalogue holds at 176**, no code added and none removed,
  derived from `docs/diagnostics.md` at both revisions and diffed both
  ways. Appendix C's opening count does not move.

## 5. `[conf.exit]`: the fence count that changes is **ZERO** (PREDICTED)

s169 ruled that a rejection exits **2** and a refusal **4**, never the
`1` a program that ran and returned an error exits with. bs46 found two
`run(exit=1)` fences that were rejections scored green for four bumps
and fixed both, and the guard it added has been holding since. Five
directives in the tree carry `exit=1` today:

```
book/ch01.md:573        run(exit=1, stdout="error: parse")
book/ch06.md:343        run(exit=1)
book/ch06.md:507        run(exit=1)
principles/exercises/ch15/ex15-5.lu    run(exit=1)
principles/exercises/ch33/ex33-4.lu    run(exit=1, stdout="error: io")
```

Every one is predicted to be a program that **ran** and returned an
error, so none becomes `exit=2` and none becomes `fail(CODE)`.
**Falsified by any one of the five failing at the new pin.**

## 6. The two runner workarounds retire (PREDICTED)

- `compiler_still_declines` reads `fail(` or `x-unsupported-construct`
  because the bare verdict was useless: `unsupported` was also what the
  default lane stamped on a program it accepted. s169's
  `[proto.record.pass]` makes a clean lowering on the default lane
  `pass`. **Prediction: `wolf conform-run` on a clean program answers
  `"verdict":"pass"` at 0.2.16 where it answered `"unsupported"` at
  0.2.15**, so the probe becomes `verdict == "pass"`. Falsified by the
  verdict still reading `unsupported`.
- `compile_failed` greps the driver's closing prose, *the package does
  not compile*. `[conf.exit.collide]` is written at this exact harness:
  "a rejection or a refusal NEVER exits 0 or 1 … a harness
  distinguishing outcomes by grepping a driver's prose is a harness owed
  this clause." So the number settles every `run(exit=0)` and
  `run(exit=1)` fence on its own, and the ambiguous claims
  (`run(exit=nonzero)`, and any future `exit=2`/`exit=4`) read the
  record's verdict instead of the prose. **Falsified by** the retired
  grep failing to reproduce bs46's two planted defects when the
  self-tests run.

## 7. Flips: **10**, and this time they are the COMPILER's (PREDICTED)

bs50's rule says predict the interpreter. This bump is expected to
invert it, and the reason is arithmetic rather than hope: lupin
0.1.37 → 0.1.38 is a **re-pin** release (`git log v0.1.37..v0.1.38` is
29 commits, of which the behaviour-changing ones are the corpus re-pin,
`trap_message` in the schema and two E0206/E1001 conformance keys),
while wolf 0.2.15 → 0.2.16 is **148 commits** carrying three separate
lowering expansions: s168's place model, s170's conc handle types and
s173's pool plus unsafe tier.

Predicted flips, by name:

1. `book/ch07.md:965` — the closer's shelf block, the chapter's one
   «assignment through nested places (c06)» row. s168's
   `wir: one place walk for mut lends and member writes — a container
   element is a place` is exactly `s.docs[i].words = w`.
2. `book/ch16.md:193` — the snapshot block,
   `lupin-run(exit=0, stdout="two procs read one snapshot: 1540 and 1540")`.
   s170 landed `[conc.proc.arg]`: a frozen `spawn proc` argument is
   shared, not moved.
3–6. **Four of ch28's six** «`mut` arguments beyond local places (c06)»
   rows (`book/ch28.md:318` and the `ex28-*` set), same s168 walk.
7–8. **Two of ch08's six** «Pool/shared constructor lowering (runtime
   shapes)» rows — s173 narrowed three refusals and advanced three
   witnesses mem → run.
9–10. **Two of ch09's** «raw-pointer casts» / «assume noalias» rows —
   s173's unsafe tier lowers, four witnesses mem → run.

**Falsified by any number other than 10.** The named list is scored
both ways: a name that did not flip and a flip that was not named are
both recorded.

Predicted **not** to flip, with reasons, because a prediction that only
names winners is not a prediction:

- `book/ch11.md:20`, the capability block. `Scope` is a prelude type
  name at 0.2.16 (s170), but the sample's parameter is an **unbounded**
  generic `[S]`, and §5.6's rule is that a generic body may use only
  what its bounds grant — `S` grants no `spawn`. A type name existing
  elsewhere does not change what `[S]` grants, so the compiler still
  refuses at the definition and the fence stays `lupin-run`. The prose
  sentence "A scope handle has no type name — not `Scope`, not `scope`;
  both are a static error on both machines" **becomes false at this
  pin and must be rewritten**, which is the row advancing even though
  the fence does not.
- `book/ch14.md:79` and `book/ch14.md:316`, the two `reason_of[P]`
  blocks. Same shape, same reason.
- Every `lupin-run(exit=trap(k))` row. TWO-MACHINES §6: a trap claim
  retires on a runtime fact and the graduation probe never executes
  anything, so these carry no automatic flip by design.

## 8. New failures: **1**, and it is the one being written (PREDICTED)

wolf-lang#444 (s177) makes `move` on a `Copy`-typed place record the
move, so reading the place afterwards is E1001 where 0.2.15 printed the
moved value on both wolfgang lanes. That is a rejection that did not
exist at the old pin.

**Prediction: no existing `run(…)` sample goes red on it**, because the
only place in the book that reads a moved leaf is the paragraph in §7.2
that has no sample — which is the maintainer's finding and item 3 of
this lane. **Falsified by any existing sample failing with E1001.**

## 9. §7.2's new sample (PREDICTED)

Added after the "nothing to free, nothing to read" paragraph, pinned
`fail(E1001)`, the mirror of wolf-lang's
`corpus/memory/move_field_use_after.lu`:

- on **0.2.16** both machines refuse it — wolf statically with E1001,
  lupin dynamically with `trap(use-after-move)` at
  `[mem.tier0.move.2]`;
- on **0.2.15** only lupin did.

The paragraph's sentence becomes true by a test rather than by
assertion. **Falsified by** the compiler accepting it at the new pin.

## 10. Exercise 8-7 (PREDICTED)

**FLIPS.** The pool LRU has been pending since bs04 — `pool[h].next = k`
did not denote a place — and s173 landed `Pool[T]`, `handle T`, the
pool accessor set and `pool[h].next = k` as the corpus centrepiece with
three lanes agreeing. Predicted verdict `run(exit=0, stdout="c a")`
green on both machines; **pending 3 → 2**.

The risk is named rather than hidden: s173 is the **compiler's**, and a
`run(…)` fence is a claim about both machines. lupin 0.1.38 is a re-pin
release. If the interpreter declines the place write, the row's blocker
text is **re-measured and rewritten**, not removed, and pending stays
at 3. `ch05/ex5-8` cannot flip (wolf-book#58's unspellable `take`,
wolf-book#39's std root) and `ch07/ex7-5` cannot (E1003 reaches no
verdict anywhere).

## 11. Console re-records, by name (PREDICTED)

Moves:

- §1.2's and the colophon's `--version` blocks — 2.
- ch22 §22.2's `wolf interface`, and the same stamp line in
  `principles/exercises/ch22/EXERCISES.md` and `book/back/solutions.md`:
  `toolchain 0.2.15` → `toolchain 0.2.16` — 3 sites, one literal.
- **Appendix E's `wolf --help` transcript re-records.** It stayed quiet
  at the 0.2.10 bump because `crates/wolf_driver/` moved only in data
  and one publish path; across this span `crates/wolf_driver/src/help.rs`
  moves (+21 lines: `wolf test`'s four row words and `--error-limit`,
  `conform-run`'s `--deny-warnings`). This is the third re-record site
  finally firing.

Holds, predicted with a reason:

- ch22's `export_hash`/`pkg_hash` and ch25 §25.2's `tree=`, `manifest=`
  and `interface=`. #292 took the compiler's version out of the digest
  at 0.2.10, and `git diff --stat 2e4ca769 93a5fe50 -- crates/wolf_pkg/`
  is **empty**. A release bump on a package nobody edited must move
  none of the six numbers.

Unbounded, stated rather than guessed: the book replays **23**
`wolf conform-run` console blocks and **4** `wolf test` blocks, and
s169 rewrites both surfaces (the `pass` verdict, `trap_message`,
`x-trap-pos`, the REJECTED column). Some of those will move. I do not
have a count and I am not inventing one — a figure no artifact carries
is a memory.

## 12. Method

The measurement is a **control** and a **subject**, as bs50 ran it: the
unedited tree at `44195fc` with the OLD pair first, then the same
unedited tree with the NEW pair, so the toolchain is the one variable
and nothing the lane edits can be mistaken for something the release
did. Both runs on kasumi (linux x86-64, 16 cores), from the release
archives, never from a source build and never from `~/.local/bin`.
