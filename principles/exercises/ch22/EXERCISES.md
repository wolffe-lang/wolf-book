# Chapter 22 — Modules: the shape of a wolf project: exercises

This chapter's exercises are multi-file by nature: each runnable one is
a small directory under `ch22/`, with the entry file named in the
solution. Commands run from this directory; outputs are pasted from
real runs. The directive-header rule from the Notation chapter is load-
bearing here: sibling `.lu` files are one module, and `member: true`
marks the files that only exist through their directory.

## §22.1 — Directory = module

**Exercise 22-1** *(fingers · lupin)*. Build the two-module project:
an entry file and a `stats/` directory exporting `mean`, with a
private `total` helper the entry never sees. Run it. Then move
`total` into a second file inside `stats/` and state what changes for
the entry file.

Solution. `ch22/metrics/`:

```wolf
// metrics/main.lu
use stats

fn main() -> !int {
    var widths = List[int]()
    (mut widths).push(4)
    (mut widths).push(6)
    (mut widths).push(8)
    print("mean {stats.mean(widths)} of {stats.count(widths)}")
    0
}
```

```console
$ lupin metrics/main.lu
mean 6 of 3
```

Moving `total` to another file inside `stats/` changes nothing
anywhere, because files are invisible to importers: the module is the
directory, `use stats` names it whole, and the split is a private
reorganization. That non-event is the design.

**Exercise 22-2** *(comprehension · lupin)*. `vault/keys.lu` defines
`pub fn count()`, `pub fn loaded()`, and private `fn secrets()` and
`fn total()`. The entry calls `vault.total()`. Predict the diagnostic
(including whether it says the name does not *exist*) and the exit code.

Solution: E0304, exit 2, and the diagnostic is precise about
existence: the name is there and visibility is the objection. A
resolver that pretended otherwise would send you hunting a typo that
is not one:

```console
$ lupin leak/main.lu
leak/main.lu: E0304: `total` exists in `vault`, but it is private; only `pub`/`pub(pkg)` items are visible across modules (D32) [mod.vis.private] at 7:19
$ echo $?
2
```

**Exercise 22-3** *(comprehension · lupin)*. `twice/main.lu` and its
sibling `twice/extra.lu` each define `fn describe()`. Neither file
imports the other. Predict the verdict, and say why "neither imports the
other" is a trap in the question.

Solution: E0302, "file boundaries create no scopes." The trap is
thinking imports are involved at all: sibling files are not two scopes
that could shadow, they are one module with one namespace, and the
second definition is a duplicate wherever it sits:

```console
$ lupin twice/main.lu
twice/main.lu: E0302: the name `describe` is defined twice in this module (defined again in `twice/main.lu`); file boundaries create no scopes (D32) — two separate programs sharing a directory each mark themselves `//! member: false` (D59) [mod.dup] at 4:4
```

**Exercise 22-4** *(comprehension · lupin)*. The entry imports `tools`
and never mentions it again. Predict: warning or error, and what the
diagnostic offers about the fix.

Solution: a hard error, E0305, and the diagnostic notes the fix is
machine-applicable: deleting the line. Wolf takes the Go position
with Go's justification: an unused import is a dependency edge that
slows every build and means nothing, and a warning would be a request:

```console
$ lupin unused/main.lu
unused/main.lu: E0305: the import `tools` is never used in `unused/main.lu`; an unused import is a hard error (D32), and deleting the line is machine-applicable [mod.use.unused] at 4:5
```

**Exercise 22-11** *(fingers · lupin)*. Two scratch programs, one
directory, on purpose: `sum.lu` totals three numbers, `widest.lu`
finds the longest of four words, and each is a whole program with its
own `main`. Mark each `//! member: false` and run both by name. Then
remove the marker from *one* of them and run the other. Why does the
diagnostic land on the program that kept its marker?

Solution. `ch22/scratch/`, both files opening with the marker:

```wolf
// scratch/sum.lu (first lines)
//! member: false
fn main() -> !int {
```

```console
$ lupin scratch/sum.lu
sum 15
$ lupin scratch/widest.lu
widest marmot
```

With `widest.lu`'s marker removed, running `sum.lu` refuses:

```console
$ lupin sum.lu
sum.lu: E0302: the name `main` is defined twice in this module (defined again in `./widest.lu`); file boundaries create no scopes (D32) — two separate programs sharing a directory each mark themselves `//! member: false` (D59) [mod.dup] at 1:4
```

The marker is a property of the *file*, not of the invocation: an
unmarked sibling defaults to membership in whatever module the entry
anchors, so the bare `widest.lu` joins `sum.lu`'s module and brings
its `main` along. The diagnostic lands on the marked program because
that is the program you ran — its module is where the collision
happened. E0302's hint says "each mark themselves" with "each" doing
real work: standalone-ness is declared per file, never inferred from
a neighbor's declaration.

**Exercise 22-12** *(comprehension + extension · lupin)*. In
`ch22/clash/`, the `labels` module is two files — `upper.lu` and
`banner.lu` — and both define `pub fn title`. Predict the diagnostic
before running (22-3 is the same law; what differs here?). Then fix
it in a copy *without deleting either file*, and say what the fix's
choices were.

Solution. Before:

```console
$ lupin clash/main.lu
clash/main.lu: E0302: the name `title` is defined twice in this module (defined again in `./labels/upper.lu`); file boundaries create no scopes (D32) — two separate programs sharing a directory each mark themselves `//! member: false` (D59) [mod.dup] at 3:14
```

What differs from 22-3 is only where the union happens: these two
files are members of a *named* module rather than the entry's own,
and the module's namespace is still the union of its files, so the
second `title` is a duplicate wherever it sits. The `member: false`
hint in the message is a red herring here, on purpose — these files
are not two programs, they are two halves of one module with one name
too many. After, `ch22/unclashed/` renames `banner.lu`'s function to
`banner`:

```console
$ lupin unclashed/main.lu
== wolf ==
** wolf **
```

The choices were exactly two, because the collision is one name with
two owners: rename one function (taken), or merge the two spellings
into one file under one `title` with a parameter. Moving `banner.lu`
to another directory is not a fix — it is a new module, and callers
would have to know which spelling lives where, which is the coupling
the rename avoids.

## §22.2 — No cycles

**Exercise 22-5** *(comprehension + extension · lupin)*. In
`ch22/tangle/`, `store` imports `index` to log entries and `index`
imports `store` to validate them: each import has a reason, which is
how real cycles are born. Predict the diagnostic. Then perform the
interface-extraction refactor in a copy: move the shared vocabulary
into a third module neither imports from, and run the result.

Solution. Before, the cycle drawn whole:

```console
$ lupin tangle/main.lu
tangle/main.lu: E0303: this import completes a cycle: `store` → `index` → `store` (in `tangle/index/index.lu`); imports between modules must form a DAG (D32) [mod.cycle] at 3:48
```

After, `ch22/untangled/` adds `kinds/`, which imports nothing;
`index` now consumes `kinds.in_batch` instead of calling back into
`store`, and the arrows form a DAG:

```console
$ lupin untangled/main.lu
stored 0
```

The refactor's discipline: the extracted module holds what both sides
*needed from each other* and nothing else. If `kinds` starts importing
things, the tangle is reassembling under a new name.

**Exercise 22-6** *(comprehension · prose)*. A library refactor
splits one 900-line module file into four files in the same directory,
moves nothing across module boundaries, and changes no `pub` markers.
List everything that changes for the library's importers, then name
the artifact from §22.2 that would prove your answer
mechanically.

Solution: nothing changes. The import path names the directory, the
module's namespace is the union of its files, and the `pub` surface is
untouched. The proof artifact is the module's export hash, which
`wolf interface` prints: a digest over the `pub` surface alone, which
the split leaves bit-identical. Run it before and after and compare the
`export_hash` line; a private helper moving between files does not
appear in the items list, so it cannot appear in the number. A refactor
you can prove invisible is a refactor you can make on a Friday.

**Exercise 22-13** *(spelunking · wolf)*. 22-6 argued from the export
hash; now hold it in your hands. Run `wolf interface` on 22-9's
`tokens` module, add a private helper to the file, and run it again.
Two things to explain from the output: why the hash did not move, and
what the `W0313` warning beside it is asking for.

Solution. Both runs, one item and one number between them:

```console
$ wolf interface ./tokens/tokens.lu
module pkg :: (root)
  wolfi v0 · toolchain 0.2.3 · edition v1
  export_hash 5e861f2235ff8d9e79fd0403a4346ad0afe0693f0524822b644f83f234bd0ed0
  pkg_hash    5e861f2235ff8d9e79fd0403a4346ad0afe0693f0524822b644f83f234bd0ed0
  deps: (none)
  items:
    [0] pub split_words — fn split_words(text: str) -> prelude.List[str] · regions (-) -> ρ_caller
```

After `fn spare() -> int { 0 }` is appended, the output is
byte-identical: the hash digests the `pub` surface alone, `spare` is
private, and a private item is invisible to the number for the same
reason it is invisible to importers. (The doc comments on the `pub`
items do not move the hash either — contracts travel with the
interface, but the digest is over the signatures.) `W0313` fires when
a `pub` item has no `///` line: "exported, but undocumented", with
the note that an item not worth documenting is rarely worth
exporting. The warning and the hash are the same doctrine at two
strengths — the module's public face is a contract, the hash makes
its *shape* checkable, and the doc comment is where its *meaning*
goes.

## §22.3 — No life before main

**Exercise 22-7** *(comprehension · wolf)*. The `init()` idiom §22.3
retires: a plugin system where each module's `init()` registers a handler
into a global table at startup, in whatever order the linker felt like.
Write the comptime replacement for **four** handlers, with a witness that
fails the build if one goes missing, and say what became of the ordering
question.

Solution. `ex22-7.lu`, run by the compiler because a `comptime fn` is
the compiler's to evaluate:

```wolf
struct Ingest  { rows: int }
struct Report  { rows: int }
struct Purge   { rows: int }
struct Reindex { rows: int }

comptime fn handlers(a: type, b: type, c: type, d: type) -> str {
    "{typeinfo(a).name} {typeinfo(b).name} {typeinfo(c).name} {typeinfo(d).name}"
}

comptime fn expect_four(a: type, b: type, c: type, d: type) -> bool {
    assert(handlers(a, b, c, d).len == 27)
    true
}

fn main() -> !int {
    const HANDLERS = handlers(Ingest, Report, Purge, Reindex)
    const CHECKED = expect_four(Ingest, Report, Purge, Reindex)
    print("{HANDLERS}")
    if CHECKED { 0 } else { 1 }
}
```

```console
$ wolf run ex22-7.lu
Ingest Report Purge Reindex
```

What builds the table: `handlers`, during compilation. When: before the
program exists. What became of the ordering question: it was deleted, not
answered. There is no phase in which two registrations could race, no
link order to depend on, and two builds of this file produce the same
table byte for byte, the determinism the sandbox exists to protect
(chapter 18). Drop `Reindex` from either call and the witness fails the
build with E0710 rather than leaving you a table that is quietly one
handler short, which is the second thing `init()` never gave you.

## Chapter batch

**Exercise 22-8** *(design)*. Import cycles are errors (D32). A
colleague argues the compiler should permit cycles and merely warn,
citing a large codebase where breaking them means touching forty files.
Argue wolf's side using what the rule *buys*, then concede the strongest
point on the other side and answer it.

Solution (discussion): the buy is threefold. Builds: a DAG gives every
module a finish order, so compilation parallelizes and incremental
builds have a frontier; a cycle collapses that into a single unit
that rebuilds together forever. Comprehension: a DAG means "what does this
depend on" has an answer that terminates; in a cycle, everything
depends on everything, and the forty files were already one file
wearing forty names. Interfaces: E0303 forced this chapter's refactor
to *name* the shared vocabulary (`kinds`), and named seams are where
documentation, testing, and ownership attach. The strongest counter is
real: retrofitting a DAG onto a tangled codebase is expensive, and a
warning would let teams migrate gradually. The answer is the one wolf
gives everywhere: gradual enforcement of a structural rule converts it
into folklore. The warned-about cycle outlives its excuse, new code
grows onto it, and the migration never happens. The forty-file cost is
paid once; the cycle's cost is paid on every build and every read,
indefinitely, by people who did not create it.

**Exercise 22-9** *(extension · lupin)*. The word counter, split along
its seam: a `tokens/` module that turns text into words and knows
nothing about counting, a `tally/` module that counts a `List[str]`
and knows nothing about lines, and an entry that owns the text and the
report loop. Build it in `ch22/wordcount/` and run it. Then defend the
seam: why does `tokens` return a `List[str]` instead of taking `tally`
as an import and counting as it splits?

Solution. `ch22/wordcount/` — the entry:

```wolf
// wordcount/main.lu
use tokens
use tally

fn main() -> !int {
    let log = """
        the wolf runs and the moon watches
        the wolf sleeps
        """
    let words = tokens.split_words(log)
    let pairs = tally.count(words)
    var i = 0
    while i < pairs.0.len {
        print("{pairs.1[i]:>3} {pairs.0[i]}")
        i += 1
    }
    0
}
```

```console
$ lupin main.lu
  3 the
  2 wolf
  1 runs
  1 and
  1 moon
  1 watches
  1 sleeps
```

The `List[str]` between them is the whole contract, and its poverty is
the point: `tokens` importing `tally` would weld the two jobs into one
module wearing two directories — counting-while-splitting cannot be
reused to split without counting, and the import edge would put
`tally` in every future importer's build graph whether they count or
not. A seam earns its module boundary exactly when the value crossing
it is duller than either side's insides. (`tally` returns two parallel
lists for 5-6's reason: first-seen order is data a `Map` forgets.)

**Exercise 22-10** *(extension · lupin)*. A calculator whose
arithmetic lives behind a seam: an `ops/` module exporting one
function, `apply(op, a, b) -> int ! {BadOp, DivZero}`, and an entry
that parses `a b op` lines and reports. Build it in `ch22/calc/`,
with `9 0 /` among the inputs. The row crosses the module boundary —
what does the entry know about *why* a line was refused, and what
would it take to start caring?

Solution. `ch22/calc/` — the module:

```wolf
// calc/ops/ops.lu
pub fn apply(op: str, a: int, b: int) -> int ! {BadOp, DivZero} {
    if op == "+" { return a + b }
    if op == "-" { return a - b }
    if op == "*" { return a * b }
    if op == "/" {
        if b == 0 { return DivZero }
        return a / b
    }
    BadOp
}
```

```console
$ lupin main.lu
7 3 - = 4
9 0 /: refused
6 7 * = 42
```

The entry's handler is `else |err| { … }` with no `match`: it knows a
line was refused and prints so, and the tag's identity dies unread in
`err`. That is a legitimate posture — chapter 6's coarse consumer —
and the row's two tags are still load-bearing, because the day the
entry wants `9 0 /: division by zero` instead, the change is one
`match` in the handler and nothing in `ops`: the information was
already crossing the seam, typed, waiting for a caller that cares.
An error row in a `pub` signature is the module promising its callers
room to grow into.

**Exercise 22-14** *(design)*. 22-9's `tokens` exports one function;
its `index_of` helper in `tally` is private; 22-10's `ops` exports
`apply` and nothing else. State the rule these three choices follow,
then argue against the tempting alternative: why not export the
helpers too, since a future caller might want them? Name what every
`pub` costs its module under 22-6's hash, and when the answer flips.

Solution (discussion): the rule is the interface chapter's — export
what callers need to do their job, keep everything whose *shape you
might change*. Every `pub` widens the export hash: it becomes a
signature importers may depend on, a name W0313 wants documented, a
row in every future "can we change this?" conversation, and — under
22-6's proof — a thing whose alteration is *visible* in the number.
The speculative export costs all of that now against a caller who may
never arrive; and when that caller does arrive, promoting a private
helper to `pub` is a one-line diff whose hash change tells the truth
about what happened. The answer flips when the helper *is* the
product — a utility module whose whole reason is its helpers — and
the honest test is whether you can write the `///` contract W0313
asks for without the word "internal" in it.
