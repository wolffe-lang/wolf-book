# Chapter 25 — Editions, stability, publishing: exercises

**None of this batch is printed.** §25.3 shipped at bs20 (the human's
2026-08-31 ruling; `wolf publish` writes the transparency record at
the pins), and the batch still waits, because every stem leans on a
subject that is still held: 25-1 through 25-5 on §25.1's edition
mechanism and §25.2's stdlib posture, 25-6 on §25.2's interface-fact
grading and a hosted registry's acceptance flow, 25-7 on §23.2's MVS,
and 25-8 on §25.1's no-2.0 rules (`book/ch25.md` carries the
measurements; TOC.md §Deltas, bs09 and bs20). They read correctly
today because every one of them reasons from a locked decision rather
than from a tool, which is also why none of them carries a pending
row: there is no program to run.

This chapter's material is policy with teeth, and its exercises are
worked the way policy is worked: by cases. Everything here is
answerable from the chapter's decisions — editions per package, the
three-ring stdlib posture, semver enforced against interface facts —
with chapters 19 and 23 supplying the mechanisms the cases lean on.

## §25.1 — Editions per package

**Exercise 25-1** *(comprehension · prose)*. Your package is edition
2027. A dependency you rely on is edition 2026 and has no plans to
move. A colleague warns this "holds back" your upgrade. What actually
happens when both compile in one build, and what precisely is the
scope of an edition?

Solution: nothing happens — each package compiles under the edition
its own manifest declares, the compiler holding both rule sets at
once, and the interface between the packages is the compiled surface,
which editions do not touch. An edition's scope is *surface syntax
and defaults within one package*: what its source may spell and what
its unstated options mean. It is not a language version, not an ABI,
not a dependency constraint. "Holds back" imports the major-version
worldview this section retires: nobody upgrades in lockstep because
nobody's edition is anybody else's business.

**Exercise 25-2** *(comprehension · prose)*. "No wolf 2.0, ever" is
a sentence about two different futures. Name the thing it promises
will never happen, the thing it conspicuously does *not* promise, and
the mechanism that makes the first promise keepable without freezing
the language.

Solution: it promises no flag day — no release that makes existing
published packages stop meaning what they meant, no ecosystem split
into before and after (Python 2/3 is the named cautionary tale). It
does not promise the language stops changing — surface can evolve
every edition. The mechanism is the pairing: editions localize
opt-in change to consenting packages, while the compiled-interface
compatibility promise holds globally across all of them. 2.0 is what
you need when you have no edition mechanism; an edition mechanism is
what you build when you have watched a 2.0.

## §25.2 — The stdlib posture

**Exercise 25-3** *(comprehension · prose)*. Three imports:
`std.list`, `std.os.path`, and `std.x.http`. Rank them by the
strength of the stability promise you are accepting, and answer the
pointed one: which of the three can break your build on a toolchain
upgrade *without anyone having done anything wrong*?

Solution: `std.list` is core — the compatibility promise at full
strength, effectively forever. `std.os.path` is penumbra — stable
intent, but platform-coupled surface with a documented, slower
deprecation channel. `std.x.http` is the nursery: `std.x` is where
APIs live *while their design is still being falsified*, and its
contract says so. That one can break you legitimately — breaking
under `std.x` is the namespace keeping its promise, not violating
it; the `x` is the consent you gave. The posture's honesty is having
a place whose instability is contractual instead of pretending the
whole stdlib is equally settled.

**Exercise 25-4** *(comprehension · prose)*. A library's public
function returns `int ! {Parse}`. In v2.3 the author wants to add a
`Range(int)` failure case. Two spellings were available back in
chapter 6: widen the closed row to `{Parse, Range(int)}`, or have
started with the open row `{Parse, ..}` in v1. State the semver
consequence of each, and what the open row traded away to buy its
minor bump.

Solution: widening the closed row is major — chapter 6's
exhaustiveness is the enforcement, since some caller matches `Parse`
with no rest arm, and that caller must stop compiling (that is the
teeth working, not failing). Growing an open row is minor — every
caller was already forced to write the `_` arm; the new tag flows
into arms that existed. The trade: open-row callers gave up
exhaustiveness as a proof — none of them can ever be told by the
compiler that they handle everything, because "everything" was
declared unfinished. The author's real decision in chapter 6 was
choosing which callers to serve: the ones who want proofs, or the
ones who want painless upgrades. Semver delivers the bill to the
address chosen.

**Exercise 25-5** *(comprehension · prose)*. Four changes to a
published package; classify each under semver-with-teeth and name
the fact source a registry check would consult (chapter 19 and this
chapter supply them): (a) a `pub` function's parameter gains a
`mut`; (b) a private helper is deleted; (c) `#[noalloc]` is added
to a hot function; (d) the package's edition moves 2026 → 2027 with
no surface change.

Solution: (a) major — the call sites must now write `mut` at the
argument (X1), so existing callers break; source: the exported
interface hash. (b) patch — private surface is invisible to
importers (chapter 22), the export hash does not move. (c) minor —
strictly more promise; removing it later would be the major (the
19-7 case); source: the contract set in the interface. (d) patch
from the world's point of view — the edition is package-local
(25-1) and the compiled surface is unchanged; the version exists so
the history says when the move happened. The pattern: semver is
computed against interface *facts* — hashes and contracts — not
against the changelog's self-assessment.

## §25.3 — Publishing

**Exercise 25-6** *(design)*. Design the registry's acceptance
gate: list what it verifies *mechanically* before `owner/pkg@v` is
served, ordered from cheapest to most expensive check, and then draw
the line — name two things a registry must not claim to verify, and
what its honest posture toward them is.

Solution (discussion): mechanically — the manifest parses and is
data (no script sections to reject because none exist to spell);
the name is the publisher's to publish under; the sum lands in the
transparency log before the bits are servable (23-5's ordering);
capabilities are declared and are a superset of what the code's
imports could reach; the semver delta is consistent with the
interface-fact diff (25-5's checks: export hash, contracts, rows).
Past the line: the registry cannot verify that the code *does* what
it claims within its granted capabilities — a `net`-holding package
may talk to any host `net` reaches — and it cannot verify quality
or maintenance. Its honest posture is to make what it did verify
legible (badges for the checks, the audit diff of 24-6) and to
refuse the security-theater badge for the rest: a registry that
implies "verified safe" sells what event-stream proved cannot be
sold. Verification's job is to shrink what trust must cover, then
say so — a smaller honest promise beats a larger decorative one.

**Exercise 25-7** *(comprehension · prose)*. Under MVS (chapter
23), can the resolver hand you a dependency version whose *edition*
is newer than your package's own? Answer, then explain why the
question feels dangerous and is not.

Solution: yes, routinely — MVS selects versions by requirement
arithmetic and never reads the edition field; your 2026 package may
build a 2028-edition dependency tomorrow. It feels dangerous because
"newer language rules than mine" pattern-matches to "code I cannot
compile." It is not, because of 25-1's scope rule: the dependency's
edition governs how *its* source is parsed, in its own compilation,
under a compiler that speaks every edition; what crosses the package
boundary is the compiled interface, which is edition-neutral. The
two chapters' mechanisms compose exactly because neither reaches
into the other's domain — MVS moves versions, editions move syntax,
and no arrow connects them.

## Chapter batch

**Exercise 25-8** *(design)*. A published package's maintainer
wants to retire the misnamed `pub fn parse_quick` in favor of
`parse_lenient`. Under no-2.0 rules, write the retirement playbook —
every step with its semver number — and then reconcile it with this
book's own numbering doctrine: what do the two systems agree a
*name* is?

Solution (discussion): the playbook — minor: introduce
`parse_lenient` beside the old name, implemented as the same
function; patch or minor: mark `parse_quick` deprecated, with the
diagnostic pointing at the replacement (a warning, machine-visible,
breaking nobody); then the fork in the road. Either the name lives
deprecated forever — cost: a permanent tombstone in the docs;
benefit: no caller ever breaks — or a major version removes it,
which no-2.0 permits *for a package* (editions killed the language
flag day, not package majors; the promise-shaped difference is that
a package major breaks only its direct, consenting upgraders). Most
of wolf's stdlib will choose the tombstone; most third-party
packages will eventually choose the major. The reconciliation:
EXERCISES.md's numbering rule — "a retired exercise leaves a
tombstone, not a renumbering" — and the registry's rule are the same
commitment at different scales: a name, once published, is an
address other people have written down, and addresses are not
reassigned while anyone might still knock.
