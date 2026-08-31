# Changelog

What changed for the reader, entry per merged sprint (D65).

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
