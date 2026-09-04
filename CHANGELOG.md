# Changelog

What changed for the reader, entry per merged sprint (D65).

## bs28 — 2026-09-03 — the ladder lights

The compiler does not move this sprint. The interpreter does, from lupin
0.1.24 to 0.1.25, and what it brings is the half of last sprint's
release that had not reached it yet: `byte`. One release ago this book
taught a scalar that only one of its two machines could read, and it
said so on the page rather than quietly running the blocks once. Both
machines read it now, and the two blocks that were waiting are executed
on both.

**§2.3's byte transcript is byte-identical across the pair.** `65 65
200`, `255 0 44 255`, `400 66 66`, exit 0, under `wolf run byte.lu` and
under `lupin byte.lu` alike — measured at the bump, before the fence
was touched. So the page shows one transcript and not two, which is the
book's rule for a program whose machines agree, and the fence moves out
of the compiler-only lane into the shape §2.4 has used for `char` since
bs17: the interpreter runs the program, the compiler runs the console
block beside it, and both readings have to match the same six lines. The
same graduation happens two hundred pages later, where §8.9's byte-
ledger reading answers `true` to all three of its relations on the
interpreter too.

A third fence went with them, and it is older than the byte. Chapter
4's trap-abandons-your-defers program has printed `counting` and
`drawer locked` under the compiler since the book had a compiler, and
its paragraph has said "both machines do this now" since the
interpreter's divergence was fixed — but nothing was checking the
second machine. It is checked now: lupin names the same `assert` and
exits `3` where the compiler exits `134`, which is the per-machine
status D60 rules and the same kind either way. Three fences into the
two-machine form, and one printed block is left that the compiler runs
alone: chapter 30's parallel grep, which writes files. The exercise
corpus keeps eight more, five of them comptime folds the interpreter
declines by design and three of them chapter 30's.

**A price with two numbers is a relation, not a number.** §8.9's byte
ledger exists to prove that holding octets as `int`s costs real memory,
and it prints relations because the units belong to whichever arena you
ran in. Running it on a second arena is what makes that argument
visible instead of merely stated: the same 65,536 octets that charge
65,584 ledger bytes compiled charge 65,568 interpreted, and the same
values pushed into a `List[int]` charge sixteen times the octets on one
machine and thirty-two on the other. All three printed relations hold
on both. The section names both multiples now, and §2.3's one-sentence
version of the argument stops quoting a single machine's sixteen.

**The two version lines no longer name the same interpreter, and that
is fine.** The compiler was tagged before this interpreter release
existed, so `wolf --version` still reports being paired with lupin
0.1.24 while `lupin --version` reports 0.1.25 — pinned, in its own
stamp, to the exact compiler revision this book pins. The colophon has
carried a sentence since the first edition saying a printing whose two
lines differ by a release is ordinary; this is that printing, so the
sentence states the fact rather than anticipating it, and §1.2 gains
three lines telling the reader how to read a pair that disagrees. Those
two transcripts are the whole of the bump's blast radius: 462 passed, 5
pending, 0 failed and 0 flips at the raw new pin, with the version
blocks the only two failures, and nothing else in the book moved.

**A gap in the interpreter that no page can reach.** lupin 0.1.25 has
the byte type but not its domain: `push(256)` into a `List[byte]`
stores 256 and prints it, where the compiler refuses the same line by
name. That is filed as wolf-interp#62 and it was predicted to touch
nothing here before the suite was run — this book pushes into no
`List[byte]` anywhere, annotates no `byte` from an integer, and hands a
`List[byte]` only to a parameter declared over one, so the only
integer-to-byte flows on any page are §2.3's four explicit casts, which
truncate by clause and agree on both machines. Measured after: nothing.
A program the compiler refuses is not one this book can print, which is
why the gap is recorded in the pin file and on no page.

Two more claims narrow because a second machine can finally be asked.
D74's string-layout codes reached the interpreter with this release, so
lupin answers `E0104` on §2.2's own program where it answered an
invented `E0109` one release ago — the same line, the same code the
page prints, in its own words. Appendix C still says the block was
shown by the compiler, because it was; the reason last sprint gave for
that has retired. And chapter 11's connection-pool row, which lost its
premise last sprint when the toolchain grew a network surface, loses
its replacement clause here: the interpreter serves the unix-domain
family too, measured on this host — listen, connect, accept, the byte
read and write pair, and a listener close that unlinks its own path.
The row stays open on the editorial call it has always rested on: this
edition has no network chapter, and no page makes a socket call.

The print edition holds at 513 pages.

## bs27 — 2026-09-03 — the scalar table grows

The pins move to wolf v0.2.4 and lupin 0.1.24, and the language has a
new scalar in it. A *byte* is one octet — eight bits, unsigned, `0`
through `255`, one byte of storage — and every builtin that hands you
raw bytes now speaks it: `bytes()`, the file readers, the socket pair.
Chapter 2 has had a section called "Bytes, honestly" since the first
edition, and no byte in it. It has one now, taught where the reader is
already counting them, with ten numbers on three printed lines doing the
whole job: the widening cast that cannot fail, the narrowing one that
keeps the low eight bits and never traps (`256 as byte` is `0`, `-1 as
byte` is `255`), and `200 as byte` added to itself printing `400`,
because arithmetic on a byte is an `int`'s arithmetic and nothing
overflows eight bits by staying in them.

That is a breaking change and the book wore it. Ninety-eight refusals
across seventeen files, measured at the new pin before a line was
touched: forty-seven comparisons of a byte against a number, forty-three
`match` arms written as bare literals, six byte views handed to
parameters that wanted integers, and two casts to a width the byte does
not bridge to directly. Every one is one line, and every one is now the
spelling the compiler's own note asks for. The word counters of chapter
26, the RPN calculator of chapter 27 and its five exercise variants, the
release-tier scanner of chapter 19 and the `wrapping[i32]` hash of
chapter 20 all say `as int` where they meet a number, and read the same
as they did.

**Why bother, when every octet fits an integer with seven bytes to
spare?** Because those seven bytes are the price and they are not the
whole of it, and §8.9 now measures the whole of it instead of asserting
it. A region holding 65,536 octets charges 65,536 octets and one list
header — the runtime knows the length before it allocates, so there is
no growth history to pay for — and the same 65,536 values pushed into a
list of integers charge at least seven times that, and on the machine
this printing was built on, sixteen. The section prints those as
relations rather than as numbers, the way it prints every other ledger
reading, and the sentence in its budget half that warned "a
sixty-four-kilobyte buffer's worth of elements can charge a megabyte of
ledger" now points at the measurement two paragraphs above it, which is
exactly that megabyte.

Chapter 2's multiline strings gained their refusals. §2.2 has stated
three layout rules since the first edition and enforced none of them on
the page; each has a code now, one rule per code, and the margin rule is
printed in full because its rendering shows both ends of the comparison
— the line that sits too far left, and the closing delimiter whose
column decided how far that was. A `"""` that shares its line with text
is one refusal whether it is the opening one or the closing one. And a
tolerance worth knowing sits at the end of §2.3: a byte order mark at
the very start of a source file is stripped and is never a diagnostic,
so an editor that insists on writing one cannot break your build.
Appendix C gains all five codes, and its count was re-measured rather
than incremented — it claimed 48 while the table held 49, and it says 54
over 54 now.

**`samples-os.toml` holds no rows.** The file of per-host differences
opened last sprint with six, four of which retired at the previous pin
when Windows grew a task layer. The last two were never about a version:
one compiler spelled the same project's paths two ways, `wolf add` and
`wolf publish` printing the host's separator where every diagnostic in
the same binary prints a slash. That is fixed at this release, and the
Windows lane said so before anything was deleted — it failed both rows
by name, as stale, and named the issue that had landed. The machinery
stays and both directions stay enforced. An empty file is a measurement:
every declared per-host difference this book has found has been answered
by the toolchain.

Two of this sprint's blocks run on the compiler alone and say so — the
book has had that lane since bs09, for exactly the programs one
implementation runs: the new byte-cast transcript and the new ledger
reading. The reference
interpreter's release predates the type, so it answers `65 as byte` with
"nothing with this name is in scope" — probed at the bump in both
directions rather than assumed, recorded in the pin file, and retiring at
that project's next release. Neither block is skipped; both are executed
and byte-compared on every lane that has a compiler.

wolf also learned unix-domain sockets this release, and no page prints
one, which is worth saying plainly: this edition has no network chapter
and makes no socket call anywhere, so there is no list of transports for
the family to join. It was measured on this host at the pin and recorded
where the book keeps toolchain facts it does not teach. What it did
retire is a stale sentence in chapter 11's own ledger, which had been
explaining a design choice with "there is no network surface at this
toolchain" long after there was one.

The clause anchors grow 411 to 417 — four for the new scalar, two for
the socket clause — with none dropped and none retargeted. The
diagnostic catalogue does not move at all: this release re-ruled four
codes and minted none. The grammar appendix regenerates to itself, since
`byte` is a type name and not a keyword. Two version transcripts,
chapter 22's interface stamp and chapter 25's publish record re-record
as they do at every bump, and no printed diagnostic moved.

The print edition sets to 513 pages, three more than the previous one.

## bs26 — 2026-09-02 — the rows retire, and the links come back

The pins move to wolf v0.2.3 and lupin 0.1.23, and the headline is a
table that no longer exists. One release ago Windows compiled and ran
your program for the first time, and refused twenty-one programs of the
compiler's own corpus by name — everything built on the task layer,
which that host had none of. It has one now. `spawn` and scopes, `proc`,
channels and `select`, `sync`/`when`, region transfer, signals and
network deadlines all compile and run there, measured at the same corpus
parity as macOS. Chapter 1 said Windows readers should expect to meet
that limit; it does not say so any more, because they will not.

The book found out the way it was built to. Four programs of chapter 30
were written down last sprint in `samples-os.toml` as refused on
Windows, in the refusal's exact words, with the release that would end
them named in the row. At the new pin the Windows lane ran them, they
passed, and the run went red — four FLIPs, each naming the row to
delete and the release it was dated to. Then the rows came out, in the
commit that moved the pin, which is the rule they were written under.
455 passed and 4 flipped on that lane; 455 + 4 is the 459 the other two
hosts report. A skip would have gone on passing quietly through the
release that made the claim false. Two rows are left, the ones that were
never about a version: `wolf add` and `wolf publish` still print
Windows' own path separator where every diagnostic in the same compiler
prints a slash, and that is still filed rather than papered over.

**The Solutions page has its links back.** Every one of the 280 collapsed
solutions is headed by the exercise number and the section that set it,
and on the web that section reference had been rendering as its own
markdown punctuation — `[§3.4](../ch03.md#3.4)`, on all 280 of them,
because the line sits inside a raw-HTML block and a markdown parser
does not look inside those. It is a real link now, on the web and in the
PDF both. The print half is why the fix waited: a printed page has no
hyperlinks to give and no `.html` to point at, so the same source line
becomes an internal cross-reference to the section's own label — which
meant the print edition had to start labelling its headings at all, with
the very same rule the web edition has always anchored them by. One
rule, two renders, 280 references that cannot drift from their targets
because a reference to a section that moved fails the build.

Chapter 1's install section was re-measured against the project's own
install page for this release. Four archives at the tag now, one per
tier-1 host — the ARM one came back after a release that built it and
threw it away — so the section says four instead of hedging, and says
what the ARM archive serves, which is less than the other three. Two
limits are left on Windows and both are quoted rather than reasoned
about: the optimizing release tier still refuses that host, and a
`reload` or `upgrade` signal sent from *another process* has nothing on
Windows to arrive through. And a sentence this book got wrong about
itself is fixed: chapter 1 and the Notation page both explained the four
quoted Windows transcripts by saying the book's runner has no Windows
lane. It has had one since the previous edition. The real reason is
narrower — the runner replays programs, and an installation is not one —
and that is what both pages now say.

The rest of the bump was quiet, which is worth reporting. The compiler
changed the width of a parse error's underline this release, and its own
measurement predicted seven of this book's printed diagnostics would
widen. None did: every E0201 in the book points at a single-character
token, where the old shape and the new shape draw the same one caret.
That was probed both ways rather than assumed. The clause anchors hold
at 411 and the diagnostic catalogue at 169, neither moving by one. The
grammar appendix grew twelve productions the specification had been
citing without defining, so three of the six ways to write a string in
wolf can now be derived from the appendix instead of inferred from
prose. Two version transcripts, chapter 22's interface stamp and chapter
25's publish record re-record as they do at every bump. And chapter 4
§4.3 loses a caveat: the one place in that chapter where the two
implementations disagreed — whether a trap runs the outermost pending
`defer` — is a place where they agree now, and the section says which
one moved.

The print edition sets to 510 pages, the same as the previous one.

## bs25 — 2026-09-02 — the samples lane is real

CI ran the samples on three machines for the first time. It had been
able to for months; the credential that lets it read the pinned
compiler was set today, and the lane that had been loudly skipping went
and did the work. macOS agreed with the machine the book is written on,
459 samples to nothing. The other two hosts had never been asked, and
they had 25 things to say.

Nineteen of them were one sentence. On Linux, `wolf build` looks for the
LLVM linker and says so when it does not find it, and the runner did not
have it — so nineteen transcripts across nine chapters gained a line the
book does not print. The rig was right to fail: the line is real output.
The question was what to do about it, and there were two answers. The
book could teach the replay to drop `note:` lines, or the lane could
have the linker. Dropping them is the answer that makes CI quieter and
the reader's terminal no different — a book that hides a line its reader
will see is a book that lied about the byte it saved. So the lane
installs `lld`, and chapter 1 §1.2 now tells you to install it too,
prints the note you get if you do not, and says the build still
succeeds. Every console block in this book is still compared byte for
byte with nothing subtracted.

Six were true statements about a host. Four programs in chapter 30
cannot be built on Windows at this pin — the parallel capstone and three
of its exercises, all of them the task layer, which wolf refuses there
by name and by symbol until the runtime lands on IOCP. Two console
blocks in chapters 23 and 25 differ by one character each, where `wolf
add` and `wolf publish` print the host's path separator while every
diagnostic in the same compiler prints a slash.

A skip would have covered all six, and the book does not skip. There is
a new ledger instead, `samples-os.toml`, which is the pending manifest
turned sideways: where that file says "not yet, anywhere", this one says
"not here — and here is exactly what here says instead". Each row
carries the outcome verbatim, and the rig holds the row to it in both
directions. A refusal that changes its wording fails. A program that
starts working flips, hard, naming the row to delete — the same
discipline that has caught every feature landing since bs09. The four
chapter 30 rows carry v0.2.2's refusal sentence whole, down to the
runtime symbol that would not link, and the date they were declared;
they come out at v0.2.3. The two transcript rows carry the Windows text
in full and cite wolf-lang#222, which was filed rather than worked
around, because one binary spelling the same project's paths two ways is
the compiler's business and not the book's.

Chapter 1 gained one correction it owed the reader independently. It
said chapters 10 through 17 were the ones needing a host with the task
layer and the rest of the book runs anywhere. Chapter 30's parallel
capstone needs it too — measured, on the Windows lane, the day the lane
first ran. The sentence now says so, and names the sequential twin that
does run anywhere. The print edition sets to 510 pages, one more than
bs24, all of it §1.2's.

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
