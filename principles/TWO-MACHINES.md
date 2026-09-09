# TWO-MACHINES.md — what a `run(…)` fence claims

wolf has two implementations and the book is true for both. This
document fixes what a runnable block in this repository asserts, which
machine has to meet it, and how a block only one machine serves is
written so that a reader is never handed a program their tool refuses.

Ratified at bs31, off wolf-book#9. Voice rules live in `TONE.md`;
fence mechanics live in `STYLE.md`; this file owns the claim.

## 1. The rule

**A `run(…)` fence is a claim about both machines, and a machine that
refuses has not met it.**

`cargo xtask samples` executes every `run(…)` sample under `lupin` and
under `wolf run`. Both must reach the declared exit; where the fence
declares `stdout=`, both must print those bytes. A block on which the
two machines disagree is not a block with a caveat, it is a block that
is wrong on one of them, and the runner names which.

## 2. What this replaces, and the reader it cost

Until bs31 the `run(…)` arm ran `lupin` and nothing else. The compiler
was never asked. Every other directive in the language had already been
built the honest way — `fail(…)` is the compiler's verdict, `ub(…)`
requires lupin's oracle *and* the checked build, `wolf-run(…)` was minted
at bs09 for the programs only the compiler executes — and `run(…)`, the
most common directive in the book, was the one that asked a single
machine and printed the answer as if both had spoken.

It cost a reader chapter 1. `str.to_int` has never been in the
compiler's builtin set. The chapter opened on it and exercise 1-10 was
built on it; `lupin` served both; `wolf run` answered `cannot compile
this yet` at every pin the chapter has ever had; the chapter's own audit
ledger recorded the refusal from bs12 onward; and CI was green the whole
time, because nothing in CI had ever run those two programs under the
compiler. The maintainer copied the exercise, ran the tool the chapter
teaches first, and found the book's claim false on their machine.

The failure was not that the refusal was scored as a pass. It is that
the refusal was never scored at all.

## 3. The spellings

| directive | machines | means |
| --- | --- | --- |
| `run(exit=N[, stdout="…"])` | both | both execute it and agree |
| `run(exit=trap(k))` | both | both fault, both name kind `k` |
| `wolf-run(…)` | compiler | the interpreter declines it, by design |
| `lupin-run(…)` | interpreter | the compiler declines it |
| `fail(E…)` | compiler | rejected statically, with that code |
| `ub(row)` | both | lupin's oracle faults, `--checked` names the row |
| `audit(E…)` | compiler | `wolf audit-surface` rejects the surface |
| bare ` ```wolf ` | compiler | gets through the static phases clean |

`wolf-run(…)` and `lupin-run(…)` are the same directive pointed in
opposite directions, and neither is a way to quiet the runner. Each one
says *this program runs on one machine*, in the fence, on the page, and
in the chapter's ledger, all three.

## 4. What a one-machine block owes the reader

Three things, and a block that skips any of them is not finished.

- **The fence names the machine.** `lupin-run(…)` or `wolf-run(…)`, so
  the runner scores what is true and the rendered label on the page says
  `interpreted run` or `compiled run` beside the block.
- **The prose carries the per-machine note.** One present-tense clause
  naming the machine that runs the program: *this program runs under
  `lupin`*. `TONE.md`'s tense discipline forbids the alternative — "not
  yet", "when X lands", "at this pin" are deferral prose and stay out of
  reader-facing text. The note names which machine runs the sample,
  never when the other will.
- **The chapter ledger carries the row, with an owner.** A one-machine
  block is a debt of the toolchain, and the ledger is where debts live
  at full volume. The row names the construct, the machine that refuses
  it, the words it refuses in, and the campaign that owns the fix.
  `cargo xtask ledger --check` holds every open row to an issue or a
  waiver.

## 5. Traps: the kind is the contract, the exit is not

`run(exit=trap(k))` requires both machines to fault and both to name
`k`. It does not require one exit status, because D60 rules the status
per-machine and Appendix B prints the table: `lupin` exits 3, a compiled
binary dies at 134. The runner knows both numbers.

A trap block that holds on one machine and not the other is reported as
a failure rather than absorbed, and then it has to be read, because two
different things wear that shape. Some are divergences, which are the
most interesting failures this rig can find. Some are differences the
specification permits, and `deadlock` is the worked example:
`[conc.deadlock.trap]` requires detection in deterministic test modes
and *permits* it elsewhere, so `lupin` traps a deadlocked program and a
compiled binary is entitled to park on it forever. Those samples are
`lupin-run(exit=trap(deadlock))`, and their ledger rows cite the clause
instead of naming an owner, because nothing is owed. A permitted
difference and a debt are spelled the same way on the fence and
differently in the ledger.

One consequence for the runner, learned the hard way on the first
two-machine run: a program the other machine parks on must not cost the
full sample budget on every run forever, and a compiled program that
hangs is a grandchild of the runner rather than its child. Graduation
probes get a short budget, and a timed-out sample has its whole process
group signalled.

## 6. Retirement is a FLIP, not a memory

A `lupin-run(…)` sample the compiler starts serving — and a
`wolf-run(…)` sample the interpreter starts serving — is reported as a
FLIP: the sample passes and its spelling is stale. A flip is a hard
error, so the pin bump that grows the feature is the commit that
graduates the fence to `run(…)`, retires the page's per-machine note,
and closes the ledger row. Nobody has to remember; the runner
remembers.

This is the same machinery `samples-pending.toml` has always used, for
the same reason: a feature landing is noticed, never silently absorbed.

## 7. What this is not

Three manifests answer three different questions and none of them
substitutes for another.

- `samples-pending.toml` — **not yet, anywhere.** The directive is
  expected to fail on both machines; the sample runs report-only.
- `samples-os.toml` — **not here.** The sample is refused on one *host*,
  by name, in exactly those words.
- `lupin-run(…)` / `wolf-run(…)` — **not on that machine.** The sample
  runs, green, on the one machine that serves it, and the page says
  which.

All three refuse the fourth answer, which is a skip. The book does not
print a program it has not executed, and it does not execute a program
without saying which machine executed it.
