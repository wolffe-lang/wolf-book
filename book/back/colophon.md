# Colophon

This printing is true for one toolchain, and these lines are it, each
implementation also naming the revision of the other it was tested
against:

```console
$ wolf --version
wolf 0.2.1 (wolfgang, pin 75fd2d0)
paired with lupin 0.1.20 (reference interpreter), pin b80d239
$ lupin --version
lupin 0.1.20 (wolf-interp, reference interpreter at pin b80d239)
```

The two lines name the same interpreter release here, and that is worth
a sentence because it is not guaranteed. Each tool reports the revision
of the other it was differentially tested against, the two releases are
cut on their own schedules, and printings where those differ by a
release are ordinary. §1.2 teaches the reader to read the pair either
way. This page prints them as they are. The compiler's first line is
also this page's proof: a build made exactly at its release tag prints
the bare version and the commit it was built from, and any other build
names itself `+dev.<commit>` instead, so the line above is a claim only
the release binary can make.

The exact revisions (the compiler, the interpreter, and the wolf
grammar vendored from wolf-lsp) are recorded in `wolf-toolchain.toml`
at the repository root. Every code sample, every console run, and every
diagnostic on these pages was executed against them by CI; the samples
runner's report is the guarantee, not this sentence. The surface grammar
in Appendix A is copied from the specification at the same revision, and
the diagnostic codes in Appendix C are checked against the compiler's own
catalog in the same run.

Two chapters and five sections in this edition are reserved rather than
written, each one saying on its own page what it covers and why it is not
here. Their section numbers are anchors and will not move when the pages
arrive.

## Setting

The text is Charter; code is Source Code Pro, self-hosted so the web
edition and the PDF set code identically. The web edition is canonical
and is built with mdBook, with syntax highlighting done at build time
from the same grammar the editor tooling uses; no highlighter runs in
the reader's browser. The PDF is set with typst from the same markdown. All
three artifacts build on every commit, and the PDF's absence fails the
build.

## License

Code samples are GPL-3.0-or-later with the wolf Runtime Library
Exception, matching the runtime, so code you take from this book into
your own programs is yours. The license for the prose is recorded in the
repository, and the repository is the authority for both.

## Errata

Corrections and reports: <https://github.com/wolffe-lang/wolf-book/issues>.
The [errata page](errata.md) states which kinds of report the book acts
on and what a printed copy promises between reprints.
