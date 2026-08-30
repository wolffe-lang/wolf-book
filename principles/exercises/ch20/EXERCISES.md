# Chapter 20 — Perf contracts: exercises

Commands run from this directory; outputs are pasted from real runs.
Two sets share this file since the bs18 re-draw of Part 4. The
printed set (20-1 through 20-4) belongs to the chapter as written:
the language's own promises, the ledger, the floors and ratchets, the
exceptions file. The contracts corpus (20-5 through 20-13, the old
ch19 nine, renumbered with their subject) is UNPRINTED: it reasons
about function-level contract attributes under a verifying compiler,
the four attributes parse today and are verified by nothing (probed
at the bs18 pins: `wolf run`, `wolf build` and lupin all execute a
`#[noalloc]` function whose first line allocates, exit 0), and a
stem cannot carry that gap into a reader's text. The set stays in
the corpus so the chapter's verified-contracts section and its
exercises arrive together the day the checker lands. The old
bench-format set that previously held these numbers is retired
(EXERCISES-PENDING.md records it).

## §20.1 — The promises the language keeps

**Exercise 20-1** *(fingers · lupin)*. Change the line to `"a"` and
predict the printed value by hand before running: the seed is 7, the
multiplier 31, and `"a"` is one byte, 97. Then explain why this
function's signature would be a lie under plain `i32`.

Solution. `ch20/ex20-1.lu`. By hand: 7 × 31 = 217, plus 97 is 314,
no wrap on one byte:

```console
$ lupin ex20-1.lu
314
```

Under plain `i32` the signature would promise checked arithmetic and
the body would need it unchecked: a longer input overflows within a
handful of bytes, by design, and the checked spelling traps exactly
there. `wrapping[i32]` is the renegotiation written where the
compiler can hold it and a caller can read it. One byte happens to
stay under the ceiling either way; the signature is about every
input, not the lucky ones.

## §20.2 — The ledger and the clock

**Exercise 20-2** *(comprehension · prose)*. The ledger's history for
2026-08-22 contains three HOLDS lines: 03:53 (counted, consecutive 1),
04:11 (eighteen minutes later), and 06:30 (139 minutes after the
counted hold). The next holds land 2026-08-23 at 03:36 and 2026-08-24
at 03:39. State the consecutive count after each of the five lines,
and name the rule that decides each step.

Solution: 1, 1, 1, 2, 3. The 03:53 hold is counted; 04:11 and 06:30
land inside the twelve-hour window that separates counted holds, so
each records with the count unchanged (two samples of one thermal
state are one sample, however many timestamps they wear). The
2026-08-23 hold is 23.7 hours after the counted one and advances the
count to 2; 2026-08-24 advances it to 3, the declaration threshold,
at which the tool prints a banner and a human decides. The rule doing
all the work is the tick rule: a second reading counts only when a
night has actually passed, and the ledger, not the enthusiasm, judges
what a night is.

## §20.3 — The floors and the ratchets

**Exercise 20-3** *(comprehension · prose)*. The emitted-IR ratio's
original design target is half, and the recorded gate is a ceiling at
the measured value instead of at the target. State what a hard gate
at the unmet target would do to every pull request, what a team
learns to do with a gate that is always red, and what the ratchet
preserves that a red wall of shame would not.

Solution: a hard gate at 0.5 against a measured 0.58 fails every
commit, including the ones that improve the number, and tells none of
them anything new. A team with a permanently red gate learns the only
lesson a permanently red gate teaches: to stop reading it, and then
to route around it, at which point the gate guards nothing. The
ratchet preserves the two things a gate is for: regressions still
fail (the ceiling is the measured value, so any backsliding is a red
that means something), and progress is banked (each earned
improvement tightens the ceiling behind it). The unmet target stays
recorded beside the ratchet, which keeps the ambition without
spending the signal.

## §20.4 — Exceptions, written and capped

**Exercise 20-4** *(design)*. A teammate proposes an exception entry:
your image-decoding kernel would match C if it could read eight bytes
past the end of the input buffer, since the read provably stays in
the same page. Draft the entry's five fields, then rule on it: does
this qualify for the exceptions file, and if it does, what revisit
condition keeps it honest? Then rule on a second candidate: a kernel
that loses because the standard library lacks a bulk-copy method.

Solution (discussion): the first candidate qualifies, and the entry
writes itself. Kernel: the decoder. Root cause: the vectorized tail
wants an over-read; every in-bounds spelling costs a scalar epilogue.
Decision that renounced the win: out-of-bounds reads are undefined
behavior wolf refuses, page arithmetic notwithstanding (the language
has no "probably mapped" memory class). Measured cost: the A/B
against the over-reading C, with the date and host. Revisit
condition: a masked-load lowering or a padded-buffer idiom that gets
the tail without the over-read, at which point the entry expires. The
second candidate does not qualify: a missing library method is a bug
with an owner, not a renounced win, and parking it in the exceptions
file would spend a capped slot to make a to-do item stop looking like
one. It goes to the loss ledger, classified, with the kernel as the
regression test for the method's arrival.

## The contracts corpus (unprinted; the verifying compiler's set)

One honesty governs this set, recorded when it was written and still
true at the bs18 pins: the four contract attributes parse and are
*verified* by nothing — the checker that proves them against WIR
facts is I15's machinery. Each exercise says which side of that line
it stands on. The stems reason about what a verifying compiler must
do, which is why they can be written before it exists and printed
only after.

**Exercise 20-5** *(comprehension · pending — blocker: perf-contract
verification (I15); owner: s24–s26 WIR fact sprints)*. `build` carries
`#[noalloc]` and allocates a `List` on its first line. State what the
verifying compiler must do with this program, then run it under
today's tools and record what actually happens.

Solution. `ch20/ex20-5.lu`. The verifying compiler rejects it: the
attribute is a proof obligation, and the `List[int]()` in the body is
a WIR allocation fact that contradicts it. (The fail code is
unassigned in today's catalog; the `.lu` header states the expected
outcome in prose.) Today, honestly:

```console
$ lupin ex20-5.lu
3
$ echo $?
0
```

lupin executes the program — attributes are inert in the dynamic
tier — and `wolf conform-run` reports `verdict=unsupported` at
`phase_reached=mem`. Neither tool lies about checking the promise;
neither checks it. That gap is this chapter's ledger entry, and CI
flips these exercises to verified the day I15's checker lands.

**Exercise 20-6** *(comprehension · prose)*. Four bodies, one
attribute. Which of these could carry `#[noalloc]` under a verifying
compiler, and for each refusal, name the allocating expression:

1. `fn mid(xs: List[int]) -> int { xs[xs.len / 2] }`
2. `fn label(n: int) -> str { "item {n}" }`
3. `fn double_all(mut xs: List[int])` — multiplies each element in place
4. `fn tail(s: str) -> str { s[1..] }`

Solution: 1, 3, and 4 qualify. Indexing reads; in-place mutation
writes into storage the caller already owns; a byte slice is two words
pointing into the original — chapter 2's claim, now earning its keep.
Number 2 is the refusal: an f-string builds a new `str`, and a new
`str` is an allocation no matter how small the sentence. The wrong
answer worth ruling out is 4: "returns a str" is not "allocates a
str" — the contract tracks allocation, not types.

**Exercise 20-7** *(comprehension · prose)*. May a verifying compiler
accept `#[nopanic]` on this function?

```wolf
fn add_prices(a: i32, b: i32) -> i32 { a + b }
```

Solution: no, and the reason is chapter 3's oldest fact wearing a new
coat: `+` on `i32` is checked, checked arithmetic traps on overflow,
and a trap is exactly what `#[nopanic]` promises away. The function is
one range analysis away from acceptable — prove `a` and `b` small
enough and the trap is unreachable — which is why `#[nopanic]` is a
hard contract to hold and why chapter 21 prices checked arithmetic
separately. The cheap fixes each change the promise: `wrapping[i32]`
keeps `nopanic` and changes the arithmetic; dropping the attribute
keeps the arithmetic and changes the promise.

**Exercise 20-8** *(fingers · lupin)*. Annotate a genuinely
allocation-free function with `#[noalloc]` and run it. Then state
precisely what today's toolchain claimed about your attribute.

Solution. `ch20/ex20-8.lu`:

```wolf
struct Vec3 { x: f64, y: f64, z: f64 }
#[noalloc]
fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
fn main() -> !int {
    let d = dot(Vec3 { x: 1.0, y: 2.0, z: 3.0 }, Vec3 { x: 3.0, y: 2.0, z: 2.0 })
    print("{d}")
    0
}
```

```console
$ lupin ex20-8.lu
13
```

Today's toolchain claimed nothing. The run proves the body computes a
dot product; it proves nothing about the attribute, which no tool read.
An unverified contract is a comment with better syntax — until I15,
exactly that, and this book will not pretend otherwise.

**Exercise 20-9** *(comprehension · prose)*. `#[inplace]` promises a
function mutates through its `mut` parameters without allocating
working storage. Which of these keeps that promise: (a) reversing a
`List` with one temporary variable inside a swap loop; (b) reversing a
`List` by building a second list backward and assigning it over the
parameter?

Solution: (a). The swap loop's temporary is a stack value — one
element in flight, storage the frame already owns. (b) allocates a
whole shadow list, and assigning it over the parameter afterward does
not un-allocate it; the contract's subject is what the function
*acquired*, not where the bytes ended up. The observable result of
both versions is identical, which is the reason the contract exists:
callers on a hot path cannot see the difference in the signature
unless the signature says it.

**Exercise 20-10** *(spelunking · wolf)*. The pinned corpus states
I15's rule in one comment (`upstream/corpus/comptime.lu`): "`#[noalloc]`
is compiler-VERIFIED against WIR facts, not a comment that rots."
Explain the "comment that rots" failure mode this contracts against,
and then reconcile the claim with what `wolf conform-run` did to
exercise 20-5 today.

Solution: a performance comment rots because nothing fails when it
stops being true — the function grows an allocation in a refactor,
the comment stays, and callers keep budgeting against fiction. The
contract moves the claim into the signature where a checker can
contradict it. Reconciliation: the corpus comment states the *design*
(I15, decided); today's `verdict=unsupported` states the
*implementation* (the WIR fact engine is s24–s26's deliverable). The
corpus and this book share one honesty model — expected outcomes are
recorded before the machinery exists, and nothing reports green
meanwhile.

**Exercise 20-11** *(comprehension · prose)*. Your dependency's
`parse_row` carried `#[noalloc]` in v1.3; v1.4 drops the attribute
with no other signature change. Under semver with teeth, what version
number must v1.4 actually be, and what breaks if the registry lets it
sail as a minor?

Solution: major. A caller was entitled to build on the promise — an
audio callback that calls `parse_row` per sample chose it *because*
allocation-free was in the signature. If the drop ships as 1.4, MVS
upgrades that caller silently and the callback now allocates on a
real-time path: no compile error, no diagnostic, a latency regression
found in production. Contracts are API is not a slogan — it is the
statement that removing one is the same event as removing a function.
(Adding a contract is the minor direction: strictly more promise.)

**Exercise 20-12** *(design)*. A colleague proposes `#[noalloc]` on
every function in your utility library "since most of them qualify."
Argue the other side using exactly two costs, then state the rule you
would adopt instead.

Solution (discussion): cost one is freedom — every attribute is a
promise you must keep through every future refactor; the day one
function wants a scratch buffer, its removal is a major version, per
20-11, for a property no caller may ever have needed. Cost two is
signal — when everything is annotated, annotation stops meaning
"chosen for a hot path" and reads as boilerplate; the one contract
that matters drowns in forty that do not. The rule that survives:
annotate at the *boundary where a caller budgets* — the parse loop,
the callback, the per-frame kernel — and leave interior helpers free
to change. A contract is a price tag, and price tags belong on things
that are for sale.

**Exercise 20-13** *(extension · lupin)*. Exercise 20-5's `build`
cannot keep its promise because it allocates its own result. Refactor
to the shape that could: the caller allocates once and lends the
storage down with `mut`. Run it. Which function in your refactor could
now honestly carry `#[noalloc]`, and which still cannot?

Solution. `ch20/ex20-13.lu`:

```wolf
fn tally_into(mut acc: List[int], base: int) {
    var i = 0
    while i < acc.len {
        acc[i] = acc[i] + base
        i += 1
    }
}
fn main() -> !int {
    var acc = List[int]()
    var i = 0
    while i < 5 { (mut acc).push(i); i += 1 }
    tally_into(mut acc, 3)
    tally_into(mut acc, 4)
    var sum = 0
    for v in acc { sum += v }
    print("{sum}")
    0
}
```

```console
$ lupin ex20-13.lu
45
```

`tally_into` qualifies: it writes through storage it was lent. `main`
does not — someone must still allocate, and the refactor's point is
choosing *who*: allocation moved from once-per-call to once-per-
program, and the function on the hot path is the one that got to make
the promise. (Verification of that promise remains pending per 20-5;
the shape is ready for it.)
