# bs61 — prediction (committed before the first change)

Rulings #40 (A) and #41 (label), at wolf-book `e7772338`, wolf 0.2.23 /
lupin 0.1.46. Written from the trunk tree and the runner's source; nothing
here was measured on a machine yet.

## 1. The tier column, by directive head

The generator reads each row's programs (`chNN/exN-M.lu`, `exN-Ma.lu`,
`exN-Mb.lu`, and for chapter 22 the package directories its stems name),
skips `//! member: true` files, and writes:

| the heads of the row's programs | tier |
|---|---|
| any program in `samples-pending.toml` | `pending` |
| `run(…)` (incl. `trap`, `nonzero`) or `ub(…)` | `run (wolf + lupin)` |
| `lupin-run(…)` only | `run (lupin)` |
| `wolf-run(…)`, `fail(…)`, `audit(…)` only | `run (wolf)` |
| a mix | the union of the machines |

A row with no program on disk keeps its hand-written tier (prose, the REPL
rows, the transcript and workflow rows); the generator says how many.

Predicted moves, **168 rows**:

| head(s) | from | to | rows |
|---|---|---|---|
| `run(` | `run (lupin)` | `run (wolf + lupin)` | 145 |
| `run(` | `run (wolf)` | `run (wolf + lupin)` | 4 (2-9, 30-3, 30-4, 30-5) |
| `run(` | `pending` | `run (wolf + lupin)` | 1 (17-6) |
| `fail(` | `run (wolf + lupin)` | `run (wolf)` | 9 (3-2, 7-3, 7-8, 8-8, 12-9, 13-3, 16-9, 20-8, C-2) |
| `fail(` | `run (lupin)` | `run (wolf)` | 4 (1-6, 22-2, 22-3, 22-4) |
| `fail(` | `pending` | `run (wolf)` | 1 (20-5) |
| `fail(` + `run(` | `run (lupin)` | `run (wolf + lupin)` | 3 (3-8, 22-5, 22-12) |
| `lupin-run(` + `run(` | `run (lupin)` | `run (wolf + lupin)` | 1 (7-4) |

Unmoved with a program: 30 `lupin-run(` → `run (lupin)`, 15 `run(` and 2
`fail(`+`run(` → `run (wolf + lupin)`, 14 `fail(` and 5 `wolf-run(` →
`run (wolf)`, 3 manifest rows → `pending` (5-8, 7-5, 8-7). Hand-kept:
108 (78 prose, 9 REPL, 8 `run (lupin)`, 8 `run (wolf)`, 2
`run (wolf + lupin)`, 3 `pending`).

Totals line, predicted: **38 run (lupin) · 9 run (lupin REPL) · 41 run
(wolf) · 173 run (wolf + lupin) · 78 prose · 6 pending = 345** (today
191 · 9 · 31 · 28 · 78 · 8).

The `fail(` rows lose lupin because the runner never asks lupin about a
`fail(…)` file; a page that prints lupin's refusal beside the compiler's
is replayed by the console gate, which is not the head. The ruling says
the head. 17-6 and 20-5 stop being `pending` because CI meets their heads
today; their pending-ness (an unprinted stem, an unverified attribute)
stays in EXERCISES-PENDING.md and the index prose.

`--check` will be seen red on one hand-edited row (a `run(` row set back
to `run (lupin)`), then green.

## 2. The quote checker

Widened to every exercise page (`principles/EXERCISES.md` and the 30
`principles/exercises/*/EXERCISES.md`) and to every label that names a
`.lu` and is followed by a wolf fence, whatever sits between the name and
the colon; a bare name resolves against the page's own directory. A fence
is the whole file, or its label opens a parenthetical with `excerpt`
and its fence is the file's lines in order, `...` at each elision.

Predicted at trunk: **114 fences, 20 red.** The 8 of wolf-book#48 (10-7,
13-5, 21-2 unlabeled windows; 9-5, 9-13, 11-5, 13-7 dedented windows;
14-7) and 12 windows whose labels bs47's parser could not see (7-15,
7-16, 7-20, 7-23, 8-6, 8-15, 14-9, 15-6, 16-2, 16-7, 18-15, 27-1).
Nineteen are fixed on the page by labeling `(excerpt)` (keeping any
description after it) and restoring the file's indentation and the
enclosing lines, or a `...`, at each elision. None needs a file change.

## 3. ex14-7

**The page is wrong.** 2372871 (wolf-book#21) moved every bare statement
send in chapter 14's corpus to `?` or `else { return total }`, and moved
14-6's page fence with its file, but not 14-7's. The file's line is the
one CI runs; the page's bare `replies.send(total)` is predicted to be
refused or warned on (not the clean run the file gets) on the compiler at
0.2.23, measured on both machines before the page changes. No file
changes, so 14-7's `//! check:` is untouched.

## 4. What I expect to be wrong

The package-directory map for chapter 22 is the one judgment in the
generator; if a stem names a directory I did not map, `--check` must say
"a program no row claims", not pass. And the 18 hand-kept transcript rows
are not checked by anything new.
