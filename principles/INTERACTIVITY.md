# INTERACTIVITY.md — the doctrine and the catalog

Interactivity in this book is rationed. A widget appears only where wolf
is novel or the concept is genuinely abstract — where manipulating a
model teaches something that a program listing and its output cannot.
Everything else is prose, code, and real tool output, which are already
interactive in the way that matters: the reader can run them.

The exemplar is *Dear Computer* (Chris Johnson), mirrored at
`wolf/refs/repos/dear-computer` (paper-trail law; the mirror line is in
`wolf/refs/fetch.sh`). §4 catalogs its interaction patterns widget by
widget, citing the files on disk. §2 sketches the wolf widgets; §3 fixes
the implementation posture.

Cross-references: widgets obey DESIGN.md's page rules (no chrome, color
for meaning only); prediction-first exercises in EXERCISES.md are the
non-interactive form of the same pedagogy; the sass budget (TONE.md §2)
applies to widget copy, which is mostly labels and should stay deadpan.

---

## 1. The doctrine

- **Only where wolf is novel or the concept is abstract.** The candidate
  list is §2, and it is short on purpose. Chapter 4 does not need a
  slider to teach `defer`.
- **Predict before observe.** Every widget asks the reader to commit to
  a prediction before it reveals behavior — the same contract as the
  comprehension exercises. A widget that only animates is decoration
  and gets cut.
- **Enhancement, never load-bearing.** A reader with no JS loses nothing
  essential: every widget has a static fallback (a figure or table
  carrying the same example), and the surrounding prose teaches the
  concept completely. The PDF shows the fallback.
- **No server.** Widgets are self-contained client-side modules; the
  book works from a file:// URL. (This is a deliberate inversion of the
  exemplar — see §4.6.)

## 2. The wolf widgets

Five candidates, one per novel concept cluster. Each sketch fixes the
concept, the manipulanda, the predict/observe contract, and the print
fallback. bs00 builds none of them; the chapter sprints build them when
their chapter lands, against this contract.

### 2.1 Region-lifetime visualizer (chapter 8)

- **Concept:** a region's life — open, allocations accumulating,
  suspend/reopen, `freeze`, and the single wholesale free — as one
  timeline, not a sequence of prose assertions.
- **Manipulanda:** a 15-line program beside a timeline. The reader
  scrubs a program counter through the code; allocations appear as
  marks on the region's track; opening a second region adds a second
  track. A `freeze` control flips the track's state; after it, mutation
  attempts on the timeline show the rejection.
- **Predict/observe:** at a chosen line, the reader marks which
  allocations they believe are live, then scrubs; the widget shows the
  actual liveness and, at region end, all marks dying in one event —
  the point of the chapter, seen as one motion.
- **Print fallback:** the same program with a static timeline figure,
  three program counters annotated; the caption asks the same
  prediction question and the solutions page answers it.

### 2.2 Move/exclusivity stepper (chapter 7)

- **Concept:** the E1001/E1002 dance — field-granular moves and the
  prefix-overlap rule for `mut` paths.
- **Manipulanda:** the ownership tree of a small struct beside its
  program. Stepping a statement grays out the moved path (and only that
  path — `p.tail` stays live when `p.lead` moves). A call builder lets
  the reader pick two paths to pass `mut` and asks for a verdict.
- **Predict/observe:** the reader picks the line they believe traps
  (or the path pair they believe is rejected), then steps; the widget
  answers with the real diagnostic text — E1001 or E1002 verbatim, the
  compiler's voice unchanged (TONE.md §1, the two voices).
- **Print fallback:** exercise 3-2's differential transcripts, which
  already carry this concept, plus a two-column legal/illegal path-pair
  table.

### 2.3 Schedule explorer (chapter 17)

- **Concept:** a schedule is a seed — deterministic exploration and
  replay of interleavings, the is07 story.
- **Manipulanda:** two small tasks with three operations each; a seed
  field and a step button. Changing the seed reorders the interleaving;
  a "find failing" control walks seeds until the assertion breaks, then
  a replay control reruns that seed, identically, as many times as the
  reader likes.
- **Predict/observe:** the reader writes down the output they expect
  for seed 1 and seed 2 before stepping; the widget shows both, then
  shows the failing seed reproducing on demand — the pitch of
  `--schedules`/`--replay` experienced rather than claimed.
- **Print fallback:** two schedule tables from real `--schedules` runs,
  seeds printed, one passing and one failing.

### 2.4 Row-widening visualizer (chapter 6)

- **Concept:** error rows composing by union through a call chain — the
  part of `!T` rows that is genuinely novel to every arrival.
- **Manipulanda:** a three-function call chain drawn as a column;
  clicking any call edge shows the row crossing it. The reader can add
  a variant (`TooLong`) to the leaf function and watch which upstream
  signatures widen and which handler match arms fall behind.
- **Predict/observe:** before adding the variant, the reader marks
  which signatures they believe must change; the widget shows the
  widening and the exhaustiveness error at the handler that fell
  behind — exercise 6-3's finding, made visible.
- **Print fallback:** exercise 6-3's before/after signature diff.

### 2.5 Comptime sandbox boundary (chapter 18)

- **Concept:** the comptime/runtime boundary and what the sandbox
  admits — pure computation and hashed inputs in, ambient effects out.
- **Manipulanda:** a two-region canvas (comptime | runtime) and a set
  of expression tiles: arithmetic, a type computed from a type, a file
  read, a clock read, a network call. The reader drags each tile across
  the boundary.
- **Predict/observe:** the reader sorts all tiles first, then checks;
  each verdict names its reason in one line (hashed input; pure;
  refused — cache-defeating; refused — audit story), tracing each
  refusal to the rationale chapter 18 teaches.
- **Print fallback:** the same tiles as a two-column table with the
  verdict column blank; the filled table is on the solutions page.

## 3. Implementation posture

- **No framework.** Each widget is one small self-contained JS module,
  loaded only on the page that uses it; a page with no widget loads no
  widget code. No bundler, no runtime dependency shared across widgets
  beyond a ~50-line mounting convention.
- **Progressive:** the fallback figure is in the markup; the module
  replaces it when it loads. No JS, slow JS, or a failed load all leave
  the fallback — which is the print artifact anyway.
- **State is local.** No login, no server, no telemetry, no
  localStorage requirement. A widget that wants to check a prediction
  checks it client-side.
- **Widget copy obeys the register.** Labels are deadpan; the one wry
  sentence per page (TONE.md §2), if spent, is spent in prose.

## 4. Appendix — the *Dear Computer* catalog

Mirror: `wolf/refs/repos/dear-computer` — 143 pages across 17 chapter
directories, one shared runtime (`main.js`, 753 lines) and stylesheet
(`style.css`, 1381 lines). Its interaction machinery is **Asker**, a
server-graded question system: pages carry `.asker-frame` mounts
(**403 of them** across the mirror, roughly three per page) whose
question content is fetched at load from `asker.twodee.org` and graded
server-side (`main.js:1–2, 512–540`). The widget types below are
enumerated from `main.js`'s dispatch (`initializeAsker`,
`main.js:328–506`).

### 4.1 Self-check reveal (`main.js:368–384`)

A question with a hidden answer panel (`.asker-reveal-panel`,
`style.css:821`); the button toggles it. **Concept:** any. **The
manipulation:** commit, then reveal. **Why it beats prose:** a printed
answer sits next to its question and gets read first; a reveal makes
the reader produce an answer to have something to check. This is the
cheapest widget in the catalog and the one our EXERCISES.md solutions
policy (collapsed `<details>`) borrows directly — without the login
gate.

### 4.2 Form questions: radio, checkbox, blanks, selects (`main.js:463–481`)

Standard form controls graded server-side; wrong choices get flagged
in place (`.radio-choice.wrong` and kin, `style.css:843`), a status
box turns right/wrong (`style.css:831–841`), and any edit clears the
verdict (`onTweak`, `main.js:399–403`). **Concept:** discrimination
between near-misses. **Why it beats prose:** the distractors encode
real misconceptions, and the per-choice verdict tells the reader which
misconception they hold — prose can list pitfalls but cannot tell you
yours. Note the checkbox/jigsaw grader deliberately withholds itemized
results ("That hands out the answer", `main.js:472–476`) — restraint
in feedback is itself a design decision.

### 4.3 Jigsaw (`main.js:21–326`)

The signature widget: drag-and-drop program assembly. A bank of code
fragments (`.asker-jigsaw-bank-piece`), a row-structured workbench,
ghost pieces under the pointer with live insertion previews
(`initializeGhost`, `main.js:21–248`), click-to-append, copy and reset
(`main.js:258–281`), pre-seeded rows for partial programs
(`question.priors`, `main.js:293–325`). **Concept:** statement
ordering and program structure. **Why it beats prose:** ordering
constraints are experienced as placement decisions rather than read as
assertions, and the fragment bank bounds the search space so the
exercise stays about structure, not recall or typing. This is the
ancestor of our schedule explorer (§2.3): interleaving tasks is a
jigsaw whose rows are time.

### 4.4 Code widget (`main.js:333–356, 386–493`)

An embedded Ace editor per question (language modes mapped at
`main.js:334–343`), a Run button that submits to the server, and
stdout/stderr consoles rendered back inline (`.asker-code-console`,
`style.css:783`; response handling `main.js:456–462`). **Concept:**
anything with executable behavior. **Why it beats prose:** the
chapter's claim becomes checkable without leaving the page. It is also
the catalog's cautionary entry: execution lives on a server, so the
widget dies with the server and never worked from the mirror. Wolf's
equivalent stays out of scope until it can run client-side; until
then, lupin transcripts are the executable claim.

### 4.5 Interleaved placement — the pattern above the widgets

The placement discipline matters more than any single widget:
`managing-memory/lifetimes.html` threads three askers into one
argument about Rust lifetimes (lines 123, 145, 147), each one
line-adjacent to the prose claim it tests; `types/`,
`expressions/`, and `polymorphism-revisited/` pages carry 6–7 mounts
each; the exam directories (`rust-exams/`, `ruby-exams/`,
`haskell-exams/`) batch eight to a page for review. The widget is the
running example, not a sidebar attraction. Our §2 sketches inherit
this: each belongs at a precise point in its chapter's argument.

### 4.6 Chrome, and what we decline to inherit

Supporting machinery on every page: settings panel — dark mode, wide
code, wrap code (`main.js:546–618`); slide-out chapter menu
(`main.js:620–640`); per-block copy buttons (`main.js:674–686`);
Prism highlighting with line-highlight, MathJax, and Ace, all from
CDNs (`index.html:6–39`, per-page heads).

Declined, with reasons. **Server dependence:** Asker requires a Canvas
login; logged-out readers see disabled submit buttons
(`warnAuthentication`, `main.js:508–510`) — the interactivity is
load-bearing for a course and dead for everyone else, the exact
failure our §1 posture forbids. **Telemetry:** the runtime logs page
loads, unloads, and every copied selection to the server
(`main.js:642–672, 711–750`); a book that teaches auditing
dependencies will not itself phone home. **CDN runtime:** our widgets
and fonts are self-hosted (DESIGN.md); the mirror renders today
precisely because static content outlives its CDNs only when it
carries its own weight. **Dark mode** is deferred by DESIGN.md §1, so
the settings panel goes with it.

What we take: predict-then-check as the universal contract, placement
inside the argument, the reveal pattern for solutions, restraint in
itemized feedback, and the jigsaw's insight that manipulating
structure teaches structure.
