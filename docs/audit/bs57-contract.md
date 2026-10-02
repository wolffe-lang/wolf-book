# bs57 — chapter 6 gains `match` over the value (ruling #21's book half)

**Class:** medium (wolf-book), **Fable**. **Wave:** 52. **Authored
2026-10-01 by the lane**, from the orchestrator's brief, STATUS.md
ruling #21 (`[type.row.match]`, ruled 2026-10-02) and bs55's contract
(`docs/audit/bs55-contract.md`) as template. Two oracles:

- **The pin:** the samples gate (`cargo xtask samples`) on kasumi
  against the release archives by digest (0.2.19 / 0.1.42, the book's
  pin), and on three CI hosts. Every sample the chapter already prints
  still replays; the three new samples run **report-only** as pending
  rows and are expected to fail there.
- **The branch:** s197's compiler, built by this lane under
  `~/lanes/bs57/` on kasumi from wolf-lang branch `s197` the day it
  exists, run over the same three programs; the bytes recorded. No
  published toolchain serves the form today (wolf-lang#497).

Deliverable, from the brief: chapter 6 gains the form in its own
section — `match` straight over a fallible value, row arms and value
arms, `_` on both halves, the E0801 that names the missing tag, the
row consumed, and when to prefer it over `else |e| match e`; one worked
sample and one deliberate refusal, both pending in
`samples-pending.toml` with the exact expected stdout and the exact
diagnostic text from s197; one exercise with its solution, pending the
same way; §6.2's "`?` and `else`" sentence revised to name all three
forms, consistent with `[type.row.operand]`; no number moves; the render
clean; every other sample replaying at the current pin.

## 1. Forbidden, absolutely
- No `rm` outside `~/lanes/bs57/` on kasumi, `/private/tmp/bs57`, and this lane's scratchpad folder; no deletion in any tree this lane did not create.
- No `git add -A`; no edit to another lane's file; no `~/.claude`; no edit to the planning repo, to wolf-lang or to wolf-interp. **No edit under `theme/` or `tests/contents/`** (bs56 is in flight there).
- No `cargo` on nomad-1 (this Mac): every build, render, probe and sample run is on kasumi, `CARGO_BUILD_JOBS=4`, launched with `setsid` and a recorded pid, never `ssh -f`; tars with `COPYFILE_DISABLE=1`.
- No deploy; no merge, no rebase-merge; no `2>/dev/null` on a checkout; assert the branch before every commit.
- No "seen red" or "replayed" without a run id, sha, path or digest in the same paragraph.
- The pin is taken from the RELEASE ARCHIVE by digest, never from a clone or `~/.local/bin`; the s197 build is a BRANCH build and is named as one — it stamps `+dev.<sha>` and claims nothing.
- The checked machine answers only via `wolf conform-run --json --checked`; `wolf run --checked` is the native build.
- No renumbering of any chapter, section, exercise, anchor or file name (how-to-read's rule, EXERCISES.md §1): the new section is **§6.6**, the new exercise **6-15**, and both take the next free number whatever their position on the page.
- Tool output verbatim from runs, never hand-typed (STYLE.md); no `...` in a diagnostic; the expected bytes of a pending sample are taken from s197's build and the snapshot is blessed from that run, named as a branch run.
- Reader-facing prose stays in the present tense (TONE.md's tense discipline): the gap between the page and the pin is stated in the manifest rows, the chapter's ledger, this contract and the PR — never on the page as "not yet".
- Kill only this lane's own pids — never a pattern, never a process group. Wait in printing loops; poll CI with `gh run view`, never `gh run watch` without `--interval 60`.
- Predict before measuring (§3 below, committed with this file, before any archive is downloaded or any probe runs).
- kasumi `/home` is at 97%: archives, clones and `target/` trees are pruned as soon as their evidence is written.
- No attribution trailers on any commit or PR.

## 2. Inputs, verified (re-derived 2026-10-01, first act)

| claim | command | result |
|---|---|---|
| wolf-book trunk | `git rev-parse origin/trunk` | **`33377456835c1aae7971bdb344ee46294c4b73d1`** — bs56 merged (PR #68, 2026-10-02 03:31Z) after this lane launched; the brief's `bd3484ed` (`bd3484edd6c3…`, bs55's merge) is its parent chain's head at launch. The branch is based on the new trunk before its first commit |
| book pins wolf 0.2.19 `c2401f05` / lupin 0.1.42 `8e2516dc` | `wolf-toolchain.toml` `[wolf].rev`, `[lupin].rev` | ✓ `c2401f05f37794a078d2acf62f837dad98e5950d`, `8e2516dc47bf808512388cc687e070981d331d98` |
| the release archives the gate runs | `gh release view v0.2.19 / v0.1.42 --json assets` | linux-x64 digests `9f3873d8…` (wolf) and `9856335a…` (lupin), neither draft nor prerelease; identical to bs55's `archive-digests.txt` |
| wolf 0.2.20 exists and the book has not taken it | `git rev-parse v0.2.20^{commit}`; `wolf-toolchain.toml` | `cdde128a30999652c9d70189664226b766a206f0` tagged (r25); the book pins 0.2.19 — so "0.2.20 at the next bs pin bump" is exact |
| lupin 0.1.43 exists | `git rev-parse v0.1.43^{commit}` | `6d6cde553ba980527dcbebd4dbe81d63f898d658` |
| wolf-lang#497 | `gh issue view 497` | OPEN; every wolfgang lane `unsupported`, lupin 0.1.42 runs it; the refusal is `crates/wolf_sema/src/check.rs:10634` at trunk `cdde128a`: ``"`match` over a fallible value (unwrap with `?` or bind the error with `else |err|`)"`` |
| ruling #21 | `git show origin/trunk:sprints/STATUS.md` (wolf) | RULED 2026-10-02, yes as proposed; clause `[type.row.match]`; lanes s197, is67, bs57 |
| s197 and is67 | `gh api repos/wolffe-lang/{wolf-lang,wolf-interp}/branches/…` | **neither branch exists on its remote** at 23:30Z (wolf-lang has `s190`–`s196`; wolf-interp `is61`–`is65`); open PRs are #505 (s195) and wolf-interp#167 (is65). "In flight" means not yet pushed |
| `[type.row.operand]` at trunk | `git show origin/trunk:spec/10-types.md` | "`?`, `else` and a `match` are the whole of how a row is handled" — the sentence #497 found ambiguous; `[type.row.else]` (s191) sits beside it |
| E0801's catalogue entry | `git show origin/trunk:docs/diagnostics.md` | "names concrete values that slip past every arm"; fixture `pattern_diagnostics__e0801_row_missing_tag.snap` already exists for `else |e| match e` |
| chapter 6 today | `grep '^## 6\.' book/ch06.md`; `ls principles/exercises/ch06` | §6.1–§6.5; exercises 6-1…6-14 (13 `.lu`, 6-8 is design); the next free numbers are **§6.6** and **6-15** |
| §6.2's sentence to revise | `book/ch06.md:224` | "A caller of a fallible function has exactly three things it can want … Wolf spells them with one character, one keyword, and one keyword with a binding." — three forms of two constructs; `match` is named only on the bound error |
| the pending manifest's twin rule | `xtask/src/verify.rs:41-60` | item 4 requires every `samples-pending.toml` id to be `principles/exercises/<id>.lu` and (for `ex` ids) to appear in `EXERCISES-PENDING.md`; an in-chapter id (`book/ch06/…`) fails it today |
| `diagnostic,from(id)` | `xtask/src/samples.rs:374-388` | a block whose sample captured no diagnostic is a hard failure ("no fail() sample with that id ran"); the loop does not consult the pending set |
| console replay | `xtask/src/console.rs:437` | `check()` takes no pending set; a block beside a pending sample is replayed against the pin like any other |
| the gate at bs55's head | `docs/audit/bs55-evidence/gate-summary.txt` | 506 samples (249 roots + 16 members, 257 book blocks), 503 passed, 3 pending, 0 failed, 0 flips; 481 of 501 console blocks replayed; corpus 265; index 344, tier totals "8 pending"; backmatter 295 published / 330 on file |
| a printed pending exercise has a precedent | `principles/exercises/ch07/EXERCISES.md` 7-5 | the stem carries "static verdict pending — blocker: …; owner: …", the solution shows lupin's real transcript and names the compiler's `unsupported`; `EXERCISES-INDEX.md:222` tiers it `pending` |
| a held section has the other precedent | `samples-pending.toml` header, `EXERCISES-PENDING.md` | §13.1 stayed vacant until `par` landed and "was written in the same commit range" as its rows left; §17.3 is still held. This lane departs from that precedent on the orchestrator's word (drift 2) |
| bs56 | wave-52 | in flight on `wolf-boot.js` and the contents gate; `theme/` and `tests/contents/` are its |
| kasumi | `df -h /home`; `which typst` | 97% (33 G free), 16 cores, load 1.1; `typst` absent, so the PDF render is CI's |
| wolf-book#45, #58, #60 | `gh issue list` | open (the three standing pending rows' owners) |

**Drift, reported not absorbed.**

1. **The compiler's half has not been pushed.** Neither `s197` nor
   `is67` exists on its remote, so the exact diagnostic text and the
   branch replay the brief asks for cannot be taken at authoring time.
   The lane writes everything else first — the section, the three
   programs with their predicted bytes, the manifest rows, the
   exercise, the §6.2 revision, the rig change — and polls the remote
   in a printing loop; the snapshot and the blocker's quoted text are
   filled from s197's build when it exists, and the PR says which
   commit of `s197` the bytes came from. If s197 never lands in this
   lane's life, the rows stay with the predicted text marked as such
   and the PR says so.
2. **The brief asks for a printed section whose samples the pin
   refuses, and the book's own precedent is to hold the section.**
   §13.1 was vacant until `par` landed; TONE.md's tense discipline
   forbids "not yet" on the page; TWO-MACHINES.md §7 says the book does
   not print a program it has not executed. The lane follows the brief,
   because the ruling is made and both machine lanes are launched, and
   makes the hold structural instead of textual: the three samples are
   pending rows (report-only, a pass is a FLIP), the page carries no
   deferral prose, and **the PR must not merge before the bs pin bump
   whose compiler serves `[type.row.match]`** — merged earlier, §6.6
   would hand a reader a program their tool refuses. The PR body and
   this file say so in those words; the merge gate is the orchestrator's.
3. **The pending manifest cannot name an in-chapter sample today.**
   `verify-docs` item 4 looks for `principles/exercises/<id>.lu`. The
   samples runner itself is id-agnostic (it already fails a row that
   names no sample), so the fix is in `verify.rs` alone: a `book/` id
   is an extracted chapter sample, checked for existence by the samples
   lane, and the `.lu` rule stays for exercise ids. Likewise a
   `diagnostic,from(id)` whose id is pending has no diagnostic to
   compare yet, which today is a hard failure; it becomes a named
   PENDING line, compared the day the row leaves. Both are small,
   both get a self-test, and both are the rig's debt to the brief, not
   the brief's to the rig.
4. **lupin 0.1.42 may already run the worked sample.** #497 measured
   `none => -1, v => v` running on lupin; a payload arm and a literal
   value arm over a raw row are unmeasured. If lupin prints the
   predicted bytes, the chapter's `$ lupin` console block replays green
   at the pin while the `run(…)` fence waits on the compiler — honest,
   and the console lane needs no change. If it does not, the console
   block is spelled `from(…)` and the console lane learns the same
   pending line as the diagnostic one. Measured in §3's table, not
   assumed.
5. **Trunk moved between the brief and the first commit.** bs56
   merged at 03:31Z (`33377456…`), so the base is that commit, not the
   brief's `bd3484ed`. bs56's files (`theme/`, `tests/contents/`) are
   now trunk's and this lane still does not touch them.

## 3. Prediction, committed before measuring

Before any archive is downloaded and before any probe runs:

**At the pin (wolf 0.2.19 `9f3873d8…`, lupin 0.1.42 `9856335a…`), the
three new programs:**

| program | wolf 0.2.19 | lupin 0.1.42 |
|---|---|---|
| `book/ch06/part-rowmatch` (worked: two row arms, `0`, `n`) | `unsupported` — the `check.rs` text above, verdict not `pass`, exit 4 under `wolf run` | **runs, exit 0**, and prints the four lines below (hypothesis: lupin dispatches a raw row to the tag arms and the success value to the value arms; a wrong hypothesis here is drift 4's second branch) |
| `book/ch06/s7` (refusal: `no_comma` missing) | `unsupported` — same text; **not** E0801, because sema refuses the shape before coverage runs | runs; `cents("tip 100")` is `no_comma`, no arm takes it — lupin reports a no-arm error on stderr and exits nonzero (exit 3 or 4; the kind is not predicted) |
| `ch06/ex6-15` (`probe`'s open row, `Io(code)`, `v`, `_`) | `unsupported` — same text | **runs, exit 0, prints `7 -4 -99`** |

The worked sample's bytes, predicted: `340 cents` / `nothing owed` /
`` `lots` at byte 7 is not a number `` / `no comma`.

**The gate at head on the pin:** 509 samples (250 corpus roots + 16
members, 259 book blocks); 503 passed; **6 pending** (the three
standing rows plus `book/ch06/part-rowmatch`, `book/ch06/s7`,
`ch06/ex6-15`); 0 failed; 0 flips. Console blocks 503 declared, 483
replayed if drift 4's first branch holds (481 + the worked sample's
`$ lupin` block + 6-15's solution transcript), 481 otherwise with two
named PENDING lines. `verify-docs`: corpus count 266; index 345 total,
tier totals "9 pending"; ch06 — 15 exercises. `backmatter --check`: 296
published / 331 on file after regeneration. `ledger --check`: 133 open
(bs57's new row, filed against wolf-lang#497). Self-test 15/15 (13 + the
two new planted breakages).

**On s197's branch build:** `part-rowmatch` passes with the four lines
on native and under `conform-run --checked`; `s7` is `fail(E0801)` and
the diagnostic names `no_comma` by name (never the alias, never
`NotANumber`); `ex6-15` prints `7 -4 -99`. The one open question the
build decides: whether `v => v` followed by `_ => …` over an open row
is accepted silently or warns E0802 on the value half; the exercise's
text is written for the first and re-worded from the measurement if the
second.

## Items
1. **The contract** (this file), first commit.
2. **The section**: §6.6 on `book/ch06.md`, placed after §6.2 (the
   number is permanent, the position is where the reader needs it;
   how-to-read's rule); `part(rowmatch)` for the worked sample, a plain
   `fail(E0801)` fence for the refusal with its `diagnostic,from`
   block; §6.2's sentence revised; TOC.md's §6 list and the chapter's
   ledger row; CHANGELOG.
3. **The rig**: `verify.rs` admits `book/` ids in the pending manifest;
   `samples.rs` reports a pending `diagnostic,from` by name; one
   planted breakage each in the self-test.
4. **The rows**: `samples-pending.toml` ×3 with the expected bytes
   quoted; `EXERCISES-PENDING.md` (6-15 and the corpus count);
   `EXERCISES-INDEX.md` (6-15, totals); `principles/exercises/ch06/`
   (`ex6-15.lu`, the solution in `EXERCISES.md`); `solutions.md`
   regenerated on kasumi.
5. **The measurements**: the pin's answers to the three programs
   (archives by digest), then the gate at head on the pin, then the
   render (web + markdown on kasumi; PDF in CI).
6. **s197**: the branch build when it exists; the three programs under
   `wolf run`, `conform-run --json --checked` and the native lane; the
   snapshot blessed from that run and the blocker text quoted from it.

## 4. Evidence index (required)
Archive digests and member hashes by name
(`docs/audit/bs57-evidence/archive-digests.txt`, `members.txt`); the
pin's probe transcripts (`probes-pin.txt`); the gate logs on kasumi
(`~/lanes/bs57/logs/`) with the count lines copied to
`bs57-evidence/gate-summary.txt`; s197's build sha and the three
transcripts (`probes-s197.txt`); the render log line; CI run ids on
three hosts at head.

## 5. Done-when
Branch `bs57` on origin; PR open, **unmerged, and marked not to merge
before the pin that serves the form**; CI green on all jobs at the head
sha (`gh run view`); five sections in the PR body with commit-hash
bullets and a test checklist; §2 drift reported; §3 precedes every
measurement; the three pending rows carry s197's bytes or say plainly
that they carry the prediction; kasumi trees pruned once the evidence
is written; no orphans; the worktree removed after the report.
