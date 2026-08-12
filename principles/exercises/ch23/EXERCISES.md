# Chapter 23 — Packages and dependencies: exercises

The package manager ships, and this batch splits along that line.
23-1, 23-5, 23-6 and 23-8 belong to the two sections chapter 23
prints, and every manifest and transcript below is the real format
and a real run. 23-2, 23-3 and 23-4 work minimal version selection by
hand; they are correct and they are **not printed**, because §23.2 is
held — no manifest a reader can write makes the resolver choose
between two versions of anything at this pin, and homework for a
section that does not exist is homework nobody assigned (TOC.md
§Deltas, bs09). 23-7 keeps its pending row.

## §23.1 — `wolf.pkg` is data

**Exercise 23-1** *(comprehension · prose)* — The manifest §23.1
prints, which is a real file in the book's fixtures:

```wolf
pkg {
    name:    "den/logsearch",
    version: "0.1.0",
    edition: "1",

    deps: {
        rows: { path: "../rows" },
    },
}
```

Answer from the file alone: what may this package do to the machine
that builds it, what may it do to the machine that runs it, and which
of those two answers required reading anything other than this file?

Solution: to the building machine — nothing beyond compilation, and
that answer needed no reading at all: the manifest is data, there is
no script section to audit, and D33 means no other file can smuggle
one in. To the running machine — the manifest declares no
`capabilities`, which is the empty set, so no `net`, no `fs`, no
`env`; chapter 24 covers what enforces that. The second answer needed
one more file, and only one: `../rows`'s own manifest, because
`effective` capabilities are the union over the graph and a dependency
may declare what you did not. `wolf audit` reads that union for you,
which is the mechanical version of this exercise.

## §23.2 — MVS in one page

**Exercise 23-2** *(comprehension · prose — written, not printed with §23.2)* — MVS by hand, round one.
Your package requires `rows ≥ 1.2` and `regex ≥ 2.0`; `rows 1.2`
itself requires `regex ≥ 2.1`. The registry holds `rows` 1.2, 1.3,
1.5 and `regex` 2.0, 2.1, 2.3. Compute the build list, then state the
general rule your computation followed in one sentence.

Solution: `rows 1.2`, `regex 2.1`. The rule: every requirement names a
*minimum*, and the build takes the maximum of the minimums for each
package — the smallest versions that satisfy everyone, never the
newest available. 1.3, 1.5, and 2.3 exist and are not consulted;
availability is not a request.

**Exercise 23-3** *(comprehension · prose — written, not printed with §23.2)* — Round two. To the world
of 23-2, add a new dependency `csv 1.0` which requires `rows ≥ 1.3`.
Recompute the build list and name precisely which versions moved and
which did not — the "did not" is the exercise.

Solution: `rows` rises to 1.3 — a new maximum among the minimums —
and `csv 1.0` joins. `regex` stays at 2.1: nothing raised its
minimum, and MVS has no concept of "while we're here." An upgrade
happens when a requirement demands it, and only to the demanded
package. The blast radius of adding a dependency is readable from the
requirements it brings, not from the registry's news feed.

**Exercise 23-4** *(comprehension · prose — written, not printed with §23.2)* — Two teammates run a
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
serves different bytes. Walk the failure: what does your `wolf.sum`
notice, what does the build do about it, and what would you have to run
to tell wolf the change was intended? Then answer the pointed one: what
does `wolf.sum` *not* protect, and who is exposed to that gap?

Solution: the ledger recorded a `b3:` digest over the dependency's
source tree the last time a human fetched it. The next build re-derives
that digest, finds a different number, and refuses with E1506 — "the
bits on disk are not the bits the ledger witnessed". Nothing is
guessed and nothing is trusted except the hash: the tag is a name, the
digest is the content, and the build compares content, which is why a
moved tag is a broken build rather than a silent substitution. Telling
wolf the change was intended is one command, `wolf update`, and the
point of making it a command is that it produces a reviewable diff: the
new digest lands in `wolf.sum` where a human signs off on it.

The gap: `wolf.sum` is a memory, so it protects only what it has
already seen. The first-ever fetcher of a poisoned version has no
recorded digest to compare against — they hash the malicious bytes and
record them as the truth. That is not a flaw in the ledger, it is the
boundary of what a per-project witness can do, and closing it needs a
record the whole world shares rather than one your project keeps.
Content addresses and an append-only record of published versions are
the designed answer, and what a reader can rely on today is the
narrower guarantee: bits cannot change under you without the build
saying so.

**Exercise 23-6** *(spelunking · wolf)* — Run `wolf tree` and then
`wolf why` on a project with one dependency, and then run `wolf why` for
a name that is not in the graph. Report all three exit codes, and say
what the third one is for.

Solution — real runs, in the `shelf` fixture §23.1 prints:

```console
$ wolf tree --dir app
capability tree (I13)
den/logsearch 0.1.0 (root) caps=[]
└── rows 1.4.0 caps=[]
effective: []
$ echo $?
0
$ wolf why rows --dir app
den/logsearch (root) -> rows 1.4.0
$ echo $?
0
$ wolf why ghost --dir app
wolf why: `ghost` is not in the resolved graph
$ echo $?
1
```

The third exit code is the one worth having. `wolf why` for a name that
is not there is not a usage error — the command was spelled correctly —
so it is not exit 2; it is a *finding*, and exit 1 is what a finding
gets. That makes both directions scriptable: `wolf why X` succeeding
means X is in your build, and failing means it is not, which is a
one-line check in a pipeline rather than a grep over a tree.

## §23.4 — Script mode, demystified

**Exercise 23-7** *(comprehension · pending — blocker: a script's
frontmatter dependencies have no spelling; owner: s51-package-manager
with s31-driver-v0 — written, not printed with §23.4)*
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

**Exercise 23-8** *(design)* — Wolf's determinism has a price: your
users do not receive a dependency's bug fixes until you raise a
minimum by hand. A colleague calls this a security liability and
wants ranges back. Take wolf's side without dodging the liability —
name the mechanism that answers it and the reason auto-upgrading is
the wrong layer for the answer.

Solution (discussion): concede the fact — when versions move only
because a human moved them, a fix you have not asked for is a fix you
do not have. The mechanism that answers it
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
