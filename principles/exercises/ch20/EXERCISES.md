# Chapter 20 — Reading `wolf bench diff`: exercises

The bench harness is s01's deliverable and does not exist yet; this
chapter teaches the *reading* of its output, which the sprint contract
pins. Every transcript in this set is therefore a **worked example in
the pinned format, not tool output** — each stem says so, and exercise
20-4 records what the real binary says today. When the harness lands,
CI regenerates these examples from real runs; until then no console
block here claims to be one.

## §20.1 — The format

**Exercise 20-1** *(comprehension · prose)* — The worked example below
is hand-written in the s01 format (it is not a tool run). Three
kernels, before and after your change:

```text
name            old ns/op   new ns/op   delta     ±MAD     allocs/op
tokenize        1204        1188        -1.3%     2.1%     0 → 0
parse_row       892         641         -28.1%    1.8%     3 → 1
render          15040       15490       +3.0%     4.9%     2 → 2
```

One row is a real improvement, one is a real question, one is nothing.
Sort them, and say what you would do about the question before
believing anything.

Solution: `parse_row` is the improvement — a 28% drop dwarfs its 1.8%
noise band, and the alloc count fell with it, which names the likely
mechanism. `tokenize` is nothing: −1.3% inside a 2.1% band is a coin
flip reported politely. `render` is the question: +3.0% against 4.9%
MAD is *probably* noise, but it is a regression-shaped maybe on a hot
kernel — the move is more iterations (tighter MAD), not a verdict.
The discipline the format teaches: delta means nothing except against
its own noise band.

**Exercise 20-2** *(comprehension · prose)* — In the same worked
format, a change produces `allocs/op 3 → 0` and `delta -0.4% ± 1.9%`
on the same kernel. A teammate calls the patch pointless. Name two
distinct reasons the allocs column can matter when ns/op does not
move.

Solution: first, microbenchmarks feel steady-state — an allocator hit
in a 900-ns kernel may cost little *there* while costing real money
under memory pressure, fragmentation, or in a region about to be
frozen and shared; the alloc count is a structural fact, the ns/op a
situational one. Second, zero is a contract boundary: at `0 → 0
allocs` the function becomes eligible for `#[noalloc]` (chapter 19),
and eligibility is worth having before the hot path needs it. The
column exists because "did not get faster" and "did not get simpler"
are different findings.

**Exercise 20-3** *(comprehension · prose)* — Five samples of one
benchmark, in ns: 1000, 1010, 990, 1005, 2400. Compute mean and
median; state which the s01 format reports and what the outlier most
plausibly was. Then say what MAD does with the same five numbers that
standard deviation would not.

Solution: mean 1281, median 1005. The format reports the median, and
the 2400 was most plausibly the measurement's environment — a
scheduler preemption, a cache gone cold, a thermal step — not the
kernel. MAD (median absolute deviation) takes the deviations from the
median — 5, 5, 15, 0, 1395 — and takes *their* median: 5. One wild
sample barely moves it, while standard deviation would be dominated
by that sample's square. Robust statistics are not sophistication;
they are the choice to measure the program instead of the machine's
weather.

## §20.2 — The variance gate

**Exercise 20-4** *(spelunking · wolf)* — Ask the binary for a
benchmark and read what comes back. Which half of the toolchain owns
the gap, and where else in this book have you seen this exact
reporting posture?

Solution — a real run, the only one in this chapter:

```console
$ wolf bench
wolf bench: not yet (grows at its own campaign; D34's single binary)
```

The binary declines rather than guessing — the same posture as the
pending exercises in chapters 18 and 19, and the same one this
chapter's worked examples adopt: state the expected shape, mark what
does not exist, never render green early. D34's single binary is why
the refusal is `wolf`'s to make: there is no second executable that
could have answered instead.

*(Filed for the chapter sprint that publishes this material: a
scaffold refusal is not product output, so this exercise's subject
does not survive TONE.md's tense discipline. It waits for `wolf
bench`.)*

**Exercise 20-5** *(comprehension · prose)* — The variance gate fails
a CI run when the delta is within the noise band, in either direction.
A teammate objects: "a 3% win is a 3% win; gate only the losses."
Using worked example 20-1's `render` row (+3.0% ± 4.9%), construct the
failure scenario the symmetric gate prevents.

Solution: run the same commit ten times and `render`'s ±4.9% band
produces "wins" and "losses" of ±3% with no code change at all. Gate
only the losses and the ratchet is one-directional noise: a lucky run
lands a fake win in the baseline, the next honest run reads as a
regression against it, and the team begins optimizing the random
number generator. The symmetric gate encodes one sentence: a delta
inside the band is not a measurement, whichever direction it flatters.

## §20.3 — Your own baseline

**Exercise 20-6** *(extension · pending — blocker: bench harness and
baseline workflow; owner: s01-test-and-bench-infrastructure /
s44-perf-validation)* — When the harness lands: record a baseline in
your own repo with `--baseline`, make a change you believe is neutral,
and run the diff. The exercise's deliverable is your surprise,
whichever row it comes from.

Solution (prose, pending execution): the workflow is capture, change,
compare — the point of doing it on *your* machine is discovering your
noise floor before trusting any delta from it. A laptop on battery
with a browser open can carry a 10% band; the worked examples' 2%
bands are what a pinned-governor CI box buys. Until the tool exists,
the habit to build is 20-1's: never read a delta without its band.

**Exercise 20-7** *(comprehension · prose)* — The sprint plan gates CI
on two co-equal tracks: compile-time and runtime (D5). Your change
makes `parse_row` 12% faster at runtime and makes the compiler spend
9% longer on the crate. In the s01 format, which gates fire, and what
does "co-equal" commit the project to when they disagree?

Solution: both fire — the runtime gate green, the compile-time gate
red, assuming both deltas clear their bands. Co-equal means the red
one is not advisory: a 9% compile-time regression blocks exactly as a
9% runtime regression would, and the disagreement goes to a human
holding the project's stated bet — wolf sells fast builds as a
feature, so compile time is a benchmark, not a byproduct. The exercise
of holding both numbers at once is the chapter's habit: performance is
plural.

## Chapter batch

**Exercise 20-8** *(design)* — Every row in the s01 format carries
metadata: toolchain commit, target triple, CPU governor, run date.
Argue which single field the *book* most depends on, given TONE.md's
rule that performance claims are made by "a program and a measurement"
— and what a printed benchmark without that field would quietly
become.

Solution (discussion): the date, with the commit a close second. A
measurement is a fact about a moment: chapter 21 prints wins and
losses that some future compiler release will invalidate in either
direction, and the date is what lets a reader in that future read the
table as history instead of as a current claim — the difference
between "measured then" and "true now" is the difference between
evidence and folklore. The commit pins *what* was measured, the date
pins *when the claim expires*; a benchmark without them is marketing
with a monospace font, which is the exact register this book forbids
itself. It is also why chapter 21's loss tables regenerate from CI
each release rather than living in the manuscript.
