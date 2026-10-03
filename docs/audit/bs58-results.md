# bs58 — results against the prediction

Prediction committed at `68563a8` (`docs/audit/bs58-prediction.md`),
before `wolf-0.2.21-*.tar.gz` or `lupin-0.1.44-*.tar.gz` was downloaded.
Measured 2026-10-03 on kasumi (linux x86-64) from the release archives,
each matched against `gh release view --json assets` the same minute
(`bs58-evidence/archive-digests.txt`): wolf 0.2.21 `09e0a6f5…`, lupin
0.1.44 `e44aae06…`; control wolf 0.2.19 `9f3873d8…`, lupin 0.1.42
`9856335a…`. Every member hashed by name (`members.txt`).

## The flip table

| sample | predicted | measured (subject run, unedited tree `68563a8`) | |
|---|---|---|---|
| `book/ch06/part-rowmatch` | FLIP; four lines on wolf, checked and lupin | `FLIP book/ch06/part-rowmatch`; `340 cents` / `nothing owed` / `` `lots` at byte 7 is not a number `` / `no comma` on `conform-run --checked` and lupin 0.1.44, `pass` on the default lane | **held** |
| `book/ch06/s7` | FLIP; `fail(E0801)` naming `no_comma`, snapshot byte-identical; lupin E0801, exit 2 | `FLIP book/ch06/s7`; `fail(E0801)` on both compiler lanes; the head run compares the page's block and the snapshot and passes; lupin «E0801: this `match` does not cover `no_comma`», exit 2 | **held** |
| `ch06/ex6-15` | FLIP; `7 -4 -99` on wolf and lupin; checked declines `Weird` | `FLIP ch06/ex6-15`; lupin `7 -4 -99`; default lane `pass`; checked «module items in checked execution» | **held** |
| 19 non-trap `lupin-run` fences | 0 flips, each refusal word for word | 0 flips; every verdict and construct identical (`probes-control.txt` vs `probes-subject.txt`, normalized diff empty outside §6.6) | **held** |
| 6 `wolf-run` fences | 0 flips | 0 flips; lupin still exits 4 on all six | **held** |
| trap rows (B-5 included) | no hand flip | B-5 still `trap(exclusivity)` on lupin, E1002 on wolf | **held** |
| `samples-os.toml` (prefork on windows) | holds | not exercised on linux; CI's windows leg is the measurement | — |

Runner arithmetic: predicted **509 / 503 passed / 3 pending / 4
failed / 3 flips**, measured **509 / 503 / 3 / 4 / 3** (`subject-summary.txt`).
Console blocks predicted 477 of 502, measured **477 of 502**.

## Everything else

| figure | predicted | measured |
|---|---|---|
| stamps | `wolf 0.2.21 (wolfgang, pin dfcc2f1)` / `paired with lupin 0.1.44 (reference interpreter), pin cdde128` / `lupin 0.1.44 (wolf-interp, reference interpreter at pin cdde128)` | **byte-identical** (`versions.txt`) |
| distance | 91 commits, one release | 91 (`git rev-list --count cdde128a..dfcc2f13`) |
| console failures | the four stamp sites, by name | **the same four**: colophon:7, ch01:153, ch22:288, ch22 EXERCISES:218 |
| program failures | 0 | **0** |
| pending | 6 → 3, the standing three in the same words | **3**; ex5-8, ex7-5, ex8-7 byte-identical on both pairs (md5 of each probe block, commit and version fields removed) |
| #176 parting | none | **none**: no book program meets it |
| anchors | 542 → 546, +4, none dropped | **546**, the four named, none dropped or remapped (`anchors-diff.txt`) |
| grammar | holds | holds (`910ff9d5…`) |
| catalogue | 176 → 178 | 178; verify-docs read Appendix C's sentence at 178 |
| glibc ceiling | ≤ 2.34 | **GLIBC_2.34** on both binaries, both pairs |
| `ledger --check` | 133 → 132 open | **131 open**, 101 closed — **missed by one**, below |
| verify-docs | 345 (296 printed), tiers 192/9/31/27/78/8 | **held** exactly |
| backmatter | 296 / 331; solutions.md regenerated only if 6-15's apparatus moved | 296 / 331; **solutions.md drifted on 22-13's `toolchain` line**, not on 6-15 — the same drift bs55 met; regenerated (`960a292`) |
| export | 377 → 380 | **377** — **missed**, below |
| self-test | — | 14/14 |
| render | — | web, 48 pages, 1328 links none dead, CSP guard |

## The misses, plainly

1. **The ch05 row (§8 of the prediction) was wrong on its central
   claim, and it was wrong on the control too.** I predicted
   `refund(340)` stays «E0502: `i32` does not implement `Neg`» on both
   compilers. It prints `-340` on wolf 0.2.19 and 0.2.21 alike, on the
   default and checked lanes, and so does §5.6's `total` block with
   its integer list written bare (`715 5.75`). wolf-lang#347 closed on
   2026-09-15 with `[type.numlit.default]`'s "A bound is context",
   which has been in the spec since v0.2.15. The book's ledger row and
   §5.6's sentence ("the list written without it is refused at the call
   by E0502, naming `i32`") had been false for five bumps; s175's
   comment on #347 said the sentence "stays true", and no bs lane since
   probed it. This lane corrected the paragraph and closed the row
   (`12c2581`). That is why the ledger reads 131, not 132: two rows
   closed, not one. Re-counted by `ledger --check`, never subtracted.
2. **The export count.** The export holds back one-machine directives
   (125, unchanged), not pending rows, so §6.6's three programs were
   already in it at the control. 377 → 377.
3. **solutions.md**: I expected it to move for 6-15's comment, if at
   all. 6-15's solution page does not quote the `.lu` header; the
   drift was 22-13's `toolchain 0.2.19` line, which every stamp bump
   moves.

## Rulings #19, #20, #21, #33 (item 3)

- **#21**: advanced. The three rows leave; chapter 6's `ba:blocker
  (bs57)` closes (`796e962`); 6-15 is `run (wolf + lupin)` in the index.
- **#19** (E0611) and **#20** (a block's `errdefer`): no book row,
  sample or claim exists for either; the static search finds no `?`
  under a defer and both `errdefer`s are function-level. Nothing to
  advance and nothing left pending.
- **#33** (s202, wolf-lang#458): no row for the binding rule itself; the
  neighbouring ch05 row is closed as above, measured.

## Open issues the pin moves

None of wolf-book #60, #58, #57, #48, #45, #43 or #29 moves: ex8-7
(#60) and ex5-8 (#58, #45) answer in the same words on both pairs;
ch28's wordtree family (#57) is still «`copy` of a value nested this
deep»; #48, #43 and #29 are about the book's own tooling and text.

## Gate at head

See `gate-summary.txt` (the head gate on kasumi) and the PR's CI runs
on three hosts.
