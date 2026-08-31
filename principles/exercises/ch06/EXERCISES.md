# Chapter 6 — Errors are values: exercises

Exercises 6-1 through 6-5 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §6.1 — `!T` and the row

**Exercise 6-6** *(comprehension · lupin)*. The row below ends in
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

Solution: `7 -4 -99`. The rest arm `_` handles `Weird`: an open row
tells every caller "there may be more," so the compiler requires the
rest arm, and that arm is what lets upstream add tags without breaking
this consumer. The price is symmetric: with `..` in the signature, no
caller can ever match exhaustively by name.

```console
$ lupin ex6-6.lu
7 -4 -99
```

## §6.2 — `?`, `else`, `else |err|`

**Exercise 6-7** *(extension · lupin)*. `head` prints a file's first
`n` lines, and a file with fewer than `n` lines is not a crash, it is an
answer. Write `head(text, n)` whose error carries how many lines actually
existed, and a caller that asks for 2 lines (succeeds) and 5 lines
(handled). Why does the payload belong in the error instead of being
printed by `head` itself?

Solution. `ch06/ex6-7.lu`:

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
a loop terminator, or nothing at all: only its caller does. Carrying
the count in the payload moves the *fact* to the code that owns the
*policy*. A function that prints its own errors has decided the policy
for every caller it will ever have.

## §6.4 — Hardening by refactor

**Exercise 6-9** *(extension · lupin)*. Three postures toward the
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

Solution. `ch06/ex6-9.lu`:

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

**Exercise 6-10** *(extension · lupin)*. The wordcount loop, grown by
one requirement: count words, and separately count words of four bytes
or more. Predict both numbers for the line `the wolf runs and the moon
watches over`, then run.

Solution. `ch06/ex6-10.lu`:

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

Five of the eight (`wolf`, `runs`, `moon`, `watches`, `over`) reach
four bytes. The filter is one `if` inside the loop the capstone
already has, which is the shape of most real requirements: the
skeleton absorbs them without growing a second loop. In part 3 this
same loop parallelizes by changing one call; the boxed promise stands.

## Chapter batch

**Exercise 6-8** *(design)*. Chapter 6's `parse` exposes the row
`{Empty, NotDigit(Bad), TooLong}`. Suppose `parse` moves into a
library, behind a public API used by fifty programs. Argue both sides:
should the public signature keep the three-tag row, or coarsen to a
single `Invalid` tag with the detail inside? Name one concrete caller
each design serves better, and what each design costs when a fourth
failure mode appears.

Solution (discussion): the wide row serves the caller that
*dispatches*: an editor that jumps the cursor to `Bad.at` needs
`NotDigit`'s payload, and coarsening would force it to parse the error
out of a string. The coarse row serves the caller that *reports*: a CLI
that prints one line per bad input gains nothing from three arms it
handles identically, and
every tag in a public row is a name the library must keep forever. The
fourth failure mode is where the designs separate: the wide row grows,
and every exhaustive caller gets a compile error, disruptive and
honest. The coarse row absorbs it silently, and the silence is the
cost, because the caller who cared cannot find out. Wolf's `..` open
row is the middle position: name the tags you commit to, and say out
loud that there may be more. Where wolf's own std faced this, it chose
narrow closed rows at leaves and coarsening at module boundaries: the
row is part of the API's promise, and promises are cheapest when made
small and kept.

**Exercise 6-11** *(extension · lupin)*. `to_int` has no inverse on
this shelf, so build one: `itoa(n)`, peeling digits with `% 10` and
`/ 10` and prepending. Handle zero and negatives, then close the loop:
feed each result back through `to_int` and make `main` exit nonzero on
any mismatch. Which of your three test values forced a line of code
the other two never touch?

Solution. `ch06/ex6-11.lu`:

```wolf
fn itoa(n: int) -> str {
    if n == 0 { return "0" }
    var v = n
    var sign = ""
    if n < 0 {
        sign = "-"
        v = 0 - n
    }
    var digits = ""
    while v > 0 {
        digits = "{v % 10}" + digits
        v = v / 10
    }
    "{sign}{digits}"
}
fn main() -> !int {
    var cases = List[int]()
    (mut cases).push(340)
    (mut cases).push(0)
    (mut cases).push(0 - 275)
    for n in cases {
        let s = itoa(n)
        let back = s.to_int() else 999999
        print("{n} -> {s}")
        if back != n { return 1 }
    }
    0
}
```

```console
$ lupin ex6-11.lu
340 -> 340
0 -> 0
-275 -> -275
```

Zero forced its own line: the peel loop runs while `v > 0`, so zero
peels no digits and would come out as the empty string — the early
`return "0"` is that case's whole cost, the same absence 3-10's binary
table met. The negative forced two more (the sign, and `0 - n`).
Round-tripping through `to_int` is what promotes "looks right" to
"is right": the else-arm's 999999 can never equal a real input, so a
parse failure fails the run instead of passing by coincidence.

**Exercise 6-12** *(extension · lupin)*. Exercise 5-11's decoder
answers garbage with garbage: `"4w"` decodes to nothing, silently.
Harden it by refactor: `decode(s) -> str ! {Empty, BadRun}`, where
empty input, a count with no letter before it, and a zero-length run
are refusals with names. Table four malformed inputs through
`else |err|` and a `match`. Which refusal required *adding* a check,
and which two fell out of checks the loop already had?

Solution. `ch06/ex6-12.lu`:

```wolf
fn decode(s: str) -> str ! {Empty, BadRun} {
    if s.is_empty() { return Empty }
    var out = ""
    var cur = ' '
    var seen = false
    var n = 0
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            if seen == false { return BadRun }
            n = n * 10 + (c as int) - ('0' as int)
        } else {
            if seen {
                if n == 0 { return BadRun }
                for _ in 0..n { out += "{cur}" }
            }
            cur = c
            seen = true
            n = 0
        }
    }
    if n == 0 { return BadRun }
    for _ in 0..n { out += "{cur}" }
    out
}
fn main() -> !int {
    var cases = List[str]()
    (mut cases).push("w4o4l1f1")
    (mut cases).push("")
    (mut cases).push("4w")
    (mut cases).push("w0")
    (mut cases).push("ab2")
    for coded in cases {
        let plain = decode(coded) else |err| {
            let why = match err {
                Empty => "empty",
                BadRun => "bad run",
            }
            print("[{coded}] refused: {why}")
            continue
        }
        print("[{coded}] -> {plain}")
    }
    0
}
```

```console
$ lupin ex6-12.lu
[w4o4l1f1] -> wwwwoooolf
[] refused: empty
[4w] refused: bad run
[w0] refused: bad run
[ab2] refused: bad run
```

`"4w"` needed the new check (`if seen == false`): the soft decoder
simply ignored digits with no letter to bind to. The other two were
already half-present: the loop tracked `n` and `seen` anyway, so
`w0` and `ab2` (a letter whose run count never arrived) are one
`n == 0` comparison promoted from "silently emit nothing" to a named
refusal. Hardening rarely means new machinery; it means the checks
the loop was implicitly making become answers the caller can hold.

**Exercise 6-13** *(extension · lupin)*. A date validator:
`parse_date("2026-02-29")` should refuse, and say *why*. Split on
`-`, parse the three fields, and return
`(int, int, int) ! {BadShape, BadMonth, BadDay}` — reusing 4-7's
`days_in` for the day ceiling, leap rule included. Why is `BadShape`
checked first, and what happens to your month test if it is not?

Solution. `ch06/ex6-13.lu`:

```wolf
fn days_in(month: int, leap: bool) -> int {
    match month {
        2 => if leap { 29 } else { 28 },
        4 => 30,
        6 => 30,
        9 => 30,
        11 => 30,
        _ => 31,
    }
}
fn parse_date(s: str) -> (int, int, int) ! {BadShape, BadMonth, BadDay} {
    var y = 0
    var m = 0
    var d = 0
    var i = 0
    for field in s.split("-") {
        let n = field.to_int() else { return BadShape }
        if i == 0 { y = n }
        if i == 1 { m = n }
        if i == 2 { d = n }
        i += 1
    }
    if i != 3 { return BadShape }
    if m < 1 || m > 12 { return BadMonth }
    let leap = if y % 400 == 0 { true } else if y % 100 == 0 { false } else { y % 4 == 0 }
    if d < 1 || d > days_in(m, leap) { return BadDay }
    (y, m, d)
}
fn main() -> !int {
    var cases = List[str]()
    (mut cases).push("2026-08-31")
    (mut cases).push("2024-02-29")
    (mut cases).push("2026-02-29")
    (mut cases).push("2026-13-01")
    (mut cases).push("soon")
    for s in cases {
        let (y, m, d) = parse_date(s) else |err| {
            let why = match err {
                BadShape => "not a date shape",
                BadMonth => "no such month",
                BadDay => "no such day",
            }
            print("{s:<12} refused: {why}")
            continue
        }
        print("{s:<12} ok: day {d} of month {m}, {y}")
    }
    0
}
```

```console
$ lupin ex6-13.lu
2026-08-31   ok: day 31 of month 8, 2026
2024-02-29   ok: day 29 of month 2, 2024
2026-02-29   refused: no such day
2026-13-01   refused: no such month
soon         refused: not a date shape
```

Shape first because the later tests read `m` and `d`, and those
variables only mean anything once three numeric fields actually
arrived. Skip the shape check and `"soon"` reaches the month test
with `m` still 0 — refused as `BadMonth`, which is a *lie about the
input*: the caller fixing "no such month" would stare at a string
with no month in it. Refusal order is part of a validator's honesty,
not a style choice. (2024-02-29 passing while 2026-02-29 refuses is
`days_in` earning its `leap` parameter back from 4-7.)
