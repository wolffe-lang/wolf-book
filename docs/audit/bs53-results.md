# bs53 — results, scored against the prediction

The prediction is `docs/audit/bs53-prediction.md`, committed at
`e4991f1` (pushed to origin) before either archive was downloaded; the
contract is `docs/audit/bs53-contract.md` at `5ff06bf`. Every run below
is on kasumi (linux x86-64) against release archives verified by digest
(`bs53-evidence/archive-digests.txt`, `members.txt`, `versions.txt`).
Full logs stay on kasumi under `~/lanes/bs53/logs/`; the lines that
carry each number are in `bs53-evidence/gate-summary.txt`.

## 1. The archives

| archive | release digest = computed sha256 | member of record |
|---|---|---|
| `wolf-0.2.17-x86_64-unknown-linux-gnu.tar.gz` | `a95d0f0f…` | `wolf` `5cdd936e…` |
| `lupin-0.1.40-x86_64-unknown-linux-gnu.tar.gz` | `509929e6…` | `lupin` `18d64444…` |
| `wolf-0.2.16-x86_64-unknown-linux-gnu.tar.gz` (control) | `84e30c05…` | `wolf` `b53b5328…` |
| `lupin-0.1.38-x86_64-unknown-linux-gnu.tar.gz` (control) | `828b5c55…` | `lupin` `f202d47f…` |

The control pair was copied from `~/lanes/bs51/archives/` and re-hashed
against the v0.2.16 / v0.1.38 release APIs. `_wolf`, `wolf.bash` and
`wolf.fish` hash the same at both pins (`2d1e4801…`, `29eb2d75…`,
`51a1b6a8…`), as bs51 recorded.

## 2. The prediction, scored

| § | predicted | measured | verdict |
|---|---|---|---|
| 2 | stamps `wolf 0.2.17 (wolfgang, pin 02afce8)` / `paired with lupin 0.1.40 …, pin 93a5fe5` / `lupin 0.1.40 (… at pin 93a5fe5)`; distance 72 | byte-identical (`versions.txt`) | ✅ |
| 3 | anchors 539 → 541, +2, none dropped | +`mem.model.place.rhs`, +`os.fs.path.domain`; `backmatter --check` matches the v0.2.17 sibling | ✅ |
| 3 | grammar moves by two lines; Appendix A by the same two | `appendix-a.md` +2, exactly those | ✅ |
| 3 | catalogue 176 → 176, no explain transcript moves | `verify-docs` ok; no `--explain` block drifted | ✅ |
| 4 | **1 flip**, `book/ch14/part-watch` | 1 flip, `book/ch14/part-watch` (subject run) | ✅ |
| 4 | ch28's six hold | no flip | ✅ verdict, ❌ reason (§4 below) |
| 5 | 0 sample failures | 0 sample failures; the subject's 4 FAILs are §9's console re-records | ✅ |
| 6 | G1–G5 graduate on both machines by respelling | all five: exit 0 on lupin 0.1.40 and on `wolf run` 0.2.17, same stdout (`probes-new.txt`) | ✅ 5/5 |
| 6 | ex11-2 stays the permitted deadlock | lupin `trap(deadlock)` exit 3; `wolf run` killed at 120 s (124); default lane `pass` | ✅ |
| 6 | one-machine non-trap fences 31 → 25 | 25 at head | ✅ |
| 7 | pending 3 → 3, ex8-7's halves in the same words | 3; ex8-7 identical on both pairs | ✅ |
| 8 | 7-23: `fail(E1002)` byte-identical; lupin exit 0; 7-23b both | as predicted, identical against the old pair too | ✅ |
| 9 | re-records: 2 `--version`, ch22's `toolchain` ×3, Windows names, `+dev.unknown`; hashes and `--help` hold | exactly those | ✅ |
| 9 | index tiers 192 / 31 / 26 | `verify-docs`: 192 / 31 / 26 | ✅ |

Not predicted, and measured:

- **ex11-2's deadlock roster re-records** (`task@676` → `task@890`,
  `18:5` → `21:5`). The respelling moves byte offsets by one, and the
  re-measure comment this lane added to the solution file moves them
  further. A console transcript bound to byte offsets is bound to the
  file's comments too.
- **Export 370 → 374** corpus programs to wolf-lang's book corpus; the
  held-back count stays 125.
- **Ledger 136 → 132 open** (ch11 and ch14 close two rows each), 7
  waived unchanged.

## 3. The flip and the graduations, per machine

| id | at 0.2.16 / 0.1.38 | at 0.2.17 / 0.1.40 | fence now |
|---|---|---|---|
| ch14 `part-watch` | wolf `3`; lupin `E0301: nothing named `Proc`` (2) | both `3` | `run(exit=0, stdout="3")` |
| G1 ch11 `part-capability` (`s: Scope`) | lupin E0301 (2); **wolf run killed at 120 s (124)** — #431 | both `2740 words, three readers, one scope` | `run(…, stdout=…)` |
| G2 ch11 `part-refresher` | lupin E0301 (2); wolf runs, 3 lines | both, same 3 lines | `run(exit=0)` |
| G3 ex11-1 | lupin E0301 (2); **wolf run 124** — #431 | both `60` | `run(exit=0)` |
| G4 ch14 §14.1 (`Proc[int]`) | lupin E0301 (2); wolf runs, 3 lines | both, same 3 lines | `run(exit=0)` |
| G5 ch14 §14.3 (`Proc[int]`) | lupin E0301 (2); wolf runs, 3 lines | both, same 3 lines | `run(exit=0)` |

The finding the table carries: **#431 blocked two of the five, not
all of them.** At 0.2.16 the native hang hit only the programs whose
spawned closure captures from a caller's frame into a handed-over scope
(G1, G3); G2's refresher and both proc-handle programs already ran
compiled. For those three the interpreter's missing names
(wolf-interp#130) were the whole wall.

The checked machine (`wolf conform-run --json --checked`) answers
`unsupported` — «structured concurrency in checked execution (C1
deferred)» — on all six and on ex11-2 (`probes-new-checked.txt`). The
book's `run(…)` is a claim about lupin and `wolf run`, both of which
serve them; the checked lane's C1 deferral is pre-existing and not this
bump's.

## 4. The miss: ch28's reason

The prediction said ch28's six hold because the refused deep copy is a
*read*, `walk(n.left[0])`. The record says the span is [445,452] in
exercise 28-1 — `leaf(w)` in `(mut n.left).push(leaf(w))` — so the
refused copy is **push's plain copy** of a `Node` holding two
`List[Node]` (wolf-book#57's rule). The verdict held for a reason the
prediction did not name; the ch28 ledger now says which.

## 5. Drift found in passing, fixed

- `samples-pending.toml`'s ex5-8 owner named wolf-book#39 (closed
  2026-09-12); now #45, its open successor. The compiler's words for
  it are «member access on a value whose type is still being inferred»,
  not E0301 — identical on both pairs, so older than this bump.
- ex7-5's row said "wolfc leaves `channel` unresolved"; the compiler
  says «borrow expressions» at `resolve`, and lupin runs the program —
  identical on both pairs.
- `principles/exercises/ch11/EXERCISES.md`'s header said
  `wolf conform-run` reports `unsupported` for the concurrency surface;
  only the checked lane does.

## 6. The gate at head

`22a0d33`, new pair, kasumi: `samples` 506 / 503 passed / 3 pending /
0 failed / 0 flips; `samples --self-test` 13/13; `verify-docs` ok;
`ledger --check` ok (132 open, 0 unfiled); `backmatter --check` up to
date and matching the v0.2.17 sibling. CI on three hosts is the PR's.
