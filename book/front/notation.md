# Notation

<!-- STUB (bs00): the full dialect walkthrough lands with bs01. The five
     block kinds below are live so the sample pipeline and the theme have
     real cargo from day one; each carries its real directive and is
     extracted and executed by `cargo xtask samples`. -->

The book sets code in five dialects. Programs are complete and executed
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

A *REPL transcript* shows a session with the interpreter; replay waits on
wolf-interp's REPL (is08) and is counted, not skipped silently:

```wolf-repl
wolf> "é".len
2 : i64
```

A *console run* shows a command and its output, prompt kept:

```console
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

Exercise numbering is K&R style: Exercise 3-2 is the second exercise of
chapter 3. Numbers are stable; a retired exercise leaves a tombstone.
