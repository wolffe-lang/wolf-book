# Colophon

<!-- STUB (bs00): the full colophon lands in bs11. The pins below are
     the live wolf-toolchain.toml values; doc-truth CI compares them. -->

This printing is true for one toolchain, and these lines are it — each
implementation also naming the revision of the other it was tested
against:

```console
$ wolf --version
wolf 0.1.0 (wolfgang)
paired with lupin 0.1.8 (reference interpreter), pin 7886559
$ lupin --version
lupin 0.1.8 (wolf-interp, reference interpreter at pin 26fa98e)
```

The exact revisions — the compiler, the interpreter, and the wolf
grammar vendored from wolf-lsp — are recorded in `wolf-toolchain.toml`
at the repository root. Every code sample, every console run, and every
diagnostic on these pages was executed against them by CI; the samples
runner's report is the guarantee, not this sentence.
