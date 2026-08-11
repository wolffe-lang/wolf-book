# Chapter 9 — The escape hatch is a door, not a cliff: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; the UB oracle's reports are shown
in full — reading them is most of this chapter's homework.

## §9.1 — The three rings

**Exercise 9-1** *(fingers + spelunking)* — The complete unsafe audit
of every program in this book's first eight chapters is one command.
Run it from `principles/exercises/`, report the number, and state what
property of the language makes the count trustworthy — what would the
same grep miss in C?

Solution:

```console
$ grep -rn "unsafe {" ch01 ch02 ch03 ch04 ch05 ch06 ch07 ch08 | wc -l
0
```

Zero: parts 1 and 2 up to this chapter never left the safe tier. The
count is trustworthy because `unsafe` is the *only* entrance — raw
pointers, foreign calls, and aliasing assertions do not parse outside
the block, so the keyword is a complete index of the ring boundary. In
C the equivalent grep misses everything, because there is no keyword:
every pointer dereference in the program is potentially the audit's
subject, which is to say the audit is the whole program.

## §9.2 — Raw-tier rules

**Exercise 9-2** *(fingers · lupin)* — Your first unsafe block, kept
legal: allocate eight bytes from C, set them all to 5, read one back,
free, print. Type it, run it, and note the exit code — the point of
this exercise is that nothing happens.

Solution — `ch09/ex9-2.lu`:

```wolf
import c "stdlib.h"
fn main() -> !int {
    unsafe {
        let p = c.malloc(8) as *u8
        c.memset(p, 5, 8)
        let v = p[3] as int
        c.free(p)
        print("{v}")
    }
    0
}
```

```console
$ lupin ex9-2.lu
5
$ echo $?
0
```

Inside the block, C's rules apply — allocate, use, free, in that
order, and the oracle has nothing to say. The block is a contract
change, not a crime scene.

**Exercise 9-3** *(comprehension · lupin)* — One character changes in
9-2: the write is `p[8] = 1`. The allocation holds eight bytes.
Predict the oracle's finding — its section number, and which optimizer
license the report will name.

Solution — the report, in full:

```console
$ lupin ex9-3.lu
ex9-3.lu: ub(mem.ub) §7/P3: write of 1 byte(s) at alloc#0[8], which holds 8 [mem.ub] at 207..215; tag created at 154..165
  licenses O3a: `dereferenceable(n)` on known-size accesses; bounds-based alias disproof between distinct allocations
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

§7/P3, out-of-bounds — and the license line is the half worth reading
twice: the *reason* one byte past the end is UB rather than a trap is
that the compiler wants to assume accesses stay inside their
allocation (`dereferenceable`, alias disproof between allocations).
Every oracle report has this shape: the fault, then the optimization
that the rule purchases. UB is not a punishment; it is a price list.

**Exercise 9-4** *(comprehension · lupin)* — The pointer is laundered
through an integer before the read:

```wolf
let p = c.malloc(8) as *u8
c.memset(p, 5, 8)
let address = p as int
c.free(p)
let q = address as *u8
let v = q[0] as int
```

An integer survives `free` untouched. Does the roundtrip save the
read? Predict the oracle's answer and its reasoning.

Solution: no. The integer survives; the *permission* does not.

```console
$ lupin ex9-4.lu
ex9-4.lu: ub(mem.ub) §7/L2: read through an exposed pointer into alloc#0, which was freed [mem.unsafe.raw.1] at 338..342; tag created at 197..208
  licenses O8: escape analysis / stack promotion without conservatively pinning addresses
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

The cast back from `int` reconnects the pointer to the allocation's
exposed tags — but every tag of a freed allocation is Disabled, so no
defined execution exists to choose. The license explains who benefits:
if an integer roundtrip could resurrect access, the compiler could
never promote an allocation to a register, because some integer
somewhere might name its address.

## §9.3 — The oracle you actually run

**Exercise 9-5** *(fingers + comprehension · lupin)* — Inject the classic:
write, free, read, through one pointer. Run it twice. What is the
oracle's finding, and — the actual question — what is identical
between the two runs that would *not* be identical for a
use-after-free in C?

Solution — `ch09/ex9-5.lu`:

```wolf
let p = c.malloc(8) as *u8
p[0] = 7
c.free(p)
let v = p[0]
```

```console
$ lupin ex9-5.lu
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 260..264; tag created at 190..201
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ lupin ex9-5.lu
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 260..264; tag created at 190..201
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
```

Byte-identical reports, twice — same fault, same spans, same tag
story. A C use-after-free reads whatever the allocator left there:
sometimes 7, sometimes a new object, sometimes a crash, varying with
allocator mood and moon phase. The oracle replaces "what happened to
be in memory" with "what the rules say about this access," and rules
do not vary between runs. Deterministic faults are what make the
escape hatch *debuggable* — this is §9.3's whole pitch, performed.

**Exercise 9-6** *(comprehension · lupin)* — Two lines:

```wolf
let n = 7
let b = n as bool
```

No pointer, no allocation, no free. Is this UB? Predict the oracle's
verdict and, if you predict a finding, name what optimization a
`bool` outside {0, 1} would break.

Solution: it is UB — validity is part of the raw tier's contract, not
only liveness:

```console
$ lupin ex9-6.lu
ex9-6.lu: ub(mem.ub) §7/T1: `7 as bool` produces a `bool` outside {0, 1}; the representation is restricted, which is what licenses niche packing and default-free jump tables [mem.ub] at 168..177
  licenses O9: niche packing; match jump tables without default arms; UTF-8 fast paths without re-validation
$ echo $?
3
```

A `bool` that might be 7 breaks niche packing (`Option`-style layouts
that store "absent" in the unused values) and jump tables compiled
without a default arm. The report says so in its own words; the
license *is* the explanation.

## §9.4 — The one door back

**Exercise 9-7** *(comprehension · lupin)* — Two programs differ by
one line's position. Both allocate eight C bytes, both cross back to
safe code through `borrow r from p` — the door. In the first, the
`malloc` happens inside `in r { }`; in the second, outside any window.
Predict each verdict before running either, and state the door's
obligation in one sentence.

Solution — `ch09/ex9-7a.lu` and `ch09/ex9-7b.lu`:

```console
$ lupin ex9-7a.lu
$ echo $?
0
$ lupin ex9-7b.lu
ex9-7b.lu: ub(mem.ub) §7/P6: `borrow region #1 from` a pointer into alloc#0, which is owned by `program` (region #0) — the obligation is that the allocation lies wholly inside the named region's footprint [mem.unsafe.door] at 273..288; tag created at 212..223
  licenses O6: safe-tier code after the door keeps all safe-tier entitlements (O1–O4) — the door is where trust concentrates
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

The obligation: the allocation must lie wholly inside the named
region's footprint. The first program allocated while `r` was ambient,
so the claim is true; the second claims bytes owned by the program
region. The license line is the chapter's thesis in one clause: after
a truthful door, safe code keeps *all* its entitlements — which is why
the door is one narrow place, and why lying to it is the worst lie in
the language.

## §9.5 — `#include`-grade C

**Exercise 9-8** *(fingers · pending — blocker: real libc calls; the
interpreter models only its documented host-intrinsic set
(approximation-contract §8); owner: s46-libclang-importer, with
s29-abi-v0 beneath it)* — Import `math.h` and call `c.sqrt(4.0)`
inside an unsafe block, printing the result. The expected program is
on disk; predict its output the day it runs.

Solution (prose): it prints `2` and exits 0 — `sqrt` is a pure
function crossing the membrane with one f64 in and one out, the
simplest possible FFI shape and the reason it is this exercise.
Today:

```console
$ lupin ex9-8.lu
ex9-8.lu: unsupported: `c.sqrt` is an imported C function this machine does not model; the host-intrinsic set is documented in `docs/approximation-contract.md` §8, and inventing a body for a real libc call would put guessed behavior into a differential comparison
$ echo $?
4
```

The refusal is itself a lesson in the differential method: a guessed
`sqrt` would poison every comparison downstream, so the tool says
`unsupported` instead — exit 4, the honest code for "outside my
scope."

## §9.6 — FFI and regions

**Exercise 9-9** *(comprehension · lupin)* — A C allocation made while
a region was ambient, escaping the region that owned it:

```wolf
let p = region r {
    let inner = c.malloc(8) as *u8
    c.memset(inner, 5, 8)
    inner
}
let v = p[0] as int
```

The pointer is a plain integer-like value; it moves out fine. Predict
what the read faults with, and why the report will differ from 9-5's
use-after-free.

Solution:

```console
$ lupin ex9-9.lu
ex9-9.lu: ub(mem.ub) §7/P4: read at alloc#0[0], whose owning region #1 was freed wholesale [mem.prov.region] at 317..321; tag created at 220..231
  licenses O3b: one alias-scope domain per region — pointers into distinct regions never alias; O4: regions not open in the current scope yield `invariant.load`
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #1
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

§7/P4, not P1: nobody called `free` — the *region* died, and it took
the allocation's permissions with it. Note the report's strange-
looking line: the allocation is still `live` (its bytes were never
individually freed) but its owning region is gone, and that is enough.
The rule C code linked into wolf must learn: memory borrowed while a
region was ambient is a loan *from the region*, and the region's
death calls it in, wholesale.

## §9.7 — Auditing: `#[trusted]` and `wolf audit`

**Exercise 9-10** *(spelunking · pending — blocker: `wolf audit` and
capability manifests land with the package manager; owner:
s51-package-manager)* — `ch09/ex9-10.lu` wraps a pretend C call in a
`#[trusted]` function. The attribute parses today and the program
runs; what does not exist yet is the ledger that makes `#[trusted]`
mean something. Answer from the chapter: when `wolf audit` lands, what
two questions about this function will it answer that reading the
source cannot?

Solution (prose): first, *transitivity* — whether anything this
function calls (or anything its dependencies call) is itself trusted
or capability-bearing; source review answers one file, the audit
answers the closure. Second, *drift* — whether the next release of a
dependency widens its capability set (`net`, `fs`, a new trusted
block) relative to what was reviewed; the audit is a diffable fact,
review is a memory. As run:

```console
$ lupin ex9-10.lu
$ echo $?
0
$ wolf audit
wolf audit: not yet (grows at its own campaign; D34's single binary)
```

## §9.8 — The four-tier picture

**Exercise 9-11** *(comprehension · prose)* — Five fragments; place
each on the four-tier map (safe values, regions, unsafe raw,
the door):

1. `let b = copy a`
2. `pool[h].value`
3. `p[8] = 1` where `p: *u8`
4. `borrow r from p`
5. `ch.send(move r)` where `r` is a closed region

Solution: 1 is tier-0 safe values — chapter 7's world, no annotations.
2 is the region tier's shared edge — a handle read, checked by
generation, faulting stale rather than dangling. 3 is the raw tier —
legal only inside `unsafe`, governed by the oracle's price list. 4 is
the door itself — the one construct that moves data *up* a tier, with
its truth obligation. 5 is the region tier's transfer verb, safe
because closed-subtree moves preserve every invariant. The picture to
keep: four tiers, one direction of trust — every construct on this
list either stays in its tier or crosses at the door, and nothing else
crosses at all.

**Exercise 9-12** *(design)* — A team wraps a 40,000-line C codec
behind wolf FFI. Debate the two candidate shapes: (a) one `unsafe`
block per call site, spread through the application; (b) one module
owning every unsafe line, exporting twenty safe functions, `#[trusted]`
on the membrane. Which failure modes does each shape optimize for, and
what does the twenty-line rule from §9.5 actually buy the reviewer?

Solution (discussion): shape (a) optimizes for nothing except writing
speed; its failure mode is that the audit surface *is* the
application, and every new call site is a new review. Shape (b)
optimizes for the reviewer: the unsafe ring is one module, the grep
from 9-1 returns one path, and the twenty safe functions are the
complete list of claims the C code makes about itself. Its failure
mode is worth naming honestly — the membrane can become a lie if the
safe signatures promise more than the C delivers (a `str` that is not
UTF-8, a buffer length the codec ignores), and `#[trusted]` marks
exactly where that lie would live. The twenty-line rule buys
*proportionality*: a reviewer can hold twenty lines to the standard
"I believe each one," which is the standard unsafe code requires and
40,000 lines cannot meet. The door metaphor closes the argument:
doors work because buildings have few of them.

## Chapter batch

**Exercise 9-13** *(extension (break-it-on-purpose) · lupin)* — Construct the
shortest program you can in which the *assertion*, not any access, is
the undefined behavior: use `assume noalias` on two pointers that
alias. Predict the oracle's wording — what does it say overlaps what?

Solution — `ch09/ex9-13.lu`:

```wolf
let p = c.malloc(8) as *u8
let q = p
assume noalias p, q
p[0] = 1
q[0] = 2
```

```console
$ lupin ex9-13.lu
ex9-13.lu: ub(mem.ub) §7/P5: `assume noalias` asserts these ranges are disjoint, and alloc#0[0..1) overlaps alloc#0[0..1) — the assertion is false [mem.unsafe.raw.2] at 223..242; tag created at 178..189
  licenses O5: the asserted ranges get `noalias` treatment in Tier-3 code — vectorization/reordering as if proven
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

The range overlaps *itself* — `alloc#0[0..1)` against `alloc#0[0..1)`
— because `q` is `p`. This is the only assertion-created UB in the
raw tier: everything else about `*T` is unrestricted, but a spoken
aliasing promise is kept or it is UB the moment the accesses disagree
with it. It is also the exercise to remember when C's `restrict`
comes up: wolf did not remove the footgun, it made the trigger
visible and gave it an oracle.

**Exercise 9-14** *(comprehension · lupin)* — The subtlest report in
the chapter. `observe` takes `a` read-only and `b` as `mut`; `main`
passes the same allocation through both:

```wolf
fn observe(a: *u8, mut b: *u8) -> int {
    b[0] = 3
    a[0] as int
}
let p = c.malloc(8) as *u8
p[0] = 1
var q = p
let n = observe(p, mut q)
```

Predict where the oracle reports the fault — at the call, at the
write, or at the later read — and why "at the write" is the answer
that lets the optimizer trust `mut`.

Solution:

```console
$ lupin ex9-14.lu
ex9-14.lu: ub(mem.ub) §7/P1: foreign write at alloc#0[0] while tag#1 (parameter) is PROTECTED for a call's extent — the protector makes the invalidation UB at the write rather than at a later use [mem.prov.state] at 175..183; tag created at 329..330
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
      tag#1 parameter Frozen PROTECTED
      tag#2 parameter Reserved PROTECTED
$ echo $?
3
```

At the write. Parameter entry mints protected tags for the call's
whole extent, so the write through `b` is *foreign* to `a`'s protected
tag and is UB immediately — even though the conflicting read has not
happened yet. That timing is the license: if invalidation only counted
at a later use, the compiler could not fold loads through `a` across
the body, because the fault might never "arrive." Protectors make
`mut`'s promise airtight for exactly one call — which is the shape
`noalias` needs. The tag tree at the report's bottom is the whole
story in four lines; learning to read it is learning the model.
