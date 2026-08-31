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

Trace `gcd(1071, 462)` on paper (write down every `(a, b)` pair the
recursion visits) and state the result before running it.

Solution: the pairs are (1071, 462), (462, 147), (147, 21), (21, 0);
the answer is 21. Each step the second argument becomes the remainder,
which strictly shrinks, which is why the recursion is finite. The
signature promises an `int` and the arithmetic delivers one.

```console
$ lupin ex4-5.lu
21
```

**Exercise 4-10** *(fingers · lupin)*. Write `rtrim`: trailing blanks
and tabs gone, everything else kept. Walk backward from the end with
byte slices and return one slice of the original. Print each result in
brackets so the trimming is visible. What does your function return
for a line that is all blanks, and did that case cost you code?

Solution. `ch04/ex4-10.lu`:

```wolf
fn rtrim(s: str) -> str {
    var end = s.len
    while end > 0 && (s[end - 1..end] == " " || s[end - 1..end] == "\t") {
        end -= 1
    }
    s[..end]
}
fn main() -> !int {
    for line in "howl   \nden\t\t\nmoon".lines() {
        print("[{rtrim(line)}]")
    }
    0
}
```

```console
$ lupin ex4-10.lu
[howl]
[den]
[moon]
```

An all-blank line walks `end` down to 0 and returns `s[..0]`, the
empty string — no extra code, because `while end > 0` is already the
guard the case needs. The return value is a slice of the input, two
words aimed at bytes that already exist: `rtrim` allocates nothing,
which is the honest cost §2.5 promised a slice would have.

## §4.2 — Functions as values

**Exercise 4-9** *(extension · lupin)*. Write `clamp_to(lo, hi)`, a
function that returns the clamping function for that range, and use
`clamp_to(0, 100)` to sanitize a list of parsed percentages. The
closure captures `lo` and `hi` by value. What would break, and what
would not, if it captured them by place?

Solution. `ch04/ex4-9.lu`:

```wolf
fn clamp_to(lo: int, hi: int) -> fn(int) -> int {
    fn(n) if n < lo { lo } else if n > hi { hi } else { n }
}
fn main() -> !int {
    let percent = clamp_to(0, 100)
    let rows = "40\n-12\n130"
    for row in rows.lines() {
        let n = row.to_int() else 0
        print("{n:>5} -> {percent(n)}")
    }
    0
}
```

```console
$ lupin ex4-9.lu
   40 -> 40
  -12 -> 0
  130 -> 100
```

Nothing here would break by-place, because `lo` and `hi` never change
after the closure is made — the difference is unobservable in this
program. What by-value buys is that the claim holds *by construction*:
`percent` means 0..100 forever, even if a later edit reassigns some
`var` the bounds came from. Capture by value makes the closure a
sealed fact rather than a live reference to the caller's mood.

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
`work(0)` fires only the defer that had already been reached: the
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
difference lives in exactly one arm of `days_in` (the `2 =>` arm) and
nowhere else, which is the argument for writing the table as a `match`
instead of scattering the leap rule through the loop.

**Exercise 4-8** *(fingers · lupin)*. The Collatz walk: halve an even
number, triple-and-add-one an odd one, count the steps to reach 1.
Write `steps(n)` and print a two-column table for 1 through 10. Which
starting point under 11 takes the longest, and is the answer where you
expected it in the table?

Solution. `ch04/ex4-8.lu`:

```wolf
fn steps(n: int) -> int {
    var v = n
    var count = 0
    while v != 1 {
        v = if v % 2 == 0 { v / 2 } else { 3 * v + 1 }
        count += 1
    }
    count
}
fn main() -> !int {
    for n in 1..11 {
        print("{n:>3}{steps(n):>5}")
    }
    0
}
```

```console
$ lupin ex4-8.lu
  1    0
  2    1
  3    7
  4    2
  5    5
  6    8
  7   16
  8    3
  9   19
 10    6
```

9 takes the longest, at 19 steps, and it is not where intuition puts
it: 7 (at 16) beats everything larger in the table, and 8 — bigger
than 7 — finishes in 3. The walk's length has no visible relation to
the starting size, which is precisely why the function earns a table
instead of a guess. (Whether *every* start reaches 1 is a famous open
question; your loop assumes it, and for 1 through 10 the assumption is
checked by termination.)

**Exercise 4-11** *(extension · lupin)*. Zeller's congruence names the
weekday of any date in four lines of arithmetic — January and February
count as months 13 and 14 of the year before. Write `weekday(y, m, d)`
returning the day's name from one `match`, and check it against three
dates whose weekday is a matter of record. Why does the month shift
exist: what property of the calendar is it buying back?

Solution. `ch04/ex4-11.lu`:

```wolf
fn weekday(y: int, m: int, d: int) -> str {
    var mm = m
    var yy = y
    if m < 3 {
        mm = m + 12
        yy = y - 1
    }
    let k = yy % 100
    let j = yy / 100
    let h = (d + 13 * (mm + 1) / 5 + k + k / 4 + j / 4 + 5 * j) % 7
    match h {
        0 => "saturday",
        1 => "sunday",
        2 => "monday",
        3 => "tuesday",
        4 => "wednesday",
        5 => "thursday",
        6 => "friday",
        _ => "never",
    }
}
fn main() -> !int {
    print("{weekday(2026, 8, 31)}")
    print("{weekday(2000, 1, 1)}")
    print("{weekday(1969, 7, 20)}")
    0
}
```

```console
$ lupin ex4-11.lu
monday
saturday
sunday
```

The shift moves February — the month whose length changes — to the
*end* of the counting year, so the leap day sits after every month the
formula counts across. That buys back a uniform month-length pattern:
`13 * (mm + 1) / 5` is a fixed staircase (the 31-30-31-30-31 rhythm
from March onward), which no formula can be over a year that keeps
February in the middle. The `_ => "never"` arm is the price of
matching on an `int`: `% 7` proves the range to you, but the checker
wants the row closed, and "never" is the honest name for it.
