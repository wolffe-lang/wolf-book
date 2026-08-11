# Chapter 7 — Who owns this?: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the wolf-lang debug build
at `impl_version 0.0.1`. Where the compiler cannot yet observe a
program (its runner lands at s31), the solution shows the observation
record it emits instead, and says so.

## §7.1 — The error we owed you

**Exercise 7-1** *(comprehension · wolf + lupin)* — Chapter 3's broken
`Pack` program, with one line added:

```wolf
struct Pack { lead: str, tail: str }
fn adopt(take w: str) -> str { w }
fn main() -> !int {
    var p = Pack { lead: "ada", tail: "grace" }
    let a = adopt(take p.lead)
    p.lead = "lin"
    let c = p.lead
    print("{a} {c} {p.tail}")
    0
}
```

Exercise 3-2's version was rejected with E1001 at `let c = p.lead`.
Predict both tools' behavior now, and name the sentence in the E1001
diagnostic you saw in chapter 3 that already told you the answer.

Solution: both accept it. Assigning to a moved-from place makes it
live again — the diagnostic's note said exactly that, and this is that
note performed. lupin runs the program; the compiler's mem checker
finds nothing to reject, and its observation record says so in two
fields — `phase_reached` is `mem` with an empty diagnostics list —
while the verdict stays `unsupported`, because a *pass* verdict would
require running the program and `wolf build|run` lands at s31. The
record is the honest shape of "statically fine, dynamically untested."

```console
$ lupin ex7-1.lu
ada lin grace
$ wolf conform-run ./ex7-1.lu
{"commit":"unknown","diagnostics":[],"file":"./ex7-1.lu","impl":"wolfc","impl_version":"0.0.1","phase_reached":"mem","protocol":1,"seeded":false,"stdout_inline":null,"stdout_sha256":null,"verdict":"unsupported"}
```

## §7.2 — Values are trees

**Exercise 7-2** *(fingers · lupin)* — Draw the ownership tree of `den`
below before running anything: one box per value, one arrow per field.
Then move the deepest leaf out with `move` and verify, by printing
them, that the leaf's *sibling* and its *cousins* are all still
usable:

Solution — `ch07/ex7-2.lu`:

```wolf
struct Wolf { name: str, call: str }
struct Den { alpha: Wolf, beta: Wolf }
fn main() -> !int {
    var den = Den {
        alpha: Wolf { name: "ada", call: "awoo" },
        beta: Wolf { name: "grace", call: "yip" },
    }
    let promoted = move den.alpha.name
    print("{promoted} leads")
    print("{den.alpha.call} still works")
    print("{den.beta.name} still here")
    0
}
```

```console
$ lupin ex7-2.lu
ada leads
awoo still works
grace still here
```

The move emptied exactly one path — `den.alpha.name` — and the tree's
other four leaves never noticed. (`move` is the plain-expression
spelling; `take` is the same act written at a call site.)

**Exercise 7-3** *(extension (break-it-on-purpose) · wolf + lupin)* — Using one
struct, one function taking `take`, and nothing else, write the
smallest program that traps `use-after-move` *through a field*. Predict
the compiler's E-code and the interpreter's trap kind before checking
both. Why does the exercise say "through a field" — what would be
different, and what the same, with a bare local?

Solution — `ch07/ex7-3.lu`:

```wolf
struct S { a: str }
fn eat(take w: str) -> str { w }
fn main() -> !int {
    var s = S { a: "x" }
    let t = eat(take s.a)
    let u = s.a
    print("{t} {u}")
    0
}
```

```console
$ wolf conform-run ./ex7-3.lu
error[E1001]: `s.a` is used here after its value moved away
 --> ./ex7-3.lu:9:13
  |
8 |     let t = eat(take s.a)
  |                      --- `s.a` moved here
9 |     let u = s.a
  |             ^^^ used after the move
  |
  = note: re-initializing the place (assigning to it) also makes it usable again.
help: to keep the original, copy it at the move
  |
8 |     let t = eat(take copy s.a)
  |
```

```console
$ lupin ex7-3.lu
ex7-3.lu: trap(use-after-move): `s.a` was moved out and is uninitialized here [mem.tier0.move.2] at 236..239; `s.a` moved here at 214..222
$ echo $?
3
```

A bare local would produce the same E-code and the same trap kind; the
field version is the stronger test because it proves the tracking is
per-path, not per-variable — `s` as a whole is neither dead nor alive,
only `s.a` is dead. Both tools cite the same clause family,
`[mem.tier0.move.2]`, which is the differential spine again.

**Exercise 7-4** *(comprehension · lupin)* — Every field of `P` is an
`int`. Predict what the second line of `main` does to `a`:

```wolf
struct P { x: int, y: int }
fn main() -> !int {
    let a = P { x: 1, y: 2 }
    let b = a
    print("{a.x} {b.y}")
    0
}
```

Solution: it moves `a`, ints and all — the trap blames `a.x`'s read and
points at `let b = a`. Structs move on assignment whatever they
contain; there is no "cheap enough to copy silently" tier for
user-defined types. The fix spells the duplication where it happens:

```console
$ lupin ex7-4.lu
ex7-4.lu: trap(use-after-move): `a.x` was moved out and is uninitialized here [mem.tier0.move.2] at 233..236; `a` moved here at 219..220
```

`ch07/ex7-4b.lu` changes one word — `let b = copy a` — and:

```console
$ lupin ex7-4b.lu
1 2
```

The wrong answer worth ruling out: "ints are `Copy`, so the struct
copies." Wolf's rule is per-*decision*, not per-*type*: the reader of
`let b = copy a` knows a duplication happened without looking up what
`P` contains.

## §7.3 — Borrowing without the word

**Exercise 7-5** *(comprehension · lupin; static verdict pending —
blocker: E1003 borrow-escape checking reaches no verdict while wolfc
leaves `channel` unresolved; owner: s33-channels-select, check
s18-tier0-exclusivity)* — A parameter is borrowed for the call; a
local borrow `&x` lives inside its function's activation. This program
tries to make one outlive it:

```wolf
fn main() -> !int {
    let ch = channel[int](1)
    let x = 41
    let p = &x
    ch.send(p)
    0
}
```

The spec's answer is rejection — E1003, "borrow escapes activation."
lupin runs this program and exits 0. Before checking either claim,
answer: is lupin *wrong*?

Solution: lupin is not wrong; it is answering a different question. In
this particular run the borrow went into the channel and the program
ended before anything read it dangling — dynamically, no fault
occurred. The static rule exists because some *other* run (a receiver
on another task, a longer-lived channel) would read the borrow after
`main`'s frame is gone, and a rule that only fails sometimes is not a
rule callers can build on. The conservative tier rejects the *shape*;
the dynamic tier faults the *event*. When the event never happens, the
conservatism is visible — that is the price, paid on purpose.

```console
$ lupin ex7-5.lu
$ echo $?
0
```

Today wolfc reports `unsupported` at resolve on this file (channels are
s33); the directive header carries the expected E1003 so CI catches the
day the verdict arrives.

## §7.4 — `mut` at both ends

**Exercise 7-6** *(fingers + spelunking · lupin)* — Write `swap` for
two `int`s using `mut` at both ends, and verify it. Then state the
single search you would run over a strange codebase to find every line
that can mutate anything — and what property of the language makes the
search complete.

Solution — `ch07/ex7-6.lu`:

```wolf
fn swap(mut a: int, mut b: int) {
    let t = a
    a = b
    b = t
}
fn main() -> !int {
    var x = 1
    var y = 3
    swap(mut x, mut y)
    print("{x} {y}")
    0
}
```

```console
$ lupin ex7-6.lu
3 1
```

The search is `grep '(mut '` (X1's argument, from exercise 4-3, now
stated as a rule): call-site `mut` is mandatory, so a call that can
write through an argument *says so at the call*. Add `grep 'var '` for
locals and the audit is the whole mutation surface — two searches, no
false negatives, which is what "required at both ends" buys.

## §7.5 — Field-granular exclusivity

**Exercise 7-7** *(comprehension · wolf + lupin)* — The simplest
possible exclusivity violation: one place, claimed twice.

```wolf
fn bump2(mut a: int, mut b: int) {
    a += 1
    b += 1
}
fn main() -> !int {
    var n = 0
    bump2(mut n, mut n)
    n
}
```

Predict what each tool says, then answer the design question hiding
under it: if the call *were* allowed, what would `n` be afterward — and
why is "it depends on the body" the real reason for the rule?

Solution: wolf rejects, lupin traps, same rule:

```console
$ wolf conform-run ./ex7-7.lu
error[E1002]: `n` cannot go `mut` here: it overlaps `n`, already passed `mut` in this call
  --> ./ex7-7.lu:10:22
   |
10 |     bump2(mut n, mut n)
   |               - `n` is passed `mut` here
   |                      ^ second exclusive claim on the same place
   |
   = note: the same place twice is never disjoint.
```

```console
$ lupin ex7-7.lu
ex7-7.lu: trap(exclusivity): `n` is accessed as `mut` while `n` is held as `mut`; the paths conflict [mem.tier0.excl.1] at 215..220; `n` held here at 208..213
```

If allowed, `n` could be 1 or 2 depending on whether `a` and `b` are
distinct copies written back in some order or two names for one cell —
the body decides, and the caller cannot see the body. Exclusivity makes
the answer not depend on the body: two `mut` claims must be provably
disjoint places, so aliasing questions are settled at the call site,
which is also what lets the compiler hand `noalias` facts to the
optimizer (§7.7's subject).

**Exercise 7-8** *(comprehension + fingers · lupin)* — Four call shapes against
`struct P { a: Q, b: Q }`, `struct Q { n: int }`. Verdict for each,
before checking any:

1. `f(mut p.a, mut p.b)`
2. `f(mut p.a.n, mut p.b.n)`
3. `f(mut p.a, mut p.a.n)`
4. `f(mut p, mut p.b)`

Solution: 1 and 2 are legal — disjoint fields, and leaves of disjoint
subtrees. 3 and 4 are rejected: in each, one path is a *prefix* of the
other, and a place conflicts with every place inside it. The rule from
exercise 4-4, one sentence: two paths conflict iff one is a prefix of
the other (`[mem.model.path.disjoint]`). The legal pair, run:

```wolf
fn bump(mut u: int, mut v: int) {
    u += 1
    v += 1
}
var p = P { a: Q { n: 5 }, b: Q { n: 7 } }
bump(mut p.a.n, mut p.b.n)
```

```console
$ lupin ex7-8.lu
6 8
```

## §7.6 — Why there are no lifetimes

**Exercise 7-9** *(spelunking · wolf)* — Run `wolf --explain E1001` and
read all of it. Quote the sentence that licenses re-initialization
(exercise 7-1's move), the phrase that states field granularity
(exercise 7-3's), and the one word in the first paragraph that makes
`let b = a` and `f(take a)` the same subject.

Solution:

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

The license is the last clause: "assigning to a moved-from place makes
it live again." The granularity phrase is "moving `s.a` away leaves
`s.b` usable." The one word is "move" itself — the text's first
sentence puts assignment and argument passing under the same verb,
which is why chapters 3 and 7 have been describing one mechanism, not
two.

**Exercise 7-10** *(design)* — Rust's zero-copy parser hands out `&str`
slices of an input buffer it does not own, with lifetimes proving the
buffer outlives every slice. Wolf has no lifetime annotations, so that
API shape is not expressible for arbitrary callers. Sketch the wolf
alternatives — copying the token text, returning byte ranges
`(start, end)` into a caller-held string, or parsing inside a region
and freezing the result — and argue which one a tokenizer library
should ship. What does each cost, and who pays it?

Solution (discussion): the range API is the honest default: tokens as
`(start, end)` pairs are plain values, move freely, and cost eight
bytes each; the caller pays one indirection — `input[t.0..t.1]` — at
each use, checked. Copying pays allocation per token to buy the
simplest caller code; for a config-file parser nobody measures, that
is the right trade, and for a log-ingest loop it is not. The
freeze design is the interesting one: parse into a region, freeze it,
and hand back *imm* tokens that reference the frozen input — sharing
without copies and without lifetimes, at the cost of making the input
immutable forever and region-resident from the start; it fits a
compiler front end, where the source text never changes after load.
The library should ship ranges and let the other two be five-line
wrappers, because ranges are the only shape that never dictates the
caller's memory story. What Rust buys with lifetime annotations is
making the borrow design *default*; what wolf buys by refusing them is
that no signature in this paragraph mentions anything but values.

## §7.7 — What the machine does

**Exercise 7-11** *(fingers · lupin REPL)* — In the REPL, move a string
out of one binding into another, then read both — the corpse first.
What does the session do that a compiled program cannot, and which
clause tag names the reason the trap did not end your session?

Solution — one session:

```console
$ lupin
wolf> let s = "wolf"
wolf> let t = move s
wolf> s
trap(use-after-move): `s` was moved out and is uninitialized here [mem.tier0.move.2] at 0..1
  `s` moved here at 8..14
the session survives the trap; the world is as the fault left it [repl.trap.alive]
wolf> t
wolf : str
wolf> :quit
```

The session takes the trap and keeps the world — `[repl.trap.alive]` —
so the state a fault left behind is inspectable, which is the REPL's
whole advantage over a crashed process. The value is intact in `t`:
a move is a transfer, never a destruction, and the machine-level story
(§7.7) is a memcpy after which the source is *forgotten*, not zeroed.

## Chapter batch

**Exercise 7-12** *(extension · lupin)* — The longest common
subsequence of two line lists is the skeleton every diff tool hangs
on. Build the DP table as a `List[List[int]]` and return its corner.
For the two three-line "files" in the solution, compute the answer on
paper first: which two lines survive in both?

Solution — `ch07/ex7-12.lu` (core):

```wolf
fn lcs_len(a: List[str], b: List[str]) -> int {
    var table = List[List[int]]()
    var i = 0
    while i <= a.len {
        var row = List[int]()
        var j = 0
        while j <= b.len {
            row.push(0)
            j += 1
        }
        table.push(row)
        i += 1
    }
    i = 1
    while i <= a.len {
        var j = 1
        while j <= b.len {
            if a[i - 1] == b[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1
            } else if table[i - 1][j] >= table[i][j - 1] {
                table[i][j] = table[i - 1][j]
            } else {
                table[i][j] = table[i][j - 1]
            }
            j += 1
        }
        i += 1
    }
    table[a.len][b.len]
}
```

```console
$ lupin ex7-12.lu
2
```

"the wolf runs" and "the elk listens" survive; the moon line does not.
Note what the function signature says about ownership: both lists are
borrowed — the caller keeps them, un-moved, and no annotation was
spent saying so.

**Exercise 7-13** *(comprehension + extension · lupin)* — Extend 7-12
into a printing diff: walk the finished table backward from the corner,
emitting `  ` for common lines, `- ` for deletions, `+ ` for
additions. Before running, predict the full output for `old` = the
wolf/moon/elk lines and `new` = wolf/elk/river. Then explain why the
walk must go *backward*.

Solution — `ch07/ex7-13.lu` (the walk):

```wolf
fn print_diff(a: List[str], b: List[str], table: List[List[int]], i: int, j: int) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        print_diff(a, b, table, i - 1, j - 1)
        print("  {a[i - 1]}")
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        print_diff(a, b, table, i, j - 1)
        print("+ {b[j - 1]}")
    } else if i > 0 {
        print_diff(a, b, table, i - 1, j)
        print("- {a[i - 1]}")
    }
}
```

```console
$ lupin ex7-13.lu
  the wolf runs
- the moon watches
  the elk listens
+ the river answers
```

The table's cell `(i, j)` only knows the best answer *up to* that
point; which choice produced it is recoverable only by comparing a
cell with its neighbors, and the neighbors that explain `(i, j)` are
behind it. The recursion runs to the origin and prints on the way
back out, so the output comes out forward — backward walk, forward
story.
