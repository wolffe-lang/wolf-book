# Chapter 14 — Procs: the unit of failure: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins. The compiler's `conform-run`
reports `unsupported` for proc programs today; this chapter is lupin
territory until s34 lands.

## §14.1 — Armstrong's argument, one page

**Exercise 14-1** *(comprehension · lupin)* — A proc's function returns
`3`. Before running, predict both fields of the line this prints:

```wolf
fn worker() -> int { 3 }
fn main() -> !int {
    let w = spawn proc worker()
    let m = w.monitor()
    select {
        exit(reason) from m => {
            print("normal={reason.is_normal()} killed={reason.is_killed()}")
        },
        timeout(1.s) => { return 1 },
    }
    0
}
```

Solution: `normal=true killed=false`. Returning a value — any value —
is a normal exit; the value rides along in the reason. Failure is a
separate channel of information: an error return or a kill, not a
number the supervisor squints at. Erlang made the same cut, and for
the same reason: the exit *reason* is the protocol, the return value
is the result.

```console
$ lupin ex14-1.lu
normal=true killed=false
```

**Exercise 14-2** *(design)* — A log-search service has three
concerns: an HTTP listener, a query planner, and one index shard per
disk. Argue where the proc boundaries go. For each boundary you draw,
name the failure it isolates and the state that dies with it; for one
boundary you chose *not* to draw, name what shared fate you accepted.

Solution (discussion): the strong answer puts each shard in its own
proc — a corrupt index or a crashing decompressor takes down one
shard's regions and nothing else, and the planner degrades to
partial results. The listener and planner can share a proc at first:
they share fate anyway (no listener, no queries), and splitting them
buys isolation only once the planner holds cache state worth keeping
through a listener restart. The boundary not drawn is the one between
planner and its per-query scratch — that is a region inside the proc,
not a proc: it dies with the query, and promoting it to a proc would
turn cheap bulk-frees into protocol. Ownership, failure, and service
land on the same line or the design fights itself — that is
Armstrong's argument, applied.

## §14.2 — Crash means bulk-free

**Exercise 14-3** *(comprehension · lupin)* — `build_then_crash`
allocates a hundred integers into a region, then returns an error.
Predict what the monitor reports and, separately, what happened to the
hundred integers — then say which line of code freed them.

Solution — `ch14/ex14-3.lu` (excerpt):

```wolf
fn build_then_crash() -> !int {
    let r = region()
    let n = in r {
        var xs = List[int]()
        for i in 0..100 { xs.push(i) }
        xs.len
    }
    Boom
}
```

```console
$ lupin ex14-3.lu
proc down; its regions are gone
```

No line of code freed them. The proc's death is the deallocation: a
proc owns its regions, and an abnormal exit frees them wholesale —
no unwinding, no per-object teardown. The monitor's `exit(reason)`
arrives after the memory is already gone.

**Exercise 14-4** *(comprehension · lupin)* — `sleeper` registers a
defer and then blocks forever on an empty channel; the owner kills it.
Two prints are written in this program: `defer-skipped` in the proc
and `released` in the owner. Predict which of them appear, and in what
order:

```wolf
fn sleeper() -> int {
    defer print_raw("defer-skipped")
    let ch = channel[int](0)
    let v = ch.recv() else |_| { return 1 }
    v
}
fn main() -> !int {
    let w = spawn proc sleeper()
    let m = w.monitor()
    w.kill()
    select {
        exit(reason) from m => {
            if reason.is_killed() { print_raw("released") } else { print_raw("wrong") }
        },
        timeout(1.s) => { print_raw("timeout") },
    }
    0
}
```

Solution: only `released`. A killed proc's defers do not run — this is
the decided rule, not an accident: a kill must be safe to issue against
a proc in any state, and running arbitrary cleanup code in a proc that
is being destroyed for misbehaving is not safe. The proc's regions
bulk-free instead. Resources that must survive a kill belong to the
owner's side of a channel, or to a supervisor.

```console
$ lupin ex14-4.lu
released
```

**Exercise 14-5** *(comprehension · lupin)* — The same shape at task
granularity. One sibling blocks on a channel with a defer registered;
the other fails. Predict the output — and then state, in one sentence
each, why this defer runs when 14-4's did not:

```wolf
fn race_them() -> !int {
    let ch = channel[int](0)
    scope s {
        s.spawn(fn() {
            defer print("sibling cleanup ran")
            let v = ch.recv()?
            v
        })
        s.spawn(fn() { fail_fast() })
    }
    0
}
```

Solution: the cleanup line prints, then the caller's handler value is
used:

```console
$ lupin ex14-5.lu
sibling cleanup ran
caught
```

Cancellation is cooperative: it arrives at a blocking point in a task
that is still healthy, so its defers run. A kill is unilateral: it
destroys a proc that may be past cooperating, so they do not. One
cleanup contract per failure primitive, and the primitive tells you
which one you are holding.

## §14.3 — Mailboxes

**Exercise 14-6** *(fingers · lupin)* — Build a counting service: a
proc that reads commands from a channel, where `0` means "reply with
the total" and any other value adds to it. Drive it with 5, 2, then a
report, and print what comes back.

Solution — `ch14/ex14-6.lu`:

```wolf
fn counter(cmds: channel[int], replies: channel[int]) -> int {
    var total = 0
    for c in cmds {
        if c == 0 { replies.send(total) } else { total += c }
    }
    total
}
fn main() -> !int {
    let cmds = channel[int](8)
    let replies = channel[int](1)
    let w = spawn proc counter(cmds, replies)
    cmds.send(5)
    cmds.send(2)
    cmds.send(0)
    let t = replies.recv() else |_| { return 1 }
    print("total={t}")
    cmds.close()
    0
}
```

```console
$ lupin ex14-6.lu
total=7
```

The proc's `for c in cmds` loop *is* the mailbox: commands queue in
the channel, the proc serializes them, and the reply channel carries
answers back. No lock appears because no state is shared — the total
lives in exactly one proc.

**Exercise 14-7** *(extension · lupin)* — Grow the protocol: `-1`
resets the counter. Report the total, reset, add 3, and report again.
Predict both numbers first, then answer: what ordering guarantee makes
your prediction safe, and which chapter taught it?

Solution — `ch14/ex14-7.lu` (excerpt):

```wolf
    for c in cmds {
        if c == 0 { replies.send(total) } else if c == 0 - 1 { total = 0 } else { total += c }
    }
```

```console
$ lupin ex14-7.lu
before=7 after=3
```

`before=7 after=3` is safe to predict because one sender's sends
arrive in order (chapter 12's happens-before): the reset cannot
overtake the first report, and the 3 cannot overtake the reset. The
protocol's integers are starting to strain — a real command set wants
a type, which is where rows and enums earn their keep.

**Exercise 14-8** *(design)* — Erlang mailboxes offer *selective
receive*: a proc can pluck the first message matching a pattern,
leaving the rest queued. Wolf's mailbox is a FIFO channel plus
`select` over multiple channels. State one protocol that selective
receive expresses more directly, then argue wolf's side: what does a
skipped-over message cost in Erlang that wolf's design refuses to pay?

Solution (discussion): call-response over a shared mailbox is the
classic selective-receive win — reply matching happens by pattern
while unrelated traffic waits. The cost is the unbounded scan: every
receive may walk the whole queue, and a proc that never matches some
message class leaks queue memory silently; Erlang folklore is full of
mailboxes that grew until the node died. Wolf's answer is one channel
per conversation — the reply channel in 14-6 is exactly that — which
turns pattern-matching into channel topology the scheduler can see.
The queue that would have grown silently becomes a channel you had to
declare, with a capacity you chose. Less expressive per receive;
every queue in the program has a name and a bound.

## Chapter batch

**Exercise 14-9** *(comprehension + schedule play · lupin)* — Two
client tasks each send two increments to the counting proc; the scope
joins, then main asks for the total. Run it under seeds 0, 1, 5, 9.
Predict first: does the total vary with the schedule, and why not —
and name the thing that *does* vary between those runs even though no
output shows it.

Solution — `ch14/ex14-9.lu` (main excerpt):

```wolf
    scope s {
        s.spawn(fn() { client(cmds) })
        s.spawn(fn() { client(cmds) })
    }
    cmds.send(0)
```

```console
$ lupin run ex14-9.lu --seed=0
total=4
$ lupin run ex14-9.lu --seed=1
total=4
$ lupin run ex14-9.lu --seed=5
total=4
$ lupin run ex14-9.lu --seed=9
total=4
```

The total is schedule-independent: the scope join happens-before the
report command, so all four increments are in the mailbox before `0`
enters it. What varies is the interleaving of the two clients' sends —
the mailbox's arrival order — which addition happens to hide, because
addition commutes. Replace `total += c` with an operation that does
not commute and the seeds stop agreeing; chapter 17 hunts exactly that
program.
