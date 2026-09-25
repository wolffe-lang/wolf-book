# bs52 — the prediction, committed before the first edit

Wave 47, subwave 6. Written 2026-09-24 against wolf-book trunk
`e0a44a3`, before any chapter, master, index, solution or snapshot in
this repository was edited. The pins do not move: wolf 0.2.16
(`93a5fe5`) and lupin 0.1.38 (`ba357aa`, pin `2e4ca76`), bs51's pair,
taken on kasumi from the release archives (`84e30c05…` and
`828b5c55…`, members `wolf b53b5328…` and `lupin f202d47f…`, the same
digests bs51 recorded). Labels: MEASURED is a number read off a run at
trunk; PREDICTED is a claim about the head, with the figure that
falsifies it.

## 0. The control, measured at trunk

The whole gate, run on kasumi against the archives at `e0a44a3`
(`~/lanes/bs52/logs/control/`):

| gate | trunk |
| --- | --- |
| `samples` | 501 samples (244 corpus roots + 16 members, 257 book blocks); 498 passed, 3 pending, 0 failed, 0 flips |
| console replay | 474 of 494; book 226 of 228, corpus 248 of 266; 18 declared rows |
| `samples --self-test` | 13/13 |
| `verify-docs` | ok; corpus count 260; index 341 exercises (292 printed) |
| `ledger --check` | 135 open (129 filed, 6 waived, 0 unfiled), 95 closed |
| `backmatter --check`, `contrast` | ok |
| snapshots | 93 files under `snapshots/diagnostics/` |

## 1. Drift against the contract's Inputs line, reported not absorbed

1. **"No 7-5" is false.** Exercise 7-5 exists: a *held* stem in §7.3
   of the master (`principles/exercises/ch07/EXERCISES.md:192`), with a
   solution file (`ch07/ex7-5.lu`, `fail(E1003)`), a `pending` row in
   the index, a row in `samples-pending.toml` and one in
   `EXERCISES-PENDING.md`. It is not printed because E1003 reaches no
   verdict on either machine (ch07's ledger, wolf-lang#153). The number
   is not a hole; it is a held exercise's. **So 4-3 cannot become 7-5**
   without renumbering a held exercise, and the map below gives it 7-6.
2. **The placement was a decision, not an oversight.** The wave-47 row
   says the two exercises were "never renumbered". They were kept on
   purpose, and three places say so: `book/ch04.md:571` ("They keep
   their numbers"), `principles/EXERCISES-INDEX.md:96` ("the numbers are
   stable (EXERCISES.md §1)"), and ch04's ledger contract-delta row.
   `EXERCISES.md` §1 reads "Numbers are stable once published — a
   retired exercise leaves a tombstone, not a renumbering". It does
   **not** say "in order of appearance"; the contract's parenthetical is
   a paraphrase the text does not carry. This lane follows the contract
   and renumbers chapter 7; §1 gains one sentence saying so, and the old
   numbers stay findable (tombstones in chapter 4's page, the chapter-7
   master's header, and the index).
3. **A second out-of-order pair the contract does not name.** 7-17 and
   7-18 are printed in §7.8, before 7-12 … 7-16 in the chapter batch;
   they are chapter 5's 5-9 and 5-10, moved at bs42 and numbered after
   the batch. In-order numbering moves them too.
4. **"The index regenerated": nothing regenerates it.**
   `EXERCISES-INDEX.md` is hand-kept (wolf-book#40) and `verify-docs`
   holds its arithmetic both ways. The generated page is
   `book/back/solutions.md` (`cargo xtask backmatter`).
5. **Item 5 already exists as 7-13.** 7-13 extends 7-12 into a printing
   diff by walking the table backward. The new exercise does not repeat
   it: it returns the diff as a value (`List[Edit]`), which is the step
   where copy-or-move becomes a decision.
6. **4-3's grep half duplicates 7-6's.** Both ask for "the one search";
   7-6's solution says "from exercise 4-3, now stated as a rule". The
   replacement drops it.
7. **The lend rule does not fire on the chapter's `Doc`.** `longest(s:
   Shelf) -> Doc` returning `s.docs[best]` with `struct Doc { title:
   str, words: int }` is `pass` on the compiler: a struct of a `str` and
   an `int` cannot reach shared storage, which `[mem.tier0.mode.read]`
   exempts (`wolf --explain E1002`, last paragraph). With `tags:
   List[str]` added it is `fail(E1002)` at `phase_reached: "mem"`.
   MEASURED on kasumi, both archives (`~/lanes/bs52/proto/lend1.lu`,
   `lend2.lu`). **lupin 0.1.38 runs both to exit 0** and prints `moves
   2`: the rows are the conservatism class (wolf-interp#115, closed by
   s175 — lupin copies at the crossing and has no static rung by its
   approximation contract). The shelf set's `Doc` carries `tags`.
8. **`List.remove` is std.** Both machines decline it with no std root
   (lupin `unsupported` at `[type.method.root]`, wolf E0301). Remove-by-
   title rebuilds the list and moves the survivors across.
9. Confirmed as written: trunk `e0a44a3`; pins; 4-3's two prompts
   disagree; 4-4's signatures are absent from the stem and its
   transcript leads with W1002 (`snapshots/diagnostics/ch04__ex4-4.txt`);
   7-8 is 4-4 done properly; the contract's count of 20 is 19 printed
   plus the held 7-5.

## 2. The renumbering map (PREDICTED final state)

Order of appearance in the master, held 7-5 included:

| old | new | where | what happens |
| --- | --- | --- | --- |
| 7-1 … 7-5 | 7-1 … 7-5 | §7.1–§7.3 | unchanged |
| **4-3** | **7-6** | §7.4 | moves under `ch07/`, then replaced by the bounded buffer (item 2) |
| 7-6 | 7-7 | §7.4 | `swap` |
| **4-4** | — | §7.5 | retired into 7-9 (item 3); solution and snapshot deleted |
| 7-7 | 7-8 | §7.5 | `bump2` |
| 7-8 | 7-9 | §7.5 | the four shapes, gains the spelunking half |
| 7-9 | 7-10 | §7.6 | `--explain E1001` |
| 7-10 | 7-11 | §7.6 | design, tokenizer |
| 7-11 | 7-12 | §7.7 | REPL |
| 7-17 | 7-13 | §7.8 | third `Draw` shape |
| 7-18 | 7-14 | §7.8 | design, cast-a-binding |
| 7-12 | 7-15 | batch | LCS |
| 7-13 | 7-16 | batch | printing diff |
| 7-14 | 7-17 | batch | plane geometry |
| 7-15 | 7-18 | batch | consuming vs lending |
| 7-16 | 7-19 | batch | geometry by match |
| — | 7-20 | batch | line diff as a value (item 5) |
| — | 7-21 | batch | shelf: remove by title, `mut` (item 4) |
| — | 7-22 | batch | shelf: merge, `take` (item 4) |
| — | 7-23 | batch | shelf: the longest document, the lend rule (item 4) |

Fourteen existing numbers change (4-3 plus thirteen chapter-7 ids);
4-4 retires; four exercises are new. Item 5 is committed before item 4
so the numbers are assigned in order and nothing is renumbered twice.

## 3. Files the renumbering chain touches (item 1, PREDICTED)

**25 paths**: 14 edited in place, 1 snapshot renamed, 10 solution files
renamed.

- Edited: `book/ch07.md`, `book/ch04.md` (the tombstone sentence and
  the ledger row), `book/ch01.md` and `book/ch05.md` (ledger comments
  naming 7-11 and 4-3), `book/back/solutions.md` (regenerated),
  `principles/EXERCISES.md` (§1's sentence, §5's 4-3 block leaves, §7's
  stats), `principles/EXERCISES-INDEX.md`, `principles/EXERCISES-PENDING.md`
  (the history paragraph naming 7-14/7-16), `principles/TOC.md`
  (chapter 4's exercise line), `principles/exercises/ch04/EXERCISES.md`,
  `principles/exercises/ch05/EXERCISES.md` (the 5-9/5-10 tombstone),
  `principles/exercises/ch07/EXERCISES.md`,
  `principles/exercises/appx/EXERCISES.md` (B-5 names 4-4),
  `samples-declined.toml` (the ch07 REPL block's line number moves).
- Snapshot: `ch07__ex7-7.txt` → `ch07__ex7-8.txt`, its two `--> ./ex7-7.lu`
  lines rewritten. **Only renamed snapshots move in item 1** — none of
  the eight `book__ch07__*` snapshots, and not `ch07__ex7-3.txt`.
- Renamed: `ch04/ex4-3.lu` → `ch07/ex7-6.lu`; in `ch07/`, 6→7, 7→8,
  8→9, 17→13, 12→15, 13→16, 14→17, 15→18, 16→19.
- Left alone on purpose, as history: `CHANGELOG.md`'s old entries,
  `audit/tells/ch07.md`, `docs/audit/promissory-prose-audit.md`,
  `wolf-toolchain.toml`'s comments, `principles/TOC.md`'s bs42 note, and
  `xtask/src/verify.rs`'s doc comments and one unit test that quote
  `ch04/ex4-3.lu` as a string (the test parses a label; it opens no file).

Falsified by: a 26th path the chain has to touch, or a book-block
snapshot that moves.

## 4. The gates at head (PREDICTED)

- **No sample changes verdict.** Every renamed file keeps its directive
  and its output; the only transcript bytes that move are file names.
  Falsified by any `failed` or `flip` line in `samples`.
- **Samples 501 → 506**: book blocks stay 257 (every new stem is prose,
  no fence); corpus roots 244 → 249 (−`ex4-3`, −`ex4-4`, +`ex7-6`,
  +`ex7-9b`, +`ex7-20`, +`ex7-21`, +`ex7-22`, +`ex7-23`, +`ex7-23b`).
  Passed 498 → 503; pending stays 3.
- **Corpus count 260 → 265** (the `verify-docs` figure and
  `EXERCISES-PENDING.md`'s claim).
- **Snapshots 93 → 94**: −`ch04__ex4-4`, +`ch07__ex7-9b`, +`ch07__ex7-23`,
  and the one rename.
- **Declared console rows stay 18**; console blocks replayed rise and
  none is newly declined.
- **The index recounts 341 → 344** (292 → 295 printed): ch04 11 → 9,
  ch07 18 → 23. Tiers: run (lupin) 195 → 193, run (wolf) 32 → 31,
  run (wolf + lupin) 19 → 25, the rest unchanged. Spread: fingers
  62 → 61, comprehension 153, extension 76 → 79, spelunking 27 → 28,
  design 37.
- **`ledger --check`** unchanged at 135 open / 95 closed: no row
  opens or closes, one row's text changes.
- **CI green on all three hosts at the head sha.**

## 5. The prompt audit (item 6, MEASURED at trunk)

Every printed chapter-7 stem diffed against its solutions-page prompt
(whitespace-normalised, `scratchpad` script `audit.py`): **19 printed,
10 differ** — 7-2, 7-3, 7-4, 4-3, 4-4, 7-7, 7-9, 7-10, 7-12, 7-13
(old numbers). Items 2 and 3 rewrite 4-3 and 4-4 whole; item 6 fixes the
other **8**. PREDICTED at head: **23 printed, 0 differ.**
