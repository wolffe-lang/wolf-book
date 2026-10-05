# bs62 — prediction (committed before the first change)

Base: wolf-book `e7772338`, `book/ch07.md` 1478 lines. Toolchain wolf
0.2.23 / lupin 0.1.46 from the release archives on kasumi. Every "measured"
below is a run on both machines (lupin, `wolf run`, `wolf conform-run
--json --checked`); logs in `bs62-evidence/`.

## 1. Sentences that are false or misleading about the default

Quoted at `e7772338`, with line numbers.

| # | where | sentence | why |
|---|---|---|---|
| F1 | §7.2 "Copying is a decision", 294 | "Assignment moves, and it moves whatever the value contains." | Only a non-`Copy` value moves (`[mem.tier0.move.1]`). `let c = p.lead` on a `str` copies. |
| F2 | same, 342-343 | "duplication is a decision made at the site where it happens, not a property of the type looked up somewhere else." | For a `Copy` type it *is* a property of the type: `let c = p.lead` duplicates with nothing written at the site (`[mem.tier0.move.3]`). |
| F3 | same, 344-346 | "a reader of `let b = a` knows `a` is finished." | False for `str`, `bool`, `char`, `byte`, every integer and float: measured, `let a = lead` then `print("{lead} {a}")` prints both. |
| F4 | same, 346-348 | "Scalars (the integers and floats the machine copies in a register anyway) are the exception the machine forces and the language admits: `let n = m` on an `int` leaves `m` alone." | The list is incomplete (`bool`, `char`, `byte`, `str`), and the reason given ("a register anyway") does not cover `str`, which is two words. |
| F5 | same, 348-349 | "Everything you define moves." | True for struct, tuple and enum (measured), but `distinct` is something you define and its status is not settled between the machines (see §5). Rewritten to name what moves. |
| F6 | §7.3, 427-429 | "Returns are the other direction and the other verb: `best` moves out to `title`, which is §7.2's rule applied at a function boundary." | `best` is a `str`: it is `Copy`, so the return copies it. The difference is invisible (the callee's binding dies), but the sentence states the wrong rule for the type on the page. |
| F7 | §7.6, 815 | "A `Tok` is two integers. It copies freely, it stores anywhere, it outlives anything" | A struct of two `int`s is not `Copy`: `let u = t` moves it (measured, E1001 / `trap(use-after-move)`; exercise 7-4 is the same fact). "Copies freely" is the spec's phrase for `str`, wrongly applied here. |
| F8 | §7.2 C++ callout, 352-354 | "wolf makes handing over the default and asks callers to opt in with a word." | True for every type you define, not for `Copy` types; qualified. |

Read and judged true, not edited: ch07 §7.2 149-152 (a *move* empties the
source path: it says "a move", and `move` on a `Copy` does empty it,
measured); §7.7 872-878 (the cost of a move); ch03 118-150 ("every value
has been a number or a string slice … copied wherever they go", then
"assignment hands the value over" about a `List`, which is true there);
ch04 498 ("A parameter borrows; a return moves": the return is the
callee's own value, which no reader can observe being copied rather than
moved; left as Part 1's simplification, the table states it exactly).

## 2. The new table (§7.1, a `###` subsection, no section renumbered)

STYLE.md forbids inventing section numbers, so the rule goes in §7.1 as
`### What each construct does`, after the two repairs and before
exercise 7-1. The table as it will be written:

| You write | The value is `Copy` | Any other value |
|---|---|---|
| `let b = a`, `b = a`, a field in a struct literal | copies; `a` stays live | moves; `a` is empty |
| `f(a)` (no mode) | `f` reads it, you keep `a` | `f` reads it, you keep `a` |
| `f(mut a)` | `f` writes it, you keep `a` | `f` writes it, you keep `a` |
| `f(take a)` | moves; `a` is empty | moves; `a` is empty |
| `return v`, a body's last value | leaves the function | leaves the function |
| `move a` | moves; `a` is empty | moves; `a` is empty |
| `copy a` | a second value | a second, independent value: deep for a struct or `List`; a `str` shares its bytes |
| `(mut xs).push(a)`, `xs[i] = a` | copies into the container | copies into the container; `take a` moves it |
| `a = …` after a move | `a` is live again | `a` is live again |

Then the `Copy` list, quoting `[mem.tier0.move.1]`, `[mem.tier0.move.3]`
and `[mem.region.escape]` ("A `str` is `Copy` — its two-word view copies
freely"): the integers, the floats, `bool`, `char`, `byte` and `str`; and
a struct, a tuple or an enum is never `Copy`, whatever its fields.
Handles are named where chapter 16 introduces them, not here.

## 3. The missing examples (named parts, so no `sN` id and no snapshot moves)

Inserting an un-named fence shifts every later `book/ch07/sN` and the
`diagnostic,from(…)` bindings to them, so each new program is a
`part(name)` fence (bs51's precedent for `part-movedleaf`):

- `part(copyread)`, §7.1's new subsection: `str`, `bool`, `char`, `int`
  read by plain `=`, both bindings printed. `run(exit=0)`, both machines.
- `part(handover)`, §7.2 "Copying is a decision": `let b = a` on a
  `Meta`, then only `b` read. `run(exit=0)`. The existing `s6` (the read
  of `a` refused, E1001) follows it unchanged.
- `part(packcopy)`, §7.2 after the re-initialization example: the
  maintainer's `pack.lu` shape exactly (`take p.lead`, `p.lead = "lin"`,
  `let c = p.lead`, both prints). `run(exit=0)`, `warns(W1003)` asserted
  (the compiler warns on `adopt` as it does on §7.2's `publish`).

Each gets a replayed `console` block. Samples predicted 509 → 512 (506 →
509 pass, 3 pending); console blocks +3.

## 4. Diagrams

A new figure fence, ```` ```memory,from(book/ch07/s3),line(5) ````, whose
body is the generated text rendering (so the markdown and the one-file
edition read as text, and `--check` holds it). `cargo xtask diagrams`
runs the bound program under the trace lupin, keeps the trace under
`snapshots/traces/`, and writes `book/diagrams/ch07/<id>-L<n>.svg`; the web
preprocessor turns the fence into an `<img>` of that SVG with the text as
its alt, and the PDF sets the same SVG with typst's `image`. `--check`
regenerates and diffs the text, the SVG and (when the trace lupin is
present) the trace; CI runs it.

Six diagrams, one per required case:

| case | program | line |
|---|---|---|
| `move` (replaces §7.2's two hand-drawn trees) | `s3` (doc.lu) | 5 `let who = move d.meta.author` |
| `take` | `part-packcopy` | 5 `let a = adopt(take p.lead)` |
| re-initialization | `part-packcopy` | 6 `p.lead = "lin"` |
| a `Copy` read | `part-packcopy` | 7 `let c = p.lead` |
| plain move of a struct | `part-handover` | 4 `let b = a` |
| `copy` | `s7` (meta.lu) | 4 `let b = copy a` |

Predicted trace content for the `Copy` read (is72's contract): before
line 7, `p.lead` live `"lin"` (re-initialized at 6), `p.tail` live
`"grace"`, `a` live `"ada"`; after, the same plus `c` live `"lin"`, a copy
of `p.lead`, and `p.lead` still live.

## 5. Ledger and filings

ch07's ledger gains one CLOSED row (the maintainer's find, these
sentences fixed): ch07 open 6 → 6, closed 7 → 8; book 128 open → 128,
106 closed → 107. One filing: the `distinct int` divergence (lupin
copies, wolf refuses E1001), on wolf-lang; no page claims anything
about `distinct`, so no ledger row.
