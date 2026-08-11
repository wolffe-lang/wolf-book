# Notation

The book sets code in six dialects. Programs are complete and executed
by CI; the blocks below are the pipeline's smoke test as much as the
reader's legend.

A *program* block is a complete `.lu` file, run as shown:

```wolf,run(exit=0, stdout="hello, wolf")
fn main() -> !int {
    print("hello, wolf")
    0
}
```

A *part* block is a named slice of a larger program; the extractor
stitches the parts and runs the whole:

```wolf,part(greet)
fn greet(name: str) -> str {
    "hello, {name}"
}
```

```wolf,part(greet, cont),run(exit=0, stdout="hello, reader")
fn main() -> !int {
    print(greet("reader"))
    0
}
```

A *REPL transcript* shows a session with the interpreter:

```wolf-repl
wolf> "é".len
2 : i64
```

A *console run* shows a command and its output, prompt kept:

```console,from(book/front-notation/s1)
$ lupin hello.lu
hello, wolf
```

A *diagnostic* block is the compiler's exact text and layout, never
retyped; this one is cross-checked against the captured run of corpus
sample `ch03/ex3-2`:

```diagnostic,from(ch03/ex3-2)
error[E1001]: `p.lead` is used here after its value moved away
  --> ./ex3-2.lu:11:13
   |
 9 |     let a = adopt(take p.lead)
   |                        ------ `p.lead` moved here
10 |     let b = p.tail
11 |     let c = p.lead
   |             ^^^^^^ used after the move
   |
   = note: re-initializing the place (assigning to it) also makes it usable again.
help: to keep the original, copy it at the move
   |
 9 |     let a = adopt(take copy p.lead)
   |
```

A *contrast* block is another language's code, shown where the honest
comparison needs the program rather than a description of it. It is
vendored under `samples/contrast/`, compiled and run by CI with warnings
denied, and checked character-for-character against the block on the
page:

```rust
pub fn tokens(input: &str) -> Vec<Token<'_>> {
    input
        .split_whitespace()
        .map(|text| Token { text })
        .collect()
}
```

Figures — ownership trees, tables of boxes and arrows — are set in the
same monospaced ground with no dialect label. They are drawings, not
code, and nothing runs them.

## Reading the dialects

The distinction that matters is who is speaking. Program and part blocks
are the book's; console runs, transcripts, and diagnostics are the
tools'. Tool output is pasted from real runs and never edited — no
elisions, no retyped error messages, no tidied spans. When a diagnostic
looks longer than the program that earned it, that is the compiler's
editorial decision and the book keeps it.

Two programs appear at the prompt, for the reason chapter 1 gives: one
language, two implementations of it, one of which compiles. A
`$ wolf …` line is the compiler — `wolf build` produces a binary,
`wolf run` produces one and executes it, and either may refuse the
program first. A `$ lupin …` line is the reference interpreter running
the same source without compiling it. Both cite specification clauses
in square brackets (`[mem.ub.defined]`) and stable error codes
(`E0202`), and both identifiers are indexed: a code or a clause you
meet in your own terminal is findable in this book.

Most of the book's runs are the interpreter's, because the book is
usually asking what a program *means* rather than what it costs, and
that is the question the reference implementation answers. Where the
compiler has something of its own to say — a diagnostic, a binary, a
measurement — it says it under its own prompt.

Exit codes recur often enough to be worth memorizing early — these are
the ones behind the book's `$ echo $?` lines. `0` and whatever else
your `main` returns are the program's own; `1` means `main` handed back
an error; `2` is a static rejection, so the program never started; `3`
is a trap, so it started and hit a rule.

## Exercises and solutions

Exercise numbering is K&R style: Exercise 3-2 is the second exercise of
chapter 3. Numbers are stable; a retired exercise leaves a tombstone
rather than renumbering its neighbors. Stems end the section whose
material they exercise, and each carries its kind and its checker —
*(comprehension · lupin)* means predict the outcome, then let the
interpreter settle it; *(spelunking · wolf)* means read what the compiler
says and explain it back.

Every exercise has a solution, and every solution program is a sample
like any other: extracted, executed, and snapshot-checked in the same CI
run as the chapters. A solution that stops compiling fails the book's
build.
