# Chapter 15 — Link, monitor, supervision: exercises

Commands run from this directory; outputs are pasted from real runs.
lupin is the interpreter `wolf-toolchain.toml` pins; proc programs are `unsupported`
under the compiler's `conform-run` until s34.

## §15.1 — Two primitives

**Exercise 15-1** *(comprehension · lupin)* — Chapter 14 showed a proc
returning a value: `is_normal()` was true. This proc returns an error
instead. Predict both fields — there are three possible exit shapes
and this line can only show you two booleans:

```wolf
fn boom() -> !int { Bad }
fn main() -> !int {
    let w = spawn proc boom()
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

Solution: `normal=false killed=false`. Both predicates are false
because an error return is the third shape: not a normal completion,
not a kill, but a failure the proc itself reported. The reason
taxonomy is three-valued — normal, killed, errored — and a supervisor
that only checks `is_normal()` treats the last two the same, which is
usually what it wants.

```console
$ lupin ex15-1.lu
normal=false killed=false
```

**Exercise 15-2** *(comprehension · lupin)* — `monitor` delivers a
message; `link` shares fate. This program links to a proc that fails,
then blocks on an empty channel. Two prints are written. Predict what
appears on stdout, and what `echo $?` shows:

```wolf
fn boom() -> !int { Bad }
fn main() -> !int {
    let w = spawn proc boom()
    w.link()
    let ch = channel[int](0)
    let v = ch.recv() else |err| { print("recv failed"); return 7 }
    print("got {v}")
    0
}
```

Solution: nothing appears, and the exit code is 1. The link propagates
the failure into `main` at its blocking point — no error value arrives
at the `else` handler, because shared fate is not an error return; it
is death. The handler that never ran is the lesson: `link` is for
"if it dies, we die," and code below a link is written in that
knowledge. Choose `monitor` when failure is information; choose
`link` when failure is contagion, and mean it.

```console
$ lupin ex15-2.lu
$ echo $?
1
```

**Exercise 15-3** *(design)* — A pipeline proc feeds a compressor
proc, which feeds an uploader proc. For each of the three pairs,
choose `link` or `monitor` and defend the choice with the failure you
are designing for. One of the three answers should be "neither" —
which, and what replaces it?

Solution (discussion): compressor–uploader wants `link`: a compressor
with no uploader is doing work nobody will receive, and vice versa —
shared fate matches the data dependency. Pipeline–compressor wants
`monitor` in the supervisor above them, not a link between them: the
pipeline can buffer or reroute while a replacement compressor spins
up, so failure is information there. The "neither" pair is whichever
one you were tempted to wire both directions: a link is already
bidirectional, and a link plus a monitor between the same two procs
means the fate decision was never actually made. The supervisor tree
makes these choices once, at the top, where the restart policy lives —
wiring fate ad hoc between siblings is how systems grow failure paths
nobody drew.

## §15.2 — A supervisor in forty lines

**Exercise 15-4** *(fingers · lupin)* — Build the smallest supervisor:
spawn a worker that fails on its first attempt and succeeds on its
second; monitor it; on an abnormal exit, print a line and respawn with
the next attempt number; stop after three attempts. Run it and keep
the output.

Solution — `ch15/ex15-4.lu`:

```wolf
fn worker(attempt: int) -> !int {
    if attempt == 1 { return Crash }
    0
}
fn main() -> !int {
    var attempt = 1
    var done = false
    while !done {
        let w = spawn proc worker(attempt)
        let m = w.monitor()
        select {
            exit(reason) from m => {
                if reason.is_normal() { done = true } else {
                    print("attempt {attempt} failed; restarting")
                    attempt += 1
                    if attempt > 3 { return 1 }
                }
            },
            timeout(1.s) => { return 1 },
        }
    }
    print("attempt {attempt} succeeded")
    0
}
```

```console
$ lupin ex15-4.lu
attempt 1 failed; restarting
attempt 2 succeeded
```

The whole mechanism is visible: a loop, a monitor, a judgment on the
reason, a bounded retry. The stdlib's supervisor adds policy — restart
strategies, intensity limits, child ordering — but no new primitive.
You have now built the thing the next section hands you.

**Exercise 15-5** *(comprehension · lupin)* — Same supervisor, but the
worker fails every time. Predict the full output and the exit code
before running — including how many times the worker actually runs.

Solution: three failures, a give-up line, exit 1. The budget is the
supervisor's honesty: without it, a deterministic crash loops forever,
and "restarting" degrades from recovery into denial. Three runs, not
four — the budget check happens after the increment, and off-by-one
predictions here are worth catching on paper rather than in a pager
rotation.

```console
$ lupin ex15-5.lu
attempt 1 failed
attempt 2 failed
attempt 3 failed
giving up after 3 attempts
$ echo $?
1
```

**Exercise 15-6** *(extension · lupin)* — Change the worker to fail
twice and succeed on the third attempt — the flappy dependency
pattern. Predict the output, run it, and then answer: your budget is
3. What single-character change makes this worker's recovery
impossible, and what does the output become?

Solution — `ch15/ex15-6.lu` (worker):

```wolf
fn worker(attempt: int) -> !int {
    if attempt < 3 { return Crash }
    0
}
```

```console
$ lupin ex15-6.lu
attempt 1 failed; restarting
attempt 2 failed; restarting
attempt 3 succeeded
```

Change the budget check `attempt > 3` to `attempt > 2` (or the
worker's `< 3` to `< 4`): the third attempt — the one that would have
succeeded — is never made, and the output becomes two failures and an
exit 1. Restart budgets are a bet about how flappy the world is;
this exercise is that bet lost by one.

## §15.3 — The root supervisor

**Exercise 15-7** *(design)* — Every proc in wolf lives under the root
supervisor; there is no unsupervised spawn. Sketch the supervision
tree for the chapter 14 log-search service (listener, planner, one
proc per shard), choosing for each internal node: restart the child
alone, restart all children, or escalate. Name the failure scenario
that made you pick each policy.

Solution (discussion): the shards sit under one internal supervisor
with restart-alone: shard failures are independent (one bad disk, one
corrupt segment), and restarting siblings would throw away warm
caches for no reason. The listener and planner sit under a
restart-all node: a planner that died mid-query leaves the listener
holding connections whose queries will never answer — restarting them
together resets the pair to a consistent nothing. The shard
supervisor escalates only when its restart budget exhausts — at which
point the service cannot search, and the root's policy (restart the
world, or die and let the OS supervisor act) is a deployment
decision, not a code decision. The tree is the failure design; the
procs are only its leaves.

**Exercise 15-8** *(spelunking · corpus)* — The pinned corpus checks
the kill rule with this directive header, from
`upstream/corpus/conc/proc_kill_defers.lu`:

```text
//! check: run(exit=0, stdout="released")
```

Explain why the `stdout=` clause — not the exit code — is the part of
this header that actually verifies the rule "defers in a killed proc
do not run." What would a conforming-looking run that violates the
rule produce, and which field would catch it?

Solution: the program is built so the killed proc's defer would print
`defer-skipped` — so a run where defers wrongly ran produces
`defer-skippedreleased` (or an interleaving of both), and only the
`stdout="released"` check sees the difference; the exit code is 0
either way. The header is a negative test smuggled into a positive
one: the asserted output is exact, so any extra byte from the
forbidden defer is a corpus failure. When a rule's violation is
silent in the exit code, pin the channel where it is loud — a
directive that checks the wrong observable verifies nothing.

## Chapter batch

**Exercise 15-9** *(design)* — A teammate proposes: "monitors are
strictly better — a link is a monitor whose handler calls exit, so
the language should ship only monitors." Take the other side using
15-2's observed behavior: name two properties of `link` that the
monitor-plus-handler encoding does not provide.

Solution (discussion): first, a link works while you are blocked —
15-2's `main` died at a `recv` it would never leave; the encoding's
handler only runs when its select next polls the monitor, so a
blocked proc survives exactly when it should not. Second, a link
cannot be forgotten on one path: the handler encoding must appear in
every select the proc ever blocks in, and the one select that omits
the monitor arm is an unsupervised wait. Fate declared once at wiring
beats fate re-asserted at every blocking point — the encoding is
strictly more expressive and reliably less used. (The full story adds
delivery guarantees under simultaneous failure; chapter 17's
scheduler material is where that becomes checkable.)
