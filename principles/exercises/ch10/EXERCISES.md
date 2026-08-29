# Chapter 10 — Spawning is a scope: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf conform-run reports
`unsupported` for this chapter's surface, so every checker here is
lupin. Seeded runs use `lupin run FILE --seed=N`.

## §10.1 — The task tree

**Exercise 10-1** *(fingers · lupin)*. Type and run your first scope:
two children each send a number into a channel, and `main` adds what it
receives after the scope closes. Then swap the two `spawn` lines and
run again. What changed?

Solution. `ch10/ex10-1.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](2)
    var total = 0
    scope s {
        s.spawn(fn() { ch.send(20) })
        s.spawn(fn() { ch.send(22) })
    }
    total += ch.recv() else |_| { return 1 }
    total += ch.recv() else |_| { return 1 }
    print("total={total}")
    0
}
```

```console
$ lupin ex10-1.lu
total=42
```

Swapping the spawns changes nothing observable: addition commutes, and
the scope's exit joins both children before the first `recv` runs, so
both values are already in the buffer either way.

**Exercise 10-2** *(comprehension · lupin)*. Predict the order of the
two lines, then say what enforces it — the scheduler, or something
stronger:

```wolf
fn main() -> !int {
    scope s {
        s.spawn(fn() { print("child speaks") })
    }
    print("main speaks")
    0
}
```

Solution: `child speaks` first, always. The scope's closing brace joins
every child; `main speaks` sits after the brace, so it cannot run until
the child has finished — under any seed. This is structure, not luck:

```console
$ lupin ex10-2.lu
child speaks
main speaks
$ lupin run ex10-2.lu --seed=2024
child speaks
main speaks
```

The wrong answer worth ruling out: "the child happened to be scheduled
first." Move the `print` *inside* the scope and the order genuinely is
the scheduler's to choose; after the brace, it is not.

**Exercise 10-3** *(comprehension · lupin)*. A child's last expression
is a value. Predict this program's exit code, and account for the 42:

```wolf
fn main() -> !int {
    scope s {
        s.spawn(fn() { 42 })
    }
    7
}
```

Solution: exit 7. The child's value is discarded at the join — a scope
joins its children for their *completion*, not their results. A child
that has something to say sends it on a channel; the 42 evaporates.

```console
$ lupin ex10-3.lu
$ echo $?
7
```

## §10.2 — The leaked goroutine, retired

**Exercise 10-4** *(extension (break-it-on-purpose) · lupin)*. Port Go's classic
leak: spawn a receiver on a channel that nobody will ever send to. In
Go the goroutine outlives the function, silently, forever. Write the
wolf version and predict what happens instead — and at which line.

Solution. `ch10/ex10-4.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](0)
    scope s {
        s.spawn(fn() {
            let v = ch.recv() else |_| { return 0 }
            print("{v}")
        })
    }
    0
}
```

```console
$ lupin ex10-4.lu
ex10-4.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@196` (task 1) [conc.deadlock.trap] at 6:5
$ echo $?
3
```

The scope's closing brace must join the child; the child is blocked in
`recv`; nothing can unblock it. Where Go leaks quietly, wolf's
structure turns the same mistake into a deadlock the runtime can see —
every live task blocked, so the trap fires and names them. The leak is
not fixed; it is *retired*: this program cannot express "and the task
lingers on unowned."

**Exercise 10-5** *(spelunking · lupin)*. Read exercise 10-4's trap
line clause by clause. What does "no timer is pending" rule out, what
is the "blocked-task roster" for, and why does the trap name `main`
itself as blocked?

Solution (prose): the trap's condition is *every* live task blocked
with nothing left that could wake one. A pending timer — a `timeout`
arm in some `select` — would eventually fire and unblock somebody, so
its absence is part of the proof; the roster is the evidence, one entry
per blocked task with its id, which is the list you would otherwise
assemble by hand from a hung process's stacks. `main` is on the roster
because the join at the scope's brace is itself a blocking point:
`main` is not running the children, it is waiting for them, and waiting
tasks are exactly what a deadlock is made of. The clause tag
`[conc.deadlock.trap]` is the spec's name for the whole rule.

## §10.3 — The dropped error, surfaced

**Exercise 10-6** *(comprehension · lupin)*. Three children compute
through `?`; one of them fails. Predict both printed lines, and name
the exact point in the source where the error crosses from child to
parent:

```wolf
fn risky(n: int) -> int ! {Torn} {
    if n == 3 { return Torn }
    n * 100
}
fn gather() -> !int {
    let ch = channel[int](4)
    scope s {
        s.spawn(fn() { ch.send(risky(1)?) })
        s.spawn(fn() { ch.send(risky(3)?) })
        s.spawn(fn() { ch.send(risky(2)?) })
    }
    0
}
fn main() -> !int {
    let r = gather() else |_| { print("join surfaced the error"); 7 }
    print("{r}")
    0
}
```

Solution: `join surfaced the error`, then `7`. The failing child's `?`
raises `Torn` inside the task; the error travels to the scope's closing
brace — the join — and re-raises there, into `gather`'s own error row,
where `main`'s `else` handles it. The crossing point is the brace. In
Go this error dies in a goroutine unless you built machinery to carry
it; here the structure is the machinery.

```console
$ lupin ex10-6.lu
join surfaced the error
7
```

**Exercise 10-7** *(extension · lupin)*. Change 10-6 so no child
fails (use 1, 2, and 4), then finish the job: close the channel, drain
it, and return the sum. Why is it correct to `close` only after the
scope's closing brace — what has the join already proved by then?

Solution. `ch10/ex10-7.lu`:

```wolf
fn gather_all() -> !int {
    let ch = channel[int](4)
    scope s {
        s.spawn(fn() { ch.send(risky(1)?) })
        s.spawn(fn() { ch.send(risky(2)?) })
        s.spawn(fn() { ch.send(risky(4)?) })
    }
    ch.close()
    var total = 0
    for v in ch { total += v }
    total
}
```

```console
$ lupin ex10-7.lu
total=700
```

After the brace, every child has completed, so every send that will
ever happen has happened — `close` cannot cut anyone off. The join
converts "I hope they are done" into a fact you may compute with.

## §10.4 — Cancellation

**Exercise 10-8** *(comprehension · lupin)*. One sibling blocks
forever; the other fails immediately. Predict all the output, and
answer the pointed part first: does the blocked sibling's `defer` run?

```wolf
fn fail_fast() -> !int { Boom }
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
fn main() -> !int {
    let r = race_them() else |_| { 42 }
    print("{r}")
    0
}
```

Solution: yes — `sibling cleanup ran`, then `42`. The failing child
makes the scope cancel its blocked sibling; cancellation lands at the
sibling's blocking point (`recv`), and the task unwinds *its own*
defers on the way out. Cancellation is polite. Chapter 14 shows the
impolite variant — `kill` on a proc skips defers by design — and the
difference between those two rules is a decided thing, not an
accident.

```console
$ lupin ex10-8.lu
sibling cleanup ran
42
```

## Chapter batch

**Exercise 10-9** *(extension · lupin)*. Build a two-stage pipeline:
a producer sends 1 through 5 into `raw`; a transformer squares each
into `squared`; `main` counts what arrives. Each stage closes the
channel it sends on, when its input runs dry. Run it under two seeds.
Then answer: which task must close `squared`, and what goes wrong if
`main` tries to?

Solution. `ch10/ex10-9.lu`:

```wolf
fn main() -> !int {
    let raw = channel[int](8)
    let squared = channel[int](8)
    var produced = 0
    scope s {
        s.spawn(fn() {
            for i in 1..=5 { raw.send(i) }
            raw.close()
        })
        s.spawn(fn() {
            for v in raw { squared.send(v * v) }
            squared.close()
        })
        for v in squared { produced += 1 }
    }
    print("{produced} values through the pipeline")
    0
}
```

```console
$ lupin ex10-9.lu
5 values through the pipeline
$ lupin run ex10-9.lu --seed=2024
5 values through the pipeline
```

Only the transformer knows when the last square has been sent — it
learns it from its own `for` loop ending, which happens when `raw`
closes and drains. If `main` closed `squared`, it would be guessing;
close is the sender's verb, and each stage owns exactly one sending
side. That ownership discipline is the whole pipeline pattern.

**Exercise 10-10** *(design)*. Go has `go f()`; wolf deliberately has
no detached spawn — a task needs a scope, and the scope must close.
Take the other side seriously: name a real program shape that detached
spawn serves well, sketch how wolf expresses it, and state what the
wolf version pays and what it collects.

Solution (discussion): the honest case for detachment is the
fire-and-forget notifier — a metrics ping, a log ship — where the
caller genuinely does not want to wait and failure is acceptable. Go
spells it in three characters. Wolf makes the lifetime explicit: the
ping lives in some scope — a long-lived one owned by the subsystem
that cares about pings, with the pattern chapter 11 builds. The
payment is real: you must decide *whose* scope, which is a design
question Go let you skip. What it collects: the answer to "can this
program exit with work still running" is knowable by reading the
scopes, every error has an owner, and the leak of exercise 10-4 is
unwritable. Wolf's position is that "whose is this task" was never
optional — Go defers the question to a runtime that cannot answer it,
and wolf asks it at the point where you still can.
