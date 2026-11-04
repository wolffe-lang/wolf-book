# bs74 — the enum primer: the prediction

_Written before the first change to a chapter, at wolf-book trunk `94bce4e`, with the
pins unchanged: wolf 0.2.36 (`5adf3d9`) and lupin 0.1.58 (`a5378e3`). The lane's
record will score every line below against the rig._

## What the book does with enums today

Measured by grep over `book/`, `principles/exercises/` and `samples/` (the census in
the lane's note):

- One enum is declared in the whole book, `enum Ordering { Less, Equal, Greater }`,
  first at §5.6 (`book/ch05.md` line 917), and its only explanation is one sentence
  after the block (line 953). It appears twice, both times as a trait's answer.
- No sample declares a variant with a payload, matches a variant by its bare name,
  is exhaustive over an enum, puts a method on one, prints one, compares two, or is
  generic over one. No exercise solution declares an enum at all.
- §3.4 teaches exhaustiveness over `int` and its ledger row says it would rather
  teach a closed set of names; the grammar refused the bare variant pattern then and
  admits it now (`[gram.pat.nullary]`).
- Error-row tags (ch06 §6.1) are the book's only other capitalized constructors, and
  they arrive with no enum beside them to tell them apart from.

## What wolf 0.2.36 and lupin 0.1.58 do

Black-box runs of the published aarch64-apple-darwin archives (digests equal to the
release's), one directory per program, `lupin`, `wolf run` and `wolf conform-run` on
each. The rows the primer rests on:

| construct | `wolf` | `lupin` |
|---|---|---|
| `enum Kind { Drink, Food, Gratuity }`, a value `Kind.Drink`, bare arms `Drink =>` | runs | runs, same bytes |
| a qualified arm `Kind.Drink =>` | runs | runs |
| a bare `Drink` in value position | E0301, "write `Kind.Drink`" | E0301, the same fix |
| a missing arm | E0801 naming the variant | runs; `unsupported`, exit 4, only if the arm is reached |
| payloads `Item(Kind, int)`, `Tip(int)`, `Void`; `_` in a payload and as an arm; a guard | runs | runs, same bytes |
| `impl Line { fn cents(self) -> int }`, `l.cents()` | runs | runs |
| `{k}` on a payload-less variant and on `Line.Item(Kind.Drink, 340)` | `Kind.Drink`, `Line.Item(Kind.Drink, 340)` | the same |
| `==` with no `Eq` anywhere | E0301 naming `Eq` | runs and answers (a divergence) |
| a local `trait Eq` and `impl Eq for Kind`, `==` and `!=` | runs, consults the impl | runs, consults the impl |
| `enum Maybe[T] { Just(T), Nothing }` | refused at resolve (the conservatism ledger) | runs |

Found on the way, each an issue draft in the hand-back and none worked around in
prose: the compiler treats a bare payload-less variant nested inside a variant's
payload pattern (`Item(Drink, _)`) as a binding and matches every value; the
compiler ICEs interpolating an enum value whose variants' payload types differ (`A(str)`
beside `B(int)`); lupin answers `==` on an enum that has no `impl Eq`, accepts an
`impl` naming a module nobody imported, evaluates `Line.Void.cents()` to `Line.Void`,
and accepts a bare payload variant (`Circle =>`) as a pattern where the compiler says
E0808.

## The change (pending the placement ruling, R-bs74-1)

The recommended placement is a new §3.5 directly after §3.4, before chapter 3's
closing batch, with two short pieces at the homes of the concepts they need:

1. `book/ch03.md` §3.5 "Enums": declare one, construct values, match bare and
   qualified, exhaustiveness with E0801 shown once, the bare-value E0301, payloads
   with `_` and a guard, a method in `impl`, printing a variant, `==` refused with
   E0301, the one sentence on rows against enums with its pointer to chapter 6, and
   the section's exercises.
2. `book/ch05.md` §5.3: a generic enum, one paragraph and one `lupin-run(…)` block,
   because the brackets are taught there.
3. `book/ch05.md` §5.6: an enum's `impl Eq`, one block beside `Row`'s, because traits
   are taught there.
4. `principles/TOC.md`: the row for §3.5.

Every new fence is a `part(name)` fence, so no positional sample id in either chapter
moves and no snapshot or `diagnostic,from(…)` reference elsewhere is touched.

## The rows

- Every new `run(…)` fence passes on both machines with the bytes in the table
  above; the `lupin-run(…)` fence passes on lupin and reports no FLIP (the
  compiler's verdict is `unsupported`).
- Every new `fail(…)` fence is the compiler's verdict with that code: E0301 twice,
  E0801 once. Three new snapshots, written by `cargo xtask samples --bless` and
  spliced into their `diagnostic,from(…)` blocks unedited.
- Every new console block is written as its `$` line alone, drifts on the first run,
  and is imported from that run's actual block by `record-consoles.py` (bs72's),
  never typed. The second run replays it byte-for-byte.
- Nothing else in the book moves: 0 flips, the pending count unchanged, no existing
  sample, console or snapshot changes.
- The counts, against r41q's green run at 0.2.36 (537 samples, 285 of them book
  blocks, 534 passed, 3 pending; 506 of 526 console blocks replayed, 249 of 251 on
  book pages): 545 samples, 293 book blocks, 542 passed, 3 pending, 0 failed, 0
  flips; 510 of 530 console blocks, 253 of 255 on book pages.
- Left for the turn after the ruling, named here so the record can say whether they
  moved: the exercise solutions and their index rows (`cargo xtask tiers`), Appendix
  C's sections column for E0301 and E0801 (`verify-docs`), the chapter ledgers' rows
  (`ledger --check`), the TOC delta, and the plant.
