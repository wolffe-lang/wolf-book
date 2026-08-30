# Chapter 18 — Comptime: one tier, no macros: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the wolf-lang debug build
at `impl_version 0.0.1`. The comptime engine is wolf's, and every
verdict below is a real run of it: the sandbox refusals, the budget
meters, the comptime-known rule, checked arithmetic at compile time, and
the reflected witnesses all evaluate today. What no lane does yet is
execute a fold's *result* — `wolf build`/`wolf run` decline a module
holding a `comptime fn`, and lupin declines one outright — so the three
exercises whose deliverable is a printed folded value stay pending with
that blocker, and their expected outcomes ride in their directive
headers. They are also the three the chapter does not print.

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

**Exercise 18-3** *(fingers · pending — blocker: no lane executes a
comptime fold's result; owner: c05-codegen / wolf-interp std subset)*.
Write `sum_squares(n)` as
a `comptime fn` and bind `const T = sum_squares(9)`. The folded value
is 285, and after s16 the program prints it having computed nothing at
runtime.

Solution. `ch18/ex18-3.lu` carries the program with its expected
directive header (`run(exit=0, stdout="285")`). The evaluator computes
this fold; what is missing is a lane that runs a program holding the
`comptime fn` afterward. Both reader-facing tools decline the module,
and this is what pending looks like:

```console
$ lupin ex18-3.lu
ex18-3.lu: unsupported: `sum_squares` is a `comptime fn`; compile-time evaluation with its sandbox and budgets is the compiler's engine (s16), and nothing in `spec/` pins it — the `comptime` namespace is still a reserved forward one
$ echo $?
4
```

Under `wolf conform-run` the observation is `verdict=unsupported` at
`phase_reached=mem`, and `wolf run` declines with the lowering gap
named. The interpreter's exit code is 4, not 2 and not 3: outside
scope, stated, rather than rejected or trapped.

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

**Exercise 18-5** *(extension · pending — blocker: a `typeinfo` result
reaching a runtime `const` is `calls outside the modelled surface` in
the checked lane, on top of the comptime-fn lowering gap; owner:
c05-codegen / wolf-interp std subset)*. Write `field_count(T: type)` using
`typeinfo`, and apply it to a struct of your own. State what the
program will print for a three-field struct once s16 lands.

Solution. `ch18/ex18-5.lu` (expected `run(exit=0)`, printing `3`).
The signature `fn field_count(T: type) -> int` is the section's whole
point in four tokens: a type arrives as an argument, like any other
value. The reflection itself runs today (§18.2's witness proves the
field count at compile time and E0710 reports a wrong one) but the
count cannot be printed by a program. Today:

```console
$ lupin ex18-5.lu
ex18-5.lu: unsupported: `field_count` is a `comptime fn`; compile-time evaluation with its sandbox and budgets is the compiler's engine (s16), and nothing in `spec/` pins it — the `comptime` namespace is still a reserved forward one
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

**Exercise 18-11** *(extension · pending — blocker: no lane executes a
comptime fold's result; owner: c05-codegen / wolf-interp std subset)*.
An L-system is a string
rewriting rule applied in rounds: here `A → A-B` and `B → -A`, with
`-` carried through. Write `expand(axiom, steps)` as a `comptime fn`
and fold `expand("A", 3)` into a `const`. Compute the expected string
by hand before reading the header.

Solution. `ch18/ex18-11.lu` (expected `run(exit=0,
stdout="A-B--A--A-B")`; the hand expansion is `A` → `A-B` → `A-B--A` →
`A-B--A--A-B`). The expected stdout was verified by running the same
function as a runtime `fn` under lupin, which prints `A-B--A--A-B`.
The algorithm is ordinary wolf, which is the tier's whole pitch:
nothing about the language changes at compile time, only the clock it
runs on. Note the two spellings the language actually has: strings are
joined by interpolation (`next = "{next}A-B"`) and not by `+=`, which
`wolf` rejects with E0409; and the recoverable slice is
`cur.get(i..i + 1)` with an `else`. Today the comptime spelling reports
`unsupported` under both tools.

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
