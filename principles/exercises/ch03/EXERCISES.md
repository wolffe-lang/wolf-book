# Chapter 3 — Values and expressions: exercises

Exercises 3-1 through 3-5 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §3.3 — Arithmetic that traps

**Exercise 3-6** *(extension (break-it-on-purpose) · lupin)*. Using one
`i32` binding and one `*`, write the smallest program that traps with
`overflow` on `i32`. State, before running it, why the number you chose
is the smallest one that works, and what the trap line will say the
product was.

Solution. `ch03/ex3-6.lu`:

```wolf
fn main() -> !int {
    let n: i32 = 46341
    print("{n * n}")
    0
}
```

```console
$ lupin ex3-6.lu
ex3-6.lu: trap(overflow): `*` produced 2147488281, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 6:13
$ echo $?
3
```

`i32`'s ceiling is 2147483647, whose square root is 46340.95…, so
46341 is the smallest integer whose square leaves the type: 46340²
is 2147395600 and fits. The trap reports the true product: the
machine computed it, checked it against the type, and refused to
pretend it fit. The annotation is what makes the multiplication an
`i32` multiplication: a bare literal is unconstrained until something
gives it a type, and `let n: i32` is that something.

**Exercise 3-7** *(comprehension · lupin)*. Predict both lines. If you
arrived from Python, predict them twice:

```wolf
fn main() -> !int {
    let a = 0 - 7
    let b = 2
    print("{a / b} {a % b}")
    print("{(a / b) * b + a % b}")
    0
}
```

Solution: `-3 -1`, then `-7`. Wolf's integer division truncates toward
zero, so `-7 / 2` is −3, not Python's floored −4. The remainder
follows the division, so `-7 % 2` is −1, not 1. The second line is the
law that binds them: `(a / b) * b + a % b == a` holds for every legal
pair, whichever convention a language picks. Pick your division and the
remainder is chosen for you.

```console
$ lupin ex3-7.lu
-3 -1
-7
```

## §3.1 — `let`, `var`, and handing values over

**Exercise 3-8** *(comprehension · lupin)*. Predict the one printed
line, then answer the pointed part: after `name = "grace"`, what
happened to `"ada"`, and why does `first` not care?

```wolf
fn main() -> !int {
    var name = "ada"
    let first = name
    name = "grace"
    print("{first} {name}")
    0
}
```

Solution: `ada grace`. The `let first = name` line handed the value
over: `first` owns `"ada"` from that point, so the later rebinding of
`name` replaces what `name` holds without reaching anything `first`
has. The deep story of "hands the value over" (what it means for the
source afterward, and when it traps) is chapter 7's, on purpose; this
chapter needs only the direction of the handover.

A historical honesty note, retired: at this batch's authoring pins
neither tool rejected a reassignment to a `let` binding. Both do now —
E0410, naming the `var` fix and the shadowing alternative — so the
single-assignment rule the book taught on trust is enforced where it
was written:

```console
$ lupin ex3-8b.lu
ex3-8b.lu: E0410: `n` is bound with `let`, so it cannot be assigned again; declare the binding with `var` to update it in place (machine-applicable), or shadow it with a second `let` if the next value is really a new thing [gram.item.let] at 7:5
$ echo $?
2
```

## §3.2 — Everything is an expression

**Exercise 3-9** *(fingers · lupin)*. The pack drill: print 1 through
15, except that multiples of 3 print `howl`, multiples of 5 print
`scratch`, and multiples of both print `howlscratch`. One `if`-chain
as a value, one `print`. Why must the `both` test come first, and
what prints if it comes last?

Solution. `ch03/ex3-9.lu`:

```wolf
fn main() -> !int {
    for i in 1..16 {
        let word = if i % 15 == 0 {
            "howlscratch"
        } else if i % 3 == 0 {
            "howl"
        } else if i % 5 == 0 {
            "scratch"
        } else {
            "{i}"
        }
        print(word)
    }
    0
}
```

```console
$ lupin ex3-9.lu
1
2
howl
4
scratch
howl
7
8
howl
scratch
11
howl
13
14
howlscratch
```

An `if`-chain takes the first arm whose test passes. 15 is a multiple
of 3, so with the `both` test last, 15 prints `howl` and the
`howlscratch` arm is unreachable — no diagnostic says so, because
every arm is still type-correct. Order is logic here, not style.

## Chapter batch

**Exercise 3-10** *(extension · lupin)*. Print 1 through 16 with each
number's binary spelling right-aligned beside it. No format spec you
have met writes base 2, so build the bits yourself: `% 2` peels the
low bit, `/ 2` shifts, and prepending assembles them in the right
order. What does your loop produce for zero, and is that a spelling
or an absence?

Solution. `ch03/ex3-10.lu`:

```wolf
fn main() -> !int {
    for n in 1..17 {
        var bits = ""
        var rest = n
        while rest > 0 {
            bits = "{rest % 2}" + bits
            rest = rest / 2
        }
        print("{n:>3} {bits:>6}")
    }
    0
}
```

```console
$ lupin ex3-10.lu
  1      1
  2     10
  3     11
  4    100
  5    101
  6    110
  7    111
  8   1000
  9   1001
 10   1010
 11   1011
 12   1100
 13   1101
 14   1110
 15   1111
 16  10000
```

For zero the loop body never runs and `bits` stays empty: an absence.
The conventional spelling is `"0"`, and honesty about the boundary
costs one `if` before the print. The table above never reaches it;
your extension should, and should decide.

**Exercise 3-11** *(fingers · lupin)*. One pass over a block of
readings: lowest, highest, and mean, integer arithmetic throughout.
The first reading has to seed `lowest` and `highest` — why is
starting them at zero wrong, and which of the two inputs in the block
below would have exposed it?

Solution. `ch03/ex3-11.lu`:

```wolf
fn main() -> !int {
    let readings = """
        18
        4
        31
        22
        7
        """
    var lowest = 0
    var highest = 0
    var total = 0
    var count = 0
    for row in readings.lines() {
        let n = row.to_int() else 0
        count += 1
        total += n
        if count == 1 {
            lowest = n
            highest = n
        }
        if n < lowest { lowest = n }
        if n > highest { highest = n }
    }
    print("low {lowest} high {highest} mean {total / count}")
    0
}
```

```console
$ lupin ex3-11.lu
low 4 high 31 mean 16
```

Zero-seeded, `lowest` would stay 0 against this all-positive block —
every reading loses to it — so the all-positive data is exactly what
exposes the bug (an all-negative block would expose `highest`
instead). Seeding from the first element makes the answer a fact
about the data, not about the seed. The mean truncates: 82 / 5 is 16
here, and chapter 2's precision specs are how a report would say
16.4.

**Exercise 3-12** *(fingers · lupin)*. Print every three-digit
palindrome divisible by 7, using arithmetic only: `/` and `%` take
the digits, and a flipped number is three multiplies away. No
strings. How many are there, and why does your divisibility test not
need the middle digit?

Solution. `ch03/ex3-12.lu`:

```wolf
fn main() -> !int {
    for n in 100..1000 {
        let flipped = n % 10 * 100 + n / 10 % 10 * 10 + n / 100
        if n == flipped {
            if n % 7 == 0 {
                print("{n}")
            }
        }
    }
    0
}
```

```console
$ lupin ex3-12.lu
161
252
343
434
525
595
616
686
707
777
868
959
```

Twelve. The trick question dissolves on inspection: the test is
`n % 7 == 0` on the whole number, so no digit is special — the middle
digit needed no test *anywhere*, because a palindrome check by
arithmetic compares `n` to its flip rather than digit to digit. The
question is a nudge to notice your own program did less work than the
problem statement implied.

**Exercise 3-13** *(comprehension · lupin)*. `loop` with `break v` is
the expression form of "search until found". Predict the one line this
prints, then answer: what would the program do if `limit` were
negative, and which keyword is missing from this loop that `while`
has?

```wolf
fn main() -> !int {
    let limit = 1000
    var p = 1
    let first_past = loop {
        p = p * 2
        if p > limit { break p }
    }
    print("{first_past}")
    0
}
```

Solution: `1024` — the first power of two past 1000. With a negative
`limit` the very first doubling (to 2) is already past it, so the loop
answers 2 immediately; no hang. The missing keyword is the condition
itself: `loop` has no test at the top, so termination lives wholly in
the `break`, which is why the value can ride out on it.

```console
$ lupin ex3-13.lu
1024
```

**Exercise 3-14** *(fingers · lupin)*. 3-9's pack drill again, decided
once instead of in a chain. That version's order was load-bearing and
nothing checked it: put the `both` test last and an arm goes dead in
silence. Write the same 1-through-15 drill as a single `match` over
the *pair* `(n % 3, n % 5)`. Then move `(_, 0)` above `(0, 0)`, run it
again, and say what is different this time about being wrong.

Solution. `ch03/ex3-14.lu`:

```wolf
fn fizzbuzz(n: int) -> str {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz",
        (0, _) => "fizz",
        (_, 0) => "buzz",
        _ => "{n}",
    }
}
fn main() -> !int {
    for n in 1..16 {
        print(fizzbuzz(n))
    }
    0
}
```

```console
$ lupin ex3-14.lu
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
```

The pair is the whole trick. Each arm is a conjunction of column
tests — `(0, 0)` says "the first element is zero *and* so is the
second" — so the three interesting cases are three arms rather than a
nested `if` whose branches have to remember what the outer test
already decided. The `_` arm catches every other pair and is what
makes the `match` exhaustive; without it the checker hands back a
witness pair the arms do not cover.

Put `(_, 0)` above `(0, 0)` and the `(0, 0)` arm can never run: every
pair it would match, the arm above already took. That is redundancy,
not incompleteness, and it has its own name. `ch03/ex3-14b.lu` is that
program, and it still runs — an unreachable arm is a warning, not a
refusal:

```console
$ wolf build ./ex3-14b.lu
warning[E0802]: this arm can never match — the arms above already cover it
 --> ./ex3-14b.lu:8:9
  |
7 |         (_, 0) => "buzz",
  |         ------ this arm already matches those values
8 |         (0, 0) => "fizzbuzz",
  |         ^^^^^^ unreachable arm
  |
  = note: delete the arm, or reorder the arms so the more specific pattern comes first.
```

The reachability walk reads column by column, which is how it knows
that `(_, 0)` swallows `(0, 0)` without being handed the values.

That is the whole difference from 3-9. Both programs can be written
with a dead branch in them, and both still run; only one of them says
so. An `if`-chain's dead arm is type-correct and therefore invisible —
3-9's answer says as much — while a `match` arm's coverage is a
property the checker already computes, so the same mistake arrives
named, located, and with the repair in the note. The order of arms is
still logic rather than style, but it is logic the compiler reads
too.
