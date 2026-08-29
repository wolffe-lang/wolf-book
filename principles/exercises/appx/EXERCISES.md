# Appendix exercises — the trap table and the diagnostic catalog

Two sets, numbered to their appendices: B-exercises work Appendix B's
trap table (twelve kinds, closed by `[conf.trap.set]`); C-exercises
work Appendix C's diagnostic catalog. Commands run from this
directory; outputs are pasted from real runs. Reference material is
deadpan by rule, and so are these.

## Appendix B — the trap zoo

The working rule for every B-exercise: before running, write down the
trap *kind* and the clause tag you expect in brackets. The table is
closed, so this is a twelve-way multiple choice with the answer key
printed in Appendix B.

**Exercise B-1** *(comprehension · lupin)* — `let big: i32 =
2147483647` then `big + 1`. Kind and clause?

```console
$ lupin exB-1.lu
exB-1.lu: trap(overflow): `+` produced 2147483648, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 6:13
```

`overflow`, `[arith.checked]`. The line also names the decision (X3)
and the intended-wraparound spelling; a trap line is a complete
citation or it is a defect.

**Exercise B-2** *(comprehension · lupin)* — `7 % d` where `d`
computed to zero. The pointed part: is remainder division?

```console
$ lupin exB-2.lu
exB-2.lu: trap(div-zero): division by zero is defined behavior in wolf: it traps [mem.ub.defined] at 7:13
```

`div-zero`, `[mem.ub.defined]` — remainder is division for this
purpose, and how the zero arrived is irrelevant (exercise 3-4 made
the same point with `/`).

**Exercise B-3** *(comprehension · lupin)* — `xs[1]` on a one-element
list.

```console
$ lupin exB-3.lu
exB-3.lu: trap(bounds): index 1 is outside a collection of 1 element(s) [mem.ub.defined] at 7:13
```

`bounds`, `[mem.ub.defined]` — one clause family covers list
indexing and byte slicing (exercise 2-6): defined fault, not UB.

**Exercise B-4** *(comprehension · lupin)* — `let t = move s` then
`print(s)`.

```console
$ lupin exB-4.lu
exB-4.lu: trap(use-after-move): `s` was moved out and is uninitialized here [mem.tier0.move.2] at 7:11; `s` moved here at 6:13
$ echo $?
3
```

`use-after-move`, `[mem.tier0.move.2]`, and both spans — the read
and the move it conflicts with. (The compiler's static verdict on
this rule is exercise C-2's subject.)

**Exercise B-5** *(comprehension · lupin)* — `wide(mut p.a, mut
p.a.n)`: a path and its own prefix, both claimed `mut`.

```console
$ lupin exB-5.lu
exB-5.lu: trap(exclusivity): `p.a.n` is accessed as `mut` while `p.a` is held as `mut`; the paths conflict [mem.model.path.disjoint] at 9:19; `p.a` held here at 9:10
```

`exclusivity`, `[mem.model.path.disjoint]` — the clause exercise 4-4
met as E1002's citation, enforced here at the second claim.

**Exercise B-6** *(comprehension · lupin)* — reserve from a pool
whose region was frozen.

```console
$ lupin exB-6.lu
exB-6.lu: trap(region-fault): region #1 is frozen: `imm` data is immutable forever [mem.region.freeze.1] at 9:13
```

`region-fault`, `[mem.region.freeze.1]` — the kind covers every
region operation its state forbids; freezing is the permanent state.

**Exercise B-7** *(comprehension · lupin)* — read through a handle
whose slot was removed.

```console
$ lupin exB-7.lu
exB-7.lu: trap(stale-handle): handle into pool#0 slot 0 carries generation 0, the slot is at generation 1; a stale handle is a deterministic fault in every profile, never UB [mem.shared.handle.2] at 11:9
```

`stale-handle`, `[mem.shared.handle.2]` — the generation arithmetic
is in the message: the handle names generation 0, the slot moved to
1, and the mismatch is the fault, deterministically, in every
profile (X5's contract).

**Exercise B-8** *(comprehension · lupin)* — `assert("wolf".len ==
claimed - 1)` with `claimed = 4`.

```console
$ lupin exB-8.lu
exB-8.lu: trap(assert): assertion failed [conf.trap.map] at 6:5
```

`assert`, `[conf.trap.map]` — the one trap you aim on purpose; its
clause lives in the conformance mapping, not the memory model,
because the fault is yours by construction.

**Exercise B-9** *(comprehension · lupin)* — `recv` on an unbuffered
channel in a one-task program.

```console
$ lupin exB-9.lu
exB-9.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0) [conc.deadlock.trap] at 7:13
```

`deadlock`, `[conc.deadlock.trap]`, with the blocked-task roster —
one task, blocked at a runtime-owned point, no timer pending. Note
what did not help: the `else` handler. It handles a channel's error
row (a closed channel); blocking forever is not an error value, it
is a fate, and the trap is what reports fates.

**Exercise B-10** *(comprehension · pending — blockers: `race` needs
the dynamic race machine observing a real conflict under exploration
(owner: s36-deterministic-scheduler and the interp's conc machine);
`alloc-contract` needs the wolf_rt quarantine allocator (owner:
s23-memory-conformance / wolf_rt))* — The two kinds this appendix
cannot yet demonstrate. State, from the table's own definitions, what
each would require a program to do, and why today's tools decline.

Solution (prose): `race` requires two tasks reaching the same place,
one writing, unordered by any happens-before edge — and today the
static capture rules plus the FIFO scheduler leave the probe corpus
race-free as observed (the store-buffer litmus runs deterministically
because closures capture by copy; its own header expects the *static*
rejection E1101, also pending). `alloc-contract` is the debug
quarantine allocator's fault kind — MTE-style tags, retag on free —
whose hooks are stubbed until s23's machinery lands. Both rows stand
in Appendix B with their clauses; the corpus holds the headers and
CI holds the appointment.

**Exercise B-11** *(comprehension · lupin)* — Through the membrane:
`malloc`, `memset`, launder the pointer through an `int`, `free`,
recast, read. Kind, and what extra evidence does this kind's report
carry that no other trap has?

```console
$ lupin exB-11.lu
exB-11.lu: ub(mem.ub) §7/L2: read through an exposed pointer into alloc#0, which was freed [mem.unsafe.raw.1] at 14:17; tag created at 9:17
  licenses O8: escape analysis / stack promotion without conservatively pinning addresses
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

`ub` — the oracle's finding, pinned as a trap expectation. The extra
evidence: a provenance report — the allocation's biography, the tag's
creation site and state, and the line naming which optimization the
finding licenses (O8). The other eleven kinds report a rule broken;
this one reports a *proof obligation* the unsafe block failed, which
is why its output reads like a lab result. Only the raw tier can earn
it; safe wolf cannot spell this program.

## Appendix C — the diagnostic catalog

**Exercise C-1** *(spelunking · wolf)* — Run `wolf --explain E1001`
and read it against any E1001 instance from this book (3-2's, or
C-2's below). Name two facts the catalog entry states that no single
instance shows.

Solution — the real entry:

```console
$ wolf --explain E1001
E1001: this value was moved away (or never given one) before this use

In wolf, assignment and argument passing *move* a value: after
`let b = a` or `f(take a)`, the name `a` no longer holds anything —
its value went to the new place, whole. Reading a moved-from (or
never-initialized) name would read nothing, so the checker stops it
here and points at the move it happened in. Moves are field-granular:
moving `s.a` away leaves `s.b` usable, and only the moved path is
off-limits. To keep using the original, make the duplication explicit
where the move happens — `copy a` produces an independent value of
any type — or give the name a new value first: assigning to a
moved-from place makes it live again.
```

Two catalog-only facts: the code also covers *never-initialized*
names, which no moved-value instance mentions; and the general
field-granularity rule — an instance shows one path blamed, the
entry states the rule that decides every path. Instance and entry
divide the labor: the diagnostic is the rule applied, `--explain` is
the rule stated.

**Exercise C-2** *(comprehension · wolf + lupin)* — The appendix's
one-rule-two-voices demonstration, minimal: a struct field moved by
`take`, then read. Predict both tools' outputs — code, kind, and the
clause families each cites:

```wolf
struct Den { name: str }
fn claim(take s: str) -> str { s }
fn main() -> !int {
    var d = Den { name: "howl" }
    let a = claim(take d.name)
    print(d.name)
    0
}
```

Solution — the compiler, before anything runs:

```console
$ wolf conform-run ./exC-2.lu
error[E1001]: `d.name` is used here after its value moved away
 --> ./exC-2.lu:9:11
  |
8 |     let a = claim(take d.name)
  |                        ------ `d.name` moved here
9 |     print(d.name)
  |           ^^^^^^ used after the move
  |
  = note: re-initializing the place (assigning to it) also makes it usable again.
help: to keep the original, copy it at the move
  |
8 |     let a = claim(take copy d.name)
  |
```

The interpreter, at the moment of the read:

```console
$ lupin exC-2.lu
exC-2.lu: trap(use-after-move): `d.name` was moved out and is uninitialized here [mem.tier0.move.2] at 9:11; `d.name` moved here at 8:19
$ echo $?
3
```

E1001 and `trap(use-after-move)` are Appendix C and Appendix B
holding the same rule — `[mem.tier0.move.*]` — at two moments. Same
blamed path, same conflicting-move span, one enforcement performed
twice. This differential is the book's spine; the appendices are
where its two halves file their paperwork.

**Exercise C-3** *(comprehension · prose)* — Five codes from this
book's own runs: E0202, E0303, E0705, E1001, E1101. From their
numbering and the chapters that produced them, name each one's
family and the phase that emits it, and state the convention the
first digits encode.

Solution: E02xx — grammar, from the parser (1-6's unclosed brace);
E03xx — modules and resolution (22-5's import cycle); E07xx —
comptime, from the sandbox and budget checks at typecheck (chapter
18 throughout); E10xx — the memory tier's static verdicts (C-2's
move); E11xx — concurrency's static verdicts (the store-buffer
litmus's expected rejection, pending). The convention: the leading
digits bucket codes by the compiler stage and rule family that owns
them, so a code's neighborhood tells you which appendix table,
which spec chapter, and which phase of `phase_reached` to consult
before you have read a word of the message. The catalog is
navigable arithmetic; that is a property this book's CI checks
(every code shown here is cross-checked against the compiler's
catalog), not a promise.
