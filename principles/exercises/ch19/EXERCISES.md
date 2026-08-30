# Chapter 19 — Reading the release tier: exercises

Commands run from this directory; outputs are pasted from real runs.
This set landed with the bs18 re-draw of Part 4 (the old ch19 subject,
perf contracts, is chapter 20's; the contracts corpus moved with it).
19-3 and 19-5 are re-homed descendants of the retired bench-format
corpus (old 20-3 and 20-8), rewritten without the worked-example
format whose instrument does not exist: what survives is the
discipline, which needs no harness.

## §19.1 — One program, two binaries

**Exercise 19-1** *(fingers · wolf + lupin)*. Type the scanner and
change the line to `"the  pack   hunts at dusk"`, with the doubled and
tripled spaces. Predict the count on paper from the two-state rule,
then build it with `--release`, run it, and run it under `lupin`.

Solution. `ch19/ex19-1.lu`. Five: a word starts at each
space-to-nonspace edge, and stretching a gap adds no edges. The
scanner counts transitions, not spaces, which is why the doubled and
tripled runs change nothing:

```console
$ wolf build --release ex19-1.lu
$ ./ex19-1
5 words
$ lupin ex19-1.lu
5 words
```

Both tiers, one answer, which is §19.1's first claim exercised: the
flag may change everything about how the binary is made and nothing
about what it means.

## §19.2 — The compiler hands LLVM less

**Exercise 19-2** *(comprehension · prose)*. A release of the
compiler makes one benchmark kernel 11 percent faster, and the same
release moves that kernel's handed-to-LLVM instruction count from 266
to 309. Explain how both numbers can be progress, and name the
transformation shape from this section that produces exactly this
signature.

Solution: loop versioning. The middle end emitted a guarded fast copy
of the loop (its bounds checks proven and folded) beside the original
slow copy that keeps every check, plus the guard chain that picks
between them at the loop's door. Two loop bodies and a guard are more
instructions than one loop body; the hot path through them is
shorter. The ratio measures work delivered to the backend, not time,
and this kernel spent 43 instructions to buy 11 percent; the numbers
are the stencil kernel's own, from the loss ledger's entry for the
versioning landing. The general lesson: any gate on a proxy metric
needs a story for the cases where the proxy and the goal move apart,
which is why the ratio is a ratchet with a paper trail rather than an
objective function.

## §19.3 — Reading a loss

**Exercise 19-3** *(comprehension · prose)*. Five timings of one
benchmark, in nanoseconds: 1000, 1010, 990, 1005, 2400. Compute the
mean and the median; state which one a careful report trusts and what
the outlier most plausibly was. Then compute the median absolute
deviation and say what it does with these five numbers that a
standard deviation would not.

Solution: mean 1281, median 1005. The report trusts the median, and
the 2400 was most plausibly the machine, not the program: a scheduler
preemption, a cache gone cold, a thermal step. The median absolute
deviation takes each sample's distance from the median (5, 5, 15, 0,
1395) and takes the median of those: 5. One wild sample barely moves
it, where a standard deviation would be dominated by that sample's
square. Robust statistics are the choice to measure the program
instead of the machine's weather, which is also why the ledger's
nightly numbers are medians and why §19.3 says a delta means nothing
except against its own spread.

**Exercise 19-4** *(comprehension · prose)*. Classify each loss into
one of this section's four kinds, and name the fix path: (a) a kernel
adds numbers that arrive over a socket, and the additions stay
checked; (b) the compiler proves a parameter's buffer disjoint from
every other, and the loop still reloads it after each store; (c) a
kernel would tie C if the language allowed reading one byte past the
end of a buffer when the read provably lands in the same page.

Solution: (a) is "the fact does not exist," and its fix path is
honesty, not code: no sound analysis can bound values the program did
not compute, the checks are the promise, and if the price crosses the
gate's line the entry goes to the exceptions file with the cost
measured (chapter 21 prints exactly this kernel's entry). (b) is "the
fact went unspent": the proof exists and the instruction stream shows
it not arriving, so the fix is in the pipeline that carries facts to
the backend, with this kernel as the regression test. (c) is "the win
was renounced": the read past the end is undefined behavior wolf
refuses regardless of page arithmetic, so the gap is a price, written
down, never a bug filed on the checker for being right.

**Exercise 19-5** *(design)*. A colleague's slide says "our runtime
is 8 percent faster than C." Using this chapter's last paragraph,
list the three pieces of provenance the claim needs before it is a
measurement, and state what the sentence quietly becomes as each one
is removed.

Solution (discussion): it needs the suite (what programs, against
which C, compiled how), the date with its noise discipline (medians
over how many runs, on what machine, against what spread), and the
commit (what exactly was measured). Remove the commit and it is a
claim about nothing in particular: nobody can re-measure it, so it
cannot be wrong, which is worse than being wrong. Remove the date and
it is folklore: true once, quoted forever, immune to the compiler
release that invalidated it. Remove the suite and it is advertising:
"faster than C" with the workload chosen after the fact. The
chapter's standard (a dated, gated, commit-pinned ledger line) is
what remains when a sentence like the slide's is required to survive
cross-examination.
