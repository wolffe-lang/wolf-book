# Appendix B — Traps

The twelve trap kinds are closed by `[conf.trap.set]`; adding one
requires revising the spec. Each entry names the kind, the fault, the
clause it enforces, and the sections where the book shows it.

A trap line has one shape, and the interpreter is the implementation that
prints it:

```text
<file>.lu: trap(<kind>): <fault> [<clause>] at <line>:<col>
```

The location is the operation that faulted (a 1-based line and a 1-based
column counted in characters), and a trap ends the process with exit 3.
The status is per-machine and documented, not part of the contract: 3 is
the interpreter's, a compiled binary ends with its own documented 134, and
conforming tools compare the *kind*, never the number
(`[conf.trap.exit]`). The same clause rules what a trap does on its way
out: it runs no `defer` and no `errdefer`, anywhere — in the root
domain death is immediate, and every scope-exit effect still pending is
abandoned (§4.3 shows it; a trap contained at a proc boundary abandons
the ones below the boundary the same way, §8.9). A compiled binary's report is two lines, the kind
and the site: `wolf-trap: <kind>`, then ` at <file>:<line>:<col>`
(`[conf.trap.report]`). A REPL session's trap line keeps entry-relative
byte offsets instead: a session has no stable line numbering, and the
prompt owns its own coordinate. A trap is not undefined behavior and not a
crash: the fault is named, the rule is cited, and the same program faults
the same way in every build profile.

| Kind | Fault | Clause | Sections |
|------|-------|--------|----------|
| `overflow` | an arithmetic operation left the range of its type | `[arith.checked]` | 3.3 |
| `div-zero` | division or remainder by zero | `[mem.ub.defined]` | 3.3 |
| `bounds` | an index, a byte range, or a `pop` outside a collection | `[mem.ub.defined]` | 1.4, 1.5, 2.3, 5.4, 6.4 |
| `use-after-move` | a place is read after its value moved away | `[mem.tier0.move.2]` | 3.1, 7.1, 7.2, 12.1 |
| `exclusivity` | two overlapping paths held `mut` at once, or a write through a read-mode binding, or mutation during iteration | `[mem.model.path.disjoint]` | 7.5 |
| `region-fault` | a region rule broken at run time: a write to frozen data, a transfer of an open region, a non-disjoint open | `[mem.region.freeze.1]`, `[mem.region.freeze.3]`, `[mem.region.multiopen]` | 8.3, 8.5, 8.6 |
| `stale-handle` | a handle's generation does not match its pool slot | `[mem.shared.handle.2]` | 8.7 |
| `alloc-contract` | an allocation contract was broken: a charge past a region's byte budget, or a negative budget at the region's creation | `[mem.region.cap.1]`, `[mem.region.cap.2]`, `[conf.trap.map]` | 8.9 |
| `assert` | a user assertion failed, or a builtin's caller contract was broken | `[conf.trap.assert]` | 4.3 |
| `race` | a data race the runtime detected | `[conc.mm.race.3]` | none |
| `ub` | the oracle caught undefined behavior in the unsafe tier | `[mem.ub]` | 9.4 |
| `deadlock` | every live task is blocked with no timer and no I/O pending | `[conc.deadlock.trap]` | 10.2 |

One kind has no page in this edition. The book argues at length in
chapters 13 and 17 that the shapes a data race is made of do not
compile, so `race` never fires in a program these pages print.

Two kinds reach the reader in a narrower form than their row suggests.
`assert` appears at run time only in §4.3, where what it demonstrates
is what a trap does to a pending `defer`; its compile-time form, where
a failed assertion is a diagnostic rather than a trap, is `E0710` in
§18.1. And `alloc-contract` is raised in this edition by the region cap
of §8.9 and by nothing else — the function-level allocation contracts
that share the kind are a surface chapter 20 scopes out of v1
(§20.1).

Detection is required for the first nine kinds in every profile. `race`
detection is permitted rather than required, and `deadlock` detection is
required in the deterministic test modes chapter 17 uses and permitted
elsewhere.
