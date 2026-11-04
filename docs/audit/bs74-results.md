# bs74 — the enum primer

_The book's first enum was declared inside §5.6's `Ord` block and explained in one
sentence. This lane put a primer before it: §3.5 Enums, directly after §3.4, with an
enum's `Eq` in §5.6 and a generic enum in §5.3, where traits and brackets are taught.
The prediction is `docs/audit/bs74-prediction.md`, committed first; this is the
lane's record._

## Prediction

Scored against `docs/audit/bs74-prediction.md` (`a9e1bd7`, its counts `842b72c`, the
post-ruling rows `56298aa`):

- **The census: as predicted.** One enum in the whole book, `Ordering`, first at
  `book/ch05.md:917` (trunk `94bce4e`); no payload variant, bare arm, exhaustive
  `match`, method, printed variant, `==` or generic enum anywhere, and no exercise
  solution with an enum in it. The table is below.
- **The machines: as predicted, row by row**, measured before any fence was
  written (`docs/audit/bs74-evidence/probes/`) and again on each fence as written,
  extracted from the branch (`probes/fences.log`): the six `run(…)` fences print the
  same bytes on both machines, the three `fail(…)` fences are the compiler's E0301,
  E0801 and E0301, and the `lupin-run(…)` fence prints `340 0` under lupin while
  `conform-run` says `unsupported`.
- **The draft's measure was not taken.** The run queued at `842b72c` was stopped
  before its turn and ran nothing, so the draft's counts (545 samples) are
  superseded rather than scored; the post-ruling measure below covers everything
  they did.
- **The samples: as predicted, exactly**, at `79aef57`
  (`docs/audit/bs74-evidence/measure-79aef57/`). The first run: 549 samples (256
  corpus roots, 293 book blocks), 546 passed, 3 pending, 11 failed, 0 flips, and
  the eleven are the new blocks and nothing else: three `diagnostic,from(…)` blocks,
  four book consoles and four corpus consoles. It blessed four snapshots, the three
  §3.5 diagnostics and `ch03/ex3-16`. After the import: 549, 546 passed, 3 pending, 0
  failed, 0 flips; 514 of 534 console blocks replayed, 253 of 255 on book pages and
  261 of 279 in the corpus; three warning sets asserted, `ex3-17b`'s E0802 the new
  one. One shape the prediction did not say: the replay joins a command's stdout and
  stderr in that order, so 3-17b's transcript prints its four lines before the
  warning, and the page keeps the rig's order.
- **The back matter: as predicted, with three figures it missed.** `cargo xtask
  tiers` wrote no change (3-16 and 3-17 read `run (wolf + lupin)`, as entered);
  `cargo xtask backmatter` regenerated the Solutions page; `tiers --check`,
  `backmatter --check` and `ledger --check` passed. `verify-docs` held three
  hand-kept figures the list did not name: EXERCISES-PENDING's 268 directive-headed
  files (272 now) and the index's taxonomy spread, comprehension 157 and extension
  81 (158 and 82). `EXERCISES.md` §2's 328 is not held by any gate, and moved with
  nothing this lane did.

## Forbidden

No merge, label, tag, release or deploy; no issue filed or closed by this lane (the
six drafts were filed by the orchestrator); no attribution trailer. No compiler or
interpreter source read: the spec at wolf-lang `5adf3d91`, its corpus programs, the
docs, and black-box runs of the published archives only. No console block, diagnostic
block or snapshot typed by hand: the consoles were imported from the rig's drift log
by bs72's `record-consoles.py`, the diagnostics spliced from the snapshots `cargo
xtask samples --bless` wrote, by `docs/audit/bs74-evidence/splice-diagnostics.py`,
and the tier column and the back matter written by `cargo xtask tiers` and `cargo
xtask backmatter`. Every compiler, interpreter and cargo run on the Linux lane was
in a container with no network, one CPU block, both of its lock names held; single
programs were probed on macOS under the build lock. No defect was worked around in
prose: each one found is a ledger row with its issue.

## Inputs verified

- Base: wolf-book trunk `94bce4e`, which contains `dadc38b`. Pins unchanged: wolf
  0.2.36 (`5adf3d9`), lupin 0.1.58 (`a5378e3`).
- The archives by digest, each read twice: linux x86-64 `d383c74c…` and `34f52792…`
  (the pin's own lines); aarch64-apple-darwin `b79d5584…` and `c1b02771…`, equal to
  the release's.
- The spec read at the pin: `[gram.expr.variant]` (a value names its enum; a bare
  capitalized name in value position is a row tag's spelling, E0301 with the fix),
  `[gram.pat.nullary]` (a bare path is a pattern; a payload variant written bare is
  E0808), `[type.interp.agg]` (an enum value renders `Enum.Variant(p1, p2)`),
  `[type.trait.op]` (nothing is synthesized: an enum without `impl Eq` does not
  compare). The corpus programs `grammar/match_nullary_variant.lu`,
  `typecheck/variant_bare_value.lu` and `typecheck/enum_union_two.lu`.
- The census, by grep over `book/`, `principles/exercises/` and `samples/` at
  `94bce4e`:

  | idea | first use | section |
  |---|---|---|
  | declaring an enum | `ch05.md:917`, `enum Ordering`; one sentence at :953 | §5.6 |
  | constructing `E.V` | `ch05.md:923`, `Ordering.Less` | §5.6 |
  | a payload variant | none; the look-alike is the row tag `NotANumber(Bad)`, `ch06.md:88` | §6.1 |
  | the bare arm | none over an enum; bare row-tag arms at `ch06.md:466` | §6.6 |
  | exhaustiveness | over `int` at `ch03.md:688`, over a row at `ch06.md:518`, over an enum nowhere | §3.4, §6.6 |
  | guards | `ch03.md:263`, `d if d < 10 =>` | §3.2 |
  | methods | none on an enum; trait methods from `ch05.md:757`; the first inherent `impl` at `ch07.md:833`, unintroduced | §5.6, ch07 |
  | `Eq` | on a struct at `ch05.md:877`; on an enum nowhere | §5.6 |
  | a printed variant | none | — |
  | generic enums | none; generic functions at `ch05.md:389` | §5.3 |
  | rows against enums | the tag case rule `ch06.md:66-74`; `Weird` at :198; the clash rule at :497 | §6.1, §6.6 |
  | "variant" for a row tag | `ch06.md:182` (6-3), :429 (6-2), `ch27.md:284` | — |

- The placement, weighed (R-bs74-1): §3.4 rewritten over an enum; a new §3.5; a new
  section in chapter 5 before `Ordering` (a §5.7 sitting between §5.5 and §5.6, the
  §6.6 precedent); a section at chapter 5's head. Ruled B, §3.5.

## Evidence index

- `a9e1bd7` the prediction, before any chapter changed; `842b72c` its counts;
  `56298aa` the post-ruling rows.
- `4e3a3f3`, `9d3440f`, `4c942a0` §3.5 Enums (the draft, its prose, the nested
  spelling).
- `6ddfef2` §5.3's generic enum; `fe75d2d` §5.6's `Kind` `Eq` and `Ordering`'s
  pointer back.
- `a803ed0` ch03's ledger: the enum papercut closed, six rows citing wolf-lang#1043,
  #1044 and wolf-interp#324–#327; `2aa3ed1`, `4935f84` ch05's ledger: the generic
  enum's row on wolf-lang#881 and the local-declaration inventory.
- `e3012e5`, `39fda0d` "variant" respelled "tag" for a row's member (6-2, 6-3, §27.3).
- `9c9c425` Appendix C; `0094546` the glossary; `f874993` the index; `19ca72f`,
  `6232c39` the TOC rows and delta.
- `420a7a0` exercises 3-16 and 3-17, four corpus programs; `b5f9ce6`, `4a479db` the
  index rows and counts.
- `79aef57` the splicer, in the tree.
- `c0a54f5` the four book consoles, imported from the rig's drift log; `7cd47cd`
  §3.5's three diagnostics, spliced from their snapshots; `ece550f` the four corpus
  consoles and `ex3-16`'s snapshot; `c7b9d50` the Solutions page, regenerated.
- `13f4909`, `21e8a3e` the two figures `verify-docs` held.
- The gauntlet at the final head, on the Linux lane: the head, the pins, the
  binaries, the archive's std manifest against `book.yml`'s, the xtask build,
  `cargo xtask samples`, `samples --self-test`, `diagrams --check`, `verify-docs`,
  `ledger --check`, `backmatter --check`, `contrast`, `cargo test -p xtask`, `tiers
  --check` and `ci/tiers-test.py`. Every step prints `END <step> exit=N` and the
  wrapper stops at the first nonzero one. The END lines and the counts are in the PR
  body.
- The plant, `bs74-plant-console`: this branch's head with one line of §3.5's first
  console edited by hand. `cargo xtask samples` must fail it by name, `book/ch03.md`
  and the block's line, `console block drifted from the real run`; the red is in the
  PR body. The plant is local and opens no PR.

## Done-when

The PR is open at its final head with `Footprint: module`, the gauntlet green at that
head, the plant red by name, and the audit run once. Rulings, as made: R-bs74-1 (B,
§3.5), R-bs74-2 (a local `trait Eq`, as §5.6 declares its others), R-bs74-3 (the
generic enum in §5.3 as a `lupin-run(…)` block, the compiler's wording and
wolf-lang#881 in the ledger row), R-bs74-4 ("tag", not "variant", for a row's member).
