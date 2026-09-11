# Changelog

What changed for the reader, entry per merged sprint (D65).

## bs43, the book takes the eleventh — 2026-09-11 — the whole of generics on one page, the bare `if`, and every send that could fail says so

The pin moves to two release tags at once: wolf 0.2.11 and lupin
0.1.33. §1.2 and the colophon print the bare stamp again, and the pair
reads one release and ninety-one commits, both computed from the
stamps. Nothing in the language moved between the dev sha this book
last pinned and the tag, so no transcript on these pages changed for
the compiler's sake; the nine papercuts that landed on the compiler's
trunk the same day are past the tag, and a reader who sees their new
wording in a fresh build is reading a compiler this printing does not
pin. §22.2's interface listing names `toolchain 0.2.11` and its hashes
did not move, which is what that page says a toolchain bump should
look like.

Chapter 5's trait section now covers generics whole, on the
maintainer's rule that a subject introduced once is covered once.
§5.3's `best` took a comparison function because nothing was known
about its `T`; §5.6 writes it again with the function gone, `best[T:
Ord]`, and reads `<` through `>=` and `<=>` off one `cmp`. Prefix `-`
dispatches through `Neg`, on the receipt's own refund. A bound is a
list, and `capped[T: Add + Ord]` uses both names in one body. One
paragraph says what no bound grants, in the compiler's own words:
`copy` needs none, and `!`, `&&` and `||` have no trait. Chapter 11's
bare `[S]` now cites that rule instead of teaching it. Every new block
runs on both machines.

Chapter 3 teaches the second spelling of `if`: `if leap then 29 else
28`, one expression a branch, closed by `then` and by the arm's comma
when it sits in a `match`. The page states the three rules and what
the formatter does, which is nothing, except break a bare `if` to
braces when its line outgrows the width, one way only. Exercise 5-7,
the RPN evaluator, is chapter 6's exercise 6-14 now, the first under
§6.2, because `?` on `pop()` is its point and chapter 5 had not taught
it; `words()` is named as the tokenizer.

Every bare statement `send` outside chapter 12 that the compiler names
as a dropped failure is spelled `?` now, or `else { … }` where the
function has no row to hand it to: twenty-five sites on pages across
seven chapters, forty-six in the exercise corpus. The task tails the
compiler does not name stay bare, and §10.1 says why beside the first
one a reader meets. The transcripts did not move, because every one of
them is the interpreter's, and the interpreter does not warn. One
finding is filed rather than absorbed: a bare integer literal handed
to a bounded generic is `i32` on the compiler, never `int`, so §5.6
calls its `refund` on a field and a binding and says nothing about
literals.

Measured at the pin, prediction first: 494 samples (251 book blocks,
243 corpus roots, 16 members), 489 passed, 5 pending, 0 flips; 439 of
462 console blocks replayed; 29 samples warn with no directive, down
from 59, and none of the 29 is a dropped send. One console re-record
was not predicted: a deadlock trap line that names its tasks by source
offset moved when a line above it grew, and the page reads the new
offset.

## bs42, the struct before the trait — 2026-09-11 — a name for the row, and the bound that grants `+`

A reader who reached chapter 5's trait section found it a leap, and
said why: the book had never introduced a struct, and the first
`struct` keyword it ever printed was inside a trait sample, beside a
`dyn`, a vtable, and an ownership rule quoted from the compiler. That
section is two sections now, and the first of them is short. §5.5
"Structs" takes the tuple §5.1 split a row into and gives it names:
`struct Row { kind: str, cents: int }`, the literal, a field read, a
field write through `var`, and the one judgment the page asks, which is
when a tuple is the better shape. Both machines run it.

§5.6 "Traits" opens where the need was, two sections up. §5.3 wrote
`total[T]` and the compiler refused it because nothing granted `+` on a
`T`; the compiler this printing pins says in its note what to add,
`T: Add`, and offers the edit. The section makes that repair and runs
`total` at `int` and `f64`, and the sentence the old section lacked is
on the page: the bound is what grants the operator, and `acc + x` on a
`T` is `Add.add(acc, x)` by another spelling. The trait, the impl and
the default body stay, on the receipt this time: a `Line` for a `Row`
and a `Header`, with one default that calls the method each type
supplies. Then the sentence that was false: `==` on a struct dispatches
through its `Eq`, and the page proves it with an impl that compares
kinds and ignores cents. The alias `Num` is shown for what it is, a
name for a list of traits, and the refusal a reader meets when a type
lacks one of them names the trait and never the alias, which is the
design question the section's exercise asks.

What did not belong in chapter 5 moved to chapter 7, where its
vocabulary lives. `dyn`, the two loads, the impl on a primitive with
the qualified call, and the cast-a-binding rule are a new §7.8,
"Deciding at run time", beside the section on what the machine does.
"A temporary has no home" is an ownership sentence, and the page reads
it in §7.2's and §7.3's words before the compiler says it in its own.
The two exercises went with the material and kept their questions.

Which machine runs which page is spelled on the page, as always. The
interpreter does not dispatch a struct's `==` through its impl and does
not parse the alias form, so those two blocks run under the compiler
and say so; everything else in the two sections runs on both. Three
things found on the way are filed rather than absorbed: a field of a
`let` binding is writable on both machines while the binding is not, a
one-line trait member is refused with a message that names the wrong
thing, and the interpreter runs a struct literal that leaves a field
out. The pin moves to the compiler that carries the operator bridge;
the catalog gains one code, the grammar one alternative, and the
appendices follow.

## bs41, the forty-eight — 2026-09-11 — a list you can slice, and a hole with a shape

Chapter 5 sliced strings three times and never once sliced a list, so
a reader who met slices in chapter 3 as a string thing had no reason to
try them on a list, and asked whether that was a rule. It is not.
§5.1 now slices a `List` beside the element it indexes — `xs[1..]`,
`xs[..^1]`, `xs[a..=b]`, the same three spellings, on a list of any
`T` — and says the one thing that is genuinely different: a `str`
slice looks at bytes it does not own, and a `List` slice is a fresh
copy. The block proves it rather than asserting it, by shortening the
original on the next line and printing three lengths. That next line
is also the chapter's one look at throwing away a fallible value on
purpose: `let _ = (mut names).pop()`, when the removal is the point
and the element is not. Exercise 5-3 grew a partner, `rest[T]`, which
is one slice and not a loop, and asks which of the two has the harder
empty case.

The rest of this entry is about the solutions manual rather than the
book, and it is one number: the exercise transcripts that nothing
replayed went from forty-eight to twenty. Twenty-four of them ran the
conformance probe, which the last sprint read as needing a lane the
book did not have. It needed one line removed instead. The probe
answers two audiences on two streams — a line of machine-readable
verdict, and the diagnostic a reader sees — and a page prints the
second. Drop the first and the rest of what the tool said is exactly
what the page claims it said. Four more printed an error code's
explanation, which reads nothing and touches nothing, and were
declined only because the rule had been drawn at "names a file".

Replaying those twenty-eight for the first time found eight pages
that did not say what the tools say. Two dropped a warning the
compiler prints above the error the exercise is about. Three were
abridged, and their own prose gave them away — one paragraph
describes five diagnostics over a block that printed one. Three were
text that had moved underneath the page: an explanation's closing
sentence, a rule that grew a fifth case (which made the surrounding
paragraph wrong as well as the block), and a suggestion that grew a
body. All eight are re-recorded from the real runs, and the two
paragraphs that had gone wrong with them are rewritten.

The twenty that remain each say why, in a sentence somebody wrote,
in a new ledger beside the two the book already keeps — one says "not
yet, anywhere", one says "not here, and here is what here says", and
this one says "not replayed at all, and here is what it would take".
The build fails if a transcript joins that set without a row, and it
fails if a row outlives the hole it explains. Eight want a file or a
package that was never checked in, six want something a person types,
six want a linked binary or a shell. That is the whole of it, and
none of it is invisible any more.

## bs40, the map alone — 2026-09-11 — a missing key is an answer

Chapter 5 has counted with two containers since it was written, and
the reason was never the receipt: no program could ask a `Map` whether
a key was bound, and a key that was not bound answered a value that
was not a value. So every tally carried a list beside the map, and the
list existed to say "have I seen this one" so the program could choose
between inserting and adding. A reader asked whether that was the
list's whole job, and guessed the count could be done with the map
alone. It can. The compiler this printing pins types the map, and a
lookup answers what `pop` on an empty list answers: the value when the
key is there, the mark `none` when it is not. The head of the chapter
is one container and one line of counting; the list returns in §5.2
for the one thing a map does not promise, which is the order the keys
arrived in, and the exercise that used to ask for both asks for the
count on the map and then asks which container gives you the order.
The old idiom, `m[k] += v`, is refused where it is written, and the
chapter prints the refusal: the compiler's note names the spelling
the chapter has used since its first program.

Three smaller things the same reader found are fixed with it. §5.3
said that indexing a list empties it; that is true of a list of an
unknown `T`, and the consequence is a refusal at compile time, so the
paragraph now says whose list, what happens, and that `let c =
cents[1]` on a `List[int]` copies the word and leaves the list whole.
Seven `0 - 1`s that meant `-1` read `-1`. And the `first` exercise
says in one sentence why it asks for a read: `pop` changes the list it
is called on, and a function that changes its argument is chapter 7's.

Which machine runs which page is spelled on the page, as always, and
this printing it points the other way for chapter 5: the interpreter
still reads a missing key as the old non-value and refuses to add to
it, so the tallies over new keys run under the compiler until the
mirror lands, and the two blocks whose keys are already bound run on
both. Elsewhere the interpreter caught up and the compiler caught up,
sixteen samples in seven chapters graduated to both machines, and the
runner is what said so, by name, before any fence moved. Two of them
could not graduate all the way and the pages say why: one prints
different bytes on the two machines because one of them has not taken
the ruling, and one has two conforming outcomes and had said so.

A warning the book teaches is a gate now. Chapter 12 prints one send
written bare, with the compiler's warning under it, replayed, and the
block declares the warning it expects; the build fails if the compiler
stops printing it or prints another. Every other sample that warns
without declaring it is counted and named in the log on each run,
so the inventory of dropped-failure sends the last printing filed is
read rather than remembered.

## bs38, the interpreter is taken — 2026-09-10 — two names that were never there

For four printings the book pinned an interpreter one release behind
the one it could have had, and the reason was two words. Chapters 11
and 14 printed `Scope` and `Proc` as the types of a scope handle and a
proc handle, and neither name has ever existed in either
implementation. The compiler said so from the beginning and was not
listened to; the interpreter used to accept the line without asking
what the name meant, and when it started asking, six of the book's
programs stopped running. Those two words were invented by this book.
They are gone.

What the specification actually promises is smaller and truer, and it
is what the two chapters now teach: a scope is an ordinary value, and
handing it to a function is visible at the call — which is where a
reader has to look anyway to find the brace the work dies at. The
signature does not announce it, because nothing in the language can
spell the type yet. Chapter 11 used to tell you that one search for a
type name would find every function in a codebase that can start a
task; it tells you to search the call instead, and says why in a
clause. That is a worse audit than the one chapter 7 offers for
mutation, and the book now says which of the two it is rather than
claiming they are the same.

Three notes the book keeps for itself said the opposite of all of this,
one of them for eight months and through three re-checks. Each re-check
measured the near-miss — the lowercase keyword — and never measured the
word the chapter actually printed. All three are rewritten with what
the tools answer, and the gap is filed against the language rather than
papered over.

With the two names re-homed the newer interpreter goes in, and the pair
of versions on the first page reads a fourth way: this time the
compiler names exactly the interpreter beside it, and the interpreter
names a compiler fifty-one commits behind the one above it — the first
printing where that second number has narrowed rather than grown. The
revision recorded for the interpreter is also, for the first time,
checked to be a commit rather than the label wrapped around one.

And 237 transcripts in the exercise collection are checked now. The
programs in that collection have been run on every commit for months;
the *output printed under them* was compared against nothing at all, so
a wrong answer could sit on a page indefinitely. One did, and was found
last printing by luck. Replaying them found seven more, none of them
anybody's fault and all of them wrong since the day they were written:
deadlock reports naming positions that shifted when a comment was added
above them, a number printed with a decimal point it never had, an
error message quoted down to its first line, a file path recorded from
the wrong folder. The rest of the collection's transcripts — the ones
that need a project on disk, an expression typed at a prompt, or a
compiled binary — are counted and listed by name every build, with what
each would need, so what is not checked is at least known.

## bs39 — 2026-09-10 — the query and the loop

Section 5.2 is rewritten, on a reader's report that the old one lost
them. It had a function that ignored its parameter and typed three
totals in by hand, on the page that teaches "a chain says what the
result is, a loop says how to make it", and it made the reader wait
fifteen lines and a paragraph to learn what the loop was for. The new
section asks one question of the table the chapter opened with (the two
categories that cost the most) and shows the two spellings of the
answer in order: first the chain, as the line it would be, then the one
sentence that names the trade (two selections against one sort), then
the loop that makes the selections, in a function that reads every
parameter it declares. The totals come from the receipt rows the head of
the chapter parsed, so the numbers on the page are the same numbers, and
the program is that program with one function added and one loop
changed.

Exercise 5-6 is one task now. It asked for `uniq`, adjacent duplicates,
two parallel lists and a question about `Map` in five lines; it asks
for the count-in-first-seen-order the chapter just did, with the same
two containers, and one question: why the `Map` alone loses the order.
The solution moved onto those containers and its output did not move.

The chain the section prints does not run on this toolchain, and the
page says so in the present tense: the std ships one combinator and a
sort is not among them. That is filed on the library this time, with
the chain as the witness, because what the library does ship is a sort
that works in place and returns nothing, and a chain needs a sort that
returns.

## bs36, the second pin bump — 2026-09-10 — the number that means something

This printing is true for a release again. The compiler's version is a
plain number with nothing stuck to the front of it, which is what a
build made at a release says and what no other build is allowed to say,
and chapter 1 and the colophon read it that way round. The commit
printed beside it is seven characters long because the toolchain now
picks that length on purpose. Last printing it picked the length from
how many objects your copy of the project happened to hold, so two
people at the same commit read two different lines; the book found that
by being wrong in public and the toolchain fixed it.

The two tools name each other as always, and this time the compiler
names an interpreter *newer* than the one beside it — the interpreter
published again between the compiler being tested and this printing
being pinned. That is a third arrangement in three printings: one
behind, then exact, now one ahead. The book keeps printing both numbers
as they are and keeps refusing to call any of it a first.

A number chapter 22 teaches has stopped lying. It is the digest of a
module's public face, and the honest answer to "can the people who
depend on me tell that I changed something". For as long as this book
has printed it, the compiler's own version went into it, so upgrading
your toolchain moved the number on a package nobody had touched — a
change-detector that fires when nothing changed. The book found that
two printings ago and wrote it down; it is ruled now, the version is
printed beside the digest instead of inside it, and both numbers move
once here for the last time. Chapter 25's line is the proof, as it has
been each time: of its three numbers, two hold to the byte and the
third does not.

Three programs' worth of paperwork closed with it. Writing into the
middle of a piece of text used to be met with "the compiler cannot do
this yet", which told a reader to wait for something that is never
coming; text does not change after it is built, and the refusal says
so now, in a sentence that also shows how to build the new text
instead. Chapter 3's exercise had already been given that rule in one
sentence last printing, on the strength of an argument rather than a
message. The message agrees with it now.

Something was wrong on a page and the build was green. An exercise in
chapter 22 shows the compiler printing a digest, and all three of its
moving lines had gone stale — the version by a release, both numbers by
the change above. Nothing caught it: the build replays every transcript
in the chapters and none of the two hundred and thirty-seven in the
exercise collection, on the reasoning that the programs there are run
directly, which is true of the programs and not of what the tools print
around them. It was found by searching for the old version number by
hand, after two other pages failed. That is written down as an issue,
because the next one will not be found by luck.

The check built last printing paid for itself twice in this one, and
neither time on the sites it was built for. Every version number
written into a sentence is now recorded with the version it was last
read against, and moving the pin fails the build until somebody
re-reads each one. It did that. It also caught two sentences this
printing changed by hand — a paragraph that swapped one interpreter
number for another and left the record stale in both directions, and a
new sentence that introduced a version number nobody had classified.
Neither was a pin moving. Both were edits, and a person would have
missed them.

## bs37 — 2026-09-10 — the book names the tool

The book has told you since chapter 2 that a formatter exists and leaves
the odd invisible byte at the top of your file alone, and it named that
formatter for the first time in chapter 3, in a clause, on its way to
somewhere else. There is a page for it now. The new appendix opens with
the whole of what the toolchain says about itself when you ask it — all
twenty-two of its subcommands, printed as a run rather than typed out,
so the list cannot fall behind the program — and then gives a section
each to the three you would otherwise never meet in these pages.

The formatter's section runs the tool. One file goes in with its `else`
keywords starting their lines and its braces in a column; the same file
comes back with the layout the specification fixes, and both texts are
on the page, so what "canonical" means is something you read rather than
something you take on trust. There are no options to compare, which the
command's own help says out loud, and the section says what the three
rules are that decided the shape — including why one of the two
functions in the example collapsed onto a single line and the other did
not.

The second section is the step between running a file and having a
project: one command turns a script into a package, and the transcript
shows the capability the script declared arriving in the manifest the
package now has, which is the part worth checking rather than assuming.
The third is the answer to a question chapters 23 and 24 raise and leave
open — what you build against when the place your dependencies came from
is not there. On a project whose dependencies are already beside it the
command says so and does nothing, which is the honest answer and the one
the page prints.

Two old defects closed with it. The syntax colors in the web edition
were painting the inside of a raw string as if the braces in it meant
something, three lines above a sentence saying they do not, and were
leaving a character literal inside an interpolated string as plain ink.
Both were reported when they were found rather than papered over in the
prose, both are fixed upstream, and this printing takes the fix. And
every page of the web edition had been asking for a print stylesheet the
renderer never wrote, so printing a chapter from a browser got the
screen's styling; the stylesheet ships now, one page's links stopped
pointing out of the book's own tree, and every internal link in the
render is checked on every build, which is what nothing had been doing.

## bs36, the pin bump — 2026-09-10 — the switch a reader expects

Chapter 3's first `match` now takes a range of numbers in one arm. A
reader who has written a switch in any other language reaches for that
on the first page they meet the construct, and this book's answer has
been a parse error whose message did not contain the word "range" —
recorded unchanged at every pin since chapter 3 was drafted. The language has ranges in an arm now — from a number up to
another, or up to and including it, with a plain number at each end,
and letters ordered the way the alphabet is — and the message for the
open forms says which spellings are patterns and which are not. Both
halves of what the ledger asked for landed, which is not how these
usually go: either the grammar grows the thing or the message explains
why it has not, and this time it was both.

The section's example carries three shapes of arm beside the catch-all
now: a plain value, a range, and a test on the value it has named.
That last one is the arm every switch-shaped program in the world ends
with, and until this printing it compiled everywhere except the
machine that turns programs into binaries.

One tool serves that page and the other does not, so the page says so:
the block is labelled a compiled run, the transcript beside it is the
compiler's, and one sentence names the machine. The other tool is
working on it, the build fails the day it catches up until somebody
removes the label, and the ledger carries the debt with an owner in the
meantime. This is the second time chapter 3 has been in this position
and the first one closed one printing later.

Two programs were predicted to start working on both tools at this
printing and neither did, and the reason is the useful part. A rule
landed that turns one kind of dropped value from an error into a
warning — the kind where nothing was produced at all. Both programs
throw away a value that IS produced: something was taken out of a list
and nobody caught it. The prediction read the rule one word wider than
it was written. Every program in the book that runs on one tool and not
the other — a hundred and one of them — was measured against the
compiler before the guess was made and again after, which is the only
way a wrong guess becomes a fact instead of a shrug.

A send on a channel can fail, and until this printing the compiler
would not admit it. It does now, and the consequence reaches every
line in chapter 12 that sends without looking at the answer: each one
was throwing away a failure, and the compiler says so. All twenty of
them ask for the answer to be handed up instead, which is the honest
spelling and the one the chapter already taught six chapters earlier.
The warning is explained once, beside the first of them, rather than
at each.

Chapter 30 was not on this printing's list and got the same edit
anyway, because the build refused to pass without it. The pages that
show a program being compiled compare every byte the tool prints, so
four new warnings in a transcript are four failures — which makes that
comparison the only thing in this repository that can see a warning at
all. Ten chapters still have that sentence somewhere and nothing
will catch them; that is written down as an issue rather than swept.

The compiler this printing is graded on is not a release. Its version
says so itself, with the commit it was built from stuck to the front,
and chapter 1 and the colophon read it that way round again. The two
tools name each other as always, and this time the compiler's name for
the other one is exact — the same release that sits beside it — while
the other one names a compiler forty-three commits back. Both numbers
are read off the tools rather than typed.

One character in the compiler's own version line is not a fact about
the compiler at all. It is the short form of the commit the build came
from, and how short git makes it depends on how many objects the copy
of the project you cloned happens to hold. Two copies of the same
project, at the same commit, print two different lines. Every check on
this machine passed and the build machine — which clones fresh, the
way a reader does — disagreed by one character. The page prints what
the fresh copy prints, the transcripts were re-recorded rather than
retyped, and the compiler has been asked to choose a length on purpose
instead of inheriting one.

The last thing about this book that was kept by hand now checks
itself. A version number written into a sentence — the name of a
download, or the line about what a build says when nobody stamped it —
has no program behind it, so nothing in the build could tell whether it
was still true, and five printings running somebody remembered to
change it. Every one of them is written down now with the version it
was last read against, on the two clocks the two tools release on, and
a printing that moves past one of those readings fails the build and
names the sentence. Nine tests plant the mistakes it is meant to catch,
including the one that actually happened.

The interpreter did not move, and that was decided rather than
defaulted. A newer one is published, and running the whole book
against it turned up twelve failures, ten of them one thing: two type
names this book prints in chapters 11 and 14 name nothing in either
tool, and the older interpreter accepted them only because it never
looked. The compiler has been refusing them all along, which is why
those pages already said they run on one tool. Taking that interpreter
is six programs to rewrite and a question to answer about what those
two names are supposed to mean, so it is a printing of its own and it
is written down as one.

## bs35, the pin bump — 2026-09-10 — the book is true for a release

Until this printing the compiler this book was graded against was a
build off the project's trunk, and it said so: a version number with a
commit stuck to the front of it and no claim to being a release. This
printing is graded on a release. The number is plain, and a compiler
unpacked from its download answers the two lines chapter 1 prints,
character for character. The paragraph that used to warn a reader that
the tool they downloaded would answer something older is gone, because
it would now be false.

Two exercises in the chapter on moving work between tasks run on both
tools. A `return` written inside a small anonymous function returns
from that function rather than from the one around it, which is how
both tools have always run it and what only one of them refused to
accept. The build found both, in the same run, and said so in its own
words: a program declared as running on one tool that starts running on
both is an error until somebody removes the declaration.

Five other programs in that family did not move, and the reason each
one did not is worth more than the two that did. Two of them turn out
to take a value out of a list and then use the list, which the book
already teaches under a different name three chapters earlier. Two
more write a bare `return` where the surrounding code produces nothing,
which is a different complaint with a different answer. The last one
compiles clean and then parks: it is a deadlock exercise, one tool
stops it and the other is entitled to wait forever, and the
specification says so. The number the book publishes for programs that
run on one tool is ninety-one, and it was arrived at by measuring every
one of them again rather than by subtracting two from the last count.
Counting again is the only way any of the above is visible.

Neither of the two version lines is exact this printing. The compiler
names an interpreter one release behind the one beside it; the
interpreter names a compiler fourteen commits before the release above
it. Both numbers are read off the tools rather than typed, chapter 1
and the colophon say which is which, and neither page calls the shape a
first, because this book has been wrong about a first before.

A string can take a single character on the end of it now, with the
plus sign, the way it always could take another string. The book does
not use it. Four solutions build a string one character at a time
through the older spelling, all four run on both tools today, and
writing them the new way would make all four run on one — the other
tool has not made the change. The older spelling is not a workaround
being taught as the way; it is the spelling that is true on both
machines, and the day that stops being so is the day these four
paragraphs get shorter.

Something moved that nothing in this release touched, and the build
caught it. Chapter 22 teaches a number the compiler prints for a
module's public surface — the honest answer to whether a change is
visible to the people who depend on you — and chapter 25 spends that
fact on version numbers. The number moved at this printing on a program
nobody edited, because the compiler's own version is part of what it
digests. Chapter 25's transcript shows it cleanly: two of its three
numbers hold and the third does not. Both transcripts on these pages
are the real runs, the finding is written down at full volume where
findings go, and it is filed with the compiler rather than papered over
here.

Two claims this book makes about itself now check themselves. The
count of diagnostic codes in the appendix, and the copy of the
compiler's catalog the appendix is checked against, were both moved by
hand at every printing that touched them, and the last two printings
said in as many words that a green build is not a checked claim. It is
one now. The count is arithmetic on the catalog, the catalog is derived
from the compiler's own document the same way a refresh derives it, and
each check has a test that plants the exact defect that went unnoticed
for the life of this edition. The catalog did not move at this
printing, which is the least convenient moment to build a gate and the
most honest one.

## bs34, the second pin bump — 2026-09-09 — an empty list answers

Taking something out of an empty list used to stop the program. It
hands back "nothing there" now, and the difference is the one this book
has been making since chapter 1: a fault is the program being wrong, and
an error is an answer the program should have an opinion about. An empty
list is the second kind. Chapter 5 said the opposite for eight
printings, and the sentence that said it is gone.

The four reads that can come up empty behave the same way now — take the
last one, ask for the one at an index, ask for the first, ask for the
last — and none of them stops the program. Reaching in by subscript
still does, and the chapter says why the two are different: an index the
program computed and got wrong is not the same event as a lookup that
found nothing.

The build did not discover that one, and could not have. The check that
notices a program has started working asks the compiler for a verdict
without running anything, which is enough to see a program stop being
refused and not enough to see it stop stopping. So this change arrived
as a broken test rather than as a graduation, which is what last
printing's notes said would happen, in writing, before it could. The
page was fixed by hand and the note that predicted it is now the note
that records it.

The aligned branch layout from last printing runs on both tools now, so
the label saying which tool ran it is gone — and that one the build did
find, in the same run, and said so in its own words. Two changes, one
release, and the machinery told the difference between them without
being asked.

Chapter 12's closed-channel error prints in lowercase on both tools at
last. The program around it still runs on one, and the reason has not
moved: the rule says a send on a closed channel hands back an error, and
on the compiler a send cannot hand back anything. That is one open
question, named on the page, and it is not this printing's to close.

The two version lines have swapped which one is exact. The interpreter
now names the compiler above it character for character, which is as
close as a pair cut on two schedules gets; the compiler runs two
releases behind the interpreter beside it, for the ordinary reason that
the interpreter published twice since. Both numbers are read off the
tools rather than typed, and neither is called a first, because this
book has been wrong about a first before and wrote that down too.

## bs34, the pin bump — 2026-09-09 — a line may start with `else`

The compiler stopped rejecting a layout people kept writing. An `if`
with its branches lined up under each other, the `else` beginning its
own line, was a syntax error until this printing; the rule came from Go
and the reasoning was that a newline ends a statement, so an `else`
below the closing brace belonged to nothing. A reader who came from C or
Python or Rust met that rule as an error message on their first branchy
program. It is gone. A line whose first token is `else` continues the
statement above it, and chapter 3 shows the aligned form and says the
one thing a reader needs alongside it: both layouts parse, and the
formatter writes them the same way. The error code for the old rule has
been retired, which is the first time a code has left the catalog in
this edition's life, and the diagnostics appendix counts one fewer.

The interpreter has not made that change yet, so the aligned program in
chapter 3 says it was run by the compiler, the way chapter 1's error
program said so last printing. That label is what the build removes on
its own the day the two tools agree.

Chapter 1's error program is that day arriving. Last printing the two
tools spelled the same failure differently and the page carried the
compiler's transcript with a note; the interpreter spells it `parse`
now, both tools print the same line, and the note is gone. The build
found it rather than a person: a program running on one tool that starts
running on both is an error until somebody removes the label. It was the
only such graduation this printing, and it was named in advance — eleven
programs in this book run on one tool by declaration, and the ten others
are waiting on things this release does not touch.

Two arguments the book had been reporting without taking a side were
settled, and both were settled the way the compiler already behaved. The
error a closed channel hands back has a name in the specification now,
lowercase like every name that carries nothing, and chapter 12 says the
rule names it instead of saying the rule declines to. What a list hands
back when you pop from an empty one is ruled too: an ordinary "nothing
there", never a fault. Chapter 5 still shows the fault, because the tool
that runs that program still produces one, and its notes now say which
printing removes it and that the build will fail rather than quietly
adjust when that happens. Predicted in writing before it can occur.

One thing on this page is worth saying plainly because the build cannot
say it. The count of diagnostic codes in the appendix is checked by
nobody: the tooling verifies that every code the book shows is real and
that every code the book mentions is listed, and never that the number
in the sentence matches the catalog it describes. The vendored copy of
that catalog is not compared against the compiler either. Both were
moved by hand here and both are filed, because a green build is not the
same thing as a checked claim, and this book's whole argument is the
difference between them.

## bs34 — 2026-09-09 — the vocabulary goes lowercase

Chapter 6 teaches that a failure is a value with a name, and the names
it taught were spelled the way the language says not to spell them. A
name in capitals promises there is data inside it; every name in this
chapter carried nothing. The compiler had been saying so, as advice
rather than a refusal, on nine of the chapter's printed programs and on
six of its solutions, so a reader who followed §1.2 and built with the
compiler met warnings the chapter never mentioned. The names are
lowercase now: `no_comma`, `empty`, `bad_shape` and the rest. The
paragraph that introduces the first one says why in a sentence, and the
exercise that declares a name of its own no longer has to explain the
ones above it.

Chapter 10 has one such name and it moved with them. Chapter 27 keeps
its capitals on purpose: that program prints the warning in its own
build and tells the reader to read it and decide, which is a teaching
point rather than an oversight. A book that never shows the warning
cannot ask the question.

One name did not move, and the rule that stopped it was not written
down on this page before. A row that ends in `..` may hand back names
its signature never lists, and out there the capitals are load-bearing:
with no signature to name it, a name's case is the only thing that says
it is a failure and not a variable. Write it in lowercase and the
compiler looks for a variable, finds none, and says so. The open row
keeps its capitalized name, the compiler has no complaint about it, and
the paragraph that opens the row says which of the two rules applies
where.

Counted the same way before and after: every program in the book and
every published solution run through the compiler, and the warnings it
printed added up. Fifteen stood on eleven printed programs last
printing. The chapter's share is zero now, and the only printed program
left that draws one is chapter 27's, twice, on purpose. Twelve
solutions in six other chapters still carry capitalized names that keep
nothing; they are those chapters' vocabularies and not this printing's
to rename.

## bs33 — 2026-09-09 — the mark is `parse`

An error in this book changed its name, and the page that prints it says
which tool printed it. Converting text that is not a number has always
produced an error rather than a crash, and the compiler used to call
that error `NotAnInt`. The language has a rule about how such names are
written — a name in capitals promises there is data inside it, and this
one carries nothing — and the compiler's own advice had been flagging
the name for as long as it had been spelling it. The specification
settled it: the error is `parse` now, after the family of conversions it
belongs to. Chapter 1 prints it, so chapter 1 moved.

The interpreter has not made the same change, so for the moment the two
tools print different words for the same error, and the book does the
thing it built the machinery for rather than the thing that would have
been easier. The block in §1.5 says it was run by the compiler, the
transcript beside it is the compiler's, and the sentence that reads the
result names the error. The day the interpreter agrees, the build fails
until somebody removes that label — the rig treats a program that starts
working on both tools as an error, on purpose, so nobody has to remember.

This printing is true for a compiler that is not a release. Both version
lines say so themselves: a build made exactly at a release prints a plain
version number, and every other build prints the commit it came from and
claims nothing. §1.2 and the colophon read the stamp that way round now,
and §1.2 says the rest of it beside the download links — a compiler
unpacked from the last release answers with the old name and the plain
version. That is the cost of printing a rule the day it was made, and it
is stated where a reader will hit it rather than in a note at the back.

Thirteen programs went back to running on both tools, and the build
found every one of them. Nine were predicted before anything ran, from
the toolchain's own account of what it had changed: displaying a value
inside a string now works for values that are not numbers, and walking a
channel a function was handed now works at all. The other four were not
predicted, and they are the useful part — a third family of programs had
quietly started working too, eight down to one.

A fourteenth was found by hand, and the reason it had to be is now
written down. The check that notices a program has started working asks
the compiler for a verdict without running anything, which is enough to
see a program stop being refused and not enough to see it stop faulting.
Chapter 5's program that pops from an empty list is the second kind: the
compiler now runs it and answers with an ordinary "nothing there" where
the interpreter stops the program. Nothing in the specification says
which is right, so the chapter keeps the fence it had and the notes say
why. Found by re-measuring all one hundred and seven one-tool programs
against both compilers and comparing them one at a time, which is the
only way this class of change can be seen.

Exercise 6-9 declares an error of its own, and it is spelled in
lowercase now for the same reason the compiler's is. The exercise gained
two sentences: what the compiler expects of a name that carries nothing,
and that it says the same thing about names earlier in the chapter and
says it as advice rather than a refusal. The larger question — chapter 6
teaches a whole vocabulary of these names, and §27.5 already prints the
warning and tells the reader to read it and decide — is written down as
a question rather than answered by a lane.

Chapter 12's closed-channel error is spelled two ways by the two tools
and by no clause at all. The section now says which tool printed the
word on the page and what the other one writes, and stops short of
choosing, because that is the specification's to choose. Measuring it
turned up something bigger underneath: the rule says a send on a closed
channel hands back an error, and on the compiler a send cannot hand back
anything. The book teaches the rule from the tool that implements it and
says which tool that is.

## bs32 — 2026-09-09 — the two tools name one revision

The interpreter has caught up. Last printing the two version lines
disagreed about which revision the pair was tested against, and the
disagreement was the interpreter's: it named a compiler three releases
behind the one printed above it. lupin 0.1.28 names `5c729e8`, which is
the compiler on the line above, character for character. The compiler's
line now runs one release behind the interpreter beside it, for the
ordinary reason a pair cut on two schedules disagrees at all: the
interpreter published after the compiler was cut. §1.2 and the colophon
say that and nothing more. The pair has worn this exact shape before, at
wolf 0.2.4 and lupin 0.1.25, which is recorded in the pin file so that
nobody prints it as a first.

Nothing else moved with it, and that was predicted before it was
measured. The pin file carries the prediction and the reading together:
zero programs changed which machines serve them, in either direction and
in the pending list, because only the interpreter moved and nothing this
release adds is on a page. Four hundred and seventy-eight samples, four
hundred and seventy-three passing, five pending, none failing. The two
version transcripts are the whole diff a reader can see.

One divergence closes without the book noticing, which is the point of
having recorded it. Last printing's pin notes wrote down that converting
a number too large for the compiler's integer type produced different
answers from the two tools, and that no page in this book asked for one.
The interpreter has fixed it. Re-checked here rather than taken on faith
from the sentence that recorded it, and still no page asks for one.

Chapter 5's generic `best` is legal wolf now, and the word that fixes it
is one the reader already met. Indexing a list hands the element over,
so a helper that reads a list twice has emptied it by the second read;
the compiler says so and suggests `copy`. That is the same suggestion
chapter 3 prints in full three chapters earlier, in the same diagnostic,
from the same tool, with only its cost held over for chapter 7. So
`copy` does not arrive in Part 1 here. It arrived in chapter 3, and
chapter 5 is the reader using it. Both spellings that fix the program
were run on both machines before the choice was made, including the one
that avoids the word; the one that avoids it teaches worse.

Exercise 5-7's answer was describing a program it did not have. The
published solution pops two values off a stack and adds them, and
popping a stack that might be empty produces a value that carries the
possibility of failure. The compiler refuses to add two of those; the
interpreter allowed it. The answer paragraph beside the code had already
claimed the program returned an error rather than trapping, which is
what it does now: each pop handles its own failure, and the answer says
why the guard above it does not excuse that. Found by re-measuring the
chapter rather than by subtracting from last printing's count, and it
had been hidden under the refusal the last toolchain bump removed.

Two of the hundred and six programs that run on one machine are back to
running on both, and neither came from a toolchain move: the book was
wrong about them and is not any more.

The spec cross-reference checks itself now. Appendix D routes a reader
from a clause tag in a tool's output to the normative text behind it,
and it does that by counting the specification's shape: how many
documents there are, how many anchor namespaces they publish, and which
belong to which. Two printings ago that page said seven documents when
there were eleven, and one printing ago it left out a namespace whose
tags a tool prints at readers. Both were caught by somebody reading the
page. The build derives all of it from the specification's own registry
now, in both directions, and a page that has drifted fails the build and
names the sentence.

## bs31 — 2026-09-08 — the first chapter is true

A reader copied chapter 1's tenth exercise, ran it with the compiler,
and got a refusal. The program was correct and the book was wrong about
it: converting a string to a number is one of the things the reference
interpreter does and the compiler does not, and the chapter had taught
it three times without saying so. It says so now, in three places, and
the machinery that let the book print a claim like that has been rebuilt
underneath.

The runner asks both machines. Every runnable program in this book
carries a directive saying what it should do, and until this printing
the runner tested that against the interpreter alone. The compiler was
never asked, so a program only one of the two implementations serves
could sit on a page, green, for as long as nobody typed it. The
directive now means what it always said: both machines run the program,
both must reach the same exit, and both must print the same bytes.

Asking the second machine for the first time cost the book its
comfortable number. Of 478 samples, 133 run on the interpreter and not
on the compiler — 28 percent, across nineteen chapters. None of them is
new; they have been true for as long as the pages have existed. Each one
now carries a directive naming the machine that runs it, each renders on
the page as an interpreted run rather than a plain one, and each chapter
records what it owes and why in its own notes. They turn back into
ordinary two-machine programs on their own, and loudly, the day the
compiler grows the construct: the runner treats a sample that starts
agreeing as an error until somebody updates the page.

Two of the 133 were not gaps but mistakes, and both are fixed. Chapter
30 printed a `select` between two ready channels and claimed which one
wins; the language makes that choice the scheduler's, seeded, and the
chapter's own prose said so two paragraphs below the block that
contradicted it. And chapter 5's `best` helper is a program the compiler
holds illegal and the interpreter runs — one word fixes it, and which
word belongs in a Part 1 chapter is a question the chapter gets to
answer rather than the build system.

Chapter 1 says which machine. The receipt in §1.1, the error verdict in
§1.5, and exercise 1-10 all run under `lupin`, and each of them now says
so in the prose where you meet it, with a sentence in §1.1 introducing
the convention the rest of the book follows. Nothing about the programs
changed. What changed is that the page no longer implies a second tool
will do the same thing.

The chapter that started this compiles. The toolchain moved twice in
this sprint, and the second move closes the report it opened with: wolf
0.2.8 puts string-to-number conversion in the compiler, spelled the way
the interpreter has always spelled it, so chapter 1's receipt, its error
verdict, and exercise 1-10 all run under either tool and the sentences
saying which machine to use are gone. Twenty-seven programs across five
chapters went back to being ordinary two-machine programs, and the build
system is what noticed: each one was reported as an error saying the
directive was now understating what the program does. Nobody re-read the
chapters looking for them.

A hundred and six samples still run on one machine. That number will
come down the same way.

The serving chapter can now say what a request costs. A program that
reads a socket the operating system has already reported ready used to
hand the work to a background thread and wait to be woken, on a socket
it had just been told was ready; it now makes the call and waits for
nothing. Section 33.3 gives the measurement, taken on one machine in one
session against the two compilers: 53.3 microseconds per request before,
21.2 after.

The earlier move, to wolf 0.2.7. The release
re-stamps which interpreter the compiler was tested against and adds a
gate that compares the two on a build machine rather than only on a
desk. Nothing about the language moved with it. Two things on the page
did: a failed build now names the code it reported rather than an
example code, and the specification publishes seven scheduler clause
anchors it had declared and never registered, which Appendix D records.
The colophon's two version lines are worth reading together this
printing: the compiler names exactly the interpreter beside it, and the
interpreter names the compiler from three releases back.

## bs30 — 2026-09-07 — the book serves

This edition has a chapter about writing a server. Chapter 33, The
serving loop, is the book's first transport and its first executed
socket call: bind a loopback listener and take one connection, ask
`net_wait` which of a whole set of sockets can be read, serve several
callers from one process with no task and no channel anywhere in it, and
then hand that listener to a child process and read what a lost accept
race costs. Seven exercises, five of them with programs, all solved and
all replayed by CI in the same commit as the prose. The suite goes from
462 samples passing to 473.

The paragraph the last printing measured and could not place is now a
section. Last time this book recorded, in its own notes, that a loop
which waits costs about what a program that only sleeps costs, while a
loop that takes turns (a short deadline on the door, another on every
open connection, around and around) is descheduled about fifteen hundred
times in five idle seconds to learn that nothing happened. No chapter
held that argument. §33.3 holds it now, and it cites §12.2's costing of
an idle connection, because the two are the same argument about two
different machines: one about the concurrency runtime, where an arm in a
wait set is a registration and not a task, and one about the kernel,
where the program spawns nothing at all.

Where the chapter sits, and the number that will look wrong. Chapter 33
is at the end of part 4, between chapters 25 and 26. Section numbers in
this book are permanent links (`#8.4` is section 8.4 for good), so a
chapter that arrives after the other thirty-two were numbered takes the
next free number rather than moving everyone else's. The alternative was
renumbering fourteen chapters, every section anchor under them, and
every exercise number in part 5, which is the trade the language
specification makes the same way and for the same reason. "How to read
this book" says so in one sentence, and nothing else in the book was
renumbered.

The accept is fair, and the chapter says when that became true. Several
processes can hold one listening socket; one connection wakes more than
one of them and exactly one takes it. Until the compiler release this
printing is true for, the hands that lost went back into a blocking
accept with their budget already spent and stayed there until the next
connection arrived: microseconds on a busy server, and on a quiet one,
never, alive, using no cpu, answering nothing. The advice for that
arrangement was to not build it. Now a hand that loses comes back inside
the budget it already armed, so §33.4 teaches the mechanism instead of
the warning, and says in past tense that it could not have.

What serves everywhere, and what two hosts decline. Two of the chapter's
calls are the host's to decline: sharing one address across processes
with `reuse_port`, and passing a descriptor to a child, are refused on
Windows for reasons the section states. The rest of the chapter is
portable (listen, connect, accept, read, write, close, arm a deadline,
ask how many cores you may be scheduled on, and wait on a set), and
`net_wait` in particular names no refusal on any host at all, which is
why the chapter is built on it.

Appendix D counts the specification correctly again. The eleven
documents publish eleven namespaces between them, and one document owns
two: the grammar owns both its own anchors and the diagnostics namespace
a tool prints at you, which had no row on the one page that routes a
reader to the normative text. The schedule-points document is the other
correction: it declares anchors the specification's registry does not
carry, so a search for one finds the document and not the anchor, and
the table says so instead of implying a prefix that can be looked up.
Chapter 33's five clause citations are in the section table now, so the
operating-system document is reachable from a page and not only from a
tool's output.

The toolchain moves to wolf 0.2.6 and lupin 0.1.27. The two version
lines disagree twice this printing, and both disagreements say the same
thing: the compiler names the interpreter release before the one beside
it, and the interpreter names the compiler release before the one above
it. The two were cut within a day of each other and neither had seen the
other's latest when it was tested. That is a pair one release apart,
wearing the lag on the name in one direction and on the sha in the
other, and §1.2 and the colophon both say which is which. Four
transcripts re-record and no diagnostic snapshot does; the diagnostic
catalog holds at 169 codes and the grammar is byte-identical, so
Appendix A regenerates to itself.

## bs29 — 2026-09-06 — the cores land outside the book

Both halves of the toolchain move this sprint, the compiler to v0.2.5
and the interpreter to lupin 0.1.26, and between them they close the one
gap the previous printing recorded and open a surface no page in this
edition can reach. The suite does not move at all: 462 passed, 5
pending, 0 failed, 0 flips, the same reading as at the pins before it.

The byte's domain is a rule on both machines now. Last printing recorded
a hole with no page in it: lupin had the `byte` type and not its range,
so `push(256)` into a `List[byte]` stored 256 and printed it where the
compiler refused the same line. lupin 0.1.26 refuses it (`E0401`, at the
`256` and not at the list it was going into, the same column the
compiler underlines), and the prediction that went with the hole was
re-run at its closing, because a gap closing can move a page as easily
as a gap opening. The corpus reason is unchanged: this book pushes into
no `List[byte]` anywhere, writes no `byte` annotation over an integer,
and hands a `List[byte]` only to a parameter declared over one, so the
only int-to-byte flows on any page are §2.3's explicit casts, which the
rule excludes by clause. Predicted zero at the opening, measured zero.
Predicted zero at the closing, measured zero.

The release's headline lands entirely outside this book. v0.2.5 is the
release where a server becomes a program you can write in wolf:
`net_wait` waits on a whole set of sockets at once, `net_listen_with`
lets several processes hold one address, `os_spawn_with` and
`net_adopt_listener` hand a listener down to a child, and `os_cpus` says
how many hands the machine will actually schedule. Five clauses, and not
one of them reaches a printed page. This edition makes no socket call
and no process call anywhere: three operating-system builtins are
executed on any page (`fs_read_text` and `fs_write_text` in Part 5's
projects, and a `net_fetch` that appears three times and is refused all
three), and three more are named in an answer and never run. So the
surface is recorded where this book records a toolchain fact it does not
teach, in the pin file, and Appendix D is corrected so that a reader has
somewhere to go for it.

A loop that waits against a loop that looks, measured here and printed
nowhere. The argument behind `net_wait` is the one a serving loop is
written from, and the measurement was taken on this repository's own
programs instead of quoted from the release: hold one connection open
and idle for five seconds, and a loop that time-slices a 25 ms deadline
on the listener and a 12 ms one on the connection makes 130 passes and
is descheduled about 1,560 times to learn that nothing happened, where
one `net_wait` over the same two handles makes one pass and is
descheduled about ten times, which is what a program that only sleeps
for those five seconds costs. No chapter in this edition holds that
paragraph. The nearest is §12.2's "The million idle connections", which
asks this question and answers it about the concurrency runtime, over
channels and timers, and hands throughput to Part 4; `net_wait` is the
same argument about a different machine, the one a program that spawns
nothing uses instead of a scheduler, and writing it there would put this
book's first socket call three sections after `select` in a chapter that
teaches no transport. The paragraph is routed instead of written: it
goes to whatever chapter this book eventually gives the network, which
is the editorial call chapter 11's ledger has carried since bs27 and
which this sprint does not make.

Appendix D said the specification is seven documents. It has been eleven
for the whole of the 0.2 line. The four that were missing are Packages,
Constant-Time, Types and the OS surface, and the last of those is where
every call named above is ruled, so the one route this book gives a
reader out of its own pages and into the normative text did not reach
the release's whole subject. The table names all eleven now, and says
that there is no builtin reference in this book and no appendix that
lists the surface.

The two version lines name one interpreter again, and the lag has moved
onto the sha. Last printing they disagreed by name: the compiler said
`paired with lupin 0.1.24` while the interpreter beside it was 0.1.25.
This compiler was tagged after this interpreter existed, so it names it,
pin clause and all. The interpreter, cut first, was tested against the
compiler before this one, so the sha it carries is the previous tag
rather than the line above it. A pair one release apart is the ordinary
case and it can lag on either side. Both transcripts re-record, and the
colophon and §1.2 say which side it is on this time.

Those two version blocks are two of the four the pin move touched. The
other two are stamps that carry the toolchain version by construction:
chapter 22's `wolf interface` output and chapter 25's publish record,
with 22-13's pair of runs re-measured beside them. Nothing else in the
book moved, no diagnostic snapshot re-records, the diagnostic catalog
holds at 169 codes identical line for line, and the grammar is
byte-identical, so Appendix A regenerates to itself. The clause anchors
grow 417 to 422 (the five the new calls are ruled by), with nothing
dropped and nothing retargeted.

The print edition holds at 513 pages.

## bs28 — 2026-09-03 — the ladder lights

The compiler does not move this sprint. The interpreter does, from lupin
0.1.24 to 0.1.25, and what it brings is the half of last sprint's
release that had not reached it yet: `byte`. One release ago this book
taught a scalar that only one of its two machines could read, and it
wrote that down where it records what it measured. Both machines read it
now, and the two blocks that were waiting are executed on both.

§2.3's byte transcript is byte-identical across the pair. `65 65 200`,
`255 0 44 255`, `400 66 66`, exit 0, under `wolf run byte.lu` and under
`lupin byte.lu` alike, measured at the bump, before the fence was
touched. So the page shows one transcript and not two, which is the
book's rule for a program whose machines agree, and the fence moves out
of the compiler-only lane into the shape §2.4 has used for `char` since
bs17: the interpreter runs the program, the compiler runs the console
block beside it, and both readings have to match the same three printed
lines. The same graduation happens two hundred pages later, where §8.9's
byte-ledger reading answers `true` to all three of its relations on the
interpreter too.

A third fence went with them, and it is older than the byte. Chapter 4's
trap-abandons-your-defers program has been the compiler's block, and its
paragraph has said "both machines do this now" since the interpreter's
divergence was fixed, but nothing was checking the second machine. It is
checked now: lupin names the same `assert` and exits `3` where the
compiler exits `134`, which is the per-machine status D60 rules and the
same kind either way. Three fences into the two-machine form, and one
printed block is left that the compiler runs alone: chapter 30's
parallel grep, which writes files. The exercise corpus keeps eight more,
five of them comptime folds the interpreter declines by design and three
of them chapter 30's.

§8.9's byte ledger exists to prove that holding octets as `int`s costs
real memory, and it prints relations because the units belong to
whichever arena you ran in. Running it on a second arena demonstrates
the argument: the same 65,536 octets that charge 65,584 ledger bytes
compiled charge 65,568 interpreted, and the same values pushed into a
`List[int]` charge sixteen times the octets on one machine and
thirty-two on the other. All three printed relations hold on both. The
section names both multiples now, and §2.3's one-sentence version of the
argument stops quoting a single machine's sixteen.

The two version lines no longer name the same interpreter. The compiler
was tagged before this interpreter release existed, so `wolf --version`
still reports being paired with lupin 0.1.24 while `lupin --version`
reports 0.1.25, pinned, in its own stamp, to the exact compiler revision
this book pins. The colophon has carried a sentence since the first
edition saying a printing whose two lines differ by a release is
ordinary; this is that printing, so the sentence states a fact instead
of anticipating one, and §1.2 gains a paragraph telling the reader how
to read a pair that disagrees. Those two transcripts are the whole of
the bump's blast radius: at the raw new pin, before a line was healed,
the suite reported 462 passed, 5 pending, 2 failed and 0 flips, and the
two failures were those two blocks. Nothing else in the book moved.

A gap in the interpreter that no page can reach. lupin 0.1.25 has the
byte type but not its domain: `push(256)` into a `List[byte]` stores 256
and prints it, where the compiler refuses the same line. That is filed
as wolf-interp#62 and it was predicted to touch nothing here before the
suite was run: this book pushes into no `List[byte]` anywhere, annotates
no `byte` from an integer, and hands a `List[byte]` only to a parameter
declared over one, so the only integer-to-byte flows on any page are
§2.3's four explicit casts, which truncate by clause and agree on both
machines. Measured after: nothing. A program the compiler refuses is not
one this book can print, so the gap is recorded in the pin file and on
no page.

Two more claims narrow because a second machine can finally be asked.
D74's string-layout codes reached the interpreter with this release, so
lupin answers `E0104` on §2.2's own program where it answered an
invented `E0109` one release ago: the same line, the same code the page
prints, in its own words. Appendix C still says the block was shown by
the compiler, because it was; the reason last sprint gave for that has
retired. And chapter 11's connection-pool row, which lost its premise
last sprint when the toolchain grew a network surface, loses its
replacement clause here: the interpreter serves the unix-domain family
too, measured on this host, over listen, connect, accept, the byte read
and write pair, and a listener close that unlinks its own path. The row
stays open on the editorial call it has always rested on: this edition
has no network chapter, and no page makes a socket call.

The print edition holds at 513 pages.

## bs27 — 2026-09-03 — the scalar table grows

The pins move to wolf v0.2.4 and lupin 0.1.24, and the language has a
new scalar in it. A *byte* is one octet (eight bits, unsigned, `0`
through `255`, one byte of storage), and every builtin that hands you
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

Every octet fits an integer with seven bytes to spare, and those seven
bytes are not the whole price. §8.9 now measures the whole of it instead
of asserting it. A region holding 65,536 octets charges 65,536 octets
and one list header (the runtime knows the length before it allocates,
so there is no growth history to pay for), and the same 65,536 values
pushed into a list of integers charge at least seven times that, and on
the machine this printing was built on, sixteen. The section prints
those as relations rather than as numbers, the way it prints every other
ledger reading, and the sentence in its budget half that warned "a
sixty-four-kilobyte buffer's worth of elements can charge a megabyte of
ledger" now points at the measurement two paragraphs above it, which is
that megabyte.

Chapter 2's multiline strings gained their refusals. §2.2 has stated
three layout rules since the first edition and enforced none of them on
the page; each has a code now, one rule per code, and the margin rule is
printed in full because its rendering shows both ends of the comparison:
the line that sits too far left, and the closing delimiter whose column
decided how far that was. A `"""` that shares its line with text is one
refusal whether it is the opening one or the closing one. And a
tolerance worth knowing sits at the end of §2.3: a byte order mark at
the very start of a source file is stripped and is never a diagnostic,
so an editor that insists on writing one cannot break your build.
Appendix C gains all five codes, and its count was re-measured rather
than incremented: it claimed 48 while the table held 49, and it says 54
over 54 now.

`samples-os.toml` holds no rows. The file of per-host differences opened
last sprint with six, four of which retired at the previous pin when
Windows grew a task layer. The last two were never about a version: one
compiler spelled the same project's paths two ways, `wolf add` and `wolf
publish` printing the host's separator where every diagnostic in the
same binary prints a slash. That is fixed at this release, and the
Windows lane said so before anything was deleted: it failed both rows,
as stale, and named the issue that had landed. The machinery stays and
both directions stay enforced. An empty file is a measurement: every
declared per-host difference this book has found has been answered by
the toolchain.

Two of this sprint's blocks run on the compiler alone and say so; the
book has had that lane since bs09, for the programs one implementation
runs, and this time they are the new byte-cast transcript and the new
ledger reading. The reference interpreter's release predates the type,
so it answers `65 as byte` with "nothing with this name is in scope",
which was probed at the bump in both directions, recorded in the pin
file, and retires at that project's next release. Neither block is
skipped; both are executed and byte-compared on every lane that has a
compiler.

wolf also learned unix-domain sockets this release, and no page prints
one, which is worth saying: this edition has no network chapter and
makes no socket call anywhere, so there is no list of transports for the
family to join. It was measured on this host at the pin and recorded
where the book keeps toolchain facts it does not teach. What it did
retire is a stale sentence in chapter 11's own ledger, which had been
explaining a design choice with "there is no network surface at this
toolchain" long after there was one.

The clause anchors grow 411 to 417 (four for the new scalar, two for the
socket clause), with none dropped and none retargeted. The diagnostic
catalogue does not move at all: this release re-ruled four codes and
minted none. The grammar appendix regenerates to itself, since `byte` is
a type name and not a keyword. Two version transcripts, chapter 22's
interface stamp and chapter 25's publish record re-record as they do at
every bump, and no printed diagnostic moved.

The print edition sets to 513 pages, three more than the previous one.

## bs26 — 2026-09-02 — the rows retire, and the links come back

The pins move to wolf v0.2.3 and lupin 0.1.23, and the headline is a
table that no longer exists. One release ago Windows compiled and ran
your program for the first time, and refused twenty-one programs of the
compiler's own corpus by name, everything built on the task layer, which
that host had none of. It has one now. `spawn` and scopes, `proc`,
channels and `select`, `sync`/`when`, region transfer, signals and
network deadlines all compile and run there, measured at the same corpus
parity as macOS. Chapter 1 said Windows readers should expect to meet
that limit; it does not say so any more, because they will not.

The book found out the way it was built to. Four programs of chapter 30
were written down last sprint in `samples-os.toml` as refused on
Windows, in the refusal's exact words, with the release that would end
them named in the row. At the new pin the Windows lane ran them, they
passed, and the run went red: four FLIPs, each naming the row to delete
and the release it was dated to. Then the rows came out, in the commit
that moved the pin, which is the rule they were written under. 455
passed and 4 flipped on that lane; 455 + 4 is the 459 the other two
hosts report. A skip would have gone on passing quietly through the
release that made the claim false. Two rows are left, the ones that were
never about a version: `wolf add` and `wolf publish` still print
Windows' own path separator where every diagnostic in the same compiler
prints a slash, and that is still filed.

The Solutions page has its links back. Every one of the 280 collapsed
solutions is headed by the exercise number and the section that set it,
and on the web that section reference had been rendering as its own
markdown punctuation: `[§3.4](../ch03.md#3.4)`, on all 280 of them,
because the line sits inside a raw-HTML block and a markdown parser does
not look inside those. It is a real link now, on the web and in the PDF
both. The print half is why the fix waited: a printed page has no
hyperlinks to give and no `.html` to point at, so the same source line
becomes an internal cross-reference to the section's own label. That
meant the print edition had to start labelling its headings at all, with
the very same rule the web edition has always anchored them by. One
rule, two renders, 280 references that cannot drift from their targets
because a reference to a section that moved fails the build.

Chapter 1's install section was re-measured against the project's own
install page for this release. Four archives at the tag now, one per
tier-1 host (the ARM one came back after a release that built it and
threw it away), so the section says four instead of hedging, and says
what the ARM archive serves, which is less than the other three. Two
limits are left on Windows, and the section quotes both: the optimizing
release tier still refuses that host, and a `reload` or `upgrade` signal
sent from *another process* has nothing on Windows to arrive through.
And a sentence this book got wrong about itself is fixed: chapter 1 and
the Notation page both explained the four quoted Windows transcripts by
saying the book's runner has no Windows lane. It has had one since the
previous edition. The real reason is narrower (the runner replays
programs, and an installation is not one), and that is what both pages
now say.

The rest of the bump was quiet, which is worth reporting. The compiler
changed the width of a parse error's underline this release, and its own
measurement predicted seven of this book's printed diagnostics would
widen. None did: every E0201 in the book points at a single-character
token, where the old shape and the new shape draw the same one caret.
That was probed both ways. The clause anchors hold at 411 and the
diagnostic catalogue at 169, neither moving by one. The grammar appendix
grew twelve productions the specification had been citing without
defining, so three of the six ways to write a string in wolf can now be
derived from the appendix instead of inferred from prose. Two version
transcripts, chapter 22's interface stamp and chapter 25's publish
record re-record as they do at every bump. And chapter 4 §4.3 loses a
caveat: the one place in that chapter where the two implementations
disagreed (whether a trap runs the outermost pending `defer`) is a place
where they agree now, and the section says which one moved.

The print edition sets to 510 pages, the same as the previous one.

## bs25 — 2026-09-02 — the samples lane is real

CI ran the samples on three machines for the first time. It had been
able to for months; the credential that lets it read the pinned compiler
was set today, and the lane that had been reporting a skip went and did
the work. macOS agreed with the machine the book is written on, 459
samples to nothing. The other two hosts had never been asked, and they
had 25 things to say.

Nineteen of them were one sentence. On Linux, `wolf build` looks for the
LLVM linker and says so when it does not find it, and the runner did not
have it, so nineteen transcripts across nine chapters gained a line the
book does not print. The rig was right to fail: the line is real output.
The question was what to do about it, and there were two answers. The
book could teach the replay to drop `note:` lines, or the lane could
have the linker. Dropping them would make CI quieter and leave the
reader's terminal unchanged, and it would hide a line the reader is
going to see. So the lane installs `lld`, and chapter 1 §1.2 now tells
you to install it too, prints the note you get if you do not, and says
the build still succeeds. Every console block in this book is still
compared byte for byte with nothing subtracted.

Six were true statements about a host. Four programs in chapter 30
cannot be built on Windows at this pin: the parallel capstone and three
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
starts working flips, hard, naming the row to delete: the same
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
capstone needs it too, measured on the Windows lane the day the lane
first ran. The sentence now says so, and names the sequential twin that
does run anywhere. The print edition sets to 510 pages, one more than
bs24, all of it §1.2's.

## bs24 — 2026-09-02 — the book sees the comma

The syntax highlighting is re-pinned. Every code block in the book is
painted at build time by a grammar vendored from wolf-lsp, and that
grammar had been sitting at a revision older than the `char` type: a
char literal was body ink, and so was the word `char` itself. Both now
paint: `char` in the type blue, `'a'` in the same green as `"a"`,
because a char literal is quoted text and the palette sorts by kind. The
escape inside a string keeps its own bronze, which is the one ordering
that had to be got right.

Six blocks change colour and no page moves: 991 rendered blocks compared
before and after, six differ, and the print edition sets to 509 pages
either way. The six are the `n as char` cast in chapter 2 and five
solutions in the back matter, where the brace-balancer and the Caesar
shift are made of char literals. The print edition takes the same six
changes from the same grammar and the same palette, which is the
single-source rule working.

Ten blocks were then read by eye against what they mean, and two of them
are painted wrongly by the pinned grammar. In a raw literal the braces
are two more characters (chapter 2 says so in a sentence, and the
sample's own output proves it), but the grammar paints them as an
interpolation, and it does the same to the raw strings the brace
balancer is scanning. And inside an interpolation a char literal goes
unpainted while the `as char` beside it paints. Neither is patched
around in the book: both are filed upstream (wolf-lsp#4, wolf-lsp#5) and
recorded in the pin, where the last rendering gap was recorded and from
where this one was closed.

Nothing else was needed. The grammar's new error node has nothing to
mark (no rendered block in the book carries an invalid escape), and the
region keywords `cap`, `rc` and `pool` are contextual by the
specification, which a grammar made of regular expressions cannot tell
from a name, so `region r(cap: n)` paints the word `region` and stops.
Both were measured.

## bs23 — 2026-09-02 — the book holds a budget

The pins move to wolf v0.2.2 and lupin 0.1.22, the learners' release,
and three things arrive with them. Chapter 8 gains §8.9: a region will
now tell you what it holds, and you can tell it what it may hold.
`region_bytes` and `live_region_bytes` are taught as the four relations
the specification guarantees on every implementation (zero at the open,
charged after a build, unchanged between two adjacent reads, and gone
wholesale at the brace), and not as a byte count, because the unit is
the machine's and the section says so. `region r(cap: n)` puts a ceiling
on the ledger, a charge past it traps at the allocation that asked for
it, and the budget in every sample is *measured* rather than estimated,
which is the section's other lesson. The last part is the one a server
wants: a request that breaches its budget inside a proc dies alone. The
reason reaches the join as a value (`is_fault()`,
`is_alloc_contract()`), the memory is back before the reason is
delivered, and the `defer` below the proc boundary never runs, which the
transcript proves by the line that is missing. Chapter 14's per-proc
accounting aside and §8.1's per-request arena both point at it, and two
exercises land beside it: 8-18 reads the ledger four times, 8-19 is the
cap kata.

Chapter 1 gains an install path. Both projects now publish a per-host
archive at every tag, so §1.2 leads with "unpack it and put it on your
PATH" instead of two cargo builds, and Windows, where the compiler
produces and runs a native `hello.exe` for the first time, is spelled
out as the project's own measured page states it: the Visual Studio
Build Tools requirement, the refusal quoted whole for a machine without
them, `lupin.exe` as one file with no installer, and the two things that
still refuse there. Those four blocks are labelled for what they are
(transcripts measured elsewhere, not replays) because this book's sample
runner has no Windows lane, and the Notation chapter now says how to
spot one.

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
since the 0.1.15 era both tools name the same interpreter release: the
colophon's paragraph explaining why they differed retires with the fact
behind it. §4.3's `defer` teaching is corrected where it was wrong (a
`defer` in a loop body runs at the end of every turn, not when the
function returns), and the section's sample now prints an interleaved
transcript that can tell the two readings apart, which no sample in the
book could before. Patterns arrive at the ladder rather than at a new
section: exercise 3-14 rewrites 3-9's pack drill as one `match` over the
pair `(n % 3, n % 5)` and shows the compiler naming the unreachable arm
that the `if`-chain version could only leave silent; 7-16 respells the
Point/Rect kata so the arms take the value apart by field name; 13-11
scans for a substring through a slice of a *lent* byte view. Appendix A
regenerates on the struct-pattern production and a `\u{…}` escape now
bounded at six hex digits; Appendix C's catalog grows 168 → 169 with
`E0814`. The corpus grows 244 → 248 files, the index recounts
mechanically at 327 exercises (278 printed), and 452 samples pass
against bs21's 448; five transcripts re-recorded at the bump, every one
classified, zero failures and zero flips.

## bs21 — 2026-08-31 — the exercises multiply

The K&R ladder: 45 new program-shaped exercises (43 printed, two held to
the masters as drills), each a self-contained tool the reader leaves
owning: temperature tables and a longest-line finder in chapter 1;
reverse, squeeze, centering, visible escapes and detab/entab in chapter
2; the pack drill, a binary table, one-pass statistics and arithmetic
palindromes in chapter 3; Collatz, a closure factory, `rtrim` and
Zeller's weekday in chapter 4; the run-length pair round-tripped by exit
code, a histogram, `any_index`, a CSV ledger and a line folder in
chapter 5; `itoa`, a hardened decoder row and a date validator in
chapter 6; the Point/Rect kata and a consume-versus-lend rewrite in
chapter 7; a region ring window, a little-endian byte round trip, a
bracket matcher and infix→postfix on the worklist stack, a caesar round
trip, substring counting both ways, a stockroom proc, the Josephus ring
moved whole, and, filling chapter 18's own ledger asks, the E0412
grammar spelunk, the `Buf[…]` identity drill, and roman numerals folded
both ways with a compile-time witness. Chapter 22 gains the multi-file
tier: a two-module word counter, a calculator behind an `ops` seam, the
`//! member: false` scratch pair with its one-marker trap, an in-module
name clash and its fix, an export-hash spelunk, and a what-earns-`pub`
design. The Notation chapter now states the directive-header and
member-marker rules it was long cited for; Appendix C gains `W0313`. The
corpus grows 191 → 244 files, the index recounts mechanically at 324
exercises (275 printed), and every new sample replays green: 448 passed
against bs20's 401, zero failures, zero flips, no new pending rows.

## bs20 — 2026-08-31 — the record on the page

Chapter 25 gains its first printed section. §25.3 teaches `wolf publish`
as measured at the pins: the one-line transparency record and its three
content addresses (tree, manifest, interface), the maintainer's
static-log append with its keyed head, and the refusal that makes a
published version immutable; both transcripts are replayed from a
fixture in CI. The "this edition does not carry this chapter" stub
retires; editions (§25.1) and the stdlib posture (§25.2) stay reserved
with their reasons on record. 401 samples green, 196 of 198 console
blocks replayed (two new).

## bs19 — 2026-08-30 — the pin tells the truth

The book's transcripts now show the released toolchain: wolf v0.2.0 and
lupin 0.1.18, printing the bare D57 version strings (a plain trunk build
fails the book's own checks by design). Every `str + str` claim reads
present-tense: the feature landed, and the prose stopped hedging.
Chapter 25's publish-gate section healed and is flagged for the human's
call; chapter 16's stale differential was re-recorded. All 401 samples
green, CI 7 of 7.

## bs18 — 2026-08-30 — the numbers teach

Two held chapters exist at last: chapter 19 (reading the release tier)
and chapter 20 (performance contracts), written against what the
toolchain does: inert attributes are never taught, and `wolf bench diff`
appears nowhere because it does not exist. Chapter 21's wrong number is
fixed (two million → four thousand), and chapter 5's stale exit-4
refusal fences retired with a re-teach.

## bs17 — 2026-08-29 — the pin catches the site

Pins advance to wolf addcd7f and lupin 0.1.16, and every trap
transcript now names its site the way the tools do: 79 line:col sites
byte-exact, one transcript and eleven claims updated for the compiled
tier's `at file:line:col` second line. Appendix B gains the D60
exit-status table. The char-era fences graduate: 398 samples, 0
failures, with the environmental class empty.

## bs16 — 2026-08-29 — the register rewrite

Four lanes (front + ch01–11, ch12–22, ch23–32 + back matter, and the
solutions mirror) rewrote the whole book to the ratified register:
running em-dash density to zero per thousand words nearly everywhere (a
handful of defended survivors), ~1000 template dashes out of the
exercise and solutions apparatus, worth-markers and reveal molds
retired, UK spellings out. Two false toolchain claims were fixed by
probing (ch27's char literal, ch31's spawn-in-loop), and chapter 2
gained a cast-trap transcript measured on both machines. 159 drifted
solution stems re-adopted their chapters' text.

## bs15 — 2026-08-28 — the exemplar and the register

The fix register itself: eleven ratified rules with three human
amendments, proven on two exemplar chapters before the fan-out: chapter
15 (14.0 → 0.0 dashes per thousand words, all sixteen rows
dispositioned, samples byte-stable) and chapter 5 (nine schedule claims
resolved by checking reality: traits landed reads present tense, absent
surface reads as scope, two unruled gaps to the ledger).

## bs14 — 2026-08-28 — the tells catalog

An audit: three lanes cataloged every AI-prose tell in the book, 538
rows across front matter, 32 chapters, back matter and solutions, zero
banned-vocabulary hits, six systemic patterns named (the
14-per-1000-words running-dash constant among them). Zero prose changed
on the page; the two passes after it executed the catalog.

## bs13 — 2026-08-28 — the write-marked receiver, and char

Pins advance to wolf a900b8c and lupin 0.1.14. `(mut xs).push(…)` is now
mandatory on both machines, so Part 1 teaches it from the start:
chapters 2–7, the appendices and a dozen exercise solutions re-spelled,
with a read-it-as-"this call writes" paragraph in chapter 3. Chapter 2
gains §2.4: char as a Unicode scalar value (D58), with `chars()`, the
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

M2 declared upstream, and the book catches up: chapter 21 opens three of
its five sections (aliasing, arenas, and the bill) with the declared
benchmark number printed as a CI artifact rather than typed into prose.
The C contrast twins (saxpy with and without `restrict`, ten thousand
nodes under malloc) compile and run in CI.

## traits-era — 2026-08-22 — the book learns the trait system

Chapter 5's §5.5 lands: the trait system the chapter promised in
§5.3, taught with executed samples at the traits-era pin, plus
exercises 5-9 and 5-10 with published solutions. The pairing line
moves for the first time since lupin 0.1.8.

## bs10 pass three — 2026-08-20 — the pin crosses the campaigns

The conc and generics campaigns cross with zero snapshot movement, the
first pin bump with that property. Gates re-measured: chapter 29 loses
the monomorphization half of its hold, chapter 13's race row half-closes
and inverts (wolf now catches the capture; wolf-interp#30 filed),
chapter 18's §18.3 claim narrows to const-generics.

## The parts shipping — 2026-08-09 → 08-12 (bs01–bs10, rp-M1, rp01, P5)

The book itself, part by part, every sample executed at pinned
toolchains and every console transcript byte-replayed:

- **bs01–bs02** — Part 1 (ch01–06): notation, values, functions,
  collections, errors-are-values with the receipt capstone.
- **bs03–bs05** — Part 2 (ch07–09): moves and modes, regions with the
  compiled Rust contrasts, the unsafe tier run three ways.
- Part 3 (ch10–17): scopes and the join law, channels, select,
  procs, supervision, let-it-crash, determinism; ch13 held until
  rp01 landed it where its gate opened: the race that does not
  compile, both tools agreeing on code and span.
- **bs08–bs09** — Part 4 opens: comptime (ch18), directory-is-module
  and the interface export-hash (ch22), the covenant chapter (ch24);
  ch19–21, ch23 and ch25 held on surface the toolchain did not yet
  have.
- **bs10 + P5** — Part 5, the guided projects: the C twins compiled
  and asserted, P1–P3, pargrep's determinism argument on the page
  (six seeds, one output hash), the allocator coda; tinyvm held on
  its measured gate.
- **rp-M1** — chapter 1 opens on `wolf build hello.lu && ./hello`;
  the pre-alpha banner and sprint-number teaching die.
- The edit pass: cross-references that resolve, terminology settled
  (task not thread, row not error union, trap not panic), appendices
  generated from the pinned spec, solutions for 214 exercises.
