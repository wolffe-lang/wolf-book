# Colophon

<!-- STUB (bs00): the full colophon lands in bs11. The pins below are
     the live wolf-toolchain.toml values; doc-truth CI compares them. -->

This printing is true for one toolchain, and these two lines are it:

```console
$ wolf --version
wolf 0.0.1 (wolfgang)
$ lupin --version
lupin 0.1.5 (wolf-interp, pin f0da6e6)
```

The exact revisions — the compiler, the interpreter, and the wolf
grammar vendored from wolf-lsp — are recorded in `wolf-toolchain.toml`
at the repository root. Every code sample, every console run, and every
diagnostic on these pages was executed against them by CI; the samples
runner's report is the guarantee, not this sentence.
