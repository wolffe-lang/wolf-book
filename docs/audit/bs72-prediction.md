# bs72 — prediction before the pin changes

2026-10-23. Control wolf-book f1fd824. Contract:
`wolf/sprints/book/bs72-the-book-at-0235.md`; wolf-book#92.
No binary has run and no archive has been downloaded by this lane.
CI is the build host; hasu runs release binaries when assigned. No Mac
build and no kasumi work. No merge, deploy, tag or issue closure.

## Inputs and predictions

- wolf v0.2.35 is cf0c13d38479f444b69bb8356ad5709d07eff323;
  lupin v0.1.56 is 28547b4324524d5477a6e54e4f6888462461a21f.
  Annotated tags were peeled to their commits. Compare r40-evidence.md.
- The compiler prints `wolf 0.2.35 (wolfgang, pin cf0c13d)` and
  `paired with lupin 0.1.56 (reference interpreter), pin 294d626`.
  lupin prints `lupin 0.1.56 (wolf-interp, reference interpreter at pin 294d626)`.
  Its clause is eleven releases / 1338 commits behind the compiler.
- The unedited samples at the new pair fail the four scalar-only ch08
  verdicts and seven version-string consoles; ch27/ex27-3 graduates.
  The initial pin-only PR run is the planted red. Its CI annotation
  must name the deliberate plant; the evidence stays in the PR history.
- `[mem.region.scalar]` says a type that transitively reaches no storage
  names no region. str reaches storage. All four old fail fences stay;
  their examples gain str fields. Runnable scalar-only counterexamples
  test surviving a region block and changing words read from a freeze.
- R-bs72-1: retain those refusal demonstrations and teach their scalar
  contrast. R-bs72-2: distinguish copy region from materializing copy at
  an outer store. R-bs72-3: graduate ex27-3 and retire its spilled-union
  ledger row. Recommended and accepted by the orchestrator before edits.
- The raw active corpus + book one-machine counts are lupin-run 52,
  wolf-run 23; 10 lupin-run fences are trap(ub), outside #268's
  graduation inventory. Thus #92's 42 -> 41 is 52 -> 51 including UB.
  run 342 -> 345 predicted (one graduation, two scalar counterexamples).
  fail 95 -> 95. No refusal fence becomes a pass fence.
- Anchor keys 672 -> 693, 21 added, none removed or retargeted;
  diagnostic codes 187 -> 187, identical sets; grammar.ebnf unchanged.
  Vendor deliberately; regenerate derived pages and snapshots on the
  execution host. No console output is typed by hand.
- STD-PIN moves e036bec -> 2386522e: CI's staged-std manifest must be
  remeasured from the release archive. Top-level help is expected not
  to move: help.rs changes wolf test's detailed help, not its summary.

## Gates

- [ ] Release digests read twice and compared to r40.
- [ ] Pin-only CI plant red, read once and retained.
- [ ] Four preserved refusal fences and both scalar counterexamples.
- [ ] cargo xtask samples on both machines at the new pair, no failures.
- [ ] Generated pages, snapshots, vendor key sets and index current.
- [ ] README read against the final change.
- [ ] Final head tier pr green; full-matrix added only then.
- [ ] lane-audit last line read directly; full watch handed back.
