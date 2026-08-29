# Chapter 24 — The covenant: no build scripts: exercises

Commands run from this directory; outputs are pasted from real runs.
The history exercises cite the incidents §24.1 sources; their solutions
reason from the section's own citations rather than importing new
claims. The enforcement exercises run today — the comptime sandbox is
live, the capability tree is real, and 24-6 is a walkthrough rather
than a sketch. 24-4 and 24-5 print their code and not the compiler's
rendered stanza, for the reason chapter 24's ledger records: E0701's
note carries a campaign id, so the rendered text is CI's business (it
is snapshot-checked on every run) and not the page's.

## §24.1 — The threat, from history

**Exercise 24-1** *(spelunking · prose)*. The event-stream incident
(2018), from §24.1's sourcing: a maintainer handed a popular npm
package to a volunteer, who shipped a version whose install-time and
runtime code targeted a specific downstream wallet application. List
the three legs the attack stood on — distribution, execution, and
concealment — and for each leg, name the wolf mechanism from this
part of the book that removes it or forces it into the open, with one
sentence on the residue each mechanism cannot remove.

Solution: distribution — a compatible-version publish flowed
automatically to downstreams; MVS (chapter 23) removes the automatic
part, since a new version reaches you when you raise a minimum, as a
reviewable diff. Residue: you can still raise it without reading.
Execution — install scripts ran arbitrary code on every fetching
machine; the covenant deletes the phase (D33: no scripts), and the
comptime sandbox refuses ambient reach at build time (24-4, 24-5
below). Residue: *runtime* malice in code you call remains possible —
which is what capabilities are for. Concealment — the payload hid in
a minified transitive dep nobody read; capability manifests plus
`wolf audit` (see 24-6) make "this dep now wants `net`"
a surfaced diff instead of archaeology. Residue: a malicious payload
*within* already-granted capabilities. The lesson the three residues
teach together: the covenant shrinks the attack surface to the part a
human must still review, and makes that part small enough to review.

**Exercise 24-2** *(comprehension · prose)*. Left-pad (2016) took
thousands of builds down without executing a byte of anyone's code.
State what kind of failure it was, why the comptime sandbox is
irrelevant to it — the pointed half of the question — and which
chapter-23 artifacts answer it instead.

Solution: an availability failure — a published name was withdrawn
and every build that resolved it fresh broke. The sandbox is
irrelevant because nothing malicious ran; no amount of execution
policy helps when the failure is *absence*. The answers are chapter
23's: the immutable registry and transparency log (a published
version cannot be unpublished into a lie), the local module cache,
and vendoring for the paranoid tail. Distinguishing "code I must not
trust" from "infrastructure I must not depend on" is the exercise;
conflating them is how teams buy a sandbox and still go down on a
Tuesday.

**Exercise 24-3** *(comprehension · prose)*. `build.rs` is the
mechanism §24.1's third paragraph indicts. Name three legitimate jobs
build scripts do in the Rust ecosystem, and for each, the covenant-
compatible replacement this part of the book offers. Then name the
job for which the honest answer is "v1 cannot vendor that" (§24.4's
subject).

Solution: generating code from a schema — comptime evaluation over the
schema *as a declared build input* (chapter 18's refusal note points
there, and the package manifest is where the declaration lives).
Discovering platform facts —
target metadata in the manifest and comptime conditionals over the
declared target, not probes of the build host. Compiling and linking
a bundled C library — the declarative recipe layer for the common
shapes, and pre-built artifacts through the membrane (chapter 9) for
the rest. The honest residue: the autotools-shaped dependency whose
build is itself a Turing-complete configuration program — ./configure
logic cannot be declared, only executed, and v1 refuses to execute
it. That refusal is priced in §24.4, not hidden.

## §24.2 — What replaces scripts

**Exercise 24-4** *(comprehension · wolf)*. The dependency that
phones home at build time, spelled as directly as wolf's syntax
allows. Predict the E-code and *which of the catalog's two refusal
reasons* the diagnostic will cite (they differ by capability —
chapter 18 sorted them):

```wolf
comptime fn latest_ad() -> str {
    net_fetch("https://deps.example.test/banner")
}
```

Solution: E0701, and the **confinement** reason — the diagnostic's first
note says so in those words, and names the scenario outright: `wolf add`
must never mean arbitrary code talks to the network with your
credentials. It arrives from the type checker, at the call inside the
`comptime fn`, with a second span at the `const` that entered the
evaluation. The sample is `ex24-4.lu` and CI checks it as a
`fail(E0701)` with a reviewed snapshot, so the exact text is verified
even where it is not printed.

The covenant is not a policy document; it is this rejection, emitted
before anything runs.

**Exercise 24-5** *(comprehension · wolf)*. The build step that reads
your CI secrets: `env_var("CI_DEPLOY_TOKEN")` inside a `comptime fn`.
Predict the refusal reason this one cites — and note before running
that 18-6 filed environment reads under one reason, while the
diagnostic gives this capability a compound answer.

Solution. The environment gets both barrels: the note reads
"determinism and confinement", because environment contents differ per
machine *and* may hold secrets. That is the compound answer the stem
warns about: chapter 18's exercise 18-6 asks a reader to sort a clock
read, a network fetch, and an environment read under the catalog's two
categories, and the catalog files the environment under confinement
while the diagnostic for this capability names both. Both are right
about different things, which is worth noticing: the category is what
the rule is *for*, and the note is why this particular intrinsic is
refused. The sample is `ex24-5.lu`, checked as a `fail(E0701)` with its
own snapshot.

The scenario this kills is the one event-stream normalized: code you
did not read, running at build time, in possession of what your CI
knows. Here that code does not get to run at all.

## §24.3 — Capabilities and `wolf audit`

**Exercise 24-6** *(comprehension · wolf)*. Take a project with one
dependency whose manifest declares no capabilities, record the world with
`wolf update`, then edit the dependency's manifest to declare `net` and
run `wolf audit --ci`. Report the exit code and the line that produced
it. Then say which artifact held the *previous* answer, and what would
have happened if that artifact had been refreshed first.

Solution. The walkthrough, in a project shaped like §23.1's:

```console
$ wolf update --dir app
wolf update: wolf.sum refreshed (1 entry)
$ wolf audit --ci --dir app
capability tree (I13)
den/logsearch 0.1.0 (root) caps=[]
└── regex 2.2.0 caps=[net]
effective: [net]
wolf audit: `regex` ACQUIRES capability `net` (was not in wolf.sum)
wolf audit: capability acquisition detected — refusing (--ci)
$ echo $?
1
```

Exit 1, from the ACQUIRES line: a text-matching library now wants the
network, and `--ci` treats acquisition as a finding rather than news.
The artifact holding the previous answer is `wolf.sum` — its last field
is the capability set, recorded the last time a human accepted the
world. That is why the ordering in the tool is not an accident: `wolf
audit` reports the diff *before* any verb rewrites the ledger, so
refreshing first destroys the evidence. Run `wolf update` before you
audit and the ledger now says `caps=net`, the diff is empty, the gate
passes, and the only trace of the change is a line in a file you did not
read. The gate is only a gate while the ledger is behind you.

The question the failing build forces is not "is this library
malicious" — that has no answer you can reach — but "what, concretely,
will you do with `net`". The upgrade will have a plausible reply ready,
and the most plausible one ("it downloads updated Unicode tables") is
exactly the one to refuse: it moves data acquisition from publish time,
where the tables are baked into a hashed artifact anyone can audit, to
run time on your machines with your network.

## §24.4 — What the covenant costs

**Exercise 24-7** *(design)*. Pick the strongest real case against
the covenant: a widely-needed C library whose build is a thicket of
`./configure` feature detection (the class §24.4 names as what v1
cannot vendor). The maintainer of a wolf wrapper asks for "one
escape hatch — a sandboxed build script, network off, fs jailed to
the package directory." Argue the refusal, then state what the
covenant's answer costs this maintainer in practice and why the line
holds anyway.

Solution (discussion): the sandboxed-script hatch fails on three
grounds. Precedent — the moment one package may run a jailed script,
every audit answer degrades from "packages cannot run build code" to
"packages cannot run build code except the ones that can," and the
exception list becomes the attack surface. Fidelity — a jail tight
enough to keep the covenant's promises (no host probes, no
environment reads) breaks `./configure` anyway, because host-probing
is that program's entire method; the hatch would be both dangerous
and useless. Determinism — feature detection's *output* depends on
the build host by design, which is the reproducibility hole D33
closed. The real cost lands on the maintainer: they must pre-build
artifacts per target (chapter 9's membrane consumes them), or
translate the feature matrix into declared target metadata by hand —
genuine, unglamorous work, borne by the few. The line holds because
the alternative distributes a worse cost to everyone else: every
consumer of every package re-auditing what build-time execution
might do. The covenant prices the pain onto the package that has the
exotic build, and keeps the ecosystem's default trustable by
reading a manifest.

## Chapter batch

**Exercise 24-8** *(comprehension · prose)*. "It's only a
dev-dependency" — a teammate waves through a test-helper package
whose new version adds comptime code, on the grounds that it ships
nothing to production. Locate the two errors, using this chapter and
one fact from chapter 18.

Solution: error one — build-time compromise does not care about
shipping: comptime code in any dependency evaluates inside your
build, on your CI, where the credentials live (the sandbox is what
stands between them, and 24-5 showed it holding — but the *audit*
question "why does a test helper now need comptime at all" remains a
human's to ask). Error two — test code runs with your project's full
runtime capabilities every time CI executes the suite, and its
output gates merges; code that can fake a green check ships things
to production without ever being in the artifact. The chapter-18
fact that anchors both: comptime is ordinary wolf evaluated by the
compiler — there is no "harmless phase," only phases with different
blast radii. Dev-dependencies are dependencies; the qualifier names
their schedule, not their trust level.
