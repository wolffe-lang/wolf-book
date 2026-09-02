# Changelog

What changed for the reader, entry per merged sprint (D65).

## bs24 — 2026-09-02 — the book sees the comma

The syntax highlighting is re-pinned. Every code block in the book is
painted at build time by a grammar vendored from wolf-lsp, and that
grammar had been sitting at a revision older than the `char` type: a
char literal was body ink, and so was the word `char` itself. Both now
paint — `char` in the type blue, `'a'` in the same green as `"a"`,
because a char literal is quoted text and the palette sorts by kind.
The escape inside a string keeps its own bronze, which is the one
ordering that had to be got right.

Six blocks change colour and no page moves: 991 rendered blocks
compared before and after, six differ, and the print edition sets to
509 pages either way — colour is ink, not metrics. The six are the
`n as char` cast in chapter 2 and five solutions in the back matter,
where the brace-balancer and the Caesar shift are made of char
literals. The print edition takes the same six changes from the same
grammar and the same palette, which is the single-source rule working
rather than being asserted.

Ten blocks were then read by eye against what they mean, and two of
them are painted wrongly by the pinned grammar. In a raw literal the
braces are two more characters — chapter 2 says so in a sentence, and
the sample's own output proves it — but the grammar paints them as an
interpolation, and it does the same to the raw strings the brace
balancer is scanning. And inside an interpolation a char literal goes
unpainted while the `as char` beside it paints. Neither is patched
around in the book: both are filed upstream (wolf-lsp#4, wolf-lsp#5)
and recorded in the pin, where the last rendering gap was recorded and
from where this one was closed.

Nothing else was needed. The grammar's new error node has nothing to
mark — no rendered block in the book carries an invalid escape — and
the region keywords `cap`, `rc` and `pool` are contextual by the
specification, which a grammar made of regular expressions cannot tell
from a name, so `region r(cap: n)` paints the word `region` and stops.
Measured, both of them, rather than assumed.

## bs23 — 2026-09-02 — the book holds a budget

The pins move to wolf v0.2.2 and lupin 0.1.22, the learners' release,
and three things arrive with them. Chapter 8 gains §8.9: a region will
now tell you what it holds, and you can tell it what it may hold.
`region_bytes` and `live_region_bytes` are taught as the four relations
the specification guarantees on every implementation — zero at the
open, charged after a build, unchanged between two adjacent reads, and
gone wholesale at the brace — and deliberately not as a byte count,
because the unit is the machine's and the section says so. `region
r(cap: n)` puts a ceiling on the ledger, a charge past it traps at the
allocation that asked for it, and the budget in every sample is
*measured* rather than estimated, which is the section's other lesson.
The last part is the one a server wants: a request that breaches its
budget inside a proc dies alone. The reason reaches the join as a value
(`is_fault()`, `is_alloc_contract()`), the memory is back before the
reason is delivered, and the `defer` below the proc boundary never
runs, which the transcript proves by the line that is missing. Chapter
14's per-proc accounting aside and §8.1's per-request arena both point
at it, and two exercises land beside it: 8-18 reads the ledger four
times, 8-19 is the cap kata.

Chapter 1 gains an install path. Both projects now publish a per-host
archive at every tag, so §1.2 leads with "unpack it and put it on your
PATH" instead of two cargo builds, and Windows — where the compiler
produces and runs a native `hello.exe` for the first time — is spelled
out exactly as the project's own measured page states it: the Visual
Studio Build Tools requirement, the refusal quoted whole for a machine
without them, `lupin.exe` as one file with no installer, and the two
things that still refuse there. Those four blocks are labelled for what
they are — transcripts measured elsewhere, not replays — because this
book's sample runner has no Windows lane, and the Notation chapter now
says how to spot one.

§4.3 answers a question its own sentence raised: `defer` runs when the
scope exits "whichever way it exits", and a trap is the way out that
runs nothing. The compiler's transcript shows an inner block's `defer`
firing on time and the outer one abandoned; the interpreter at this
pin still runs the outer one, and the section names that as a recorded
divergence rather than a second reading of the rule. §4.2 teaches the
separator law that landed with this release: a comma between closure
parameters is required, the refusal quotes the production it enforces
and writes the repair out, and the same sentence governs struct
literals, patterns and capture lists. The book's own prose was swept
for the comma-less spellings and had none.

Appendix A regenerates on three grammar changes, Appendix B gives
`alloc-contract` and `assert` the sections they now have and states
what a trap does to a pending `defer`, and the anchors grow 404 → 411
while the diagnostic catalog holds at 169. Four transcripts re-recorded
at the bump, every one classified, zero failures and zero flips. The
corpus grows 248 → 250 files, the index recounts at 329 exercises (280
printed), and 459 samples pass against bs22's 452.

## bs22 — 2026-09-01 — the book takes up arms

The pins move to wolf v0.2.1 and lupin 0.1.20, and for the first time
since the 0.1.15 era both tools name the same interpreter release:
the colophon's paragraph explaining why they differed retires with
the fact behind it. §4.3's `defer` teaching is corrected where it was
wrong — a `defer` in a loop body runs at the end of every turn, not
when the function returns — and the section's sample now prints an
interleaved transcript that can tell the two readings apart, which no
sample in the book could before. Patterns arrive at the ladder rather
than at a new section: exercise 3-14 rewrites 3-9's pack drill as one
`match` over the pair `(n % 3, n % 5)` and shows the compiler naming
the unreachable arm that the `if`-chain version could only leave
silent; 7-16 respells the Point/Rect kata so the arms take the value
apart by field name; 13-11 scans for a substring through a slice of a
*lent* byte view. Appendix A regenerates on the struct-pattern
production and a `\u{…}` escape now bounded at six hex digits;
Appendix C's catalog grows 168 → 169 with `E0814`. The corpus grows
244 → 248 files, the index recounts mechanically at 327 exercises
(278 printed), and 452 samples pass against bs21's 448 — five
transcripts re-recorded at the bump, every one classified, zero
failures and zero flips.

## bs21 — 2026-08-31 — the exercises multiply

The K&R ladder: 45 new program-shaped exercises (43 printed, two held
to the masters as drills), each a self-contained tool the reader
leaves owning — temperature tables and a longest-line finder in
chapter 1; reverse, squeeze, centering, visible escapes and
detab/entab in chapter 2; the pack drill, a binary table, one-pass
statistics and arithmetic palindromes in chapter 3; Collatz, a
closure factory, `rtrim` and Zeller's weekday in chapter 4; the
run-length pair round-tripped by exit code, a histogram, `any_index`,
a CSV ledger and a line folder in chapter 5; `itoa`, a hardened
decoder row and a date validator in chapter 6; the Point/Rect kata
and a consume-versus-lend rewrite in chapter 7; a region ring window,
a little-endian byte round trip, a bracket matcher and infix→postfix
on the worklist stack, a caesar round trip, substring counting both
ways, a stockroom proc, the Josephus ring moved whole, and — filling
chapter 18's own ledger asks — the E0412 grammar spelunk, the
`Buf[…]` identity drill, and roman numerals folded both ways with a
compile-time witness. Chapter 22 gains the multi-file tier: a
two-module word counter, a calculator behind an `ops` seam, the
`//! member: false` scratch pair with its one-marker trap, an
in-module name clash and its fix, an export-hash spelunk, and a
what-earns-`pub` design. The Notation chapter now states the
directive-header and member-marker rules it was long cited for;
Appendix C gains `W0313`. The corpus grows 191 → 244 files, the index
recounts mechanically at 324 exercises (275 printed), and every new
sample replays green: 448 passed against bs20's 401, zero failures,
zero flips, no new pending rows.

## bs20 — 2026-08-31 — the record on the page

Chapter 25 gains its first printed section. §25.3 teaches `wolf
publish` as measured at the pins: the one-line transparency record
and its three content addresses (tree, manifest, interface), the
maintainer's static-log append with its keyed head, and the refusal
that makes a published version immutable — both transcripts replayed
from a fixture in CI. The "this edition does not carry this chapter"
stub retires; editions (§25.1) and the stdlib posture (§25.2) stay
reserved with their reasons on record. 401 samples green, 196 of 198
console blocks replayed (two new).

## bs19 — 2026-08-30 — the pin tells the truth

The book's transcripts now show the released toolchain: wolf v0.2.0
and lupin 0.1.18, printing the bare D57 version strings (a plain
trunk build fails the book's own checks by design). Every `str + str`
claim reads present-tense — the feature landed, and the prose stopped
hedging. Chapter 25's publish-gate section healed and is flagged for
the human's call; chapter 16's stale differential was re-recorded.
All 401 samples green, CI 7 of 7.

## bs18 — 2026-08-30 — the numbers teach

Two held chapters exist at last: chapter 19 (reading the release
tier) and chapter 20 (performance contracts), written against what
the toolchain actually does — inert attributes are never taught, and
`wolf bench diff` appears nowhere because it does not exist. Chapter
21's wrong number is fixed (two million → four thousand), and chapter
5's stale exit-4 refusal fences retired with a re-teach.

## bs17 — 2026-08-29 — the pin catches the site

Pins advance to wolf addcd7f and lupin 0.1.16, and every trap
transcript now names its site the way the tools do: 79 line:col
sites byte-exact, one transcript and eleven claims updated for the
compiled tier's `at file:line:col` second line. Appendix B gains the
D60 exit-status table. The char-era fences graduate — 398 samples, 0
failures, with the environmental class empty.

## bs16 — 2026-08-29 — the register rewrite

Four lanes (front + ch01–11, ch12–22, ch23–32 + back matter, and the
solutions mirror) rewrote the whole book to the ratified register:
running em-dash density to zero per thousand words nearly everywhere
(a handful of defended survivors), ~1000 template dashes out of the
exercise and solutions apparatus, worth-markers and reveal molds
retired, UK spellings out. Two false toolchain claims were fixed by
probing, not by wording (ch27's char literal, ch31's spawn-in-loop),
and chapter 2 gained a cast-trap transcript measured on both
machines. 159 drifted solution stems re-adopted their chapters' text.

## bs15 — 2026-08-28 — the exemplar and the register

The fix register itself: eleven ratified rules with three human
amendments, proven on two exemplar chapters before the fan-out —
chapter 15 (14.0 → 0.0 dashes per thousand words, all sixteen rows
dispositioned, samples byte-stable) and chapter 5 (nine schedule
claims resolved by checking reality: traits landed reads present
tense, absent surface reads as scope, two unruled gaps to the
ledger).

## bs14 — 2026-08-28 — the tells catalog

An audit, not an edit: three lanes cataloged every AI-prose tell in
the book — 538 rows across front matter, 32 chapters, back matter and
solutions, zero banned-vocabulary hits, six systemic patterns named
(the 14-per-1000-words running-dash constant among them). Zero prose
changed on the page; the catalog is what bs15 and bs16 executed.

## bs13 — 2026-08-28 — the write-marked receiver, and char

Pins advance to wolf a900b8c and lupin 0.1.14. `(mut xs).push(…)` is
now mandatory on both machines, so Part 1 teaches it from the start —
chapters 2–7, the appendices and a dozen exercise solutions re-spelled,
with a read-it-as-"this call writes" paragraph in chapter 3. Chapter 2
gains §2.4: char as a Unicode scalar value (D58) — `chars()`, the
grapheme refusal with E0110 printed in full, scalar order, and the
not-an-integer rule. Every chapter's ledger was re-probed at the pin;
the E0804 Part-1 blocker resolved.

## bs12 — 2026-08-27 — the ledger triage

Every chapter's claim ledger re-probed against the current pin:
healed rows ticked (E1101/E1102 both tools, the unsafe ring enforced
under the interpreter, spawn-in-loop and the select ICE closed),
surviving rows narrowed with fresh evidence, and all 108 open rows
filed upstream as fourteen theme issues (wolf-lang#150–#163). A new
`cargo xtask ledger` gate keeps every future row filed or waived.

## rp03 — 2026-08-26 — the dialects look different

Every code block on the page now wears its dialect: a label, an
accent rule and a ground tint per dialect, generated from one
taxonomy table, identical on web and PDF. The notation page's legend
says what each dialect is and what CI holds it to.

## rp02 — 2026-08-24 — the pin catches the declaration

M2 declared upstream, and the book catches up: chapter 21 opens three
of its five sections — aliasing, arenas, and the bill — with the
declared benchmark number printed as a CI artifact rather than
typed into prose. The C contrast twins (saxpy with and without
`restrict`, ten thousand nodes under malloc) compile and run in CI.

## traits-era — 2026-08-22 — the book learns the trait system

Chapter 5's §5.5 lands: the trait system the chapter promised in
§5.3, taught with executed samples at the traits-era pin, plus
exercises 5-9 and 5-10 with published solutions. The pairing line
moves for the first time since lupin 0.1.8.

## bs10 pass three — 2026-08-20 — the pin crosses the campaigns

The conc and generics campaigns cross with zero snapshot movement —
the first pin bump with that property. Gates re-measured: chapter 29
loses the monomorphization half of its hold, chapter 13's race row
half-closes and inverts (wolf now catches the capture; wolf-interp#30
filed), chapter 18's §18.3 claim narrows to const-generics.

## The parts shipping — 2026-08-09 → 08-12 (bs01–bs10, rp-M1, rp01, P5)

The book itself, part by part, every sample executed at pinned
toolchains and every console transcript byte-replayed:

- **bs01–bs02** — Part 1 (ch01–06): notation, values, functions,
  collections, errors-are-values with the receipt capstone.
- **bs03–bs05** — Part 2 (ch07–09): moves and modes, regions with the
  compiled Rust contrasts, the unsafe tier run three ways.
- Part 3 (ch10–17): scopes and the join law, channels, select,
  procs, supervision, let-it-crash, determinism; ch13 held until
  rp01 landed it where its gate opened — the race that does not
  compile, both tools agreeing on code and span.
- **bs08–bs09** — Part 4 opens: comptime (ch18), directory-is-module
  and the interface export-hash (ch22), the covenant chapter (ch24);
  ch19–21, ch23 and ch25 held honestly on surface the toolchain did
  not yet have.
- **bs10 + P5** — Part 5, the guided projects: the C twins compiled
  and asserted, P1–P3, pargrep's determinism argument on the page
  (six seeds, one output hash), the allocator coda; tinyvm held on
  its measured gate.
- **rp-M1** — chapter 1 opens on `wolf build hello.lu && ./hello`;
  the pre-alpha banner and sprint-number teaching die.
- The edit pass: cross-references that resolve, terminology settled
  (task not thread, row not error union, trap not panic), appendices
  generated from the pinned spec, solutions for 214 exercises.
