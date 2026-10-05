# bs61 — results against the prediction

Rulings #40 (A) and #41 (label) at wolf 0.2.23 / lupin 0.1.46 (release
archives by digest: wolf linux x86-64 `6f505eb5…`, lupin `d13a0379…`),
measured on kasumi under `~/lanes/bs61/`. Every log named here is in
`docs/audit/bs61-evidence/` with its digest.

## 1. The tier column — as predicted, all 168

`cargo xtask tiers --check` at `21f53bc` (the generator and its CI step,
the index untouched): **168 rows** named, exit 1
(`tiers-21f53bc-1-cargoxtasktierscheck.log`, `275b8ef0…`). `cargo xtask
tiers` then moved exactly those rows (`tiers-21f53bc-2-…`, `6228b54b…`),
and `--check` went green (`tiers-21f53bc-4-…`, `472321c0…`). With 1-5's
generated tier set back to `run (lupin)` by hand, `--check` named that
one row and exited 1 (`tiers-21f53bc-6-…`, `b5841edf…`).

| head(s) | from | to | predicted | moved |
|---|---|---|---|---|
| `run(` | `run (lupin)` | `run (wolf + lupin)` | 145 | 145 |
| `run(` | `run (wolf)` | `run (wolf + lupin)` | 4 | 4 |
| `run(` | `pending` | `run (wolf + lupin)` | 1 | 1 (17-6) |
| `fail(` | `run (wolf + lupin)` | `run (wolf)` | 9 | 9 |
| `fail(` | `run (lupin)` | `run (wolf)` | 4 | 4 |
| `fail(` | `pending` | `run (wolf)` | 1 | 1 (20-5) |
| `fail(` + `run(` | `run (lupin)` | `run (wolf + lupin)` | 3 | 3 |
| `lupin-run(` + `run(` | `run (lupin)` | `run (wolf + lupin)` | 1 | 1 (7-4) |

Totals: **38 run (lupin) · 9 run (lupin REPL) · 41 run (wolf) · 173 run
(wolf + lupin) · 78 prose · 6 pending = 345**, as predicted. 237 rows read
off their heads, 108 hand-kept.

**One miss in the generator, not the prediction.** At the head gate on
`5778175`, `tiers --check` refused `appx/.lu-cache/` as "a package
directory with no owning exercise" (`tiers-cache-red-5778175.log`,
`aff36140…`): `cargo xtask samples` had run first in the same checkout
and left its build cache in every chapter. §4 of the prediction named
the package map as the one judgment that could be wrong; it was wrong
about what a package is, not whom it belongs to. Fixed in `93be88d`
(a package is a visible directory holding a `.lu`), with a test, and
the gate re-runs `tiers --check` and `cargo test` after `samples` now.

## 2. The quote checker — as predicted, 20 of 114

At `9df056b` (the widened checker, no page changed): `verify-docs` 20
failures, exit 1, the 20 predicted by page and line
(`quotes-red-9df056b-verify-docs.log`, `d2a89015…`); `cargo test` red on
the two real-file tests (`quotes-red-9df056b-xtask-test.log`,
`a517c128…`). At `93be88d`: "114 quoted solution fences on 32 exercise
pages are their files (92 whole, 22 excerpts)"
(`quotes-green-93be88d-verify-docs.log`, `b8b94f10…`).

Nineteen windows were fixed on the page — a label that opens with
`(excerpt`, the file's indentation, `...` at each interior elision;
none needed a file change. The doctrine page's convention was kept:
the window's ends need no `...` (its 6-3 has none); the fences that
open or close inside a block carry one anyway.

**Not predicted: the line keys moved.** Three `samples-declined.toml`
rows key a console block by page line, and three edits pushed their
blocks down (EXERCISES.md +2, ch09 +3, ch11 +2): `samples` 6 failed at
`013087d` (`samples-declined-red-013087d.log`, `5702feb8…`), fixed in
`5778175`.

## 3. ex14-7 — the page was wrong, as predicted

`ex14-7-probe.txt` (`fb904836…`), at `9df056b`: the file
(`a467bf92…`) runs `before=7 after=3`, exit 0, on lupin and `wolf run`,
`conform-run` verdict `pass` with no warning. The page's line
substituted into the same file (`8558f691…`) runs the same on both, and
the compiler prints **W0601** ("this `() ! {cancelled, closed}` result
is discarded") at 8:21 — the warning 2372871 (wolf-book#21) removed from
chapter 14 by spelling this send `else { return total }` in the file and
in 14-6's fence, and missed in 14-7's. Predicted "refused or warned";
it is warned. The page now prints the file's line; no `.lu` changed.

## 4. The head gate, `93be88d`

`gate-summary-93be88d.txt` (`4b11c089…`): fmt, clippy `-D warnings`,
193 xtask tests, `tiers --check`, `verify-docs`, `ledger --check` (128
open, 0 unfiled), `backmatter --check`, `samples` (509 / 506 passed / 3
pending / 0 failed / 0 flips; 482 of 502 consoles; 4 SKIP lines, the
same four as bs60's head), `samples --self-test` (14/14), `contrast`,
`render web` (1328 links, none dead), then `tiers --check` and `cargo
test` again after `samples` — every step exit 0.

## 5. What this lane did not change

- The 108 hand-kept rows. Five of them carry a run tier and print
  neither a program nor a transcript — 10-5, 11-3, 17-4 (`run (lupin)`)
  and 20-10, 21-6 (`run (wolf)`) answer in prose. Ruling #40 is about
  rows with a head, so these are reported, not moved.
- Directory labels (``Solution. `ch22/metrics/`:``): four fences name
  their file in a first-line comment; the three that quote a whole file
  match it, measured once by hand. `verify_quoted_solutions` does not
  hold them.
