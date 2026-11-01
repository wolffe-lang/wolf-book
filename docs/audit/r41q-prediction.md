# r41q — prediction before the pin changes

2026-11-01. Control wolf-book `c78cea5a` (bs73's record). Contract: `wolf/sprints/release/r41q-the-app-pins.md`
(the book's lupin and wolf pins, as its own CI reads them). No binary of the new pair has run against the
book yet. CI is the build host; hasu runs the release binaries. No Mac build, no kasumi. No merge, deploy,
tag or issue closure.

## Inputs and predictions

- wolf v0.2.36 is `5adf3d91f75b3545ff8d38e5b6693b0b9f367a05` (tag object `86d61559`); lupin v0.1.58 is
  `6240efe5570dba1a33ba6269e3ba44bae33dd972` (tag object `dc4689e3`). Both peeled from the API and
  compared with r41's record (`wolf/sprints/release/r41-evidence.md`); the linux x86-64 archives hashed
  twice on hasu (`d383c74c…`, `34f52792…`).
- The compiler prints `wolf 0.2.36 (wolfgang, pin 5adf3d9)` and `paired with lupin 0.1.58 (reference
  interpreter), pin a5378e3`; lupin prints `lupin 0.1.58 (wolf-interp, reference interpreter at pin
  a5378e3)`. The interpreter's clause no longer names v0.2.24's `294d626`: the pin moved with lupin.
- The unedited samples at the new pair fail the version-string consoles (ch01, the colophon, and the
  stamps in ch27 and ch34 that print them) and nothing else is predicted to move. 0.2.36's documented
  changes reach no program the book runs: no book source declares a new prelude name (`VaList`, the
  four `va_*`, `env_arg0`, `env_args_bytes`); the `else`-range refusal text (#932) and the spawn
  `os_error()` numbers (#936) are re-checked by the run, not assumed. Any other drift is classified in
  the results, never imported blind.
- The spec moves: `vendor/spec/anchors.json` 693 -> 706 keys (13 added: the ten `abi.c.variadic*`,
  `abi.target.main`, `exec.checked.argv`, `os.env.argv`; none removed, none retargeted);
  `vendor/spec/diagnostic-codes.txt` 187 -> 192 (E0822-E0826 added, none removed, order kept);
  `vendor/spec/grammar.ebnf` one rule (`params` gains the variadic `','  '...'`). Vendored as copies
  from wolf-lang at the pinned rev, never edited; Appendix A and the other generated pages regenerated
  by `cargo xtask backmatter`.
- `STD-PIN` moves `2386522e` -> `78b9d3cb`: the samples job's staged-std manifest (`fa26d724…`, 0.2.35's)
  is re-measured from the 0.2.36 release archive's `std/` and from a wolf-std checkout staged as the job
  stages it; the two must agree.
- `[lupin-trace]`: `src/eval/placetrace.rs` changed between v0.1.56 and v0.1.58, so the four chapter-7
  traces are re-taken with the 0.1.58 interpreter. Byte-identical: the section follows `[lupin].rev`.
  Otherwise it stays at 0.1.56's rev and CI builds the second lupin (R-r41q-7).
- `audit/version-literals.txt`: 22 entries carry `audited-at-wolf=0.2.35` or `audited-at-lupin=0.1.56`.
  Each sentence is re-read at the new pair. The literals that name the current pin (ch01, ch22, the
  colophon) move; the historical ones keep their text and their clock moves.

## Gates

- [ ] Release digests read twice and compared with r41.
- [ ] `cargo xtask samples` on both machines at the new pair, no failures, the consoles imported from
      the real run (no console text typed by hand).
- [ ] Vendored spec copies equal wolf-lang at `5adf3d91`; anchor key sets diffed both ways.
- [ ] Generated back matter current; `verify-docs`, `ledger --check`, `diagrams --check`, `contrast`,
      `cargo test -p xtask`, `samples --self-test`, `ci/tiers-test.py`.
- [ ] README read against the final change.
- [ ] Final head `tier pr` green; the orchestrator labels for `tier full`.
- [ ] `lane-audit.sh` once, its last line read directly.
