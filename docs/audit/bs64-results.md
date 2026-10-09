# bs64 — results against the prediction

The book at wolf 0.2.26 (wolf-lang `89dc1394`, release 408143286) /
lupin 0.1.49 (wolf-interp `f516a5f4`, release 408028965), from 0.2.25 /
0.1.48. Prediction `6d90749`, committed before either archive was
downloaded. Head gate on kasumi at `1cc9c1a`
(`bs64-evidence/gate-1cc9c1a/`, every gate exit 0, SKIPs counted in
`gate-summary.txt`); every later commit is under `docs/audit/` or the
plant and its revert. CI at the PR head: see the PR.

## The move table: every program, 0.2.25 / 0.1.48 against 0.2.26 / 0.1.49

577 programs (bs63's `allprogs.py`: every book fence outside `back/`,
`part()` fences assembled, every corpus `.lu`), each alone, on lupin,
`conform-run --json` native and `--checked`, and a `--release` build and
run for every native `pass` with a `main` (368). Each pair swept twice:
`sweep-c25a/b.jsonl`, `sweep-newa/b.jsonl`; noise `sweep-c25-noise.txt`,
`sweep-new-noise.txt`; the comparison `sweep-c25-vs-new.txt`.

| program | machine | 0.2.25 / 0.1.48 | 0.2.26 / 0.1.49 | why |
|---|---|---|---|---|
| ch08 §8.2 s3 (`newest`), s4 (8-3's fence), ex8-3 | native, checked | `fail(E1010)` | `fail(E1010)`, the note one sentence longer | E1010's note names `copy region` (s216, `wolf_mem`); `cmp.py` compares codes, so no sweep row; three snapshots re-blessed |
| ex16-7, ex16-8 (maze bitmasks, `walls[c] & 14`) | checked | `unsupported`: «this operator in checked execution» | `unsupported`: «structured concurrency in checked execution (C1 deferred)» | **not predicted**: the checked machine now runs `&` on plain integers (s213, #575) and declines at the next construct; the verdict class holds, and no sample reads the checked machine |
| every other program | all four | — | — | no verdict, stdout, code or lupin exit moves |

Release rows that move are the scheduler's and UB's, and each moves
between two runs of the same archive too: B-11 and ex9-4's exit status
after UB; ch17's balances and §17.1; ex11-5's worker order; ex30-4 and
ex30-5's arrival order; and, at 0.2.26 only, ch10's first scope program
(`book/ch10.md:84`, "the scope is still open" and "the reader is
counting" in either order), which moves inside `sweep-new-noise.txt`
itself. **No book program printed a wrong answer that 0.2.26 corrects,
and #618 refuses none**: no program holds a declared fn's `str` result
past a region.

Samples runner: control (c25, unedited `851b996`) 512 / 509 / 3 / 0
failed / 0 flips, 485 of 505 consoles; subject (unedited `6d90749`, the
new pair) **506 / 3 / 12 failed / 0 flips, 477 of 505**; head
**515 / 512 / 3 / 0 / 0, 488 of 508** (also under `taskset -c 0-3`).

## Predicted, and what happened

| prediction | result |
|---|---|
| stamps to the character | as predicted (`members.txt`) |
| distance 213 commits, two releases | 213; the two sentences rewritten (`15ae4a7`) |
| anchors 595 → 607, none dropped; grammar unchanged; catalogue 187 | as predicted (`anchors-diff.txt`; `backmatter: appendix-a.md up to date`) |
| 11 members; `_wolf`, `wolf.bash`, `wolf.fish`, both licences = 0.2.25's | as predicted |
| GLIBC ≤ 2.34 | 2.34 on both |
| subject: 506 / 3 / 12 failed / 0 flips, 477 of 505, the twelve by name | exactly those twelve (`subject-gates.txt`) |
| appendix E's reason line `… at tally/main.lu:L:5 (+1 more)`, L = 3 or 4 | L = 4 |
| sweep: native and lupin 0 rows; checked 0 rows; release noise only | native 0, lupin 0, release noise only; **checked 2 rows, missed** (above): my scan for `&` on an integer used a pattern that needs a name or `)` before the operator and missed `walls[c] & 14` |
| traces byte-identical on the release lupin 0.1.49 | 6 diagrams from 4 traces, checked, kasumi and CI |
| head 515 / 512 / 3 / 0 / 0, 488 of 508 | as predicted |
| ledger 127 → 126 open, 108 → 109 closed | as predicted (the E1010 explain row, `b4fee94`) |
| verify-docs tiers unchanged; Appendix C 187 / 60 | as predicted, after two edits verify-docs asked for (below) |

Not predicted: verify-docs refused two things my new prose said, and both
were right. §2.3 named E0409, which Appendix C does not list, so the
float refusal is now named without its code (`4793644`); §5.6's quote
said "at this pin" on one line, which TONE.md's tense rule keeps in the
ledger (`e97617f`). And the declined 24-6 row's premise was wrong (below).

## The new surfaces, placed

Each is a `part(…)` fence, so no positional sample id after it moved, with
a `run(…)` directive (lupin and `wolf run`) and a console block the replay
checks. Each was also run on all four machines, and on the control pair,
where each is refused (`probes-new.txt`, `probes-c25.txt`):

| ruling | where | fence | 0.2.26 / 0.1.49, four machines | 0.2.25 / 0.1.48 |
|---|---|---|---|---|
| #51, `!` on integers | ch02 §2.3 (bytes, where masks are named), and ch05 §5.6's sentence | `part(mask)`: `n & !0xff`, `(!b) as byte` | `-301 44 256 -201 55` on lupin, native, checked, release | lupin `unsupported` (`!` needs a bool), compiler E0409 |
| #50, `-> never` | ch06 §6.2, beside `else |err|`'s "the block's value becomes the expression's" | `part(die)`: `else |_| die(…)` in an `int` position | `comma at 3` everywhere; the trap twin `trap(assert)` everywhere; a `-> never` body that reaches its end is E0401 on all four | E0301 `never` |
| #56, `copy region` | **ch08 §8.2** (regions), beside the E1010 repairs, not ch07 (contract drift, `851b996`) | `part(keep)`: a command loop keeping one summary per turn | three summaries everywhere | compiler E1010; lupin exit 3 |

The prose claims beside them were measured too: an outer binding assigned
inside a `copy region` block is still E1010 (lupin `region-fault`), a
channel as the block's value is E1010 by name (lupin `region-fault`);
`!` on a float is E0409 on every machine.

## Seen red

Plant `cb50c3b` took s217's reason line back out of ch24's first
`pkg/acquired` transcript, the 0.2.25 shape of that block, so only a
console replay against a 0.2.26 compiler can see it. Run 37978331246
(PR #76): `samples (ubuntu-latest)` 113982177123, `samples
(macos-latest)` 113982177183 and `samples (windows-latest)` 113982177195
red with exactly one failure each, `book/ch24.md:240: console block
drifted from the real run`; the rig's `test` jobs and `render` green
(`plant-red-37978331246.txt`). Reverted at `56a8cd4`. The plant run also
gives the CI counts with one console failing: ubuntu and macOS 512 / 3 /
1, 487 of 508; windows 511 / 3 / 1, 424 of 508 (windows' declared
prefork refusal and its OS lane, as at trunk).

## Sentences fixed

- ch01 §1.2 and the colophon: `294d626` is v0.2.24's tag, two releases
  and 213 commits back, no longer the previous tag (`15ae4a7`).
- ch05 §5.6: `!` is `bool`'s and an integer's (`2f06e9b`, `e97617f`).
- ch08 §8.2: the fourth repair (`ca3aa99`); ch08's ledger row on the
  E1010 explain closes, measured on both pairs (`b4fee94`).
- ch22 §22.3: `toolchain 0.2.26` (`d02a744`).
- ch24 §24.3: the reason line, and "reach" is the code, not only the
  imports, measured with a dependency that calls `fs_read_text` and
  declares nothing: 0.2.25 builds it and audits `effective: []`, 0.2.26
  refuses it with E1504 and audits it UNDECLARED (`16cb114`,
  `pkgprobe.txt`). Appendix E: the promoted package's reason line
  (`d06d787`).
- The version-literal ledger re-read at 0.2.26 / 0.1.49 (`9f8da66`).

Every drifted transcript was regenerated from a run: snapshots by
`samples --bless` (`21a7ed2`); console and `diagnostic,from` blocks by
`splice.py` from the samples log's `actual:` (`7a937eb`, `3f066f3`,
`14ae231`); `solutions.md` by `backmatter` (`80949a4`). The declined 24-6
block, which nothing replays, from a staged run of the exercise's own
steps on 0.2.26 (`6227b33`, `pkgprobe.txt`). `splice.py` had a bug bs63
never hit: it applied a file's drifts top-down, so a block that grew moved
the line of the next one in the same file (ch24 has two); it now applies
them bottom-up (`ea65c9b`).

## Corrections

- **The contract**: `copy region` belongs in chapter 8, where regions and
  E1010 are taught; chapter 7's `copy` section says nothing about regions
  and needed no edit.
- **samples-declined.toml, 24-6's row** said the transcript "runs `wolf
  update` BEFORE `wolf audit --ci` and still expects the ACQUIRES line,
  which is the opposite of what the page's own prose says". It is not:
  the exercise records the world while `regex` declares nothing, then
  edits the manifest, then audits; measured exactly so on both pairs. The
  row's `needs` and `retires` now say so.
- **My prediction**: the checked machine's two ch16 rows (above).

## Filed

- wolf-lang#639: E1504's `--explain` describes only the import half since s217.
- wolf-lang#640: E0501's note says `!` is `bool`'s alone (stale since
  ruling #51), and `!x` / `x && y` on a bare `T` draw a stray E0401.
- wolf-interp#221: lupin runs `!x` and `x && y` on an unbounded type
  parameter; the compiler refuses both with E0501.
- A witness on wolf-lang#555: on release an `assert` inside an inlined
  helper reports the caller's site (0.2.25 and 0.2.26;
  `probes-release-site.txt`).
