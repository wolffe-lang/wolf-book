# Chapter 1 — Hello, Wolf: exercises

Exercises 1-1 through 1-3 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; their numbering is stable and this file
continues from it. Commands run from this directory; outputs are pasted
from real runs. lupin is `lupin 0.1.0 (wolf-interp)`; wolf is the
wolf-lang debug build at `impl_version 0.0.1`.

## §1.4 — The REPL: a spec you can interrogate

**Exercise 1-4** *(fingers · lupin REPL)* — Open the REPL. Compute the
number of seconds in a day, ask `:type` what type that expression has,
then define a function mid-session and call it twice. Before you ask
`:type`, write down your guess.

Solution — one session:

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
`i64`. Definitions persist for the whole session — the REPL is a
workbench, not a calculator.

## §1.3 — Scripts before projects

**Exercise 1-5** *(fingers + extension · lupin)* — The first program in
*The C Programming Language* prints a Fahrenheit-to-Celsius table. Write
wolf's: 0 to 120 degrees in steps of 20, one line per row. Then look
hard at the 20-degree row. Is it right?

Solution — `ch01/ex1-5.lu`:

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

## §1.5 — What `run` was doing for you

**Exercise 1-6** *(spelunking · lupin)* — Delete the closing brace of a
working program's `main` and run it. Read the whole diagnostic: the
code, the message, the clause tag, the span. Then run `echo $?`. Which
of the manual's exit codes is this, and why is it not the code a trap
would produce?

Solution — `ch01/ex1-6.lu` (broken on purpose):

```console
$ lupin ex1-6.lu
ex1-6.lu: E0202: the file ends where `}` was required [gram.expr.block] at 162..162
$ echo $?
2
```

Exit 2 is a static-phase rejection: the program never ran. A trap exits
3 and can only happen to a program that was legal and started — the two
codes divide "wolf refused" from "wolf obeyed, and the program hit a
rule." The clause tag `[gram.expr.block]` names the grammar rule the
file broke, and the span points at the end of the file, which is where
the absence lives.

## §1.2 — Installing the one tool

**Exercise 1-7** *(fingers · lupin)* — Ask both tools who they are:
`lupin --version` and `wolf --version`. Write down which parts of each
line will appear in this book's colophon, and why a book would print
them at all.

Solution:

```console
$ lupin --version
lupin 0.1.0 (wolf-interp, pin cbde620)
$ wolf --version
wolf 0.0.1 (pre-alpha)
```

The version and the pin are the colophon's material: every sample in
this book is CI-verified against a specific toolchain, and these lines
are how you check that your tools are the ones the book's claims are
true for. Output that differs from a book's printed output is a
version question before it is a bug.
