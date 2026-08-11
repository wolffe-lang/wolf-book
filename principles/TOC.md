# TOC.md — the table of contents

The full structure of *The Wolf Book*, owned here. It is derived from the
chapter plans in the bs01–bs11 sprint files, but where a sprint file and
this document disagree, this document wins and the delta is recorded in
§Deltas for that sprint's future author. Each section carries a one-line
promise: what the reader can *do* after it that they could not before.

Numbering: parts are numbered 1–5; chapters run continuously 1–32;
sections are `N.M` and their numbers are the stable anchors of the web
edition (see DESIGN.md §Navigation).

---

## Front matter

- **How to read this book** — pick the on-ramp for your background
  (Python, Go, Rust, C), and learn the one promise: every sample in this
  book was executed by CI against the toolchain version printed in the
  colophon.
- **Notation** — read the code-block dialects (program, part, REPL
  transcript, console run, diagnostic) and the exercise numbering.

## Part 1 — Foundations

*The reader goes from nothing installed to writing real single-threaded
wolf, without hearing the word "lifetime."*

### Chapter 1 — Hello, Wolf
- 1.1 A program worth keeping — run a complete 15-line script that does
  something you would keep, before any installation ceremony.
- 1.2 Two implementations, one language — `wolf build` produces a
  binary, `wolf run` produces one and runs it, `lupin` interprets the
  same source; install both and watch them agree.
- 1.3 Scripts before projects — run `.lu` files directly with
  `wolf run`: no project, no venv, no lockfile to drift.
- 1.4 The REPL: a spec you can interrogate — start the interpreter's
  session, try expressions, and learn `:mem` exists before you need it.
- 1.5 What `run` was doing for you — meet `wolf build` and the shape of
  a compiled artifact.
- Exercises 1-1 … 1-3.

### Chapter 2 — Strings, honestly
- 2.1 Literals, methods, interpolation — write `"{x}"` in any literal
  and format anything with `"{total:>8.2}"`.
- 2.2 Multiline and raw — use `"""` dedented blocks and `r"…"` without
  escape archaeology.
- 2.3 Bytes, honestly — predict `"é".len`, slice by byte offset with
  checked ranges, and say why there is no `s[i]`.
- 2.4 Iterating meaning — walk `words()` and `lines()` and pick the
  iterator that spells its unit.
- 2.5 What the machine does — know what a slice costs (two words) and
  what an f-string compiles to.
- Exercises 2-1 … 2-4.

### Chapter 3 — Values and expressions
- 3.1 `let`, `var`, and handing values over — declare, rebind, and read
  "assignment hands the value over" without the deep story (deferred,
  explicitly, to chapter 7).
- 3.2 Everything is an expression — get values out of `if`, `match`, and
  blocks; live without a ternary.
- 3.3 Arithmetic that traps — predict overflow and division by zero in
  every build profile, and spell intentional wrapping.
- 3.4 `match`, exhaustively — write a match the compiler proves total.
- Exercises 3-1 … 3-5.

### Chapter 4 — Functions
- 4.1 Signatures are the contract — write signatures on items, nothing
  in bodies, and read a definition as its own documentation.
- 4.2 Functions as values — pass closures with `fn(a, b) expr`, compose
  them, return them.
- 4.3 `defer` — put cleanup next to acquisition and predict LIFO order.
- 4.4 Borrow by default — pass parameters with no sigils, return by
  move, and state the rule in one sentence.
- Exercises 4-1 … 4-4.

### Chapter 5 — Collections and generics without fear
- 5.1 `List`, `Map`, `Set`, tuples — build and index the workhorses.
- 5.2 The combinator style — chain `.pairs().sorted_by(…).take(n)` and
  then see what it desugars to.
- 5.3 Generics in square brackets — write `fn top[T](…)`, get errors at
  the definition site, and never type a turbofish.
- 5.4 Indexing that traps — predict `xs[10]` on a one-element list.
- Exercises 5-1 … 5-5.

### Chapter 6 — Errors are values
- 6.1 `!T` and the row — read `int ! {Empty, NotDigit(Bad)}` as the
  complete list of what can go wrong.
- 6.2 `?`, `else`, `else |err|` — propagate with one character, default
  with one keyword, handle with a match on the row.
- 6.3 `errdefer` — run cleanup only on the error path, and predict both
  paths.
- 6.4 Hardening by refactor — take chapter 1's panicky script to
  production honesty one construct at a time.
- 6.5 Capstone: wordcount — build Part 1's whole toolkit into one real
  program, with a boxed promise: in Part 3 this loop parallelizes by
  changing one call.
- Exercises 6-1 … 6-5.

## Part 2 — Memory

*The reader holds wolf's entire memory model as one question — "who owns
this, and how big is the granule?" — asked at four sizes.*

### Chapter 7 — Who owns this?
- 7.1 The error we owed you — re-read chapter 3's use-after-move with
  its full explanation.
- 7.2 Values are trees — draw an ownership tree, move a subtree with
  `take`, copy only on purpose.
- 7.3 Borrowing without the word — state the parameter rule in one
  sentence and see why it needs no syntax.
- 7.4 `mut` at both ends — write `grow(mut list)` and grep a codebase
  for its entire mutation surface.
- 7.5 Field-granular exclusivity — pass `mut p.x, mut p.y` legally, and
  read the diagnostic when paths overlap.
- 7.6 Why there are no lifetimes — judge the trade with both sides
  shown, including the zero-copy parser wolf loses and what it does
  instead.
- 7.7 What the machine does — connect `mut`/`read`/moves to `noalias`,
  freezing, and memcpy-and-forget.
- Exercises 7-1 ….

### Chapter 8 — Regions: memory in the shape you meant
- 8.1 You already think in regions — name the three arenas in programs
  you have already written.
- 8.2 The block form — open `region scratch { }`, allocate ambiently,
  walk away.
- 8.3 Regions are values — create, pass, and open regions as ordinary
  values.
- 8.4 Cycles are fine here — build the doubly-linked LRU Rust folklore
  says you cannot, and say what is checked at the border instead.
- 8.5 Freeze — publish an immutable snapshot with one verb and no copy.
- 8.6 Open, and open again — hold two regions open, checked disjoint.
- 8.7 `shared` and `handle` — choose an escape type by its failure
  contract, from the half-page decision table.
- 8.8 What the machine does — see bump allocation, wholesale free, and
  the aliasing fact C cannot state.
- Exercises 8-1 ….

### Chapter 9 — The escape hatch is a door, not a cliff
- 9.1 The three rings — grep a codebase for its complete unsafe surface.
- 9.2 Raw-tier rules — use C's pointer rules inside `unsafe`, with an
  oracle instead of an aliasing exam.
- 9.3 The oracle you actually run — inject a use-after-free and watch
  the checker fault it deterministically.
- 9.4 The one door back — re-enter safe code at the single sanctioned
  crossing.
- 9.5 `#include`-grade C — import a real header, call it, and wrap it in
  twenty safe lines.
- 9.6 FFI and regions — know what C may hold and for how long.
- 9.7 Auditing: `#[trusted]` and the audit surface — read a package's
  unsafe rings and hold its trusted roster to the manifest.
- 9.8 The four-tier picture — close Part 2 with the whole model on one
  page.
- Exercises 9-1 ….

## Part 3 — Concurrency

*The reader writes concurrent programs where leaks are structural
impossibilities, races do not compile, and a failing schedule is a seed
you can replay.*

### Chapter 10 — Spawning is a scope
- 10.1 The task tree — one arrow in, one arrow out; scope exit joins all
  children.
- 10.2 The leaked goroutine, retired — port Go's own leak example and
  watch the wolf version die honestly.
- 10.3 The dropped error, surfaced — see a child's error arrive at the
  scope join, handled or propagated.
- 10.4 Cancellation — reason about cooperative cancellation points,
  including the FFI boundary.
- Exercises 10-1 ….

### Chapter 11 — Scopes as values
- 11.1 The scope as a capability — spawn into a caller's scope, visibly.
- 11.2 The background refresher — build the connection-pool pattern
  without a detached anything.
- 11.3 The structured dump — read the task tree the interpreter shows.
- Exercises 11-1 ….

### Chapter 12 — Channels and select
- 12.1 Typed channels — synchronize with send/recv and one paragraph of
  happens-before.
- 12.2 `select` with timeouts — multiplex completion-based I/O without
  a task per idle connection.
- 12.3 When channels are the wrong queue — cite the numbers, reach for
  `std` queues.
- 12.4 `when (a, b)` — acquire lock sets whole, without ordering
  folklore.
- Exercises 12-1 ….

### Chapter 13 — Dividing one job
- (held) 13.1 `par` — parallelize Part 1's wordcount by changing one
  call (the promise, kept). Held at rp01: `par` has no surface in
  either implementation. The `(held)` prefix is what stops
  `verify-docs` demanding the heading in a shipped chapter; drop it
  when the section is written. The number stays reserved (§Deltas,
  rp01).
- 13.2 The race that does not compile — introduce a real race and read
  the rejection with its three suggested fixes.
- Exercises 13-1 ….

### Chapter 14 — Procs: the unit of failure
- 14.1 Armstrong's argument, one page — align service, failure, and
  ownership on one boundary.
- 14.2 Crash means bulk-free — see a proc's death free its regions.
- 14.3 Mailboxes — build on typed channels; know why selective receive
  is absent.
- Exercises 14-1 ….

### Chapter 15 — Link, monitor, supervision
- 15.1 Two primitives — wire `link` for shared fate, `monitor` for exit
  reasons.
- 15.2 A supervisor in forty lines — build the one the language does not
  ship.
- 15.3 The root supervisor — give the daemon shape a name and a tree.
- Exercises 15-1 ….

### Chapter 16 — Region transfer: fearless messaging
- 16.1 `ch.send(move r)` — move a cyclic object graph across procs with
  one word and zero copies.
- 16.2 Freeze, then share — pick transfer or sharing by shape.
- 16.3 The honest lineup — run the same workload against Rust's
  `Arc<Mutex>`, and weigh it against what Erlang's copy and Go's share
  permit.
- Exercises 16-1 ….

### Chapter 17 — The failing schedule, replayed
- 17.1 The bug that typechecks — meet an ordering bug that survives the
  type system, and hear the book say so plainly.
- 17.2 The seed, the schedule, and the frontier — hunt the heisenbug
  with `--seed`, `--explore` and `--schedule`, pin its counterexample,
  reproduce it deterministically, and watch the fix close the frontier.
- 17.3 Scope honesty — know what exploration cannot see and what v1
  does not promise.
- Exercises 17-1 ….

## Part 4 — Systems

*The reader makes wolf fast and keeps it fast: comptime as the mechanism,
contracts as the promise, benchmarks as the referee — and ships code into
an ecosystem with no build scripts to fear.*

### Chapter 18 — Comptime: one tier, no macros
- 18.1 Wolf at compile time — evaluate ordinary wolf during
  compilation, and know from the compiler's own refusals that it
  happened there.
- 18.2 Types as values — pass a type as an argument, reflect its fields
  and its traits, and make the answer a build-stopping witness.
- 18.3 Where comptime already touched your code — recognize f-string
  compilation and const-generic normalization as the same mechanism.
- 18.4 What it refuses to do — trace each refusal to caching,
  auditability, or the audit surface of a dependency.
- Exercises 18-1 ….

### Chapter 19 — Perf contracts
- 19.1 Four promises — break `#[noalloc]`, `#[inplace]`, `#[nopanic]`,
  `#[bounded_stack]` one at a time and read the errors.
- 19.2 Contracts are API — see a dependency lose `#[noalloc]` and semver
  notice.
- 19.3 When not to — smell contract noise.
- Exercises 19-1 ….

### Chapter 20 — Reading `wolf bench diff`
- 20.1 The format — read ns/op, allocs, and metadata; trust medians and
  MAD.
- 20.2 The variance gate — watch a "3% win" get correctly called noise.
- 20.3 Your own baseline — run the `--baseline` workflow in your repo.
- Exercises 20-1 ….

### Chapter 21 — Beating C honestly
- 21.1 Aliasing — compare safe wolf against hand-`restrict` C on the
  same kernel.
- 21.2 Arenas — measure region allocation against malloc discipline.
- 21.3 Layout — win a traversal benchmark with `Soa[T]` legally.
- 21.4 Checked arithmetic's bill — see the real post-optimization cost,
  from CI, dated.
- 21.5 Where C wins today — read the current losses with tracking
  issues, regenerated each release.
- Exercises 21-1 ….

### Chapter 22 — Modules: the shape of a wolf project
- 22.1 Directory = module — structure a project where files are
  invisible to importers.
- 22.2 No cycles — hit the error, do the interface-extraction refactor,
  and name what the rule buys.
- 22.3 No life before main — replace `init()` registration with comptime
  registries.
- Exercises 22-1 ….

### Chapter 23 — Packages and dependencies
- 23.1 `wolf.pkg` is data — read a manifest as the whole truth.
- 23.2 MVS in one page — get the same versions forever from the same
  manifest.
- 23.3 `wolf.sum` and the log — verify even the author cannot swap bits
  under a tag.
- 23.4 Script mode, demystified — see chapter 1's frontmatter deps as
  the same machinery.
- Exercises 23-1 ….

### Chapter 24 — The covenant: no build scripts
- 24.1 The threat, from history — event-stream, left-pad, build.rs; one
  paragraph each, sourced.
- 24.2 What replaces scripts — declarative recipes plus the sandboxed
  comptime you already trust.
- 24.3 Capabilities and `wolf audit` — catch the dependency that
  suddenly wants `net`.
- 24.4 What the covenant costs — name the autotools-shaped things v1
  cannot vendor, and why the line holds.
- Exercises 24-1 ….

### Chapter 25 — Editions, stability, publishing
- 25.1 Editions per package — upgrade on your clock; no wolf 2.0, ever.
- 25.2 The stdlib posture — depend on core, penumbra, or `std.x` with
  eyes open.
- 25.3 Publishing — ship `owner/pkg` with capabilities declared and
  semver with teeth.
- Exercises 25-1 ….

## Part 5 — Projects

*The reader builds five programs guided, one alone, and reads the
allocator they never needed — spending every skill the book taught and
learning nothing new, which is the point.*

The part's opening page states the **side-by-side honesty rule**: three
of these builds stand beside the C programs they descend from, wolf
wins are shown, and wolf non-wins are admitted in the text. Line counts
come from `wc`, not from vibes.

Gate notes below are CI vocabulary and stay out of reader text (TONE.md
§Tense discipline). A chapter waits for its gate *whole*: no project
ships partially. Chapters 26–28 shipped at the bs10 pin bump; 29–32 wait,
and their notes say on what.

### Chapter 26 — `count`, twice
*after K&R §1.5–1.6 · shipped (bs10) · Part 5's opener lands at this chapter's head*
- 26.1 The tool, twice — read the two-column form, and the honesty rule
  it runs on.
- 26.2 Ritchie's state machine — read the C twin whole, and name the two
  conventions it is built from.
- 26.3 The same machine as a `match` — write the wolf loop and say which
  column is shorter, from `wc`.
- 26.4 Per file, and a total — grow the program to many inputs, reading
  each through `fs_read_text` over rows.
- 26.5 Where wolf is not shorter — read the places the C wins, with the
  measurement.
- Exercises 26-1 ….

### Chapter 27 — `rpn`, twice
*after K&R §4.3 · shipped (bs10) · the part's one both-implementations project*
- 27.1 A stack and a switch — read the C twin, and find the value that
  means both "zero" and "empty".
- 27.2 The operand stack as a `List` — build the stack, push, pop.
- 27.3 Parse errors as payload-carrying tags — replace the error flag
  with a row, and propagate it with `?`.
- 27.4 The operator dispatch as a `match` — complete the calculator.
- 27.5 Develop interpreted, ship compiled — run the same source under
  lupin while writing it and `wolf build` it when done.
- Exercises 27-1 ….

### Chapter 28 — `wordtree`, twice
*after K&R §6.5 · shipped (bs10) · interpreted, for the reason in §Deltas*
- 28.1 The malloc showpiece — read the C twin, and count its allocations
  and its frees.
- 28.2 The tree in a region — allocate nodes ambiently and link them
  freely.
- 28.3 Counting words — grow the tree from real input.
- 28.4 The alphabetized walk — print both programs' output side by side
  and diff them.
- 28.5 The closing brace — replace `treefree` with a brace, and read the
  line counts for both programs.
- Exercises 28-1 ….

### Chapter 29 — `tinyvm`
*wolf-native · gate: pools/match native*
- 29.1 Fetch, decode, execute — build the dispatch loop as one `match`.
- 29.2 Registers as a fixed `List` — give the machine its state.
- 29.3 The heap as a `Pool` — allocate VM objects behind generational
  handles.
- 29.4 A dangling reference is a stale handle — inject one and read the
  trap, not the exploit.
- Exercises 29-1 ….

### Chapter 30 — `pargrep`
*wolf-native · gate: c07 native concurrency*
- 30.1 Sharding the input — split the work before spawning anything.
- 30.2 A task per shard — search slices inside one scope.
- 30.3 Results through a channel — collect matches without a lock.
- 30.4 The frozen pattern table — share one table across every task with
  no copy.
- 30.5 Testing with a seed — reproduce a parallel run exactly (X12).
- Exercises 30-1 ….

### Chapter 31 — `logden`, alone
*the solo · gate: c07 native concurrency (+ c10 optional §)*
- 31.1 The problem — read the whole specification of the program, and
  nothing else.
- 31.2 The milestone ladder — check your own work against six tagged
  checkpoints, each with one hint.
- No walkthrough, and no solutions: checkpoints are published, answers
  are not (EXERCISES.md §Solutions policy — the one exception, and this
  chapter is now all of it).

### Chapter 32 — Coda: the allocator you never needed
*after K&R §8.7 · gate: unsafe-tier surface (chapter 9's material)*
- 32.1 Why C ends here — read why allocation is where K&R had to finish.
- 32.2 A free-list allocator in a page — build one in the unsafe tier,
  because the floor is simpler-ruled than the safe tier.
- 32.3 The brace you already had — look back at chapter 28, where this
  entire job was one closing brace.

## Back matter

- **Appendix A — Grammar summary** — the surface grammar in one place,
  generated from the spec's `grammar.ebnf`, never hand-maintained.
- **Appendix B — Traps** — the twelve trap kinds, closed by
  `[conf.trap.set]`, each with kind, fault, and clause.
- **Appendix C — Diagnostics** — every stable error code the book shows,
  cross-checked against the compiler's catalog in CI.
- **Appendix D — Spec cross-reference** — book section → spec clause,
  for readers who want the normative text.
- **Solutions** — every exercise in chapters 1–30, collapsed by default
  in the web edition, extracted and CI-run like all samples. The solo
  (chapter 31) publishes milestone checkpoints instead — the one
  exception, and the only one.
- **Glossary** — one term per concept, the copyedit enforcement list.
- **Index** — hand-curated entries plus every error code, trap kind,
  and flag; section numbers, not page numbers, in the web edition.
- **Colophon** — the toolchain version this printing is true for, the
  CI-verified-samples guarantee, and the errata address.

---

## Deltas (per-sprint, for future authors)

Recorded where this ToC deliberately departs from the sprint files. The
sprint files remain the implementation contracts for everything else.

- **bs01:** the REPL moves from chapter 2 into chapter 1 (§1.4). The
  reader gets the lab bench before the strings chapter uses it; chapter
  2 then opens already holding the tool. bs01's "REPL as the chapter's
  lab bench" framing transfers to §1.4 intact.
- **bs01:** `wolf build` is a one-section chapter closer (§1.5), not a
  chapter-end aside; §1.2 is where installation happens.
- **rp-M1:** the "the book never asks for a second tool" promise is
  retired, not moved. It was never demonstrable — the book runs most of
  its programs under the reference interpreter and says so — and
  bs01's own amendment says a one-binary story is demonstrated or not
  told. §1.2 demonstrates the part that is true (`wolf` alone builds,
  runs, and ships a program) under its new title, "Two implementations,
  one language", which is also the front matter's framing.
- **bs02:** the hardening-by-refactor arc (sprint: chapter 6's teaching
  spine) is a named section (§6.4) distinct from the wordcount capstone
  (§6.5), so the capstone stays a build, not a rescue.
- **bs03:** the sprint's single chapter 7 keeps all its sections; the
  running-example seed (`shelf` value types) lives inside §7.2 and §7.6
  rather than as its own section — two forward-gestures to chapter 8 are
  the counted maximum.
- **bs06:** the sprint's four chapters map to chapters 10–13 unchanged;
  the `when (a, b)` material stays in the channels chapter (§12.4)
  rather than the parallel-iterators chapter its subject might suggest,
  because deadlock-freedom is a synchronization story.
- **bs07:** the sprint's four chapters map to 14–17 unchanged. §17.1's
  obligation — saying plainly that the heisenbug is an *ordering* bug,
  not a data race — is promoted to the section promise, since that
  honesty is the pitch's credibility.
- **bs07 (ch17's sections, 2026-08-11):** chapter 17 ships **three**
  sections, not four. The chaos section has no surface: `lupin run FILE
  --chaos` is `error: unexpected argument '--chaos' found`, and nothing
  in either implementation injects faults at effect points, so a
  section whose whole subject is injection cannot be written. The
  numbering closes up — the old 17.4 (scope honesty) is 17.3 — because
  chapter 17 had never shipped and its anchors were never published,
  which is the licence bs10 took when it restructured Part 5. Exercise
  17-6 (the chaos stem) is written, its baseline program runs green in
  the corpus, and it is **not printed** in the chapter; the day
  injection lands, the section and the exercise arrive together and the
  numbering grows a 17.4 at the end rather than renumbering anything.
  §17.2's title also changes: the contract, X12 and D23 name the flags
  `--schedules`/`--replay`/`--chaos`, and what ships is
  `--seed`/`--schedule`/`--explore`, so the section is titled for the
  instrument rather than for a spelling. The flag-branding feedback the
  contract asks be delivered before the naming decision closes is in
  `book/ch17.md`'s ledger, row 2.
- **bs07 (§15.2, recorded not edited):** the TOC promise reads "build
  one before being handed the stdlib's." There is no stdlib supervisor
  at the pin, so the section builds the supervisor, names the four
  decisions every supervisor makes, and gives the restart strategies as
  a design table — claiming no library. The promise wants rewording to
  "build the one the language does not ship"; left for bs11. **Reworded
  at rp01** to exactly that, since the row proposed the wording and the
  section has not moved.
- **bs07 (§16.3, recorded not edited):** the TOC promise reads "run the
  same program against Erlang's copy, Go's share, and Rust's
  `Arc<Mutex>`." The Rust half runs — vendored, compiled with warnings
  denied, asserted, and printed verbatim by the contrast rig. The
  Erlang and Go halves are credited prose boxes, for the reason
  chapter 10's Go boxes are: there is no Erlang or Go toolchain in the
  contrast lane, and what is being contrasted is what those runtimes
  permit rather than what a program of theirs prints. **Reworded at
  rp01** to "run the same workload against Rust's `Arc<Mutex>`, and
  weigh it against what Erlang's copy and Go's share permit" — the old
  wording promised three programs running and one runs.
- **bs08:** the sprint's four chapters map to 18–21 unchanged; §21.5
  ("where C wins today") is a first-class section, not a caveat box, and
  its regenerated-from-CI requirement is inherited from the sprint file.
- **bs08 (what shipped, 2026-08-11):** chapter 18 ships with all four
  sections and the Part 4 opener at its head; chapters 19, 20 and 21 are
  **held whole**, with the measured reasons in HOLD notes in their stubs
  and in `book/ch18.md`'s ledger. The one-line version: the comptime
  engine is real and the reader can run every verdict in chapter 18,
  while the perf half of the part has no instrument — the four I15
  contract attributes parse and are verified by nothing, `wolf bench`
  answers `not yet`, and `wolf build --release` answers that v0 has
  exactly one tier. Three sections' promises are reworded above, all in
  chapter 18, which has never shipped and whose anchors have never been
  published (the licence bs07 and bs10 took):
  - §18.1's promise loses "inputs are hashed, not forbidden" — the
    declared-build-inputs half of hermeticity is the package manifest's
    story and belongs to chapter 24 — and gains the honest source of the
    reader's confidence: the compiler's refusals are what prove the
    evaluation happened at compile time. There is no positive witness for
    a fold at this pin, because the folded value is not handed to the
    lane that executes the program (ledger, third row).
  - §18.2's promise loses `Soa[T]`. `typebuild` exists in the evaluator
    with no surface spelling for the `(name, type)` pairs it consumes,
    and there is no place projection, so the I9 worked example cannot be
    written; the section teaches types-as-values through `typeinfo`,
    `implements`, and the comptime `assert` witness instead. The same
    gap holds §21.3.
  - §18.3's promise loses test-table expansion — `wolf test` discovers
    zero-parameter `fn test_*` and expands nothing — and keeps the two
    mechanisms that are real: the f-string spec is read during
    compilation (E0412 on a misspelled one) and const-generic equality
    ring-normalizes at a documented line (E0707 at its boundary).
  Exercise 18-6 is printed in §18.4 rather than the §18.3 the generated
  index assigns it; §18.3 prints no exercise, and the two stems it wants
  are named in the ledger for the editing pass.
- **bs10 (superseded):** the earlier delta compressed a capstone into
  one chapter (26) with seven milestone sections. The single-capstone
  plan is retired at the sprint's own instruction (2026-08-11 user
  decisions): Part 5 is now **Projects** — five guided builds, the solo,
  and the coda, chapters 26–32 — and the monolith survives as chapter
  31, the one project the reader builds with no walkthrough.
- **bs10 (gates, 2026-08-11 — SUPERSEDED at the bs10 pin bump):** the
  earlier note recorded P1–P3 as undraftable, because at wolf-lang
  `f0da6e6` and lupin 0.1.4 the `str` method surface was typed in sema
  only, `fs_read_text` was refused in native lowering, `List` did not
  lower, and interpolating a `str` *value* was refused — so a one-line
  program that printed a word it had read could not be built. That
  finding was correct and it is now history: the wave-four close
  (wolf-lang `13b811f`) lands all 21 `str` methods, the `List`
  operations, value-position interpolation with format specs, and the
  nine `fs_*` builtins in native lowering. Chapters 26, 27 and 28 ship.
  The note is kept rather than deleted because it is the record of a
  gate holding, which is the mechanism the sprint contract depends on.
- **bs10 (what landed with the scaffolding pass):** the C contrast
  dialect the sprint's Mechanics section calls for (`cargo xtask
  contrast` compiles `samples/contrast/*.c` with `-std=c99 -Wall -Werror`
  and executes every case in `samples/contrast/cases.toml`, asserting
  stdout, stderr and exit status), the three C twins for P1–P3 as
  original K&R-idiom implementations with eleven asserted cases between
  them, their PERMISSIONS.md rows (§2, K1–K3 — `planned` until the
  chapters printed them, `placed` since), the Part-5 restructure, and the
  solutions exception narrowed to chapter 31.
- **bs10 (what shipped, and on which implementation):** the three
  side-by-sides do not all run on the same tool, and the chapters say so
  where a reader can see it (a console block names its command) without
  ever naming a schedule.
  - **Chapter 26, `count`: compiled.** `wolf build count.lu && ./count`
    is the chapter's transcript. It has to be: `fs_read_text` and
    `read_line` are the compiled column's, and the reference interpreter
    has no filesystem *by design* — it declines the effect rather than
    mocking one. The chapter's `wolf` fences therefore carry no `run`
    directive (the runner's `run(…)` lane is lupin's); they are checked
    by `conform-run`, and the console blocks are what assert the output.
  - **Chapter 27, `rpn`: both.** Identical five-line output under
    `lupin rpn.lu` and `wolf build rpn.lu && ./rpn`, both replayed by CI.
    §27.5 is built on that fact, and the reason it is available is that
    `rpn` has no effects.
  - **Chapter 28, `wordtree`: interpreted.** The recursive mutating
    insert passes `mut` through a nested place (`add(mut n.left[0], w)`),
    which native lowering refuses ("`mut` arguments beyond local places",
    c06). Everything else in the program lowers natively. The chapter's
    ledger carries the row.
- **bs10 (the tree's shape, recorded because it was not a free choice):**
  chapter 28's node is `struct Node { word: str, count: int, left:
  List[Node], right: List[Node] }`, with an absent child spelled as an
  empty list. That reads well — it is `NULL` without a null, and §28.2
  makes it the section's first point — and it was also the only shape
  available. `l[i] = v` is `NotYetCheckable` on **every** lane
  ("assignment through this place", s17), so an index-linked arena cannot
  patch a parent's child index after the child is pushed; and `Pool[T]` +
  `handle T`, the language's designed answer and the one chapter 8
  teaches, is refused by native lowering and, under lupin, a field write
  through a pool index "does not denote a place at run time" (the blocker
  exercise 8-7 has held on since bs04). Three spellings of one structure,
  none complete. A future author with any of the three closed should
  re-read §28.2 before assuming the list-child shape is load-bearing.
- **bs10 (no argv — the part's largest honest cost):** there is no way
  for a wolf program to read its command line at this pin. `fn
  main(args: …)` is rejected at the entry-signature check, `env_var` is
  typed and implemented by neither lane, and no `argv` name exists in
  the prelude. All three projects therefore hold their input as data:
  `count` writes and reads the two files it counts, `rpn` and `wordtree`
  carry theirs as literals. §26.5 charges the reader for it in the
  honesty section — as a property of the two listings, never as a
  schedule — and chapter 26's ledger carries it as the part's
  highest-priority row. The sprint's acceptance sentence ("every project
  produces a useful, self-contained binary") is met on
  *self-contained* and strained on *useful*.
- **bs10 (§27.4's dispatch, and a compiler bug):** the TOC promise reads
  "the operator dispatch as a `match`", and it is a `match` — over the
  operator's **byte**, with the character in a comment, because `match`
  on `str` literal patterns is broken: the reachability analysis reports
  every string arm after the first as unreachable (`warning[E0802]`,
  naming the first arm as already covering them) and native lowering
  then refuses the construct. Under lupin the same program evaluates
  correctly. §27.4 states on the page that the C's `case '+':` is the
  better spelling, which is the honesty rule doing its job; chapter 27's
  ledger carries the defect as its top row. Fixing E0802's string case
  would let the section be rewritten with `"+" =>` arms and would remove
  the ugliest four lines in the part.
- **bs10 (two extensions the contract names and this sprint could not
  set):** P2's "variables" extension wants a name-to-value table, and
  `Map` is a prelude name with no signature on either lane, so exercise
  27-8 is a *design* exercise that sketches it and says what a reader can
  build today. P2's "REPL loop" is buildable — `read_line()` lowers
  natively — and is not *showable*: a ```console block is replayed by
  `xtask` with stdin null, and an interactive transcript interleaves the
  reader's typing with the program's output in a way a replay rig cannot
  separate. P3's `-n` extension is set (exercise 28-3) without the
  contract's `std.sort`, which does not exist; `List` has no
  `sorted`/`sorted_by` on either lane, so the solution is a selection
  walk.
- **bs10 (machinery added, so a future author does not rebuild it):**
  two doc-truth checks landed with these chapters. `cargo xtask
  contrast` now checks a new fence dialect, ```` ```c-run,from(<case
  name>) ````, against `samples/contrast/cases.toml`: the printed
  transcript is derived from the case alone — the prompt from the file
  stem and argv, the body from the asserted streams, the exit status when
  it is nonzero — so a side-by-side's C half is as rot-proof as its wolf
  half. `cargo xtask verify-docs` now (a) recomputes every `<!-- WC
  (verify-docs): path=N … -->` claim, which is how "line counts come
  from `wc`, not from vibes" is enforced rather than promised, and (b)
  requires each `samples/projects/<name>/<name>.lu` to appear verbatim
  in the concatenation of its chapter's wolf listings, so the on-disk
  program and the printed program cannot drift.
- **bs10 (Part 5's reference budget, spent one of three):** chapter 28
  carries the part's single dark-Romantic placement for chapters 26–28
  (Mahler's Ninth, at the head, public domain and untracked in
  PERMISSIONS.md by design). Chapters 26 and 27 carry no epigraph and no
  in-prose simile. The part's one Cage placement and its second
  dark-Romantic placement are unspent and reserved for the solo and the
  coda, as the sprint contract sets them.
- **rp01 (chapter 13 ships one section of two, 2026-08-11):** the
  chapter's gate opened halfway. E1101, E1102 and E1103 all exist now,
  in **both** implementations, with the same codes, clause tags and byte
  spans — so §13.2 is writable and is written. `par` still has no
  surface in either tool, so §13.1 is held.
  - **The numbering does not close up**, which is the opposite of the
    licence bs07 took for ch17's chaos section. It cannot: `book/ch06.md`
    §6.5's boxed promise names §13.1 as the place the wordcount's
    one-call diff gets checked, and that box is shipped, reader-facing
    text. So §13.2 keeps its number, §13.1 stays vacant, and `par`
    arrives in the slot ch06 already points at. Nothing is renumbered
    and no shipped page is edited to make room. The held row above
    carries a `(held)` prefix because `verify-docs` matches TOC rows
    beginning with a dotted number against `## N.M` headings, and a
    shipped chapter must not render an empty section; dropping the
    prefix and writing the heading is one edit when the surface lands.
  - **The chapter is retitled** from "Parallel iterators" to "Dividing
    one job". A title naming a construct the chapter does not contain
    is a promise by another spelling, and the new title is true both
    now and after §13.1 lands. The chapter had never shipped, so the
    title was free (the licence bs07 and bs10 took).
  - **Exercises:** five of eight printed — 13-2, 13-3, 13-4 in §13.2 and
    13-5, 13-7 in the chapter batch. 13-1 and 13-6 hold their
    `samples-pending.toml` rows on `par`; 13-8 is a design stem about
    `par`'s decomposition contract and is written, on file, and not
    printed. 13-2 is printed in §13.2 while the generated index assigns
    it to §13.1 — the ch18 precedent (18-6).
  - **13-3 was rewritten at this pin, not re-blessed.** Its second half
    used to turn on lupin *running* the racy program to a silently wrong
    exit 0; lupin now rejects it statically, so the exercise's subject
    changed from a differential to an agreement, and the stem asks about
    the one thing that still differs (how much output each tool gives
    for two offending spawns).
- **rp01 (the promise ledger):** four recorded TOC rewordings landed —
  §10.2 and §11.3 (bs06's rows, named by the contract) and §15.2 and
  §16.3 (bs07's, which the contract did not name; taken because the
  sprint goal is a clean ledger for bs11 and both rows were pure
  wording reconciliations still true at this pin). Not closed, and why:
  ch06 §6.5's box (audit row 26) is untouched — it is `par`'s, its
  pointer is correct, and rp01's gate did not open on `par`; and bs08's
  reword-not-edited row, ch10 §10.1's "Part 4 asks it", stays
  outstanding because chapters 19–21 are still held.
- **bs11:** back matter gains an explicit Appendix D (spec
  cross-reference), which bs11's checklist implies ("claims traced to
  spec clauses") but never lists as an artifact. The index doctrine
  (section numbers as anchors) is stated here and in DESIGN.md so bs11
  inherits it rather than deciding it.
