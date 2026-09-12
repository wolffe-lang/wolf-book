# Appendix E — The driver's surface

Every chapter in this book reaches for `wolf` and takes what it needs:
`wolf run` in chapter 1, `wolf build` a section later, `wolf test`,
`wolf add`, `wolf audit`, `wolf publish`, each introduced where it earns
its place and never listed beside its neighbors. This page is the list.
It is here because a book that says *the formatter* in chapter 2 and
names `wolf fmt` in chapter 3 owes the reader somewhere to look it up.

The driver describes itself, and what it prints is the whole surface —
one binary, twenty-two subcommands, three groups:

```console
$ wolf --help
wolf — the wolf toolchain: one binary that compiles, runs, tests, formats,
documents and packages wolf programs. Wolf is a compiled systems language
whose memory lives in regions the compiler infers, so a program carries no
lifetime annotations, and whose arithmetic is checked in every profile.

Usage: wolf <command> [options] [file.lu]

Your first program — put it in a directory of its own:

    mkdir hello && cd hello
    cat > hello.lu <<'EOF'
    fn main() {
        print("hello, wolf\n")
    }
    EOF
    wolf run hello.lu

A DIRECTORY is a module (D32), not a file: every `.lu` file beside
`hello.lu` joins the same module, so a second scratch file with its own
`main` is an error (E0302), not a second program. Give each program its
own directory.

Writing and running code:
  build          compile an entry file to a native executable
  run            build an entry file and run it, passing on its exit code
  test           discover and run `*_test.lu` files
  fmt            reformat source in the one canonical style
  fix            apply the compiler's machine-applicable suggestions
  doc            generate the package's documentation

Packages:
  init           promote a script to a package
  add            add a dependency to `wolf.pkg`
  rm             remove a dependency from `wolf.pkg`
  update         re-fetch pinned dependencies and refresh the ledger
  audit          the capability tree, and what changed since `wolf.sum`
  tree           the resolved dependency tree
  why            the chain that pulls one dependency into the build
  vendor         mirror the resolved dependencies into `vendor/wolf/`
  publish        verify a package and emit its signed log record
  cache          where the script cache lives, and collecting it

Inspecting and reporting:
  interface      print every module's public interface
  audit-surface  the package's complete unsafety inventory
  profile        read and merge profile-guided-optimization data
  c-import       import C headers, and show what was refused
  conform-run    observe one program and emit a conformance record
  lsp            serve the Language Server Protocol over stdio

On its own:
  --explain E####        the full explanation of one diagnostic code
  --version              this build's identity and its paired interpreter
  --help, -h             this message; `wolf <command> --help` for one command
  --man                  the man page, for an installer to place
  --completions <shell>  a completion script for bash, zsh or fish

Exit codes:
  0  success
  1  the program did not compile, or a test or a check failed
  2  a usage or environment error

Documentation and the specification:
  https://github.com/wolffe-lang/wolf-lang
The standard library ships separately and no package of the compiler
carries it; point wolf at a checkout with `--std-root <dir>` or the
`WOLF_STD` environment variable:
  https://github.com/wolffe-lang/wolf-std
```

That block is a run, not a transcription, so this page cannot fall
behind the binary: a subcommand the driver grows appears here the day it
appears there.

Three of the twenty-two are load-bearing for a reader of this book and
are taught nowhere else in it, and the rest of this appendix is them.
`wolf fmt` decides what your source looks like whether you invite it to
or not. `wolf init` is the door between a script and a package, which is
the crossing chapters 22 and 23 assume you have already made. `wolf
vendor` is the answer to a question chapter 24 raises and does not
close: what happens to a build when the network is not there.

The remaining four you will not meet in these pages — `profile`,
`c-import`, `cache` and `lsp` — are reference material with no chapter
behind them, and the driver's own `wolf <command> --help` is the whole
of what this book would tell you about them.

## `wolf fmt` — one style, and no argument about it

The formatter's shape is fixed by the specification rather than by your
project, and the command has no options for changing it:

```console,in(pkg/fmt)
$ wolf fmt --help
wolf fmt — reformat source in the one canonical style.

usage: wolf fmt [--check] <file.lu|dir|->...

Files and directories (recursing into `*.lu`) are rewritten in place;
`-` formats stdin to stdout; `--check` rewrites nothing and exits
nonzero listing what is not canonical. There are no other options,
deliberately (D34): the style is fixed by the specification, not
configured per project.
$ wolf fmt --check aligned/widths.lu
wolf fmt --check: aligned/widths.lu is not canonically formatted
$ echo $?
1
$ wolf fmt --check canonical/widths.lu
$ echo $?
0
$ wolf fmt aligned/widths.lu && lupin aligned/widths.lu
narrow: a column narrow enough to read at a glance
wide: a column that will wrap on somebody
$ wolf fmt --check aligned/widths.lu
$ echo $?
0
```

`--check` is the whole of the integration story: it rewrites nothing and
exits `1` on the first file that is not canonical, which is what a
continuous-integration job wants and what a pre-commit hook wants, and
there is no third mode to choose between.

Here is what it rewrote. The file `aligned/widths.lu` above is written
with each `else` leading its line and the braces in a column:

```wolf,file(pkg/fmt/aligned/widths.lu)
fn label(n: int) -> str {
    if n < 10      { "narrow" }
    else if n < 80 { "usual" }
    else           { "wide" }
}

fn describe(n: int) -> str {
    if n < 10      { "a column narrow enough to read at a glance" }
    else if n < 80 { "a column the terminal was measured for" }
    else           { "a column that will wrap on somebody" }
}

fn main() -> !int {
    print("{label(4)}: {describe(4)}")
    print("{label(400)}: {describe(400)}")
    0
}
```

That layout parses — §3.2 says why, and the two functions are the same
chain twice with the arms at different lengths. `canonical/widths.lu` is
what came back, and the run above ends with `aligned/widths.lu` byte for
byte the same as it:

```wolf,file(pkg/fmt/canonical/widths.lu)
fn label(n: int) -> str {
    if n < 10 { "narrow" } else if n < 80 { "usual" } else { "wide" }
}

fn describe(n: int) -> str {
    if n < 10 {
        "a column narrow enough to read at a glance"
    } else if n < 80 {
        "a column the terminal was measured for"
    } else {
        "a column that will wrap on somebody"
    }
}

fn main() -> !int {
    print("{label(4)}: {describe(4)}")
    print("{label(400)}: {describe(400)}")
    0
}
```

Three clauses decided that, and reading them in order explains why one
function collapsed and the other did not. `[gram.fmt.brace]` puts the
opening brace on the construct's line and `} else` on one line, so no
`else` in canonical wolf ever starts a line. `[gram.fmt.inline]` lets a
block stay on one line when its body is guard-clause-shaped — at most
two statements — and fits the width. `[gram.fmt.indent]` sets that width
at 100 columns and forbids breaking mid-token. `label` fits, so all three
of its arms are on one line; `describe` does not, and **a braced chain
breaks as one**: the arms that would still have fit break with the arm
that did not, because a chain laid out half inline and half broken reads
as two constructs rather than one. (That last rule is the newer half. At
wolf 0.2.11 the formatter broke only the arm that ran out of room, and
the file above is what it printed then; 0.2.12 re-lays it, along with
thirty-three files of wolf-std and twenty-eight of lobo. A formatter that
is a fixed point is not a formatter that never changes — it is one that
changes in a release, in public, with the corpus re-laid in the same
commit.)

The style is *canonical*, which is a stronger claim than fixed:
`[gram.fmt.canon]` requires the formatter to be a fixed point on every
file of the compiler's own corpus, byte for byte. Run it twice and the
second run is a no-op; that is a rule with tests behind it, not a
property somebody hopes holds. The clause declares exactly one exception,
and it is declared per file: a source file whose header carries
`//! fmt: relaid` pins a layout the parser admits and the formatter
re-lays anyway. Those files exist so that the leading-`else` form of
§3.2 has something holding it to its meaning; the formatter is not the
identity on them, and they say so in their first line rather than in a
configuration file.

What the formatter will not do is guess. `[gram.fmt.region]` rewrites a
region between its sugar and value forms only when the syntax alone
proves which one is right, and where it cannot tell, it leaves your text
where you put it.

## `wolf init` — a script becomes a package

Chapter 1 runs `.lu` files with no project around them, and chapter 22
starts from a directory that is already a package. `wolf init` is the
step between, and it is the step a diagnostic sends you to: a script's
frontmatter is a subset of a manifest, and asking it to carry a key only
a real package can mean is `E1507`, whose fix-it names this verb.

```console,in(pkg/promote)
$ wolf init --help
wolf init — promote a script to a package.

usage: wolf init --from-script <file.lu> [--dir DIR]

The script's `//!` frontmatter — dependencies, capabilities, edition —
moves into a real `wolf.pkg` verbatim, and its code moves into
`main.lu`. This is the verb the E1507 diagnostic's fix-it names.
$ wolf init --from-script tally.lu --dir tally
wolf init: wrote tally/wolf.pkg and tally/main.lu from tally.lu (the script is untouched)
$ wolf audit --dir tally
capability tree (I13)
local/tally 0.1.0 (root) caps=[fs]
effective: [fs]
wolf audit: no wolf.sum yet — nothing to diff against (run a verb that writes it)
```

The script it was given is one file with its manifest in the leading doc
comment:

```wolf,file(pkg/promote/tally.lu)
//! Count the lines in a file, the smallest thing worth a package.
//! pkg {
//!     edition: "1",
//!     capabilities: [fs],
//! }

fn main() -> !int {
    fs_write_text("notes.txt", "one\ntwo\nthree\n")?
    print("{fs_read_text("notes.txt")?.lines().count()} lines")
    0
}
```

Three things in that transcript are the promise, and the third is the
one worth trusting the command for. The capability the script declared
is the capability the package declares — `wolf audit` reads the new
manifest and answers `caps=[fs]`, so the promotion did not quietly widen
or drop what the program may do. The doc comment that was not manifest
stayed with the code, in `main.lu`, where it still documents the module.
And the script is untouched: the command says so in its own output, and
a promotion that deleted its input is a promotion nobody would try
twice.

## `wolf vendor` — the dependencies, mirrored

Chapter 23 puts a lock file beside the manifest and chapter 24 argues
that a build must not run code it was not asked to run. Neither of them
answers the question that follows: what do you build against when the
registry is unreachable, or gone. `wolf vendor` is that answer.

```console,in(pkg/added)
$ wolf vendor --help
wolf vendor — mirror the resolved dependencies into `vendor/wolf/`.

usage: wolf vendor [--dir DIR]

The vendor tree IS a store, so every guarantee carries over: hashes
re-derive on use, and a tampered mirror fails exactly like a tampered
store. The next build prefers it automatically.
$ wolf vendor
wolf vendor: no wolf.pkg in . (run `wolf add` to create one, or --dir)
$ echo $?
2
$ wolf vendor --dir app
wolf vendor: nothing store-backed to vendor (path dependencies travel with the tree)
```

The second answer is the interesting one, and it is a fact about this
project rather than a refusal. `app` depends on `rows` by path, and a
path dependency is already beside you: mirroring it would copy a
directory you can already see into a directory you cannot. So `wolf
vendor` reports that there is nothing store-backed to mirror and exits
`0`. The command is for the dependencies that came from a store, and it
tells you when you have none rather than making an empty tree and
calling it a success.

What it does when there are some is the sentence its own help gives, and
it is worth reading twice: *the vendor tree is a store*. It is not a
snapshot the build trusts because it is local. Hashes re-derive on use,
so a mirror somebody edited fails the same way an edited store fails,
with the same diagnostic and at the same moment — which is what makes
vendoring safe to do and safe to commit. The next build prefers the
mirror without being asked, so the offline case needs no flag and no
second manifest.
