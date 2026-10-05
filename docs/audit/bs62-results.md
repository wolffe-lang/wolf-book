# bs62 — results against the prediction

Head gate on kasumi at `c63d457` (wolf 0.2.23 / lupin 0.1.46 from the
release archives, trace lupin built at wolf-interp `a56f924`): every gate
exit 0, logs in `bs62-evidence/gate-c63d457/`. Four SKIP lines in the
samples log, all four present at trunk (solutions.md extraction, two
console blocks needing a shell or `grep`, the 12 REPL blocks).

## Predicted, and what happened

| prediction | result |
|---|---|
| F1–F8 false or misleading, fixed | all eight fixed as predicted |
| the table in §7.1 as a `###`, no section renumbered | as predicted; the `return` row reads "copies out / moves out to the caller" rather than "leaves the function" |
| three named parts, `sN` ids unmoved | as predicted; no snapshot or `diagnostic,from(…)` binding moved |
| samples 509 → 512, 509 pass, 3 pending | 512, 509 pass, 3 pending, 0 failed, 0 flips |
| console blocks +3 | 482 → 485 of 502 → 505 replayed |
| six diagrams | six: `s3-L5` move, `part-packcopy-L5` take, `-L6` re-initialization, `-L7` Copy read, `part-handover-L4` struct plain move, `s7-L4` copy |
| ledger ch07 open 6 / closed 8; book 128 / 107 | ch07 open 6 (filed 4, waived 2), closed 8; book 128 open, 107 closed |
| one filing, the `distinct int` divergence | filed wolf-lang#594 |

## Not predicted

- Two more sentences, found while editing: §7.3's "the only duplication
  in the program is the one the loop asks for by name" (the loop's
  `copy d.title` copies a `str`, which a plain `=` copies anyway;
  measured, `best = d.title` runs on both machines) and exercise 7-12's
  stem ("move a string out of one binding into another": a plain `=`
  copies it, so the stem now says `move`). The corpus page's 7-4 answer
  ("Wolf's rule is per-decision, not per-type") was narrowed to the
  types you define. Solutions page regenerated.
- 7-12's REPL block moved from EXERCISES.md:550 to :553;
  `samples-declined.toml` follows it.
- bs61 merged during the lane; rebased onto `76143be` (two conflicts,
  `xtask/src/main.rs` and the workflow, both kept).

## Diagrams

- `cargo xtask diagrams` writes the fence body (text), the SVG under
  `book/diagrams/ch07/`, and the trace under `snapshots/traces/ch07/`.
  `--check` redraws and diffs; with `LUPIN_TRACE` it re-takes the trace
  too. CI: the rig lane checks without a toolchain on three hosts; the
  samples lane builds the `[lupin-trace]` lupin and re-takes the traces.
- Traces first taken with a lupin built at is72's work-in-progress
  `6b4c380`; re-taken at `a56f924` (is72's handed-over head),
  byte-identical.
- Seen red: a planted trace in which `let c = p.lead` MOVED the field
  (`3c579d8`), run 37364533746, `test (ubuntu-latest)` job 111946456081
  (`plant-red-ubuntu.log`): ch07.md:425's text and
  `part-packcopy-L7.svg` named; `test (windows-latest)` red too.
  Reverted at `406fba4`.
- SVG digests: part-handover-L4 5f9fae68…, part-packcopy-L5 940a1ba5…,
  -L6 949cdef7…, -L7 3e63609b…, s3-L5 65406175…, s7-L4 d7209369….
- Traces: part-handover 95b3813a…, part-packcopy 568a9369…, s3
  dd762f99…, s7 3bea5828….
