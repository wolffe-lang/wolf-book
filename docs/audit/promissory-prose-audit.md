# Promissory-prose audit — Part 1 (ch01–ch06 + front matter)

Ordered quality gate. Standard, verbatim: **"We want the text to reflect
the ultimate product, not a promise that we may not deliver on."**

Scope: every instance of deferral/promissory language in the shipped
prose; the guidance clauses that caused it; the M1-timing question.
This document proposes; the integrator routes. No chapter or TONE.md
edit is made here.

Severity key:

- **COSMETIC** — one sentence to cut later; nothing else moves.
- **STRUCTURAL** — the section's shape exists because of a gap; the
  section survives, but paragraphs/samples get replaced when the
  surface lands.
- **FOUNDATIONAL** — the chapter teaches a workflow that is obsolete at
  M1 (or at s37–s38 for std surface); the section is rewritten, not
  patched.

---

## 1. Catalog

28 findings: **7 FOUNDATIONAL rows (4 section-level units) · 17
STRUCTURAL · 4 COSMETIC.**

### Front matter

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| F1 | `book/front/how-to-read.md:9–11` | "Stub — this page's prose lands with the chapter sprints. The promise it will carry: …" | STRUCTURAL — a shipped placeholder page whose entire content is a promise. |
| F2 | `book/front/notation.md:33–34` | "replay waits on wolf-interp's REPL (is08) and is counted, not skipped silently" | COSMETIC — internal sprint id (is08) in reader-facing text. |
| F3 | `book/front/notation.md:87–89` | "`4` is `unsupported`, the young implementation declining work it will do later." | STRUCTURAL — readers are told to *memorize* a scaffold-era verdict as part of the exit-code contract. At the ultimate product, exit 4 is at most a footnote. |

### Chapter 1 — Hello, Wolf

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 1 | `book/ch01.md:95–100` | "It is also not true yet, and this book does not pretend otherwise. Today the toolchain is two programs … It cannot yet produce an executable." | FOUNDATIONAL — §1.2's opening move is a retraction of §1.2's own first paragraph. |
| 2 | `book/ch01.md:114–117` | Console block: "`wolf: pre-alpha scaffold; \`wolf build\|run\` lands at sprint s31`" | FOUNDATIONAL — a tool-scaffold message, with an internal sprint number, taught as product output. The single worst offender in Part 1. |
| 3 | `book/ch01.md:119–124` | "Two tools for now, then; one when that line comes true … the promise stands as a promise." | FOUNDATIONAL — prose that names itself a promise. |
| 4 | `book/ch01.md:126–139` | Install = `cargo build` from two source repos; "Given a Rust toolchain and the two repositories …" | FOUNDATIONAL — the install workflow is the bootstrap dev workflow, obsolete the day there is a release channel. |
| 5 | `book/ch01.md:151` | "For you, today, the useful consequence is that wolf programs run." | COSMETIC — "today" framing. |
| 6 | `book/ch01.md:156–159` | Exercise 1-7 — run both `--version` lines, "why a book would print them at all." | STRUCTURAL — an exercise whose subject is the two-tool scaffold. |
| 7 | `book/ch01.md:186–189` | "That machinery is not in the toolchain yet, and this book prints no sample it cannot run; when it lands, this section grows the four lines it takes." | STRUCTURAL — the chapter's own ledger admits the script-mode thesis is "one third demonstrated and two thirds promised" (s53 frontmatter deps). |
| 8 | `book/ch01.md:289–291` | "Having no artifact is what makes a script convenient; producing one is what a compiler is for, and this section is named after the day wolf does both." | FOUNDATIONAL — §1.5 is named for a capability that does not exist. |
| 9 | `book/ch01.md:364–371` | "**The implementation has not got there yet (exit 4).** A young toolchain also declines work it will do later … you will see it a few times in chapter 2." | STRUCTURAL — a whole verdict class exists in the pedagogy because the pin is incomplete; prose pre-apologizes for chapter 2. |
| 10 | `book/ch01.md:373–378` | "What is missing from this section is `wolf build`. There is no compiled artifact today, no `--release`, no binary to hand somebody. When the compiler's driver lands … this section gains the artifact it is named for." | FOUNDATIONAL — half of §1.5 is a deferral paragraph. |

### Chapter 2 — Strings, honestly

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 11 | `book/ch02.md:87–99` | "One honest gap. A precision spec … is not implemented in the pinned interpreter … So every table in this book counts in whole numbers … When the spec lands in the implementation, that exercise gets its float column." | STRUCTURAL — a *book-wide* formatting constraint (integer tables everywhere) shaped by one missing feature. |
| 12 | `book/ch02.md:121–123` | "Two you will reach for and not find yet are `split` and `find` — the interpreter's string surface is a documented subset … §2.4 shows the scan that does the same work today." | STRUCTURAL. |
| 13 | `book/ch02.md:326–331` | "Splitting on a separator you choose is the method §2.1 does not have, so until it lands the spelling is a scan." | STRUCTURAL — the comma-scan workaround. Note the blast radius: `comma()` and the byte-scan idiom recur in ch03, ch04 (`comma`/`name_of`/`cents_of`), ch05, and ch06 (§6.1, §6.4). The scan has real pedagogical value (byte-honest slicing) and can *survive* as pedagogy — but today it is taught as the *only* spelling, and one sentence per site says so. |

### Chapter 3 — Values and expressions

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 14 | `book/ch03.md:556–566` | "`unsupported: no \`match\` arm applied; exhaustiveness is the type checker's` … That is the young toolchain being honest about the division of labor." | STRUCTURAL (mild) — a teaching beat built on a scaffold refusal. The division-of-labor point (totality is static; the interpreter does not guess) is permanent and salvageable; the "young toolchain" apology and exit-4 framing are not. |

Otherwise ch03 is clean: §3.1–§3.3 and the rest of §3.4 are pure
language semantics with real diagnostics from both tools.

### Chapter 4 — Functions

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 15 | `book/ch04.md:210–215` | "Two honest limits at this pin … a closure that captures a `var` captures its value, not the place — a shape this chapter avoids, and one the audit ledger below files rather than teaches." | STRUCTURAL — unsettled semantics (ledger: ba:blocker, no spec clause) dictate what the chapter may not show. |
| 16 | `book/ch04.md:311–315` | "This is where files would close themselves, if the pinned toolchain had files. It does not yet, so the samples above print instead of closing." | STRUCTURAL — `defer` taught entirely on `print` because there is no resource to release. |
| 17 | `book/ch04.md:439–441` | "Exercises 4-3 and 4-4 … are set in §7.4 instead, where the one word they need — a call-site `mut` — has been taught." | COSMETIC — this is *pedagogical* sequencing mandated by bs02's non-targets, not a toolchain gap; listed for completeness because it reads as a deferral. |

### Chapter 5 — Collections and generics without fear

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 18 | `book/ch05.md:179–204` | "That leaves `Set`, and here the book has to be honest: the pinned interpreter has not got one." + `run(exit=4)` refusal sample + "the right one is a feature away." | STRUCTURAL — the `Set` section *is* a refusal demo; `seen_at`/linear scan substitutes throughout the chapter. |
| 19 | `book/ch05.md:220–256` | "None of the combinators exist yet — not `sorted_by`, not `take`, not `map`, `filter`, or `sum`. … Instead of writing the chain and then desugaring it for understanding, we write the desugaring because it runs." | FOUNDATIONAL — §5.2's entire teaching order is inverted by the gap. The contract wanted chain-first ("used *before* it is explained — it reads like Python"); the section ships a refused chain (`run(exit=4)`) plus a 15-line hand-rolled ranking loop as the runnable truth. |
| 20 | `book/ch05.md:317–323` | "**Exercise 5-8** *(comprehension · pending — blocker: `sorted_by` / `take` absent from the interpreter's std subset; owner: s37-core-types)*" | STRUCTURAL — an exercise shipped with a blocker annotation and an internal sprint owner in reader-facing text. |
| 21 | `book/ch05.md:423–426` | Diagnostic note "no trait covers this operator yet (operator traits are a later sprint)" amplified by prose: "a comparison trait that lands with the trait system … the compiler being honest about its own schedule." | STRUCTURAL — the compiler's scaffold note is quoted (unavoidable, verbatim rule) and then *endorsed* rather than contained. |
| 22 | `book/ch05.md:499–529` | "`()` is what the pinned interpreter yields for a key that is not there … That is not the design — a missing key wants an answer the type system can see … but it is the truth at this pin, and the book prints the truth at this pin." | FOUNDATIONAL — §5.4 teaches a `Map` discipline (probe with a parallel `List`, branch insert-vs-update) that is dead code the day `m[k] += 1` works. The ledger itself calls this "the single highest-yield std gap the book has hit." The discipline is then *practiced* by the ch05 opener and the ch06 capstone. |

### Chapter 6 — Errors are values

| # | Where | Quote | Severity |
|---|-------|-------|----------|
| 23 | `book/ch06.md:317–323` | "at this pin the barest one in the toolchain: no clause tag, no span, and no trace … Error return traces are a promised part of the debug profile and they do not exist yet; when they land, this page grows the run that shows one." | STRUCTURAL — the "fifth verdict" (exit 1) is taught bare, with a promise where the trace run should be. |
| 24 | `book/ch06.md:608–615, 624–641, 653–667` | Capstone `count`/`top`: `index_of` containment scan + `if at < 0 { order.push(w); tally[w] = 1 } else { tally[w] += 1 }` + the §5.2 ranking loop. | STRUCTURAL — the known case: the capstone is reshaped around `Map`'s missing absent-key story and the missing combinators. (The `order` list survives on its own merits — first-seen ordering — but its second job as containment oracle does not.) |
| 25 | `book/ch06.md:706–712` | "The text is embedded because the pinned interpreter has no arguments and no files; the day `main` takes an argument list, that `if` is where it arrives." | STRUCTURAL — the capstone cannot be the CLI program reports/05 §Candidate A specifies; the usage path is exercised by an empty-string hack. |
| 26 | `book/ch06.md:714–722` | "**The promise this loop owes you** … Hold us to it. If parallelizing this program turns out to need a rewrite, the claim was false and the book will say so on that page." | COSMETIC — contract-mandated (bs02 target 4), falsifiable, about the *language* not the toolchain, with a named check (§13.1, bs06 obligation). A different class from the rest; flagged for a deliberate keep/cut decision rather than assumed bad. |

One reverse finding, recorded because it matters to the recommendation:
`book/ch06.md:219–221` teaches "the exhaustiveness rules from §3.4
apply to the row," while the chapter's own ledger records that nothing
about rows is statically checked at this pin ("enforced by neither tool
at this pin"). This is the one place Part 1 already writes in
ultimate-product tense — and it reads exactly the way the ordered
standard wants the whole book to read. It also violates the book's
executed-truth doctrine, which is the tension §3 below resolves.

---

## 2. The guidance that caused it

The first fact the audit turned up: **the contracts did not intend
pre-M1 chapters.** bs01 and bs02 both gate "after **M1** (s31) +
s37–s38." The chapters shipped anyway, roughly 2–3 sprints early,
through a loophole — and once drafting was underway pre-M1, a cluster
of "honesty" clauses did exactly what they say, and produced the
catalog above. Clauses to amend, by file:

### `principles/TONE.md`

1. **§1, "No feature is promised before its campaign shipped.
   Deferrals are stated plainly ('v1 does not do this')."**
   (TONE.md:50–51.) The second sentence is the deferral license. It
   conflates two different deferrals: *language-scope* deferrals ("v1
   has no macros" — permanent, belongs in the book) and *toolchain-
   completeness* deferrals ("split does not exist yet" — transient,
   does not). Every "not yet / when it lands / at this pin" sentence in
   the catalog is this clause doing what it was told.
2. **§1, "Honest failure output is part of the product."**
   (TONE.md:31–32.) Written for traps and diagnostics; read in
   practice to cover `unsupported` scaffold refusals and the
   `pre-alpha scaffold` banner, which are *absence* of product, not
   product. Needs a carve-out.

### `wolf/sprints/book/00-scaffolding/bs00-toolchain-and-voice.md`

3. **Target 2, the pin doctrine** ("no code appears in the book that CI
   did not execute," samples run "against the **pinned** wolf
   toolchain"). The invariant is right and stays — but it has no floor
   on what the pin must *contain*. Pre-M1 it inverts: since only what
   runs may be printed, every gap becomes a printed refusal or a
   taught workaround. The missing clause is a surface floor per
   chapter.
4. **Target 3 hard rule** ("no feature promised before its campaign
   shipped") — same amendment as clause 1; it is the same sentence,
   duplicated.

### `wolf/sprints/book/01-foundations/bs01-part1-getting-started.md`

5. **Gate clause: "Draft may begin early against nightlies; *green CI*
   is what gates merge."** (bs01:6–7; bs02 inherits it.) This is the
   loophole. CI was green — against a pin where the chapter's surface
   does not exist — so the merge gate passed while the M1+s37 gate was
   still 2–3 sprints out. "Green CI" must mean green *at a pin that
   implements the chapter's declared surface*.
6. **Target 1: "the book never asks the reader to install a second
   tool, and says so as a promise."** (bs01:32–34.) The contract
   explicitly mandates promissory prose; ch01:119–124 is its direct
   output ("the promise stands as a promise").
7. **Target 3: "`ba:papercut` — teachable with an apology in the
   prose."** (bs01:70.) The apology license. Under the ordered
   standard, a surface that cannot be taught without the prose
   apologizing is not teachable yet; the papercut is filed and the
   passage waits.

### `wolf/sprints/book/01-foundations/bs02-part1-language-core.md`

8. **Gate clause** — same wording as bs01's; same amendment.
9. **Non-targets: "If a sample can't be written honestly without them,
   that is a finding: file it and reshape the sample."** (bs02:87–89.)
   The reshape license, and the single most productive cause of
   STRUCTURAL findings: it is the sentence behind §5.1's Set-refusal,
   §5.2's inverted order, §5.4's Map discipline, and the capstone's
   `index_of` dance. The *filing* half is right (the audit ledgers are
   the best artifact of bc01). The *reshaping* half needs a condition:
   reshape only into a form the finished book would keep; otherwise
   hold the section.

Count: **9 clauses (7 distinct doctrines)** across TONE.md, bs00, bs01,
bs02. Not implicated: STYLE.md (verbatim-output and ≤25-line rules are
fine as-is), the template's unpublished audit-ledger block (correct and
load-bearing — it is where deferrals *should* live), and the pending
manifest / FLIP machinery (correct: it notices landings without
printing promises).

Root cause in one sentence: the index doctrine "chapters trail feature
campaigns; writing the chapter is the ergonomics audit" was run in
reverse — the chapters *led* the campaigns — and the honesty clauses,
designed to keep a post-M1 book truthful, dutifully documented the gap
between today's pin and the product instead.

---

## 3. The timing question

Milestone map used below: **M1 = s31** (`wolf build|run` complete; s29
in flight, s30 DWARF next). The **std surface** (split/find, format
precision, combinators, `Set`, `Map` absent-key, `main` args, fs) is
**s37–s38**, *after* M1. Error traces, trait bounds, row checking have
their own owners (debug profile, trait sprint, s15). So "revision at
M1" is really **two named passes**:

- **Pass A (at M1/s31):** tool-tour and run-workflow material — the
  install, `wolf build|run`, the artifact, the exit-code table, and
  the global question of which command fronts the console blocks
  (every `$ lupin file.lu` line in Part 1 is implicitly
  conform-run-as-the-only-door).
- **Pass B (at s37–s38):** std-surface material — strings, collections,
  the capstone's shape.

### Per-chapter verdict and revision cost

| Chapter | Stable core | M1-sensitive / gap-shaped | Est. stable | Pass | Cost |
|---------|-------------|---------------------------|------------|------|------|
| front (notation, how-to-read) | Dialect legend, exercise conventions | Exit-code table (F3), stub page (F1), is08 note (F2) | ~70% | A | LOW — table row + stub page + one clause |
| ch01 | §1.1 cold open, §1.3 exit-code/expression rules, §1.4 REPL | §1.2 total rewrite (install, one-binary story), §1.5 half-rewrite (gains `wolf build`, artifact, `--release`; exit-4 demoted), Ex 1-7, Python box (s53 — may outwait even Pass B) | ~55% | A (+s53) | **HIGH** — the only chapter where whole sections are replaced, plus global command re-fronting |
| ch02 | Literals, interpolation, multiline/raw, byte-honesty, views/cost model | Precision-spec para (11), split/find paras (12, 13); comma-scan re-framed from "the only spelling" to "the teaching spelling" | ~85% | B | MEDIUM — paragraphs, not sections; float tables optional |
| ch03 | Everything: let/var, moves, expressions, checked arithmetic, match | One paragraph in §3.4 (14) | ~95% | B | **LOW** — cut two sentences, reframe one beat |
| ch04 | Signatures, inference boundary, closures, defer semantics, borrow/move rule | Capture-limits para (15), files-would-close para (16) | ~90% | B | LOW-MEDIUM — §4.3 gains a real file sample; two paras cut |
| ch05 | List/Map/tuple mechanics, generics + E0501, indexing traps | §5.2 rewritten right-way-round (19), §5.4 Map section rewritten (22), §5.1 Set section becomes real (18), Ex 5-8 unblocked (20), trait note (21) | ~60% | B | **HIGH** — two section rewrites plus a real Set section |
| ch06 | The row, `?`/`else`/`else \|err\|`, errdefer, **§6.4 hardening narrative (untouched)** | Trace promise → real trace run (23), capstone tally + args (24, 25), exit-1 verdict documented | ~85% | B | MEDIUM — capstone simplifies (deletions, not rewrites); one page gains a run |

**Aggregate: ch03–ch06 cores are ~82% stable** (ch03 95, ch04 90, ch05
60, ch06 85); the instability is concentrated in exactly two places —
ch01's tool tour (Pass A) and ch05's std-surface sections (Pass B).
The receipt→hardening→wordcount narrative spine of Part 1 survives M1
and s37 completely intact; what changes is plumbing around it.

---

## 4. Proposed amendments

### 4.1 `principles/TONE.md` — new section (proposed insertion after §1)

> ## Tense discipline
>
> The book speaks in the present tense about what exists. Not what is
> scheduled, landing, or promised — what a reader with the pinned
> toolchain observes today, described as the product it is.
>
> The operative rule: **where the toolchain and the book disagree, the
> book waits or the toolchain hurries. There is no third option.** The
> third option — prose that teaches the gap — is what this section
> exists to forbid. Specifically:
>
> - **No deferral prose.** "Does not exist yet", "when X lands", "at
>   this pin", "for now", "a feature away", "the day wolf does both" —
>   none of these appear in reader-facing text. A chapter that needs
>   them is a chapter whose gate has not opened; it stays a draft.
> - **No sprint numbers, milestone names, or pin apparatus in the
>   reader's text.** s31, s37, is08, "the pinned interpreter" — these
>   are CI's vocabulary and the colophon's, never a section's. The
>   reader is told which toolchain version the book is true for
>   exactly once, in the colophon.
> - **Scaffold output is not product output.** The honest-failure rule
>   (§1) covers traps and diagnostics — the product's real voice. It
>   does not cover `unsupported` refusals or pre-alpha banners, which
>   are the *absence* of product. They are never taught, never made
>   into exercises, and never used as a section's load-bearing sample.
> - **No workaround taught as the way.** When the designed surface is
>   missing, the section that teaches it waits for the surface. A
>   longer spelling may be taught *beside* the designed one when it
>   earns its page on pedagogy alone (a byte-scan that teaches
>   slicing), and the test is: would this passage survive, unedited,
>   the day the feature lands? If not, it is deferral prose in
>   disguise.
> - **Language-scope statements are not deferrals.** "v1 has no
>   macros" is a fact about the product and belongs in the book,
>   present tense, no apology. The line between scope and schedule:
>   scope is a decision with a D-number; schedule is a sprint. The
>   book states decisions and never states schedules.
> - **Falsifiable cross-references within the book** ("in Part 3 this
>   loop parallelizes by changing one call") are permitted only when a
>   named later chapter is contractually bound to cash them, and are
>   budgeted like sass: rare, deliberate, and each one signed off in
>   review.
>
> Where drafting hits a gap, the finding goes to the audit ledger and
> the `book-audit` loop — at full volume, with the same honesty this
> section removes from the prose. The ledger is where "not yet" lives.
> The reader never sees it.

### 4.2 bs03+ contract amendments (proposed boilerplate, replacing the bs01/bs02 gate-and-honesty framing)

Proposed for bs03 and every subsequent chapter sprint; also proposed
retroactively as errata notes on bs01/bs02 so the doctrine has one
source:

> **Gate (replaces "draft early / green CI merges"):** A chapter
> *gates on its surface existing*: every construct, std call, tool
> command, and diagnostic the chapter teaches must be demonstrable at
> a single pinned toolchain, and green CI **at that pin** is what
> gates merge. Drafting early against nightlies is encouraged — the
> ergonomics audit is the point — but a draft whose surface is not at
> the pin stays out of `book/` (drafts live unpublished; ledgers file
> immediately). "Green CI" against a pin that refuses the chapter's
> subject matter satisfies nothing.
>
> **Tense discipline:** prose obeys TONE.md "Tense discipline" — no
> deferral language, no sprint numbers, no scaffold output taught, no
> workaround taught as the way. The reshape rule is amended: if a
> sample cannot be written without a missing surface, *file the
> finding and hold the sample* — reshape only into a form the
> finished book would keep.
>
> **Severity vocabulary:** `ba:papercut` is redefined as "teachable
> without comment in the prose; filed for c14." The former definition
> ("teachable with an apology in the prose") is retired: a surface
> that needs an apology is not teachable yet, and the section waits.
>
> **Named obligation — the Part-1 revision passes:** two passes are
> standing contract obligations, owned by the first book sprint whose
> gate follows the enabling milestone:
> - **rp-M1** (after s31): rewrite ch01 §1.2 and §1.5 around the real
>   `wolf build|run`; re-front the console blocks' command story
>   book-wide; demote exit 4 and document the failure-exit verdict;
>   replace the how-to-read stub; delete findings 1–10, F1–F3 of
>   `docs/audit/promissory-prose-audit.md`.
> - **rp-std** (after s37–s38): rewrite ch05 §5.2 chain-first and
>   §5.4's Map story; real `Set` section; unblock Ex 5-8; strings
>   gaps in ch02; capstone gains `main` args and sheds the
>   containment scan; ch06 gains the real error-trace run; delete
>   findings 11–25.
> Each pass closes by re-running this audit's grep set and reporting
> zero reader-facing hits.

### 4.3 One-line amendments to existing clauses

- TONE.md §1: replace "Deferrals are stated plainly ('v1 does not do
  this')" with "Scope is stated plainly ('v1 does not do this');
  schedule is never stated (see Tense discipline)."
- TONE.md §1 honest-failure rule: append "— traps and diagnostics,
  not scaffold refusals (see Tense discipline)."
- bs00 target 2: append "A chapter merges only at a pin implementing
  its surface; refusals are never load-bearing samples."
- bs01 target 1: strike "and says so as a promise"; the one-binary
  story is *demonstrated* at rp-M1 or not told.

---

## 5. Recommendation

**Continue with gated scope. Do not pause the track. Do not un-ship
Part 1; schedule rp-M1 and rp-std instead.**

Reasoning, from the counts:

1. **The damage is concentrated, not diffuse.** 4 foundational units
   in 28 findings, all in two places: ch01's tool tour and ch05's
   std-surface sections. ch03–ch06's cores are ~82% stable (ch03 at
   95%, ch06's hardening narrative — the best thing in Part 1 —
   untouched by either pass). If the number were 50%, pausing would be
   right; at 82% concentrated in known sections, pausing burns the
   stable majority to protect the unstable tenth.
2. **The audit loop is the track's highest-yield artifact and it only
   runs when chapters are being written.** Part 1's ledgers filed ~40
   findings including two genuine language-level blockers (bare-variant
   patterns bind instead of match; closure capture semantics
   unspecified). Pausing until M1 forfeits 2–3 sprints of that signal
   during exactly the window (s29–s31) when it is cheapest to act on.
3. **The fix that actually meets the ordered standard is the gate, not
   the pause.** bs01/bs02 already gated on M1+s37; the failure was the
   "green CI merges" loophole plus the apology/reshape licenses. Close
   those (amendments §4.2) and bs03+ *cannot* reproduce the problem:
   a chapter whose surface exists needs no deferral prose, and a
   chapter whose surface doesn't stays an unpublished draft with a
   loud ledger.
4. **Part 2's surface is mostly compiler-core, not std** — moves,
   `mut`, `take`, regions are exactly what wolf-lang has been building
   (mem/wir phases, E1001 already teaching-grade in ch03). bs03 likely
   gates open earlier than a std-heavy chapter would; each sprint
   verifies its own surface floor at pin before prose lands in
   `book/`. Where a Part-2 section's surface is missing (e.g.
   ch07/ex7-5's pending row), that *section* waits — gating is
   per-surface, not per-era.
5. Practical order: adopt the TONE amendment and bs03+ boilerplate
   now; run rp-M1 as a small named sprint immediately after s31
   (ch01 is one chapter; HIGH cost locally but ~a week's work);
   fold rp-std into the first post-s38 book sprint. Part 1's
   already-shipped deferral prose is quarantined by being enumerated
   here — every finding has a file:line and a pass that deletes it.

---

*Method note: catalog built from full reads of `book/front/*.md`,
`book/ch01–ch06.md`, `principles/TONE.md`, `STYLE.md`,
`samples-pending.toml`, and the bs00/bs01/bs02 contracts in
`wolf/sprints/book/`. The chapters' unpublished audit ledgers were used
as corroborating evidence (they are not reader-facing and are not
findings themselves — they are the mechanism working as designed).*
