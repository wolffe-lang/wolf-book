# Errata

This book claims that its samples cannot rot: every program, transcript,
and diagnostic on these pages was executed by CI against the toolchain in
the [colophon](colophon.md), and a sample that stops behaving fails the
build. That covers the code. It does not cover the prose, the arithmetic
in a paragraph, or a claim that was true when it was written and is false
now, so this page exists and so does the process behind it.

Report anything wrong at
<https://github.com/wolffe-lang/wolf-book/issues>. Three kinds of report
are useful, and the issue template asks which one you have:

**Technical.** A sample that is wrong, a claim that is false, a
diagnostic that no longer says what the page says it says, a comparison
to another language that its own users would object to. These go through
the reviewer checklist before the fix lands: the claim gets traced to a
spec clause or a decision, and the sample gets executed. Include the
toolchain version from `wolf --version` and `lupin --version`, because
the first question is always whether your pin is the colophon's.

**Editorial.** Typos, broken sentences, a term used two ways, a
cross-reference pointing at the wrong section, a heading that lies about
its section. These take the fast path and land as soon as they are read.

**Drift.** A sample that no longer runs on a newer toolchain than the
colophon's. CI catches most of this before a reader does (the book builds
against the compiler's own main branch nightly, and a break is visible
there first), but the nightly is a report and not a gate, so a reader on a
newer toolchain may arrive first. Drift is fixed at the next pin bump.

## What this edition promises

The web edition tracks one toolchain at a time and says which one in the
colophon. When the pin moves, every sample is re-executed, every
diagnostic is re-captured, and the changes are listed in the repository's
history with the bump. A printed copy is a photograph of one such moment:
corrections to print are made at reprint, and until then they are listed
here, so a reader holding paper and a reader holding a browser are
reading the same book with one of them carrying a footnote.

A technical correction changes the page and says so in the commit that
carries it; an editorial one changes the page. Every correction stays
visible in the history.

## Known corrections

None yet in this edition.
