# Chapter 12 — Channels and select: exercises

Commands run from this directory; outputs are pasted from real runs.
Every checker in this chapter is lupin except 12-9, whose rejection is
a parse-rung error both implementations report; seeded runs use
`lupin run FILE --seed=N` and exploration uses
`lupin conform-run FILE --explore=N`.

## §12.1 — Typed channels

**Exercise 12-1** *(fingers · lupin)*. A producer sends four squares and
closes; `main` drains with a `for` loop. Type it, run it, then delete the
`ch.close()` line and predict what the second run does before you try it.

Solution. `ch12/ex12-1.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](4)
    var total = 0
    scope s {
        s.spawn(fn() {
            for i in 1..=4 { ch.send(i * i) }
            ch.close()
        })
        for v in ch { total += v }
    }
    print("total={total}")
    0
}
```

```console
$ lupin ex12-1.lu
total=30
```

Without the close, `main`'s `for` waits forever for a fifth value, the
producer is already gone, and the deadlock trap fires. A `for` over a
channel is a loop whose termination condition is *someone else's*
promise. Close is how that promise is kept.

**Exercise 12-2** *(extension (break-it-on-purpose) · lupin)*. Using one
task and one channel of capacity 1, write the shortest program you can
whose second statement never finishes. Predict the trap kind and the
roster before running.

Solution. `ch12/ex12-2.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](1)
    ch.send(1)
    ch.send(2)
    print("never printed")
    0
}
```

```console
$ lupin ex12-2.lu
ex12-2.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0) [conc.deadlock.trap] at 8:5
$ echo $?
3
```

The buffer holds one value; the second `send` blocks until a receive
makes room, and the only task that could receive is the one blocked
sending. A deadlock needs a cycle of waiting, not two tasks, and
`main` alone closes a cycle of length one.

## §12.2 — `select` with timeouts

**Exercise 12-3** *(comprehension · lupin)*. Two identical `select`s;
between them, one `send`. Predict both printed lines:

```wolf
fn main() -> !int {
    let a = channel[int](1)
    select {
        v from a => { print("got {v}") },
        timeout(5.ms) => { print("timed out") },
    }
    a.send(9)
    select {
        v from a => { print("got {v}") },
        timeout(5.ms) => { print("timed out") },
    }
    0
}
```

Solution: `timed out`, then `got 9`. A timeout arm is the arm that
wins when no other arm can, not a delay. The first select has an
empty channel and nothing pending, so the timer is the only way out;
the second finds a value ready and the timer never enters into it.

```console
$ lupin ex12-3.lu
timed out
got 9
```

**Exercise 12-4** *(comprehension + schedule play · lupin)*. Both
channels are ready before the `select` runs. Write down every output this
program is *allowed* to print, then run it under seed 1 and seed 2024:

```wolf
fn main() -> !int {
    let a = channel[int](1)
    let b = channel[int](1)
    a.send(1)
    b.send(2)
    var got = 0
    select {
        v from a => { got = v },
        v from b => { got = v },
    }
    print("{got}")
    0
}
```

Solution: `1` and `2` are both conforming: two simultaneously ready
arms make the pick a recorded scheduler decision, drawn from the seed.

```console
$ lupin run ex12-4.lu --seed=1
2
$ lupin run ex12-4.lu --seed=2024
1
```

Each seed replays byte-identically, forever. "Nondeterministic" in
wolf means the *spec* admits more than one outcome; any single seeded
run is as reproducible as arithmetic.

**Exercise 12-5** *(spelunking · lupin)*. Run the explorer over 12-4
and read its report line by line:

```console
$ lupin conform-run ex12-4.lu --explore=8
ex12-4.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×1 stdout=1\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0
    exit(0) ×1 stdout=2\n leaks=0 forest=ok — replay: --seed=4611686018427387905
      decision stream: ev:1
  deadlocks: 0 · races: 0 · max depth: 1 decision(s)
$ echo $?
1
```

Why "2 schedule(s)" and not eight? What is a `decision stream`, and
why does the tool exit 1 when nothing failed?

Solution (prose): the budget of 8 is a ceiling, not a quota: the
program contains exactly one decision with two choices, so the
frontier closes after two schedules and `max depth: 1 decision(s)`
says so. The decision stream (`ev:0`, `ev:1`) is the schedule written
out as the sequence of choices taken; either replays exactly, and
each outcome also carries a packed `--seed` spelling of the same
stream. Exit 1 is the differential protocol's honesty: a
schedule-dependent program is a *finding* (something a test suite
should know about) even when every outcome is individually fine.
Deterministic-under-every-schedule is the verdict that exits 0, and
exercise 12-8 earns it.

## §12.3 — When channels are the wrong queue

**Exercise 12-6** *(extension · lupin)*. Build a router: one task reads
an inbox and forwards each value to an `evens` or `odds` sink. `main`
feeds 1 through 8 and then sums both sinks. Mind the closes: who closes
what, in what order?

Solution. `ch12/ex12-6.lu`:

```wolf
fn main() -> !int {
    let src = channel[int](8)
    let evens = channel[int](8)
    let odds = channel[int](8)
    scope s {
        s.spawn(fn() {
            for v in src {
                if v % 2 == 0 { evens.send(v) } else { odds.send(v) }
            }
            evens.close()
            odds.close()
        })
        for i in 1..=8 { src.send(i) }
        src.close()
    }
    var esum = 0
    var osum = 0
    for v in evens { esum += v }
    for v in odds { osum += v }
    print("evens {esum}, odds {osum}")
    0
}
```

```console
$ lupin ex12-6.lu
evens 20, odds 16
```

Close flows downstream: `main` closes `src` when the feed ends; the
router's loop ends because of that close, and only then does the
router close its two output channels. Each channel is closed by its
only sender, and the close order is forced by the data flow: trace it
backward from the sums and every close is where it must be.

**Exercise 12-7** *(design)*. A single task maintains a work list it
alone pushes to and pops from. Argue why a `channel` is the wrong type
for that list even though it would work, and name the two properties a
channel charges for that this task does not use. When does the answer
flip?

Solution (discussion): a channel buys synchronization (safe handoff
between tasks) and blocking (a receiver waits for a sender). A
single-task work list uses neither: nothing is handed off and waiting
on yourself is exercise 12-2's one-task deadlock wearing work clothes.
`List` push/pop states the actual invariant (one owner, no
concurrency) and the type system holds you to it; a channel would
advertise a concurrency that does not exist to every future reader.
The answer flips at the moment a second task appears: the day the work
list is fed by a producer or drained by a pool, the channel's two
costs become exactly the two features you need, and the refactor is
chapter 11's worker pool. Types are claims; make the cheapest claim
that is true.

**Exercise 12-10** *(extension · lupin)*. The worklist as a checker:
`balanced(line)` walks a line's characters with a `List[char]` stack —
push every opener among `([{`, pop and compare on every closer. Table
four lines, two of them wolf-shaped code fragments. Three ways to be
unbalanced hide in one function; name all three and the line of your
program that catches each.

Solution. `ch12/ex12-10.lu`:

```wolf
fn balanced(line: str) -> bool {
    var stack = List[char]()
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' {
            (mut stack).push(c)
        } else if c == ')' || c == ']' || c == '}' {
            if stack.len == 0 { return false }
            let open = stack[stack.len - 1]
            (mut stack).pop()
            if c == ')' && open != '(' { return false }
            if c == ']' && open != '[' { return false }
            if c == '}' && open != '{' { return false }
        }
    }
    stack.len == 0
}
fn main() -> !int {
    var cases = List[str]()
    (mut cases).push(r"when (a, b) { c[0] }")
    (mut cases).push(r"select { (a from q => { ) }")
    (mut cases).push(r"([{}])")
    (mut cases).push(r"((")
    for line in cases {
        let verdict = if balanced(line) { "ok " } else { "BAD" }
        print("{verdict} {line}")
    }
    0
}
```

```console
$ lupin ex12-10.lu
ok  when (a, b) { c[0] }
BAD select { (a from q => { ) }
ok  ([{}])
BAD ((
```

The three failures: a closer with nothing open (`stack.len == 0` —
the early `return false`), a closer of the wrong kind (the three
comparisons against the popped opener — the second case dies here,
`)` against `{`), and openers left over at the end (the final
`stack.len == 0`, which is the whole return expression — the fourth
case). The raw-string test data is not decoration: the braces in
`{ c[0] }` would be interpolations in an ordinary literal, so the
checker's own input demonstrates §2.2's reason raw literals exist.

**Exercise 12-11** *(extension · lupin)*. Two stacks make a compiler
front half: convert `3 + 4 * 2 - 6 / 3` from infix to postfix with
one operator stack — pop while the stack's top has precedence at
least the incoming operator's, then push; drain at the end. Feed the
result to 5-7's RPN evaluator in your head to check it. Which single
comparison in your loop decides that `3 + 4 * 2` is 11 and not 14?

Solution. `ch12/ex12-11.lu`:

```wolf
fn prec(op: str) -> int {
    if op == "*" || op == "/" { 2 } else { 1 }
}
fn is_op(t: str) -> bool {
    t == "+" || t == "-" || t == "*" || t == "/"
}
fn main() -> !int {
    let infix = "3 + 4 * 2 - 6 / 3"
    var ops = List[str]()
    var out = ""
    for t in infix.words() {
        if is_op(t) {
            while ops.len > 0 && prec(ops[ops.len - 1]) >= prec(t) {
                out += "{ops[ops.len - 1]} "
                (mut ops).pop()
            }
            (mut ops).push(t)
        } else {
            out += "{t} "
        }
    }
    while ops.len > 0 {
        out += "{ops[ops.len - 1]} "
        (mut ops).pop()
    }
    print(out.trim())
    0
}
```

```console
$ lupin ex12-11.lu
3 4 2 * + 6 3 / -
```

`prec(ops[ops.len - 1]) >= prec(t)` is the whole grammar. When `*`
arrives with `+` on the stack, 1 >= 2 is false, so `+` stays put and
`*` stacks on top — emitted first, which is what binds `4 * 2` before
the add. Flip the comparison's verdict (or make all precedences
equal) and the output evaluates left to right: 14. One integer
comparison is carrying operator precedence for the whole language,
which is the honest size of that famous feature. The `>=` (rather
than `>`) is left-associativity, checkable on the `- … /` tail.

**Exercise 12-8** *(comprehension · lupin)*. Two tasks acquire the
same two mutexes in *opposite* spellings. Predict the total, and
predict what the explorer says about this program, then check both:

```wolf
fn main() -> !int {
    let a = Mutex(1)
    let b = Mutex(2)
    var total = 0
    scope s {
        s.spawn(fn() {
            when (a, b) { a += 10; b += 10 }
        })
        s.spawn(fn() {
            when (b, a) { b += 100; a += 100 }
        })
    }
    when (a, b) { total = a + b }
    print("{total}")
    0
}
```

Solution: 223. Both bodies run whole, in some order, on both
mutexes: 1+2 plus 110 plus 110. `when (b, a)` and `when (a, b)`
perform identical acquisitions because `when` sorts its set into
canonical order before taking anything; the spelling order is
documentation, not semantics. The explorer confirms there is nothing
to find:

```console
$ lupin ex12-8.lu
223
$ lupin conform-run ex12-8.lu --explore=16
ex12-8.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 1 distinct — observably deterministic (every schedule agrees)
    exit(0) ×2 stdout=223\n leaks=0 forest=ok — replay: --seed=0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
```

"Observably deterministic (every schedule agrees)" is the verdict
exercise 12-5's program could not earn. Addition commutes; that is
doing part of the work here, and the stem's real lesson is in 12-9.

**Exercise 12-9** *(extension (break-it-on-purpose) · lupin)*. Now
construct the classic deadlock `when` was designed to kill: task one
takes `a` then `b`, task two takes `b` then `a`, nested. Write it and
report what actually happens: at what phase does this program die?

Solution. `ch12/ex12-9.lu`:

```wolf
fn main() -> !int {
    let a = Mutex(0)
    let b = Mutex(0)
    scope s {
        s.spawn(fn() { when (a) { when (b) { a += 1 } } })
        s.spawn(fn() { when (b) { when (a) { b += 1 } } })
    }
    0
}
```

```console
$ lupin ex12-9.lu
ex12-9.lu: E0201: `when` acquires a set, so it needs at least two operands; for one, call the method on the sync type [gram.expr.conc] at 14:24
$ echo $?
2
```

It dies in the parser. The AB-BA deadlock needs *incremental*
acquisition (hold one lock while asking for another) and `when`'s
grammar has no one-lock form to nest: E0201 says take the set whole or
do not use `when`. The bug is not detected; it is unspellable, which
is a stronger guarantee than any detector. (Deadlock through channels
remains constructible, as exercise 12-2 shows, because waiting for
*data* is a program's own business; acquiring *locks* piecemeal was
never anything but a bug factory.)

Note: both implementations report E0201, with the same rule in their
own words, one at the parse rung and one before the first line runs.
The transcript above is lupin's; §12.4 prints the compiler's half
beside it.
