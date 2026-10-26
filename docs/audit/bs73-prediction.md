# bs73 — the 0.2.35 fallout

## Prediction

Control: wolf-book 591cd5d07 (the full revision is recorded with the first
measurement). Before any exercise or chapter change, reproduce every exercise
with wolf 0.2.35 / lupin 0.1.56 and the bundled std 2386522e on hasu block B.
The issue predicts eleven compiler rejections and seven native refusals. The
wl70 census instead separates fail 18, refused 10, error 1 and timeout 4; the
first measurement will identify the individual programs and distinguish expected
refusals and pending rows. That measured table, not the issue's shorthand, is the
record. No tier graduation is predicted merely from a failed corpus build.

R-bs73-1 recommendation: retain each exercise's teaching idiom and repair only
invalid source; file valid-source compiler defects and use the existing pending
manifest. Implement this recommendation, without deciding a ruling. Expect
s237 shifts, explicit lending, and moved std spellings to explain source fixes;
measure before assigning causes. Keep existing one-machine rows unless both
machines demonstrate a graduation. Regenerate every affected transcript from the
binary, with zero unrelated flips. Regenerate the index and report exact tier
movements after measurement.

## Forbidden

No hand-written console output, Mac builds, attribution trailers, PR labels,
merge, tag, deploy, issue closure, planning-repo edits or ~/.claude edits. Compiler
defects are filed, not disguised by prose. Use only this lane's directories;
rootless container, block B flock, CPUs 6–11, 12 GiB. Never wait for CI: read each
push's run id once and hand the final watch to the orchestrator.

## Inputs verified

Read bs73 and bs72 contracts, bs72's docs/audit/bs72-prediction.md and rig
scripts, wolf-book issues #94/#91/#89 whole, wolf-lang's 0.2.35 CHANGELOG,
and RULES.md sections 1–10 plus hasu-env README. The named wl70 note does not
exist on wollf trunk or its wl70 branch; the orchestrator corrected the input to
train/data/repos/tables/census-wl70.json and asked for a fresh measurement.
Neither named lane checkout was pre-staged; create both at origin/trunk.

## Evidence index

- Initial inventory: `bs73-evidence/measurement-summary.txt`, `census.json`,
  `wl70-comparison.json`; exact commands in `ctr.sh` and `census.py`.
  Control 591cd5d0779764b7bf043ebe712ec52e1e356aa5.
  Baseline: `baseline-samples.log`, 537 samples, 534 pass, 3 existing pending,
  0 failures, 0 flips; 506/526 console blocks replayed.
- All 33 wl70 anomalies: `bs73-evidence/exercise-table.tsv`. The eighteen
  `fail` rows split into eleven native rejections and seven UB-oracle
  mismatches; ten additional native refusals are explicit one-machine or
  existing pending samples. No exercise repair is owed by the book rig.
  The orchestrator explicitly confirmed that intentional trap demonstrations
  must remain unchanged. wollf#43 already owns `run(exit=nonzero)` parsing.
- #89 is already satisfied by bs72 commit 3e51c7e: the Mode literal names
  all ten fields and passes untouched. Verify again at the final head.
- Final samples, tiers, generated index, anchor/catalogue counts: pending.
- CI plant, final run id and lane-audit: pending.

## Done-when

- [x] Initial census committed before exercise changes.
- [x] No exercise edits: all eleven rejections and ten refusals are declared
  one-machine/pending samples; the table quotes their native diagnostics.
- [ ] Five std wrappers and term.Mode fields built at the pin.
- [ ] Samples and tiers checked; generated index current; no unrelated flips.
- [ ] README read; counts and anchor/catalogue key sets verified.
- [ ] PR open, Footprint: cross-cutting, final head pushed; run id read once.
- [ ] Audit last line and final evidence handed back without waiting on CI.
