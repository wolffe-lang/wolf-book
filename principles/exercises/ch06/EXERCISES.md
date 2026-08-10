# Chapter 6 — Errors are values: exercises

Exercises 6-1 through 6-5 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §6.1 — `!T` and the row

**Exercise 6-6** *(comprehension · lupin)* — The row below ends in
`..`, which makes it *open*: `probe` may return tags the signature
never lists. Predict all three numbers, and say which arm of the
`match` handles `probe(-1)` and why the program compiles at all when
`Weird` appears nowhere in any signature:

```wolf
fn probe(n: int) -> int ! {Io(int), ..} {
    if n < 0 { return Weird }
    if n == 0 { return Io(4) }
    n
}
fn code_for(n: int) -> int {
    probe(n) else |err| {
        match err {
            Io(code) => 0 - code,
            _ => 0 - 99,
        }
    }
}
```

Solution: `7 -4 -99`. The rest arm `_` handles `Weird` — an open row
tells every caller "there may be more," so the compiler requires the
rest arm, and that arm is what lets upstream add tags without breaking
this consumer. The price is symmetric: with `..` in the signature, no
caller can ever match exhaustively by name.

```console
$ lupin ex6-6.lu
7 -4 -99
```

## §6.2 — `?`, `else`, `else |err|`

**Exercise 6-7** *(extension · lupin)* — `head` prints a file's first
`n` lines — and a file with fewer than `n` lines is not a crash, it is
an answer. Write `head(text, n)` whose error carries how many lines
actually existed, and a caller that asks for 2 lines (succeeds) and 5
lines (handled). Why does the payload belong in the error instead of
being printed by `head` itself?

Solution — `ch06/ex6-7.lu`:

```wolf
fn head(text: str, n: int) -> str ! {TooFew(int)} {
    var out = ""
    var got = 0
    for line in text.lines() {
        if got < n {
            out += "{line}\n"
            got += 1
        }
    }
    if got < n { return TooFew(got) }
    out
}
fn main() -> !int {
    let log = """
        alpha
        beta
        gamma
        """
    let two = head(log, 2) else |_| { return 1 }
    print_raw(two)
    let five = head(log, 5) else |err| {
        match err {
            TooFew(have) => {
                print("wanted 5, file has {have}")
                ""
            },
        }
    }
    0
}
```

```console
$ lupin ex6-7.lu
alpha
beta
wanted 5, file has 3
```

`head` does not know whether a short file is a diagnostic-worthy event,
a loop terminator, or nothing at all — only its caller does. Carrying
the count in the payload moves the *fact* to the code that owns the
*policy*. A function that prints its own errors has decided the policy
for every caller it will ever have.

## §6.4 — Hardening by refactor

**Exercise 6-9** *(extension · lupin)* — Three postures toward the
input `"7x"`: trap on it, default it to zero, or refuse it out loud.
The script hardens one construct at a time:

```wolf
fn parse_or_zero(s: str) -> int {
    s.to_int() else 0
}
fn parse(s: str) -> int ! {NotAnInt} {
    s.to_int() else { return NotAnInt }
}
```

Predict all three printed lines of the solution's `main`, which calls
`parse_or_zero("7x")` and then `parse("7x")` with a handler. Then the
pointed part: rank the three postures for a program that reads
numbers from a config file, and defend last place.

Solution — `ch06/ex6-9.lu`:

```console
$ lupin ex6-9.lu
0
refused: not a number
-1
```

Last place is `else 0`. The trap at least stops the program at the
lie; the row at least tells the caller the truth; `else 0` silently
converts "your config is broken" into "your limit is zero" and ships
it. A default is a *decision*, and `else 0` makes it in the one place
that cannot know whether zero is safe. The row version moves that
decision to the caller, which is the whole hardening arc of this
section in one function.

## §6.5 — Capstone: wordcount

**Exercise 6-10** *(extension · lupin)* — The wordcount loop, grown by
one requirement: count words, and separately count words of four bytes
or more. Predict both numbers for the line
`the wolf runs and the moon watches over`, then run.

Solution — `ch06/ex6-10.lu`:

```wolf
fn main() -> !int {
    let text = """
        the wolf runs and the moon watches over
        """
    var words = 0
    var long_words = 0
    for w in text.words() {
        words += 1
        if w.len >= 4 { long_words += 1 }
    }
    print("{words} words, {long_words} of length 4+")
    0
}
```

```console
$ lupin ex6-10.lu
8 words, 5 of length 4+
```

Five of the eight — `wolf`, `runs`, `moon`, `watches`, `over` — reach
four bytes. The filter is one `if` inside the loop the capstone
already has, which is the shape of most real requirements: the
skeleton absorbs them without growing a second loop. In part 3 this
same loop parallelizes by changing one call; the boxed promise stands.

## Chapter batch

**Exercise 6-8** *(design)* — Chapter 6's `parse` exposes the row
`{Empty, NotDigit(Bad), TooLong}`. Suppose `parse` moves into a
library, behind a public API used by fifty programs. Argue both sides:
should the public signature keep the three-tag row, or coarsen to a
single `Invalid` tag with the detail inside? Name one concrete caller
each design serves better, and what each design costs when a fourth
failure mode appears.

Solution (discussion): the wide row serves the caller that *dispatches* —
an editor that jumps the cursor to `Bad.at` needs `NotDigit`'s payload,
and coarsening would force it to parse the error out of a string. The
coarse row serves the caller that *reports* — a CLI that prints one line
per bad input gains nothing from three arms it handles identically, and
every tag in a public row is a name the library must keep forever. The
fourth failure mode is where the designs separate: the wide row grows,
and every exhaustive caller gets a compile error — disruptive, and
honest. The coarse row absorbs it silently — smooth, and silent is the
cost, because the caller who cared cannot find out. Wolf's `..` open
row is the middle position: name the tags you commit to, and say out
loud that there may be more. Where wolf's own std faced this, it chose
narrow closed rows at leaves and coarsening at module boundaries — the
row is part of the API's promise, and promises are cheapest when made
small and kept.
