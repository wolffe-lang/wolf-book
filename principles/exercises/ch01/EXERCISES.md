# Chapter 1 — Hello, Wolf: exercises

Exercises 1-1 through 1-3 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; their numbering is stable and this file
continues from it. Commands run from this directory; outputs are pasted
from real runs against the tools `wolf-toolchain.toml` pins: `lupin
0.1.4 (wolf-interp, pin ad6cef7)` and `wolf 0.0.1 (wolfgang)`.

## §1.4 — The REPL: a spec you can interrogate

**Exercise 1-4** *(fingers · lupin REPL)*. Open the REPL. Compute the
number of seconds in a day, ask `:type` what type that expression has,
then define a function mid-session and call it twice. Before you ask
`:type`, write down your guess.

Solution. One session:

```console
$ lupin
wolf> 60 * 60 * 24
86400 : i32
wolf> :type 60 * 60 * 24
i32
wolf> fn area(r: int) -> int { r * r * 3 }
defined fn `area`
wolf> area(10)
300 : i64
wolf> let a = area(2)
wolf> a + 1
13 : i64
wolf> :quit
```

Literal arithmetic infers `i32`; a function annotated `int` returns
`i64`. Definitions persist for the whole session. The REPL is a
workbench, not a calculator.

## §1.3 — Scripts before projects

**Exercise 1-5** *(fingers + extension · lupin)*. The first table in
*The C Programming Language* converts Fahrenheit to Celsius. Write
wolf's: 0 to 120 degrees in steps of 20, one line per row. Then look
hard at the 20-degree row. Is it right?

Solution. `ch01/ex1-5.lu`:

```wolf
fn main() -> !int {
    var f = 0
    while f <= 120 {
        let c = (f - 32) * 5 / 9
        print("{f}	{c}")
        f += 20
    }
    0
}
```

```console
$ lupin ex1-5.lu
0	-17
20	-6
40	4
60	15
80	26
100	37
120	48
```

The 20-degree row says −6; the exact answer is −6.67, and 20°F is
colder than −6°C suggests. Integer division truncates toward zero, so
every Celsius entry here is rounded toward warm. K&R's version had the
same bug and fixed it with floats; wolf gets there in chapter 2's
format specs.

**Exercise 1-9** *(fingers · lupin)*. Drive 1-5's table the other way:
Celsius −20 to 40 in steps of 10, Fahrenheit beside it, both columns
right-aligned. This direction has no rounding bug. Say why not before
you run it.

Solution. `ch01/ex1-9.lu`:

```wolf
fn main() -> !int {
    var c = -20
    while c <= 40 {
        let f = c * 9 / 5 + 32
        print("{c:>4}{f:>6}")
        c += 10
    }
    0
}
```

```console
$ lupin ex1-9.lu
 -20    -4
 -10    14
   0    32
  10    50
  20    68
  30    86
  40   104
```

The multiply happens first, and every Celsius entry here is a multiple
of 5, so `c * 9` is a multiple of 45 and the division by 5 is exact.
1-5 divided a number that was not a multiple of 9 by 9 and lost the
remainder. Same operators, opposite order, and one direction tells the
truth by accident of the inputs — which is why chapter 2's format
specs, not luck, are the durable fix.

## §1.5 — What `run` was doing for you

**Exercise 1-6** *(spelunking · lupin)*. Delete the closing brace of a
working program's `main` and run it. Read the whole diagnostic: the
code, the message, the clause tag, the span. Then run `echo $?`. Which
of the manual's exit codes is this, and why is it not the code a trap
would produce?

Solution. `ch01/ex1-6.lu` (broken on purpose):

```console
$ lupin ex1-6.lu
ex1-6.lu: E0202: the file ends where `}` was required [gram.expr.block] at 6:6
$ echo $?
2
```

Exit 2 is a static-phase rejection: the program never ran. A trap exits
3 and can only happen to a program that was legal and started: the two
codes divide "wolf refused" from "wolf obeyed, and the program hit a
rule." The clause tag `[gram.expr.block]` names the grammar rule the
file broke, and the span points at the end of the file, which is where
the absence lives.

**Exercise 1-8** *(comprehension · wolf + lupin)*. Build the greeting,
then run `wolf build hello.lu --verbose` a second time without editing
the file. A successful build is silent by default; `--verbose` is how
you watch it work. Predict what the second build does before you run
it, and where it put what it kept.

Solution:

```console
$ wolf build hello.lu --verbose
wolf build: root: reused object (key a476d75665e8c37a)
```

The second build compiles nothing. `.lu-cache/` beside the source holds
the object file keyed by everything that could change its contents (the
module's source, the compiler's own build id, the profile, and the
interface surfaces of what it depends on), so an unchanged key is an
answer already on disk. The key in your terminal will differ from the
one above; the word `reused` will not.

**Exercise 1-10** *(comprehension · lupin)*. Before running, write down
what this program prints and what `echo $?` shows afterward:

```wolf
fn main() -> !int {
    let amounts = """
        3
        four
        5
        """
    var total = 0
    for row in amounts.lines() {
        total += row.to_int() else 0
    }
    print("counted")
    total
}
```

Solution: it prints `counted` and exits `8`. The middle row is not a
number, `else 0` makes it worth nothing, and the sum of the other two
rides out of `main` as the exit code — the same last-expression rule
1-2 established, with the value computed instead of literal.

```console
$ lupin ex1-10.lu
counted
$ echo $?
8
```

## §1.2 — Two implementations, one language

**Exercise 1-7** *(fingers · wolf + lupin)*. Compile the greeting with
`wolf build`, run the binary, then run the source under `lupin`. Compare
the two outputs byte for byte: `diff <(./hello) <(lupin hello.lu)` will
do it. Then say which of the two runs could have printed something
different, and what it would mean about the language if it had.

Solution:

```console
$ wolf build hello.lu
$ diff <(./hello) <(lupin hello.lu)
$ echo $?
0
```

`diff` printing nothing is the whole result. Either run could have
disagreed: the compiler lowered the f-string to a sequence of writes
against the runtime's print shims, and the interpreter evaluated it
against its own string model: two separate pieces of code, written
from the specification rather than from each other. A byte of
disagreement between them is a bug in one implementation or a hole in
the specification, and it is found here rather than in your program.

## §1.1 — A program worth keeping

**Exercise 1-11** *(fingers · lupin)*. Find the longest line of a
multiline block and print its length and the line itself, in that
order. One pass, one comparison. What does your program print when two
lines tie, and which line of your code decided that?

Solution. `ch01/ex1-11.lu`:

```wolf
fn main() -> !int {
    let log = """
        the wolf runs
        the moon watches the ridge
        dawn
        """
    var best = ""
    for line in log.lines() {
        if line.len > best.len { best = line }
    }
    print("{best.len} {best}")
    0
}
```

```console
$ lupin ex1-11.lu
26 the moon watches the ridge
```

On a tie the first of the tied lines wins, and the `>` decided it: a
later line replaces `best` only by being strictly longer. Writing `>=`
hands the tie to the last line instead. One character of the program is
the whole policy, which is the reason to know where it is.
