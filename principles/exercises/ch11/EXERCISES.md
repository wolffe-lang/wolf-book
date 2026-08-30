# Chapter 11 — Scopes as values: exercises

Commands run from this directory; outputs are pasted from real runs.
Every checker in this chapter is lupin; wolf conform-run reports
`unsupported` for the concurrency surface.

## §11.1 — The scope as a capability

**Exercise 11-1** *(fingers · lupin)*. A function cannot spawn unless
somebody hands it a scope. Write `launch(s, ch, n)` that spawns into a
caller's scope, and a `main` that calls it three times inside one `scope`
block. The `Scope` parameter is the entire mechanism: nothing else in
the signature says "concurrent."

Solution. `ch11/ex11-1.lu`:

```wolf
fn launch(s: Scope, ch: channel[int], n: int) {
    s.spawn(fn() { ch.send(n * 10) })
}
fn main() -> !int {
    let ch = channel[int](3)
    var total = 0
    scope s {
        launch(s, ch, 1)
        launch(s, ch, 2)
        launch(s, ch, 3)
    }
    for _ in 0..3 { total += ch.recv() else |_| { return 1 } }
    print("{total}")
    0
}
```

```console
$ lupin ex11-1.lu
60
```

**Exercise 11-2** *(comprehension · lupin)*. Take 11-1 and change one
character: make the channel a rendezvous, `channel[int](0)`. Predict
precisely what happens and why: the answer involves which side of the
scope's closing brace the receives sit on.

Solution: deadlock. With no buffer, each child's `send` blocks until
someone receives; the receives are after the scope's brace; the brace
is a join that waits for the children. Children wait for `main`, `main`
waits for children, and the trap names all four:

```console
$ lupin ex11-2.lu
ex11-2.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@219` (task 1), `task@219` (task 2), `task@219` (task 3) [conc.deadlock.trap] at 11:5
$ echo $?
3
```

The buffered version worked because capacity 3 let every send complete
without a receiver. Buffer size here is part of the program's
correctness argument, not a tuning knob.

**Exercise 11-3** *(comprehension · lupin)*. Using only the text of
11-1's program, answer: which functions in it are able to spawn tasks,
and what single search over a large codebase would find every function
with that ability? (Chapter 7 asked the same question about mutation.)

Solution (prose): `main` can spawn (it owns a `scope` block) and
`launch` can spawn (it receives a `Scope`). Nothing else can. The
search is for `Scope` in parameter lists plus `scope` blocks: the
spawn surface is exactly the set of functions the type system shows
holding the capability, the same audit `grep '(mut '` performs for
mutation. A capability you can grep for is a capability you can
review.

## §11.2 — The background refresher

**Exercise 11-4** *(extension · lupin)*. Build a worker pool: three
workers share one `jobs` channel and one `results` channel; `main` feeds
six jobs and closes. Each worker is the same four lines. Why does the
pool need no "shut down workers" message?

Solution. `ch11/ex11-4.lu`:

```wolf
fn main() -> !int {
    let jobs = channel[int](8)
    let results = channel[int](8)
    scope s {
        for w in 0..3 {
            s.spawn(fn() {
                for j in jobs { results.send(j * j) }
            })
        }
        for j in 1..=6 { jobs.send(j) }
        jobs.close()
    }
    results.close()
    var total = 0
    for r in results { total += r }
    print("total={total}")
    0
}
```

```console
$ lupin ex11-4.lu
total=91
```

A worker's loop ends when `jobs` closes and drains: the close *is*
the shutdown message, broadcast to every receiver at once. The scope's
brace then proves all workers are gone before `results` is touched.
Two channel closes and one join replace the ad-hoc "poison pill"
protocols other ecosystems teach.

**Exercise 11-5** *(comprehension + schedule play · lupin)*. Shrink the
pool to two workers and four jobs, and tag each result with the worker
that produced it. Before running: is the *assignment* of jobs to workers
part of the program, or part of the schedule? Run under seed 1 and seed
2024 and defend your answer with the outputs.

Solution. `ch11/ex11-5.lu` (excerpt):

```wolf
scope s {
    for w in 0..2 {
        s.spawn(fn() {
            for j in jobs { results.send("worker {w} took job {j}") }
        })
    }
    for j in 1..=4 { jobs.send(j) }
    jobs.close()
}
```

```console
$ lupin run ex11-5.lu --seed=1
worker 1 took job 1
worker 1 took job 2
worker 1 took job 3
worker 1 took job 4
$ lupin run ex11-5.lu --seed=2024
worker 0 took job 1
worker 0 took job 2
worker 0 took job 3
worker 0 took job 4
```

The assignment is the schedule's: seed 1 lets worker 1 drain the whole
queue, seed 2024 hands it to worker 0, and both are conforming runs of
the same program. What the program owns is the *set* of results: four
squares would be identical in every schedule, which is exercise 11-4's
sum. Write programs whose meaning lives in what is computed, not in
who computed it; the seeds exist to catch you when you have not.

## §11.3 — The structured dump

**Exercise 11-6** *(spelunking · lupin REPL)*. Turn on the trace and
run a scope with two children, then read the scheduler's own account:

```console
$ lupin
wolf> :trace on
trace on: every rule firing is recorded with its clause anchor
wolf> scope s { s.spawn(fn() { print("a") }); s.spawn(fn() { print("b") }) }
a
b
wolf> :trace
     0..0      SchedSpawn [conc.task.spawn] ev#3 spawn `task@63` (task 2) under scope#0 in proc#0
    40..68     Assign [gram.expr.assign] write `s`
    40..68     ProvState [mem.prov.state] write alloc#0[0..1) through tag#2 — tree consistent
    40..68     ProvTag [mem.prov.tag] protector on tag#2 (`t0:0:s`) released: the call's extent ended
     8..70     Block [gram.expr.block] block yields its tail
    31..34     StrInterp [str.interp] f-string
    31..34     EvalStrictOrder [mem.model.order] 1 argument(s) evaluated left to right
    23..37     Block [gram.expr.block] block yields its tail
     0..0      TaskJoin [conc.task.join] ev#4 `main` blocks at scope#0's exit join
     0..0      SchedPark [conc.det.events] ev#5 park `main` (task 0)
     0..0      SchedDecision [conc.det.events] ev#6 schedule `task@33` (task 1), picked 0 of 2 ready
     0..0      TaskName [conc.task.name] ev#7 task `task@33` (task 1) completes: ok
    61..64     StrInterp [str.interp] f-string
    61..64     EvalStrictOrder [mem.model.order] 1 argument(s) evaluated left to right
    53..67     Block [gram.expr.block] block yields its tail
     0..0      SchedDecision [conc.det.events] ev#8 schedule `task@63` (task 2), picked 0 of 1 ready
     0..0      TaskName [conc.task.name] ev#9 task `task@63` (task 2) completes: ok
     0..0      SchedUnpark [conc.det.events] ev#10 unpark `main` (task 0)
     0..0      SchedDecision [conc.det.events] ev#11 schedule `main` (task 0), picked 0 of 1 ready
     0..0      TaskJoin [conc.task.join] ev#12 scope#0 joins: all 2 child(ren) complete
wolf> :quit
```

From the trace alone, reconstruct the task tree: which tasks exist,
who owns them, and in what order they completed.

Solution (prose): three tasks. `main` is task 0; `task@33` (task 1)
and `task@63` (task 2) were spawned under `scope#0`, which `main`
owns, inside `proc#0`. The completion order is written in the
`TaskName` events: task 1 at ev#7, task 2 at ev#9, and then `main`
unparks and the scope joins at ev#12 with "all 2 child(ren) complete."
The dump is not a stack sample; it is the ownership tree the language
defined, reported by the machine that enforced it.

**Exercise 11-7** *(comprehension · lupin REPL)*. In 11-6's trace,
find every `SchedDecision` line and read its "picked 0 of N ready"
suffix. At which event did the scheduler actually have a choice, and
what does that tell you about how many *different* traces this
one-line program could produce?

Solution (prose): only ev#6 offered a choice: "picked 0 of 2 ready,"
with both children runnable. ev#8 and ev#11 each had one ready task,
which is no decision at all. One binary choice, so two inequivalent
schedules exist: task 1 first or task 2 first, precisely the two
outputs `a b` and `b a`. Counting the "of N ready" suffixes is a hand
computation of what chapter 17's `--explore` computes for real
programs; do it once by eye to believe the tool.

## Chapter batch

**Exercise 11-8** *(design)*. A library offers
`fetch_all(urls: List[str]) -> List[Response]` and wants to fetch
concurrently. Two candidate signatures:

```wolf
fn fetch_all(urls: List[str]) -> List[Response]
fn fetch_all(s: Scope, urls: List[str]) -> List[Response]
```

The first hides an internal scope; the second borrows the caller's.
Argue for each: who controls cancellation and lifetime in each design,
and which caller is each one honest to?

Solution (discussion): the internal-scope version is honest to the
caller who wants a blocking call: when it returns, no task it started
survives. The function is externally sequential, concurrency as an
implementation detail, nothing to cancel from outside because nothing
outlives the call. The `Scope` parameter is honest to the caller who
wants to *compose* lifetimes: the fetches join when the caller's scope
closes, so the caller can hang ten calls on one scope and cancel the
lot by leaving it. But the signature now admits that tasks may
outlive the call itself, and every reader of the call site must look
up to find the brace those tasks die at. The library rule of thumb
wolf's std follows: take a `Scope` when the work's lifetime is
legitimately the caller's decision; keep the scope internal when the
function's contract is "done means done." The wrong design is the
secret third one: an internal scope that detaches work past its own
return, which is the chapter 10 leak wearing a signature.
