# Appendix C — Diagnostics

Every diagnostic the book shows, with the one-line summary the compiler's
own catalog carries for it. The catalog holds 169 codes; these 54 are the
ones a page in this edition names. A code is stable: it identifies a
rule, not a message, and `wolf --explain E1001` prints the whole entry
for any of them (the summary, the reasoning, and the fix) whether or
not this book shows it.

The *Shown by* column names which implementation printed the text on the
page — or, for a code a section names as a rule without printing its
output, whose catalog the rule is quoted from. Both tools use the same
codes for the same rules, so a code the book shows under one prompt is
the same rule under the other. Where the column says `both`, some page
shows both readings of it side by side.

| Code | What it says | Shown by | Sections |
|------|--------------|----------|----------|
| `E0102` | unterminated string literal or interpolation | wolf | 2.2 |
| `E0103` | a `"""` delimiter shares its line with text | wolf | 2.2 |
| `E0104` | a multiline string line sits left of the margin | wolf | 2.2 |
| `E0105` | margin tabs and spaces do not match the closing `"""` | wolf | 2.2 |
| `E0107` | a stray character that fits no token | wolf | 2.3 |
| `E0110` | a malformed `char` literal | wolf | 2.4 |
| `E0201` | the parser expected a different token or construct here | both | 1.5, 4.1, 12.4, 14.1 |
| `E0202` | an opening delimiter is never closed | both | 1.5, Notation |
| `E0203` | expected a declaration at the top level | wolf | Solutions |
| `E0301` | nothing with this name is in scope | wolf | 10.1 |
| `E0302` | the same name is defined twice in one module | lupin | 22.1 |
| `E0303` | modules import each other in a cycle | lupin | 22.2 |
| `E0304` | the item exists but is not visible from here | lupin | 22.1 |
| `E0305` | this import is never used | lupin | 22.1 |
| `E0401` | the types do not match | wolf | 3.2, 4.1 |
| `E0410` | a `let` binding cannot be assigned again | wolf | 3.1 |
| `E0412` | this format spec is malformed | wolf | 18.3 |
| `E0501` | the generic body uses something its bounds do not provide | wolf | 5.3 |
| `E0602` | the error row does not include this tag | wolf | 26.4, 26.5 |
| `E0701` | comptime code reached for ambient IO | wolf | 18.4, 24.2 |
| `E0702` | comptime evaluation ran out of fuel | wolf | 18.4 |
| `E0703` | comptime evaluation exceeded its heap budget | wolf | 18.4 |
| `E0704` | comptime evaluation recursed too deeply | wolf | 18.4 |
| `E0705` | this value is not comptime-known | wolf | 18.1 |
| `E0706` | comptime arithmetic faulted | wolf | 18.1 |
| `E0707` | const-generic equality needs a witness | wolf | 18.3 |
| `E0708` | layout is unresolved until codegen | wolf | Solutions |
| `E0709` | invalid comptime budget attribute | wolf | 18.4 |
| `E0710` | a comptime assertion failed | wolf | 18.1, 18.2, 22.3 |
| `E0801` | this `match` does not cover every case | wolf | 3.4 |
| `E1001` | this value was moved away (or never given one) before this use | wolf | 3.1, 7.1, 7.2, 7.6, 7.7, 9.8, Notation |
| `E1002` | this needs exclusive access, but the value is in use here | wolf | 7.5 |
| `E1005` | the region is open here, so its handle cannot move or freeze | wolf | 8.3 |
| `E1006` | this type's `shared` references form a strong cycle | wolf | 8.5, 8.7 |
| `E1007` | the argument's mode does not match the parameter's | wolf | 7.4 |
| `E1008` | the method touches a field outside its declared view | wolf | 7.5 |
| `E1010` | the value's region is freed while the value is still needed | wolf | 8.2, 8.4, 9.8 |
| `E1011` | this would open a region while a region that contains it is open | wolf | 8.6 |
| `E1012` | frozen data cannot be written | wolf | 8.5, 8.8, 30.4, 30.5 |
| `E1101` | a task may not mutate state it captured from the enclosing function | both | 13.2 |
| `E1102` | this channel's payload type is not sendable | both | 13.2 |
| `E1301` | this raw-tier operation needs an `unsafe` block | wolf | 9.2 |
| `E1302` | a raw pointer type cannot cross this boundary | wolf | 9.2, 9.5, 32.2 |
| `E1303` | this module holds `#[trusted]` code the manifest does not declare | wolf | 9.7, 32.3 |
| `E1305` | this door needs a region and a raw pointer, in that order | wolf | 9.4 |
| `E1401` | undefined behavior detected by the checked-build UB machine | both | 9.3 |
| `E1503` | the manifest declares a build-time script hook — wolf has none, ever | wolf | 24.2 |
| `E1504` | this package uses a capability its manifest does not declare | wolf | 24.3 |
| `E1506` | a dependency's content hash does not match wolf.sum | wolf | 23.3 |
| `W0313` | this `pub` item has no doc comment | wolf | 22.2 |
| `W0603` | this row tag's case contradicts its payload | wolf | 27.5 |
| `W1003` | this `take` parameter is returned unchanged | wolf | 7.2, Notation |
| `W1101` | this write stays inside the task | wolf | 13.2 |
| `W1102` | the closure captured this value before it changed | wolf | Solutions |

Two conventions run through that table. `E`-numbered codes stop
the build or the run; `W`-numbered codes are advice, and the artifact is
produced anyway (§27.5 shows a binary built over two warnings). And the
families are blocked by hundreds: `E01xx` is the lexer, `E02xx` the
parser, `E03xx` resolution and modules, `E04xx` types, `E05xx` generics,
`E06xx` error rows, `E07xx` comptime, `E08xx` exhaustiveness, `E10xx`
memory, `E11xx` concurrency, `E13xx` the unsafe tier, `E15xx` packages.

Warnings the book does not print but the compiler emits often enough to
recognize: `W1001` (a region that never allocates) and `W1002` (a `mut`
binding the body never writes).
