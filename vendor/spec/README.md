# vendor/spec — the pinned specification artifacts

Three files, copied from wolf-lang at the revision in
`wolf-toolchain.toml`. Nothing here is edited in this repository.

- `grammar.ebnf` — the surface grammar, extracted from the spec's
  `01-grammar.md` by wolf-lang's `spec-extract`. Appendix A is generated
  from this file by `cargo xtask backmatter`.
- `anchors.json` — every registered clause anchor, tag to spec document.
  `cargo xtask verify-docs` checks that each `[clause.tag]` the book
  prints is in this set. Six tags are not, and Appendix D lists them.
- `diagnostic-codes.txt` — every code in the compiler's catalog, one per
  line. `verify-docs` checks Appendix C against it in both directions.

`cargo xtask backmatter --check` compares these copies against a sibling
wolf-lang checkout when one is present and says so in the log when it is
not. The samples lane has the sibling; the doc-truth lane runs without
it.
