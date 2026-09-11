# Colophon

This printing is true for one toolchain, and these lines are it, each
implementation also naming the revision of the other it was tested
against:

```console
$ wolf --version
wolf 0.2.10+dev.5289501 (wolfgang, pin 5289501)
paired with lupin 0.1.31 (reference interpreter), pin 4c60946
$ lupin --version
lupin 0.1.32 (wolf-interp, reference interpreter at pin e0ce018)
```

The two lines name each other, and this printing neither name is
exact. Each tool reports the revision of the other it was
differentially tested against, and the two projects are cut on their
own schedules. The compiler names `lupin 0.1.31`, one release before
the interpreter under it. The interpreter names `e0ce018`, one hundred and ten
commits before the revision the compiler above it was built at. §1.2
teaches the reader to read the pair either way. This page prints them
as they are. The compiler's first line is also this page's proof, read
the other way round: a build made exactly at a release tag prints the
bare version, and every other build names itself `+dev.<commit>` and
claims nothing. This printing's compiler is the second kind, built from
trunk at the revision it prints, and that revision is abbreviated to a
width the toolchain fixes rather than inherits from whoever cloned it.

The exact revisions (the compiler, the interpreter, and the wolf
grammar vendored from wolf-lsp) are recorded in `wolf-toolchain.toml`
at the repository root. Every code sample, every console run, and every
diagnostic on these pages was executed against them by CI; the samples
runner's report is the guarantee, not this sentence. The surface grammar
in Appendix A is copied from the specification at the same revision, and
the diagnostic codes in Appendix C are checked against the compiler's own
catalog in the same run, along with the count that opens that appendix
and the copy of the catalog it is counted from.

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
