# TONE.md — the voice bible

This document defines the register of *The Wolf Book* by rule and by
demonstration. Every chapter author — human or agent — writes to it; every
reviewer quotes it by line when objecting. The calibration passages in §3
are written about real wolf material and may be lifted as seeds by the
bs01–bs04 authors.

The register in one sentence: K&R's economy and precision, walking beside
the reader rather than lecturing at them, with a dry aside where the
material has earned one.

K&R settled the temperature fifty years ago: "C is not a big language, and
it is not well served by a big book." That sentence is confident, plain,
faintly amused, and it wastes nothing. It is the ceiling for wit in this
book. Hitchhiker's Guide is not.

---

## 1. Hard rules

The is10 register rules (wolf-interp's manual holds to them; so do we):

- **No exclamation marks.** Anywhere. If a sentence needs one, the
  sentence is wrong.
- **No "simply", "just", "easy", "of course".** If it were simple the
  sentence would be unnecessary; if it is not, the word is a lie.
- **No marketing adjectives.** "Powerful", "elegant", "blazing",
  "seamless" — deleted on sight. A claim of merit is made by a program
  and a measurement or it is not made.
- **Honest failure output is part of the product.** Errors and traps are
  shown in full, never elided with `...`, never retyped by hand.

The book-specific rules:

- **"We" walks, "you" acts.** "We" is author and reader moving through
  the material together ("we now have a program that leaks"). "You" is
  for the reader's own hands ("type this and run it"). The author alone
  is "I" only in the preface.
- **Present tense.** The program prints; the compiler rejects; the region
  dies. Past tense is for history (real history, with dates).
- **Code speaks first.** A concept is introduced by a program that needs
  it, then explained. If prose precedes code for more than half a page,
  restructure.
- **Every term is italicized once, at its definition, and never again.**
  One term per concept, book-wide: *task*, *proc*, *region*, *scope*,
  *trap*, *row*. Synonym drift is a copyedit defect.
- **Comparisons to other languages are ones we would defend to that
  language's designers.** Honest contrast, credited lineage, no strawmen.
- **No feature is promised before its campaign shipped.** Deferrals are
  stated plainly ("v1 does not do this").
- **Second person, American spelling, no rhetorical questions as section
  glue** (one per chapter, if the material genuinely poses it).

### The two voices

The book quotes tools. The tools keep their own voice — the compiler's
Elm-shaped diagnostics and lupin's trap lines are shown byte-for-byte from
real runs, and the prose never paraphrases them into chattiness. The
book's voice and the tool's voice are distinct and both stable. When the
compiler says:

```text
error[E1001]: `p.lead` is used here after its value moved away
```

the prose does not repeat "the value moved away" in scare quotes; it
builds on it. The diagnostic is a primary source, treated with the same
respect as a spec clause.

---

## 2. The sass budget

Sass is a seasoning, not a sauce.

- **At most one wry sentence per page.** Most pages have none.
- **Never in reference tables, appendices, or diagnostics.** Reference
  material is deadpan.
- **Never at the reader's expense.** The joke lands on the machine, the
  history, or the authors — not on the person who did not know.
- **Dry, not cute.** The test: read it aloud in the voice of someone who
  has been debugging since 1978. If it needs a wink, cut it.
- A wry sentence must also be a true sentence. Humor is not license.

---

## 3. Calibration passages

Each passage is written twice: flat (competent, correct, dead) and in the
book's voice. Margin notes name what changed. The subjects are real wolf
material; bs01–bs04 authors may use them as seeds.

### 3.1 Concept introduction — regions (chapter 8 seed)

**Flat:**

> A region is a memory management construct that groups allocations
> together so they can be deallocated at the same time. Regions are
> useful for request handling, temporary computations, and other
> scenarios where objects share a lifetime. Wolf checks region usage at
> compile time to prevent dangling pointers.

**Book voice:**

> You have written this program before. A server that builds up parse
> trees, buffers, and half-rendered responses for one request, then
> throws the whole lot away when the response goes out. In C you built
> an arena for it. In Rust you fought the borrow checker to encode it.
> Wolf spells it in one word:
>
> ```wolf
> region request {
>     // allocate freely; walk away; gone
> }
> ```
>
> A *region* is that arena, checked: the compiler proves nothing escapes
> it alive, and the whole thing dies in one motion. Wolf did not invent
> the granule. It checks the one you already believed in.

*Margin notes:* the flat version defines, the book version recognizes —
it starts from a program the reader has already written, which is the
strongest opening move this book has. Code appears before the term; the
term is italicized at definition; the last two sentences carry the
chapter's thesis in eleven words each. No adjective praises the feature;
the claim ("proves nothing escapes it alive") is checkable.

### 3.2 Code walkthrough — the `?` operator (chapter 6 seed)

**Flat:**

> The `?` operator propagates errors. When applied to an expression of
> type `int ! {Empty}`, it unwraps the success value or returns the
> error to the caller. This reduces boilerplate compared to matching on
> every result.

**Book voice:**

> ```wolf
> fn chain(s: str) -> int ! {Empty, NotDigit(Bad)} {
>     let v = parse(s)?
>     v + 1
> }
> ```
>
> Read `?` as "or hand it up." If `parse` succeeds, `v` is the value and
> the next line runs. If it fails, `chain` stops there and returns the
> error itself — whose row, `{Empty, NotDigit(Bad)}`, flows into ours by
> union. No wrapping, no `From` ceremony: the signature already tells
> the caller everything that can go wrong, because the compiler composed
> the list. Go asks you to write `if err != nil` at every call; wolf
> asks you to write one character, and the signature stays honest.

*Margin notes:* code first, then a pronunciation ("or hand it up") — give
operators a reading and the reader can subvocalize the program. The Go
contrast is one sentence, factual, defensible. "The signature stays
honest" does the pitch without an adjective.

### 3.3 Caveat paragraph — byte-honest strings (chapter 2 seed)

**Flat:**

> Note that string length is measured in bytes, not characters. This may
> be surprising for users coming from Python. For example, `"é".len` is
> 2 because the character é occupies two bytes in UTF-8.

**Book voice:**

> One honesty up front, because it will bite the Python refugee within
> the hour:
>
> ```console
> wolf> "é".len
> 2 : i64
> ```
>
> `len` counts bytes. There is no `s[i]` that hands you "the ith
> character", because after thirty years of Unicode there is no cheap
> answer to what a character is — only answers that lie at different
> speeds. Wolf gives you byte offsets, checked slices, and iterators
> that spell their unit (`words()`, `lines()`). When a count of *bytes*
> is the wrong tool, you will know, and you will have said so in the
> code.

*Margin notes:* the caveat leads with the reader's pain ("will bite ...
within the hour"), shows the surprising fact as a real transcript rather
than describing it, and defends the design instead of apologizing for it.
"Lie at different speeds" is the page's one wry clause, and it is also a
true claim about grapheme-vs-codepoint-vs-byte trade-offs.

### 3.4 Trap walkthrough — overflow (chapter 3 seed)

**Flat:**

> If an arithmetic operation overflows, the program will terminate with
> an error message. This happens in all build profiles. The error
> message indicates the operation and the type involved.

**Book voice:**

> Push an `i32` past its ceiling and the program does not wrap, does not
> continue, does not negotiate:
>
> ```console
> $ lupin overflow.lu
> overflow.lu: trap(overflow): `+` produced 2147483648, outside `i32` —
> checked arithmetic traps in every profile (X3); spell intended
> overflow `wrapping[i32]` [arith.checked] at 178..185
> ```
>
> That is a *trap*: the fault of a defined execution. The program was
> legal, it ran, and it hit a rule wolf enforces at runtime — in release
> builds too, which is the part that surprises C programmers. The trap
> names the operation, the value, the rule, and the fix for the rare
> case where wrapping was the plan. Nothing here is undefined; the
> program's last act is to tell you exactly what happened. Most bugs
> should be so polite.

*Margin notes:* the tool's full output is the centerpiece, unedited. The
term *trap* is defined against the transcript, not in the abstract. "Does
not negotiate" and "most bugs should be so polite" are the page's sass
allocation — both attach to the machine's behavior, not the reader's.

### 3.5 Exercise stem — use-after-move (chapter 3 seed)

**Flat:**

> Exercise: The following program contains a use-after-move error.
> Determine what happens when you compile it and when you run it under
> the interpreter.

**Book voice:**

> **Exercise 3-2.** The pack loses its lead:
>
> ```wolf
> var p = Pack { lead: "ada", tail: "grace" }
> let a = adopt(take p.lead)
> let b = p.tail
> let c = p.lead
> ```
>
> Before running anything, write down two predictions: what `wolf` says
> about this program, and what `lupin` does with it. They enforce the
> same rule at different moments — one of them never lets the program
> start, the other lets it die honestly. Check both. Which line does
> each tool blame, and why is `let b` not the one?

*Margin notes:* the stem asks for prediction before execution — that is
the comprehension taxonomy working. It also plants the differential story
(compiler vs interpreter) without a lecture. The question at the end
("why is `let b` not the one?") aims at the actual rule — field-granular
paths — rather than trivia.

### 3.6 Chapter opening — "Who owns this?" (chapter 7 seed)

**Flat:**

> In this chapter we will cover wolf's ownership model. We will learn
> about moves, borrowing, and mutation. By the end of the chapter you
> will understand how wolf manages memory without a garbage collector.

**Book voice:**

> In chapter 3 a value moved and the program died, and we told you to
> wait. This is the chapter where wolf stops apologizing.
>
> One question runs through everything wolf does with memory: *who owns
> this, and how big is the granule?* Every rule in this part — moves,
> `mut`, regions, the escape hatches — is that question asked at a
> different size. Hold onto it and the rest of this part is
> consequences.

*Margin notes:* no "in this chapter we will" inventory — the opening
cashes a promise made four chapters earlier, which rewards the reader for
having been paying attention. The organizing question is italicized once
and becomes the part's refrain. Two paragraphs, fifty-eight words fewer
than the flat version's tour bus.

### 3.7 Transition — out of values, toward regions (chapter 7 → 8 seam)

**Flat:**

> Now that we have covered ownership and mutation, we can move on to
> regions, which are wolf's mechanism for managing groups of
> allocations with a shared lifetime.

**Book voice:**

> The value rules are complete, and they have a ceiling. Our document
> store works — and clones a document every time two indexes want the
> same one, because a tree of single owners has no way to say "these
> five thousand objects live and die together." You can feel the shape
> of the missing feature. It is not a smarter borrow. It is a bigger
> granule.

*Margin notes:* a transition earns its page by naming the *limit* of what
was learned — the itch — and pointing the reader's own intuition at the
next chapter's answer. The last two sentences do the work of a section:
short, declarative, no forward-reference apparatus.

### 3.8 Reference-table introduction (appendix seed — deadpan control)

**Flat and book voice, identical by design:**

> The twelve trap kinds are closed by `[conf.trap.set]`; adding one
> requires revising the spec. Each entry names the kind, the fault, and
> the clause it enforces.

*Margin notes:* reference material gets no sass and no "we". The two
versions converging is the point: at the reference register, flat *is*
the voice. If a table introduction reads distinctively, revise it down.

---

## 4. References & epigraphs

Two reference registers season the book beyond the sass budget. Both are
garnish: a reader who recognizes neither loses nothing technical.

### 4.1 The dark-Romantic register

In-prose similes and chapter epigraphs drawn from the brooding end of the
Western Romantic repertoire — Mahler above all, with Bruckner, late
Brahms, the Pathétique's moods, Rachmaninoff, Tristan-era Wagner. Less
Schumann, more Mahler. These are public domain and safe to allude to
freely. They may carry light structural analogy — a development that
refuses to resolve, the hammer blow that was always coming, an adagio
that knows how it ends — but never technical load: delete the allusion
and the paragraph must still teach.

**Frequency:** at most one epigraph per chapter (most chapters have
none), at most one in-prose simile per chapter, never both within a page
of each other.

**Worked placement 1 — chapter epigraph, chapter 7 (moves):**

> *The finale of Mahler's Sixth promises its hammer for eighty minutes,
> and keeps the promise.*
>
> In chapter 3 a value moved and the program died, and we told you to
> wait...

The epigraph works because the chapter is structured the same way: the
blow (use-after-move) was shown early and lands here with its full
explanation. Set in italics, epigraph position, no attribution apparatus
beyond the composer's name — it is an allusion, not a citation.

**Worked placement 2 — in-prose simile, the freeze section (chapter 8):**

> `freeze r` is not a lock, to be taken and released. It is a cadence:
> after it, the region is *imm* — immutable, shareable, permanent — and
> there is no unfreeze. Like the Tristan chord, the suspension does not
> resolve; unlike the Tristan chord, this is a feature.

The simile carries one structural idea (irreversibility) and the second
clause defuses it before it can carry more. This is the ceiling for how
much analogy a simile may bear.

**Worked placement 3 — part opening, Part 3 (concurrency):**

> *A Bruckner pause: every voice stops at once, and the silence is part
> of the score.*
>
> A scope exit joins all its children. Nothing outlives the bar line.

Two sentences of prose beneath the epigraph tie the image to the actual
semantics (scoped join), then the part proceeds without it.

### 4.2 The Cage the Elephant register

Occasional quotations placed deliberately to produce a moment of reader
confusion — the double-take is the point — before the section resolves
into ordinary precision. Rules, which are strict because this register is
the easiest to ruin:

- **Always typographically distinct from the programming meat:**
  epigraph position or a set-off attributed line. Never inline in an
  explanation, never load-bearing.
- **Always attributed** (band and song title).
- **Rarer than the sass budget:** at most one per part. Five or six in
  the whole book.
- **Permissions caveat (recorded here per the sprint contract):** song
  lyrics are copyrighted and lyric quotation is litigious even at short
  length. Prefer title fragments and near-title allusion, which sit
  safer than verse lines. Direct quotes stay brief, attributed, and are
  flagged on `PERMISSIONS.md` (a ledger this file mandates; bs00 creates
  it) for review before any print edition. The web edition carries the
  same discipline; "it's only online" is not a legal theory we test.

**Worked placement 1 — epigraph, the scheduler chapter (Part 3):**

> *"Ain't no rest for the wicked."*
> — Cage the Elephant
>
> The scheduler agrees. A runnable task is runnable until it blocks, and
> the deterministic scheduler in `--schedules` mode exists precisely to
> deny your program a quiet moment you did not order.

Title-only quotation (safe), attributed, and the following prose turns
the confusion into the section's actual claim within two sentences.

**Worked placement 2 — set-off line, the diagnostics appendix intro:**

> — *"Trouble"* is a Cage the Elephant song and also the only thing a
> diagnostic is about. The catalog below is the compiler's complete
> vocabulary for it.

Near-title allusion, set off, attributed in-line; the appendix itself
then proceeds at the deadpan reference register (§3.8) with no further
seasoning.

**Worked placement 3 — epigraph, the C-membrane chapter (Part 2):**

> *"Come a little closer."*
> — Cage the Elephant
>
> The C library is twenty years old and it works. This chapter is about
> exactly how close we let it come.

The confusion (why is an invitation opening the FFI chapter?) resolves in
the first sentence of prose, which is the pattern every placement in this
register must follow: bewilder for one beat, then pay it off.

---

## 5. The stranger test

A new author with one chapter assignment should be able to answer, from
this document alone: what a page sounds like (§1, §3), how often it is
allowed to smile (§2, §4), and who is speaking at any moment (the book,
the compiler, or lupin — §1). If a draft reads as marketing, quote §1; if
it reads as a stand-up set, quote §2; if a Mahler reference is doing
load-bearing work, quote §4.1 and cut it.
