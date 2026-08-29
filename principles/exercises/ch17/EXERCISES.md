# Chapter 17 — The failing schedule, replayed: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; scheduler controls are
`lupin run FILE --seed=N | --schedule=…` and
`lupin conform-run FILE --explore=N`. The compiler answers
`unsupported` at `resolve` for every program here, so every verdict
below is lupin's.

## §17.1 — The bug that typechecks

**Exercise 17-1** *(comprehension · lupin)*. Two tasks each deposit
50 into a balance that starts at 0, through a get-then-set protocol
with the store loop in `main`. There is no shared mutable capture, no
`Mutex`, nothing chapter 13 would reject. Predict the balance under
the default FIFO schedule — and state what the correct answer would
be if deposits never interfered:

```wolf
fn deposit(getreq: channel[int], getrep: channel[int], setch: channel[int]) {
    getreq.send(1)
    let v = getrep.recv() else |_| { return }
    setch.send(v + 50)
}
fn main() -> !int {
    let getreq = channel[int](0)
    let getrep = channel[int](0)
    let setch = channel[int](0)
    var balance = 0
    scope s {
        s.spawn(fn() { deposit(getreq, getrep, setch) })
        s.spawn(fn() { deposit(getreq, getrep, setch) })
        var served = 0
        while served < 4 {
            select {
                _ from getreq => { getrep.send(balance) },
                v from setch => { balance = v },
            }
            served += 1
        }
    }
    print("balance={balance}")
    0
}
```

Solution: the correct total is 100; FIFO prints 50. Both tasks get 0
before either set arrives, so both compute 0 + 50 and the second set
overwrites the first — the lost update. Every individual message is
delivered exactly once, in order, race-free; the *composite* operation
get-then-set is what was never atomic. This is an ordering bug, not a
data race, and no type system that admits channels can reject it —
which the book says plainly, because the pitch for chapter 13 was
narrower than folklore remembers.

```console
$ lupin ex17-1.lu
balance=50
```

**Exercise 17-2** *(comprehension (schedule play) · lupin)*. Hunt it:
run 17-1 under seeds 0 through 5. Record each balance. Which seeds
produce the correct answer, and what had to happen in the schedule for
100 to come out?

Solution:

```console
$ lupin run ex17-1.lu --seed=0
balance=50
$ lupin run ex17-1.lu --seed=1
balance=100
$ lupin run ex17-1.lu --seed=2
balance=50
$ lupin run ex17-1.lu --seed=3
balance=50
$ lupin run ex17-1.lu --seed=4
balance=100
$ lupin run ex17-1.lu --seed=5
balance=100
```

Seeds 1, 4, and 5 serialize the deposits: one task's set reaches the
store before the other task's get, so the second deposit reads 50 and
writes 100. The failing outcome is not rare — it is the *common* one
here, which inverts the usual heisenbug story: in production this bug
would look like a test that occasionally passes.

The explorer agrees, and says which schedules do it:

```console
$ lupin conform-run ex17-1.lu --explore=500
ex17-1.lu: explored 20 schedule(s) in 20 execution(s) (DPOR; 0 slept, 6 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×16 stdout=balance=50\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0,0,0,0,0,0,0,0,0,0,0,0
    exit(0) ×4 stdout=balance=100\n leaks=0 forest=ok — replay: --seed=4611686018427387910
      decision stream: ev:0,1,1,0,0,0,0,0,0,0,0
  deadlocks: 0 · races: 0 · max depth: 12 decision(s)
$ echo $?
1
```

(The frontier claim is what §17.2 rests its argument on, so it is worth
knowing that it is checked: an earlier interpreter release reported this
program `observably deterministic` while a seeded run printed the wrong
balance, and that disagreement was filed against the interpreter rather
than written around here.)

## §17.2 — The seed, the schedule, and the frontier

**Exercise 17-3** *(fingers · lupin)*. Two sends race into one channel;
main prints the arrival order. Run it twice with `--seed=0`, once with
`--seed=3`, and once with `--schedule=ev:0,0,0`. Before running: which
pairs of those four runs are guaranteed to match?

```wolf
    scope s {
        s.spawn(fn() { ch.send(1) })
        s.spawn(fn() { ch.send(2) })
    }
    let a = ch.recv() else |_| { return 1 }
    let b = ch.recv() else |_| { return 1 }
    print("{a}{b}")
```

Solution: the two `--seed=0` runs must match — a seed selects the
whole decision stream, and the same seed replays byte-identically.
The `ev:0,0,0` run matches them here because that stream is the one
seed 0 selects. `--seed=3` carries no guarantee relative to the
others; it happens to choose the other order:

```console
$ lupin run ex17-3.lu --seed=0
12
$ lupin run ex17-3.lu --seed=0
12
$ lupin run ex17-3.lu --seed=3
21
$ lupin run ex17-3.lu --schedule=ev:0,0,0
12
```

A schedule is a value. Two runs disagree only if their schedules
disagree, and a schedule you can name is a schedule you can rerun —
that is the entire mechanism this chapter's debugging story stands on.

**Exercise 17-4** *(comprehension · lupin)*. The explorer prints the two
schedules of 17-3 as decision streams `ev:0,0,0` and `ev:1,0,0`. Three
decisions, but only the first digit ever differs. What is the first
decision choosing between, and why are the remaining two decisions no
longer choices once it is made?

Solution: the first decision picks which spawned task runs at the
first scheduling point — task one's send or task two's send fires
first. After that, the program has no freedom left: the other send is
the only runnable step, and the two receives in `main` drain the
channel in arrival order. A decision stream records *choices*, not
events; a program's concurrency is measured by how many entries in
that stream could have gone otherwise, and this program has exactly
one.

**Exercise 17-5** *(spelunking · lupin)*. Run
`lupin conform-run ex17-3.lu --explore=64` and read the report back:
explain `explored 2 schedule(s)`, `DPOR`, `frontier closed`,
`SCHEDULE-DEPENDENT`, the per-outcome `replay:` seeds, and the process
exit code.

Solution. The run:

```console
$ lupin conform-run ex17-3.lu --explore=64
ex17-3.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×1 stdout=12\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0,0,0
    exit(0) ×1 stdout=21\n leaks=0 forest=ok — replay: --seed=4611686018427387905
      decision stream: ev:1,0,0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
$ echo $?
1
```

`explored 2 schedule(s)`: the program has two inequivalent orderings,
and both were run. `DPOR` is dynamic partial-order reduction — the
algorithm that knew the other interleavings were equivalent to these
two, so 2 executions covered the space a naive search would have
enumerated. `frontier closed`: no reachable schedule was left
untried within the budget. `SCHEDULE-DEPENDENT`: the outcomes differ
across schedules — a finding, which is why the exit code is 1 even
though every individual run exited 0. Each outcome carries a
`replay:` seed — the finding arrives with its own reproduction
command, which is the difference between a bug report and an anecdote.

## §17.3 (held) — `--chaos`

The chapter ships three sections and this is not one of them: fault
injection has no surface at the pin, so the stem below is written,
its baseline program runs in the corpus, and neither is printed in
`book/ch17.md`. See TOC.md §Deltas, bs07 (ch17's sections).

**Exercise 17-6** *(comprehension · pending — blocker: `--chaos`
fault injection at declared effect points; owner:
s36-deterministic-scheduler)*. `fetch` declares its effect point: a
`recv` that can fail into the `Lost` tag. Under chaos testing the
runtime injects that failure on schedule-chosen runs. State what
outputs a chaos campaign over this program must produce (both of
them), and why a program whose only failure handling is tested by
chaos is better off than one whose failure handling is tested by
outage:

```wolf
fn fetch(ch: channel[int]) -> int ! {Lost} {
    let v = ch.recv() else |_| { return Lost }
    v
}
fn main() -> !int {
    let ch = channel[int](1)
    ch.send(41)
    let v = fetch(ch) else |_| { 0 - 1 }
    print("v={v}")
    if v == 41 || v == 0 - 1 { 0 } else { 1 }
}
```

Solution (prose): the campaign must show `v=41` (no injection) and
`v=-1` (injected `Lost`, absorbed by the caller's handler), both
exiting 0 — the assertion in `main` encodes "either outcome is
acceptable," which is what makes the program chaos-clean. The error
path is code like any other code; untested, it is where the bugs
retire to. Chaos runs it deterministically, on a seed, before an
outage runs it in production without one.

Today, honestly:

```console
$ lupin run ex17-6.lu --chaos
error: unexpected argument '--chaos' found
$ lupin ex17-6.lu
v=41
```

The baseline runs and is pinned by the directive header; the
injection half waits on s36.

## §17.3 — Scope honesty (what exploration cannot see)

**Exercise 17-7** *(comprehension · lupin)*. Rerun 17-3's exploration
with `--explore-preemptions=0`. Predict what the report will claim about
determinism before you run it, then reconcile the claim with 17-5's.

Solution:

```console
$ lupin conform-run ex17-3.lu --explore=64 --explore-preemptions=0
ex17-3.lu: explored 1 schedule(s) in 1 execution(s) (DPOR; 0 slept, 0 pruned), frontier OPEN
  note: preemption bound 0 skipped alternatives; frontier open
  outcomes: 1 distinct — observably deterministic (every schedule agrees)
    exit(0) ×1 stdout=12\n leaks=0 forest=ok — replay: --seed=0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
$ echo $?
0
```

With zero preemptions allowed, only the FIFO schedule is explored, and
the report says "observably deterministic — every schedule agrees":
true over the schedules it looked at, and wrong about the program, as
17-5 proved. The tool is honest about the gap — `frontier OPEN` and
the `note:` line say the search was cut short, and the exit code is 0
only because no finding was reached. Read exploration reports the way
you read benchmarks: the verdict is conditional on the budget line,
and "frontier open" is the condition talking.

**Exercise 17-8** *(design)*. List three behaviors of a real
concurrent service that seeded schedule exploration, as this chapter
defines it, cannot find — and for each, name the tool or practice
that owns it instead. The chapter's own scope-honesty section (§17.4)
claims v1 promises less than folklore expects; your answer is that
claim, made concrete.

Solution (discussion): first, value nondeterminism — a hash seed, a
random backoff, an id from the OS: exploration permutes *scheduling*
decisions, not data, so property tests and fuzzing own that axis.
Second, real time — a timeout that fires only when a peer takes 30
actual seconds, kernel-buffer pressure, the network: the deterministic
scheduler virtualizes time, so what it validates is your *handling*
of a timeout, never the calibration of one; load tests own the
calibration. Third, anything past the FFI membrane — a C library's
internal threads and its file-descriptor games are invisible to a
scheduler that only sees wolf's blocking points; the audit boundary
of chapter 9 and the C library's own test suite own that. Exploration
proves ordering properties over the events it can see and permute —
that sentence, with both clauses stressed, is §17.4's whole content.

## Chapter batch

**Exercise 17-9** *(extension (break-it-on-purpose) · lupin)*. Construct
a deadlock from two tasks and two rendezvous channels, each task
receiving first and sending second. Predict the trap's roster before
running: how many tasks does it name, and why is the answer three when
you wrote two?

Solution. `ch17/ex17-9.lu`:

```wolf
fn main() -> !int {
    let a = channel[int](0)
    let b = channel[int](0)
    scope s {
        s.spawn(fn() {
            let x = a.recv() else |_| { return }
            b.send(x)
        })
        s.spawn(fn() {
            let y = b.recv() else |_| { return }
            a.send(y)
        })
    }
    0
}
```

```console
$ lupin ex17-9.lu
ex17-9.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@231` (task 1), `task@336` (task 2) [conc.deadlock.trap] at 7:5
$ echo $?
3
```

Three, because `main` is blocked too — at the scope join, waiting for
children who are waiting for each other. The roster is the trap's gift:
it names every task and where it blocked, which is the state a
production deadlock never hands you. The trap fires because *every*
live task is blocked with no timer pending — a quiet program and a
deadlocked one differ in exactly that clause, and the runtime can tell
them apart. (Compare 12.4: `when (a, b)` exists so lock-order
deadlocks cannot be written; this exercise built the channel-order
equivalent by hand, on purpose.)
