# EXERCISES-PENDING.md — the pending manifest

The ledger model, applied to homework: every exercise below has its
full stem and its solution prose in its chapter file, and — wherever
the solution is a program — a directive-headed `.lu` on disk stating
the expected outcome. None of them is green today, and none is claimed
to be. The day a blocking feature lands, its exercises join the CI run
their headers already describe; until then this manifest is the honest
list. No aspirational green.

Note for the compiler track: the whole corpus — 184 directive-headed
`.lu` files under `principles/exercises/`, runnable and pending alike —
is a seed farm for the toolchain's corpus and fuzz harnesses. The
pending files are the most valuable seeds: each one encodes an expected
verdict for a feature that does not exist yet.

## Entries

| exercise | expected (directive) | blocker | owner |
|---|---|---|---|
| 7-5 (static half) | `fail(E1003)` | borrow-escape checking reaches no static verdict; wolfc leaves `channel` unresolved | s33-channels-select, then s18-tier0-exclusivity |
| 8-7 | `run(exit=0, stdout="c a")` | field writes through a pool index (`pool[h].next = k`) do not denote a place in the interp std subset | s37-core-types (std surface pinning) |
| 13-1 | `run(exit=0, stdout="285")` | `par` absent from the interp std subset | s32-tasks-scheduler / s37-core-types |
| 13-6 | `run(exit=0)` | `par` absent from the interp std subset | s32-tasks-scheduler / s37-core-types |
| 5-8 | `run(exit=0, stdout="marmot 5")` | `sorted_by` / `take` absent from the interp std subset | s37-core-types (std surface pinning) |
| 17-6 | `run(exit=0)` | `--chaos` fault injection at declared effect points — and with it §17.3 of chapter 17, which is why this stem is written and **not printed** (TOC.md §Deltas, bs07) | s36-deterministic-scheduler |
| 18-3 | `run(exit=0, stdout="285")` | positive comptime evaluation (CTFE engine) | s16-ctfe |
| 18-5 | `run(exit=0)` | comptime reflection (`typeinfo`) | s16-ctfe |
| 18-11 | `run(exit=0, stdout="A-B--A--A-B")` | positive comptime evaluation (CTFE engine) | s16-ctfe |
| 19-1 | `run(exit=0, stdout="3")` | perf-contract verification (I15) | s24–s26 WIR fact sprints |
| 20-6 | none — workflow exercise | `wolf bench` harness and `--baseline` workflow | s01-test-and-bench-infrastructure / s44-perf-validation |
| 21-8 | none — measurement exercise | bench rigs and CI perf gates | s44-perf-validation |
| 22-7 | none — sketch exercise | comptime registries (positive CTFE) | s16-ctfe |
| 23-7 | none — workflow exercise | script-mode dependency resolution | s51-package-manager with s31-driver-v0 |
| 24-6 | none — workflow exercise | capability manifests and `wolf audit` | s51-package-manager |
| B-10 | none — the absence is the exercise | `race` needs the dynamic race machine observing a real conflict; `alloc-contract` needs the wolf_rt quarantine allocator | s36-deterministic-scheduler; s23-memory-conformance / wolf_rt |

## What today's tools say

Recorded at authoring time (lupin 0.1.0, wolfc 0.0.1 debug), so the
flip is detectable: 5-8, 8-7, 13-1, 13-6, 18-3, 18-5, 18-11 exit
4 `unsupported` with the reason on stderr. 17-6: the flag is rejected. 19-1: `#[noalloc]` parses
and is not verified. 7-5: lupin reaches its dynamic verdict; wolfc
reports `unsupported` at resolve. The workflow entries (20-6, 21-8,
22-7, 23-7, 24-6) have no tool to run at all. Exercises whose stems
show a worked-example transcript (the ch20 bench-format set) label it
as a worked example in the stem itself; nothing in the corpus presents
invented output as a run.

## Retired entries

A row leaves this table only when its feature lands and the runner
reports the FLIP. The removals so far, kept here so the ledger reads as
a history rather than a snapshot:

- **13-3** (`fail(E1101)`) and **16-9** (`fail(E1102)`) — retired at the
  bs10 pin bump. Unsynchronized mutable capture across tasks and
  unsendable channel payloads are both static rejections in the compiler
  at this pin, with the diagnostics the two stems predicted: E1101 names
  all three fixes (channel, `Mutex` in a `when`, `par` with a
  reduction), and E1102 names all four admitted payload classes. Both
  exercises are now ordinary green samples with reviewed snapshots, and
  their solution transcripts in `principles/exercises/ch13/` and
  `principles/exercises/ch16/` show the real rejections instead of the
  honest-today runs they used to carry.
