# Chapter 19 — Perf contracts: exercises

Commands run from this directory; outputs are pasted from real runs.
One honesty governs this chapter's set: the four contract attributes
parse today and are *verified* by nothing — the checker that proves
them against WIR facts is I15's machinery, landing with s24–s26. The
exercises below say which side of that line they stand on.

## §19.1 — Four promises

**Exercise 19-1** *(comprehension · pending — blocker: perf-contract
verification (I15); owner: s24–s26 WIR fact sprints)*. `build` carries
`#[noalloc]` and allocates a `List` on its first line. State what the
verifying compiler must do with this program, then run it under
today's tools and record what actually happens.

Solution. `ch19/ex19-1.lu`. The verifying compiler rejects it: the
attribute is a proof obligation, and the `List[int]()` in the body is
a WIR allocation fact that contradicts it. (The fail code is
unassigned in today's catalog; the `.lu` header states the expected
outcome in prose.) Today, honestly:

```console
$ lupin ex19-1.lu
3
$ echo $?
0
```

lupin executes the program — attributes are inert in the dynamic
tier — and `wolf conform-run` reports `verdict=unsupported` at
`phase_reached=mem`. Neither tool lies about checking the promise;
neither checks it. That gap is this chapter's ledger entry, and CI
flips these exercises to verified the day I15's checker lands.

**Exercise 19-2** *(comprehension · prose)*. Four bodies, one
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

**Exercise 19-3** *(comprehension · prose)*. May a verifying compiler
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

**Exercise 19-4** *(fingers · lupin)*. Annotate a genuinely
allocation-free function with `#[noalloc]` and run it. Then state
precisely what today's toolchain claimed about your attribute.

Solution. `ch19/ex19-4.lu`:

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
$ lupin ex19-4.lu
13
```

Today's toolchain claimed nothing. The run proves the body computes a
dot product; it proves nothing about the attribute, which no tool read.
An unverified contract is a comment with better syntax — until I15,
exactly that, and this book will not pretend otherwise.

**Exercise 19-5** *(comprehension · prose)*. `#[inplace]` promises a
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

## §19.2 — Contracts are API

**Exercise 19-6** *(spelunking · wolf)*. The pinned corpus states
I15's rule in one comment (`upstream/corpus/comptime.lu`): "`#[noalloc]`
is compiler-VERIFIED against WIR facts, not a comment that rots."
Explain the "comment that rots" failure mode this contracts against,
and then reconcile the claim with what `wolf conform-run` did to
exercise 19-1 today.

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

**Exercise 19-7** *(comprehension · prose)*. Your dependency's
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

## §19.3 — When not to

**Exercise 19-8** *(design)*. A colleague proposes `#[noalloc]` on
every function in your utility library "since most of them qualify."
Argue the other side using exactly two costs, then state the rule you
would adopt instead.

Solution (discussion): cost one is freedom — every attribute is a
promise you must keep through every future refactor; the day one
function wants a scratch buffer, its removal is a major version, per
19-7, for a property no caller may ever have needed. Cost two is
signal — when everything is annotated, annotation stops meaning
"chosen for a hot path" and reads as boilerplate; the one contract
that matters drowns in forty that do not. The rule that survives:
annotate at the *boundary where a caller budgets* — the parse loop,
the callback, the per-frame kernel — and leave interior helpers free
to change. A contract is a price tag, and price tags belong on things
that are for sale.

## Chapter batch

**Exercise 19-9** *(extension · lupin)*. Exercise 19-1's `build`
cannot keep its promise because it allocates its own result. Refactor
to the shape that could: the caller allocates once and lends the
storage down with `mut`. Run it. Which function in your refactor could
now honestly carry `#[noalloc]`, and which still cannot?

Solution. `ch19/ex19-9.lu`:

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
$ lupin ex19-9.lu
45
```

`tally_into` qualifies: it writes through storage it was lent. `main`
does not — someone must still allocate, and the refactor's point is
choosing *who*: allocation moved from once-per-call to once-per-
program, and the function on the hot path is the one that got to make
the promise. (Verification of that promise remains pending per 19-1;
the shape is ready for it.)
