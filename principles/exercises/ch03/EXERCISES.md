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
is 2147395600 and fits. The trap reports the true product — the
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
zero, so `-7 / 2` is −3, not Python's floored −4 — and the remainder
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
over — `first` owns `"ada"` from that point, so the later rebinding of
`name` replaces what `name` holds without reaching anything `first`
has. The deep story of "hands the value over" — what it means for the
source afterward, and when it traps — is chapter 7's, on purpose; this
chapter needs only the direction of the handover.

Audit note (authoring-time finding): a reassignment to a `let` binding
(`let n = 1` then `n = 2`) is accepted and executes under both tools
today — lupin prints `2`, wolfc reports no diagnostic. The book
teaches `let` as single-assignment; the missing rejection is filed for
the trackers.
