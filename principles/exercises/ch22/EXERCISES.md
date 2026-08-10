# Chapter 22 — Modules: the shape of a wolf project: exercises

This chapter's exercises are multi-file by nature: each runnable one is
a small directory under `ch22/`, with the entry file named in the
solution. Commands run from this directory; outputs are pasted from
real runs. The directive-header rule from the Notation chapter is load-
bearing here: sibling `.lu` files are one module, and `member: true`
marks the files that only exist through their directory.

## §22.1 — Directory = module

**Exercise 22-1** *(fingers · lupin)* — Build the two-module project:
an entry file and a `stats/` directory exporting `mean`, with a
private `total` helper the entry never sees. Run it. Then move
`total` into a second file inside `stats/` and state what changes for
the entry file.

Solution — `ch22/metrics/`:

```wolf
// metrics/main.lu
use stats

fn main() -> !int {
    var xs = List[int]()
    xs.push(4)
    xs.push(6)
    xs.push(8)
    print("{stats.mean(xs)}")
    0
}
```

```console
$ lupin metrics/main.lu
6
```

Moving `total` to another file inside `stats/` changes nothing
anywhere: files are invisible to importers — the module is the
directory, `use stats` names it whole, and the split is a private
reorganization. That non-event is the design.

**Exercise 22-2** *(comprehension · lupin)* — `vault/keys.lu` defines
`pub fn count()`, private `fn secrets()`, and private `fn total()`.
The entry calls `vault.total()`. Predict the diagnostic — including
whether it says the name does not *exist* — and the exit code.

Solution: E0304, exit 2, and the diagnostic is precise about
existence: the name is there and visibility is the objection. A
resolver that pretended otherwise would send you hunting a typo that
is not one:

```console
$ lupin leak/main.lu
leak/main.lu: E0304: `total` exists in `vault`, but it is private; only `pub`/`pub(pkg)` items are visible across modules (D32) [mod.vis.private] at 148..153
$ echo $?
2
```

**Exercise 22-3** *(comprehension · lupin)* — `twice/main.lu` and its
sibling `twice/extra.lu` each define `fn describe()`. Neither file
imports the other. Predict the verdict, and say why "neither imports
the other" is a trap in the question.

Solution: E0302 — "file boundaries create no scopes." The trap is
thinking imports are involved at all: sibling files are not two scopes
that could shadow, they are one module with one namespace, and the
second definition is a duplicate wherever it sits:

```console
$ lupin twice/main.lu
twice/main.lu: E0302: the name `describe` is defined twice in this module (defined again in `twice/main.lu`); file boundaries create no scopes (D32) [mod.dup] at 103..111
```

**Exercise 22-4** *(comprehension · lupin)* — The entry imports
`tools` and never mentions it again. Predict: warning or error, and
what the diagnostic offers about the fix.

Solution: a hard error, E0305, and the diagnostic notes the fix is
machine-applicable — deleting the line. Wolf takes the Go position
with Go's justification: an unused import is a dependency edge that
slows every build and means nothing, and a warning would be a request:

```console
$ lupin unused/main.lu
unused/main.lu: E0305: the import `tools` is never used in `unused/main.lu`; an unused import is a hard error (D32), and deleting the line is machine-applicable [mod.use.unused] at 113..118
```

## §22.2 — No cycles

**Exercise 22-5** *(comprehension + extension · lupin)* — In
`ch22/tangle/`, `store` imports `index` to log entries and `index`
imports `store` to validate them — each import has a reason, which is
how real cycles are born. Predict the diagnostic. Then perform the
interface-extraction refactor in a copy: move the shared vocabulary
into a third module neither imports from, and run the result.

Solution — before, the cycle drawn whole:

```console
$ lupin tangle/main.lu
tangle/main.lu: E0303: this import completes a cycle: `store` → `index` → `store` (in `tangle/index/i.lu`); imports between modules must form a DAG (D32) [mod.cycle] at 17..27
```

After — `ch22/untangled/` adds `kinds/`, which imports nothing;
`index` now consumes `kinds.classify` instead of calling back into
`store`, and the arrows form a DAG:

```console
$ lupin untangled/main.lu
$ echo $?
0
```

The refactor's discipline: the extracted module holds what both sides
*needed from each other* and nothing else. If `kinds` starts importing
things, the tangle is reassembling under a new name.

**Exercise 22-6** *(comprehension · prose)* — A library refactor
splits one 900-line module file into four files in the same directory,
moves nothing across module boundaries, and changes no `pub` markers.
List everything that changes for the library's importers, then name
the artifact from this chapter's toolchain discussion that would
prove your answer mechanically.

Solution: nothing changes — the import path names the directory, the
module's namespace is the union of its files, and the `pub` surface is
untouched. The proof artifact is the module's export hash: an
interface digest computed from the `pub` surface alone, which the
split leaves bit-identical. (The corpus's visibility cases distinguish
exactly this: package-visible items alter the package hash and never
the export hash.) A refactor you can prove invisible is a refactor you
can make on a Friday.

## §22.3 — No life before main

**Exercise 22-7** *(comprehension · pending — blocker: comptime
registries (positive CTFE); owner: s16-ctfe)* — The `init()` idiom
this section retires: a plugin system where each module's `init()`
registers a handler into a global table at startup, in whatever order
the linker felt like. Sketch the comptime replacement: what builds
the table, when, and what became of the ordering question.

Solution (prose, pending the registry machinery): a comptime function
walks the declared handler types and folds the complete table into a
`const` — the table is a *value* computed at compile time, not a
mutation performed at startup. The ordering question does not get
answered; it gets deleted: there is no "before main" in which
registration races, no order in which side effects interleave, and
two builds produce the same table bit for bit (the determinism the
sandbox exists to protect — chapter 18). What `init()` could do that
the registry cannot is exactly the set of things this section is glad
to lose: arbitrary effects at an unspecified time.

## Chapter batch

**Exercise 22-8** *(design)* — Import cycles are errors (D32). A
colleague argues the compiler should permit cycles and merely warn,
citing a large codebase where breaking them means touching forty
files. Argue wolf's side using what the rule *buys*, then concede the
strongest point on the other side and answer it.

Solution (discussion): the buy is threefold. Builds: a DAG gives every
module a finish order, so compilation parallelizes and incremental
builds have a frontier — cycles collapse that into a single unit that
rebuilds together forever. Comprehension: a DAG means "what does this
depend on" has an answer that terminates; in a cycle, everything
depends on everything, and the forty files were already one file
wearing forty names. Interfaces: E0303 forced this chapter's refactor
to *name* the shared vocabulary (`kinds`), and named seams are where
documentation, testing, and ownership attach. The strongest counter is
real: retrofitting a DAG onto a tangled codebase is expensive, and a
warning would let teams migrate gradually. The answer is the one wolf
gives everywhere: gradual enforcement of a structural rule converts it
into folklore — the warned-about cycle outlives its excuse, new code
grows onto it, and the migration never happens. The forty-file cost is
paid once; the cycle's cost is paid on every build and every read,
indefinitely, by people who did not create it.
