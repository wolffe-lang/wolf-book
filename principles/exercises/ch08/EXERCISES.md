# Chapter 8 — Regions: memory in the shape you meant: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the wolf-lang debug build
at `impl_version 0.0.1`.

## §8.1 — You already think in regions

**Exercise 8-1** *(comprehension · prose)*. Three programs you have met
or written: (a) a web server handling one request (parse the headers,
build a response, send it); (b) a compiler pass (read an AST, produce a
transformed AST, discard the scratch); (c) a game loop (each frame
computes collision pairs and a display list, then draws). For each, name
the group of allocations that share a death, the moment they all die,
and the one value (if any) that must survive. No wolf required; the
point is that the regions were already there.

Solution: (a) everything parsed and built for the request dies when
the response is flushed; the survivor is the response bytes (and any
session state, which was never the request's to own). (b) the scratch
and the *input* AST die at pass end; the survivor is the output AST —
which is why pass frameworks so often copy it out of an arena. (c) the
collision pairs and display list die at frame end, sixty times a
second; the survivor is the game state that feeds the next frame. In
every case the granule was "these thousands of objects, together," and
the death was one moment — a shape single-owner trees cannot spell,
which is the chapter's opening argument.

## §8.2 — The block form

**Exercise 8-2** *(fingers · lupin)*. Sum the first hundred integers
using a list a helper function builds (the helper writing no
region code at all) inside `region tmp { }`. State where `fill`'s list
is allocated, and what happens to it at the closing brace.

Solution. `ch08/ex8-2.lu`:

```wolf
fn fill(n: int) -> List[int] {
    var xs = List[int]()
    for i in 0..n { (mut xs).push(i) }
    xs
}
fn main() -> !int {
    var total = 0
    region tmp {
        let xs = fill(100)
        for x in xs { total += x }
    }
    print("{total}")
    0
}
```

```console
$ lupin ex8-2.lu
4950
```

`fill` allocates into the *caller's* current region — that is the
default, and it is why the helper needed no annotation. At the brace,
the region frees wholesale: the list, its buffer, all of it, in one
motion. `total` survives because an `int` is a value in the frame, not
an allocation in the region.

**Exercise 8-3** *(comprehension · wolf)*. One assignment tries to
smuggle a region value past the brace:

```wolf
struct Node { value: int }
fn main() -> !int {
    var keep = Node { value: 0 }
    region tmp {
        keep = Node { value: 7 }
    }
    if keep.value == 7 { 0 } else { 1 }
}
```

Predict the compiler's verdict, and — before reading the diagnostic —
list the three code locations you expect it to point at.

Solution: rejected, E1010, and the diagnostic points at exactly the
three moments of the story: the region's creation, the escape, and the
free.

```console
$ wolf conform-run ./ex8-3.lu
error[E1010]: `keep` still holds a value allocated in region `tmp` when the region is freed
 --> ./ex8-3.lu:8:9
  |
7 |     region tmp {
  |            --- region `tmp` is created here
8 |         keep = Node { value: 7 }
  |         ^^^^ the value flows out of the region here
  |                ----------------- allocated here, into region `tmp`
9 |     }
  |     - the region is freed here — everything in it is freed wholesale, as one unit
  |
  = note: to keep the value, allocate it where it must live: build it outside the region block, or
    aim the allocation at a longer-lived region explicitly (`let r = region()` … `in r { …
    }`); widening the region block to cover every use also works. Two keep-alive
    alternatives change the ownership instead: `freeze` the region (immutable forever) or
    make the value a `shared` cell (reference-counted, never dangles).
```

The checker speaks in allocation, escape, and free — the word
"lifetime" appears nowhere, because the region *is* the lifetime,
reified. lupin enforces the same rule at run time and blames the read
rather than the write: the value died with its region, and the fault
fires where the program finally reaches for it.

```console
$ lupin ex8-3.lu
ex8-3.lu: trap(region-fault): `keep.value` reaches into `tmp` (region #1), which was freed wholesale; the value died with the region [mem.region.intra.2] at 10:8; the region was created here at 7:5
$ echo $?
3
```

One rule, two moments: the compiler refuses the program before it
starts; the interpreter lets it run and faults the exact access that
needed the freed data. Which line each tool blames is the difference
between "this could dangle" and "this did."

## §8.3 — Regions are values

**Exercise 8-4** *(fingers · lupin REPL)*. In the REPL: define a
one-field struct, create a region with `region(rc)`, allocate one value
into it with `in r { … }`, and look at `:regions` before and after
`freeze r`. Predict the two state words you will see before you look.

Solution. One session:

```console
$ lupin
wolf> struct Howl { n: int }
defined type `Howl`
wolf> let r = region(rc)
wolf> let h = in r { Howl { n: 3 } }
wolf> :regions
regions:
  #0 `program` arena state=open objects=0
  #1 `-` rc state=suspended objects=1
wolf> let f = freeze r
wolf> :regions
regions:
  #0 `program` arena state=open objects=0
  #1 `-` rc state=frozen objects=1
wolf> h.n
3 : i32
wolf> :quit
```

The states are `suspended` and `frozen`. Creating a region does not
open it — the `in r { }` window did, briefly, and left it suspended
with one object inside. `freeze` is a state of the region, not a
property of any binding.

**Exercise 8-5** *(comprehension · lupin)*. A region is being sent
somewhere while a window into it is still open:

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    in r {
        var xs = List[int]()
        (mut xs).push(1)
        ch.send(move r)
        0
    }
}
```

Predict the event, its trap kind, and the static error code the trap
line will mention.

Solution: a `region-fault` — a region moves as a *closed* subtree, and
this one is open at the send:

```console
$ lupin ex8-5.lu
ex8-5.lu: trap(region-fault): region #1 is open here and cannot be transferred; a region moves as a closed subtree (the compiler's E1005) [mem.region.freeze.3] at 10:17
$ echo $?
3
```

The trap line names E1005 itself: the interpreter is enforcing at run
time the same rule the compiler will reject at compile time, and it
cites the compiler's number so the two tiers stay one story.

## §8.4 — Cycles are fine here

**Exercise 8-6** *(fingers · lupin)*. Build a five-node doubly-linked
ring in a pool region: each node points `next` and `prev`. Then prove
both directions work: walk five steps forward from the head (where do
you land?), and two steps backward. Rust folklore says this program
requires `unsafe` or `Rc<RefCell<…>>`; say in one sentence why wolf's
checker does not object here.

Solution. `ch08/ex8-6.lu` (core):

```wolf
struct Node { value: int, next: handle Node, prev: handle Node }
var pool = Pool[Node]()
var hs = List[handle Node]()
for _ in 0..5 { (mut hs).push((mut pool).reserve()) }
for i in 0..5 {
    (mut pool).init(hs[i], Node {
        value: (i + 1) * 10,
        next: hs[(i + 1) % 5],
        prev: hs[(i + 4) % 5],
    })
}
```

```console
$ lupin ex8-6.lu
10 40
```

Five steps forward from node 1 is node 1 again — it is a ring — and
two steps back lands on node 4. The checker does not object because
the cycle never crosses the region border: intra-region edges are
unrestricted, and the region dies as one unit, so no edge can dangle.
The two-phase `reserve`/`init` is what closed the cycle without a null
in sight.

**Exercise 8-7** *(extension · prose)*. Grow the ring into the real
folklore program: an LRU cache with sentinel head and tail, `unlink`
and `push_front` as the only two link operations, promotion on get, and
eviction of `tail.prev` at capacity. Trace it by hand: after put a, put
b, get a, put c at capacity 2, what does the front-to-back walk
print?

Solution (prose): `c a`. Insertion order is b-then-a reversed by
`push_front`; the get promotes `a` past `b`; the eviction takes
`tail.prev`, which the promotion made `b`; `c` lands in front. The
walk visits `head.next` to `tail`. Every link mutation in the program
goes through `unlink` or `push_front` — four pointer writes and five —
which is the entire aliasing surface a reviewer must read.

## §8.5 — Freeze

**Exercise 8-8** *(comprehension · wolf + lupin)*. A struct type with a
strong `shared` edge back to itself:

```wolf
struct Node { value: int, next: shared Node }
fn main() -> !int { 0 }
```

`main` builds nothing. Predict each tool's verdict anyway, then
explain the asymmetry: which tool is answering "could any program with
this type leak," and which is answering "did this program fault"?

Solution: wolf rejects the *type*; lupin runs the empty `main` to exit
0.

```console
$ wolf conform-run ./ex8-8.lu
error[E1006]: `Node` holds a strong `shared` path back to itself
 --> ./ex8-8.lu:4:27
  |
4 | struct Node { value: int, next: shared Node }
  |                           ^^^^ this `shared` edge closes the cycle Node → Node
  |
  = note: strong `shared` references drop their target when the last count drops, so a strong
    cycle would keep itself alive forever — and wolf has no cycle collector
    ([mem.shared.rc.2]). Break the back-edge: make this field `weak Node` (upgrade to reach
    the value without keeping it alive) or `handle Node` (a generational index that faults
    if the target is gone). If the structure is genuinely cyclic, keep the whole graph in
    one region instead — intra-region cycles are safe and freed wholesale
    ([mem.region.intra.1]).
```

```console
$ lupin ex8-8.lu
$ echo $?
0
```

This is conservatism, legitimately: the static tier rejects by *shape*
because some program with this type leaks, and refusing the type is
the only way to refuse all of those programs at once. The dynamic tier
saw no allocation, no cycle, no fault — also true. The note is the
chapter in miniature: `weak`, `handle`, or put the cycle in a region.

**Exercise 8-9** *(comprehension + spelunking · wolf)*. One write after
a freeze:

```wolf
struct Config { limit: int }
fn main() -> !int {
    var cfg = freeze region { Config { limit: 42 } }
    cfg.limit = 7
    cfg.limit
}
```

Predict the verdict and, from `--explain`-level knowledge, the two
repairs the note will offer.

Solution:

```console
$ wolf conform-run ./ex8-9.lu
error[E1012]: `cfg.limit` is frozen, so it cannot be assigned through
 --> ./ex8-9.lu:7:5
  |
6 |     var cfg = freeze region { Config { limit: 42 } }
  |               -------------------------------------- the freeze happens here — the promotion to `imm` is deep and permanent
7 |     cfg.limit = 7
  |     ^^^^^^^^^ this needs the data to be mutable
  |
  = note: `freeze` promotes the whole graph to `imm`: shareable from anywhere, forever, and never
    writable again. Build the value completely before freezing it, or keep a mutable copy
    (`copy`) alongside the frozen one.
```

The repairs: finish building before freezing, or `copy` a mutable
twin. There is no third repair, because there is no unfreeze —
`freeze` is a cadence, not a lock. lupin reaches the same verdict from
the other side, trapping `region-fault` at `[mem.region.freeze.1]` —
the clause E1012 enforces statically:

```console
$ lupin ex8-9.lu
ex8-9.lu: trap(region-fault): region #1 is frozen: `imm` data is immutable forever [mem.region.freeze.1] at 7:5
$ echo $?
3
```

**Exercise 8-10** *(comprehension · lupin)*. The dynamic half of the
same contract: create a pool region, freeze the region value, then call
`reserve` on the pool. Predict the trap kind and the clause tag.

Solution. `ch08/ex8-10.lu`:

```wolf
struct Node { value: int }
fn main() -> !int {
    let r = region(pool(Node))
    let p = in r { Pool[Node]() }
    let frozen = freeze r
    let h = p.reserve()
    0
}
```

```console
$ lupin ex8-10.lu
ex8-10.lu: trap(region-fault): region #1 is frozen: `imm` data is immutable forever [mem.region.freeze.1] at 9:13
$ echo $?
3
```

`region-fault`, citing `[mem.region.freeze.1]` — the same clause the
E1012 diagnostic enforces statically in 8-9. `reserve` is a mutation
of the region's interior, and frozen means frozen all the way down.

## §8.6 — Open, and open again

**Exercise 8-11** *(comprehension · lupin)*. Two region values, two
nested `in` windows, reads and writes crossing both:

```wolf
let a = region()
let b = region()
in a {
    var xs = List[int]()
    (mut xs).push(1)
    in b {
        var ys = List[int]()
        (mut ys).push(2)
        total += xs[0] + ys[0]
    }
    (mut xs).push(3)
    total += xs[1]
}
```

Predict the printed total. Then the antichain question: of the shapes
(1) `in a { in b { } }`, (2) `in a { in a { } }`, (3)
`in a { } in a { }` — sequential reopen — which are legal? Answer from
the rule that region values are affine and windows must be into
*distinct* regions, then check the one the program demonstrates.

Solution: total is 6 (1 + 2, then 3). Shape 1 is legal — distinct
regions, both open, provably disjoint because region values are affine
(no alias of `a` can exist to sneak into the inner window). Shape 2 is
the one the rule forbids: the same region twice would make the two
windows alias. Shape 3 is legal — suspend, then reopen; the REPL
showed a region surviving between windows in 8-4.

```console
$ lupin ex8-11.lu
6
```

## §8.7 — `shared` and `handle`

**Exercise 8-12** *(comprehension · lupin)*. A handle is used after its
slot is gone:

```wolf
region r: pool(Node) {
    var pool = Pool[Node]()
    let h = (mut pool).reserve()
    (mut pool).init(h, Node { value: 1 })
    (mut pool).remove(h)
    let v = pool[h].value
    v
}
```

Predict the trap kind, and — the part worth being precise about — what
the trap line will say about *generations*.

Solution:

```console
$ lupin ex8-12.lu
ex8-12.lu: trap(stale-handle): handle into pool#0 slot 0 carries generation 0, the slot is at generation 1; a stale handle is a deterministic fault in every profile, never UB [mem.shared.handle.2] at 11:17
$ echo $?
3
```

The handle remembers the generation it was issued at; `remove` bumped
the slot's generation; the mismatch is the fault. The sentence to
carry out of this exercise is the trap's own: deterministic, in every
profile, never UB — a stale handle in wolf is a *defined* event, which
is the entire difference between a handle and a C pointer into a
freed arena.

**Exercise 8-13** *(design)*. Four fields, one decision each: (a) a
parent pointer in a tree whose nodes a region owns; (b) an edge in a
social graph where nodes are deleted while neighbors hold references;
(c) a config blob read by every task for the process's whole life; (d) a
cache entry another subsystem may hold while the cache evicts it. For
each: `shared`, `weak`, `handle`, or a plain intra-region edge; and
name the *failure contract* you chose, not only the shape.

Solution (discussion): (a) plain intra-region edge (or `handle` if
nodes are removed individually): the cycle is safe inside the region,
and the failure contract is "none — the region dies as one." (b)
`handle`: deletion must be observable, and the contract is "reads of a
dead neighbor fault, deterministically" — a `weak` would answer
"gone" quietly, which a graph traversal can misread as "no edge." (c)
`freeze` and share the frozen region: read-only forever wants the
no-synchronization contract, and freezing is how you buy it. (d)
`weak` inside `shared`: the cache wants eviction to *win* — the other
subsystem's read should answer "gone, reload" rather than fault or
keep the entry alive; a strong `shared` would silently defeat the
eviction policy. The pattern under all four: pick the reference type
by what you want *failure* to look like, because that is the part the
type system will hold you to.

## §8.8 — What the machine does

**Exercise 8-14** *(spelunking · wolf)*. Run `wolf --explain E1012` and
read it against exercise 8-9. Find: the sentence that explains why
frozen data needs no locks, the phrase that makes the promotion
transitive, and the reason "readable forever" is a *performance* claim,
not only a safety one.

Solution:

```console
$ wolf --explain E1012
E1012: frozen data cannot be written

`freeze` consumes a region and promotes everything in it to `imm`:
deeply immutable, shareable from anywhere — across threads, without
synchronization — and readable forever. That deal is permanent, and
it is why frozen data needs no locks and no lifetimes; a single write
anywhere would break every reader everywhere. This write reaches data
that a `freeze` already promoted (the freeze site is marked). Do the
mutation before freezing — build the value completely, freeze last —
or keep a mutable `copy` alongside the frozen one.
```

No-locks: "a single write anywhere would break every reader
everywhere" — the absence of writers is what synchronization would
otherwise buy. Transitivity: "promotes *everything in it*" — the deal
is per-region, not per-binding. And "readable forever" is a
performance claim because a fact the compiler can rely on is a fact
the optimizer can spend: loads from *imm* data can be hoisted, merged,
and const-propagated across calls — the aliasing fact C cannot state
(§8.8's refrain), available here because the type system made it
unbreakable.

## Chapter batch

**Exercise 8-15** *(extension · lupin)*. A text adventure's world is a
cyclic graph: rooms point at each other in four directions, and "north
then south" must come home. Build three rooms (den, ridge, river bank)
in a pool region, close the cycles with two-phase init, and walk the
path north, east, west, south, printing the room at each step. Predict
the four lines before running; the fourth is the one that checks your
`south` links are real.

Solution. `ch08/ex8-15.lu` (walk shown; full wiring on disk):

```wolf
for step in path {
    if step == "north" { here = pool[here].north } else if step == "south" { here = pool[here].south } else if step == "east" { here = pool[here].east } else { here = pool[here].west }
    print("you are at the {pool[here].name}")
}
```

```console
$ lupin ex8-15.lu
you are at the ridge
you are at the river bank
you are at the den
you are at the den
```

The last two lines are both "den": west from the river bank comes
home, and the den's `south` is wired to itself — a self-loop is the
graph's honest spelling of "you cannot go that way," and it costs no
optional type. Room graphs are why the region chapter and the
adventure genre get along: the whole world dies in one motion when
the game ends, cycles and all.

**Exercise 8-16** *(extension · lupin)*. `wc`, wolfished: count the
lines and words of a multiline block, but store every line in a scratch
region while counting; then let the region die and print the counts
after it is gone. State what survives the brace and why this program's
memory use at peak is "the text, once" rather than "the text, twice."

Solution. `ch08/ex8-16.lu`:

```wolf
fn main() -> !int {
    let text = """
        the wolf runs at dusk
        the moon rises
        the pack answers
        """
    var lines = 0
    var words = 0
    region scratch {
        var stored = List[str]()
        for line in text.lines() { (mut stored).push(line) }
        lines = stored.len
        for line in stored {
            for _ in line.words() { words += 1 }
        }
    }
    print("{lines} lines, {words} words")
    0
}
```

```console
$ lupin ex8-16.lu
3 lines, 11 words
```

The counts survive — plain `int`s in the frame. The stored lines do
not, and did not need to: line *views* into the original text are
two-word slices, so the region holds a list of views, not a second
copy of the text. Peak memory is the text plus a few dozen bytes of
list — and at the brace, even those go, wholesale. That is the region
answer to a question `wc` authors in C solve with careful `free`
bookkeeping: put the bookkeeping in the shape, then stop doing it.
