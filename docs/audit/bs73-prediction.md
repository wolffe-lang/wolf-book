# bs73 — the 0.2.35 fallout

## Prediction

The prediction was committed before changes as c03de357a92a91403554fefa3651b54303a6ffc8.
The control is 591cd5d0779764b7bf043ebe712ec52e1e356aa5. The contract's
proposed eighteen repairs were not justified: the unmodified book rig passes
534 samples, with three existing pending, zero failures and zero flips.

The first measurement was committed as ae61d57 before chapter edits.
wl70's eighteen `fail` rows comprise eleven native exit-2 observations and
seven runs whose UB claim belongs to lupin's oracle. Its other ten native
refusals also repeat: nine explicit lupin-only programs and the already-pending
8-7. Four timeouts are intentional deterministic-deadlock demonstrations.
The remaining error is the verifier's inability to parse `run(exit=nonzero)`,
already recorded in wollf#43. The issue's “11 rejected and 7 refusing natively”
is not the measured classification.

R-bs73-1 recommendation, implemented for review: preserve deliberate trap
exercises and explicit one-machine directives. The orchestrator confirmed that
item 1 shrinks to the measurement when no book-rig regression is found.
No exercise changes, new pending rows or new wolf-lang issues are needed.
Every anomalous exercise, its directive, quoted native error and disposition
is in `bs73-evidence/exercise-table.tsv`.

R-bs73-2 recommendation, implemented for review: teach the bundled std's actual
shapes, not only renamed builtins. `wait_status_core(take child)` consumes a
`Child` and returns the core-aware integer; `poll_core()` returns a
`CoreChange`, whose status is normalized and whose `core` retains the flag.
The example polls after reaping its child, so all five wrappers execute without
a live-child polling race. These are recommendations, not lane decisions.

## Forbidden

No hand-edited console output, Mac builds, trailers, labels, merge, deployment,
tag, issue closure, planning-repo edit or ~/.claude edit. A compiler defect is
filed, never hidden by prose. Builds and process witnesses run only inside the
rootless container on hasu block B, CPUs 6–11, under hasu-B.lock, with 12 GiB.
No CI wait and no claim that a queued run passed.

## Inputs verified

- bs73 and bs72 contracts; bs72's `docs/audit/bs72-prediction.md` and rig;
  wolf-book #94/#91/#89 whole; wolf-lang's 0.2.35 CHANGELOG; RULES §1–§10;
  hasu-env README; STYLE.md and TONE.md.
- Neither lane checkout was pre-staged. The named wl70 note does not exist on
  wollf trunk or its wl70 branch. The orchestrator corrected the input to
  `train/data/repos/tables/census-wl70.json` and the retained per-exercise file
  on hasu; a fresh native census and the book's own rig are the record.
- wolf 0.2.35, cf0c13d38479f444b69bb8356ad5709d07eff323;
  lupin 0.1.56, 28547b4324524d5477a6e54e4f6888462461a21f.
  Archive SHA-256 values, each read twice:
  wolf `4cc4045bbbb605c07835bc4f32685953c784eeef4cd3139a48b457dfc90dad1a`;
  lupin `ad703550ac4787e8e4bb26989d591a731410274d7332f2b4e8b9a64c878822cc`.
  Bundled STD-REV, read twice:
  `2386522eb059cca5a84108562c2c0a0e72345d34`.
- #89 is already satisfied by bs72's 3e51c7e: the Mode literal names all ten
  fields. It is unchanged and passes again at this pin.
- README read, current. No command, layout or chapter-count statement moved.

## Evidence index

All paths below are under `docs/audit/bs73-evidence/`.

- `baseline-samples.log`: 537 samples, 534 passed, 3 existing pending,
  0 failures, 0 flips; 506/526 console blocks replayed.
- `census.json`, `census.py`, `wl70-comparison.json`, `table.py`,
  `exercise-table.tsv`, `measurement-summary.txt`: every one of the book's
  252 exercise roots is measured natively; all 33 wl70 anomalies are tabulated.
  wl70's 240-fixture scope counts pass 175, fail 18, refused 10, error 1,
  timeout 4, expects-fail 32. The fresh direct census counts ran 190,
  rejected 47, refused 11, timeout 4. The additional refusal is existing
  pending 7-5; other additional exits 2 are intentional fail directives.
  Native UB values need not repeat. The full rig also checks 285 book blocks
  and all declared interpreter, static-rejection and pending behavior.
- `wrappers-before-record.log`, `console-import.log`: the only drift is the
  added no-child poll line in §34.6. bs72's `record-consoles.py` imports the
  actual block from the rig's log. The unchanged one-meaning and Mode
  transcripts are replayed, not typed. Commit 5e8ae07 contains the wrappers,
  their prose and the recorded output.
- `final-samples.log`: 537 samples (252 roots, 16 members, 285 book blocks),
  534 passed, 3 existing pending, 0 failed, 0 flips in 382.8 s.
  Console replay: 506/526, comprising 249/251 book and 257/275 corpus blocks.
  The existing skips, declines and pending rows remain named in the log.
  `timing-and-index.txt` shows the final run opened after the chapter's last
  edit and records the tested chapter's SHA-256.
- `static-gates.log`: tiers regenerated and checked; backmatter regenerated;
  doc truth green; 211 unit tests passed; sample self-test caught 14/14 plants;
  planner tests PASS. `pinned-vendor-check.log` replaces the initial loud
  absent-sibling SKIP with a green comparison against a sparse checkout of the
  exact compiler pin (`spec-checkout.txt`), followed by tiers --check.
- Tier rows moved: none. EXERCISES-INDEX regenerated byte-identically
  (`timing-and-index.txt`). Its 352 rows comprise 239 derived and 113
  hand-kept rows without programs. Totals: 37 run (lupin), 9 run (lupin REPL),
  43 run (wolf), 174 run (wolf + lupin), 83 prose, 6 pending. Doc truth counts
  352 exercises, 303 printed, 268 corpus files; backmatter publishes 303
  solutions, 338 on file.
- `counts.txt`, `counts.py`: anchor keys 693→693, no removed, added or
  retargeted keys; diagnostic catalogue 187→187, unchanged set. Grammar
  unchanged, SHA-256
  `67816f8fcb04f09638bd98a53f4c951a1a16e30c2ddecf1a93bb0b49401948c7`.
  Active directives unchanged: run 345, lupin-run 51 (10 UB, 41 other),
  wolf-run 22, fail 95.
- `release-readback.txt`, `ctr.sh`, `gates.sh`, `gates-job.sh`: toolchain
  readbacks and the complete container recipe and gate driver.

Three persistent plants are draft PRs and must never merge:

- #96, `bs73-plant-exit17`, b2d218b4f30d3fd948d3ca652a77426dece2f0e1,
  run 40261264409: the Mode listing claims exit 17.
- #97, `bs73-plant-console`, 41614eb16a0b930127d3c4fdd7677266344d27bf,
  run 40261269554: the program prints a changed label; its console is untouched.
- #98, `bs73-plant-tiers`, 943f469ce6796808721a733a0e7d832660e0a96b,
  run 40261304222: 9-15's displayed tier changes without its header.

All were queued at their recorded id read. No red result is claimed; the
orchestrator watches and closes them red. Initial #95 run 40260885312 is
superseded, not plant evidence. The plant/restore pair is absent from #95's
final history and retained on the dedicated plant ref. The final head's run
id and the audit's last line are in #95's body and the hand-back, recorded after
the final push without changing the tested chapter.

## Done-when

- [x] Prediction and initial census committed before chapter changes.
- [x] All anomalies explained; preserve intentional traps and existing pending rows.
- [x] Five std wrappers, consuming Child wait and CoreChange poll taught and run.
- [x] Existing ten-field Mode literal verified at the pin.
- [x] Console imported by the rig recorder; final full samples have no failures/flips.
- [x] Index and backmatter regenerated; tiers and doc truth checked; counts re-pasted.
- [x] Pinned vendor comparison, unit tests, sample self-test and planner tests green.
- [x] README read, current; no Mac builds or attribution trailers.
- [ ] Orchestrator watches #95 at the final head, labels it for tier full,
  verifies Ubuntu and Windows green, then merges. This lane does not wait.
- [ ] Orchestrator closes #94/#89 from the evidence and the three plant drafts
  unmerged after red. No issue is closed by this lane.
