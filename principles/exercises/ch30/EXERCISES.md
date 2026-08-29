# Chapter 30 — `pargrep`

Seven exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`. This chapter's programs are the compiler's — the
project reads files, and the reference interpreter has no filesystem by
design — so the solution programs carry `wolf-run(…)` directives and the
transcripts below are `wolf build` and a binary.

Two of the seven carry no program on disk, and both for stated reasons:
30-2's modified program is *flaky by construction*, which is the whole
point of the exercise and also a thing this repository must not ship as a
CI sample; 30-6 and 30-7 are reading and design.

## The chapter batch

**Exercise 30-1** *(fingers · wolf)*. Build both programs and run them
against the same two files with three patterns of your own. Then run
`wc -l` on both and write down, in one sentence, what you would tell a
colleague who proposed the parallel one for a log directory of four files.

Solution. No new program; the two are `samples/projects/seqgrep/seqgrep.lu`
and `samples/projects/pargrep/pargrep.lu`, both printed in the chapter.

```text
$ wc -l samples/projects/seqgrep/seqgrep.lu samples/projects/pargrep/pargrep.lu
  57 samples/projects/seqgrep/seqgrep.lu
  97 samples/projects/pargrep/pargrep.lu
```

The sentence: for four files, ship the sequential one — the parallel
version costs forty lines and thirty-seven of them are the fan-out being
written out by hand, so the version worth arguing about is the one that
takes its shard count from the input, and that is not the version on the
page.

**Exercise 30-2** *(comprehension · wolf)*. `hits` and `done` are both
`channel[int](0)`. Give `hits` a buffer (`channel[int](64)`) and predict
what happens before you run it. Then run the binary twenty times and count
the report lines each time. Two questions: what can `done.send(1)` do now
that it could not do before, and which line of the collector is the one
that loses the hits?

Solution. One character of the program changes and it stops working.
Twenty runs of the modified binary, counting report lines:

```text
$ for i in $(seq 1 20); do ./pargrep | wc -l; done | sort -n | uniq -c
     11 1
      2 2
      7 4
```

Four is the correct answer and it came up seven times.

What `done.send(1)` can do now: finish first. With `hits` unbuffered, a
task's send blocks until the collector has taken the value, so everything
a task found is already in `found` by the time its completion is sent.
Buffered, the send returns immediately, and a task can queue four hits and
then report itself done while all four are still in the channel.

The line that loses them is `while live > 0`. The collector stops the
moment the fourth completion arrives, and anything still sitting in the
buffer is never received. The report then walks the input and finds `found`
missing most of it.

This program is not in this directory. A sample whose output changes
between runs cannot be a CI sample, and a book that shipped one would be
teaching its own alarm to cry wolf. The measurement above is what the
exercise is for; the program is three seconds of your own editing.

**Exercise 30-3** *(comprehension · wolf)*. Delete `freeze` from the
`needles` binding, leaving `let needles = pattern.split("|")`. Predict
whether the program still compiles before you try it. Then explain, in two
sentences, what the four tasks are allowed to do with `needles` in each
version.

Solution. `ex30-3.lu`. It compiles, and it runs correctly:

```console
$ wolf build ex30-3.lu && ./ex30-3
a.log:1: 06:12 the wolf runs
a.log:3: 07:02 the wolf howls
b.log:2: 08:52 the wolf sleeps
b.log:4: 09:44 the wolf wakes
```

The prediction most readers write down is "it will not compile", and the
useful part of this exercise is being wrong about it. `needles` is bound
with `let` and never written, so the four tasks read a value nobody
mutates and the program is correct — by inspection, and only by
inspection.

The two sentences. With `freeze`, the tasks read an `imm` graph: nothing
anywhere can write it, the compiler knows that, and adding a write
somewhere else in the program is a compile error rather than a race
(§30.4's E1012 is that error). Without `freeze`, the tasks read ordinary
data that happens not to be written today, and the guarantee is a property
of the current text rather than of the type — which is exactly the
distinction the whole of Part 3 is about.

**Exercise 30-4** *(extension · wolf)*. Instrument the collector: print
each index as it arrives, before pushing it. Run the binary twenty times
and count the distinct arrival orders you see; then confirm that the four
report lines never move. Which of the two outputs would you put in a test?

Solution. `ex30-4.lu`, one `print` added to the `hits` arm.

```console
$ wolf build ex30-4.lu && ./ex30-4
arrive 0
arrive 2
arrive 5
arrive 7
a.log:1: 06:12 the wolf runs
a.log:3: 07:02 the wolf howls
b.log:2: 08:52 the wolf sleeps
b.log:4: 09:44 the wolf wakes
```

That is one run. Over twenty, the four `arrive` lines came out in five
different orders — `0 2 5 7` most often, then `0 2 7 5`, `0 5 2 7`,
`0 5 7 2`, `0 7 2 5` — and the four report lines hashed to one value
twenty times out of twenty. Which one goes in a test is therefore not a
matter of taste: the report is a property of the program, the arrival
order is a property of the afternoon.

Index `0` arrives first on nearly every run, which is worth not
over-reading. The first shard's first line matches, and its task has the
shortest path to a rendezvous. That is a bias, not a guarantee, and a test
that assumed it would fail on a loaded machine.

**Exercise 30-5** *(extension · wolf)*. Make the report order-dependent
on purpose: delete both channels and the collector, and have each task
print its own matches directly. Run the binary twenty times and hash
the output. You will get more damage than you predicted. Say what the
extra damage is, and then say what you have broken in terms of §30.5's rule
rather than in terms of tasks.

Solution. `ex30-5.lu`. Twenty runs, twenty distinct outputs. Here is one:

```text
$ ./ex30-5
a.loga.log:1: 06:12 the wolf runs
:3: 07:02 the wolf howls
b.log:4: b.log:2: 08:52 the wolf sleeps
09:44 the wolf wakes
```

The extra damage is that the lines are not merely out of order, they are
*torn*: `a.log` from one task and `:3: 07:02 …` from another arrived in
the same line of output. `print` is not atomic across tasks, and four
tasks writing one stream interleave inside a line as readily as between
two.

In §30.5's terms: the four report lines used to be a fact about the
program, and now they are a fact about the run. Nothing was added to the
program to break this — something was removed. The collector was not
overhead; it was the single owner of the output, and a single owner is
what made the output reproducible. The general form of the rule is that
the last stage of a concurrent pipeline should be sequential, and the
cheapest way to obey it is to let exactly one task print.

**Exercise 30-6** *(spelunking · wolf)*. Read the E1012 note in §30.4 in
full. It offers two ways out: build the value completely before freezing,
or keep a mutable `copy` alongside. Say which one `pargrep` uses and what
the other one would cost in a four-task program.

Solution. `pargrep` uses the first, twice, and the shape is visible in
both `freeze region { … }` blocks: everything the value will ever contain
is pushed inside the block, and the block's last expression is the finished
value. The pattern list is built by `split` in one call; the table is
accumulated by a loop over the files and then handed out as a `Table`.
Neither is touched again.

The second way out — keeping a mutable `copy` beside the frozen one — is a
correct answer to a different question, and in a four-task program it is
usually the wrong one. The copy is not shareable, so it cannot cross into
a task, so the only place it can be used is the parent; and the moment
what the parent has and what the tasks have can differ, the program has a
consistency question it did not have before. The cost is not the memory.
It is that "the frozen table" stops being a single noun.

**Exercise 30-7** *(design)*. The shard count is a constant. Sketch the
version that takes it from the input: what the ranges become, what the
collector's `live` counter becomes, and what `pargrep` would need from the
language to spell the fan-out in one loop instead of four spawns. Then say
whether four shards on two files was ever the right number.

Solution. The arithmetic is the easy third of it. With `w` shards, the
`k`th range is `(n * k) / w` to `(n * (k + 1)) / w`, which is the same
integer division §30.1 already does and needs no boundary variables at
all; `live` starts at `w` instead of `4`; and the report loop does not
change, because it never knew how many tasks there were. The whole
difference is that three `let b`s become one expression evaluated inside a
loop.

What the language has to give is the loop: `for k in 0..w { s.spawn(fn() {
… }) }`, with each closure capturing its own `k`. That is one construct,
and everything else in the sketch is already spelled.

Whether four was ever right: no, and the honest reason is that nobody
chose it for this input. Four is a plausible default for a machine and a
poor one for two files of four lines, where a single task would finish
before a second one started. The number a real version wants is the
smaller of the worker count and the input's shard count, with a floor of
one — and the interesting part is that the program cannot ask for the
first of those two either. A tool that shards should take `-j` from the
command line, the way `make` does, which makes this exercise's answer one
more argument for the loop.

## Ledger

- 30-2 and 30-6 and 30-7 carry no `.lu`: the first because the program is
  deliberately flaky (stated in its solution), the other two because they
  are reading and design.
- 30-4's and 30-5's transcripts are single runs of nondeterministic
  programs and are labelled as such on the page. Their directives assert
  the exit status only, which is the part that does not vary.
