# EXERCISES.md — the exercise doctrine

Exercises are how the reader finds out whether the chapter happened to
them. This document fixes their numbering, density, taxonomy, and
solutions policy, then proves the doctrine on real content: §5 is the
complete exemplar batch for the bs01–bs02 material — every exercise
written, every solution program on disk under `principles/exercises/`,
every claimed output pasted from a real run.

Two tools check exercises today:

- **lupin** (wolf-interp, `target/release/lupin`) — runs programs,
  reports traps, evaluates REPL snippets. Anything with a dynamic
  outcome is lupin's.
- **wolf** (wolf-lang, `wolf conform-run`) — rejects programs statically
  with the shipping diagnostics. Anything whose answer is an error code
  is wolf's.

Every exercise names its checker, and four more names are in use because
four kinds of exercise have no single tool behind them:

- **prose** — the answer is discussion, and nothing runs.
- **lupin REPL** — the answer is a transcript of a session.
- **corpus** — the reader runs a program the corpus already holds.
- **the C twin** — the answer is read out of the C program printed
  beside the wolf one, in the projects part.

A kind may be hybrid (`comprehension + extension`), and a vein may
qualify a kind in parentheses (`comprehension (schedule play)`); §8 lists
the veins. Where a stem joins a kind and a vein with `+`, read it as the
parenthetical form.

---

## 1. Numbering

K&R style, per chapter: **Exercise 3-2** is the second exercise of
chapter 3. Numbers are stable once published — a retired exercise leaves
a tombstone, not a renumbering. Sections end with the exercises they
earned; chapters end with a batch that mixes the chapter's sections.

## 2. Density

Every section that teaches something ends with at least one exercise. A
chapter ends with three to six more. The book-wide target was
order-of-150+ as a floor; the corpus filled it at full scale: **275
exercises** — 49 in part 1, 43 in part 2, 71 in part 3, 70 in part 4, 28
in part 5, and 14 on the appendices. Chapter 31 brings none by design
(§4) and chapter 32 none at all; chapter 29's batch lands with its
chapter. The per-chapter sets live in
`principles/exercises/chNN/EXERCISES.md` (this file's §5 remains the
chapters 1–6 exemplar batch, folded in unchanged; the chapter files
continue its numbering); `EXERCISES-INDEX.md` maps section → exercise →
tier, and `EXERCISES-PENDING.md` is the honest list of the eleven whose
blocking features have not landed, each named with its blocker and
owner. 189 exercises have a solution program that CI executes; 72 are
discussion solutions by design, 9 live in a REPL transcript, and the
rest re-run a sibling's program or read a tool's output.

## 3. Taxonomy

Five kinds, each with a distinct verb. The batch in §5 tags every
exercise.

- **fingers** — type it, run it, change one line. The goal is motor
  memory and the reader's first proof that the toolchain obeys them.
- **comprehension** — predict the output or the verdict *before*
  running. lupin makes predictions checkable; wolf makes verdicts
  checkable. The solution states the prediction and shows the run.
- **extension** — grow the sample: add a case, a field, a variant, a
  flag. The solution is a program diff plus its run.
- **spelunking** — read a primary source (a diagnostic, a trap line, a
  spec clause, `wolf --explain`) and explain it back. The solution is
  prose that cites the source by its identifier.
- **design** — no single answer. The solution page discusses the
  trade-offs and, where wolf took a side, why.

## 4. Solutions policy

Every printed exercise has a solution in the back matter, collapsed by
default, linked to the section that set it (`book/back/solutions.md`,
generated from these files by `cargo xtask backmatter`). An exercise
whose section is held is not printed, and an unprinted exercise
publishes no solution: the answer to a question the reader was never
asked is repo bookkeeping. Solution programs are ordinary
samples: extracted by the bs00 pipeline, executed in CI, snapshot-checked
where they show diagnostics. A solution that stops compiling fails the
book's build — solutions rot first, so they are wired to the same alarm
as everything else.

The one exception, narrowed (bs10): **the solo — chapter 31, `logden`**
— publishes milestone checkpoints, not solutions. A project with an
answer key is a tutorial; the checkpoints give the reader diffable
safety nets instead.

The exception used to cover "the capstone", when the book closed with
one. It now covers exactly one chapter, and the narrowing matters in
both directions. The five *guided* projects (chapters 26–30) are
walkthroughs, so they end with an extension batch numbered and solved
like every other chapter's — a walkthrough that withholds answers is
just a slower lecture. Chapters 26, 27 and 28 have theirs, in
`principles/exercises/ch26/`, `ch27/` and `ch28/`. The solo withholds
them on purpose, because the chapter's whole claim is that the reader can
now stand up: everything it needs was taught, and it says only that. No
community-solutions link, no "if stuck, see the appendix" — the hints in
the milestone ladder are the entire safety net, one per milestone.

The coda (chapter 32) carries no exercise batch: it is a page of reading
about the allocator the reader never needed, not a build.

Solution register per TONE.md: deadpan for fingers and comprehension,
discursive for design. A solution may say "we chose the boring way" and
show it; it may not say "obviously."

---

## 5. The exemplar batch — chapters 1–6

Programs live in `principles/exercises/chNN/`. Commands are as run from
each chapter's directory; outputs are pasted from the runs, unedited.
lupin is the interpreter `wolf-toolchain.toml` pins; wolf is the wolf-lang
debug build at `impl_version 0.0.1`.

### Chapter 1 — Hello, Wolf

**Exercise 1-1** *(fingers · lupin)*. Type the hello program exactly as
printed and run it. Then make it print a second line of your choosing.

Solution. `ch01/ex1-1.lu`:

```wolf
fn main() -> !int {
    print("hello, wolf")
    print("the moon is up")
    0
}
```

```console
$ lupin ex1-1.lu
hello, wolf
the moon is up
```

**Exercise 1-2** *(comprehension · lupin)*. Before running, write down
what this program prints and what `echo $?` shows afterward:

```wolf
fn main() -> !int {
    print("working")
    3
}
```

Solution: it prints `working` and exits `3` — `main`'s last expression
is the process exit code, and `print` output is unrelated to it.

```console
$ lupin ex1-2.lu
working
$ echo $?
3
```

**Exercise 1-3** *(fingers · lupin)*. Braces interpolate expressions,
not only names. Print a sentence that computes `6 * 7` twice inside one
string literal.

Solution. `ch01/ex1-3.lu`:

```wolf
fn main() -> !int {
    let name = "reader"
    print("hello, {name} — {6 * 7} is still {6 * 7}")
    0
}
```

```console
$ lupin ex1-3.lu
hello, reader — 42 is still 42
```

### Chapter 2 — Strings, honestly

**Exercise 2-1** *(comprehension · lupin REPL)*. Predict all three
before evaluating: `"wolf".len`, `"é".len`, `"🐺".len`.

Solution: 4, 2, 4 — `len` counts UTF-8 bytes. One ASCII letter is one
byte, é is two, the wolf emoji is four.

```console
$ lupin eval '"wolf".len'
4 : i64
$ lupin eval '"é".len'
2 : i64
$ lupin eval '"🐺".len'
4 : i64
```

**Exercise 2-2** *(fingers · lupin)*. Using format specs, print a
two-column table: names left-aligned in 10 columns, numbers
right-aligned in 4.

Solution. `ch02/ex2-2.lu`:

```wolf
fn main() -> !int {
    var names = List[str]()
    names.push("wolf")
    names.push("marmot")
    names.push("elk")
    var i = 0
    for n in names {
        i += 1
        print("{n:<10}{i:>4}")
    }
    0
}
```

```console
$ lupin ex2-2.lu
wolf         1
marmot       2
elk          3
```

**Exercise 2-3** *(comprehension · lupin)*. `"wolf"` has four bytes.
Predict the exact behavior of:

```wolf
let s = "wolf"
let t = s[2..9]
```

Solution: the program traps. Slicing is checked; an out-of-range byte
slice is a defined fault, not undefined behavior, and the trap says so
in its clause tag:

```console
$ lupin ex2-3.lu
ex2-3.lu: trap(bounds): byte range 2..9 is outside a 4-byte string [mem.ub.defined] at 6:13
$ echo $?
3
```

**Exercise 2-4** *(extension · lupin)*. Extend the word counter to also
report lines and bytes for a `"""` multiline block. Predict which of the
three numbers is a byte count before running.

Solution. `ch02/ex2-4.lu`:

```wolf
fn main() -> !int {
    let text = """
        the wolf runs
        the moon watches
        """
    var words = 0
    for _ in text.words() { words += 1 }
    let lines = text.lines().count()
    print("{words} words, {lines} lines, {text.len} bytes")
    0
}
```

```console
$ lupin ex2-4.lu
6 words, 2 lines, 31 bytes
```

31 is bytes: 13 for the first line, 16 for the second, and two
newlines — the dedent removed the leading spaces before counting.

### Chapter 3 — Values and expressions

**Exercise 3-1** *(comprehension · lupin)*. Predict the one line this
prints. Both `match` and `if` are expressions here; nothing is a
statement:

```wolf
fn main() -> !int {
    let n = 3
    let kind = match n {
        0 => "none",
        1 => "one",
        _ => "many",
    }
    let parity = if n % 2 == 0 { "even" } else { "odd" }
    print("{kind} and {parity}")
    0
}
```

Solution: `many and odd`.

```console
$ lupin ex3-1.lu
many and odd
```

**Exercise 3-2** *(comprehension · wolf + lupin)*. The pack loses its
lead:

```wolf
struct Pack { lead: str, tail: str }
fn adopt(take w: str) -> str { w }
fn main() -> !int {
    var p = Pack { lead: "ada", tail: "grace" }
    let a = adopt(take p.lead)
    let b = p.tail
    let c = p.lead
    print("{a} {b} {c}")
    0
}
```

Before running anything, write down two predictions: what `wolf` says
about this program, and what `lupin` does with it. Which line does each
tool blame, and why is `let b` not the one?

Solution: the compiler rejects it statically; the interpreter runs it
and traps at the same use. `let b` survives because `p.tail` is a
disjoint field — moving `p.lead` empties only that path.

```console
$ wolf conform-run ./ex3-2.lu
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

```console
$ lupin ./ex3-2.lu
./ex3-2.lu: trap(use-after-move): `p.lead` was moved out and is uninitialized here [mem.tier0.move.2] at 11:13; `p.lead` moved here at 9:19
$ echo $?
3
```

One rule, two enforcement points: the compiler proves it before the
program starts; lupin enforces it dynamically and cites the same clause
family. This differential is the book's spine and the reader meets it
here first.

**Exercise 3-3** *(comprehension · lupin)*. `2147483647` is `i32`'s
ceiling. Predict what `big + 1` does in a release build. (Trick
warning: the answer is the same in every build.)

Solution: it traps, in every profile — X3 is the decision the trap line
cites, and the line also names the spelling for intentional wraparound:

```console
$ lupin ex3-3.lu
ex3-3.lu: trap(overflow): `+` produced 2147483648, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 6:16
```

**Exercise 3-4** *(comprehension · lupin)*. The divisor is computed,
not literal. Does that change anything?

```wolf
let n = 10
let d = n - 10
print("{n / d}")
```

Solution: no. Division by zero is a defined trap regardless of how the
zero arrived:

```console
$ lupin ex3-4.lu
ex3-4.lu: trap(div-zero): division by zero is defined behavior in wolf: it traps [mem.ub.defined] at 7:13
```

**Exercise 3-5** *(design)*. Wolf has no ternary operator. Write the
expression you would have used one for, in wolf, and then argue either
side: is `if`-as-expression enough?

Solution (discussion): `let parity = if n % 2 == 0 { "even" } else
{ "odd" }` is the whole answer to the common case, and it nests without
precedence archaeology, which `?:` never did. The honest cost: it is
longer, and chained conditions (`a ? x : b ? y : z`) become `match` or
stacked `if`/`else` — more lines, each readable. Wolf's position is that
a second conditional syntax buys keystrokes and costs a grammar
production and a style war; the book spends neither.

### Chapter 4 — Functions

**Exercise 4-1** *(fingers · lupin)*. Functions are values. Write
`compose` so that `compose(double, double)` returns a function, and
apply it to 10.

Solution. `ch04/ex4-1.lu`:

```wolf
fn main() -> !int {
    let double = fn(n) n * 2
    let compose = fn(f, g) fn(x) f(g(x))
    let quad = compose(double, double)
    print("{quad(10)}")
    0
}
```

```console
$ lupin ex4-1.lu
40
```

**Exercise 4-2** *(comprehension · lupin)*. Predict the order of the
three lines:

```wolf
fn main() -> !int {
    defer print("first registered")
    defer print("second registered")
    print("body")
    0
}
```

Solution: `body`, then the defers in reverse registration order —
`defer` is a stack, because teardown must unwind what setup wound:

```console
$ lupin ex4-2.lu
body
second registered
first registered
```

**Exercise 4-3** *(extension · lupin)*. Give the list a `shrink`
function to pair with `grow`. Then, without running anything, state how
you would find every mutation in this program with one search.

Solution. `ch04/ex4-3.lu`:

```wolf
fn grow(mut xs: List[int]) { xs.push(7) }
fn shrink(mut xs: List[int]) { let _ = xs.pop() }
fn main() -> !int {
    var xs = List[int]()
    grow(mut xs)
    grow(mut xs)
    shrink(mut xs)
    print("len={xs.len}")
    0
}
```

```console
$ lupin ex4-3.lu
len=1
```

The search is `grep 'mut '` (or, stricter, `(mut `): call-site `mut` is
required, so the callers are the complete mutation audit — that is X1's
entire argument, performed on your own file.

**Exercise 4-4** *(comprehension + spelunking · wolf)*. One of these
calls is legal and one is not:

```wolf
bump(mut p.a.n, mut p.b.n)
wide(mut p.a, mut p.a.n)
```

Say which and why, then check yourself against the compiler and against
`wolf --explain E1002`.

Solution: `bump` passes two *disjoint* paths — legal. `wide` passes a
path and its own prefix; `p.a.n` lives inside `p.a`, so two exclusive
claims overlap:

```console
$ wolf conform-run ./ex4-4.lu
error[E1002]: `p.a.n` cannot go `mut` here: it overlaps `p.a`, already passed `mut` in this call
  --> ./ex4-4.lu:11:23
   |
11 |     wide(mut p.a, mut p.a.n)
   |              --- `p.a` is passed `mut` here
   |                       ^^^^^ second exclusive claim on the same place
   |
   = note: `p.a.n` is inside `p.a` — a path and its prefix conflict [mem.model.path.disjoint].
     Disjoint fields (`x.a` with `x.b`) are fine together.
```

The `--explain` entry states the general rule the diagnostic instances:
"Two paths conflict iff one is a prefix of the other"
(`[mem.model.path.disjoint]`). Under lupin the same program runs to the
call and traps `exclusivity`, citing the same clause — predict that,
too, and check it.

### Chapter 5 — Collections and generics

**Exercise 5-1** *(fingers · lupin)*. A `List` is also a stack. Push
three values, pop one, and print the popped value and the remaining
length.

Solution. `ch05/ex5-1.lu`:

```wolf
fn main() -> !int {
    var xs = List[int]()
    xs.push(1)
    xs.push(2)
    xs.push(3)
    let top = xs.pop()
    print("top={top} len={xs.len}")
    0
}
```

```console
$ lupin ex5-1.lu
top=3 len=2
```

**Exercise 5-2** *(fingers · lupin)*. Score the pack: write two scores
into a `Map`, raise one by reading it back, and print the table with
format specs.

Solution. `ch05/ex5-2.lu`:

```wolf
fn main() -> !int {
    var scores = Map[str, int]()
    scores["wolf"] = 3
    scores["marmot"] = 5
    scores["wolf"] = scores["wolf"] + 1
    for (name, n) in scores.pairs() {
        print("{name:<8}{n:>3}")
    }
    0
}
```

```console
$ lupin ex5-2.lu
wolf      4
marmot    5
```

**Exercise 5-3** *(extension · lupin)*. Write `first[T]` with a
fallback for the empty case, and call it twice: once with the type
named, once letting inference name it.

Solution. `ch05/ex5-3.lu`:

```wolf
fn first[T](xs: List[T], fallback: T) -> T {
    if xs.is_empty() { fallback } else { xs[0] }
}
fn main() -> !int {
    var howls = List[str]()
    howls.push("awoo")
    let empty = List[int]()
    print("{first[str](howls, "silence")} {first(empty, -1)}")
    0
}
```

```console
$ lupin ex5-3.lu
awoo -1
```

Both calls hit the same definition; `[str]` at the call site is
documentation, not a requirement, because the arguments already pin `T`.

**Exercise 5-4** *(comprehension · lupin)*. The list has one element.
Predict `xs[10]`, precisely: what kind of event, and what exit code.

Solution: a `bounds` trap, exit 3 — same contract as string slicing,
because indexing is checked everywhere:

```console
$ lupin ex5-4.lu
ex5-4.lu: trap(bounds): index 10 is outside a collection of 1 element(s) [mem.ub.defined] at 7:13
$ echo $?
3
```

**Exercise 5-5** *(design)*. Wolf writes generics `top[T]` and indexing
`m["k"]` with the same brackets. Rust chose `::<T>` partly to avoid that
ambiguity. What does wolf's choice cost, and where is the cost paid?

Solution (discussion): the cost is real and it is paid in the
grammar/sema seam, not by the reader: `e[…]` parses as one postfix form
and *semantic analysis* decides index versus generic-apply (the corpus
pins this in `grammar/brackets_index.lu`). The compiler carries the
complexity so that user code never grows a `::<>`. The trade to defend:
error messages at that seam must stay excellent, because when sema
guesses wrong, the diagnostic — not the syntax — is what saves the
reader. That is a bet on tooling quality, made on purpose, and this book
holds the compiler to it.

### Chapter 6 — Errors are values

**Exercise 6-1** *(fingers · lupin)*. Write `parse` so empty input is
an error, and give two call sites: one defaulting with `else 0`, one
with `else 7`. Predict both prints first.

Solution. `ch06/ex6-1.lu`:

```wolf
fn parse(s: str) -> int ! {Empty} {
    if s.is_empty() { return Empty }
    s.to_int() else 0
}
fn main() -> !int {
    let a = parse("42") else 0
    let b = parse("") else 7
    print("a={a} b={b}")
    0
}
```

```console
$ lupin ex6-1.lu
a=42 b=7
```

**Exercise 6-2** *(comprehension · lupin)*. `chain` calls `parse`
through `?`. Predict `a` and `b`, and name which row variant `b`'s
handler sees:

```wolf
fn chain(s: str) -> int ! {Empty, NotDigit(Bad)} {
    let v = parse(s)?
    v + 1
}
fn main() -> !int {
    let a = chain("42") else |_| -1
    let b = chain("") else |err| {
        match err {
            Empty => -2,
            NotDigit(e) => -3,
        }
    }
    print("a={a} b={b}")
    0
}
```

Solution: `a=43` (parse succeeds, `?` unwraps, one is added); `b=-2` —
`parse("")` returns `Empty`, `?` hands it up unchanged, and the handler
matches it. The variant crossed one call boundary without wrapping;
that is the row composing by union.

```console
$ lupin ex6-2.lu
a=43 b=-2
```

**Exercise 6-3** *(extension · lupin)*. Grow the row: add a `TooLong`
variant for inputs over four bytes and handle it. What else did you
have to change, and what told you?

Solution. `ch06/ex6-3.lu` (excerpt):

```wolf
fn parse(s: str) -> int ! {Empty, NotDigit(Bad), TooLong} {
    if s.is_empty() { return Empty }
    if s.len > 4 { return TooLong }
    ...
}
    let v = parse("40000") else |err| {
        match err {
            TooLong => -4,
            NotDigit(e) => 0 - e.at - 3,
            Empty => -2,
        }
    }
```

```console
$ lupin ex6-3.lu
v=-4
```

The signature grew (rows are spelled, not sprung on callers) and the
match grew an arm — exhaustiveness is what tells you, at compile time,
that a handler fell behind its row.

**Exercise 6-4** *(comprehension · lupin)*. The error carries a
payload. Predict both printed lines:

```wolf
fn digit(s: str, i: int) -> int ! {NotDigit(Bad)} {
    let c = s[i..i + 1]
    if c < "0" || c > "9" { return NotDigit(Bad { at: i, found: c }) }
    c.to_int() else 0
}
fn main() -> !int {
    let v = digit("4x", 1) else |err| {
        match err {
            NotDigit(e) => {
                print("bad digit `{e.found}` at byte {e.at}")
                -1
            },
        }
    }
    print("v={v}")
    0
}
```

Solution: byte 1 of `"4x"` is `x`, so the handler prints the payload's
fields, then `v=-1` — the handler's value becomes the expression's:

```console
$ lupin ex6-4.lu
bad digit `x` at byte 1
v=-1
```

**Exercise 6-5** *(comprehension · lupin)*. `errdefer` runs only on
the error path. `work(true)` succeeds; `work(false)` fails after the
`errdefer` is registered. Predict all four output lines:

```wolf
fn work(ok: bool) -> int ! {Fail} {
    var r = get(true)?
    errdefer print("cleanup ran")
    let v = get(ok)?
    r.n + v.n
}
```

Solution: the success path prints nothing extra; the failure path fires
the cleanup between the failing `?` and the caller's `else`:

```console
$ lupin ex6-5.lu
a=2
cleanup ran
b=-1
```

---

## 6. Audit ledger — findings from writing this batch

The doctrine says writing exercises is an audit. Writing these 26 filed
the following (real findings, from the runs above; they become
`book-audit` issues when the bs01 issue machinery exists):

1. **lupin dispatches bare variant patterns as binders.** A `match` arm
   whose pattern is a bare variant name (`Empty =>`) matches *any*
   value, first-arm-wins: `brightness(Color.Green)` takes the `Red` arm.
   The corpus's `typecheck/match_exhaustive.lu` does not catch this —
   `Rgb(1, 2, 3)` averages to 2, which equals the `Red` arm's answer, so
   the file passes by arithmetic coincidence. Exercise 6-3's solution
   orders its arms so the checked output stays honest, and this ledger
   entry is the flag. (Severity: ba:blocker against wolf-interp's
   evaluator; plus a corpus fix so the coincidence cannot recur.)
2. **`wolf conform-run FILE` fails without a path prefix.** From the
   file's own directory, `wolf conform-run ex3-2.lu` reports "the
   package root has no wolf source files"; `./ex3-2.lu` works. Bare
   relative paths lose their parent directory somewhere in package-root
   resolution. (ba:papercut, wolf-lang driver.)
3. **Map index reads of absent keys evaluate to `()` under lupin**
   (`m["missing"]` → `()`), and compound assignment through a map index
   (`tally[w] += 1`) is `unsupported` while `wordcount.lu`'s comment
   promises absent-key-defaults-to-zero. The tally idiom the book wants
   to teach in chapter 5 does not currently have a runnable spelling in
   the interpreter's std subset. (ba:blocker against the std surface
   pinning, from the chapter-5 author's point of view.)
4. **lupin's std subset gaps that shaped this batch:** no `str.split`,
   `str.find`, `List.contains/sorted/reverse`, `Map.keys/values/
   contains_key`, no `^n` end-relative indexing, no `wrapping[i32]`
   constructor at runtime. Each absence either reshaped an exercise or
   moved it to a different checker. (ba:doc-only for now; becomes a
   pinning worklist when the std surface spec lands.)

No finding was silently worked around: where a workaround shaped an
exercise, the exercise says so in place.

## 7. Stats for this batch

26 exercises: 7 fingers, 12 comprehension, 4 extension, 2 design, 1
spelunking-hybrid (4-4). Checkers: 23 exercised under lupin (including
the REPL exercise), 3 under wolf conform-run (3-2 and 4-4 under both).
All 23 solution programs on disk ran with the outputs shown; the REPL
exercise (2-1) lives in its transcript, and the two design exercises have
discussion solutions and no program.

---

## 8. Appendix — the veins

Where exercises come from. The taxonomy (§3) says what an exercise asks
of the reader; the vein says what the exercise is *made of*. Authors
draw from all of them; the imagination mandate holds — no two adjacent
chapters lean on the same vein, and repetition of a domain across
chapters is a defect.

- **Predict-the-outcome** — given a program, name the verdict before
  running: the exit code, the trap kind, or the E-code, and why. lupin
  makes it checkable; wolf's specialty when the answer is static.
- **Differential pairs** — unique to this book: predict both lupin's
  dynamic outcome and wolf's static verdict for one program, and
  explain a case where they legitimately differ (conservatism). Part 2
  is their home turf.
- **Break-it-on-purpose** — construct the program that earns a named
  diagnostic using only the constructs given; the smallest program
  that traps a given kind.
- **Region thinking** — refactor to zero annotations, predict
  wholesale-free points, the disjoint-open antichain puzzles.
- **Schedule play** — seeded interleaving predictions, defer order
  under kill vs cancel, construct-a-deadlock and let the trap explain,
  `--explore` findings.
- **Build-a-thing minis** — small programs spread deliberately across
  domains, at most one domain per chapter: temperature table (ch1),
  run-length coder (ch2), calendar arithmetic (ch4), RPN calculator
  (ch5), head-style row handling (ch6), LCS diff (ch7), LRU cache and
  room graph (ch8), pipeline (ch10), worker pool (ch11), message
  router (ch12), grep-lite and n-body step (ch13), mailbox service
  (ch14), supervisor-in-miniature (ch15), maze in a region (ch16),
  L-system table (ch18), saxpy kernel (ch21).
- **The coreutils vein** — used sparingly and made wolfish, budget
  four for the whole book, all four spent: uniq (5-6, order plus
  tally), head (6-7, error rows), wc (8-16, regions for line storage),
  grep-lite (13-2, line views and rows). No further coreutils mimics.
- **Spelunking** — read a primary source (a diagnostic, a trap line, a
  `--explain` entry, a directive header) and explain it back, citing
  the source by identifier.
- **Design** — no single answer; solutions discuss and, where wolf
  took a side, say why.
