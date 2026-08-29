# Appendix D — Spec cross-reference

The book teaches; the specification rules. Where a page cites a clause in
square brackets — `[mem.ub.defined]`, `[conc.task.spawn]` — this table is
how you find the normative text behind it. Clause anchors are stable, so
a tag printed by a tool in your terminal is findable in the spec even when
the tool's wording changes.

The specification is seven documents:

| Document | Anchors |
|----------|---------|
| 01 — Surface Grammar | `gram.*` |
| 02 — Memory Model | `mem.*` |
| 03 — Concurrency | `conc.*` |
| 04 — ABI | `abi.*` |
| 05 — Conformance | `conf.*` |
| 06 — Differential Protocol | `proto.*` |
| 07 — Schedule Points | `sched.*` |

## Book section to clause

| Section | Clause | Document |
|---------|--------|----------|
| 1.4 The REPL | `[mem.ub.defined]` | 02 |
| 1.5 What `run` was doing for you | `[gram.expr.block]`, `[mem.ub.defined]` | 01, 02 |
| 2.3 Bytes, honestly | `[mem.ub.defined]` | 02 |
| 3.1 `let`, `var`, and handing values over | `[mem.tier0.move.2]` | 02 |
| 3.3 Arithmetic that traps | `[mem.ub.defined]` | 02 |
| 5.4 Indexing that traps | `[mem.ub.defined]` | 02 |
| 6.4 Hardening by refactor | `[mem.ub.defined]` | 02 |
| 7.1 The error we owed you | `[mem.tier0.move.2]` | 02 |
| 7.2 Values are trees | `[mem.tier0.move.2]` | 02 |
| 7.5 Field-granular exclusivity | `[mem.model.path.disjoint]` | 02 |
| 7.7 What the machine does | `[mem.tier0.excl.1]` | 02 |
| 8.3 Regions are values | `[mem.region.freeze.3]` | 02 |
| 8.5 Freeze | `[mem.region.freeze.1]` | 02 |
| 8.6 Open, and open again | `[mem.region.multiopen]` | 02 |
| 8.7 `shared` and `handle` | `[mem.shared.handle.2]`, `[mem.shared.rc.2]`, `[mem.region.intra.1]` | 02 |
| 9.3 The oracle you actually run | `[mem.ub]`, `[mem.prov.state]` | 02 |
| 9.4 The one door back | `[mem.unsafe.door]` | 02 |
| 9.6 FFI and regions | `[mem.prov.region]` | 02 |
| 10.2 The leaked goroutine, retired | `[conc.deadlock.trap]` | 03 |
| 12.1 Typed channels | `[mem.tier0.move.2]` | 02 |
| 12.4 `when (a, b)` | `[gram.expr.conc]` | 01 |
| 13.2 The race that does not compile | `[conc.task.spawn]`, `[conc.chan.type]` | 03 |
| 14.1 Armstrong's argument, one page | `[gram.expr.conc]` | 01 |
| 16.1 `ch.send(move r)` | `[mem.tier0.move.2]` | 02 |
| 22.1 Directory = module | `[mod.dup]`, `[mod.vis.private]`, `[mod.use.unused]` | see below |
| 22.2 No cycles | `[mod.cycle]` | see below |
| Appendix B | `[conf.trap.set]`, `[conf.trap.map]`, `[conf.trap.assert]`, `[conf.trap.exit]`, `[conf.trap.report]`, `[conc.mm.race.3]` | 05, 03 |

## Six tags with no clause

Six of the tags a reader meets in this book are printed by a tool and
carry no anchor in the specification. They are listed here rather than
quietly dropped, because a reader who searches the spec for one of them
should find out from the book that the search will fail:

| Tag | Where it reaches the reader |
|-----|-----------------------------|
| `arith.checked` | the `trap(overflow)` line, §3.3 |
| `mod.dup` | `E0302`, §22.1 |
| `mod.vis.private` | `E0304`, §22.1 |
| `mod.use.unused` | `E0305`, §22.1 |
| `mod.cycle` | `E0303`, §22.2 |
| `repl.trap.alive` | the REPL's trap line, §1.4 |

The module rules those four `mod.*` tags name are real and are enforced
by both implementations — chapter 22 runs every one of them. What is
missing is the anchor, and the fix belongs in the specification rather
than on this page. The book's own CI holds this list to its length: a new
unanchored tag in the prose fails the doc-truth job, and an anchor that
arrives in the spec removes its row from this table.

## Reading the spec against the book

The two documents answer different questions. A section here shows a
program and argues about it; a clause there states a rule and closes its
set. Where they disagree, the specification wins and the book has a bug.
The book's `[conf.*]` citations are the exception worth knowing about:
those clauses are what make the samples in this edition checkable, so
that appendix and this one describe the same machinery from two sides.
