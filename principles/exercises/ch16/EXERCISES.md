# Chapter 16 — Region transfer: fearless messaging: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; the compiler's `conform-run`
reports `unsupported` for channel programs today, so verdicts below are
lupin's except where marked pending.

## §16.1 — `ch.send(move r)`

**Exercise 16-1** *(comprehension · lupin)* — The sender builds a list
inside a region — two pushes — and sends the region. Predict: does the
receiver's `in r2 { … }` block run before or after both pushes are
visible, and what synchronization made that true?

```wolf
    s.spawn(fn() {
        let r = region()
        let xs = in r {
            var v = List[int]()
            (mut v).push(41)
            (mut v).push(1)
            v
        }
        ch.send(move r)
    })
    let r2 = ch.recv() else |_| { return 1 }
    got = in r2 { 42 }
```

Solution: after — the `move` send happens-before the receive, so every
write into the region before the send is visible to the receiver, and
no other synchronization exists or is needed. The channel carried one
word (the region), not the data; the *ownership* is what moved. This
is the message-passing litmus from the memory model, and it is the
entire safety argument of this chapter in eight lines.

```console
$ lupin ex16-1.lu
received
```

**Exercise 16-2** *(fingers · lupin)* — Make the transfer carry real
freight: build the two-element list in `main`, send the region to a
receiving task, and have the receiver sum the list *it never built*.
Print the sum from the receiver's side.

Solution — `ch16/ex16-2.lu` (receiver):

```wolf
    s.spawn(fn() {
        let r2 = ch.recv() else |_| { return }
        let total = in r2 {
            var t = 0
            for x in xs { t += x }
            t
        }
        print("sum={total}")
    })
    ch.send(move r)
```

```console
$ lupin ex16-2.lu
sum=42
```

The receiver iterates a list allocated by someone else, in memory that
changed owners, and reads exactly what was written. Nothing was
serialized, nothing was copied — the graph moved as a graph. (Erlang
gets this safety by deep-copying every message; the next section's
lineup prices that.)

**Exercise 16-3** *(extension (break-it-on-purpose) · lupin)* — Construct the
smallest program in which a sender touches a region *after* sending it
with `move`. Predict the exact trap kind before running — it is one
you met in chapter 7, not a new one.

Solution — `ch16/ex16-3.lu`:

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    let n = in r { 41 }
    ch.send(move r)
    let m = in r { 1 }
    m
}
```

```console
$ lupin ex16-3.lu
ex16-3.lu: trap(use-after-move): `r` was moved out and is uninitialized here [mem.tier0.move.2] at 254..255; `r` moved here at 231..237
$ echo $?
3
```

The trap is `use-after-move`, citing the same clause family as chapter
7's moved string — because a region is a value and `move` into a
channel is the same move as `take` into a function. No new rule was
needed to make cross-task transfer safe; the old rule was enough, which
is the design working as designed. (The compiler rejects the shape statically, at the send; lupin carries
the same lesson dynamically, at the read.)

## §16.2 — Freeze, then share

**Exercise 16-4** *(comprehension · lupin)* — Ten squares are frozen
into `table`; two tasks each read one entry and send it back. Predict
the printed number, and answer precisely: how many copies of the table
exist while both tasks read it?

```wolf
    let table = freeze region {
        var xs = List[int]()
        for i in 0..10 { (mut xs).push(i * i) }
        xs
    }
    scope s {
        s.spawn(fn() { ch.send(table[3]) })
        s.spawn(fn() { ch.send(table[4]) })
    }
```

Solution: `25` (9 + 16), and one copy — the frozen original. Both
tasks hold references into the same immutable memory; `freeze`
happens-before every cross-task read, and immutability makes
concurrent reading trivially safe. Where `move` gives the data one
owner at a time, `freeze` gives it no owner that can write — two
different ways to make "who else is touching this?" unaskable.

```console
$ lupin ex16-4.lu
25
```

**Exercise 16-5** *(design)* — For each payload, choose `move` or
`freeze` and defend it in one sentence: (a) a parsed configuration
read by every worker for the process lifetime; (b) a request's parse
tree handed from parser to executor; (c) a routing table rebuilt every
30 seconds and read constantly; (d) a 2 GB index segment consulted by
eight shards.

Solution (discussion): (a) freeze — many readers, no writer, forever:
the definitional freeze. (b) move — exactly one consumer, which will
mutate and then discard it; freezing would forbid the executor its
annotations. (c) the interesting one: each rebuild is built mutable in
a fresh region, then frozen and published; readers of the old table
keep reading it until they pick up the new one — freeze does not mean
one-forever, it means immutable-per-edition. (d) freeze, and the size
is the argument: eight copies is 16 GB and one shared frozen segment
is 2, with no lock on the read path. The pattern under all four:
mutate privately, then either hand it over whole or make it
untouchable — wolf gives no third verb, on purpose.

## §16.3 — The honest lineup

**Exercise 16-6** *(design)* — The same workload — a producer builds a
million-node tree, a consumer walks it — in four systems: Erlang
(copying send), Go (send a pointer), Rust (`Arc<Mutex<Tree>>`), wolf
(`ch.send(move r)`). For each, name what the transfer costs at the
moment of send, and what it costs the *receiver* to be safe while
reading. One of the four pays at a different time than the others —
which?

Solution (discussion): Erlang pays at send — a deep copy of a million
nodes — and the receiver is then perfectly safe reading its private
copy; safety was purchased in one large cash payment. Go pays nothing
at send and the receiver is safe only by convention: nothing stops the
producer from mutating the shared tree, and the race detector, not the
compiler, is the enforcement. Rust's `Arc<Mutex>` pays a little at
send (refcount) and then pays on every read — lock traffic on a tree
that will never be written again, unless the design graduates to
`Arc<Tree>` frozen-by-construction, which is Rust spelling wolf's
answer manually. Wolf pays one word at send and nothing at read; the
compiler's move check is the whole bill, paid at compile time. That is
the odd one out: three systems pay at runtime in copies, faith, or
locks; wolf pays before the program runs. The honest caveat: Erlang's
copy buys process isolation across *machines* with the same
semantics, which no move can — distribution is where the copy stops
looking expensive.

## Chapter batch

**Exercise 16-7** *(extension · lupin)* — A maze is a graph, and a
graph is a region's favorite payload. Carve a 5×5 maze with a seeded
generator (per-cell wall bitmasks: 1=N 2=E 4=S 8=W, depth-first carve,
a small linear-congruential step for direction choice), building the
wall table inside a region. Send the region to a solver task; the
solver breadth-first-searches it in place and prints the shortest-path
distance from corner to corner. Seed 1: run it. Before you do, answer:
how many times is the wall table copied between carver and solver?

Solution — `ch16/ex16-7.lu` (the transfer; carve and solve are in the
file):

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    let walls = in r { carve(5, 5, 1) }
    scope s {
        s.spawn(fn() {
            let r2 = ch.recv() else |_| { return }
            let d = in r2 { solve(walls, 5, 5) }
            print("distance={d}")
        })
        ch.send(move r)
    }
    0
}
```

```console
$ lupin ex16-7.lu
distance=18
```

Zero copies. The carver allocated the table, the stack, and the
visited set into one region; the solver's BFS queue and distance table
could live there too. One `move` later the solver owns all of it, and
the answer — 18 steps for seed 1 — comes out of memory the solver
never allocated.

**Exercise 16-8** *(comprehension + schedule play · lupin)* — Change
the carve seed to 2 and run the program three times, including once
under `lupin run … --seed=7`. Predict: which of the two seeds in play
changes the printed distance, and which cannot — and why does this
program print the same distance under every scheduler seed?

Solution — `ch16/ex16-8.lu` (one line differs from 16-7):

```console
$ lupin ex16-8.lu
distance=10
$ lupin ex16-8.lu
distance=10
$ lupin run ex16-8.lu --seed=7
distance=10
```

The carve seed changes the maze — seed 2 happens to carve a more
direct route, distance 10. The scheduler seed cannot change anything:
the program has one send, one receive, and a join; every schedule
orders them the same way, so the output is schedule-independent by
construction. Determinism you can argue from the program's shape is
worth more than determinism you observed in three runs — chapter 17
is about the programs where you cannot argue it.

**Exercise 16-9** *(comprehension · wolf + lupin)* — This program
declares a channel of bare `List[int]` — not `Copy`, not imm, not a
region, not sync:

```wolf
fn main() -> !int {
    let ch = channel[List[int]](1)
    0
}
```

Predict the verdict this program earns and the rule behind it, and
explain why each of the four admitted payload classes is safe where a
bare `List` is not.

Solution: the verdict is `fail(E1102)`, and the note names all four
classes:

```console
$ wolf conform-run ./ex16-9.lu
error[E1102]: `List[int]` cannot be sent through a channel
 --> ./ex16-9.lu:7:22
  |
7 |     let ch = channel[List[int]](1)
  |                      ^^^^^^^^^ not a sendable payload type
  |
  = note: channel payloads must be `Copy` data, `imm` data, a region value (the send is its affine
    move), or a `sync` type ([conc.chan.type]) — sending anything else would give two tasks
    one mutable value. D14's verbs are the ways out: `move` the data into a region and send
    the region, `freeze` it into shareable `imm` data, or guard it with a `Mutex`.
```

Each admitted class removes one half of the race. `Copy` data: the
receiver gets its own bits, so there is no shared location. `imm` data:
there is a shared location and nobody may write it. A region value: the
send is a move, so exactly one task owns it at any instant. A `sync`
type: the sharing is real and the coordination is the type's own job. A
bare `List` is none of these — sending it would give two tasks live
access to one mutable buffer with no coordination, which is chapter 13's
store-buffer program wearing a channel as a disguise.

Note that the rejection is a property of the *declaration*: no `send`
appears in the program, and none is needed. The type of the channel is
already the claim, and E1102 is the compiler declining it. The
interpreter takes the other route and constructs the channel — its
dynamic machine catches an actual cross-task mutation rather than the
declaration — so this is one more program the compiler stops and lupin
runs:

```console
$ lupin ex16-9.lu
$ echo $?
0
```

The corpus carries the same expectation in `conc/chan_unsendable.lu`.
