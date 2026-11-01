# r41q — the book at wolf 0.2.36 / lupin 0.1.58

_The book's part of r41q (the apps, the games and the docs to the published pair). The prediction is
`docs/audit/r41q-prediction.md`, committed first; this is the lane's record._

## Prediction

Scored against `docs/audit/r41q-prediction.md` (`56995fb`):

- **The stamps: as predicted.** `wolf 0.2.36 (wolfgang, pin 5adf3d9)`, `paired with lupin 0.1.58
  (reference interpreter), pin a5378e3`, `lupin 0.1.58 (wolf-interp, reference interpreter at pin
  a5378e3)`, read from the release archives on hasu.
- **The samples: as predicted, five console blocks and nothing else.** The unedited book at the new pair
  (`cdd581f`, the pin alone) ran 537 samples, 534 passed, 3 pending (report-only), 0 flips; the five
  failures were console blocks, every one a version string and nothing else: the colophon's and §1.2's
  `--version` pair, ch22's and exercise 22's `wolfi … toolchain` line (both hashes unchanged), and
  ch34's `conform-run` JSON (`commit`, `impl_version`; the program's output unchanged). The prediction
  named ch27 too, wrongly: its only 0.2.35 mention is a ledger comment, not a console.
- **The spec: as predicted.** Anchors 693 -> 706, 13 added, none removed, none retargeted; codes 187 ->
  192 (E0822-E0826), order kept; grammar one rule. Vendored byte-equal from wolf-lang `5adf3d91`.
- **The std: as predicted.** The 0.2.36 archive's `std/` is 59 files, manifest `f8a46216…`, the same
  from a wolf-std checkout at `78b9d3cb` staged as the samples job stages it.
- **`[lupin-trace]`: the first branch.** `placetrace.rs` changed between v0.1.56 and v0.1.58, but the
  four traces re-take byte-identical with the 0.1.58 archive (6 diagrams from 4 traces, checked), so the
  section follows `[lupin].rev` and CI builds no second lupin.
- **The literals: as predicted,** 22 entries re-read; the five that name the current pin moved.

## Forbidden

No merge, label, tag, release, deploy, issue filed or closed; no attribution trailer. No build on the
Mac (the importer, which edits text, ran there against the hasu log); every run of a compiler, the
interpreter or cargo on hasu, one block per run, both its lock names held, `--network none`. kasumi not
touched. No console text typed by hand: the five blocks are the real run's, imported by bs72's
`record-consoles.py`, which replaces only an exact unique expected block with the full actual one. No
vendored spec file edited: each is a byte copy. No chapter's teaching changed: the prose touched is the
pin's own (§1.2's stamp and pair paragraphs, the colophon's, ch22's toolchain sentence) and Appendix C's
count.

## Inputs verified

- Base: wolf-book trunk `c78cea5a`, as the contract read it on 2026-10-31.
- The pair, peeled from the API: wolf `v0.2.36` = tag object `86d61559` -> `5adf3d91`; lupin `v0.1.58` =
  tag object `dc4689e3` -> `6240efe5`. Digests: the release API's two passes (00:11:56Z, 00:11:57Z),
  identical and equal to r41's record; linux x86-64 `d383c74c…` and `34f52792…` hashed twice on hasu.
  `STD-PIN` at `5adf3d91` names `78b9d3cb`, the archive's `std/STD-REV`.
- The interpreter's new clause: `a5378e3` is wolf-lang's "corpus: gate variadic declarations by target"
  (2026-10-25), 138 commits after v0.2.35 and 123 before v0.2.36 (the compare API, read twice), an
  ancestor of `5adf3d91` and no tag. §1.2 and the colophon now say so: 123 commits, inside one release.
- A stale sentence found and fixed by the same re-read: §1.2 said "the `pin 9da7bdc` clause beside it"
  while the console above it printed `cf0c13d` (0.2.34's tag, left by an earlier bump); it now names
  `5adf3d9`, the console's.
- The 22 version literals, each sentence read: the current-pin ones (§1.2's three `0.2.36`, its
  `+dev.unknown`, its `0.1.58`; ch22's `toolchain`; the colophon's `0.1.58`) moved; the seventeen
  historical ones (ch04, ch05, ch06, ch09, ch13, ch27, ch30, ch32, ch33, Appendix E) are still true at
  the new pair (ch33's parking history concerns when the reactor is made, which 0.2.36's s260 kill rule
  does not touch) and only their clocks moved.
- README read: it names no version and no count a pin moves; current.
- Found, not owed by this pass: `CHANGELOG.md`'s last entry is bs58's (2026-10-03); the lanes since have
  not kept it (D65 says one entry per merged sprint). Listed as an issue to file.
- **R-r41q-7, `[lupin-trace]`:** (A) follow `[lupin].rev` on byte-identical re-taken traces; (B) stay at
  0.1.56's rev. Recommend and implemented (A), measured.
- **R-r41q-8, the CHANGELOG gap:** (A) list it for the book's owner lane; (B) write bs59-bs73's entries
  here. Recommend and implemented (A): this pass is the pin, and those entries are other lanes' records.

## Evidence index

- `56995fb` the prediction, before any change.
- `cdd581f` the pin: `[wolf]` 0.2.36 at `5adf3d91`, `[lupin]` 0.1.58 at `6240efe5`, each with its digest
  line.
- `8a38484` `book.yml`: the staged std's manifest is the 0.2.36 archive's `f8a46216…`.
- `5fec9f3` the current-pin prose (§1.2, the colophon, ch22) and the 22 version literals re-read.
- `47f6600` `vendor/spec/` at wolf-lang `5adf3d91`, the key sets in `docs/audit/r41q-evidence/anchor-sets.txt`.
- `65920b5` Appendix C: the catalog holds 192 codes.
- `0ccaee4` the five consoles imported from the real run (`docs/audit/r41q-evidence/baseline-samples.log`,
  `console-import.log`).
- `1ec81f4` `[lupin-trace]` follows `[lupin]` (`docs/audit/r41q-evidence/trace-lupin-0.1.58.log`).
- `20a9422` Appendix A and the Solutions regenerated by `cargo xtask backmatter`.
- The gauntlet at the final head, on hasu (`/home/mfwolffe/lanes/r41q/book/book-gauntlet.sh` in
  `book-ctr.sh`, under `bin/block`; log under `/home/mfwolffe/lanes/r41q/book/logs/`): the head, the
  pins against the expected revs and lines, the staged-std manifest against `book.yml`'s, the xtask
  build, `cargo xtask samples` (no failures), `samples --self-test`, `diagrams --check` with the pinned
  lupin, `verify-docs`, `ledger --check`, `backmatter --check` against wolf-lang `5adf3d91`, `contrast`,
  `cargo test -p xtask`, `tiers --check` and `ci/tiers-test.py`. Every step prints `END <step> exit=N`
  and the wrapper stops at the first nonzero one. The END lines and the counts are in the PR body.

## Done-when

The PR is open at its final head with `tier pr` green, `Footprint: cross-cutting` (the pins and CI), the
audit run once and passing but for the labelled `tier full` (the samples on ubuntu and windows), which
the orchestrator labels and watches. wolf-web's pin to the book follows this merge. To file: wolf-book,
the CHANGELOG has no entry after bs58 (R-r41q-8).
