# bs52 — results, scored against the prediction

The prediction is `docs/audit/bs52-prediction.md`, committed at
`cb80639` before any other file in this tree was edited. Every run
below is on kasumi (linux x86-64) against the release archives, wolf
0.2.16 (`wolf` member `b53b5328…`) and lupin 0.1.38 (`lupin` member
`f202d47f…`), copied from `~/lanes/s178/archives/` into
`~/lanes/bs52/archives/` and re-hashed (`84e30c05…`, `828b5c55…`).
Logs are under `~/lanes/bs52/logs/` and `~/lanes/bs52/evidence/`.

## 1. The renumbering map, as landed

| old | new | commit |
| --- | --- | --- |
| 7-6 … 7-11 | 7-7 … 7-12 | `3338438` |
| 7-17, 7-18 | 7-13, 7-14 | `3338438` |
| 7-12 … 7-16 | 7-15 … 7-19 | `3338438` |
| 4-3 | 7-6 | `49dd522` (moved), `eb8a63a` (replaced by the bounded buffer) |
| 4-4 | retired into 7-9 | `11dcd30` |
| — | 7-20 | `020f74e` |
| — | 7-21, 7-22, 7-23 | `a16cb21`, `aac1e33`, `af56c93` |

7-1 … 7-5 did not move; 7-5 is still held. §1 of `EXERCISES.md` records
the exception (`401ba67`), and the chapter-7 master's header carries the
map.

## 2. The prediction, scored

| § | predicted | measured | verdict |
| --- | --- | --- | --- |
| 2 | 4-3 → 7-6, held 7-5 keeps its number, fourteen numbers change | as predicted | ✅ |
| 3 | item 1 touches **25 paths** | **24** (13 edited, 1 snapshot renamed, 10 solution files renamed). The 25th, `principles/exercises/appx/EXERCISES.md`, names 4-4, so it moved with 4-4's retirement in item 3 (`11dcd30`), not in the renumbering | ❌ off by one, the reason named |
| 3 | only the renamed snapshot moves in item 1 | `ch07__ex7-7.txt` → `ch07__ex7-8.txt`, nothing else | ✅ |
| 3 | `samples-declined.toml` moves one row | two rows: the ch07 REPL block, and the exemplar's `lupin eval` block (`EXERCISES.md:368` → `373`), which §1's new sentence pushed down | ⚠️ path right, row count off by one |
| 4 | no sample changes verdict | 0 failed, 0 flips at every gated step | ✅ |
| 4 | samples 501 → 506, passed 498 → 503, pending 3 | 506 / 503 / 3 | ✅ |
| 4 | corpus count 260 → 265 | 265 | ✅ |
| 4 | snapshots 93 → 94 | 94 | ✅ |
| 4 | declared console rows stay 18, none newly declined | 18; replayed 474/494 → 481/501 | ✅ |
| 4 | index 341 → 344, printed 292 → 295; ch04 11 → 9, ch07 18 → 23 | exact | ✅ |
| 4 | tiers 193 / 31 / 25 (lupin / wolf / wolf + lupin) | exact | ✅ |
| 4 | spread fingers 61, comprehension 153, extension 79, spelunking 28, design 37 | exact | ✅ |
| 4 | `ledger --check` unchanged at 135 open | **136 open** (129 filed, **7 waived**) | ❌ |
| 5 | prompt audit at head: "23 printed, 0 differ" | **22 printed**, 0 differ | ⚠️ the zero holds; 23 counted the held 7-5 |

The ledger miss is the interesting one. 7-23's refusal is the
compiler's, and lupin runs the same program to exit 0, so the chapter
owes a row saying so, the way §7.8's E0810 block has one. The prediction
treated a new `fail(…)` solution as ledger-neutral; a `fail(…)` the
other machine runs is not. The row is waived to wolf-interp#115, whose
closing ruling (s175) is that the read-parameter rows are the
interpreter's conservatism class.

Not predicted, and worth saying: **none of chapter 7's corpus travels
to wolf-lang's corpus export.** The export is held back by directory,
and `ch07/ex7-4.lu` is `lupin-run(exit=trap(use-after-move))`, so the
whole `ch07/` directory stays home (118 → 125 files held back, the
seven new ones). The export went 372 → 370, the two lost being
`ch04/ex4-3.lu` and `ch04/ex4-4.lu`, which used to travel from `ch04/`.
Pre-existing, and the new exercises inherit it.

## 3. Each new or rewritten solution, run on both machines

From `~/lanes/bs52/evidence/<file>.log`, each file copied alone into a
fresh directory; digests are of the committed files.

| file | sha256 (16) | lupin | `wolf run` | `wolf conform-run` |
| --- | --- | --- | --- | --- |
| `ex7-6.lu` | `da64681146f629c4` | exit 0, `1 2 -1` / `3 of 4: 10 20 30` | exit 0, same | `wir`, `pass` |
| `ex7-9.lu` | `f1d00105179a73bc` | exit 0, `16 18` | exit 0, same | `wir`, `pass` |
| `ex7-9b.lu` | `1639f122499199e3` | exit 3, `trap(exclusivity)` at 17:17 | exit 2, two E1002, no warning | `mem`, `fail(E1002)` |
| `ex7-20.lu` | `57fc95df0c3fa0fe` | exit 0, six lines | exit 0, same | `wir`, `pass` |
| `ex7-21.lu` | `efef0f24d7e2f9c3` | exit 0, `2 -1 1 moves` | exit 0, same | `wir`, `pass` |
| `ex7-22.lu` | `b1f76229f8483ae1` | exit 0, `3: regions moves tokens` | exit 0, same | `wir`, `pass` |
| `ex7-23.lu` | `6a3b9d215e0fa8b2` | **exit 0**, `moves, and the shelf still holds 3` | **exit 2, E1002** (the lend rule) | `mem`, `fail(E1002)` |
| `ex7-23b.lu` | `586aeb54f2ce3f32` | exit 0, same line | exit 0, same | `wir`, `pass` |

The claims the solutions make beyond their own output were measured
before they were written, under `~/lanes/bs52/proto/`: `copy b.items`
in 7-6 prints the same; a `Doc` of a `str` and an `int` is not refused
by the lend rule (`lend1.lu`, `pass`) and one with `tags: List[str]` is
(`lend2.lu`, `fail(E1002)`); `take a[0]` from a read-mode list is E1014
on wolf and `trap(exclusivity)` on lupin (`take1.lu`) and runs from a
`take` list (`take2.lu`); a push without `take` prints the same in 7-21
and 7-22; a read of `b` after `take b` is E1001 and
`trap(use-after-move)` (`merge2.lu`); `List.remove` is std and declined
by both machines with no std root (`rm1.lu`).

## 4. The prompt audit (item 6)

Every printed chapter-7 stem against its solutions-page prompt,
whitespace-normalised, a fence's directive ignored (build metadata, not
prompt). Trunk numbering in the first column.

| trunk | head | at trunk | fixed by | how |
| --- | --- | --- | --- | --- |
| 7-1 | 7-1 | same | — | — |
| 7-2 | 7-2 | differs | `30c5748` | the master asks in the chapter's words: struct declarations, not a `den` "below" |
| 7-3 | 7-3 | differs (tag) | `e27b922` | the chapter prints the vein, `(break-it-on-purpose)`, as the master and the index tag it |
| 7-4 | 7-4 | differs | `32c6137` | the master gains the second question and its answer (what the copy costs) |
| 4-3 | 7-6 | differs | `eb8a63a` | replaced whole (item 2) |
| 7-6 | 7-7 | same | — | — |
| 4-4 | — | differs | `11dcd30` | retired (item 3) |
| 7-7 | 7-8 | differs | `f5a9850` | the master asks in the chapter's words; the program moves into the answer |
| 7-8 | 7-9 | same | `11dcd30` | rewritten in both places together (item 3) |
| 7-9 | 7-10 | differs | `cd762d6` | two cross-references the reader never saw leave the master |
| 7-10 | 7-11 | differs | `b068728` | the master stops naming a region the chapter has not taught |
| 7-11 | 7-12 | same | — | — |
| 7-17 | 7-13 | same | — | — |
| 7-18 | 7-14 | same | — | — |
| 7-12 | 7-15 | differs | `101daae` | the master asks the chapter's ownership question and answers it |
| 7-13 | 7-16 | differs | `1ba72fd` | wording only |
| 7-14 … 7-16 | 7-17 … 7-19 | same | — | — |
| — | 7-20 … 7-23 | same | written once | — |

**At trunk: 19 printed, 10 differ. At head: 22 printed, 0 differ.**
