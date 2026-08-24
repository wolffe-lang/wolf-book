# Chapter 21 — Beating C honestly: exercises

Commands run from this directory; outputs are pasted from real runs.
The honesty that heads the chapter heads its exercises: semantic claims
run today under lupin; *measured* claims wait for s44's rigs, and no
number below pretends otherwise. Arithmetic the reader can do by hand
is the point of half this set — the performance argument of this
chapter is mostly counting, and counting is checkable.

## §21.1 — Aliasing

**Exercise 21-1** *(comprehension · prose)* — Here is saxpy in C:

```c
void saxpy(double a, const double *xs, double *ys, size_t n) {
    for (size_t i = 0; i < n; i++) ys[i] = a * xs[i] + ys[i];
}
```

Without `restrict`, name the specific possibility the C compiler must
plan for, and the optimization it therefore hesitates on. Then state
what a wolf compiler knows about `saxpy(a, xs, mut ys)` from the
signature alone, and who did the work of establishing it.

Solution: the C compiler must assume `xs` and `ys` may overlap — a
store through `ys[i]` could change some later `xs[j]`, so reordering
and vectorizing the loads requires either a runtime overlap check or
giving up the transform. `restrict` is the programmer *promising*
disjointness, unchecked: get it wrong and the program is undefined. In
wolf, `mut ys` is an exclusive claim and `xs` a shared read — chapter
7's rule — so disjointness is a fact the type system already proved at
every call site. Same fact, different laborer: C trusts the
programmer's word; wolf makes the caller demonstrate it, once, at
compile time.

**Exercise 21-2** *(fingers · lupin)* — Type the wolf saxpy and run
it: five elements, `a = 2.0`, `ys` all tens. Predict both printed
values first.

Solution — `ch21/ex21-2.lu`:

```wolf
fn saxpy(a: f64, xs: List[f64], mut ys: List[f64]) {
    var i = 0
    while i < xs.len {
        ys[i] = a * xs[i] + ys[i]
        i += 1
    }
}
```

```console
$ lupin ex21-2.lu
12 20
```

2·1 + 10 and 2·5 + 10, printed the way a whole-valued `f64` prints —
shortest round-trip, so `12` rather than `12.0`. The kernel is
deliberately the same one as 21-1: what runs here is the semantics, on
both machines and on the compiler's release tier alike. The suite
§21.4 cites gates this same shape against naive `clang -O3`; the
numbers on the page stay CI's.

## §21.2 — Arenas

**Exercise 21-3** *(comprehension · prose)* — A request handler builds
a parse tree of 10,000 nodes, reads it, and discards it. Count the
allocator interactions — calls into allocate and free machinery — for
(a) malloc discipline with individual `free`, (b) malloc discipline
with one arena library, (c) a wolf region. Then name the cost in (c)
that did *not* disappear and where it went.

Solution: (a) 20,000 — every node allocated and freed retail. (b) on
the order of a few dozen — the arena grabs slabs and frees them
wholesale; nodes are pointer bumps, which is the arena's entire trick.
(c) matches (b) at runtime — bump allocation, one wholesale free at
region end — with the checking moved to compile time: the guarantee
that no node pointer outlives the region is the region checker's
proof, not a code review's hope. What did not disappear: the proof
obligation. C's arena has the same lifetime rule and enforces it with
discipline; wolf's region has it as a type fact. The allocator math is
identical — chapter 8 said so — and the difference is who catches the
escapee.

## §21.3 — Layout

**Exercise 21-4** *(comprehension · prose)* — 1,000 particles, each
`{ x: f64, y: f64, z: f64 }` (24 bytes, no padding). A pass reads only
`x`. With 64-byte cache lines, compute the lines fetched for the pass
under array-of-structs and under `Soa[Particle]`'s x-column, and the
fraction of each fetched byte that was used.

Solution: AoS — 24,000 contiguous bytes, ceil(24000/64) = 375 lines,
of which the pass uses 8,000 bytes: exactly one third of the traffic
did work. SoA — the x-column is 8,000 contiguous bytes, 125 lines,
every byte used. Three times fewer lines is the mechanical claim
behind §21.3's benchmark; nothing about it is wolf-specific except
who builds the layout — `Soa[T]` is comptime machinery (chapter 18)
rather than a macro or a hand-maintained pair of parallel arrays,
which is why "legally" appears in the section title.

## §21.4 — Checked arithmetic's bill

**Exercise 21-5** *(comprehension · lupin)* — The bill and the payout
in one program: `sum_to(n)` adds 1,000,000 to an `i32` accumulator `n`
times. Predict both calls' fates — `sum_to(2000)`, then
`sum_to(3000)` — with the arithmetic that decides them.

Solution: 2000 × 1,000,000 = 2.0 × 10⁹ fits under `i32`'s
2,147,483,647 ceiling; 3000 × 1,000,000 crosses it at iteration 2148:

```console
$ lupin ex21-5.lu
2000000000
ex21-5.lu: trap(overflow): `+` produced 2148000000, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 272..286
$ echo $?
3
```

Every one of those two million additions carried the check that made
the last one honest. What the check *costs* after optimization is a
measured number with a date on it, and §21.4 prints it from CI's own
ledger — the checked-adds exception in the suite's gate is that cost
made explicit — rather than asserting it here.

**Exercise 21-6** *(spelunking · lupin)* — From exercise 21-5's trap
line alone: name the decision id it cites, the clause tag it enforces,
and the documented spelling for the program that *wanted* wraparound.
Then state, in one sentence, why this trap firing "in every profile"
is the chapter's honesty rather than the chapter's embarrassment.

Solution: X3 is the decision; `[arith.checked]` the clause;
`wrapping[i32]` the intended-overflow spelling — all three are in the
line, which is the point of trap lines. The one sentence: a language
claiming to beat C while quietly disabling its own safety checks in
release builds would be rigging the race, and X3 is wolf agreeing to
be benchmarked with the checks on.

## §21.5 — Where C wins today

**Exercise 21-7** *(design)* — A loss table in §21.5's format (a
worked example, not CI output) reads:

```text
kernel          vs C     tracking
strchr-16       -22%     #g214  glibc hand-vectorized SIMD
bitrev-perm     -9%      #g221  autovectorizer misses permute idiom
memcpy-small    -4%      #g208  call overhead under 64 bytes
```

Rank the three by how likely they close from compiler work alone, and
name what each would take. Which of the three would you *bet against*
ever closing, and why is printing that row still the right call?

Solution (discussion): `memcpy-small` closes most plausibly — inlining
thresholds and a builtin lowering are ordinary mid-end work (s42's
territory). `bitrev-perm` is a pattern-match away *if* the idiom is
recognizable, which autovectorizers hit and miss on; call it even
money. `strchr-16` is the bet-against: glibc's version is
hand-scheduled SIMD by people who read uarch manuals for sport, and
matching artisanal assembly with a general-purpose compiler is a
decades-old open engagement. Printing the row anyway is the chapter's
thesis performed: a perf pitch that hides its losses converts nobody
who has been lied to before — and the tracking issue turns each loss
from an admission into a work item with a date.

## Chapter batch

**Exercise 21-8** *(comprehension · pending — blocker: bench rigs and
CI perf gates; owner: s44-perf-validation)* — When the rigs land: run
the saxpy comparison from 21-1/21-2 — wolf against `clang -O2`, with
and without `restrict` on the C side — and read the three-way result
against your 21-1 answer. The C-without-`restrict` column is the
aliasing tax made visible; predict its sign before looking.

Solution (prose, pending execution): the prediction on record — wolf
and C-with-`restrict` within noise of each other, C-without-`restrict`
behind on vectorizable sizes, the gap widening with stride complexity.
If the measured result disagrees, the *exercise* is finding out why;
this corpus keeps the header and CI keeps the appointment.

**Exercise 21-9** *(comprehension · prose)* — "Beats naive C, and the
claim is a falsifiable CI gate" is a sentence with a specific
engineering content. Name the three artifacts that must exist for the
claim to be falsifiable rather than promotional, and for each say
whether this edition already prints it.

Solution: a pinned, public benchmark suite — the kernels, their C
twins, and the gate that reads them (this edition prints its verdict
line in §21.4, with the repository path and the date); a variance
discipline that can call a delta noise — medians, mean absolute
deviation, a symmetric gate (the instrument that would put that
discipline in your hands is chapter 20's subject, and this edition
does not carry chapter 20); and a dated, regenerated record wired to
CI so the claim expires when the world changes — the colophon's
toolchain pin and the ledger line §21.4 quotes, which names its
commit and its night. Remove any one and the sentence degrades to
advertising: no suite and it is unmeasured, no variance gate and it
is cherry-picked, no date and it is folklore.
