# Chapter 24 — The covenant: no build scripts: exercises

Commands run from this directory; outputs are pasted from real runs.
The history exercises cite the incidents §24.1 sources; their solutions
reason from the section's own citations rather than importing new
claims. The enforcement exercises run today — the comptime sandbox is
live in wolf, and two of its refusals below are this chapter's whole
argument in diagnostic form.

## §24.1 — The threat, from history

**Exercise 24-1** *(spelunking · prose)* — The event-stream incident
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
`wolf audit` (pending s51; see 24-6) make "this dep now wants `net`"
a surfaced diff instead of archaeology. Residue: a malicious payload
*within* already-granted capabilities. The lesson the three residues
teach together: the covenant shrinks the attack surface to the part a
human must still review, and makes that part small enough to review.

**Exercise 24-2** *(comprehension · prose)* — Left-pad (2016) took
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

**Exercise 24-3** *(comprehension · prose)* — `build.rs` is the
mechanism §24.1's third paragraph indicts. Name three legitimate jobs
build scripts do in the Rust ecosystem, and for each, the covenant-
compatible replacement this part of the book offers. Then name the
job for which the honest answer is "v1 cannot vendor that" (§24.4's
subject).

Solution: generating code from a schema — comptime evaluation over the
schema *as a declared build input* (chapter 18's refusal note points
there; s51 delivers the declaration). Discovering platform facts —
target metadata in the manifest and comptime conditionals over the
declared target, not probes of the build host. Compiling and linking
a bundled C library — the declarative recipe layer for the common
shapes, and pre-built artifacts through the membrane (chapter 9) for
the rest. The honest residue: the autotools-shaped dependency whose
build is itself a Turing-complete configuration program — ./configure
logic cannot be declared, only executed, and v1 refuses to execute
it. That refusal is priced in §24.4, not hidden.

## §24.2 — What replaces scripts

**Exercise 24-4** *(comprehension · wolf)* — The dependency that
phones home at build time, spelled as directly as wolf's syntax
allows. Predict the E-code and *which of the catalog's two refusal
reasons* the diagnostic will cite (they differ by capability —
chapter 18 sorted them):

```wolf
comptime fn latest_ad() -> str {
    net_fetch("https://deps.example.test/banner")
}
```

Solution: E0701, and the confinement reason — with `wolf add` itself
in the note's example scenario:

```console
$ wolf conform-run ./ex24-4.lu
error[E0701]: `net_fetch` reaches the network, which comptime code can never touch
 --> ./ex24-4.lu:6:5
  |
6 |     net_fetch("https://deps.example.test/banner")
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ambient IO at compile time
...
9 |     const AD = latest_ad()
  |                ----------- while evaluating `latest_ad`, entered here
  |                ----------- while evaluating `main`, entered here
  |
  = note: why it is refused — confinement: `wolf add` must never mean arbitrary code talks to the
    network with your credentials.
  = note: the comptime sandbox is hermetic (D33): the intrinsics available at compile time are an
    explicit allowlist, and nothing ambient is on it. Compute this value at runtime instead;
    file contents will arrive later as declared build inputs (s51), never as an evaluator
    capability.
```

The covenant is not a policy document; it is this rejection, emitted
by the type checker, before anything runs.

**Exercise 24-5** *(comprehension · wolf)* — The build step that reads
your CI secrets: `env_var("CI_DEPLOY_TOKEN")` inside a `comptime fn`.
Predict the refusal reason this one cites — and note before running
that 18-6 filed environment reads under one reason, while the
diagnostic gives this capability a compound answer.

Solution — the environment gets both barrels, and the note names
secrets outright:

```console
$ wolf conform-run ./ex24-5.lu
error[E0701]: `env_var` reaches environment variables, which comptime code can never touch
 --> ./ex24-5.lu:5:5
  |
5 |     env_var("CI_DEPLOY_TOKEN")
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ ambient IO at compile time
...
8 |     const T = exfil()
  |               ------- while evaluating `exfil`, entered here
  |               ------- while evaluating `main`, entered here
  |
  = note: why it is refused — determinism and confinement: environment contents differ per machine
    and may hold secrets.
  = note: the comptime sandbox is hermetic (D33): the intrinsics available at compile time are an
    explicit allowlist, and nothing ambient is on it. Compute this value at runtime instead;
    file contents will arrive later as declared build inputs (s51), never as an evaluator
    capability.
```

The scenario this kills is the one event-stream normalized: code you
did not read, running at build time, in possession of what your CI
knows. Here that code does not get to run at all.

## §24.3 — Capabilities and `wolf audit`

**Exercise 24-6** *(comprehension · pending — blocker: capability
manifests and `wolf audit`; owner: s51-package-manager)* — A worked
audit diff in the pinned format (an example, not tool output — the
tool answers with its s31 scaffold line today):

```text
$ wolf audit --diff
den/rows      1.4.0 -> 1.5.0   capabilities: (none) -> (none)
forest/regex  2.1.0 -> 2.2.0   capabilities: (none) -> net
```

One line stops the merge. Say which, what question it forces, and why
"it needs to download updated Unicode tables" — the plausible answer
the upgrade's changelog will offer — is precisely the answer the
covenant rejects.

Solution: the `regex` line — a text-matching library that suddenly
wants `net` is either compromised or has grown ambitions no caller
asked for; the forced question is "what, concretely, will you do with
it." The Unicode-tables answer fails because it relocates data
acquisition from publish time (tables baked into the released
artifact, hashed, auditable) to run time on *your* machines with
*your* network — a determinism loss and an exfiltration channel in
one move. The audit gate's job is exactly to make that trade visible
before it is your incident. Until s51, this discipline runs on human
review of dependency diffs; the exercise is the reflex.

## §24.4 — What the covenant costs

**Exercise 24-7** *(design)* — Pick the strongest real case against
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

**Exercise 24-8** *(comprehension · prose)* — "It's only a
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
