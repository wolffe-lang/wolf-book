# Chapter 18 — Comptime: one tier, no macros: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the
compiler it pins. The comptime engine is wolf's, and every verdict
below is a real run of it: the sandbox refusals, the budget meters,
the comptime-known rule, checked arithmetic at compile time, and the
reflected witnesses all evaluate today. The compiler also executes a
fold's *result* now — `wolf run` builds and runs a module holding a
`comptime fn`, which is the `wolf-run(…)` directive lane 18-3, 18-5
and 18-11 moved to when their old blocker retired (the pending doc's
retired-entries table has the accounting). lupin still declines
`comptime fn` by design, exit 4, so this chapter is the compiler's
territory end to end.

## §18.1 — Wolf at compile time

**Exercise 18-1** *(comprehension · wolf)*. One binding keeps this
program out of compile time:

```wolf
comptime fn double(n: int) -> int {
    n + n
}
fn main() -> !int {
    let x = 21
    const Y = double(x)
    if Y == 42 { 0 } else { 1 }
}
```

Predict the compiler's verdict, and name the one-character change that
fixes the program. What may an argument to a `comptime fn` be?

Solution: `let x` is a runtime value, and a `comptime fn` cannot
receive one. The fix is `const x = 21`. Arguments must be literals,
`const`s, types, or results of other comptime calls; the diagnostic
recites the list:

```console
$ wolf conform-run ./ex18-1.lu
error[E0705]: `x` is a runtime value, so this cannot evaluate at compile time
 --> ./ex18-1.lu:9:22
  |
9 |     const Y = double(x)
  |               --------- while evaluating `main`, entered here
  |                      ^ must be comptime-known
  |
  = note: a `comptime fn` runs during compilation: every argument must be a literal, a `const`, a
    type, or the result of another comptime call.
```

**Exercise 18-2** *(comprehension · wolf)*. Chapter 3 taught you what
`2147483647 + 1` does at runtime. Predict what it does inside a
`comptime fn`, and predict the decision the diagnostic cites:

```wolf
comptime fn brim() -> i32 {
    let big: i32 = 2147483647
    big + 1
}
```

Solution: the same rule, moved earlier: what would trap at runtime is
a compile error at comptime, and the diagnostic cites X3, the one
checked-arithmetic semantics for every profile and now every phase:

```console
$ wolf conform-run ./ex18-2.lu
error[E0706]: this `+` on `i32` faults at compile time: 2147483647 + 1 leaves `i32`'s range
 --> ./ex18-2.lu:6:5
  |
6 |     big + 1
  |     ^^^^^^^ checked arithmetic, comptime included
...
9 |     const B = brim()
  |               ------ while evaluating `brim`, entered here
  |               ------ while evaluating `main`, entered here
  |
  = note: checked arithmetic has one semantics everywhere (X3): what would trap at runtime is an
    error at comptime — intended wraparound is spelled `wrapping[T]`, never a mode.
```

**Exercise 18-3** *(fingers · wolf)*. Write `sum_squares(n)` as
a `comptime fn` and bind `const T = sum_squares(9)`. The folded value
is 285, and the program prints it having computed nothing at runtime.

Solution. `ch18/ex18-3.lu`, on the `wolf-run(…)` lane because the
compiler both folds the table and runs the result; lupin declines a
`comptime fn` by design (exit 4, outside scope stated rather than
rejected), which keeps this exercise single-lane on purpose:

```console
$ wolf run ex18-3.lu
285
```

## §18.2 — Types as values

**Exercise 18-4** *(comprehension · wolf)*. `size_of(Vec2)` for a
struct of two `f64` fields is 16 on every target wolf supports. Predict
the verdict of `const S = size_of(Vec2)` anyway, and then explain why a
number that obvious is refused at comptime.

Solution: E0708. Layout belongs to the code generator, and the
checker refuses to promise a number another phase owns. The obviousness
is the trap: field reordering, padding, and target ABIs make aggregate
layout a codegen fact, and a comptime that guessed would have to be
right forever:

```console
$ wolf conform-run ./ex18-4.lu
error[E0708]: the size of `Vec2` is not resolved until codegen lays it out
 --> ./ex18-4.lu:9:15
  |
9 |     const S = size_of(Vec2)
  |               ^^^^^^^^^^^^^ unresolved until codegen
  |               ------------- while evaluating `main`, entered here
  |
  = note: layout (sizes, offsets) is decided by the code generator, not the type checker; comptime
    can answer for fixed-width primitives today, but not yet for aggregates.
```

**Exercise 18-5** *(extension · wolf)*. Write `field_count(T: type)`
using `typeinfo`, and apply it to a struct of your own. Predict what
the program prints for a three-field struct.

Solution. `ch18/ex18-5.lu`, on the `wolf-run(…)` lane. The signature
`fn field_count(T: type) -> int` is the section's whole point in four
tokens: a type arrives as an argument, like any other value. The
reflected count folds into the `const` and the program prints it:

```console
$ wolf run ex18-5.lu
3
```

## §18.3 — Where comptime already touched your code

**Exercise 18-6** *(spelunking · wolf)*. Run `wolf --explain E0701`
and read the entry in full. It names two distinct reasons a comptime
capability can be refused. Name both, and sort these refusals under
them: a clock read, a network fetch, an environment variable.

Solution: the two reasons are *confinement* (compiling a package must
never act on or read the machine that compiles it) and *determinism*
(the same program and target must produce bit-identical comptime
results on every host). The clock is determinism: two identical
builds must not observe different times. The network fetch is
confinement: the entry's own example is that `wolf add` must never
mean arbitrary code runs with your credentials. The environment
variable is both, and the catalog files it under confinement: it reads
the compiling machine, and it also varies host to host. From the real
entry:

```console
$ wolf --explain E0701
E0701: comptime code reached for ambient IO

Comptime evaluation is hermetically sandboxed (D33): no filesystem, no
network, no environment variables, no clock, no randomness, no FFI —
the intrinsics available at compile time are an explicit allowlist,
and nothing ambient is on it. Each refusal names its category and its
reason: confinement (compiling a package must never act on or read
the machine that compiles it — `wolf add` must never mean arbitrary
code runs with your credentials) or determinism (the same program and
target must produce bit-identical comptime results on every host).
Compute the value at runtime instead; file contents belong in
*declared build inputs* through the package manifest, never in an
evaluator capability.
```

**Exercise 18-13** *(spelunking · wolf)*. Misspell a format spec on
purpose — `{total:>9.2z}` — and read the grammar out of the error.
From the diagnostic alone: which one letter fixes this spec for a
money column, and which *two* letters would print the same number in
bases the chapter has not used yet?

Solution. `ch18/ex18-13.lu` (broken on purpose):

```console
$ wolf conform-run ./ex18-13.lu
error[E0412]: `z` has no place in a format spec — the grammar is `[[fill]align][+][0][width][.precision][type]` with type one of `b o x X e E f`
 --> ./ex18-13.lu:7:18
  |
7 |     print("{total:>9.2z}")
  |                  ^^^^^^ in this format spec
  |
```

The fix is `f`: fixed-point, which with `.2` is the money column
(`{total:>9.2f}`). The two unused letters are `b` and `o` — binary
and octal — sitting in the grammar beside the `x`/`X` hex the string
chapter did print. The diagnostic *is* the section's claim performed:
the f-string compiled to checked calls, so a bad spec is a compile
error with the whole grammar in it, not a runtime surprise — and the
grammar line answers questions the chapter never got to.

**Exercise 18-14** *(comprehension · wolf)*. Four `Buf[…]` parameter
and return pairs: `Buf[N + 1]` with `Buf[1 + N]`; `Buf[N + 2 - 1]`
with `Buf[1 + N]`; `Buf[2 + 2]` with `Buf[4]`; `Buf[N * 2]` with
`Buf[2 * N]`. Predict which pairs the checker equates on its own and
which one needs a witness, then check the odd one out and read which
of the three steps the note says it fell past.

Solution. `ch18/ex18-14.lu` (the `*` pair, broken on purpose; the
first three are the chapter's own `shuffle`/`widen`/`closed`, all
accepted):

```console
$ wolf conform-run ./ex18-14.lu
error[E0707]: `Buf[N * 2]` and `Buf[2 * N]` may be equal, but proving it needs a witness
 --> ./ex18-14.lu:6:51
  |
6 | fn double[N: type](b: Buf[N * 2]) -> Buf[2 * N] { b }
  |                                   ------------- the return type is declared here
  |                                                   ^ these const expressions differ beyond linear arithmetic
  |
  = note: const-expression equality is decided in three steps, and the line is fixed: (1) closed
    expressions evaluate and compare by value; (2) `+`/`-` arithmetic over generic
    parameters compares by ring normalization, so `N + 1` equals `1 + N`; (3) anything
    beyond — `*`, `/`, `%`, shifts, bit operators — needs an explicit witness. This pair
    sits at step 3.
  = note: state the equality where the reader can see it: a comptime `assert` on the sizes
    involved, or rewrite both spellings into the same `+`/`-` form.
```

Closed values (step 1) equate `2 + 2` with `4`; ring normalization
(step 2) equates both `+`-shaped pairs. The `*` pair is mathematics
any reader can do and the checker *will not*, because the line
between "normalized" and "proved" is fixed where the chapter said it
is: multiplication is step 3, witness territory, however trivial the
instance. A checker that did easy multiplications would have a
boundary nobody could state.

## §18.4 — What it refuses to do

**Exercise 18-7** *(comprehension · wolf)*. Five expression tiles.
Sort each onto the comptime side of the boundary or the runtime side
before running anything: `6 * 7`; a function from a type to a type; a
file read; a clock read; a network fetch. Then check the three you
sorted as refused, with three one-line programs. Do the three
diagnostics give the same reason?

Solution: arithmetic and type-to-type functions are admitted: pure
computation over values the compiler already holds. The file read, the
clock read, and the network fetch are refused, all as E0701, but not
for one reason; each refusal names its own:

```console
$ wolf conform-run ./ex18-7a.lu
error[E0701]: `read_text` reaches the filesystem, which comptime code can never touch
  = note: why it is refused — confinement: a build must not read the machine it runs on — and the
    same source would compile differently on different machines.
$ wolf conform-run ./ex18-7b.lu
error[E0701]: `clock_ms` reaches the clock, which comptime code can never touch
  = note: why it is refused — determinism: two identical builds must not observe different times.
$ wolf conform-run ./ex18-7c.lu
error[E0701]: `net_fetch` reaches the network, which comptime code can never touch
  = note: why it is refused — confinement: `wolf add` must never mean arbitrary code talks to the
    network with your credentials.
```

(Each run also prints the span rendering and the shared hermetic-
sandbox note; the lines above are the ones that differ. The full
outputs are in `ex18-7a.lu` through `ex18-7c.lu`'s runs.)

**Exercise 18-8** *(comprehension · wolf)*. A reader decides budgets
are noise and writes `#[budget(fuel = 0)]` to turn the meter off.
Predict what the compiler does with a *trivial* call under that
attribute: a `comptime fn` that returns `10` and computes nothing.

Solution: the rejection is about the attribute, not the workload.
Budgets are raised, never removed; there is no spelling that disables
one, and the trivial body never gets a chance to demonstrate its
innocence:

```console
$ wolf conform-run ./ex18-8.lu
error[E0709]: a comptime budget cannot be turned off — `fuel = 0` would disable the limit
 --> ./ex18-8.lu:8:14
  |
8 |     #[budget(fuel = 0)]
  |              ^^^^^^^^ budgets are raised, never removed
  |
  = note: the sandbox guarantee (D33) includes bounded evaluation: every budget has a default, a
    per-site override, and a hard ceiling — there is no spelling that removes one.
```

**Exercise 18-9** *(comprehension · wolf)*. Two runaway programs, two
different budgets. Before running, match each to the resource it
exhausts and the E-code it earns:

```wolf
// program A
comptime fn dive(n: int) -> int {
    dive(n + 1)
}
// program B
comptime fn spin() -> int {
    while true {}
    0
}
```

Solution: A recurses, so it hits the *depth* budget (E0704, 256 call
frames); B loops in one frame, so it burns *fuel* (E0702, a step
count). Both diagnostics end with the same shape of help (raise the
budget at the use site) because the compiler cannot tell a runaway
from a computation that is merely large; only you can:

```console
$ wolf conform-run ./ex18-9a.lu
error[E0704]: comptime evaluation recursed past 256 call frames
help: raise the budget here: `#[budget(depth = 512)]`
$ wolf conform-run ./ex18-9b.lu
error[E0702]: comptime evaluation ran out of fuel after 1000000 steps
help: raise the budget here: `#[budget(fuel = 2000000)]`
```

**Exercise 18-10** *(extension (break-it-on-purpose) · wolf)*. Earn
E0703 (the *heap* budget) using only a `while` loop and a `var`,
without tripping fuel first. (You will need to grant fuel to get
there.)

Solution. `ch18/ex18-10.lu`: grant a large fuel budget so the loop
lives long enough to exhaust the 65536-cell comptime heap instead:

```wolf
comptime fn flood() -> int {
    var n = 0
    while n < 100000000 {
        n = n + 1
    }
    n
}
fn main() -> !int {
    #[budget(fuel = 100000000)]
    const N = flood()
    if N == 0 { 1 } else { 0 }
}
```

```console
$ wolf conform-run ./ex18-10.lu
error[E0703]: comptime evaluation exceeded its heap budget of 65536 cells
  --> ./ex18-10.lu:13:15
   |
13 |     const N = flood()
   |               ^^^^^^^ the allocation that went over happened here
   |               ------- while evaluating `flood`, entered here
   |               ------- while evaluating `main`, entered here
   |
   = note: the comptime heap is capped so evaluation cannot exhaust the machine compiling the
     program (D33); most overruns are unbounded value growth in a loop.
help: raise the budget here: `#[budget(heap = 131072)]`
```

The order of the two limits is the lesson: budgets are independent
meters, and the first one exhausted names the failure.

## Chapter batch

**Exercise 18-11** *(extension · wolf)*. An L-system is a string
rewriting rule applied in rounds: here `A → A-B` and `B → -A`, with
`-` carried through. Write `expand(axiom, steps)` as a `comptime fn`
and fold `expand("A", 3)` into a `const`. Compute the expected string
by hand before running.

Solution. `ch18/ex18-11.lu`, on the `wolf-run(…)` lane (the hand
expansion is `A` → `A-B` → `A-B--A` → `A-B--A--A-B`). The algorithm
is ordinary wolf, which is the tier's whole pitch: nothing about the
language changes at compile time, only the clock it runs on. Note the
solution's two spellings: the string grows by interpolation
(`next = "{next}A-B"` — `+=` on two strings appends the same way),
and the recoverable slice is `cur.get(i..i + 1)` with an `else`.

```console
$ wolf run ex18-11.lu
A-B--A--A-B
```

**Exercise 18-15** *(extension · wolf)*. Roman numerals, both ways,
folded: write `to_roman(n)` and `from_roman(s)` as `comptime fn`s
(the value table as a pair of indexed helpers, subtractive pairs and
all), and a witness `round_trips(n)` whose `assert` proves
`from_roman(to_roman(n)) == n` at compile time. Fold a year each way
into `const`s and print them. What does the witness buy that printing
both values does not, and which famous wrong numeral — `IIII` — does
your `from_roman` quietly accept, and why is that fine here?

Solution. `ch18/ex18-15.lu` (excerpt — the table helpers are
if-chains over an index, `glyph(at)`/`worth(at)`):

```wolf
comptime fn to_roman(n: int) -> str {
    var out = ""
    var rest = n
    var at = 0
    while at < 13 {
        while rest >= worth(at) {
            out = "{out}{glyph(at)}"
            rest -= worth(at)
        }
        at += 1
    }
    out
}
comptime fn from_roman(s: str) -> int {
    var total = 0
    var i = 0
    while i < s.len {
        let here = one(s, i)
        let next = if i + 1 < s.len { one(s, i + 1) } else { 0 }
        total += if here < next { 0 - here } else { here }
        i += 1
    }
    total
}
comptime fn round_trips(n: int) -> bool {
    assert(from_roman(to_roman(n)) == n)
    true
}
fn main() -> !int {
    const YEAR = to_roman(1994)
    const BACK = from_roman("MMXXVI")
    const WITNESS = round_trips(3888)
    print("{YEAR} {BACK}")
    if WITNESS { 0 } else { 1 }
}
```

```console
$ wolf run ex18-15.lu
MCMXCIV 2026
```

The witness buys a *build-breaking* claim: printed values need a
reader to check them, but a failed comptime `assert` is E0710 and no
binary exists — 3888 (`MMMDCCCLXXXVIII`, the longest numeral under
4000) round-trips or the program is refused. `IIII` is accepted
because `from_roman` implements the subtractive *reading* rule (a
smaller value before a larger one subtracts), which maps every
well-formed numeral correctly and some malformed ones charitably;
rejecting non-canonical spellings is a validator's job, and the
witness only claims the round trip from `to_roman`'s canonical
output. Stating exactly what the assert proves — no more — is most of
what writing one teaches.

**Exercise 18-12** *(design)*. The sandbox refuses a file read
(E0701) but the catalog entry points at *declared build inputs* through
the package manifest instead. Draw the line between the two designs:
what exactly does declaring an input buy that an ambient read does not
have? Name the failure the ambient read permits in each of: caching,
cross-machine reproducibility, and auditing a dependency you did not
write.

Solution (discussion): a declared input is part of the build's
identity: it is hashed, so the cache can key on it; it is listed, so
another machine can be handed the same bytes; it is visible, so an
auditor reads the manifest instead of the evaluator's traffic. The
ambient read defeats each in turn: a cache cannot know the file
mattered, so it serves stale artifacts; a second machine has a
different file or none, so the "same" build diverges; and an auditor
must now treat every comptime expression as a potential filesystem
probe, which is the exact posture chapter 24 spends a chapter
dismantling. The refusal is not a missing feature; it is the
load-bearing wall of the caching, reproducibility, and audit stories,
and the package manifest is where the need is threaded instead of
through the evaluator. *What* is read can be data; *that* it was read
must be declaration.
