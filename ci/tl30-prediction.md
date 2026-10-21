# tl30 — the macOS cap: the prediction, before the first change

Written before any tl30 change to this repository is committed, against
trunk `8ae02b9`. Contract: `sprints/tooling/tl30-footprints-and-the-macos-cap.md`
(planning repo `wolffe-lang/wolf`, trunk `809e40f`), under `sprints/RULES.md`.
The org-wide census (every repository's macOS jobs, the last 60 runs each
ran one, every red macOS job against its Linux counterpart) is wolf-lang's
`ci/tl30.md`; this is the book's slice.

**Why.** Since the Team plan (2026-10-20) the Linux pool is 60 slots and the
macOS pool is still 5: tl29 measured all five macOS slots busy 55 % of the
time after the switch, a median macOS wait of 17.6 minutes (a half-minute
job waited 44.6). The book's merge gate (`tier full`: the `full-matrix`
label, and again on the trunk push) holds two macOS slots: `samples
(macos-latest)`, 6.5 to 10 minutes, and `test (macos-latest)`, about one.

**What the macOS jobs caught.** The last 60 runs with a macOS job
(36953773273 .. 39433843056, 2026-10-02 to 10-20): 9 red macOS jobs, and
in every one the same job on ubuntu was red in the same attempt (the
planted breaks and the shared reds of bs63 to bs69). **No macOS job caught a
red that no Linux job caught.** So the merge keeps one macOS smoke and the
rest goes to the nightly.

**The smoke** is `test (macos-latest)`: the rig built on macOS, its fmt,
clippy and unit tests (one minute). `samples (macos-latest)` (every sample
built with the pinned wolf and run on macOS against its directives) leaves
the merge for the nightly, and still runs on a toolchain release.

## The shape

| tier | when | test | samples |
|---|---|---|---|
| `all` (new) | the nightly; a toolchain release's `repository_dispatch`; `workflow_dispatch -f tier=all` | ubuntu, macOS, windows | ubuntu, macOS, windows |
| `full` | the `full-matrix` label (the merge gate); a trunk push; a dispatch (the default); a path no rule names | ubuntu, macOS, windows | ubuntu, windows |
| `pr` | any other pull-request run; `-f tier=pr` | ubuntu | ubuntu |
| `noop` | prose alone; another label | — | — |

`render` and `contents` (ubuntu) are in `all` and `full` as before. The
nightly is `tier all`, every job on every host as `tier full` was before
tl30, and its red still opens the `nightly-red` issue (`ci/nightly-red.sh`,
unchanged). A toolchain release re-verifies the book on every host as
before (the web edition redeploys only after it). `lane-audit.sh` accepts a
green `tier all` as the superset of `tier full` (tl29); the merge rule is
unchanged.

## Prediction

Before (the labelled and trunk-push `tier full` runs 39350839571 and
39359674421, and the 55 untiered push and pull-request runs before tl28,
which ran the same macOS jobs): **8 to 10 macOS-minutes a run** (samples
6.5 to 8.6, test 1.0 to 1.1), two macOS slots, about 63 job-minutes.

1. `tier full` (the label and every trunk push): **about 1 macOS-minute and
   one macOS slot a run** (falsified above 3), about 55 job-minutes
   (falsified above 65). About **−7 to −9 macOS-minutes and one macOS slot
   a run, −15 and two slots a merge** (the label's run and the trunk
   push's).
2. `tier all` (the nightly, a toolchain release, `-f tier=all`): unchanged,
   every job on every host.
3. `tier pr`: unchanged (ubuntu alone).
4. The planted break (`full_leg` answering every leg, the merge's shape
   before tl30) reds `ci/tiers-test.py` in the plan job of a dispatch
   `-f tier=pr` on its own branch; nothing else runs; under 2 job-minutes.
5. This pull request touches `ci/` and `.github/`, so its pull-request run
   is `tier pr`; the labelled run at the final head is the first `tier
   full` of the new shape and is the measurement (the orchestrator watches
   the run, RULES §6).
