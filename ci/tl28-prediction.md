# tl28 — the prediction, before the first change (ruling #92)

Written before any tl28 change to this repository is committed, against
trunk `e9862f6`.

**Before** (the ten completed push/pull_request runs of `book` up to
39178370550): ten jobs a run, the rig's tests and the samples on ubuntu,
macOS and windows, the render, the contents gate in three engines;
**63.3 job-minutes** a run (median 64.4) and **18.6 minutes of wall**
(median; max 103.8, queueing). The contents gate is 34 to 39 of the
job-minutes (firefox 12 to 17, webkit 11 to 14, chromium 7 to 8); the
three-host samples 16 to 25, the tests 3 to 5, the render 1.

**After, predicted:**

1. A chapter's (or the rig's, or CI's) pull-request push is `tier pr`: the
   plan, `test (ubuntu-latest)`, `samples (ubuntu-latest)`, the render, the
   contents gate in three engines, the verdict: **43 to 50 job-minutes,
   about 18 minutes of wall** (the gate is the pole), 8 jobs where there
   were 10, macOS and windows skipped.
2. A samples, exercises, traces or audit-ledger push is `tier pr` with the
   tests and samples on ubuntu alone: **about 8 job-minutes, under 10 of
   wall**. A contents-gate push is the render and the gate: about 35.
3. A docs/-only push (the lanes' audits and evidence: 696 of 1,366 tracked
   paths) is `tier noop`: the plan and the verdict.
4. The `full-matrix` label at the head is `tier full`: every job on every
   leg, **about 63 job-minutes and 19 of wall**, as before.
5. A dispatch with `-f tier=pr` of a branch that adds a path no rule names
   comes out `tier full`.
6. The planted break (an unmapped path classified pr in `ci/tiers.py`) reds
   the plan job at "the planner can say no", skips every job, and the
   verdict job, `tier unplanned`, is red: **1 to 2 job-minutes**.
