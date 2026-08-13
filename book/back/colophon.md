# Colophon

This printing is true for one toolchain, and these lines are it — each
implementation also naming the revision of the other it was tested
against:

```console
$ wolf --version
wolf 0.1.0 (wolfgang)
paired with lupin 0.1.8 (reference interpreter), pin 7886559
$ lupin --version
lupin 0.1.9 (wolf-interp, reference interpreter at pin 0b4e79c)
```

The two lines name different interpreter releases on purpose. Each tool
reports the revision of the other that it was differentially tested
against, and those tests happen one release apart: the interpreter here is
0.1.9, and the compiler's last differential run was against 0.1.8.
§1.2 teaches the reader to read the pair that way. This page prints them
as they are.

The exact revisions — the compiler, the interpreter, and the wolf
grammar vendored from wolf-lsp — are recorded in `wolf-toolchain.toml`
at the repository root. Every code sample, every console run, and every
diagnostic on these pages was executed against them by CI; the samples
runner's report is the guarantee, not this sentence. The surface grammar
in Appendix A is copied from the specification at the same revision, and
the diagnostic codes in Appendix C are checked against the compiler's own
catalog in the same run.

Five chapters and three sections in this edition are reserved rather than
written, each one saying on its own page what it covers and why it is not
here. Their section numbers are anchors and will not move when the pages
arrive.

## Setting

The text is Charter; code is Source Code Pro, self-hosted so the web
edition and the PDF set code identically. The web edition is canonical
and is built with mdBook, with syntax highlighting done at build time
from the same grammar the editor tooling uses — no highlighter runs in the
reader's browser. The PDF is set with typst from the same markdown. All
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
