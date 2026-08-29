# Chapter 4 — Functions: exercises

Exercises 4-1 through 4-4 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §4.1 — Signatures are the contract

**Exercise 4-5** *(comprehension · lupin)*. Euclid's algorithm, in the
expression style:

```wolf
fn gcd(a: int, b: int) -> int {
    if b == 0 { a } else { gcd(b, a % b) }
}
```

Trace `gcd(1071, 462)` on paper — write down every `(a, b)` pair the
recursion visits — and state the result before running it.

Solution: the pairs are (1071, 462), (462, 147), (147, 21), (21, 0);
the answer is 21. Each step the second argument becomes the remainder,
which strictly shrinks, which is why the recursion is finite — the
signature promises an `int` and the arithmetic delivers one.

```console
$ lupin ex4-5.lu
21
```

## §4.3 — `defer`

**Exercise 4-6** *(comprehension · lupin)*. A `defer` is registered
when execution reaches it. Predict all five output lines, in order:

```wolf
fn work(n: int) -> int {
    defer print("one")
    if n == 0 { return 10 }
    defer print("two")
    20
}
fn main() -> !int {
    print("{work(0)}")
    print("{work(1)}")
    0
}
```

Solution: `one`, `10`, `two`, `one`, `20`. The early return in
`work(0)` fires only the defer that had already been reached — the
second `defer` line never executed, so it never registered. `work(1)`
reaches both and unwinds them in reverse. A defer is not a property of
the function; it is an event in its execution.

```console
$ lupin ex4-6.lu
one
10
two
one
20
```

## Chapter batch

**Exercise 4-7** *(extension · lupin)*. Build `day_of_year(month, day,
leap)` from two functions: `days_in(month, leap)` as one `match`, and a
loop that sums the months before yours. Verify: March 1st is day 60 in
a common year. Which date is day 60 in a leap year, and where in your
code did that difference come from?

Solution. `ch04/ex4-7.lu`:

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
fn day_of_year(month: int, day: int, leap: bool) -> int {
    var total = 0
    var m = 1
    while m < month {
        total += days_in(m, leap)
        m += 1
    }
    total + day
}
fn main() -> !int {
    print("{day_of_year(3, 1, false)} {day_of_year(3, 1, true)}")
    print("{day_of_year(12, 31, false)} {day_of_year(12, 31, true)}")
    0
}
```

```console
$ lupin ex4-7.lu
60 61
365 366
```

In a leap year, day 60 is February 29th, and March 1st moves to 61. The
difference lives in exactly one arm of `days_in` — the `2 =>` arm — and
nowhere else, which is the argument for writing the table as a `match`
instead of scattering the leap rule through the loop.
