# Chapter 9 — The escape hatch is a door, not a cliff: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; the UB oracle's reports are shown
in full — reading them is most of this chapter's homework.

## §9.1 — The three rings

**Exercise 9-1** *(fingers + spelunking · wolf)* — The complete unsafe audit
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
pointer operations, foreign calls, and aliasing assertions do not parse
outside the block, so the keyword is a complete index of the ring
boundary. In C the equivalent grep misses everything, because there is
no keyword: every pointer dereference in the program is potentially the
audit's subject, which is to say the audit is the whole program.

## §9.2 — Raw-tier rules

**Exercise 9-2** *(fingers · lupin)* — Your first unsafe block, kept
legal: allocate eight bytes from C, set them all to 5, read one back,
free, print. Type it, run it, and note the exit code — the point of
this exercise is that nothing happens.

Solution — `ch09/ex9-2.lu`:

```wolf
import c "stdlib.h"
fn main() -> !int {
    // # Safety: the eight bytes are written before they are read, every
    // access is in bounds, and the allocation is freed exactly once.
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
change, not a crime scene. Note the `# Safety:` line above it: the
compiler warns when it is missing, and it is the sentence a reviewer
checks the body against.

**Exercise 9-3** *(comprehension · lupin)* — One character changes in
9-2: the write is `p[8] = 1`. The allocation holds eight bytes.
Predict the oracle's finding — its row, and which optimizer license
the report will name.

Solution — the report, in full:

```console
$ lupin ex9-3.lu
ex9-3.lu: ub(mem.ub) §7/P3: write of 1 byte(s) at alloc#0[8], which holds 8 [mem.ub] at 350..358; tag created at 297..308
  licenses O3a: `dereferenceable(n)` on known-size accesses; bounds-based alias disproof between distinct allocations
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

Row P3, out of bounds — and the license line is the half worth reading
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
ex9-4.lu: ub(mem.ub) §7/L2: read through an exposed pointer into alloc#0, which was freed [mem.unsafe.raw.1] at 458..462; tag created at 317..328
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
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 333..337; tag created at 263..274
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ lupin ex9-5.lu
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 333..337; tag created at 263..274
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

**Exercise 9-6** *(comprehension · lupin)* — Allocate 64 bytes with
`c.malloc` and read one of them before anything writes it:

```wolf
let p = c.malloc(64) as *u8
let v = p[0] as int
c.free(p)
```

No free-before-use, no bounds problem, no aliasing. Predict whether
this is undefined behavior, which row it lands on, and what
optimization the row's license names.

Solution: it is UB, and the row is L1 — reading memory nothing has
written is on the list in its own right:

```console
$ lupin ex9-6.lu
ex9-6.lu: ub(mem.ub) §7/L1: read of alloc#0[0], which nothing has written [mem.ub] at 306..310; tag created at 270..282
  licenses O7: moves lower to memcpy-and-forget; dead-store elimination on moved-from places; no zero-init of locals
  alloc#0 `c.malloc(64)` 64 byte(s), live, owned by region #0
    tag#0 c.malloc(64)#root Active exposed
$ echo $?
3
```

The license is the explanation, as always. If reading uninitialized
memory had a defined answer, every local would have to be zeroed, a
move could not be a copy-and-forget, and a store to a place that is
about to be moved out of could not be deleted. The price of skipping
all three is that the bytes `malloc` hands back are not a value yet.
`c.calloc` is the call that makes them one.

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
ex9-7b.lu: ub(mem.ub) §7/P6: `borrow region #1 from` a pointer into alloc#0, which is owned by `program` (region #0) — the obligation is that the allocation lies wholly inside the named region's footprint [mem.unsafe.door] at 395..410; tag created at 334..345
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

**Exercise 9-8** *(fingers · wolf + lupin)* — Take §9.5's `pack` and
spell its second allocation the other way round: `c.calloc(1, bytes)`
instead of `c.calloc(bytes, 1)`. Both allocate the same number of
bytes. Run the program under the interpreter, then compile it and run
the binary. Report both outputs and say what a difference between them
would have meant.

Solution — `ch09/ex9-8.lu`, and there is no difference:

```console
$ lupin ex9-8.lu
64 bytes out, and not a pointer in sight
$ wolf build ex9-8.lu && ./ex9-8
64 bytes out, and not a pointer in sight
```

The first line is a model of the C heap; the second is glibc. A
difference between them would have meant one of two things, and both
are bugs: either a model of `calloc` disagrees with `calloc`, or the
compiler's membrane passes the arguments in the wrong order. The
history here is not hypothetical — `calloc(n, size)` is `n * size`
bytes, one of the models once made it `n`, and this is the shape of
program that found it. Running a program two ways and comparing is not
a testing technique bolted on afterward; it is what "one language, two
implementations" is *for*.

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
ex9-9.lu: ub(mem.ub) §7/P4: read at alloc#0[0], whose owning region #1 was freed wholesale [mem.prov.region] at 438..442; tag created at 341..352
  licenses O3b: one alias-scope domain per region — pointers into distinct regions never alias; O4: regions not open in the current scope yield `invariant.load`
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #1
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

Row P4, not P1: nobody called `free` — the *region* died, and it took
the allocation's permissions with it. Note the report's strange-
looking line: the allocation is still `live` (its bytes were never
individually freed) but its owning region is gone, and that is enough.
The rule C code linked into wolf must learn: memory borrowed while a
region was ambient is a loan *from the region*, and the region's
death calls it in, wholesale.

## §9.7 — Auditing: `#[trusted]` and the audit surface

**Exercise 9-10** *(spelunking · lupin)* — `ch09/ex9-10.lu` wraps its
unsafe block in a `#[trusted]` function carrying its obligation as a
string. Run it. Then answer from the chapter: what two questions about
this function does a manifest-and-inventory audit answer that reading
the function's source cannot?

Solution — the program runs like any other:

```console
$ lupin ex9-10.lu
64 bytes out, declared
$ echo $?
0
```

The two questions are *closure* and *drift*. Closure: reading `pack`
tells you what `pack` does, and tells you nothing about whether
anything it calls — or anything its dependencies call — is itself
trusted or holds unsafe rings of its own. The inventory answers for the
whole package, module by module, which is a question source review
answers one file at a time and therefore usually does not. Drift:
whether the roster is the same roster it was last release. Review is a
memory; the manifest is a diffable fact, and a dependency that grows a
trusted module has to grow a manifest line to do it. Neither question
is about whether `pack` is correct — nothing mechanical answers that,
which is exactly why the obligation is written in the attribute in
English, for a person.

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
ex9-13.lu: ub(mem.ub) §7/P5: `assume noalias` asserts these ranges are disjoint, and alloc#0[0..1) overlaps alloc#0[0..1) — the assertion is false [mem.unsafe.raw.2] at 337..356; tag created at 292..303
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
the chapter. Take §9.4's door program and add one line: after
`let counts = borrow scratch from p`, write `p[0] = 1` through the raw
pointer, and only then read `counts[0]`. Predict where the fault is
reported and what the tag tree at the bottom of the report will have
in it that no other report in this chapter has shown.

Solution — `ch09/ex9-14.lu`:

```console
$ lupin ex9-14.lu
ex9-14.lu: ub(mem.ub) §7/P1: read through tag#1 (`borrow … from`), which is Disabled at alloc#0[0] [mem.prov.state] at 478..487; tag created at 423..444
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #1
    tag#0 c.malloc(8)#root Active exposed
      tag#1 `borrow … from` Disabled|Reserved|Reserved|Reserved|Reserved|Reserved|Reserved|Reserved
$ echo $?
3
```

Two tags, one indented under the other: this is the first report in
the chapter with a tree in it rather than a single root. The door
minted `tag#1` as a child of the allocation's root tag, and that child
is what safe code was handed. Writing through `p` afterward is a write
through a tag that is *not* an ancestor of `tag#1` — foreign, in the
model's word — so `tag#1` goes Disabled at the byte that was written,
and the read through `counts` is P1. The per-byte spelling is the
detail worth noticing: only byte 0 was written, so only byte 0's
permission died, and the other seven are still Reserved. Permissions
are per location, not per pointer.

The general lesson is the reason the door exists. Once safe code holds
a value, the raw tier must stop touching those bytes — because
everything downstream is entitled to assume nobody is. Reaching around
the door is not a slightly risky shortcut; it is the one thing the
door's license forbids.
