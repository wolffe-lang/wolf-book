# Chapter 23 — Packages and dependencies: exercises

The package manager is s51's deliverable; the manifest format and the
MVS algorithm are decided (D33 forbids build scripts; MVS is the
version rule) and are exactly the kind of material a reader can work
by hand — which is what this set does. Worked manifests below are
labeled examples in the pinned format, not tool output; exercise 23-6
records what the binary says today.

## §23.1 — `wolf.pkg` is data

**Exercise 23-1** *(comprehension · prose)* — A worked `wolf.pkg` (an
example in the pinned format, not tool output):

```text
package = "den/logsearch"
edition = "2027"
capabilities = []

[deps]
"den/rows"     = "1.4.0"
"forest/regex" = "2.1.0"
```

Answer from the file alone: what may this package do to the machine
that builds it, what may it do to the machine that runs it, and which
of those two answers required reading anything other than this file?

Solution: to the building machine — nothing beyond compilation, and
that answer needed no reading at all: the manifest is data, there is
no script section to audit, and D33 means no other file can smuggle
one in. To the running machine — also nothing ambient: `capabilities
= []` declares no `net`, no `fs`, and chapter 24 covers what enforces
it. Both answers came from this file, which is the section title as a
property: nothing about a wolf package's build behavior lives anywhere
a manifest reader cannot see.

## §23.2 — MVS in one page

**Exercise 23-2** *(comprehension · prose)* — MVS by hand, round one.
Your package requires `rows ≥ 1.2` and `regex ≥ 2.0`; `rows 1.2`
itself requires `regex ≥ 2.1`. The registry holds `rows` 1.2, 1.3,
1.5 and `regex` 2.0, 2.1, 2.3. Compute the build list, then state the
general rule your computation followed in one sentence.

Solution: `rows 1.2`, `regex 2.1`. The rule: every requirement names a
*minimum*, and the build takes the maximum of the minimums for each
package — the smallest versions that satisfy everyone, never the
newest available. 1.3, 1.5, and 2.3 exist and are not consulted;
availability is not a request.

**Exercise 23-3** *(comprehension · prose)* — Round two. To the world
of 23-2, add a new dependency `csv 1.0` which requires `rows ≥ 1.3`.
Recompute the build list and name precisely which versions moved and
which did not — the "did not" is the exercise.

Solution: `rows` rises to 1.3 — a new maximum among the minimums —
and `csv 1.0` joins. `regex` stays at 2.1: nothing raised its
minimum, and MVS has no concept of "while we're here." An upgrade
happens when a requirement demands it, and only to the demanded
package. The blast radius of adding a dependency is readable from the
requirements it brings, not from the registry's news feed.

**Exercise 23-4** *(comprehension · prose)* — Two teammates run a
build of the same commit six months apart. Under a range-and-solver
regime (`^1.2`, latest-compatible wins), name two distinct events in
the intervening months that make their binaries differ. Under MVS
from the same manifest, name what would have to happen instead.

Solution: under ranges — any dependency publishing a new compatible
version changes the resolution, and a solver heuristic update changes
how ties resolve; both are invisible to the manifest. Under MVS, the
inputs are the manifest's stated minimums, so a difference requires an
edit to the manifest itself (or a registry serving different bits for
the same version — which is 23-5's subject). "Same manifest, same
versions, forever" is not a caching strategy; it is the absence of a
solver with opinions.

## §23.3 — `wolf.sum` and the log

**Exercise 23-5** *(comprehension · prose)* — An upstream author
force-pushes tag `v1.4.0` of `den/rows` so the same version name now
serves different bytes. Walk the failure: who notices — your
`wolf.sum`, the transparency log, or both — and what does each
artifact's notice mean? Then answer the pointed one: what protects
the *first-ever* fetcher of a poisoned version, whom `wolf.sum`
cannot help?

Solution: an existing project's `wolf.sum` notices first — the fetched
bytes hash differently from the recorded sum, and the build stops; its
meaning is "not what this project verified before." The log notices
independently: the author's new hash for an existing version either
lands as a visible, permanent contradiction of the old entry or the
registry refuses it; its meaning is "not what the world saw." The
first-ever fetcher has no local history, so only the log defends
them: their client checks the version's hash against the public,
append-only record before trusting it. `wolf.sum` protects your
project from change; the log protects everyone from a lie agreed to
early — "even the author cannot swap bits under a tag" is the log's
sentence, not the lockfile's.

**Exercise 23-6** *(spelunking · wolf)* — Ask the binary to add a
dependency and to audit one. Read what comes back, and reconcile it
with this chapter's existence.

Solution — real runs, both:

```console
$ wolf add example/pkg
wolf add: not yet (grows at its own campaign; D34's single binary)
$ wolf audit
wolf audit: not yet (grows at its own campaign; D34's single binary)
```

The binary declines rather than guessing at behavior — the ledger
posture again. The chapter can exist because the *decisions* (D33,
MVS, the sum-and-log design) are locked and the formats pinned; the
exercises that need the tool say so and wait (23-8), and the ones you
can do by hand, you did.

*(Filed for the chapter sprint that publishes this material: same
finding as exercise 20-4 — a refusal is not product output, and this
stem waits for `wolf add`/`wolf audit`.)*

## §23.4 — Script mode, demystified

**Exercise 23-7** *(comprehension · pending — blocker: script-mode
dependency resolution; owner: s51-package-manager with s31-driver-v0)*
— Chapter 1 promised that a script's frontmatter deps are "the same
machinery." Given this script header (pinned format, not yet
executable):

```text
#! deps: "den/rows" = "1.4.0"
```

state what resolution work happens on first run, where the versions
come from on the second run, and which chapter-23 artifact the script
implicitly carries even though no `wolf.sum` file sits beside it.

Solution (prose, pending execution): first run — the frontmatter *is*
a manifest, so MVS runs over its requirements exactly as for a
package; second run — the same minimums produce the same versions,
MVS needing no lockfile to be deterministic. The implicitly carried
artifact is the sum verification: fetched bytes are checked against
the transparency log's record, the cache remembers them, and the
script gains the same swap-proof property as a project. "No venv, no
requirements.txt, no drift" is chapter 1's spelling of this chapter's
determinism.

## Chapter batch

**Exercise 23-8** *(design)* — MVS's determinism has a price: your
users do not receive a dependency's bug fixes until you raise a
minimum by hand. A colleague calls this a security liability and
wants ranges back. Take wolf's side without dodging the liability —
name the mechanism that answers it and the reason auto-upgrading is
the wrong layer for the answer.

Solution (discussion): concede the fact — under MVS, a fix you have
not asked for is a fix you do not have. The mechanism that answers it
is advisory tooling above the resolver: an audit that reads your
build list against a vulnerability feed and *tells you* which
minimums to raise turns upgrades into reviewed, attributable diffs —
one line in `wolf.pkg`, one entry in review history. Auto-upgrade
answers at the wrong layer because it converts every upstream publish
into an unreviewed change to your shipped artifact: the same channel
that delivers a fix delivers a compromise (chapter 24's event-stream
is the canonical case — the malicious version arrived as a routine
compatible upgrade nobody read). Determinism does not slow the urgent
fix; raising a minimum is one edit. What it removes is the *silent*
path — and silent is what the attacker was renting.
