# Chapter 34 — Signals and the process

Seven exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`. Two carry a program and five do not: 34-2, 34-3
and 34-4 are comprehension, 34-5 is reading and 34-6 is design.

Both programs import `std.signal`, which the interpreter unpacked from
its own archive does not have, so both are compiler runs. Each signals
only itself.

## §34.1 — A signal is an event

**Exercise 34-1** *(extension · wolf)*. Listen for `terminate()` and
`quit()` together, raise `quit()`, and print which of the two the wait
answered. Then poll the same set and print what the poll answers. Predict
the second line before you run it.

Solution. `ex34-1.lu`.

```console
$ wolf run ex34-1.lu
arrived: quit
still queued: 0
```

The poll answers 0. One signal was raised and one delivery was queued,
and the wait took it off the queue when it answered. A wait and a poll
read the same queue, and neither looks at it without taking what it
finds, so a program that wants to know which signal came keeps the
wait's answer rather than asking twice.

## §34.2 — Sending, numbering, and what was ignored

**Exercise 34-2** *(comprehension · prose)*. `signal.send(pid, 0)` sends
nothing. Name two questions a program answers with it, and say why its
rows are decided in the order they are: the shape first, then the host,
then the target.

Solution (discussion): the two questions are *is there a process with
this id* (`not_found` says no) and *may I signal it* (`denied` says no,
`ok` says yes to both). A program asks them before it trusts a pid file
(a server that finds its own pid file asks whether the process it names
is still running before refusing to start), and a supervisor asks them
about a process it did not start. The order puts the caller's own
mistake first: a `pid` that is not above 0, or two meanings at once, is
wrong on every host, so it is `invalid` everywhere and nothing is sent.
The host comes second because a host with no way to signal a process
cannot ask the target anything. The target comes last because only the
real call can answer for it, and its answer is about one moment: a
process that existed when the send was made may be gone by the next
line.

## §34.3 — Who this process is, and its terminal

**Exercise 34-3** *(comprehension · prose)*. A program builds a `Mode`
with `canonical`, `echo` and `signals` set and nothing else, sets it, and
exits. Afterwards its user finds that Ctrl-S no longer pauses a long
listing, and that Ctrl-W no longer erases a word at a `cat` prompt. Say
which two fields did it, and what the program should have done instead
of building the mode by hand.

Solution (discussion): `flow` and `extended`. A mode built by hand says
what it wants of every switch, and leaving those two out turns flow
control (Ctrl-S and Ctrl-Q) and the host's extended input processing
(word erase among it) off. The program should have read the mode the
terminal had with `term.mode`, kept that value, changed only the fields
it meant to change (or passed the read mode to `term.raw`), and set the
saved mode back before it exited. Nothing restores a terminal's mode for
a program that leaves.

## §34.4 — What a call charges, and where it is refused

**Exercise 34-4** *(comprehension · prose)*. A package's manifest
declares `env` and nothing else, and its own code calls `os_pid`,
`os_term_size` and `os_signal_send`. Which of the three fails the build,
and which word would the manifest need for it? Say why it is that word
and not `env`.

Solution (discussion): `os_signal_send` fails the build with E1504: it
charges `exec`. `os_pid` charges `env`, which the manifest declares, and
`os_term_size` charges nothing. Sending a signal to another process is
process control, the same authority as starting one or killing a child,
and that authority is `exec`. `env` is reading this process's own
context: its ids, what it ignores, its working directory. A package
that only reads who it is should not also be able to stop someone else.

## Exercises

**Exercise 34-5** *(spelunking · prose)*. Read `[os.signal.set.stable]`
in the specification. Say what it promises a program that stores a set
of meanings in a file and reads it back under a later release, and what
it does not promise a program that writes a bit by hand.

Solution (discussion): it promises that the set means the same thing
when it is read back. Every meaning a release has published keeps its
bit forever, a bit is never reassigned or reused even if its meaning is
retired, and a new meaning only takes a bit no release has used, so a
set written by one release names the same meanings under every later
one. It promises nothing about a bit written by hand except that the bit
is stable. Nothing checks that the number a person typed is the meaning
they meant, a slip of one bit is another meaning or the `io` row, and a
reader of the program has to look the number up. The clause says it
directly: portable code names a meaning, never a literal bit or a host's
signal number.

**Exercise 34-6** *(design)*. A long-running program is started by a
user who means it to survive the terminal closing. Write down the first
three things it does with signals, in order, and say why reading
`signal.ignored()` has to come before its own first `listen`.

Solution (discussion): first, read `signal.ignored()` and keep the
answer: it is the set the program was started ignoring, and a program
started under `nohup` finds `reload()` in it. Second, leave alone what
it was started ignoring (a shell's `trap` has the same rule), and
otherwise decide what a hang-up means to it: ignore `reload()` to
survive the terminal closing, or listen for it to re-read its
configuration. Third, listen for `terminate()` and `quit()` and shut
down cleanly when one arrives. The read has to come first because every
later call changes the answer: a listen ends an ignore, an ignore sets
one, and after either `signal.ignored()` reports the program's own
choice rather than how it was started.

**Exercise 34-7** *(extension · wolf)*. Put all twenty-eight names in a
list. For each, ask the host for its number and try a listen (then reset
what you listened for). Print how many meanings there are, how many the
host numbers, how many a listen takes, and what `signal.number` answers
for two meanings at once. Print counts, not numbers.

Solution. `ex34-7.lu`.

```console
$ wolf run ex34-7.lu
28 meanings, 28 numbered here
20 can be heard
two at once: invalid
```

Every meaning has a number on a Unix host, the eight send-only ones
included: a host numbers `SIGKILL` like any other signal. Twenty are
listenable, and a listen of any of the other eight is the shape rule's
`io`. Two meanings at once are `invalid` to `signal.number`, because a
number names one signal. The program never prints a number, and that is
the exercise: the counts are the same on Linux and macOS, and the
numbers are not.
