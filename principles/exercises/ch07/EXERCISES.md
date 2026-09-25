# Chapter 7 — Who owns this?: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the wolf-lang debug build
at `impl_version 0.0.1`. Where the compiler cannot yet observe a
program (its runner lands at s31), the solution shows the observation
record it emits instead, and says so.

Numbered in order of appearance since bs52 (EXERCISES.md §1 records the
exception). The map, old to new: 4-3 → 7-6; 7-6 → 7-7; 7-7 → 7-8;
7-8 → 7-9; 7-9 → 7-10; 7-10 → 7-11; 7-11 → 7-12; 7-17 → 7-13;
7-18 → 7-14; 7-12 → 7-15; 7-13 → 7-16; 7-14 → 7-17; 7-15 → 7-18;
7-16 → 7-19. 7-1 … 7-5 did not move, and 7-5 is still held.

## §7.1 — The error we owed you

**Exercise 7-1** *(comprehension · wolf + lupin)*. Chapter 3's broken
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
live again: the diagnostic's note said exactly that, and this is that
note performed. Both implementations run it, and they print the same
line, which is the differential doing its ordinary job:

```console
$ lupin ex7-1.lu
ada lin grace
```

The compiler has one thing to add, and it is about the signature rather
than the move: `adopt` takes a value only to hand it straight back, so
`W1003` asks whether the `take` earns itself. The binary is produced
anyway and prints `ada lin grace`.

## §7.2 — Values are trees

**Exercise 7-2** *(fingers · lupin)*. Given
`struct Wolf { name: str, call: str }` and
`struct Den { alpha: Wolf, beta: Wolf }`, draw the ownership tree of a
`Den` before running anything: one box per value, one arrow per field.
Then move the deepest leaf out with `move` and verify, by printing them,
that the leaf's sibling and its cousins are all still usable.

Solution. `ch07/ex7-2.lu`:

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

The move emptied exactly one path (`den.alpha.name`) and the tree's
other four leaves never noticed. (`move` is the plain-expression
spelling; `take` is the same act written at a call site.)

**Exercise 7-3** *(extension (break-it-on-purpose) · wolf + lupin)*. Using one
struct, one function taking `take`, and nothing else, write the
smallest program that traps `use-after-move` *through a field*. Predict
the compiler's E-code and the interpreter's trap kind before checking
both. Why does the exercise say "through a field": what would be
different, and what the same, with a bare local?

Solution. `ch07/ex7-3.lu`:

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
warning[W1003]: `w` is taken, never touched, and returned
 --> ./ex7-3.lu:5:8
  |
5 | fn eat(take w: str) -> str { w }
  |        ^^^^ consumption that consumes nothing
  |
  = note: the caller gives the value up only to receive it back; if callers could reasonably keep
    it, the signature is wrong.
help: drop the `take` and hand back `copy w` (call sites drop theirs and keep their binding; a `read` parameter returned without the `copy` is E1002, #366)
  |
5 | fn eat(w: str) -> str { w }
  |

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

Two diagnostics, and the order is the compiler's rather than the
exercise's. `eat` takes a value and hands it straight back, which is
§7.2's W1003 in its own right — a consumption that consumes nothing —
and it is reported first because it is found first. The E1001 the
exercise is about is the second. Leaving the warning off this page
would make the page shorter and the transcript false; a console block
here is replayed against the pinned tools byte for byte.

```console
$ lupin ex7-3.lu
ex7-3.lu: trap(use-after-move): `s.a` was moved out and is uninitialized here [mem.tier0.move.2] at 9:13; `s.a` moved here at 8:17
$ echo $?
3
```

A bare local would produce the same E-code and the same trap kind; the
field version is the stronger test because it proves the tracking is
per-path, not per-variable: `s` as a whole is neither dead nor alive,
only `s.a` is dead. Both tools cite the same clause family,
`[mem.tier0.move.2]`, which is the differential spine again.

**Exercise 7-4** *(comprehension · lupin)*. Every field of `P` is an
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

Then change one word so it prints `1 2`, and say what the changed
program costs that the original did not.

Solution: it moves `a`, ints and all: the trap blames `a.x`'s read and
points at `let b = a`. Structs move on assignment whatever they
contain; there is no "cheap enough to copy silently" tier for
user-defined types. The fix spells the duplication where it happens:

```console
$ lupin ex7-4.lu
ex7-4.lu: trap(use-after-move): `a.x` was moved out and is uninitialized here [mem.tier0.move.2] at 8:13; `a` moved here at 7:13
```

`ch07/ex7-4b.lu` changes one word (`let b = copy a`) and:

```console
$ lupin ex7-4b.lu
1 2
```

The wrong answer is "ints are `Copy`, so the struct copies." Wolf's
rule is per-*decision*, not per-*type*: the reader of
`let b = copy a` knows a duplication happened without looking up what
`P` contains. That is also the cost the question asks for: a second
`P`, made at the line that says `copy`. The original made none, because
a move duplicates nothing a reader needs to account for.

## §7.3 — Borrowing without the word

**Exercise 7-5** *(comprehension · lupin; static verdict pending —
blocker: E1003 borrow-escape checking reaches no verdict while wolfc
leaves `channel` unresolved; owner: s33-channels-select, check
s18-tier0-exclusivity)*. A parameter is borrowed for the call; a
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

The spec's answer is rejection: E1003, "borrow escapes activation."
lupin runs this program and exits 0. Before checking either claim,
answer: is lupin *wrong*?

Solution: no. lupin is answering a different question. In
this particular run the borrow went into the channel and the program
ended before anything read it dangling: dynamically, no fault
occurred. The static rule exists because some *other* run (a receiver
on another task, a longer-lived channel) would read the borrow after
`main`'s frame is gone, and a rule that only fails sometimes is not a
rule callers can build on. The conservative tier rejects the *shape*;
the dynamic tier faults the *event*. When the event never happens, the
conservatism is visible. That is the price, paid on purpose.

```console
$ lupin ex7-5.lu
$ echo $?
0
```

wolfc reaches no verdict on this file (`unsupported` at resolve); the
directive header records the expected E1003, and CI enforces the
directive.

## §7.4 — `mut` at both ends

**Exercise 7-6** *(extension · wolf + lupin)*. A bounded buffer:
`struct Buf { items: List[int], cap: int }`. Write `push(mut b, x)`,
which adds `x` and returns the new length, or the error row `{full}`
when `b` already holds `cap` items; and `grow(mut b, extra)`, which
builds a larger `Buf` and moves the old list into it. Fill a buffer of
two, watch the third push fail, grow it by two, and push again. The
`cap` is yours because the list's is not: a wolf `List` grows on its
own and exposes no capacity to read or reserve. How many elements does
`grow` copy?

Solution. `ch07/ex7-6.lu`:

```wolf
struct Buf { items: List[int], cap: int }
fn push(mut b: Buf, x: int) -> int ! {full} {
    if b.items.len == b.cap { return full }
    (mut b.items).push(x)
    b.items.len
}
fn grow(mut b: Buf, extra: int) {
    let bigger = Buf { items: move b.items, cap: b.cap + extra }
    b = bigger
}
fn main() -> !int {
    var b = Buf { items: List[int](), cap: 2 }
    let first = push(mut b, 10) else -1
    let second = push(mut b, 20) else -1
    let third = push(mut b, 30) else -1
    print("{first} {second} {third}")
    grow(mut b, 2)
    let again = push(mut b, 30) else -1
    print("{again} of {b.cap}: {b.items[0]} {b.items[1]} {b.items[2]}")
    0
}
```

```console
$ lupin ex7-6.lu
1 2 -1
3 of 4: 10 20 30
$ wolf run ex7-6.lu
1 2 -1
3 of 4: 10 20 30
```

None. `move b.items` hands the list to the new `Buf` whole: the move
copies the list's top-level words (§7.7's pointer, length and
capacity) and no element, and it empties `b.items`. `b = bigger`
then fills the emptied place, which §7.2 said assignment does, so `b`
is a working name again by the time `grow` returns. Write `copy
b.items` instead and the program prints the same two lines while
duplicating every element to get there. The one word is the whole
difference in cost, and it is written where the cost is paid.

`push` answers `full` rather than trapping because a full buffer is
not a defect in the program; it is a state the caller decides about,
which is chapter 6's rule, and `else -1` is the smallest decision. The
bound is a field because it has to be: the list underneath keeps its
own capacity and grows it without asking, so the only limit a wolf
program can hold is one it stores itself.

**Exercise 7-7** *(fingers + spelunking · lupin)*. Write `swap` for two
`int`s using `mut` at both ends, and verify it. Then state the single
search you would run over a strange codebase to find every line that can
mutate anything, and what property of the language makes the search
complete.

Solution. `ch07/ex7-7.lu`:

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
$ lupin ex7-7.lu
3 1
```

The search is `grep '(mut '` (X1's argument, stated as a rule):
call-site `mut` is mandatory, so a call that can write through an
argument *says so at the call*. Add `grep 'var '` for
locals and the audit is the whole mutation surface: two searches, no
false negatives, which is what "required at both ends" buys.

## §7.5 — Field-granular exclusivity

**Exercise 7-8** *(comprehension · wolf + lupin)*. The simplest
possible exclusivity violation is one place claimed twice:
`bump2(mut n, mut n)` where `bump2` takes two `mut int`s. Predict what
each tool says, then answer the design question hiding under it: if the
call *were* allowed, what would `n` be afterward, and why is "it
depends on the body" the real reason for the rule?

Solution. `ch07/ex7-8.lu`:

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

wolf rejects, lupin traps, same rule:

```console
$ wolf conform-run ./ex7-8.lu
error[E1002]: `n` cannot go `mut` here: it overlaps `n`, already passed `mut` in this call
  --> ./ex7-8.lu:10:22
   |
10 |     bump2(mut n, mut n)
   |               - `n` is passed `mut` here
   |                      ^ second exclusive claim on the same place
   |
   = note: the same place twice is never disjoint.
```

```console
$ lupin ex7-8.lu
ex7-8.lu: trap(exclusivity): `n` is accessed as `mut` while `n` is held as `mut`; the paths conflict [mem.tier0.excl.1] at 10:18; `n` held here at 10:11
```

If allowed, `n` could be 1 or 2 depending on whether `a` and `b` are
distinct copies written back in some order or two names for one cell:
the body decides, and the caller cannot see the body. Exclusivity makes
the answer not depend on the body: two `mut` claims must be provably
disjoint places, so aliasing questions are settled at the call site,
which is also what lets the compiler hand `noalias` facts to the
optimizer (§7.7's subject).

**Exercise 7-9** *(comprehension + spelunking · wolf + lupin)*. Four
call shapes against `struct P { a: Q, b: Q }`, `struct Q { n: int }`.
In each, `f` takes both of its arguments `mut` and writes through both,
so its signature is the two argument types with `mut` on each: shape 1
calls `fn f(mut x: Q, mut y: Q)`, shape 2 `fn f(mut u: int, mut v:
int)`. Verdict for each, before checking any:

1. `f(mut p.a, mut p.b)`
2. `f(mut p.a.n, mut p.b.n)`
3. `f(mut p.a, mut p.a.n)`
4. `f(mut p, mut p.b)`

Then check yourself against the compiler, and find the sentence in
`wolf --explain E1002` that decides all four.

Solution: 1 and 2 are legal: disjoint fields, and leaves of disjoint
subtrees. 3 and 4 are rejected: in each, one path is a *prefix* of the
other, and a place conflicts with every place inside it. The legal
pair, with every parameter written, `ch07/ex7-9.lu`:

```wolf
fn f1(mut x: Q, mut y: Q) {
    x.n += 1
    y.n += 1
}
fn f2(mut u: int, mut v: int) {
    u += 10
    v += 10
}
var p = P { a: Q { n: 5 }, b: Q { n: 7 } }
f1(mut p.a, mut p.b)
f2(mut p.a.n, mut p.b.n)
```

```console
$ lupin ex7-9.lu
16 18
$ wolf run ex7-9.lu
16 18
```

The other two, `ch07/ex7-9b.lu`, in one program: `f3` takes a `Q` and
an `int`, `f4` a `P` and a `Q`, and both write both parameters, so the
compiler has nothing to say about them except the overlap:

```console
$ wolf conform-run ./ex7-9b.lu
error[E1002]: `p.a.n` cannot go `mut` here: it overlaps `p.a`, already passed `mut` in this call
  --> ./ex7-9b.lu:17:21
   |
17 |     f3(mut p.a, mut p.a.n)
   |            --- `p.a` is passed `mut` here
   |                     ^^^^^ second exclusive claim on the same place
   |
   = note: `p.a.n` is inside `p.a` — a path and its prefix conflict [mem.model.path.disjoint].
     Disjoint fields (`x.a` with `x.b`) are fine together.

error[E1002]: `p.b` cannot go `mut` here: it overlaps `p`, already passed `mut` in this call
  --> ./ex7-9b.lu:18:19
   |
18 |     f4(mut p, mut p.b)
   |            - `p` is passed `mut` here
   |                   ^^^ second exclusive claim on the same place
   |
   = note: `p.b` is inside `p` — a path and its prefix conflict [mem.model.path.disjoint]. Disjoint
     fields (`x.a` with `x.b`) are fine together.
```

The compiler reports both calls; the interpreter stops at the first,
because it finds the conflict by running into it:

```console
$ lupin ex7-9b.lu
ex7-9b.lu: trap(exclusivity): `p.a.n` is accessed as `mut` while `p.a` is held as `mut`; the paths conflict [mem.model.path.disjoint] at 17:17; `p.a` held here at 17:8
```

The sentence in `wolf --explain E1002` is the second of its first
paragraph: "Distinct fields are distinct places — `f(mut p.x, mut p.y)` is
fine — but `f(mut p, p.x)` is not, because `p.x` lives inside `p`."
Shapes 1 and 2 are its first half, and 3 and 4 its second, with the
second argument written `mut` rather than read, which conflicts for the
same reason. The clause both tools cite, `[mem.model.path.disjoint]`,
is the same rule in one line: two paths conflict iff one is a prefix
of the other.

## §7.6 — Why there are no lifetimes

**Exercise 7-10** *(spelunking · wolf)*. Run `wolf --explain E1001` and
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
`s.b` usable." The one word is "move" itself: the text's first
sentence puts assignment and argument passing under the same verb,
which is why chapters 3 and 7 have been describing one mechanism, not
two.

**Exercise 7-11** *(design)*. Rust's zero-copy parser hands out `&str`
slices of an input buffer it does not own, with lifetimes proving the
buffer outlives every slice. Wolf has no lifetime annotations, so that
API shape is not expressible for arbitrary callers. Sketch the wolf
alternatives (copying the token text, returning byte ranges
`(start, end)` into a caller-held string, or parsing inside a region
and freezing the result) and argue which one a tokenizer library
should ship. What does each cost, and who pays it?

Solution (discussion): the range API is the honest default: tokens as
`(start, end)` pairs are plain values, move freely, and cost eight
bytes each; the caller pays one indirection (`input[t.0..t.1]`) at
each use, checked. Copying pays allocation per token to buy the
simplest caller code; for a config-file parser nobody measures, that
is the right trade, and for a log-ingest loop it is not. The
freeze design is the interesting one: parse into a region, freeze it,
and hand back *imm* tokens that reference the frozen input: sharing
without copies and without lifetimes, at the cost of making the input
immutable forever and region-resident from the start; it fits a
compiler front end, where the source text never changes after load.
The library should ship ranges and let the other two be five-line
wrappers, because ranges are the only shape that never dictates the
caller's memory story. What Rust buys with lifetime annotations is
making the borrow design *default*; what wolf buys by refusing them is
that no signature in this paragraph mentions anything but values.

## §7.7 — What the machine does

**Exercise 7-12** *(fingers · lupin REPL)*. In the REPL, move a string
out of one binding into another, then read both, the corpse first. What
does the session do that a compiled program cannot, and which clause tag
names the reason the trap did not end your session?

Solution. One session:

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

The session takes the trap and keeps the world (`[repl.trap.alive]`),
so the state a fault left behind is inspectable, which is the REPL's
whole advantage over a crashed process. The value is intact in `t`:
a move is a transfer, never a destruction, and the machine-level story
(§7.7) is a memcpy after which the source is *forgotten*, not zeroed.

## §7.8 — Deciding at run time

**Exercise 7-13** *(extension · lupin)*. Add a third shape to the
`Draw` example, and then make `render` count its calls: what has to
change, and what does not?

Solution. `ch07/ex7-13.lu`, and the third impl is one line:

```wolf
trait Draw {
    fn draw(self) -> str
}
struct Dot { x: int }
struct Ring { r: int }
struct Star { points: int }
impl Draw for Dot { fn draw(self) -> str { "dot at {self.x}" } }
impl Draw for Ring { fn draw(self) -> str { "ring of {self.r}" } }
impl Draw for Star { fn draw(self) -> str { "star of {self.points}" } }
fn render(o: dyn Draw) -> str { o.draw() }
fn main() -> !int {
    let d = Dot { x: 3 }
    let r = Ring { r: 9 }
    let s = Star { points: 5 }
    var calls = 0
    print(render(d as dyn Draw))
    calls = calls + 1
    print(render(r as dyn Draw))
    calls = calls + 1
    print(render(s as dyn Draw))
    calls = calls + 1
    print("{calls} renders")
    0
}
```

```console
$ lupin ex7-13.lu
dot at 3
ring of 9
star of 5
3 renders
```

What changed: one struct, one impl, one binding, one call. What did
not: `render`. That is erasure earning its keep: the function that
takes `dyn Draw` never learns how many implementors exist. The counter
lives at the call sites, because `render` has nowhere to keep state.
It reads its argument through the pair and owns nothing, which is
§7.8's rule seen from the callee's side.

**Exercise 7-14** *(design)*. The cast-a-binding rule exists because
the dyn pair points at its operand rather than owning it. What would
the language have to invent for `Dot { x: 3 } as dyn Draw` to be legal,
and who would pay for it?

Solution. The temporary needs a home that outlives the expression, so
the language would have to invent one: a hidden allocation (a box the
reader never wrote), or a compiler-synthesized binding with a lifetime
the reader never chose. Both are costs paid silently, and wolf's
temperament is that erasure may change dispatch but never ownership:
the pair points at your value, in your frame or your region, and the
`let home = …` the error asks for is the language declining to
allocate behind your back. The reader pays one visible line; the
alternative is every reader paying an invisible allocation.

Exercises 7-13 and 7-14 were chapter 5's 5-9 and 5-10 before `dyn` and
the cast-a-binding rule moved into this chapter.

## Chapter batch

**Exercise 7-15** *(extension · lupin)*. The longest common
subsequence of two line lists is the skeleton every diff tool hangs
on. Build the DP table as a `List[List[int]]` and return its corner.
For the two three-line "files" in the solution, compute the answer on
paper first: which two lines survive in both?

Solution. `ch07/ex7-15.lu` (core):

```wolf
fn lcs_len(a: List[str], b: List[str]) -> int {
    var table = List[List[int]]()
    var i = 0
    while i <= a.len {
        var row = List[int]()
        var j = 0
        while j <= b.len {
            (mut row).push(0)
            j += 1
        }
        (mut table).push(row)
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
$ lupin ex7-15.lu
2
```

"the wolf runs" and "the elk listens" survive; the moon line does not.
Note what the function signature says about ownership: both lists are
borrowed. The caller keeps them, un-moved, and no annotation was
spent saying so.

**Exercise 7-16** *(comprehension + extension · lupin)*. Extend 7-15
into a printing diff: walk the finished table backward from the corner,
emitting `  ` for common lines, `- ` for deletions, `+ ` for
additions. Before running, predict the full output for `old` = the
wolf/moon/elk lines and `new` = wolf/elk/river. Then explain why the
walk must go *backward*.

Solution. `ch07/ex7-16.lu` (the walk):

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
$ lupin ex7-16.lu
  the wolf runs
- the moon watches
  the elk listens
+ the river answers
```

The table's cell `(i, j)` only knows the best answer *up to* that
point; which choice produced it is recoverable only by comparing a
cell with its neighbors, and the neighbors that explain `(i, j)` are
behind it. The recursion runs to the origin and prints on the way
back out, so the output comes out forward.

**Exercise 7-17** *(fingers · lupin)*. The plane-geometry kata: a
`Point`, a `Rect` of two points (low corner in, high corner out), and
an `impl` giving `Rect` three methods — `contains(self, p)`,
`overlaps(self, o)`, `area(self)`. Probe the edges: a point on the low
edge, a point on the high edge, a rectangle that shares only a corner
line. Every method here borrows. How do you know that from the
signatures alone?

Solution. `ch07/ex7-17.lu`:

```wolf
struct Point { x: int, y: int }
struct Rect { lo: Point, hi: Point }
impl Rect {
    fn contains(self, p: Point) -> bool {
        p.x >= self.lo.x && p.x < self.hi.x && p.y >= self.lo.y && p.y < self.hi.y
    }
    fn overlaps(self, o: Rect) -> bool {
        self.lo.x < o.hi.x && o.lo.x < self.hi.x && self.lo.y < o.hi.y && o.lo.y < self.hi.y
    }
    fn area(self) -> int {
        (self.hi.x - self.lo.x) * (self.hi.y - self.lo.y)
    }
}
fn main() -> !int {
    let den = Rect { lo: Point { x: 0, y: 0 }, hi: Point { x: 4, y: 3 } }
    let ridge = Rect { lo: Point { x: 3, y: 1 }, hi: Point { x: 6, y: 5 } }
    let creek = Rect { lo: Point { x: 4, y: 3 }, hi: Point { x: 7, y: 6 } }
    print("area {den.area()}")
    print("{den.contains(Point { x: 3, y: 2 })} {den.contains(Point { x: 4, y: 2 })}")
    print("{den.overlaps(ridge)} {den.overlaps(creek)}")
    0
}
```

```console
$ lupin ex7-17.lu
area 12
true false
true false
```

Half-open on purpose: the low edge is in (`>=`), the high edge is out
(`<`), so `contains` at x = 4 in a rect that ends at 4 is `false`, and
two rects that only touch (`creek` starts exactly where `den` ends)
do not overlap — the same convention every slice in chapter 2 used,
because shared edges double-count under closed intervals. The
signatures carry no `mut`, no `take`: every parameter is the default
mode, which chapter 7 defined as borrow-and-read. The absence *is*
the documentation, and it is checkable — add a write to any method
body and the compiler names the missing `mut` at both ends.

**Exercise 7-18** *(extension · lupin)*. One job, two ownership
stories: uppercase every string in a list. Write it consuming —
`shouted(take xs)` returns a new list and the argument is gone — and
lending — `shout(mut xs)` rewrites in place and returns nothing. Run
both. Count what each costs at the call site and in allocations, then
answer: which one should a library export, and does the other need to
exist at all?

Solution. `ch07/ex7-18.lu`:

```wolf
fn shouted(take xs: List[str]) -> List[str] {
    var out = List[str]()
    for x in xs {
        (mut out).push(x.upper())
    }
    out
}
fn shout(mut xs: List[str]) {
    var i = 0
    while i < xs.len {
        xs[i] = xs[i].upper()
        i += 1
    }
}
fn main() -> !int {
    var a = List[str]()
    (mut a).push("howl")
    (mut a).push("scratch")
    let b = shouted(take a)
    print("{b[0]} {b[1]}")
    var c = List[str]()
    (mut c).push("howl")
    (mut c).push("scratch")
    shout(mut c)
    print("{c[0]} {c[1]}")
    0
}
```

```console
$ lupin ex7-18.lu
HOWL SCRATCH
HOWL SCRATCH
```

The consuming version costs a whole second list and takes the caller's
original away — after `take a`, using `a` is the E1001 this chapter
opened with. The lending version allocates only the strings it
replaces and the caller keeps their binding; the price is one `mut` at
each end, which is a price in *candor*, not in machinery. Export the
lender: the caller who wanted the consuming shape can build it from
the lender in two lines (`copy`, then `shout`), but the reverse is
impossible — a consumed argument cannot be un-eaten. An API should
take the least ownership that does the job, precisely so the bigger
appetite stays the caller's decision.

**Exercise 7-19** *(fingers · lupin)*. The same plane geometry as
7-17, asked of the arms instead of the fields. Write `corner(p)`,
which names where a `Point` sits relative to the axes, and `kind(r)`,
which describes a `Rect` — both as a single `match` whose arms take
the value apart by field name rather than reading `p.x` and
`self.lo.y` through the value. Two questions when it runs: which arm
of `corner` would become unreachable if you moved it to the top, and
why does `kind`'s second arm need no `_` beside it?

Solution. `ch07/ex7-19.lu`:

```wolf
struct Point { x: int, y: int }
struct Rect { lo: Point, hi: Point }
fn corner(p: Point) -> str {
    match p {
        Point { x: 0, y: 0 } => "the origin",
        Point { x: 0, .. } => "on the y axis",
        Point { y: 0, .. } => "on the x axis",
        Point { x, y } => "{x} {y}",
    }
}
fn kind(r: Rect) -> str {
    match r {
        Rect { lo: Point { x: 0, y: 0 }, hi: Point { x, y } } => "anchored, {x} by {y}",
        Rect { lo: Point { x: lx, y: ly }, hi: Point { x: hx, y: hy } } =>
            "{hx - lx} by {hy - ly} at {lx} {ly}",
    }
}
fn main() -> !int {
    let den = Rect { lo: Point { x: 0, y: 0 }, hi: Point { x: 4, y: 3 } }
    let ridge = Rect { lo: Point { x: 3, y: 1 }, hi: Point { x: 6, y: 5 } }
    print(corner(den.lo))
    print(corner(ridge.lo))
    print(corner(Point { x: 0, y: 3 }))
    print(kind(den))
    print(kind(ridge))
    0
}
```

```console
$ lupin ex7-19.lu
the origin
3 1
on the y axis
anchored, 4 by 3
3 by 4 at 3 1
```

An arm is a conjunction of field tests over the value's own shape.
`Point { x: 0, y: 0 }` tests both fields against literals;
`Point { x: 0, .. }` tests one and says the rest is deliberately
ignored; `Point { x, y }` tests nothing and binds both under their own
names. The comma before `..` is not decoration — the field list
separates its members with commas and `..` is one more member, so
`Point { x: 0, .. }` is the spelling.

Move `Point { x, y }` to the top of `corner` and every arm below it
dies: an all-binder product covers everything, which is exactly what
makes it a catch-all and exactly why it goes last. `kind`'s second arm
needs no `_` for the same reason in reverse — it binds all four
coordinates and constrains none of them, so it already *is* the
catch-all and the `match` is exhaustive without one.

The signatures still carry no `mut` and no `take`, so 7-17's answer
survives unchanged: every parameter is the default mode, and testing a
value is not taking it. An arm that binds a non-`Copy` piece would move
the whole scrutinee; every field here is an `int`, so nothing moves and
`den` is still readable on the line after.

**Exercise 7-20** *(extension · wolf + lupin)*. Take 7-16's walk one
step further. Instead of printing, `diff(a, b)` returns the edits as a
`List[Edit]`, with `struct Edit { mark: str, line: str }`, and the
caller prints them and counts the changed lines. Before writing it,
decide whether `diff` lends its two lists or takes them, and say in one
sentence what the lines in the result are under each choice. Run it on
a three-line config file and a four-line successor of your own.

Solution. `ch07/ex7-20.lu` (the walk and `diff`; `build_table` is
7-16's, unchanged):

```wolf
struct Edit { mark: str, line: str }
fn walk(a: List[str], b: List[str], t: List[List[int]], i: int, j: int, mut out: List[Edit]) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        walk(a, b, t, i - 1, j - 1, mut out)
        (mut out).push(Edit { mark: " ", line: a[i - 1] })
    } else if j > 0 && (i == 0 || t[i][j - 1] >= t[i - 1][j]) {
        walk(a, b, t, i, j - 1, mut out)
        (mut out).push(Edit { mark: "+", line: b[j - 1] })
    } else if i > 0 {
        walk(a, b, t, i - 1, j, mut out)
        (mut out).push(Edit { mark: "-", line: a[i - 1] })
    }
}
fn diff(a: List[str], b: List[str]) -> List[Edit] {
    let t = build_table(a, b)
    var out = List[Edit]()
    walk(a, b, t, a.len, b.len, mut out)
    out
}
```

```console
$ lupin ex7-20.lu
  host = kasumi
- port = 80
+ port = 8080
  user = ada
+ log = on
3 of 5 lines changed; old still has 3
$ wolf run ex7-20.lu
  host = kasumi
- port = 80
+ port = 8080
  user = ada
+ log = on
3 of 5 lines changed; old still has 3
```

`diff` lends both lists, and the last line of the run is the reason:
the caller still has `old` afterward, and the caller of a diff usually
wants both files next, to print beside the edits or to patch. The
price is that every line in the result is a copy of the caller's. A
`push` copies what it stores unless the argument is written `take`,
and a line read out of a lent list cannot be taken: `take a[0]` on a
parameter with no mode is a compile error and a `trap(exclusivity)`
under lupin, both citing `[mem.tier0.mode.read]`.
A `diff(take a, take b)` could `take` each line into its `Edit` and
copy nothing, and the caller would have no files left to compare.

The recursion in `walk` is 7-16's, with one change that is this
chapter's whole subject: it cannot print, so it needs somewhere to put
the edits, and `mut out` at both ends is that place. The edits come
out forward for the same reason 7-16's lines did.

**Exercise 7-21** *(extension · wolf + lupin)*. The last three
exercises share one shelf, whose documents now carry a list of their
own: `struct Doc { title: str, words: int, tags: List[str] }` and
`struct Shelf { docs: List[Doc] }`. First, `remove(mut s, title)`:
take every document with that title off the shelf and return how many
went, or the error row `{missing}` when none did. Build the shelf's new
list by moving the survivors into it. How many documents does `remove`
copy, and why is the shelf `mut` rather than `take`?

Solution. `ch07/ex7-21.lu`:

```wolf
struct Doc { title: str, words: int, tags: List[str] }
struct Shelf { docs: List[Doc] }
fn remove(mut s: Shelf, title: str) -> int ! {missing} {
    let old = move s.docs
    var kept = List[Doc]()
    var gone = 0
    for d in old {
        if d.title == title {
            gone += 1
        } else {
            (mut kept).push(take d)
        }
    }
    s.docs = kept
    if gone == 0 { return missing }
    gone
}
fn main() -> !int {
    var shelf = Shelf { docs: List[Doc]() }
    (mut shelf.docs).push(Doc { title: "regions", words: 900, tags: List[str]() })
    (mut shelf.docs).push(Doc { title: "moves", words: 640, tags: List[str]() })
    (mut shelf.docs).push(Doc { title: "regions", words: 1200, tags: List[str]() })
    let n = remove(mut shelf, "regions") else 0
    let m = remove(mut shelf, "lifetimes") else -1
    print("{n} {m} {shelf.docs.len} {shelf.docs[0].title}")
    0
}
```

```console
$ lupin ex7-21.lu
2 -1 1 moves
$ wolf run ex7-21.lu
2 -1 1 moves
```

None. `move s.docs` hands the whole list to `old` and leaves the
shelf's field empty; each survivor leaves `old` with `take d` and
lands in `kept` whole, tags and all; `s.docs = kept` fills the emptied
field again, which is §7.2's rule for an emptied place. The two
documents titled `regions` are never handed anywhere, and they go when
`old` does, at the end of the call. Leave `take` off the `push` and the
program prints the same line while copying every survivor, because a
container store copies what it is given unless the argument says
`take`.

The shelf is `mut` because `remove` changes it and the caller keeps
it. A `take` signature would make every caller give the shelf up and
receive a new one back, which is the same program with a longer call
site and a window in which the caller has no shelf at all. `mut` says
exactly what happens: the caller's shelf is written through, in place,
and the call site says so.

**Exercise 7-22** *(extension · wolf + lupin)*. Second, `merge(mut
into, take from)`: move every document of `from` onto the end of
`into`. Why is `from` taken rather than `mut`, what may the caller do
with its own binding after the call, and what would leaving `take` off
the `push` inside the loop cost?

Solution. `ch07/ex7-22.lu`:

```wolf
struct Doc { title: str, words: int, tags: List[str] }
struct Shelf { docs: List[Doc] }
fn merge(mut into: Shelf, take from: Shelf) {
    for d in from.docs {
        (mut into.docs).push(take d)
    }
}
fn main() -> !int {
    var a = Shelf { docs: List[Doc]() }
    var b = Shelf { docs: List[Doc]() }
    (mut a.docs).push(Doc { title: "regions", words: 900, tags: List[str]() })
    (mut b.docs).push(Doc { title: "moves", words: 640, tags: List[str]() })
    (mut b.docs).push(Doc { title: "tokens", words: 300, tags: List[str]() })
    merge(mut a, take b)
    print("{a.docs.len}: {a.docs[0].title} {a.docs[1].title} {a.docs[2].title}")
    0
}
```

```console
$ lupin ex7-22.lu
3: regions moves tokens
$ wolf run ex7-22.lu
3: regions moves tokens
```

`merge` is the end of `b` as a shelf, and `take` is the signature
saying so. A `mut from` would leave the caller holding `b` after the
call, so `merge` would either copy every document, and both shelves
would hold them, or empty `b` by hand, and the caller would hold an
empty shelf that nothing in the signature mentioned. With `take`, the
caller wrote `take b` at the call and the binding is spent: add a line
that reads `b.docs.len` after the call and the compiler stops it with
E1001 at that read, naming `take b` as the move, while lupin runs up
to it and traps `use-after-move`. The caller may assign `b` a new
shelf and use it again; it may not read the old one.

Because `from` is `merge`'s own, so are its documents, and `take d`
moves each one onto `into` whole, its tags with it. Without the `take`
the program prints the same line and copies every document it moves,
tags included, into a list that is about to be dropped anyway: the
container store copies unless the argument says otherwise.

**Exercise 7-23** *(comprehension + spelunking · wolf + lupin)*.
Third, `longest(s)`: return the document with the most words, the
`Doc` itself and not its title, from a shelf passed with no mode. Write
it the obvious way, ending on `s.docs[best]`, and build it. Read what
the compiler says, then `wolf --explain E1002`, and pick one of the two
repairs it offers: which one does a query on a shelf want? Then run the
unrepaired program under `lupin`, and say why §7.3's `longest` never
met this, and why it would not have met it even returning a `Doc`, as
long as that `Doc` had no `tags`.

Solution. `ch07/ex7-23.lu` (the function; `main` fills a shelf of
three and prints the longest title and the shelf's length):

```wolf
struct Doc { title: str, words: int, tags: List[str] }
struct Shelf { docs: List[Doc] }
fn longest(s: Shelf) -> Doc {
    var best = 0
    var i = 1
    while i < s.docs.len {
        if s.docs[i].words > s.docs[best].words { best = i }
        i += 1
    }
    s.docs[best]
}
```

```console
$ wolf conform-run ./ex7-23.lu
error[E1002]: `s` is the caller's value, and it is returned here while the caller still holds it
  --> ./ex7-23.lu:14:5
   |
 7 | fn longest(s: Shelf) -> Doc {
   |            - `s` is declared without a mode — that spells `read`: lent for the call, and still the caller's after it
...
14 |     s.docs[best]
   |     ^^^^^^^^^^^^ this would be a second live path to the value `s` was lent
   |
   = note: a `read` parameter is lent, never given [mem.tier0.mode.read]. Handing it on past the
     call would leave two places able to write one value, with no `shared` spelled anywhere
     [mem.tier0.excl.1]. Hand on an independent value with `copy`, or declare the parameter
     `take` so the caller gives it up (call sites then spell `take`).
help: to hand on an independent value, copy it at the move
   |
14 |     copy s.docs[best]
   |
```

This is the lend rule, and E1002 is its code because it is
exclusivity again, one step removed. The shelf was lent: after the call
it is still the caller's, whole. Returning `s.docs[best]` would hand the
caller a second owner of a document the shelf still holds, and a `Doc`
now carries a `List`, which is storage two owners could both write,
with no `shared` written anywhere to say so. The explain text's second
paragraph is the general rule, and its last sentence is the answer to
the last question: "Values that cannot reach shared storage — a struct
of scalars, a `str` — are not refused: their second path is a copy."
§7.3's `longest` copied a title out and returned a `str`, and a `Doc`
of a `title` and a `words` is a struct of a `str` and an `int`. The rule
first has something to say when the value has a list inside it.

The two repairs are two different claims. `take s` says the query
consumes the shelf, so the caller would give up every document to learn
which one is longest; that is the wrong signature for a question.
`copy` at the move says the caller gets an independent document and the
shelf is untouched, and it is what `ch07/ex7-23b.lu` writes:

```console
$ lupin ex7-23b.lu
moves, and the shelf still holds 3
$ wolf run ex7-23b.lu
moves, and the shelf still holds 3
```

The copy costs the document and its tags list. A caller that only wants
to look can skip even that by asking for the position, `best` itself,
and reading `shelf.docs[i]` in place, the way §7.6's `Tok` hands back
two integers instead of the text.

The interpreter does not refuse the unrepaired program:

```console
$ lupin ex7-23.lu
moves, and the shelf still holds 3
```

The lend rule is static, and `lupin` copies at the crossing, so the
program it runs is the one with the `copy` already in it. That is why
the compiler's verdict is the one this exercise is about: it is the
machine that makes the copy a word you write, where a reader can see
what it costs.
