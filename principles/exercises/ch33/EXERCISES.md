# Chapter 33 — The serving loop

Seven exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`. Five carry a program and two do not: 33-5 is
reading and 33-6 is design.

Every program here binds `127.0.0.1:0` and waits on a bounded deadline,
which is the same discipline the chapter teaches. Nothing in this
directory touches a fixed port or an address outside the machine.

## §33.1 — A door of your own

**Exercise 33-1** *(fingers · lupin)*. Change the cold open's request from
`"regions"` to a title the shelf does not hold, predict the printed line
before you run it, then run it. Then move `net_close(door)` above
`net_accept` and say, in one sentence, which of the two programs on this
page you have written.

Solution. `ex33-1.lu`.

```console
$ lupin ex33-1.lu
shelves: unknown
```

The prediction to make is that nothing fails. `words_in` walks the shelf
and returns `"unknown"` when it falls off the end, so a miss is a string
the caller receives like any other answer, and the row machinery is never
reached. That is a design decision worth naming: a shelf that knows its
whole contents can answer "not here" as data, and only a lookup that
might fail for a reason the caller could act on deserves a row.

The second half writes the other program on the page. Closing the door
before accepting is §33.1's second block: the listener is gone, the
caller's dial has nowhere to land, and the answer is `refused`. The two
programs differ by the order of two lines.

## §33.2 — Waiting on the whole set

**Exercise 33-2** *(comprehension · lupin)*. Delete the `net_accept` line
and the `conn` variable from the program above, leaving the fourth
`net_wait` in place. Predict the fourth line before you run it, then run
it, and explain the answer using one sentence of this section.

Solution. `ex33-2.lu`.

```console
$ lupin ex33-2.lu
nobody there: 0 ready
someone there: 1 ready, and it is the door: true
looking took nothing: 1 still ready
still nobody took it: 1 ready
```

The fourth line is `1 ready`, not `0`. The sentence that explains it is
the section's third rule: asking consumes nothing. In the chapter's
version the count drops to zero because `net_accept` took the connection
and the door genuinely has nothing pending; here nothing was taken, so
the door is still holding it and every wait says so. Readiness reports
the state of the socket, not the history of your looking, which is why a
loop that forgets to accept spins at full speed instead of blocking —
the wait keeps returning immediately, correctly, forever.

## §33.3 — The loop

**Exercise 33-3** *(extension · lupin)*. The loop above never lets go of a
connection. Give it the other half: when `net_read` answers the `closed`
row, close that handle and remove it from `open` instead of writing back.
Then say why the program cannot learn that a peer has gone without
reading it.

Solution. `ex33-3.lu`.

```console
$ lupin ex33-3.lu
served 2, dropped 2
```

Two changes and one helper. The read grows an `else` that sets a flag
rather than propagating, the branch under it closes the handle and
rebuilds `open` without it, and `without` is the three-line filter that
does the rebuilding. The loop's exit condition moves from "two served" to
"two dropped", which is the honest one for a server: a connection is
finished when its peer says so, not when you have answered enough.

Why the read is unavoidable: readiness is the only thing the wait
reports, and a connection whose peer has closed is ready. It will be
named by every `net_wait` from now until something reads it, because
there genuinely is something to read — the end of the stream. The read is
what turns that into the `closed` row, and the close is what takes the
handle out of the set. A loop that watches without reading gets a wait
that returns instantly every time and a process at full cpu with nothing
to show for it.

## §33.4 — Many hands on one door

**Exercise 33-4** *(comprehension · lupin)*. `net_wait` declares one row,
`io`, and one of the things it means is a set that could never become
ready. Write the four-line program that produces it — an empty list and a
deadline that never expires — and predict the exit code before you run it.
Then say what a program that meant to sleep should have called instead.

Solution. `ex33-4.lu`.

```console
$ lupin ex33-4.lu
error: io
$ echo $?
1
```

There is no handle that could become ready and no clock that could end
the wait, so the call cannot answer and cannot honestly park: it takes
its one row. Propagated with `?` from `main`, a row is the documented
process outcome — the tag on stdout and exit 1 — which is chapter 6's
rule and not a network rule.

The program that meant to sleep calls `time_sleep_ms`. It is worth
noticing what the alternative designs would have cost: a wait that
returned an empty answer here would be indistinguishable from a healthy
timeout, and a wait that parked forever would be a hang with no name on
it. The row is the only answer that tells the caller which mistake it
made.

## The chapter batch

**Exercise 33-5** *(spelunking · prose)*. §33.4 says a lost accept race is
not an error row. Read `[os.net.accept]` in the specification and write
down the two candidate rows it rejects and the one sentence of reasoning
that rejects each. Then say which of the two would have been the more
tempting mistake.

Solution (discussion): the two candidates are a new row of its own and
the existing `timeout`.

A new row is rejected because a lost race carries nothing: from the
program's side it is indistinguishable from the connection never having
arrived, which is the same thing one hand alone already meets when a peer
aborts between the wake and the take. A row would therefore make
`net_accept(l)?` fail on a stranger's reset — a program with no budget
armed would start reporting an error it can do nothing about.

`timeout` is rejected because it would be a lie about a clock: a program
that armed no deadline would be told its deadline expired.

`timeout` is the more tempting mistake, and by some distance. It costs no
new vocabulary, it is already in the call's row, and it is what the hand
eventually answers anyway when the budget does run out — so a hurried
implementation reaches for it and the difference only shows on the path
where no budget was armed at all. The rejected reasoning is worth keeping
because it generalizes: a row is a claim, and a call should not make a
claim it cannot support.

**Exercise 33-6** *(design)*. A configuration file says `workers auto`.
Write down the rule your program follows: what it starts when `os_cpus`
answers, what it starts when `os_cpus` answers `io`, and what it logs in
each case. Then defend the number you chose for the second case against
the two obvious alternatives.

Solution (discussion): when `os_cpus` answers `n`, start `n` workers and
log the number and where it came from — "starting 4 workers (os_cpus)" —
because the number is a decision the operator will want to audit against
the container they think they gave you.

When it answers `io`, start one worker and log that the count was not
learned: "starting 1 worker (core count unavailable: io)". One, plus a
line that says the count is a fallback rather than a measurement.

The two alternatives are worse in different directions. Refusing to start
turns a question the program could not answer into an outage, and the
program's job is to serve; a machine whose core count is unreadable can
still answer requests. Guessing a plausible number — four, eight,
whatever the last machine had — is worse still, because it looks like a
measurement in the log and will be believed. One worker is honest: it is
the smallest thing that works, it cannot oversubscribe a container, and
the log line tells the operator exactly which knob to set by hand. This
is also why `os_cpus` has a row instead of quietly answering 1 — a silent
1 would have made this decision for you, and made it invisible.

**Exercise 33-7** *(extension · lupin)*. §33.3's loop answers one question
per connection and forgets it. Make the shelf answer two: keep serving a
connection until its read answers `closed`, and have each caller ask twice
before hanging up. Which of the loop's data structures had to change, and
which did not?

Solution. `ex33-7.lu`.

```console
$ lupin ex33-7.lu
answered 4 questions on 2 connections
```

Nothing had to change. `open` is the same list, the set is built the same
way, and the body of the loop is the same three branches: the door is
accepted, a readable connection is read and answered, a closed one is
dropped. What changed is a counter — the loop leaves when two connections
have closed rather than when two questions have been answered — and that
is the point of the exercise. A loop written around readiness is already
a loop that serves a conversation; it was only the exit condition that
assumed one question.

The callers are tasks here so that one process can hold both ends of two
conversations at once. That is a page-fitting convenience and not part of
the lesson: `net_wait` neither parks a task nor disturbs the scheduler, so
the loop is unchanged whether the callers are tasks beside it or machines
somewhere else.
