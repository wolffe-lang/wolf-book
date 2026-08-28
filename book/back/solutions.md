# Solutions

Every exercise in chapters 1 through 30 has a solution here, and
every solution program is a sample like any other: extracted,
executed, and snapshot-checked in the same CI run as the chapters. A
solution that stops compiling fails the book's build. Outputs are
pasted from those runs.

Solutions are collapsed. Open one when you have written yours, or
when you are stuck in the specific way that a hint cannot reach.
Reading a solution before attempting the exercise costs you the
exercise; the book has no way to stop you and no interest in trying.

The solo project publishes six checkpoints inside its own chapter
instead of answers, and it is the only page in the book that
withholds one. The coda sets no exercises.

## Chapter 1

<details>
<summary>Exercise 1-1 — [§1.1](../ch01.md#1.1)</summary>

**Exercise 1-1** *(fingers · lupin)* — Type the hello program exactly as
printed and run it. Then make it print a second line of your choosing.

Solution — `ch01/ex1-1.lu`:

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
</details>

<details>
<summary>Exercise 1-2 — [§1.3](../ch01.md#1.3)</summary>

**Exercise 1-2** *(comprehension · lupin)* — Before running, write down
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
</details>

<details>
<summary>Exercise 1-3 — [§1.1](../ch01.md#1.1)</summary>

**Exercise 1-3** *(fingers · lupin)* — Braces interpolate expressions,
not only names. Print a sentence that computes `6 * 7` twice inside one
string literal.

Solution — `ch01/ex1-3.lu`:

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
</details>

<details>
<summary>Exercise 1-4 — [§1.4](../ch01.md#1.4)</summary>

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
</details>

<details>
<summary>Exercise 1-5 — [§1.3](../ch01.md#1.3)</summary>

**Exercise 1-5** *(fingers + extension · lupin)* — The first table in
*The C Programming Language* converts Fahrenheit to Celsius. Write
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
</details>

<details>
<summary>Exercise 1-6 — [§1.5](../ch01.md#1.5)</summary>

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
</details>

<details>
<summary>Exercise 1-7 — [§1.2](../ch01.md#1.2)</summary>

**Exercise 1-7** *(fingers · wolf + lupin)* — Compile the greeting with
`wolf build`, run the binary, then run the source under `lupin`.
Compare the two outputs byte for byte — `diff <(./hello) <(lupin
hello.lu)` will do it. Then say which of the two runs could have
printed something different, and what it would mean about the language
if it had.

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
against its own string model — two separate pieces of code, written
from the specification rather than from each other. A byte of
disagreement between them is a bug in one implementation or a hole in
the specification, and it is found here rather than in your program.
</details>

<details>
<summary>Exercise 1-8 — [§1.5](../ch01.md#1.5)</summary>

**Exercise 1-8** *(comprehension · wolf + lupin)* — Build the greeting,
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
the object file keyed by everything that could change its contents —
the module's source, the compiler's own build id, the profile, and the
interface surfaces of what it depends on — so an unchanged key is an
answer already on disk. The key in your terminal will differ from the
one above; what will not differ is the word `reused`.
</details>

## Chapter 2

<details>
<summary>Exercise 2-1 — [§2.3](../ch02.md#2.3)</summary>

**Exercise 2-1** *(comprehension · lupin REPL)* — Predict all three
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
</details>

<details>
<summary>Exercise 2-2 — [§2.1](../ch02.md#2.1)</summary>

**Exercise 2-2** *(fingers · lupin)* — Using format specs, print a
two-column table: names left-aligned in 10 columns, numbers
right-aligned in 4.

Solution — `ch02/ex2-2.lu`:

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
</details>

<details>
<summary>Exercise 2-3 — [§2.3](../ch02.md#2.3)</summary>

**Exercise 2-3** *(comprehension · lupin)* — `"wolf"` has four bytes.
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
ex2-3.lu: trap(bounds): byte range 2..9 is outside a 4-byte string [mem.ub.defined] at 156..163
$ echo $?
3
```
</details>

<details>
<summary>Exercise 2-4 — [§2.2](../ch02.md#2.2)</summary>

**Exercise 2-4** *(extension · lupin)* — Extend the word counter to also
report lines and bytes for a `"""` multiline block. Predict which of the
three numbers is a byte count before running.

Solution — `ch02/ex2-4.lu`:

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
</details>

<details>
<summary>Exercise 2-5 — [§2.2](../ch02.md#2.2)</summary>

**Exercise 2-5** *(comprehension · lupin REPL)* — Predict all three
lengths before evaluating: `"\n".len`, `r"\n".len`, `r"C:\temp".len`.

Solution: 1, 2, 7. In an ordinary literal `\n` is one byte, a newline.
In a raw literal it is two bytes, a backslash and an `n` — raw means
the escape table is off, not that backslashes are special some other
way. `r"C:\temp"` is the seven bytes you can count.

```console
$ lupin eval '"\n".len'
1 : i64
$ lupin eval 'r"\n".len'
2 : i64
$ lupin eval 'r"C:\temp".len'
7 : i64
```
</details>

<details>
<summary>Exercise 2-6 — [§2.3](../ch02.md#2.3)</summary>

**Exercise 2-6** *(comprehension · lupin REPL)* — `"wolf"` has four
bytes. Predict each of these, precisely — value or event:
`"wolf"[..2]`, `"wolf"[2..]`, `"wolf"[4..4]`, `"wolf"[3..2]`.

Solution: `wo`, `lf`, the empty string, and a trap. An open end fills
in the boundary; `4..4` is an empty range *at* a legal boundary, which
is a value, not a fault; `3..2` runs backward, and a backward range is
outside the contract no matter how short it is.

```console
$ lupin eval '"wolf"[..2]'
wo : str
$ lupin eval '"wolf"[2..]'
lf : str
$ lupin eval '"wolf"[4..4]'
 : str
$ lupin eval '"wolf"[3..2]'
trap(bounds): byte range 3..2 is outside a 4-byte string [mem.ub.defined] at 0..12
the session survives the trap; the world is as the fault left it [repl.trap.alive]
```

The wrong answer worth ruling out: `[4..4]` does not trap. The boundary
after the last byte is a real position — it is where appending happens.
</details>

<details>
<summary>Exercise 2-7 — [§2.4](../ch02.md#2.4)</summary>

**Exercise 2-7** *(extension · lupin)* — Write `encode`, a run-length
encoder over bytes: `"aaabcc"` becomes `"a3b1c2"`. Walk the string with
byte slices and equality only. What does your encoder do with the empty
string, and did you have to write a special case for it?

Solution — `ch02/ex2-7.lu`:

```wolf
fn encode(s: str) -> str {
    var out = ""
    var i = 0
    while i < s.len {
        let ch = s[i..i + 1]
        var n = 1
        while i + n < s.len && s[i + n..i + n + 1] == ch { n += 1 }
        out += "{ch}{n}"
        i += n
    }
    out
}
fn main() -> !int {
    print(encode("aaabcc"))
    0
}
```

```console
$ lupin ex2-7.lu
a3b1c2
```

The empty string needs no special case: the outer loop's condition is
false immediately and `out` is returned as the empty string it started
as. A loop whose bounds are honest handles its degenerate input by
arithmetic, not by an `if` bolted on the front.
</details>

<details>
<summary>Exercise 2-8 — [§2.5](../ch02.md#2.5)</summary>

**Exercise 2-8** *(comprehension · lupin REPL)* — `s` is
`"wolfpack"`. Predict all four values, then say what slicing `s`
cost — did any of these lines copy eight bytes?

```console
wolf> let s = "wolfpack"
wolf> let t = s[..4]
wolf> t
wolf : str
wolf> t.len
4 : i64
wolf> s.len
8 : i64
wolf> s[4..].len
4 : i64
```

Solution: `wolf`, 4, 8, 4 — and nothing copied. A slice is a *view*:
two words, a pointer and a length, aimed into bytes that already
exist. `s` is untouched by every line here, which is why `s.len` is
still 8 after `t` was made from it. The chapter's cost claim is
checkable from the values alone: if slicing copied, substring-heavy
code would pay by the byte; because it is two words, `t = s[..4]`
costs the same whether `s` is eight bytes or eight megabytes.
</details>

## Chapter 3

<details>
<summary>Exercise 3-1 — [§3.4](../ch03.md#3.4)</summary>

**Exercise 3-1** *(comprehension · lupin)* — Predict the one line this
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
</details>

<details>
<summary>Exercise 3-2 — [§3.1](../ch03.md#3.1)</summary>

**Exercise 3-2** *(comprehension · wolf + lupin)* — The pack loses its
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
./ex3-2.lu: trap(use-after-move): `p.lead` was moved out and is uninitialized here [mem.tier0.move.2] at 381..387; `p.lead` moved here at 337..348
$ echo $?
3
```

One rule, two enforcement points: the compiler proves it before the
program starts; lupin enforces it dynamically and cites the same clause
family. This differential is the book's spine and the reader meets it
here first.
</details>

<details>
<summary>Exercise 3-3 — [§3.3](../ch03.md#3.3)</summary>

**Exercise 3-3** *(comprehension · lupin)* — `2147483647` is `i32`'s
ceiling. Predict what `big + 1` does in a release build. (Trick
warning: the answer is the same in every build.)

Solution: it traps, in every profile — X3 is the decision the trap line
cites, and the line also names the spelling for intentional wraparound:

```console
$ lupin ex3-3.lu
ex3-3.lu: trap(overflow): `+` produced 2147483648, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 178..185
```
</details>

<details>
<summary>Exercise 3-4 — [§3.3](../ch03.md#3.3)</summary>

**Exercise 3-4** *(comprehension · lupin)* — The divisor is computed,
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
ex3-4.lu: trap(div-zero): division by zero is defined behavior in wolf: it traps [mem.ub.defined] at 187..192
```
</details>

<details>
<summary>Exercise 3-5 — [§3.2](../ch03.md#3.2)</summary>

**Exercise 3-5** *(design)* — Wolf has no ternary operator. Write the
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
</details>

<details>
<summary>Exercise 3-6 — [§3.4](../ch03.md#3.4)</summary>

**Exercise 3-6** *(extension (break-it-on-purpose) · lupin)* — Using one
`i32` binding and one `*`, write the smallest program that traps with
`overflow` on `i32`. State, before running it, why the number you chose
is the smallest one that works, and what the trap line will say the
product was.

Solution — `ch03/ex3-6.lu`:

```wolf
fn main() -> !int {
    let n: i32 = 46341
    print("{n * n}")
    0
}
```

```console
$ lupin ex3-6.lu
ex3-6.lu: trap(overflow): `*` produced 2147488281, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 160..165
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
</details>

<details>
<summary>Exercise 3-7 — [§3.4](../ch03.md#3.4)</summary>

**Exercise 3-7** *(comprehension · lupin)* — Predict both lines. If you
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
</details>

<details>
<summary>Exercise 3-8 — [§3.1](../ch03.md#3.1)</summary>

**Exercise 3-8** *(comprehension · lupin)* — Predict the one printed
line, then answer the pointed part: after `name = "grace"`, what
happened to `"ada"` — and why does `first` not care?

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
</details>

## Chapter 4

<details>
<summary>Exercise 4-1 — [§4.2](../ch04.md#4.2)</summary>

**Exercise 4-1** *(fingers · lupin)* — Functions are values. Write
`compose` so that `compose(double, double)` returns a function, and
apply it to 10.

Solution — `ch04/ex4-1.lu`:

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
</details>

<details>
<summary>Exercise 4-2 — [§4.3](../ch04.md#4.3)</summary>

**Exercise 4-2** *(comprehension · lupin)* — Predict the order of the
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
</details>

<details>
<summary>Exercise 4-3 — [§7.4](../ch07.md#7.4)</summary>

**Exercise 4-3** *(extension · lupin)* — Give the list a `shrink`
function to pair with `grow`. Then, without running anything, state how
you would find every mutation in this program with one search.

Solution — `ch04/ex4-3.lu`:

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
</details>

<details>
<summary>Exercise 4-4 — [§7.5](../ch07.md#7.5)</summary>

**Exercise 4-4** *(comprehension + spelunking · wolf)* — One of these
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
</details>

<details>
<summary>Exercise 4-5 — [§4.1](../ch04.md#4.1)</summary>

**Exercise 4-5** *(comprehension · lupin)* — Euclid's algorithm, in the
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
</details>

<details>
<summary>Exercise 4-6 — [§4.3](../ch04.md#4.3)</summary>

**Exercise 4-6** *(comprehension · lupin)* — A `defer` is registered
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
</details>

<details>
<summary>Exercise 4-7 — [§4.4](../ch04.md#4.4)</summary>

**Exercise 4-7** *(extension · lupin)* — Build `day_of_year(month, day,
leap)` from two functions: `days_in(month, leap)` as one `match`, and a
loop that sums the months before yours. Verify: March 1st is day 60 in
a common year. Which date is day 60 in a leap year, and where in your
code did that difference come from?

Solution — `ch04/ex4-7.lu`:

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
</details>

## Chapter 5

<details>
<summary>Exercise 5-1 — [§5.1](../ch05.md#5.1)</summary>

**Exercise 5-1** *(fingers · lupin)* — A `List` is also a stack. Push
three values, pop one, and print the popped value and the remaining
length.

Solution — `ch05/ex5-1.lu`:

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
</details>

<details>
<summary>Exercise 5-2 — [§5.1](../ch05.md#5.1)</summary>

**Exercise 5-2** *(fingers · lupin)* — Score the pack: write two scores
into a `Map`, raise one by reading it back, and print the table with
format specs.

Solution — `ch05/ex5-2.lu`:

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
</details>

<details>
<summary>Exercise 5-3 — [§5.3](../ch05.md#5.3)</summary>

**Exercise 5-3** *(extension · lupin)* — Write `first[T]` with a
fallback for the empty case, and call it twice: once with the type
named, once letting inference name it.

Solution — `ch05/ex5-3.lu`:

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
</details>

<details>
<summary>Exercise 5-4 — [§5.4](../ch05.md#5.4)</summary>

**Exercise 5-4** *(comprehension · lupin)* — The list has one element.
Predict `xs[10]`, precisely: what kind of event, and what exit code.

Solution: a `bounds` trap, exit 3 — same contract as string slicing,
because indexing is checked everywhere:

```console
$ lupin ex5-4.lu
ex5-4.lu: trap(bounds): index 10 is outside a collection of 1 element(s) [mem.ub.defined] at 193..199
$ echo $?
3
```
</details>

<details>
<summary>Exercise 5-5 — [§5.3](../ch05.md#5.3)</summary>

**Exercise 5-5** *(design)* — Wolf writes generics `top[T]` and indexing
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
</details>

<details>
<summary>Exercise 5-6 — [§5.2](../ch05.md#5.2)</summary>

**Exercise 5-6** *(extension · lupin)* — `uniq` counts adjacent
duplicates; yours will count all of them and keep first-seen order.
Read a multiline block line by line and print each distinct line once,
with its count, in the order lines first appeared. Two parallel lists —
one of lines seen, one of counts — are enough. Why does a `Map` alone
not solve this?

Solution — `ch05/ex5-6.lu`:

```wolf
fn index_of(xs: List[str], s: str) -> int {
    var i = 0
    for x in xs {
        if x == s { return i }
        i += 1
    }
    0 - 1
}
fn main() -> !int {
    let log = """
        howl
        howl
        scratch
        howl
        scratch
        """
    var seen = List[str]()
    var counts = List[int]()
    for line in log.lines() {
        let at = index_of(seen, line)
        if at < 0 {
            (mut seen).push(line)
            (mut counts).push(1)
        } else {
            counts[at] = counts[at] + 1
        }
    }
    var i = 0
    for s in seen {
        print("{counts[i]:>4} {s}")
        i += 1
    }
    0
}
```

```console
$ lupin ex5-6.lu
   3 howl
   2 scratch
```

A `Map` alone loses the arrival order: its pairs come back in the map's
order, not the input's. The list carries the order and the parallel
list carries the tally — two simple structures composing beats one
structure that almost fits.
</details>

<details>
<summary>Exercise 5-7 — [§5.4](../ch05.md#5.4)</summary>

**Exercise 5-7** *(comprehension + extension · lupin)* — An RPN
evaluator is a loop and a stack, and the stack is a `List`. Given the
tokens `3 4 + 2 *`, trace the stack contents after each token on
paper, then run. Then answer from your trace, not from the code: which
input would make `stack.len < 2` true at an operator, and what does
your evaluator do about it?

Solution — `ch05/ex5-7.lu`:

```wolf
fn eval_rpn(tokens: List[str]) -> int ! {Underflow, BadToken} {
    var stack = List[int]()
    for t in tokens {
        if t == "+" || t == "-" || t == "*" || t == "/" {
            if stack.len < 2 { return Underflow }
            let b = (mut stack).pop()
            let a = (mut stack).pop()
            if t == "+" { (mut stack).push(a + b) } else if t == "-" { (mut stack).push(a - b) } else if t == "*" { (mut stack).push(a * b) } else { (mut stack).push(a / b) }
        } else {
            let n = t.to_int() else { return BadToken }
            (mut stack).push(n)
        }
    }
    if stack.len != 1 { return Underflow }
    (mut stack).pop()
}
fn main() -> !int {
    var tokens = List[str]()
    (mut tokens).push("3")
    (mut tokens).push("4")
    (mut tokens).push("+")
    (mut tokens).push("2")
    (mut tokens).push("*")
    let v = eval_rpn(tokens) else |_| { return 1 }
    print("{v}")
    0
}
```

```console
$ lupin ex5-7.lu
14
```

The trace: `[3]`, `[3 4]`, `[7]`, `[7 2]`, `[14]`. An input like
`3 +` reaches the operator with one element on the stack, and the
evaluator returns `Underflow` instead of trapping on `pop` — the error
row is doing bounds-checking's job one level up, where the caller can
do something about it. (The row previews chapter 6; reading it is
enough here.)
</details>

<details>
<summary>Exercise 5-9 — [§5.5](../ch05.md#5.5)</summary>

**Exercise 5-9** *(extension)* — Add a third shape to the `Draw`
example, and then make `render` count its calls: what has to change,
and what — pleasantly — does not?

Solution — the third impl is three lines, and that is the point:

```wolf,run(exit=0)
trait Draw {
    fn draw(self) -> str
}
struct Dot { x: int }
struct Ring { r: int }
struct Star { points: int }
impl Draw for Dot { fn draw(self) -> str { "dot at {self.x}" } }
impl Draw for Ring { fn draw(self) -> str { "ring of {self.r}" } }
impl Draw for Star { fn draw(self) -> str { "star of {self.points}" } }
fn render(o: dyn Draw) -> str { o.draw() }
fn main() -> !int {
    let d = Dot { x: 3 }
    let r = Ring { r: 9 }
    let s = Star { points: 5 }
    var calls = 0
    print(render(d as dyn Draw))
    calls = calls + 1
    print(render(r as dyn Draw))
    calls = calls + 1
    print(render(s as dyn Draw))
    calls = calls + 1
    print("{calls} renders")
    0
}
```

What changed: one struct, one impl, one binding, one call. What did
not: `render`. That is erasure earning its keep — the function that
takes `dyn Draw` never learns how many implementors exist. The counter
lives at the call sites, because `render` has nowhere to keep state:
it borrows its argument and owns nothing — the same ownership honesty
Part 2 makes precise.
</details>

<details>
<summary>Exercise 5-10 — [§5.5](../ch05.md#5.5)</summary>

**Exercise 5-10** *(design)* — The cast-a-binding rule exists because
the dyn pair points at its operand rather than owning it. What would
the language have to invent for `Dot { x: 3 } as dyn Draw` to be legal,
and who would pay for it?

Solution — the temporary needs a home that outlives the expression, so
the language would have to invent one: a hidden allocation (a box the
reader never wrote), or a compiler-synthesized binding with a lifetime
the reader never chose. Both are costs paid silently, and wolf's
temperament is that erasure may change dispatch but never ownership —
the pair points at your value, in your frame or your region, and the
`let home = …` the error asks for is the language declining to
allocate behind your back. The reader pays one visible line; the
alternative is every reader paying an invisible allocation.
</details>

## Chapter 6

<details>
<summary>Exercise 6-1 — [§6.2](../ch06.md#6.2)</summary>

**Exercise 6-1** *(fingers · lupin)* — Write `parse` so empty input is
an error, and give two call sites: one defaulting with `else 0`, one
with `else 7`. Predict both prints first.

Solution — `ch06/ex6-1.lu`:

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
</details>

<details>
<summary>Exercise 6-2 — [§6.2](../ch06.md#6.2)</summary>

**Exercise 6-2** *(comprehension · lupin)* — `chain` calls `parse`
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
</details>

<details>
<summary>Exercise 6-3 — [§6.1](../ch06.md#6.1)</summary>

**Exercise 6-3** *(extension · lupin)* — Grow the row: add a `TooLong`
variant for inputs over four bytes and handle it. What else did you
have to change, and what told you?

Solution — `ch06/ex6-3.lu` (excerpt):

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
</details>

<details>
<summary>Exercise 6-4 — [§6.1](../ch06.md#6.1)</summary>

**Exercise 6-4** *(comprehension · lupin)* — The error carries a
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
</details>

<details>
<summary>Exercise 6-5 — [§6.3](../ch06.md#6.3)</summary>

**Exercise 6-5** *(comprehension · lupin)* — `errdefer` runs only on
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
</details>

<details>
<summary>Exercise 6-6 — [§6.1](../ch06.md#6.1)</summary>

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
</details>

<details>
<summary>Exercise 6-7 — [§6.2](../ch06.md#6.2)</summary>

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
</details>

<details>
<summary>Exercise 6-8 — [§6.5](../ch06.md#6.5)</summary>

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
</details>

<details>
<summary>Exercise 6-9 — [§6.4](../ch06.md#6.4)</summary>

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
</details>

<details>
<summary>Exercise 6-10 — [§6.5](../ch06.md#6.5)</summary>

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
</details>

## Chapter 7

<details>
<summary>Exercise 7-1 — [§7.1](../ch07.md#7.1)</summary>

**Exercise 7-1** *(comprehension · wolf + lupin)* — Chapter 3's broken
`Pack` program, with one line added:

```wolf
struct Pack { lead: str, tail: str }
fn adopt(take w: str) -> str { w }
fn main() -> !int {
    var p = Pack { lead: "ada", tail: "grace" }
    let a = adopt(take p.lead)
    p.lead = "lin"
    let c = p.lead
    print("{a} {c} {p.tail}")
    0
}
```

Exercise 3-2's version was rejected with E1001 at `let c = p.lead`.
Predict both tools' behavior now, and name the sentence in the E1001
diagnostic you saw in chapter 3 that already told you the answer.

Solution: both accept it. Assigning to a moved-from place makes it
live again — the diagnostic's note said exactly that, and this is that
note performed. Both implementations run it, and they print the same
line, which is the differential doing its ordinary job:

```console
$ lupin ex7-1.lu
ada lin grace
```

The compiler has one thing to add, and it is about the signature rather
than the move: `adopt` takes a value only to hand it straight back, so
`W1003` asks whether the `take` earns itself. The binary is produced
anyway and prints `ada lin grace`.
</details>

<details>
<summary>Exercise 7-2 — [§7.2](../ch07.md#7.2)</summary>

**Exercise 7-2** *(fingers · lupin)* — Draw the ownership tree of `den`
below before running anything: one box per value, one arrow per field.
Then move the deepest leaf out with `move` and verify, by printing
them, that the leaf's *sibling* and its *cousins* are all still
usable:

Solution — `ch07/ex7-2.lu`:

```wolf
struct Wolf { name: str, call: str }
struct Den { alpha: Wolf, beta: Wolf }
fn main() -> !int {
    var den = Den {
        alpha: Wolf { name: "ada", call: "awoo" },
        beta: Wolf { name: "grace", call: "yip" },
    }
    let promoted = move den.alpha.name
    print("{promoted} leads")
    print("{den.alpha.call} still works")
    print("{den.beta.name} still here")
    0
}
```

```console
$ lupin ex7-2.lu
ada leads
awoo still works
grace still here
```

The move emptied exactly one path — `den.alpha.name` — and the tree's
other four leaves never noticed. (`move` is the plain-expression
spelling; `take` is the same act written at a call site.)
</details>

<details>
<summary>Exercise 7-3 — [§7.2](../ch07.md#7.2)</summary>

**Exercise 7-3** *(extension (break-it-on-purpose) · wolf + lupin)* — Using one
struct, one function taking `take`, and nothing else, write the
smallest program that traps `use-after-move` *through a field*. Predict
the compiler's E-code and the interpreter's trap kind before checking
both. Why does the exercise say "through a field" — what would be
different, and what the same, with a bare local?

Solution — `ch07/ex7-3.lu`:

```wolf
struct S { a: str }
fn eat(take w: str) -> str { w }
fn main() -> !int {
    var s = S { a: "x" }
    let t = eat(take s.a)
    let u = s.a
    print("{t} {u}")
    0
}
```

```console
$ wolf conform-run ./ex7-3.lu
error[E1001]: `s.a` is used here after its value moved away
 --> ./ex7-3.lu:9:13
  |
8 |     let t = eat(take s.a)
  |                      --- `s.a` moved here
9 |     let u = s.a
  |             ^^^ used after the move
  |
  = note: re-initializing the place (assigning to it) also makes it usable again.
help: to keep the original, copy it at the move
  |
8 |     let t = eat(take copy s.a)
  |
```

```console
$ lupin ex7-3.lu
ex7-3.lu: trap(use-after-move): `s.a` was moved out and is uninitialized here [mem.tier0.move.2] at 236..239; `s.a` moved here at 214..222
$ echo $?
3
```

A bare local would produce the same E-code and the same trap kind; the
field version is the stronger test because it proves the tracking is
per-path, not per-variable — `s` as a whole is neither dead nor alive,
only `s.a` is dead. Both tools cite the same clause family,
`[mem.tier0.move.2]`, which is the differential spine again.
</details>

<details>
<summary>Exercise 7-4 — [§7.2](../ch07.md#7.2)</summary>

**Exercise 7-4** *(comprehension · lupin)* — Every field of `P` is an
`int`. Predict what the second line of `main` does to `a`:

```wolf
struct P { x: int, y: int }
fn main() -> !int {
    let a = P { x: 1, y: 2 }
    let b = a
    print("{a.x} {b.y}")
    0
}
```

Solution: it moves `a`, ints and all — the trap blames `a.x`'s read and
points at `let b = a`. Structs move on assignment whatever they
contain; there is no "cheap enough to copy silently" tier for
user-defined types. The fix spells the duplication where it happens:

```console
$ lupin ex7-4.lu
ex7-4.lu: trap(use-after-move): `a.x` was moved out and is uninitialized here [mem.tier0.move.2] at 233..236; `a` moved here at 219..220
```

`ch07/ex7-4b.lu` changes one word — `let b = copy a` — and:

```console
$ lupin ex7-4b.lu
1 2
```

The wrong answer worth ruling out: "ints are `Copy`, so the struct
copies." Wolf's rule is per-*decision*, not per-*type*: the reader of
`let b = copy a` knows a duplication happened without looking up what
`P` contains.
</details>

<details>
<summary>Exercise 7-6 — [§7.4](../ch07.md#7.4)</summary>

**Exercise 7-6** *(fingers + spelunking · lupin)* — Write `swap` for
two `int`s using `mut` at both ends, and verify it. Then state the
single search you would run over a strange codebase to find every line
that can mutate anything — and what property of the language makes the
search complete.

Solution — `ch07/ex7-6.lu`:

```wolf
fn swap(mut a: int, mut b: int) {
    let t = a
    a = b
    b = t
}
fn main() -> !int {
    var x = 1
    var y = 3
    swap(mut x, mut y)
    print("{x} {y}")
    0
}
```

```console
$ lupin ex7-6.lu
3 1
```

The search is `grep '(mut '` (X1's argument, from exercise 4-3, now
stated as a rule): call-site `mut` is mandatory, so a call that can
write through an argument *says so at the call*. Add `grep 'var '` for
locals and the audit is the whole mutation surface — two searches, no
false negatives, which is what "required at both ends" buys.
</details>

<details>
<summary>Exercise 7-7 — [§7.5](../ch07.md#7.5)</summary>

**Exercise 7-7** *(comprehension · wolf + lupin)* — The simplest
possible exclusivity violation: one place, claimed twice.

```wolf
fn bump2(mut a: int, mut b: int) {
    a += 1
    b += 1
}
fn main() -> !int {
    var n = 0
    bump2(mut n, mut n)
    n
}
```

Predict what each tool says, then answer the design question hiding
under it: if the call *were* allowed, what would `n` be afterward — and
why is "it depends on the body" the real reason for the rule?

Solution: wolf rejects, lupin traps, same rule:

```console
$ wolf conform-run ./ex7-7.lu
error[E1002]: `n` cannot go `mut` here: it overlaps `n`, already passed `mut` in this call
  --> ./ex7-7.lu:10:22
   |
10 |     bump2(mut n, mut n)
   |               - `n` is passed `mut` here
   |                      ^ second exclusive claim on the same place
   |
   = note: the same place twice is never disjoint.
```

```console
$ lupin ex7-7.lu
ex7-7.lu: trap(exclusivity): `n` is accessed as `mut` while `n` is held as `mut`; the paths conflict [mem.tier0.excl.1] at 215..220; `n` held here at 208..213
```

If allowed, `n` could be 1 or 2 depending on whether `a` and `b` are
distinct copies written back in some order or two names for one cell —
the body decides, and the caller cannot see the body. Exclusivity makes
the answer not depend on the body: two `mut` claims must be provably
disjoint places, so aliasing questions are settled at the call site,
which is also what lets the compiler hand `noalias` facts to the
optimizer (§7.7's subject).
</details>

<details>
<summary>Exercise 7-8 — [§7.5](../ch07.md#7.5)</summary>

**Exercise 7-8** *(comprehension + fingers · lupin)* — Four call shapes against
`struct P { a: Q, b: Q }`, `struct Q { n: int }`. Verdict for each,
before checking any:

1. `f(mut p.a, mut p.b)`
2. `f(mut p.a.n, mut p.b.n)`
3. `f(mut p.a, mut p.a.n)`
4. `f(mut p, mut p.b)`

Solution: 1 and 2 are legal — disjoint fields, and leaves of disjoint
subtrees. 3 and 4 are rejected: in each, one path is a *prefix* of the
other, and a place conflicts with every place inside it. The rule from
exercise 4-4, one sentence: two paths conflict iff one is a prefix of
the other (`[mem.model.path.disjoint]`). The legal pair, run:

```wolf
fn bump(mut u: int, mut v: int) {
    u += 1
    v += 1
}
var p = P { a: Q { n: 5 }, b: Q { n: 7 } }
bump(mut p.a.n, mut p.b.n)
```

```console
$ lupin ex7-8.lu
6 8
```
</details>

<details>
<summary>Exercise 7-9 — [§7.6](../ch07.md#7.6)</summary>

**Exercise 7-9** *(spelunking · wolf)* — Run `wolf --explain E1001` and
read all of it. Quote the sentence that licenses re-initialization
(exercise 7-1's move), the phrase that states field granularity
(exercise 7-3's), and the one word in the first paragraph that makes
`let b = a` and `f(take a)` the same subject.

Solution:

```console
$ wolf --explain E1001
E1001: this value was moved away (or never given one) before this use

In wolf, assignment and argument passing *move* a value: after
`let b = a` or `f(take a)`, the name `a` no longer holds anything —
its value went to the new place, whole. Reading a moved-from (or
never-initialized) name would read nothing, so the checker stops it
here and points at the move it happened in. Moves are field-granular:
moving `s.a` away leaves `s.b` usable, and only the moved path is
off-limits. To keep using the original, make the duplication explicit
where the move happens — `copy a` produces an independent value of
any type — or give the name a new value first: assigning to a
moved-from place makes it live again.
```

The license is the last clause: "assigning to a moved-from place makes
it live again." The granularity phrase is "moving `s.a` away leaves
`s.b` usable." The one word is "move" itself — the text's first
sentence puts assignment and argument passing under the same verb,
which is why chapters 3 and 7 have been describing one mechanism, not
two.
</details>

<details>
<summary>Exercise 7-10 — [§7.6](../ch07.md#7.6)</summary>

**Exercise 7-10** *(design)* — Rust's zero-copy parser hands out `&str`
slices of an input buffer it does not own, with lifetimes proving the
buffer outlives every slice. Wolf has no lifetime annotations, so that
API shape is not expressible for arbitrary callers. Sketch the wolf
alternatives — copying the token text, returning byte ranges
`(start, end)` into a caller-held string, or parsing inside a region
and freezing the result — and argue which one a tokenizer library
should ship. What does each cost, and who pays it?

Solution (discussion): the range API is the honest default: tokens as
`(start, end)` pairs are plain values, move freely, and cost eight
bytes each; the caller pays one indirection — `input[t.0..t.1]` — at
each use, checked. Copying pays allocation per token to buy the
simplest caller code; for a config-file parser nobody measures, that
is the right trade, and for a log-ingest loop it is not. The
freeze design is the interesting one: parse into a region, freeze it,
and hand back *imm* tokens that reference the frozen input — sharing
without copies and without lifetimes, at the cost of making the input
immutable forever and region-resident from the start; it fits a
compiler front end, where the source text never changes after load.
The library should ship ranges and let the other two be five-line
wrappers, because ranges are the only shape that never dictates the
caller's memory story. What Rust buys with lifetime annotations is
making the borrow design *default*; what wolf buys by refusing them is
that no signature in this paragraph mentions anything but values.
</details>

<details>
<summary>Exercise 7-11 — [§7.7](../ch07.md#7.7)</summary>

**Exercise 7-11** *(fingers · lupin REPL)* — In the REPL, move a string
out of one binding into another, then read both — the corpse first.
What does the session do that a compiled program cannot, and which
clause tag names the reason the trap did not end your session?

Solution — one session:

```console
$ lupin
wolf> let s = "wolf"
wolf> let t = move s
wolf> s
trap(use-after-move): `s` was moved out and is uninitialized here [mem.tier0.move.2] at 0..1
  `s` moved here at 8..14
the session survives the trap; the world is as the fault left it [repl.trap.alive]
wolf> t
wolf : str
wolf> :quit
```

The session takes the trap and keeps the world — `[repl.trap.alive]` —
so the state a fault left behind is inspectable, which is the REPL's
whole advantage over a crashed process. The value is intact in `t`:
a move is a transfer, never a destruction, and the machine-level story
(§7.7) is a memcpy after which the source is *forgotten*, not zeroed.
</details>

<details>
<summary>Exercise 7-12 — [§7.7](../ch07.md#7.7)</summary>

**Exercise 7-12** *(extension · lupin)* — The longest common
subsequence of two line lists is the skeleton every diff tool hangs
on. Build the DP table as a `List[List[int]]` and return its corner.
For the two three-line "files" in the solution, compute the answer on
paper first: which two lines survive in both?

Solution — `ch07/ex7-12.lu` (core):

```wolf
fn lcs_len(a: List[str], b: List[str]) -> int {
    var table = List[List[int]]()
    var i = 0
    while i <= a.len {
        var row = List[int]()
        var j = 0
        while j <= b.len {
            row.push(0)
            j += 1
        }
        table.push(row)
        i += 1
    }
    i = 1
    while i <= a.len {
        var j = 1
        while j <= b.len {
            if a[i - 1] == b[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1
            } else if table[i - 1][j] >= table[i][j - 1] {
                table[i][j] = table[i - 1][j]
            } else {
                table[i][j] = table[i][j - 1]
            }
            j += 1
        }
        i += 1
    }
    table[a.len][b.len]
}
```

```console
$ lupin ex7-12.lu
2
```

"the wolf runs" and "the elk listens" survive; the moon line does not.
Note what the function signature says about ownership: both lists are
borrowed — the caller keeps them, un-moved, and no annotation was
spent saying so.
</details>

<details>
<summary>Exercise 7-13 — [§7.7](../ch07.md#7.7)</summary>

**Exercise 7-13** *(comprehension + extension · lupin)* — Extend 7-12
into a printing diff: walk the finished table backward from the corner,
emitting `  ` for common lines, `- ` for deletions, `+ ` for
additions. Before running, predict the full output for `old` = the
wolf/moon/elk lines and `new` = wolf/elk/river. Then explain why the
walk must go *backward*.

Solution — `ch07/ex7-13.lu` (the walk):

```wolf
fn print_diff(a: List[str], b: List[str], table: List[List[int]], i: int, j: int) {
    if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
        print_diff(a, b, table, i - 1, j - 1)
        print("  {a[i - 1]}")
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        print_diff(a, b, table, i, j - 1)
        print("+ {b[j - 1]}")
    } else if i > 0 {
        print_diff(a, b, table, i - 1, j)
        print("- {a[i - 1]}")
    }
}
```

```console
$ lupin ex7-13.lu
  the wolf runs
- the moon watches
  the elk listens
+ the river answers
```

The table's cell `(i, j)` only knows the best answer *up to* that
point; which choice produced it is recoverable only by comparing a
cell with its neighbors, and the neighbors that explain `(i, j)` are
behind it. The recursion runs to the origin and prints on the way
back out, so the output comes out forward — backward walk, forward
story.
</details>

## Chapter 8

<details>
<summary>Exercise 8-1 — [§8.1](../ch08.md#8.1)</summary>

**Exercise 8-1** *(comprehension · prose)* — Three programs you have
met or written: (a) a web server handling one request — parse the
headers, build a response, send it; (b) a compiler pass — read an AST,
produce a transformed AST, discard the scratch; (c) a game loop — each
frame computes collision pairs and a display list, then draws. For
each, name the group of allocations that share a death, the moment
they all die, and the one value (if any) that must survive. No wolf
required; the point is that the regions were already there.

Solution: (a) everything parsed and built for the request dies when
the response is flushed; the survivor is the response bytes (and any
session state, which was never the request's to own). (b) the scratch
and the *input* AST die at pass end; the survivor is the output AST —
which is why pass frameworks so often copy it out of an arena. (c) the
collision pairs and display list die at frame end, sixty times a
second; the survivor is the game state that feeds the next frame. In
every case the granule was "these thousands of objects, together," and
the death was one moment — a shape single-owner trees cannot spell,
which is the chapter's opening argument.
</details>

<details>
<summary>Exercise 8-2 — [§8.2](../ch08.md#8.2)</summary>

**Exercise 8-2** *(fingers · lupin)* — Sum the first hundred integers
using a list a helper function builds — with the helper writing no
region code at all — inside `region tmp { }`. State where `fill`'s
list is allocated, and what happens to it at the closing brace.

Solution — `ch08/ex8-2.lu`:

```wolf
fn fill(n: int) -> List[int] {
    var xs = List[int]()
    for i in 0..n { (mut xs).push(i) }
    xs
}
fn main() -> !int {
    var total = 0
    region tmp {
        let xs = fill(100)
        for x in xs { total += x }
    }
    print("{total}")
    0
}
```

```console
$ lupin ex8-2.lu
4950
```

`fill` allocates into the *caller's* current region — that is the
default, and it is why the helper needed no annotation. At the brace,
the region frees wholesale: the list, its buffer, all of it, in one
motion. `total` survives because an `int` is a value in the frame, not
an allocation in the region.
</details>

<details>
<summary>Exercise 8-3 — [§8.2](../ch08.md#8.2)</summary>

**Exercise 8-3** *(comprehension · wolf)* — One assignment tries to
smuggle a region value past the brace:

```wolf
struct Node { value: int }
fn main() -> !int {
    var keep = Node { value: 0 }
    region tmp {
        keep = Node { value: 7 }
    }
    if keep.value == 7 { 0 } else { 1 }
}
```

Predict the compiler's verdict, and — before reading the diagnostic —
list the three code locations you expect it to point at.

Solution: rejected, E1010, and the diagnostic points at exactly the
three moments of the story: the region's creation, the escape, and the
free.

```console
$ wolf conform-run ./ex8-3.lu
error[E1010]: `keep` still holds a value allocated in region `tmp` when the region is freed
 --> ./ex8-3.lu:8:9
  |
7 |     region tmp {
  |            --- region `tmp` is created here
8 |         keep = Node { value: 7 }
  |         ^^^^ the value flows out of the region here
  |                ----------------- allocated here, into region `tmp`
9 |     }
  |     - the region is freed here — everything in it is freed wholesale, as one unit
  |
  = note: to keep the value, allocate it where it must live: build it outside the region block, or
    aim the allocation at a longer-lived region explicitly (`let r = region()` … `in r { …
    }`); widening the region block to cover every use also works. Two keep-alive
    alternatives change the ownership instead: `freeze` the region (immutable forever) or
    make the value a `shared` cell (reference-counted, never dangles).
```

The checker speaks in allocation, escape, and free — the word
"lifetime" appears nowhere, because the region *is* the lifetime,
reified. lupin enforces the same rule at run time and blames the read
rather than the write: the value died with its region, and the fault
fires where the program finally reaches for it.

```console
$ lupin ex8-3.lu
ex8-3.lu: trap(region-fault): `keep.value` reaches into `tmp` (region #1), which was freed wholesale; the value died with the region [mem.region.intra.2] at 249..259; the region was created here at 190..241
$ echo $?
3
```

One rule, two moments: the compiler refuses the program before it
starts; the interpreter lets it run and faults the exact access that
needed the freed data. Which line each tool blames is the difference
between "this could dangle" and "this did."
</details>

<details>
<summary>Exercise 8-4 — [§8.3](../ch08.md#8.3)</summary>

**Exercise 8-4** *(fingers · lupin REPL)* — In the REPL: define a
one-field struct, create a region with `region(rc)`, allocate one
value into it with `in r { … }`, and look at `:regions` before and
after `freeze r`. Predict the two state words you will see before you
look.

Solution — one session:

```console
$ lupin
wolf> struct Howl { n: int }
defined type `Howl`
wolf> let r = region(rc)
wolf> let h = in r { Howl { n: 3 } }
wolf> :regions
regions:
  #0 `program` arena state=open objects=0
  #1 `-` rc state=suspended objects=1
wolf> let f = freeze r
wolf> :regions
regions:
  #0 `program` arena state=open objects=0
  #1 `-` rc state=frozen objects=1
wolf> h.n
3 : i32
wolf> :quit
```

The states are `suspended` and `frozen`. Creating a region does not
open it — the `in r { }` window did, briefly, and left it suspended
with one object inside. `freeze` is a state of the region, not a
property of any binding.
</details>

<details>
<summary>Exercise 8-5 — [§8.3](../ch08.md#8.3)</summary>

**Exercise 8-5** *(comprehension · lupin)* — A region is being sent
somewhere while a window into it is still open:

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    in r {
        var xs = List[int]()
        (mut xs).push(1)
        ch.send(move r)
        0
    }
}
```

Predict the event, its trap kind, and the static error code the trap
line will mention.

Solution: a `region-fault` — a region moves as a *closed* subtree, and
this one is open at the send:

```console
$ lupin ex8-5.lu
ex8-5.lu: trap(region-fault): region #1 is open here and cannot be transferred; a region moves as a closed subtree (the compiler's E1005) [mem.region.freeze.3] at 277..283
$ echo $?
3
```

The trap line names E1005 itself: the interpreter is enforcing at run
time the same rule the compiler will reject at compile time, and it
cites the compiler's number so the two tiers stay one story.
</details>

<details>
<summary>Exercise 8-6 — [§8.4](../ch08.md#8.4)</summary>

**Exercise 8-6** *(fingers · lupin)* — Build a five-node doubly-linked
ring in a pool region: each node points `next` and `prev`. Then prove
both directions work: walk five steps forward from the head (where do
you land?), and two steps backward. Rust folklore says this program
requires `unsafe` or `Rc<RefCell<…>>`; say in one sentence why wolf's
checker does not object here.

Solution — `ch08/ex8-6.lu` (core):

```wolf
struct Node { value: int, next: handle Node, prev: handle Node }
var pool = Pool[Node]()
var hs = List[handle Node]()
for _ in 0..5 { (mut hs).push((mut pool).reserve()) }
for i in 0..5 {
    (mut pool).init(hs[i], Node {
        value: (i + 1) * 10,
        next: hs[(i + 1) % 5],
        prev: hs[(i + 4) % 5],
    })
}
```

```console
$ lupin ex8-6.lu
10 40
```

Five steps forward from node 1 is node 1 again — it is a ring — and
two steps back lands on node 4. The checker does not object because
the cycle never crosses the region border: intra-region edges are
unrestricted, and the region dies as one unit, so no edge can dangle.
The two-phase `reserve`/`init` is what closed the cycle without a null
in sight.
</details>

<details>
<summary>Exercise 8-7 — [§8.4](../ch08.md#8.4)</summary>

**Exercise 8-7** *(extension · prose)* — Grow the ring into the real
folklore program: an LRU cache with sentinel head and tail, `unlink`
and `push_front` as the only two link operations, promotion on get, and
eviction of `tail.prev` at capacity. Trace it by hand: after put a, put
b, get a, put c at capacity 2, what does the front-to-back walk
print?

Solution (prose): `c a`. Insertion order is b-then-a reversed by
`push_front`; the get promotes `a` past `b`; the eviction takes
`tail.prev`, which the promotion made `b`; `c` lands in front. The
walk visits `head.next` to `tail`. Every link mutation in the program
goes through `unlink` or `push_front` — four pointer writes and five —
which is the entire aliasing surface a reviewer must read.
</details>

<details>
<summary>Exercise 8-8 — [§8.5](../ch08.md#8.5)</summary>

**Exercise 8-8** *(comprehension · wolf + lupin)* — A struct type with
a strong `shared` edge back to itself:

```wolf
struct Node { value: int, next: shared Node }
fn main() -> !int { 0 }
```

`main` builds nothing. Predict each tool's verdict anyway, then
explain the asymmetry: which tool is answering "could any program with
this type leak," and which is answering "did this program fault"?

Solution: wolf rejects the *type*; lupin runs the empty `main` to exit
0.

```console
$ wolf conform-run ./ex8-8.lu
error[E1006]: `Node` holds a strong `shared` path back to itself
 --> ./ex8-8.lu:4:27
  |
4 | struct Node { value: int, next: shared Node }
  |                           ^^^^ this `shared` edge closes the cycle Node → Node
  |
  = note: strong `shared` references drop their target when the last count drops, so a strong
    cycle would keep itself alive forever — and wolf has no cycle collector
    ([mem.shared.rc.2]). Break the back-edge: make this field `weak Node` (upgrade to reach
    the value without keeping it alive) or `handle Node` (a generational index that faults
    if the target is gone). If the structure is genuinely cyclic, keep the whole graph in
    one region instead — intra-region cycles are safe and freed wholesale
    ([mem.region.intra.1]).
```

```console
$ lupin ex8-8.lu
$ echo $?
0
```

This is conservatism, legitimately: the static tier rejects by *shape*
because some program with this type leaks, and refusing the type is
the only way to refuse all of those programs at once. The dynamic tier
saw no allocation, no cycle, no fault — also true. The note is the
chapter in miniature: `weak`, `handle`, or put the cycle in a region.
</details>

<details>
<summary>Exercise 8-9 — [§8.5](../ch08.md#8.5)</summary>

**Exercise 8-9** *(comprehension + spelunking · wolf)* — One write after a freeze:

```wolf
struct Config { limit: int }
fn main() -> !int {
    var cfg = freeze region { Config { limit: 42 } }
    cfg.limit = 7
    cfg.limit
}
```

Predict the verdict and, from `--explain`-level knowledge, the two
repairs the note will offer.

Solution:

```console
$ wolf conform-run ./ex8-9.lu
error[E1012]: `cfg.limit` is frozen, so it cannot be assigned through
 --> ./ex8-9.lu:7:5
  |
6 |     var cfg = freeze region { Config { limit: 42 } }
  |               -------------------------------------- the freeze happens here — the promotion to `imm` is deep and permanent
7 |     cfg.limit = 7
  |     ^^^^^^^^^ this needs the data to be mutable
  |
  = note: `freeze` promotes the whole graph to `imm`: shareable from anywhere, forever, and never
    writable again. Build the value completely before freezing it, or keep a mutable copy
    (`copy`) alongside the frozen one.
```

The repairs: finish building before freezing, or `copy` a mutable
twin. There is no third repair, because there is no unfreeze —
`freeze` is a cadence, not a lock. lupin reaches the same verdict from
the other side, trapping `region-fault` at `[mem.region.freeze.1]` —
the clause E1012 enforces statically:

```console
$ lupin ex8-9.lu
ex8-9.lu: trap(region-fault): region #1 is frozen: `imm` data is immutable forever [mem.region.freeze.1] at 195..208
$ echo $?
3
```
</details>

<details>
<summary>Exercise 8-10 — [§8.5](../ch08.md#8.5)</summary>

**Exercise 8-10** *(comprehension · lupin)* — The dynamic half of the
same contract: create a pool region, freeze the region value, then
call `reserve` on the pool. Predict the trap kind and the clause tag.

Solution — `ch08/ex8-10.lu`:

```wolf
struct Node { value: int }
fn main() -> !int {
    let r = region(pool(Node))
    let p = in r { Pool[Node]() }
    let frozen = freeze r
    let h = p.reserve()
    0
}
```

```console
$ lupin ex8-10.lu
ex8-10.lu: trap(region-fault): region #1 is frozen: `imm` data is immutable forever [mem.region.freeze.1] at 264..275
$ echo $?
3
```

`region-fault`, citing `[mem.region.freeze.1]` — the same clause the
E1012 diagnostic enforces statically in 8-9. `reserve` is a mutation
of the region's interior, and frozen means frozen all the way down.
</details>

<details>
<summary>Exercise 8-11 — [§8.6](../ch08.md#8.6)</summary>

**Exercise 8-11** *(comprehension · lupin)* — Two region values, two
nested `in` windows, reads and writes crossing both:

```wolf
let a = region()
let b = region()
in a {
    var xs = List[int]()
    (mut xs).push(1)
    in b {
        var ys = List[int]()
        (mut ys).push(2)
        total += xs[0] + ys[0]
    }
    (mut xs).push(3)
    total += xs[1]
}
```

Predict the printed total. Then the antichain question: of the shapes
(1) `in a { in b { } }`, (2) `in a { in a { } }`, (3)
`in a { } in a { }` — sequential reopen — which are legal? Answer from
the rule that region values are affine and windows must be into
*distinct* regions, then check the one the program demonstrates.

Solution: total is 6 (1 + 2, then 3). Shape 1 is legal — distinct
regions, both open, provably disjoint because region values are affine
(no alias of `a` can exist to sneak into the inner window). Shape 2 is
the one the rule forbids: the same region twice would make the two
windows alias. Shape 3 is legal — suspend, then reopen; the REPL
showed a region surviving between windows in 8-4.

```console
$ lupin ex8-11.lu
6
```
</details>

<details>
<summary>Exercise 8-12 — [§8.7](../ch08.md#8.7)</summary>

**Exercise 8-12** *(comprehension · lupin)* — A handle is used after
its slot is gone:

```wolf
region r: pool(Node) {
    var pool = Pool[Node]()
    let h = (mut pool).reserve()
    (mut pool).init(h, Node { value: 1 })
    (mut pool).remove(h)
    let v = pool[h].value
    v
}
```

Predict the trap kind, and — the part worth being precise about — what
the trap line will say about *generations*.

Solution:

```console
$ lupin ex8-12.lu
ex8-12.lu: trap(stale-handle): handle into pool#0 slot 0 carries generation 0, the slot is at generation 1; a stale handle is a deterministic fault in every profile, never UB [mem.shared.handle.2] at 348..355
$ echo $?
3
```

The handle remembers the generation it was issued at; `remove` bumped
the slot's generation; the mismatch is the fault. The sentence to
carry out of this exercise is the trap's own: deterministic, in every
profile, never UB — a stale handle in wolf is a *defined* event, which
is the entire difference between a handle and a C pointer into a
freed arena.
</details>

<details>
<summary>Exercise 8-13 — [§8.7](../ch08.md#8.7)</summary>

**Exercise 8-13** *(design)* — Four fields, one decision each: (a) a
parent pointer in a tree whose nodes a region owns; (b) an
edge in a social graph where nodes are deleted while neighbors hold
references; (c) a config blob read by every task for the process's
whole life; (d) a cache entry another subsystem may hold while the
cache evicts it. For each: `shared`, `weak`, `handle`, or a plain
intra-region edge — and name the *failure contract* you chose, not
only the shape.

Solution (discussion): (a) plain intra-region edge (or `handle` if
nodes are removed individually): the cycle is safe inside the region,
and the failure contract is "none — the region dies as one." (b)
`handle`: deletion must be observable, and the contract is "reads of a
dead neighbor fault, deterministically" — a `weak` would answer
"gone" quietly, which a graph traversal can misread as "no edge." (c)
`freeze` and share the frozen region: read-only forever wants the
no-synchronization contract, and freezing is how you buy it. (d)
`weak` inside `shared`: the cache wants eviction to *win* — the other
subsystem's read should answer "gone, reload" rather than fault or
keep the entry alive; a strong `shared` would silently defeat the
eviction policy. The pattern under all four: pick the reference type
by what you want *failure* to look like, because that is the part the
type system will hold you to.
</details>

<details>
<summary>Exercise 8-14 — [§8.8](../ch08.md#8.8)</summary>

**Exercise 8-14** *(spelunking · wolf)* — Run `wolf --explain E1012`
and read it against exercise 8-9. Find: the sentence that explains why
frozen data needs no locks, the phrase that makes the promotion
transitive, and the reason "readable forever" is a *performance*
claim, not only a safety one.

Solution:

```console
$ wolf --explain E1012
E1012: frozen data cannot be written

`freeze` consumes a region and promotes everything in it to `imm`:
deeply immutable, shareable from anywhere — across threads, without
synchronization — and readable forever. That deal is permanent, and
it is why frozen data needs no locks and no lifetimes; a single write
anywhere would break every reader everywhere. This write reaches data
that a `freeze` already promoted (the freeze site is marked). Do the
mutation before freezing — build the value completely, freeze last —
or keep a mutable `copy` alongside the frozen one.
```

No-locks: "a single write anywhere would break every reader
everywhere" — the absence of writers is what synchronization would
otherwise buy. Transitivity: "promotes *everything in it*" — the deal
is per-region, not per-binding. And "readable forever" is a
performance claim because a fact the compiler can rely on is a fact
the optimizer can spend: loads from *imm* data can be hoisted, merged,
and const-propagated across calls — the aliasing fact C cannot state
(§8.8's refrain), available here because the type system made it
unbreakable.
</details>

<details>
<summary>Exercise 8-15 — [§8.8](../ch08.md#8.8)</summary>

**Exercise 8-15** *(extension · lupin)* — A text adventure's world is
a cyclic graph: rooms point at each other in four directions, and
"north then south" must come home. Build three rooms — den, ridge,
river bank — in a pool region, close the cycles with two-phase init,
and walk the path north, east, west, south, printing the room at each
step. Predict the four lines before running; the fourth is the one
that checks you wired `south` self-loops honestly.

Solution — `ch08/ex8-15.lu` (walk shown; full wiring on disk):

```wolf
for step in path {
    if step == "north" { here = pool[here].north } else if step == "south" { here = pool[here].south } else if step == "east" { here = pool[here].east } else { here = pool[here].west }
    print("you are at the {pool[here].name}")
}
```

```console
$ lupin ex8-15.lu
you are at the ridge
you are at the river bank
you are at the den
you are at the den
```

The last two lines are both "den": west from the river bank comes
home, and the den's `south` is wired to itself — a self-loop is the
graph's honest spelling of "you cannot go that way," and it costs no
optional type. Room graphs are why the region chapter and the
adventure genre get along: the whole world dies in one motion when
the game ends, cycles and all.
</details>

<details>
<summary>Exercise 8-16 — [§8.8](../ch08.md#8.8)</summary>

**Exercise 8-16** *(extension · lupin)* — `wc`, wolfished: count the
lines and words of a multiline block, but store every line in a
scratch region while counting — then let the region die and print the
counts after it is gone. State what survives the brace and why this
program's memory use at peak is "the text, once" rather than "the
text, twice."

Solution — `ch08/ex8-16.lu`:

```wolf
fn main() -> !int {
    let text = """
        the wolf runs at dusk
        the moon rises
        the pack answers
        """
    var lines = 0
    var words = 0
    region scratch {
        var stored = List[str]()
        for line in text.lines() { (mut stored).push(line) }
        lines = stored.len
        for line in stored {
            for _ in line.words() { words += 1 }
        }
    }
    print("{lines} lines, {words} words")
    0
}
```

```console
$ lupin ex8-16.lu
3 lines, 11 words
```

The counts survive — plain `int`s in the frame. The stored lines do
not, and did not need to: line *views* into the original text are
two-word slices, so the region holds a list of views, not a second
copy of the text. Peak memory is the text plus a few dozen bytes of
list — and at the brace, even those go, wholesale. That is the region
answer to a question `wc` authors in C solve with careful `free`
bookkeeping: put the bookkeeping in the shape, then stop doing it.
</details>

## Chapter 9

<details>
<summary>Exercise 9-1 — [§9.1](../ch09.md#9.1)</summary>

**Exercise 9-1** *(fingers + spelunking · wolf)* — The complete unsafe audit
of every program in this book's first eight chapters is one command.
Run it from `principles/exercises/`, report the number, and state what
property of the language makes the count trustworthy — what would the
same grep miss in C?

Solution:

```console
$ grep -rn "unsafe {" ch01 ch02 ch03 ch04 ch05 ch06 ch07 ch08 | wc -l
0
```

Zero: parts 1 and 2 up to this chapter never left the safe tier. The
count is trustworthy because `unsafe` is the *only* entrance — raw
pointer operations, foreign calls, and aliasing assertions do not parse
outside the block, so the keyword is a complete index of the ring
boundary. In C the equivalent grep misses everything, because there is
no keyword: every pointer dereference in the program is potentially the
audit's subject, which is to say the audit is the whole program.
</details>

<details>
<summary>Exercise 9-2 — [§9.2](../ch09.md#9.2)</summary>

**Exercise 9-2** *(fingers · lupin)* — Your first unsafe block, kept
legal: allocate eight bytes from C, set them all to 5, read one back,
free, print. Type it, run it, and note the exit code — the point of
this exercise is that nothing happens.

Solution — `ch09/ex9-2.lu`:

```wolf
import c "stdlib.h"
fn main() -> !int {
    // # Safety: the eight bytes are written before they are read, every
    // access is in bounds, and the allocation is freed exactly once.
    unsafe {
        let p = c.malloc(8) as *u8
        c.memset(p, 5, 8)
        let v = p[3] as int
        c.free(p)
        print("{v}")
    }
    0
}
```

```console
$ lupin ex9-2.lu
5
$ echo $?
0
```

Inside the block, C's rules apply — allocate, use, free, in that
order, and the oracle has nothing to say. The block is a contract
change, not a crime scene. Note the `# Safety:` line above it: the
compiler warns when it is missing, and it is the sentence a reviewer
checks the body against.
</details>

<details>
<summary>Exercise 9-3 — [§9.2](../ch09.md#9.2)</summary>

**Exercise 9-3** *(comprehension · lupin)* — One character changes in
9-2: the write is `p[8] = 1`. The allocation holds eight bytes.
Predict the oracle's finding — its row, and which optimizer license
the report will name.

Solution — the report, in full:

```console
$ lupin ex9-3.lu
ex9-3.lu: ub(mem.ub) §7/P3: write of 1 byte(s) at alloc#0[8], which holds 8 [mem.ub] at 350..358; tag created at 297..308
  licenses O3a: `dereferenceable(n)` on known-size accesses; bounds-based alias disproof between distinct allocations
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

Row P3, out of bounds — and the license line is the half worth reading
twice: the *reason* one byte past the end is UB rather than a trap is
that the compiler wants to assume accesses stay inside their
allocation (`dereferenceable`, alias disproof between allocations).
Every oracle report has this shape: the fault, then the optimization
that the rule purchases. UB is not a punishment; it is a price list.
</details>

<details>
<summary>Exercise 9-4 — [§9.2](../ch09.md#9.2)</summary>

**Exercise 9-4** *(comprehension · lupin)* — The pointer is laundered
through an integer before the read:

```wolf
let p = c.malloc(8) as *u8
c.memset(p, 5, 8)
let address = p as int
c.free(p)
let q = address as *u8
let v = q[0] as int
```

An integer survives `free` untouched. Does the roundtrip save the
read? Predict the oracle's answer and its reasoning.

Solution: no. The integer survives; the *permission* does not.

```console
$ lupin ex9-4.lu
ex9-4.lu: ub(mem.ub) §7/L2: read through an exposed pointer into alloc#0, which was freed [mem.unsafe.raw.1] at 458..462; tag created at 317..328
  licenses O8: escape analysis / stack promotion without conservatively pinning addresses
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

The cast back from `int` reconnects the pointer to the allocation's
exposed tags — but every tag of a freed allocation is Disabled, so no
defined execution exists to choose. The license explains who benefits:
if an integer roundtrip could resurrect access, the compiler could
never promote an allocation to a register, because some integer
somewhere might name its address.
</details>

<details>
<summary>Exercise 9-5 — [§9.3](../ch09.md#9.3)</summary>

**Exercise 9-5** *(fingers + comprehension · lupin)* — Inject the classic:
write, free, read, through one pointer. Run it twice. What is the
oracle's finding, and — the actual question — what is identical
between the two runs that would *not* be identical for a
use-after-free in C?

Solution — `ch09/ex9-5.lu`:

```wolf
let p = c.malloc(8) as *u8
p[0] = 7
c.free(p)
let v = p[0]
```

```console
$ lupin ex9-5.lu
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 333..337; tag created at 263..274
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
$ lupin ex9-5.lu
ex9-5.lu: ub(mem.ub) §7/P1: read through tag#0 (c.malloc(8)#root), which is Disabled at alloc#0[0] [mem.prov.state] at 333..337; tag created at 263..274
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), FREED, owned by region #0
    tag#0 c.malloc(8)#root Disabled exposed
```

Byte-identical reports, twice — same fault, same spans, same tag
story. A C use-after-free reads whatever the allocator left there:
sometimes 7, sometimes a new object, sometimes a crash, varying with
allocator mood and moon phase. The oracle replaces "what happened to
be in memory" with "what the rules say about this access," and rules
do not vary between runs. Deterministic faults are what make the
escape hatch *debuggable* — this is §9.3's whole pitch, performed.
</details>

<details>
<summary>Exercise 9-6 — [§9.3](../ch09.md#9.3)</summary>

**Exercise 9-6** *(comprehension · lupin)* — Allocate 64 bytes with
`c.malloc` and read one of them before anything writes it:

```wolf
let p = c.malloc(64) as *u8
let v = p[0] as int
c.free(p)
```

No free-before-use, no bounds problem, no aliasing. Predict whether
this is undefined behavior, which row it lands on, and what
optimization the row's license names.

Solution: it is UB, and the row is L1 — reading memory nothing has
written is on the list in its own right:

```console
$ lupin ex9-6.lu
ex9-6.lu: ub(mem.ub) §7/L1: read of alloc#0[0], which nothing has written [mem.ub] at 306..310; tag created at 270..282
  licenses O7: moves lower to memcpy-and-forget; dead-store elimination on moved-from places; no zero-init of locals
  alloc#0 `c.malloc(64)` 64 byte(s), live, owned by region #0
    tag#0 c.malloc(64)#root Active exposed
$ echo $?
3
```

The license is the explanation, as always. If reading uninitialized
memory had a defined answer, every local would have to be zeroed, a
move could not be a copy-and-forget, and a store to a place that is
about to be moved out of could not be deleted. The price of skipping
all three is that the bytes `malloc` hands back are not a value yet.
`c.calloc` is the call that makes them one.
</details>

<details>
<summary>Exercise 9-7 — [§9.4](../ch09.md#9.4)</summary>

**Exercise 9-7** *(comprehension · lupin)* — Two programs differ by
one line's position. Both allocate eight C bytes, both cross back to
safe code through `borrow r from p` — the door. In the first, the
`malloc` happens inside `in r { }`; in the second, outside any window.
Predict each verdict before running either, and state the door's
obligation in one sentence.

Solution — `ch09/ex9-7a.lu` and `ch09/ex9-7b.lu`:

```console
$ lupin ex9-7a.lu
$ echo $?
0
$ lupin ex9-7b.lu
ex9-7b.lu: ub(mem.ub) §7/P6: `borrow region #1 from` a pointer into alloc#0, which is owned by `program` (region #0) — the obligation is that the allocation lies wholly inside the named region's footprint [mem.unsafe.door] at 395..410; tag created at 334..345
  licenses O6: safe-tier code after the door keeps all safe-tier entitlements (O1–O4) — the door is where trust concentrates
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

The obligation: the allocation must lie wholly inside the named
region's footprint. The first program allocated while `r` was ambient,
so the claim is true; the second claims bytes owned by the program
region. The license line is the chapter's thesis in one clause: after
a truthful door, safe code keeps *all* its entitlements — which is why
the door is one narrow place, and why lying to it is the worst lie in
the language.
</details>

<details>
<summary>Exercise 9-8 — [§9.5](../ch09.md#9.5)</summary>

**Exercise 9-8** *(fingers · wolf + lupin)* — Take §9.5's `pack` and
spell its second allocation the other way round: `c.calloc(1, bytes)`
instead of `c.calloc(bytes, 1)`. Both allocate the same number of
bytes. Run the program under the interpreter, then compile it and run
the binary. Report both outputs and say what a difference between them
would have meant.

Solution — `ch09/ex9-8.lu`, and there is no difference:

```console
$ lupin ex9-8.lu
64 bytes out, and not a pointer in sight
$ wolf build ex9-8.lu && ./ex9-8
64 bytes out, and not a pointer in sight
```

The first line is a model of the C heap; the second is glibc. A
difference between them would have meant one of two things, and both
are bugs: either a model of `calloc` disagrees with `calloc`, or the
compiler's membrane passes the arguments in the wrong order. The
history here is not hypothetical — `calloc(n, size)` is `n * size`
bytes, one of the models once made it `n`, and this is the shape of
program that found it. Running a program two ways and comparing is not
a testing technique bolted on afterward; it is what "one language, two
implementations" is *for*.
</details>

<details>
<summary>Exercise 9-9 — [§9.6](../ch09.md#9.6)</summary>

**Exercise 9-9** *(comprehension · lupin)* — A C allocation made while
a region was ambient, escaping the region that owned it:

```wolf
let p = region r {
    let inner = c.malloc(8) as *u8
    c.memset(inner, 5, 8)
    inner
}
let v = p[0] as int
```

The pointer is a plain integer-like value; it moves out fine. Predict
what the read faults with, and why the report will differ from 9-5's
use-after-free.

Solution:

```console
$ lupin ex9-9.lu
ex9-9.lu: ub(mem.ub) §7/P4: read at alloc#0[0], whose owning region #1 was freed wholesale [mem.prov.region] at 438..442; tag created at 341..352
  licenses O3b: one alias-scope domain per region — pointers into distinct regions never alias; O4: regions not open in the current scope yield `invariant.load`
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #1
    tag#0 c.malloc(8)#root Disabled exposed
$ echo $?
3
```

Row P4, not P1: nobody called `free` — the *region* died, and it took
the allocation's permissions with it. Note the report's strange-
looking line: the allocation is still `live` (its bytes were never
individually freed) but its owning region is gone, and that is enough.
The rule C code linked into wolf must learn: memory borrowed while a
region was ambient is a loan *from the region*, and the region's
death calls it in, wholesale.
</details>

<details>
<summary>Exercise 9-10 — [§9.7](../ch09.md#9.7)</summary>

**Exercise 9-10** *(spelunking · lupin)* — `ch09/ex9-10.lu` wraps its
unsafe block in a `#[trusted]` function carrying its obligation as a
string. Run it. Then answer from the chapter: what two questions about
this function does a manifest-and-inventory audit answer that reading
the function's source cannot?

Solution — the program runs like any other:

```console
$ lupin ex9-10.lu
64 bytes out, declared
$ echo $?
0
```

The two questions are *closure* and *drift*. Closure: reading `pack`
tells you what `pack` does, and tells you nothing about whether
anything it calls — or anything its dependencies call — is itself
trusted or holds unsafe rings of its own. The inventory answers for the
whole package, module by module, which is a question source review
answers one file at a time and therefore usually does not. Drift:
whether the roster is the same roster it was last release. Review is a
memory; the manifest is a diffable fact, and a dependency that grows a
trusted module has to grow a manifest line to do it. Neither question
is about whether `pack` is correct — nothing mechanical answers that,
which is exactly why the obligation is written in the attribute in
English, for a person.
</details>

<details>
<summary>Exercise 9-11 — [§9.8](../ch09.md#9.8)</summary>

**Exercise 9-11** *(comprehension · prose)* — Five fragments; place
each on the four-tier map (safe values, regions, unsafe raw,
the door):

1. `let b = copy a`
2. `pool[h].value`
3. `p[8] = 1` where `p: *u8`
4. `borrow r from p`
5. `ch.send(move r)` where `r` is a closed region

Solution: 1 is tier-0 safe values — chapter 7's world, no annotations.
2 is the region tier's shared edge — a handle read, checked by
generation, faulting stale rather than dangling. 3 is the raw tier —
legal only inside `unsafe`, governed by the oracle's price list. 4 is
the door itself — the one construct that moves data *up* a tier, with
its truth obligation. 5 is the region tier's transfer verb, safe
because closed-subtree moves preserve every invariant. The picture to
keep: four tiers, one direction of trust — every construct on this
list either stays in its tier or crosses at the door, and nothing else
crosses at all.
</details>

<details>
<summary>Exercise 9-12 — [§9.8](../ch09.md#9.8)</summary>

**Exercise 9-12** *(design)* — A team wraps a 40,000-line C codec
behind wolf FFI. Debate the two candidate shapes: (a) one `unsafe`
block per call site, spread through the application; (b) one module
owning every unsafe line, exporting twenty safe functions, `#[trusted]`
on the membrane. Which failure modes does each shape optimize for, and
what does the twenty-line rule from §9.5 actually buy the reviewer?

Solution (discussion): shape (a) optimizes for nothing except writing
speed; its failure mode is that the audit surface *is* the
application, and every new call site is a new review. Shape (b)
optimizes for the reviewer: the unsafe ring is one module, the grep
from 9-1 returns one path, and the twenty safe functions are the
complete list of claims the C code makes about itself. Its failure
mode is worth naming honestly — the membrane can become a lie if the
safe signatures promise more than the C delivers (a `str` that is not
UTF-8, a buffer length the codec ignores), and `#[trusted]` marks
exactly where that lie would live. The twenty-line rule buys
*proportionality*: a reviewer can hold twenty lines to the standard
"I believe each one," which is the standard unsafe code requires and
40,000 lines cannot meet. The door metaphor closes the argument:
doors work because buildings have few of them.
</details>

<details>
<summary>Exercise 9-13 — [§9.8](../ch09.md#9.8)</summary>

**Exercise 9-13** *(extension (break-it-on-purpose) · lupin)* — Construct the
shortest program you can in which the *assertion*, not any access, is
the undefined behavior: use `assume noalias` on two pointers that
alias. Predict the oracle's wording — what does it say overlaps what?

Solution — `ch09/ex9-13.lu`:

```wolf
let p = c.malloc(8) as *u8
let q = p
assume noalias p, q
p[0] = 1
q[0] = 2
```

```console
$ lupin ex9-13.lu
ex9-13.lu: ub(mem.ub) §7/P5: `assume noalias` asserts these ranges are disjoint, and alloc#0[0..1) overlaps alloc#0[0..1) — the assertion is false [mem.unsafe.raw.2] at 337..356; tag created at 292..303
  licenses O5: the asserted ranges get `noalias` treatment in Tier-3 code — vectorization/reordering as if proven
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #0
    tag#0 c.malloc(8)#root Active exposed
$ echo $?
3
```

The range overlaps *itself* — `alloc#0[0..1)` against `alloc#0[0..1)`
— because `q` is `p`. This is the only assertion-created UB in the
raw tier: everything else about `*T` is unrestricted, but a spoken
aliasing promise is kept or it is UB the moment the accesses disagree
with it. It is also the exercise to remember when C's `restrict`
comes up: wolf did not remove the footgun, it made the trigger
visible and gave it an oracle.
</details>

<details>
<summary>Exercise 9-14 — [§9.8](../ch09.md#9.8)</summary>

**Exercise 9-14** *(comprehension · lupin)* — The subtlest report in
the chapter. Take §9.4's door program and add one line: after
`let counts = borrow scratch from p`, write `p[0] = 1` through the raw
pointer, and only then read `counts[0]`. Predict where the fault is
reported and what the tag tree at the bottom of the report will have
in it that no other report in this chapter has shown.

Solution — `ch09/ex9-14.lu`:

```console
$ lupin ex9-14.lu
ex9-14.lu: ub(mem.ub) §7/P1: read through tag#1 (`borrow … from`), which is Disabled at alloc#0[0] [mem.prov.state] at 478..487; tag created at 423..444
  licenses O1: `mut` params lower to `noalias` + `dereferenceable`; unique-tag stores forward without memory checks
  alloc#0 `c.malloc(8)` 8 byte(s), live, owned by region #1
    tag#0 c.malloc(8)#root Active exposed
      tag#1 `borrow … from` Disabled|Reserved|Reserved|Reserved|Reserved|Reserved|Reserved|Reserved
$ echo $?
3
```

Two tags, one indented under the other: this is the first report in
the chapter with a tree in it rather than a single root. The door
minted `tag#1` as a child of the allocation's root tag, and that child
is what safe code was handed. Writing through `p` afterward is a write
through a tag that is *not* an ancestor of `tag#1` — foreign, in the
model's word — so `tag#1` goes Disabled at the byte that was written,
and the read through `counts` is P1. The per-byte spelling is the
detail worth noticing: only byte 0 was written, so only byte 0's
permission died, and the other seven are still Reserved. Permissions
are per location, not per pointer.

The general lesson is the reason the door exists. Once safe code holds
a value, the raw tier must stop touching those bytes — because
everything downstream is entitled to assume nobody is. Reaching around
the door is not a slightly risky shortcut; it is the one thing the
door's license forbids.
</details>

## Chapter 10

<details>
<summary>Exercise 10-1 — [§10.1](../ch10.md#10.1)</summary>

**Exercise 10-1** *(fingers · lupin)* — Type and run your first scope:
two children each send a number into a channel, and `main` adds what it
receives after the scope closes. Then swap the two `spawn` lines and
run again. What changed?

Solution — `ch10/ex10-1.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](2)
    var total = 0
    scope s {
        s.spawn(fn() { ch.send(20) })
        s.spawn(fn() { ch.send(22) })
    }
    total += ch.recv() else |_| { return 1 }
    total += ch.recv() else |_| { return 1 }
    print("total={total}")
    0
}
```

```console
$ lupin ex10-1.lu
total=42
```

Swapping the spawns changes nothing observable: addition commutes, and
the scope's exit joins both children before the first `recv` runs, so
both values are already in the buffer either way.
</details>

<details>
<summary>Exercise 10-2 — [§10.1](../ch10.md#10.1)</summary>

**Exercise 10-2** *(comprehension · lupin)* — Predict the order of the
two lines, then say what enforces it — the scheduler, or something
stronger:

```wolf
fn main() -> !int {
    scope s {
        s.spawn(fn() { print("child speaks") })
    }
    print("main speaks")
    0
}
```

Solution: `child speaks` first, always. The scope's closing brace joins
every child; `main speaks` sits after the brace, so it cannot run until
the child has finished — under any seed. This is structure, not luck:

```console
$ lupin ex10-2.lu
child speaks
main speaks
$ lupin run ex10-2.lu --seed=2024
child speaks
main speaks
```

The wrong answer worth ruling out: "the child happened to be scheduled
first." Move the `print` *inside* the scope and the order genuinely is
the scheduler's to choose; after the brace, it is not.
</details>

<details>
<summary>Exercise 10-3 — [§10.1](../ch10.md#10.1)</summary>

**Exercise 10-3** *(comprehension · lupin)* — A child's last expression
is a value. Predict this program's exit code, and account for the 42:

```wolf
fn main() -> !int {
    scope s {
        s.spawn(fn() { 42 })
    }
    7
}
```

Solution: exit 7. The child's value is discarded at the join — a scope
joins its children for their *completion*, not their results. A child
that has something to say sends it on a channel; the 42 evaporates.

```console
$ lupin ex10-3.lu
$ echo $?
7
```
</details>

<details>
<summary>Exercise 10-4 — [§10.2](../ch10.md#10.2)</summary>

**Exercise 10-4** *(extension (break-it-on-purpose) · lupin)* — Port Go's classic
leak: spawn a receiver on a channel that nobody will ever send to. In
Go the goroutine outlives the function, silently, forever. Write the
wolf version and predict what happens instead — and at which line.

Solution — `ch10/ex10-4.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](0)
    scope s {
        s.spawn(fn() {
            let v = ch.recv() else |_| { return 0 }
            print("{v}")
        })
    }
    0
}
```

```console
$ lupin ex10-4.lu
ex10-4.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@196` (task 1) [conc.deadlock.trap] at 178..304
$ echo $?
3
```

The scope's closing brace must join the child; the child is blocked in
`recv`; nothing can unblock it. Where Go leaks quietly, wolf's
structure turns the same mistake into a deadlock the runtime can see —
every live task blocked, so the trap fires and names them. The leak is
not fixed; it is *retired*: this program cannot express "and the task
lingers on unowned."
</details>

<details>
<summary>Exercise 10-5 — [§10.2](../ch10.md#10.2)</summary>

**Exercise 10-5** *(spelunking · lupin)* — Read exercise 10-4's trap
line clause by clause. What does "no timer is pending" rule out, what
is the "blocked-task roster" for, and why does the trap name `main`
itself as blocked?

Solution (prose): the trap's condition is *every* live task blocked
with nothing left that could wake one. A pending timer — a `timeout`
arm in some `select` — would eventually fire and unblock somebody, so
its absence is part of the proof; the roster is the evidence, one entry
per blocked task with its id, which is the list you would otherwise
assemble by hand from a hung process's stacks. `main` is on the roster
because the join at the scope's brace is itself a blocking point:
`main` is not running the children, it is waiting for them, and waiting
tasks are exactly what a deadlock is made of. The clause tag
`[conc.deadlock.trap]` is the spec's name for the whole rule.
</details>

<details>
<summary>Exercise 10-6 — [§10.3](../ch10.md#10.3)</summary>

**Exercise 10-6** *(comprehension · lupin)* — Three children compute
through `?`; one of them fails. Predict both printed lines, and name
the exact point in the source where the error crosses from child to
parent:

```wolf
fn risky(n: int) -> int ! {Torn} {
    if n == 3 { return Torn }
    n * 100
}
fn gather() -> !int {
    let ch = channel[int](4)
    scope s {
        s.spawn(fn() { ch.send(risky(1)?) })
        s.spawn(fn() { ch.send(risky(3)?) })
        s.spawn(fn() { ch.send(risky(2)?) })
    }
    0
}
fn main() -> !int {
    let r = gather() else |_| { print("join surfaced the error"); 7 }
    print("{r}")
    0
}
```

Solution: `join surfaced the error`, then `7`. The failing child's `?`
raises `Torn` inside the task; the error travels to the scope's closing
brace — the join — and re-raises there, into `gather`'s own error row,
where `main`'s `else` handles it. The crossing point is the brace. In
Go this error dies in a goroutine unless you built machinery to carry
it; here the structure is the machinery.

```console
$ lupin ex10-6.lu
join surfaced the error
7
```
</details>

<details>
<summary>Exercise 10-7 — [§10.3](../ch10.md#10.3)</summary>

**Exercise 10-7** *(extension · lupin)* — Change 10-6 so no child
fails (use 1, 2, and 4), then finish the job: close the channel, drain
it, and return the sum. Why is it correct to `close` only after the
scope's closing brace — what has the join already proved by then?

Solution — `ch10/ex10-7.lu`:

```wolf
fn gather_all() -> !int {
    let ch = channel[int](4)
    scope s {
        s.spawn(fn() { ch.send(risky(1)?) })
        s.spawn(fn() { ch.send(risky(2)?) })
        s.spawn(fn() { ch.send(risky(4)?) })
    }
    ch.close()
    var total = 0
    for v in ch { total += v }
    total
}
```

```console
$ lupin ex10-7.lu
total=700
```

After the brace, every child has completed, so every send that will
ever happen has happened — `close` cannot cut anyone off. The join
converts "I hope they are done" into a fact you may compute with.
</details>

<details>
<summary>Exercise 10-8 — [§10.4](../ch10.md#10.4)</summary>

**Exercise 10-8** *(comprehension · lupin)* — One sibling blocks
forever; the other fails immediately. Predict all the output, and
answer the pointed part first: does the blocked sibling's `defer` run?

```wolf
fn fail_fast() -> !int { Boom }
fn race_them() -> !int {
    let ch = channel[int](0)
    scope s {
        s.spawn(fn() {
            defer print("sibling cleanup ran")
            let v = ch.recv()?
            v
        })
        s.spawn(fn() { fail_fast() })
    }
    0
}
fn main() -> !int {
    let r = race_them() else |_| { 42 }
    print("{r}")
    0
}
```

Solution: yes — `sibling cleanup ran`, then `42`. The failing child
makes the scope cancel its blocked sibling; cancellation lands at the
sibling's blocking point (`recv`), and the task unwinds *its own*
defers on the way out. Cancellation is polite. Chapter 14 shows the
impolite variant — `kill` on a proc skips defers by design — and the
difference between those two rules is a decided thing, not an
accident.

```console
$ lupin ex10-8.lu
sibling cleanup ran
42
```
</details>

<details>
<summary>Exercise 10-9 — [§10.4](../ch10.md#10.4)</summary>

**Exercise 10-9** *(extension · lupin)* — Build a two-stage pipeline:
a producer sends 1 through 5 into `raw`; a transformer squares each
into `squared`; `main` counts what arrives. Each stage closes the
channel it sends on, when its input runs dry. Run it under two seeds.
Then answer: which task must close `squared`, and what goes wrong if
`main` tries to?

Solution — `ch10/ex10-9.lu`:

```wolf
fn main() -> !int {
    let raw = channel[int](8)
    let squared = channel[int](8)
    var produced = 0
    scope s {
        s.spawn(fn() {
            for i in 1..=5 { raw.send(i) }
            raw.close()
        })
        s.spawn(fn() {
            for v in raw { squared.send(v * v) }
            squared.close()
        })
        for v in squared { produced += 1 }
    }
    print("{produced} values through the pipeline")
    0
}
```

```console
$ lupin ex10-9.lu
5 values through the pipeline
$ lupin run ex10-9.lu --seed=2024
5 values through the pipeline
```

Only the transformer knows when the last square has been sent — it
learns it from its own `for` loop ending, which happens when `raw`
closes and drains. If `main` closed `squared`, it would be guessing;
close is the sender's verb, and each stage owns exactly one sending
side. That ownership discipline is the whole pipeline pattern.
</details>

<details>
<summary>Exercise 10-10 — [§10.4](../ch10.md#10.4)</summary>

**Exercise 10-10** *(design)* — Go has `go f()`; wolf deliberately has
no detached spawn — a task needs a scope, and the scope must close.
Take the other side seriously: name a real program shape that detached
spawn serves well, sketch how wolf expresses it, and state what the
wolf version pays and what it collects.

Solution (discussion): the honest case for detachment is the
fire-and-forget notifier — a metrics ping, a log ship — where the
caller genuinely does not want to wait and failure is acceptable. Go
spells it in three characters. Wolf makes the lifetime explicit: the
ping lives in some scope — a long-lived one owned by the subsystem
that cares about pings, with the pattern chapter 11 builds. The
payment is real: you must decide *whose* scope, which is a design
question Go let you skip. What it collects: the answer to "can this
program exit with work still running" is knowable by reading the
scopes, every error has an owner, and the leak of exercise 10-4 is
unwritable. Wolf's position is that "whose is this task" was never
optional — Go defers the question to a runtime that cannot answer it,
and wolf asks it at the point where you still can.
</details>

## Chapter 11

<details>
<summary>Exercise 11-1 — [§11.1](../ch11.md#11.1)</summary>

**Exercise 11-1** *(fingers · lupin)* — A function cannot spawn unless
somebody hands it a scope. Write `launch(s, ch, n)` that spawns into a
caller's scope, and a `main` that calls it three times inside one
`scope` block. The `Scope` parameter is the entire mechanism — nothing
else in the signature says "concurrent."

Solution — `ch11/ex11-1.lu`:

```wolf
fn launch(s: Scope, ch: channel[int], n: int) {
    s.spawn(fn() { ch.send(n * 10) })
}
fn main() -> !int {
    let ch = channel[int](3)
    var total = 0
    scope s {
        launch(s, ch, 1)
        launch(s, ch, 2)
        launch(s, ch, 3)
    }
    for _ in 0..3 { total += ch.recv() else |_| { return 1 } }
    print("{total}")
    0
}
```

```console
$ lupin ex11-1.lu
60
```
</details>

<details>
<summary>Exercise 11-2 — [§11.1](../ch11.md#11.1)</summary>

**Exercise 11-2** *(comprehension · lupin)* — Take 11-1 and change one
character: make the channel a rendezvous, `channel[int](0)`. Predict
precisely what happens and why — the answer involves which side of the
scope's closing brace the receives sit on.

Solution: deadlock. With no buffer, each child's `send` blocks until
someone receives; the receives are after the scope's brace; the brace
is a join that waits for the children. Children wait for `main`, `main`
waits for children, and the trap names all four:

```console
$ lupin ex11-2.lu
ex11-2.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@219` (task 1), `task@219` (task 2), `task@219` (task 3) [conc.deadlock.trap] at 326..416
$ echo $?
3
```

The buffered version worked because capacity 3 let every send complete
without a receiver. Buffer size is not a tuning knob here; it is part
of the program's correctness argument.
</details>

<details>
<summary>Exercise 11-3 — [§11.1](../ch11.md#11.1)</summary>

**Exercise 11-3** *(comprehension · lupin)* — Using only the text of
11-1's program, answer: which functions in it are able to spawn tasks,
and what single search over a large codebase would find every function
with that ability? (Chapter 7 asked the same question about mutation.)

Solution (prose): `main` can spawn (it owns a `scope` block) and
`launch` can spawn (it receives a `Scope`). Nothing else can. The
search is for `Scope` in parameter lists plus `scope` blocks — the
spawn surface is exactly the set of functions the type system shows
holding the capability, the same audit `grep '(mut '` performs for
mutation. A capability you can grep for is a capability you can
review.
</details>

<details>
<summary>Exercise 11-4 — [§11.2](../ch11.md#11.2)</summary>

**Exercise 11-4** *(extension · lupin)* — Build a worker pool: three
workers share one `jobs` channel and one `results` channel; `main`
feeds six jobs and closes. Each worker is the same four lines. Why does
the pool need no "shut down workers" message?

Solution — `ch11/ex11-4.lu`:

```wolf
fn main() -> !int {
    let jobs = channel[int](8)
    let results = channel[int](8)
    scope s {
        for w in 0..3 {
            s.spawn(fn() {
                for j in jobs { results.send(j * j) }
            })
        }
        for j in 1..=6 { jobs.send(j) }
        jobs.close()
    }
    results.close()
    var total = 0
    for r in results { total += r }
    print("total={total}")
    0
}
```

```console
$ lupin ex11-4.lu
total=91
```

A worker's loop ends when `jobs` closes and drains — the close *is*
the shutdown message, broadcast to every receiver at once. The scope's
brace then proves all workers are gone before `results` is touched.
Two channel closes and one join replace the ad-hoc "poison pill"
protocols other ecosystems teach.
</details>

<details>
<summary>Exercise 11-5 — [§11.2](../ch11.md#11.2)</summary>

**Exercise 11-5** *(comprehension + schedule play · lupin)* — Shrink
the pool to two workers and four jobs, and tag each result with the
worker that produced it. Before running: is the *assignment* of jobs
to workers part of the program, or part of the schedule? Run under
seed 1 and seed 2024 and defend your answer with the outputs.

Solution — `ch11/ex11-5.lu` (excerpt):

```wolf
scope s {
    for w in 0..2 {
        s.spawn(fn() {
            for j in jobs { results.send("worker {w} took job {j}") }
        })
    }
    for j in 1..=4 { jobs.send(j) }
    jobs.close()
}
```

```console
$ lupin run ex11-5.lu --seed=1
worker 1 took job 1
worker 1 took job 2
worker 1 took job 3
worker 1 took job 4
$ lupin run ex11-5.lu --seed=2024
worker 0 took job 1
worker 0 took job 2
worker 0 took job 3
worker 0 took job 4
```

The assignment is the schedule's: seed 1 lets worker 1 drain the whole
queue, seed 2024 hands it to worker 0, and both are conforming runs of
the same program. What the program owns is the *set* of results — four
squares would be identical in every schedule, which is exercise 11-4's
sum. Write programs whose meaning lives in what is computed, not in
who computed it; the seeds exist to catch you when you have not.
</details>

<details>
<summary>Exercise 11-6 — [§11.3](../ch11.md#11.3)</summary>

**Exercise 11-6** *(spelunking · lupin REPL)* — Turn on the trace and
run a scope with two children, then read the scheduler's own account:

```console
$ lupin
wolf> :trace on
trace on: every rule firing is recorded with its clause anchor
wolf> scope s { s.spawn(fn() { print("a") }); s.spawn(fn() { print("b") }) }
a
b
wolf> :trace
     0..0      SchedSpawn [conc.task.spawn] ev#3 spawn `task@63` (task 2) under scope#0 in proc#0
    40..68     Assign [gram.expr.assign] write `s`
    40..68     ProvState [mem.prov.state] write alloc#0[0..1) through tag#2 — tree consistent
    40..68     ProvTag [mem.prov.tag] protector on tag#2 (`t0:0:s`) released: the call's extent ended
     8..70     Block [gram.expr.block] block yields its tail
    31..34     StrInterp [str.interp] f-string
    31..34     EvalStrictOrder [mem.model.order] 1 argument(s) evaluated left to right
    23..37     Block [gram.expr.block] block yields its tail
     0..0      TaskJoin [conc.task.join] ev#4 `main` blocks at scope#0's exit join
     0..0      SchedPark [conc.det.events] ev#5 park `main` (task 0)
     0..0      SchedDecision [conc.det.events] ev#6 schedule `task@33` (task 1), picked 0 of 2 ready
     0..0      TaskName [conc.task.name] ev#7 task `task@33` (task 1) completes: ok
    61..64     StrInterp [str.interp] f-string
    61..64     EvalStrictOrder [mem.model.order] 1 argument(s) evaluated left to right
    53..67     Block [gram.expr.block] block yields its tail
     0..0      SchedDecision [conc.det.events] ev#8 schedule `task@63` (task 2), picked 0 of 1 ready
     0..0      TaskName [conc.task.name] ev#9 task `task@63` (task 2) completes: ok
     0..0      SchedUnpark [conc.det.events] ev#10 unpark `main` (task 0)
     0..0      SchedDecision [conc.det.events] ev#11 schedule `main` (task 0), picked 0 of 1 ready
     0..0      TaskJoin [conc.task.join] ev#12 scope#0 joins: all 2 child(ren) complete
wolf> :quit
```

From the trace alone, reconstruct the task tree — which tasks exist,
who owns them, and in what order they completed.

Solution (prose): three tasks. `main` is task 0; `task@33` (task 1)
and `task@63` (task 2) were spawned under `scope#0`, which `main`
owns, inside `proc#0`. The completion order is written in the
`TaskName` events: task 1 at ev#7, task 2 at ev#9, and then `main`
unparks and the scope joins at ev#12 with "all 2 child(ren) complete."
The dump is not a stack sample; it is the ownership tree the language
defined, reported by the machine that enforced it.
</details>

<details>
<summary>Exercise 11-7 — [§11.3](../ch11.md#11.3)</summary>

**Exercise 11-7** *(comprehension · lupin REPL)* — In 11-6's trace,
find every `SchedDecision` line and read its "picked 0 of N ready"
suffix. At which event did the scheduler actually have a choice, and
what does that tell you about how many *different* traces this
one-line program could produce?

Solution (prose): only ev#6 offered a choice — "picked 0 of 2 ready,"
with both children runnable. ev#8 and ev#11 each had one ready task,
which is no decision at all. One binary choice, so two inequivalent
schedules exist: task 1 first or task 2 first — precisely the two
outputs `a b` and `b a`. Counting the "of N ready" suffixes is a hand
computation of what chapter 17's `--explore` computes for real
programs, and it is worth doing once by eye to believe the tool.
</details>

<details>
<summary>Exercise 11-8 — [§11.3](../ch11.md#11.3)</summary>

**Exercise 11-8** *(design)* — A library offers
`fetch_all(urls: List[str]) -> List[Response]` and wants to fetch
concurrently. Two candidate signatures:

```wolf
fn fetch_all(urls: List[str]) -> List[Response]
fn fetch_all(s: Scope, urls: List[str]) -> List[Response]
```

The first hides an internal scope; the second borrows the caller's.
Argue for each: who controls cancellation and lifetime in each design,
and which caller is each one honest to?

Solution (discussion): the internal-scope version is honest to the
caller who wants a blocking call: when it returns, no task it started
survives — the function is externally sequential, concurrency as an
implementation detail, nothing to cancel from outside because nothing
outlives the call. The `Scope` parameter is honest to the caller who
wants to *compose* lifetimes: the fetches join when the caller's scope
closes, so the caller can hang ten calls on one scope and cancel the
lot by leaving it — but the signature now admits that tasks may
outlive the call itself, and every reader of the call site must look
up to find the brace those tasks die at. The library rule of thumb
wolf's std follows: take a `Scope` when the work's lifetime is
legitimately the caller's decision; keep the scope internal when the
function's contract is "done means done." The wrong design is the
secret third one — an internal scope that detaches work past its own
return, which is the chapter 10 leak wearing a signature.
</details>

## Chapter 12

<details>
<summary>Exercise 12-1 — [§12.1](../ch12.md#12.1)</summary>

**Exercise 12-1** *(fingers · lupin)* — A producer sends four squares
and closes; `main` drains with a `for` loop. Type it, run it, then
delete the `ch.close()` line and predict what the second run does
before you try it.

Solution — `ch12/ex12-1.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](4)
    var total = 0
    scope s {
        s.spawn(fn() {
            for i in 1..=4 { ch.send(i * i) }
            ch.close()
        })
        for v in ch { total += v }
    }
    print("total={total}")
    0
}
```

```console
$ lupin ex12-1.lu
total=30
```

Without the close, `main`'s `for` waits forever for a fifth value, the
producer is already gone, and the deadlock trap fires. A `for` over a
channel is a loop whose termination condition is *someone else's*
promise — close is how that promise is kept.
</details>

<details>
<summary>Exercise 12-2 — [§12.1](../ch12.md#12.1)</summary>

**Exercise 12-2** *(extension (break-it-on-purpose) · lupin)* — Using one task and
one channel of capacity 1, write the shortest program you can whose
second statement never finishes. Predict the trap kind and the roster
before running.

Solution — `ch12/ex12-2.lu`:

```wolf
fn main() -> !int {
    let ch = channel[int](1)
    ch.send(1)
    ch.send(2)
    print("never printed")
    0
}
```

```console
$ lupin ex12-2.lu
ex12-2.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0) [conc.deadlock.trap] at 211..221
$ echo $?
3
```

The buffer holds one value; the second `send` blocks until a receive
makes room, and the only task that could receive is the one blocked
sending. A deadlock does not need two tasks — it needs a cycle of
waiting, and `main` alone closes a cycle of length one.
</details>

<details>
<summary>Exercise 12-3 — [§12.2](../ch12.md#12.2)</summary>

**Exercise 12-3** *(comprehension · lupin)* — Two identical `select`s;
between them, one `send`. Predict both printed lines:

```wolf
fn main() -> !int {
    let a = channel[int](1)
    select {
        v from a => { print("got {v}") },
        timeout(5.ms) => { print("timed out") },
    }
    a.send(9)
    select {
        v from a => { print("got {v}") },
        timeout(5.ms) => { print("timed out") },
    }
    0
}
```

Solution: `timed out`, then `got 9`. A timeout arm is not a delay; it
is the arm that wins when no other arm can. The first select has an
empty channel and nothing pending, so the timer is the only way out;
the second finds a value ready and the timer never enters into it.

```console
$ lupin ex12-3.lu
timed out
got 9
```
</details>

<details>
<summary>Exercise 12-4 — [§12.2](../ch12.md#12.2)</summary>

**Exercise 12-4** *(comprehension + schedule play · lupin)* — Both
channels are ready before the `select` runs. Write down every output
this program is *allowed* to print, then run it under seed 1 and seed
2024:

```wolf
fn main() -> !int {
    let a = channel[int](1)
    let b = channel[int](1)
    a.send(1)
    b.send(2)
    var got = 0
    select {
        v from a => { got = v },
        v from b => { got = v },
    }
    print("{got}")
    0
}
```

Solution: `1` and `2` are both conforming — two simultaneously ready
arms make the pick a recorded scheduler decision, drawn from the seed.

```console
$ lupin run ex12-4.lu --seed=1
2
$ lupin run ex12-4.lu --seed=2024
1
```

Each seed replays byte-identically, forever. "Nondeterministic" in
wolf means the *spec* admits more than one outcome; any single seeded
run is as reproducible as arithmetic.
</details>

<details>
<summary>Exercise 12-5 — [§12.2](../ch12.md#12.2)</summary>

**Exercise 12-5** *(spelunking · lupin)* — Run the explorer over 12-4
and read its report line by line:

```console
$ lupin conform-run ex12-4.lu --explore=8
ex12-4.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×1 stdout=1\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0
    exit(0) ×1 stdout=2\n leaks=0 forest=ok — replay: --seed=4611686018427387905
      decision stream: ev:1
  deadlocks: 0 · races: 0 · max depth: 1 decision(s)
$ echo $?
1
```

Why "2 schedule(s)" and not eight? What is a `decision stream`, and
why does the tool exit 1 when nothing failed?

Solution (prose): the budget of 8 is a ceiling, not a quota — the
program contains exactly one decision with two choices, so the
frontier closes after two schedules and `max depth: 1 decision(s)`
says so. The decision stream (`ev:0`, `ev:1`) is the schedule written
out as the sequence of choices taken; either replays exactly, and
each outcome also carries a packed `--seed` spelling of the same
stream. Exit 1 is the differential protocol's honesty: a
schedule-dependent program is a *finding* — something a test suite
should know about — even when every outcome is individually fine.
Deterministic-under-every-schedule is the verdict that exits 0, and
exercise 12-8 earns it.
</details>

<details>
<summary>Exercise 12-6 — [§12.3](../ch12.md#12.3)</summary>

**Exercise 12-6** *(extension · lupin)* — Build a router: one task
reads an inbox and forwards each value to an `evens` or `odds` sink.
`main` feeds 1 through 8 and then sums both sinks. Mind the closes:
who closes what, in what order?

Solution — `ch12/ex12-6.lu`:

```wolf
fn main() -> !int {
    let src = channel[int](8)
    let evens = channel[int](8)
    let odds = channel[int](8)
    scope s {
        s.spawn(fn() {
            for v in src {
                if v % 2 == 0 { evens.send(v) } else { odds.send(v) }
            }
            evens.close()
            odds.close()
        })
        for i in 1..=8 { src.send(i) }
        src.close()
    }
    var esum = 0
    var osum = 0
    for v in evens { esum += v }
    for v in odds { osum += v }
    print("evens {esum}, odds {osum}")
    0
}
```

```console
$ lupin ex12-6.lu
evens 20, odds 16
```

Close flows downstream: `main` closes `src` when the feed ends; the
router's loop ends because of that close, and only then does the
router close its two output channels. Each channel is closed by its
only sender, and the close order is forced by the data flow — trace it
backward from the sums and every close is where it must be.
</details>

<details>
<summary>Exercise 12-7 — [§12.3](../ch12.md#12.3)</summary>

**Exercise 12-7** *(design)* — A single task maintains a work list it
alone pushes to and pops from. Argue why a `channel` is the wrong type
for that list even though it would work, and name the two properties a
channel charges for that this task does not use. When does the answer
flip?

Solution (discussion): a channel buys synchronization (safe handoff
between tasks) and blocking (a receiver waits for a sender). A
single-task work list uses neither: nothing is handed off and waiting
on yourself is exercise 12-2's one-task deadlock wearing work clothes.
`List` push/pop states the actual invariant — one owner, no
concurrency — and the type system holds you to it; a channel would
advertise a concurrency that does not exist to every future reader.
The answer flips at the moment a second task appears: the day the work
list is fed by a producer or drained by a pool, the channel's two
costs become exactly the two features you need, and the refactor is
chapter 11's worker pool. Types are claims; make the cheapest claim
that is true.
</details>

<details>
<summary>Exercise 12-8 — [§12.4](../ch12.md#12.4)</summary>

**Exercise 12-8** *(comprehension · lupin)* — Two tasks acquire the
same two mutexes in *opposite* spellings. Predict the total, and
predict what the explorer says about this program — then check both:

```wolf
fn main() -> !int {
    let a = Mutex(1)
    let b = Mutex(2)
    var total = 0
    scope s {
        s.spawn(fn() {
            when (a, b) { a += 10; b += 10 }
        })
        s.spawn(fn() {
            when (b, a) { b += 100; a += 100 }
        })
    }
    when (a, b) { total = a + b }
    print("{total}")
    0
}
```

Solution: 223 — both bodies run whole, in some order, on both
mutexes: 1+2 plus 110 plus 110. `when (b, a)` and `when (a, b)`
perform identical acquisitions because `when` sorts its set into
canonical order before taking anything; the spelling order is
documentation, not semantics. The explorer confirms there is nothing
to find:

```console
$ lupin ex12-8.lu
223
$ lupin conform-run ex12-8.lu --explore=16
ex12-8.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 1 distinct — observably deterministic (every schedule agrees)
    exit(0) ×2 stdout=223\n leaks=0 forest=ok — replay: --seed=0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
```

"Observably deterministic (every schedule agrees)" is the verdict
exercise 12-5's program could not earn. Addition commutes; that is
doing part of the work here, and the stem's real lesson is in 12-9.
</details>

<details>
<summary>Exercise 12-9 — [§12.4](../ch12.md#12.4)</summary>

**Exercise 12-9** *(extension (break-it-on-purpose) · lupin)* — Now construct the
classic deadlock `when` was designed to kill: task one takes `a` then
`b`, task two takes `b` then `a`, nested. Write it and report what
actually happens — at what phase does this program die?

Solution — `ch12/ex12-9.lu`:

```wolf
fn main() -> !int {
    let a = Mutex(0)
    let b = Mutex(0)
    scope s {
        s.spawn(fn() { when (a) { when (b) { a += 1 } } })
        s.spawn(fn() { when (b) { when (a) { b += 1 } } })
    }
    0
}
```

```console
$ lupin ex12-9.lu
ex12-9.lu: E0201: `when` acquires a set, so it needs at least two operands; for one, call the method on the sync type [gram.expr.conc] at 544..552
$ echo $?
2
```

It dies in the parser. The AB-BA deadlock needs *incremental*
acquisition — hold one lock while asking for another — and `when`'s
grammar has no one-lock form to nest: E0201 says take the set whole or
do not use `when`. The bug is not detected; it is unspellable, which
is a stronger guarantee than any detector. (Deadlock through channels
remains constructible — exercise 12-2 — because waiting for *data* is
a program's own business; acquiring *locks* piecemeal was never
anything but a bug factory.)

Note: the two implementations once disagreed here — wolf rejecting the
program as `E0201` where lupin said `E0203`. Both report E0201, with
the same rule in their own words, one at the parse rung and one before
the first line runs. The transcript above is lupin's at the current
pin; §12.4 prints the compiler's half beside it.
</details>

## Chapter 13

<details>
<summary>Exercise 13-2 — [§13.2](../ch13.md#13.2)</summary>

**Exercise 13-2** *(fingers · lupin)* — Nine numbers, squared and
summed, on one core: build the list, square each into a second list, add
them up, print the total. Run it and keep the program. The number it
prints is the number every divided version of this job has to agree
with, and a divided job that does not reproduce its sequential answer is
not faster — it is wrong.

(Printed in §13.2, not the §13.1 the index assigns it: §13.1 is held,
and the sequential baseline is what §13.2 opens on. The one-line-diff
framing this stem used to carry is 13-1's and travels back to it when
`par` lands.)

Solution — `ch13/ex13-2.lu`:

```wolf
fn main() -> !int {
    var xs = List[int]()
    for i in 1..=9 { xs.push(i) }
    var sq = List[int]()
    for x in xs { sq.push(x * x) }
    var sum = 0
    for v in sq { sum += v }
    print("{sum}")
    0
}
```

```console
$ lupin ex13-2.lu
285
```
</details>

<details>
<summary>Exercise 13-3 — [§13.2](../ch13.md#13.2)</summary>

**Exercise 13-3** *(comprehension · wolf + lupin)* — Two tasks
increment a captured `var`:

```wolf
fn main() -> !int {
    var hits = 0
    scope s {
        s.spawn(fn() { hits += 1 })
        s.spawn(fn() { hits += 1 })
    }
    hits
}
```

Before running it, predict the three fixes the note offers and which two
apply here. Then run it under both tools and account for the difference
in what they print — the codes and the spans agree, and the amount of
output does not. Which tool tells you about the second `spawn`, and what
does the extra warning on it say that `W1101` did not?

Solution: the three fixes are a channel (each task sends, one owner
adds), a `Mutex` acquired in a `when` (for state that is genuinely
shared), and `par` with a reduction (for the loop-shaped cases). The
first two apply here; the third wants a loop over a collection, and this
program has two hand-written tasks. The compiler names all three, once
per spawn:

```console
$ wolf conform-run ./ex13-3.lu
error[E1101]: this task writes to `hits`, which it captures from the enclosing function
 --> ./ex13-3.lu:9:24
  |
9 |         s.spawn(fn() { hits += 1 })
  |         --------------------------- the task's closure captures it at this spawn
  |                        ^^^^ tasks cannot mutate captured state
  |
  = note: task captures are copies, `imm` shares, or region moves (D14) — never mutable windows
    onto the parent's locals; two tasks writing one binding is the data race the memory
    model forbids. Three ways out: send results over a `channel` and let one owner mutate;
    guard truly shared state with a `Mutex` acquired in a `when` block; or, for loop-shaped
    work, use `par` with a reduction.
```

The second `s.spawn` earns the same error at line 10, and two warnings
ride along: W1101 ("this write to `hits` stays inside the task") and,
on the second spawn only, W1102 ("the closure above captured `hits` by
value, so it will not see this assignment"). W1102 is the one the
question asks about — it is not about this closure's write but about
the *previous* closure, which took its copy of `hits` before this line
ran and will never see it. Five diagnostics for one mistake with two
instances.

The interpreter refuses the same program, and says it once:

```console
$ lupin ex13-3.lu
ex13-3.lu: E1101: this task writes to `hits`, which it captures from the enclosing function: unsynchronized mutable capture across tasks (D14 — copy, share `imm`, or `move`; a `sync` type mediates shared writes) [conc.task.spawn] at 318..322
$ echo $?
2
```

Same code, same headline, same span as the compiler's first error —
`318..322` is the `hits` at line 9. What differs is volume, not verdict:
the compiler reports every offending spawn and both warnings, the
interpreter reports the first refusal and stops. Neither runs it.

Worth knowing what this exercise used to be. Until lupin 0.1.6 the
second half was a *differential*: the interpreter ran this program to
exit 0, having captured by value, so both increments landed on private
copies and were lost and `hits` printed as 0 — a silently wrong answer
from a clean exit code. That contrast was the argument for making the
capture a compile error. The argument won; the demonstration is gone,
and W1101's note is where the lost-update story now lives.
</details>

<details>
<summary>Exercise 13-4 — [§13.2](../ch13.md#13.2)</summary>

**Exercise 13-4** *(spelunking · lupin)* — Two programs differ in one
`print`. Explore both and read the verdicts:

```console
$ lupin conform-run ex13-4a.lu --explore=8
ex13-4a.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×1 stdout=1 then 2, sum 3\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0,0,0
    exit(0) ×1 stdout=2 then 1, sum 3\n leaks=0 forest=ok — replay: --seed=4611686018427387905
      decision stream: ev:1,0,0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
$ lupin conform-run ex13-4b.lu --explore=8
ex13-4b.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 1 distinct — observably deterministic (every schedule agrees)
    exit(0) ×2 stdout=sum 3\n leaks=0 forest=ok — replay: --seed=0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
```

`ex13-4a.lu` prints arrival order and the sum; `ex13-4b.lu` prints
only the sum. Same tasks, same channel, same schedules. Why do the
verdicts differ, and what does that mean for how you design a parallel
reduction's output?

Solution (prose): determinism, as the explorer measures it, is a
property of what the program *observes about* its schedules, not of
the schedules themselves. Both programs run the same two interleavings
(`races: 0` — channel operations synchronize; nothing here is a data
race); 4a copies the arrival order into stdout, so its two schedules
produce two outputs and the verdict is schedule-dependent, while 4b
folds the values with `+`, which commutes, collapsing both schedules
into one observable outcome. This is the design rule for `par`
reductions in one sentence: combine with operations whose result does
not encode arrival order, and the whole parallel program stays
observably deterministic — which is the property `--explore` exists to
certify, and the property 13-1's `par` preserves by returning results
in input order.
</details>

<details>
<summary>Exercise 13-5 — [§13.2](../ch13.md#13.2)</summary>

**Exercise 13-5** *(extension · lupin)* — grep, wolfished: write
`grep(text, pattern) -> List[str] ! {EmptyPattern}` returning the
matching lines. Substring search is yours to write with byte slices.
Why is the empty pattern an *error* here, when POSIX grep happily
matches it everywhere?

Solution — `ch13/ex13-5.lu`:

```wolf
fn contains(hay: str, needle: str) -> bool {
    if hay.len < needle.len { return false }
    var i = 0
    while i + needle.len <= hay.len {
        if hay[i..i + needle.len] == needle { return true }
        i += 1
    }
    false
}
fn grep(text: str, pattern: str) -> List[str] ! {EmptyPattern} {
    if pattern.is_empty() { return EmptyPattern }
    var hits = List[str]()
    for line in text.lines() {
        if contains(line, pattern) { hits.push(line) }
    }
    hits
}
```

```console
$ lupin ex13-5.lu
the wolf runs at dusk
a lone wolf watches
2 match(es)
empty pattern refused
```

POSIX grep's empty pattern means "match every line," a convention a
human at a terminal can exploit and a program calling a function
almost never intends — it is usually a variable that turned out
blank. The row makes the caller say which they meant: handle
`EmptyPattern` with "all lines" if that is truly the wish. An API's
defaults should serve its likeliest accident, not its cleverest use.
</details>

<details>
<summary>Exercise 13-7 — [§13.2](../ch13.md#13.2)</summary>

**Exercise 13-7** *(comprehension · lupin)* — One Euler step for two
bodies on a line, gravity only, equal masses. Before running: what is
`v1 + v2` after the step, and is your answer exact or approximate for
f64 arithmetic?

Solution — `ch13/ex13-7.lu` (excerpt):

```wolf
let r = absf(x2 - x1)
let a = g / (r * r)
v1 += a * dt
v2 -= a * dt
x1 += v1 * dt
x2 += v2 * dt
```

```console
$ lupin ex13-7.lu
x1=1.0 v1=1.0
x2=9.0 v2=-1.0
momentum 0.0
```

Exactly zero, and not by luck: both velocity updates add and subtract
the *same computed value* `a * dt`, and for any f64 value `x`, the sum
`x + (0.0 - x)` is exactly 0.0. Momentum conservation here is a
property of sharing one rounding, not of infinite precision — compute
the two accelerations separately with different roundings and the
symmetry is gone. That observation is the seed of every reproducible
n-body benchmark in Part 4.
</details>

## Chapter 14

<details>
<summary>Exercise 14-1 — [§14.1](../ch14.md#14.1)</summary>

**Exercise 14-1** *(comprehension · lupin)* — A proc's function returns
`3`. Before running, predict both fields of the line this prints:

```wolf
fn worker() -> int { 3 }
fn main() -> !int {
    let w = spawn proc worker()
    let m = w.monitor()
    select {
        exit(reason) from m => {
            print("normal={reason.is_normal()} killed={reason.is_killed()}")
        },
        timeout(1.s) => { return 1 },
    }
    0
}
```

Solution: `normal=true killed=false`. Returning a value — any value —
is a normal exit; the value rides along in the reason. Failure is a
separate channel of information: an error return or a kill, not a
number the supervisor squints at. Erlang made the same cut, and for
the same reason: the exit *reason* is the protocol, the return value
is the result.

```console
$ lupin ex14-1.lu
normal=true killed=false
```
</details>

<details>
<summary>Exercise 14-2 — [§14.1](../ch14.md#14.1)</summary>

**Exercise 14-2** *(design)* — A log-search service has three
concerns: an HTTP listener, a query planner, and one index shard per
disk. Argue where the proc boundaries go. For each boundary you draw,
name the failure it isolates and the state that dies with it; for one
boundary you chose *not* to draw, name what shared fate you accepted.

Solution (discussion): the strong answer puts each shard in its own
proc — a corrupt index or a crashing decompressor takes down one
shard's regions and nothing else, and the planner degrades to
partial results. The listener and planner can share a proc at first:
they share fate anyway (no listener, no queries), and splitting them
buys isolation only once the planner holds cache state worth keeping
through a listener restart. The boundary not drawn is the one between
planner and its per-query scratch — that is a region inside the proc,
not a proc: it dies with the query, and promoting it to a proc would
turn cheap bulk-frees into protocol. Ownership, failure, and service
land on the same line or the design fights itself — that is
Armstrong's argument, applied.
</details>

<details>
<summary>Exercise 14-3 — [§14.2](../ch14.md#14.2)</summary>

**Exercise 14-3** *(comprehension · lupin)* — `build_then_crash`
allocates a hundred integers into a region, then returns an error.
Predict what the monitor reports and, separately, what happened to the
hundred integers — then say which line of code freed them.

Solution — `ch14/ex14-3.lu` (excerpt):

```wolf
fn build_then_crash() -> !int {
    let r = region()
    let n = in r {
        var xs = List[int]()
        for i in 0..100 { xs.push(i) }
        xs.len
    }
    Boom
}
```

```console
$ lupin ex14-3.lu
proc down; its regions are gone
```

No line of code freed them. The proc's death is the deallocation: a
proc owns its regions, and an abnormal exit frees them wholesale —
no unwinding, no per-object teardown. The monitor's `exit(reason)`
arrives after the memory is already gone.
</details>

<details>
<summary>Exercise 14-4 — [§14.2](../ch14.md#14.2)</summary>

**Exercise 14-4** *(comprehension · lupin)* — `sleeper` registers a
defer and then blocks forever on an empty channel; the owner kills it.
Two prints are written in this program: `defer-skipped` in the proc
and `released` in the owner. Predict which of them appear, and in what
order:

```wolf
fn sleeper() -> int {
    defer print_raw("defer-skipped")
    let ch = channel[int](0)
    let v = ch.recv() else |_| { return 1 }
    v
}
fn main() -> !int {
    let w = spawn proc sleeper()
    let m = w.monitor()
    w.kill()
    select {
        exit(reason) from m => {
            if reason.is_killed() { print_raw("released") } else { print_raw("wrong") }
        },
        timeout(1.s) => { print_raw("timeout") },
    }
    0
}
```

Solution: only `released`. A killed proc's defers do not run — this is
the decided rule, not an accident: a kill must be safe to issue against
a proc in any state, and running arbitrary cleanup code in a proc that
is being destroyed for misbehaving is not safe. The proc's regions
bulk-free instead. Resources that must survive a kill belong to the
owner's side of a channel, or to a supervisor.

```console
$ lupin ex14-4.lu
released
```
</details>

<details>
<summary>Exercise 14-5 — [§14.2](../ch14.md#14.2)</summary>

**Exercise 14-5** *(comprehension · lupin)* — The same shape at task
granularity. One sibling blocks on a channel with a defer registered;
the other fails. Predict the output — and then state, in one sentence
each, why this defer runs when 14-4's did not:

```wolf
fn race_them() -> !int {
    let ch = channel[int](0)
    scope s {
        s.spawn(fn() {
            defer print("sibling cleanup ran")
            let v = ch.recv()?
            v
        })
        s.spawn(fn() { fail_fast() })
    }
    0
}
```

Solution: the cleanup line prints, then the caller's handler value is
used:

```console
$ lupin ex14-5.lu
sibling cleanup ran
caught
```

Cancellation is cooperative: it arrives at a blocking point in a task
that is still healthy, so its defers run. A kill is unilateral: it
destroys a proc that may be past cooperating, so they do not. One
cleanup contract per failure primitive, and the primitive tells you
which one you are holding.
</details>

<details>
<summary>Exercise 14-6 — [§14.3](../ch14.md#14.3)</summary>

**Exercise 14-6** *(fingers · lupin)* — Build a counting service: a
proc that reads commands from a channel, where `0` means "reply with
the total" and any other value adds to it. Drive it with 5, 2, then a
report, and print what comes back.

Solution — `ch14/ex14-6.lu`:

```wolf
fn counter(cmds: channel[int], replies: channel[int]) -> int {
    var total = 0
    for c in cmds {
        if c == 0 { replies.send(total) } else { total += c }
    }
    total
}
fn main() -> !int {
    let cmds = channel[int](8)
    let replies = channel[int](1)
    let w = spawn proc counter(cmds, replies)
    cmds.send(5)
    cmds.send(2)
    cmds.send(0)
    let t = replies.recv() else |_| { return 1 }
    print("total={t}")
    cmds.close()
    0
}
```

```console
$ lupin ex14-6.lu
total=7
```

The proc's `for c in cmds` loop *is* the mailbox: commands queue in
the channel, the proc serializes them, and the reply channel carries
answers back. No lock appears because no state is shared — the total
lives in exactly one proc.
</details>

<details>
<summary>Exercise 14-7 — [§14.3](../ch14.md#14.3)</summary>

**Exercise 14-7** *(extension · lupin)* — Grow the protocol: `-1`
resets the counter. Report the total, reset, add 3, and report again.
Predict both numbers first, then answer: what ordering guarantee makes
your prediction safe, and which chapter taught it?

Solution — `ch14/ex14-7.lu` (excerpt):

```wolf
    for c in cmds {
        if c == 0 { replies.send(total) } else if c == 0 - 1 { total = 0 } else { total += c }
    }
```

```console
$ lupin ex14-7.lu
before=7 after=3
```

`before=7 after=3` is safe to predict because one sender's sends
arrive in order (chapter 12's happens-before): the reset cannot
overtake the first report, and the 3 cannot overtake the reset. The
protocol's integers are starting to strain — a real command set wants
a type, which is where rows and enums earn their keep.
</details>

<details>
<summary>Exercise 14-8 — [§14.3](../ch14.md#14.3)</summary>

**Exercise 14-8** *(design)* — Erlang mailboxes offer *selective
receive*: a proc can pluck the first message matching a pattern,
leaving the rest queued. Wolf's mailbox is a FIFO channel plus
`select` over multiple channels. State one protocol that selective
receive expresses more directly, then argue wolf's side: what does a
skipped-over message cost in Erlang that wolf's design refuses to pay?

Solution (discussion): call-response over a shared mailbox is the
classic selective-receive win — reply matching happens by pattern
while unrelated traffic waits. The cost is the unbounded scan: every
receive may walk the whole queue, and a proc that never matches some
message class leaks queue memory silently; Erlang folklore is full of
mailboxes that grew until the node died. Wolf's answer is one channel
per conversation — the reply channel in 14-6 is exactly that — which
turns pattern-matching into channel topology the scheduler can see.
The queue that would have grown silently becomes a channel you had to
declare, with a capacity you chose. Less expressive per receive;
every queue in the program has a name and a bound.
</details>

<details>
<summary>Exercise 14-9 — [§14.3](../ch14.md#14.3)</summary>

**Exercise 14-9** *(comprehension + schedule play · lupin)* — Two
client tasks each send two increments to the counting proc; the scope
joins, then main asks for the total. Run it under seeds 0, 1, 5, 9.
Predict first: does the total vary with the schedule, and why not —
and name the thing that *does* vary between those runs even though no
output shows it.

Solution — `ch14/ex14-9.lu` (main excerpt):

```wolf
    scope s {
        s.spawn(fn() { client(cmds) })
        s.spawn(fn() { client(cmds) })
    }
    cmds.send(0)
```

```console
$ lupin run ex14-9.lu --seed=0
total=4
$ lupin run ex14-9.lu --seed=1
total=4
$ lupin run ex14-9.lu --seed=5
total=4
$ lupin run ex14-9.lu --seed=9
total=4
```

The total is schedule-independent: the scope join happens-before the
report command, so all four increments are in the mailbox before `0`
enters it. What varies is the interleaving of the two clients' sends —
the mailbox's arrival order — which addition happens to hide, because
addition commutes. Replace `total += c` with an operation that does
not commute and the seeds stop agreeing; chapter 17 hunts exactly that
program.
</details>

## Chapter 15

<details>
<summary>Exercise 15-1 — [§15.1](../ch15.md#15.1)</summary>

**Exercise 15-1** *(comprehension · lupin)* — Chapter 14 showed a proc
returning a value: `is_normal()` was true. This proc returns an error
instead. Predict both fields — there are three possible exit shapes
and this line can only show you two booleans:

```wolf
fn boom() -> !int { Bad }
fn main() -> !int {
    let w = spawn proc boom()
    let m = w.monitor()
    select {
        exit(reason) from m => {
            print("normal={reason.is_normal()} killed={reason.is_killed()}")
        },
        timeout(1.s) => { return 1 },
    }
    0
}
```

Solution: `normal=false killed=false`. Both predicates are false
because an error return is the third shape: not a normal completion,
not a kill, but a failure the proc itself reported. The reason
taxonomy is three-valued — normal, killed, errored — and a supervisor
that only checks `is_normal()` treats the last two the same, which is
usually what it wants.

```console
$ lupin ex15-1.lu
normal=false killed=false
```
</details>

<details>
<summary>Exercise 15-2 — [§15.1](../ch15.md#15.1)</summary>

**Exercise 15-2** *(comprehension · lupin)* — `monitor` delivers a
message; `link` shares fate. This program links to a proc that fails,
then blocks on an empty channel. Two prints are written. Predict what
appears on stdout, and what `echo $?` shows:

```wolf
fn boom() -> !int { Bad }
fn main() -> !int {
    let w = spawn proc boom()
    w.link()
    let ch = channel[int](0)
    let v = ch.recv() else |err| { print("recv failed"); return 7 }
    print("got {v}")
    0
}
```

Solution: nothing appears, and the exit code is 1. The link propagates
the failure into `main` at its blocking point — no error value arrives
at the `else` handler, because shared fate is not an error return; it
is death. The handler that never ran is the lesson: `link` is for
"if it dies, we die," and code below a link is written in that
knowledge. Choose `monitor` when failure is information; choose
`link` when failure is contagion, and mean it.

```console
$ lupin ex15-2.lu
$ echo $?
1
```
</details>

<details>
<summary>Exercise 15-3 — [§15.1](../ch15.md#15.1)</summary>

**Exercise 15-3** *(design)* — A pipeline proc feeds a compressor
proc, which feeds an uploader proc. For each of the three pairs,
choose `link` or `monitor` and defend the choice with the failure you
are designing for. One of the three answers should be "neither" —
which, and what replaces it?

Solution (discussion): compressor–uploader wants `link`: a compressor
with no uploader is doing work nobody will receive, and vice versa —
shared fate matches the data dependency. Pipeline–compressor wants
`monitor` in the supervisor above them, not a link between them: the
pipeline can buffer or reroute while a replacement compressor spins
up, so failure is information there. The "neither" pair is whichever
one you were tempted to wire both directions: a link is already
bidirectional, and a link plus a monitor between the same two procs
means the fate decision was never actually made. The supervisor tree
makes these choices once, at the top, where the restart policy lives —
wiring fate ad hoc between siblings is how systems grow failure paths
nobody drew.
</details>

<details>
<summary>Exercise 15-4 — [§15.2](../ch15.md#15.2)</summary>

**Exercise 15-4** *(fingers · lupin)* — Build the smallest supervisor:
spawn a worker that fails on its first attempt and succeeds on its
second; monitor it; on an abnormal exit, print a line and respawn with
the next attempt number; stop after three attempts. Run it and keep
the output.

Solution — `ch15/ex15-4.lu`:

```wolf
fn worker(attempt: int) -> !int {
    if attempt == 1 { return Crash }
    0
}
fn main() -> !int {
    var attempt = 1
    var done = false
    while !done {
        let w = spawn proc worker(attempt)
        let m = w.monitor()
        select {
            exit(reason) from m => {
                if reason.is_normal() { done = true } else {
                    print("attempt {attempt} failed; restarting")
                    attempt += 1
                    if attempt > 3 { return 1 }
                }
            },
            timeout(1.s) => { return 1 },
        }
    }
    print("attempt {attempt} succeeded")
    0
}
```

```console
$ lupin ex15-4.lu
attempt 1 failed; restarting
attempt 2 succeeded
```

The whole mechanism is visible: a loop, a monitor, a judgment on the
reason, a bounded retry. The stdlib's supervisor adds policy — restart
strategies, intensity limits, child ordering — but no new primitive.
You have now built the thing the next section hands you.
</details>

<details>
<summary>Exercise 15-5 — [§15.2](../ch15.md#15.2)</summary>

**Exercise 15-5** *(comprehension · lupin)* — Same supervisor, but the
worker fails every time. Predict the full output and the exit code
before running — including how many times the worker actually runs.

Solution: three failures, a give-up line, exit 1. The budget is the
supervisor's honesty: without it, a deterministic crash loops forever,
and "restarting" degrades from recovery into denial. Three runs, not
four — the budget check happens after the increment, and off-by-one
predictions here are worth catching on paper rather than in a pager
rotation.

```console
$ lupin ex15-5.lu
attempt 1 failed
attempt 2 failed
attempt 3 failed
giving up after 3 attempts
$ echo $?
1
```
</details>

<details>
<summary>Exercise 15-6 — [§15.2](../ch15.md#15.2)</summary>

**Exercise 15-6** *(extension · lupin)* — Change the worker to fail
twice and succeed on the third attempt — the flappy dependency
pattern. Predict the output, run it, and then answer: your budget is
3. What single-character change makes this worker's recovery
impossible, and what does the output become?

Solution — `ch15/ex15-6.lu` (worker):

```wolf
fn worker(attempt: int) -> !int {
    if attempt < 3 { return Crash }
    0
}
```

```console
$ lupin ex15-6.lu
attempt 1 failed; restarting
attempt 2 failed; restarting
attempt 3 succeeded
```

Change the budget check `attempt > 3` to `attempt > 2` (or the
worker's `< 3` to `< 4`): the third attempt — the one that would have
succeeded — is never made, and the output becomes two failures and an
exit 1. Restart budgets are a bet about how flappy the world is;
this exercise is that bet lost by one.
</details>

<details>
<summary>Exercise 15-7 — [§15.3](../ch15.md#15.3)</summary>

**Exercise 15-7** *(design)* — Every proc in wolf lives under the root
supervisor; there is no unsupervised spawn. Sketch the supervision
tree for the chapter 14 log-search service (listener, planner, one
proc per shard), choosing for each internal node: restart the child
alone, restart all children, or escalate. Name the failure scenario
that made you pick each policy.

Solution (discussion): the shards sit under one internal supervisor
with restart-alone: shard failures are independent (one bad disk, one
corrupt segment), and restarting siblings would throw away warm
caches for no reason. The listener and planner sit under a
restart-all node: a planner that died mid-query leaves the listener
holding connections whose queries will never answer — restarting them
together resets the pair to a consistent nothing. The shard
supervisor escalates only when its restart budget exhausts — at which
point the service cannot search, and the root's policy (restart the
world, or die and let the OS supervisor act) is a deployment
decision, not a code decision. The tree is the failure design; the
procs are only its leaves.
</details>

<details>
<summary>Exercise 15-8 — [§15.3](../ch15.md#15.3)</summary>

**Exercise 15-8** *(spelunking · corpus)* — The pinned corpus checks
the kill rule with this directive header, from
`upstream/corpus/conc/proc_kill_defers.lu`:

```text
//! check: run(exit=0, stdout="released")
```

Explain why the `stdout=` clause — not the exit code — is the part of
this header that actually verifies the rule "defers in a killed proc
do not run." What would a conforming-looking run that violates the
rule produce, and which field would catch it?

Solution: the program is built so the killed proc's defer would print
`defer-skipped` — so a run where defers wrongly ran produces
`defer-skippedreleased` (or an interleaving of both), and only the
`stdout="released"` check sees the difference; the exit code is 0
either way. The header is a negative test smuggled into a positive
one: the asserted output is exact, so any extra byte from the
forbidden defer is a corpus failure. When a rule's violation is
silent in the exit code, pin the channel where it is loud — a
directive that checks the wrong observable verifies nothing.
</details>

<details>
<summary>Exercise 15-9 — [§15.3](../ch15.md#15.3)</summary>

**Exercise 15-9** *(design)* — A teammate proposes: "monitors are
strictly better — a link is a monitor whose handler calls exit, so
the language should ship only monitors." Take the other side using
15-2's observed behavior: name two properties of `link` that the
monitor-plus-handler encoding does not provide.

Solution (discussion): first, a link works while you are blocked —
15-2's `main` died at a `recv` it would never leave; the encoding's
handler only runs when its select next polls the monitor, so a
blocked proc survives exactly when it should not. Second, a link
cannot be forgotten on one path: the handler encoding must appear in
every select the proc ever blocks in, and the one select that omits
the monitor arm is an unsupervised wait. Fate declared once at wiring
beats fate re-asserted at every blocking point — the encoding is
strictly more expressive and reliably less used. (The full story adds
delivery guarantees under simultaneous failure; chapter 17's
scheduler material is where that becomes checkable.)
</details>

## Chapter 16

<details>
<summary>Exercise 16-1 — [§16.1](../ch16.md#16.1)</summary>

**Exercise 16-1** *(comprehension · lupin)* — The sender builds a list
inside a region — two pushes — and sends the region. Predict: does the
receiver's `in r2 { … }` block run before or after both pushes are
visible, and what synchronization made that true?

```wolf
    s.spawn(fn() {
        let r = region()
        let xs = in r {
            var v = List[int]()
            v.push(41)
            v.push(1)
            v
        }
        ch.send(move r)
    })
    let r2 = ch.recv() else |_| { return 1 }
    got = in r2 { 42 }
```

Solution: after — the `move` send happens-before the receive, so every
write into the region before the send is visible to the receiver, and
no other synchronization exists or is needed. The channel carried one
word (the region), not the data; the *ownership* is what moved. This
is the message-passing litmus from the memory model, and it is the
entire safety argument of this chapter in eight lines.

```console
$ lupin ex16-1.lu
received
```
</details>

<details>
<summary>Exercise 16-2 — [§16.1](../ch16.md#16.1)</summary>

**Exercise 16-2** *(fingers · lupin)* — Make the transfer carry real
freight: build the two-element list in `main`, send the region to a
receiving task, and have the receiver sum the list *it never built*.
Print the sum from the receiver's side.

Solution — `ch16/ex16-2.lu` (receiver):

```wolf
    s.spawn(fn() {
        let r2 = ch.recv() else |_| { return }
        let total = in r2 {
            var t = 0
            for x in xs { t += x }
            t
        }
        print("sum={total}")
    })
    ch.send(move r)
```

```console
$ lupin ex16-2.lu
sum=42
```

The receiver iterates a list allocated by someone else, in memory that
changed owners, and reads exactly what was written. Nothing was
serialized, nothing was copied — the graph moved as a graph. (Erlang
gets this safety by deep-copying every message; the next section's
lineup prices that.)
</details>

<details>
<summary>Exercise 16-3 — [§16.1](../ch16.md#16.1)</summary>

**Exercise 16-3** *(extension (break-it-on-purpose) · lupin)* — Construct the
smallest program in which a sender touches a region *after* sending it
with `move`. Predict the exact trap kind before running — it is one
you met in chapter 7, not a new one.

Solution — `ch16/ex16-3.lu`:

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    let n = in r { 41 }
    ch.send(move r)
    let m = in r { 1 }
    m
}
```

```console
$ lupin ex16-3.lu
ex16-3.lu: trap(use-after-move): `r` was moved out and is uninitialized here [mem.tier0.move.2] at 254..255; `r` moved here at 231..237
$ echo $?
3
```

The trap is `use-after-move`, citing the same clause family as chapter
7's moved string — because a region is a value and `move` into a
channel is the same move as `take` into a function. No new rule was
needed to make cross-task transfer safe; the old rule was enough, which
is the design working as designed. (The compiler rejects the shape statically, at the send; lupin carries
the same lesson dynamically, at the read.)
</details>

<details>
<summary>Exercise 16-4 — [§16.2](../ch16.md#16.2)</summary>

**Exercise 16-4** *(comprehension · lupin)* — Ten squares are frozen
into `table`; two tasks each read one entry and send it back. Predict
the printed number, and answer precisely: how many copies of the table
exist while both tasks read it?

```wolf
    let table = freeze region {
        var xs = List[int]()
        for i in 0..10 { xs.push(i * i) }
        xs
    }
    scope s {
        s.spawn(fn() { ch.send(table[3]) })
        s.spawn(fn() { ch.send(table[4]) })
    }
```

Solution: `25` (9 + 16), and one copy — the frozen original. Both
tasks hold references into the same immutable memory; `freeze`
happens-before every cross-task read, and immutability makes
concurrent reading trivially safe. Where `move` gives the data one
owner at a time, `freeze` gives it no owner that can write — two
different ways to make "who else is touching this?" unaskable.

```console
$ lupin ex16-4.lu
25
```
</details>

<details>
<summary>Exercise 16-5 — [§16.2](../ch16.md#16.2)</summary>

**Exercise 16-5** *(design)* — For each payload, choose `move` or
`freeze` and defend it in one sentence: (a) a parsed configuration
read by every worker for the process lifetime; (b) a request's parse
tree handed from parser to executor; (c) a routing table rebuilt every
30 seconds and read constantly; (d) a 2 GB index segment consulted by
eight shards.

Solution (discussion): (a) freeze — many readers, no writer, forever:
the definitional freeze. (b) move — exactly one consumer, which will
mutate and then discard it; freezing would forbid the executor its
annotations. (c) the interesting one: each rebuild is built mutable in
a fresh region, then frozen and published; readers of the old table
keep reading it until they pick up the new one — freeze does not mean
one-forever, it means immutable-per-edition. (d) freeze, and the size
is the argument: eight copies is 16 GB and one shared frozen segment
is 2, with no lock on the read path. The pattern under all four:
mutate privately, then either hand it over whole or make it
untouchable — wolf gives no third verb, on purpose.
</details>

<details>
<summary>Exercise 16-6 — [§16.3](../ch16.md#16.3)</summary>

**Exercise 16-6** *(design)* — The same workload — a producer builds a
million-node tree, a consumer walks it — in four systems: Erlang
(copying send), Go (send a pointer), Rust (`Arc<Mutex<Tree>>`), wolf
(`ch.send(move r)`). For each, name what the transfer costs at the
moment of send, and what it costs the *receiver* to be safe while
reading. One of the four pays at a different time than the others —
which?

Solution (discussion): Erlang pays at send — a deep copy of a million
nodes — and the receiver is then perfectly safe reading its private
copy; safety was purchased in one large cash payment. Go pays nothing
at send and the receiver is safe only by convention: nothing stops the
producer from mutating the shared tree, and the race detector, not the
compiler, is the enforcement. Rust's `Arc<Mutex>` pays a little at
send (refcount) and then pays on every read — lock traffic on a tree
that will never be written again, unless the design graduates to
`Arc<Tree>` frozen-by-construction, which is Rust spelling wolf's
answer manually. Wolf pays one word at send and nothing at read; the
compiler's move check is the whole bill, paid at compile time. That is
the odd one out: three systems pay at runtime in copies, faith, or
locks; wolf pays before the program runs. The honest caveat: Erlang's
copy buys process isolation across *machines* with the same
semantics, which no move can — distribution is where the copy stops
looking expensive.
</details>

<details>
<summary>Exercise 16-7 — [§16.3](../ch16.md#16.3)</summary>

**Exercise 16-7** *(extension · lupin)* — A maze is a graph, and a
graph is a region's favorite payload. Carve a 5×5 maze with a seeded
generator (per-cell wall bitmasks: 1=N 2=E 4=S 8=W, depth-first carve,
a small linear-congruential step for direction choice), building the
wall table inside a region. Send the region to a solver task; the
solver breadth-first-searches it in place and prints the shortest-path
distance from corner to corner. Seed 1: run it. Before you do, answer:
how many times is the wall table copied between carver and solver?

Solution — `ch16/ex16-7.lu` (the transfer; carve and solve are in the
file):

```wolf
fn main() -> !int {
    let ch = channel[region](1)
    let r = region()
    let walls = in r { carve(5, 5, 1) }
    scope s {
        s.spawn(fn() {
            let r2 = ch.recv() else |_| { return }
            let d = in r2 { solve(walls, 5, 5) }
            print("distance={d}")
        })
        ch.send(move r)
    }
    0
}
```

```console
$ lupin ex16-7.lu
distance=18
```

Zero copies. The carver allocated the table, the stack, and the
visited set into one region; the solver's BFS queue and distance table
could live there too. One `move` later the solver owns all of it, and
the answer — 18 steps for seed 1 — comes out of memory the solver
never allocated.
</details>

<details>
<summary>Exercise 16-8 — [§16.3](../ch16.md#16.3)</summary>

**Exercise 16-8** *(comprehension + schedule play · lupin)* — Change
the carve seed to 2 and run the program three times, including once
under `lupin run … --seed=7`. Predict: which of the two seeds in play
changes the printed distance, and which cannot — and why does this
program print the same distance under every scheduler seed?

Solution — `ch16/ex16-8.lu` (one line differs from 16-7):

```console
$ lupin ex16-8.lu
distance=10
$ lupin ex16-8.lu
distance=10
$ lupin run ex16-8.lu --seed=7
distance=10
```

The carve seed changes the maze — seed 2 happens to carve a more
direct route, distance 10. The scheduler seed cannot change anything:
the program has one send, one receive, and a join; every schedule
orders them the same way, so the output is schedule-independent by
construction. Determinism you can argue from the program's shape is
worth more than determinism you observed in three runs — chapter 17
is about the programs where you cannot argue it.
</details>

<details>
<summary>Exercise 16-9 — [§16.3](../ch16.md#16.3)</summary>

**Exercise 16-9** *(comprehension · wolf + lupin)* — This program
declares a channel of bare `List[int]` — not `Copy`, not imm, not a
region, not sync:

```wolf
fn main() -> !int {
    let ch = channel[List[int]](1)
    0
}
```

Predict the verdict this program earns and the rule behind it, and
explain why each of the four admitted payload classes is safe where a
bare `List` is not.

Solution: the verdict is `fail(E1102)`, and the note names all four
classes:

```console
$ wolf conform-run ./ex16-9.lu
error[E1102]: `List[int]` cannot be sent through a channel
 --> ./ex16-9.lu:7:22
  |
7 |     let ch = channel[List[int]](1)
  |                      ^^^^^^^^^ not a sendable payload type
  |
  = note: channel payloads must be `Copy` data, `imm` data, a region value (the send is its affine
    move), or a `sync` type ([conc.chan.type]) — sending anything else would give two tasks
    one mutable value. D14's verbs are the ways out: `move` the data into a region and send
    the region, `freeze` it into shareable `imm` data, or guard it with a `Mutex`.
```

Each admitted class removes one half of the race. `Copy` data: the
receiver gets its own bits, so there is no shared location. `imm` data:
there is a shared location and nobody may write it. A region value: the
send is a move, so exactly one task owns it at any instant. A `sync`
type: the sharing is real and the coordination is the type's own job. A
bare `List` is none of these — sending it would give two tasks live
access to one mutable buffer with no coordination, which is chapter 13's
store-buffer program wearing a channel as a disguise.

Note that the rejection is a property of the *declaration*: no `send`
appears in the program, and none is needed. The type of the channel is
already the claim, and E1102 is the compiler declining it. The
interpreter takes the other route and constructs the channel — its
dynamic machine catches an actual cross-task mutation rather than the
declaration — so this is one more program the compiler stops and lupin
runs:

```console
$ lupin ex16-9.lu
$ echo $?
0
```

The corpus carries the same expectation in `conc/chan_unsendable.lu`.
</details>

## Chapter 17

<details>
<summary>Exercise 17-1 — [§17.1](../ch17.md#17.1)</summary>

**Exercise 17-1** *(comprehension · lupin)* — Two tasks each deposit
50 into a balance that starts at 0, through a get-then-set protocol
with the store loop in `main`. There is no shared mutable capture, no
`Mutex`, nothing chapter 13 would reject. Predict the balance under
the default FIFO schedule — and state what the correct answer would
be if deposits never interfered:

```wolf
fn deposit(getreq: channel[int], getrep: channel[int], setch: channel[int]) {
    getreq.send(1)
    let v = getrep.recv() else |_| { return }
    setch.send(v + 50)
}
fn main() -> !int {
    let getreq = channel[int](0)
    let getrep = channel[int](0)
    let setch = channel[int](0)
    var balance = 0
    scope s {
        s.spawn(fn() { deposit(getreq, getrep, setch) })
        s.spawn(fn() { deposit(getreq, getrep, setch) })
        var served = 0
        while served < 4 {
            select {
                _ from getreq => { getrep.send(balance) },
                v from setch => { balance = v },
            }
            served += 1
        }
    }
    print("balance={balance}")
    0
}
```

Solution: the correct total is 100; FIFO prints 50. Both tasks get 0
before either set arrives, so both compute 0 + 50 and the second set
overwrites the first — the lost update. Every individual message is
delivered exactly once, in order, race-free; the *composite* operation
get-then-set is what was never atomic. This is an ordering bug, not a
data race, and no type system that admits channels can reject it —
which the book says plainly, because the pitch for chapter 13 was
narrower than folklore remembers.

```console
$ lupin ex17-1.lu
balance=50
```
</details>

<details>
<summary>Exercise 17-2 — [§17.1](../ch17.md#17.1)</summary>

**Exercise 17-2** *(comprehension (schedule play) · lupin)* — Hunt it: run 17-1 under
seeds 0 through 5. Record each balance. Which seeds produce the
correct answer, and what had to happen in the schedule for 100 to
come out?

Solution:

```console
$ lupin run ex17-1.lu --seed=0
balance=50
$ lupin run ex17-1.lu --seed=1
balance=100
$ lupin run ex17-1.lu --seed=2
balance=50
$ lupin run ex17-1.lu --seed=3
balance=50
$ lupin run ex17-1.lu --seed=4
balance=100
$ lupin run ex17-1.lu --seed=5
balance=100
```

Seeds 1, 4, and 5 serialize the deposits: one task's set reaches the
store before the other task's get, so the second deposit reads 50 and
writes 100. The failing outcome is not rare — it is the *common* one
here, which inverts the usual heisenbug story: in production this bug
would look like a test that occasionally passes.

The explorer agrees, and says which schedules do it:

```console
$ lupin conform-run ex17-1.lu --explore=500
ex17-1.lu: explored 20 schedule(s) in 20 execution(s) (DPOR; 0 slept, 6 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×16 stdout=balance=50\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0,0,0,0,0,0,0,0,0,0,0,0
    exit(0) ×4 stdout=balance=100\n leaks=0 forest=ok — replay: --seed=4611686018427387910
      decision stream: ev:0,1,1,0,0,0,0,0,0,0,0
  deadlocks: 0 · races: 0 · max depth: 12 decision(s)
$ echo $?
1
```

(The frontier claim is what §17.2 rests its argument on, so it is worth
knowing that it is checked: an earlier interpreter release reported this
program `observably deterministic` while a seeded run printed the wrong
balance, and that disagreement was filed against the interpreter rather
than written around here.)
</details>

<details>
<summary>Exercise 17-3 — [§17.2](../ch17.md#17.2)</summary>

**Exercise 17-3** *(fingers · lupin)* — Two sends race into one
channel; main prints the arrival order. Run it twice with `--seed=0`,
once with `--seed=3`, and once with `--schedule=ev:0,0,0`. Before
running: which pairs of those four runs are guaranteed to match?

```wolf
    scope s {
        s.spawn(fn() { ch.send(1) })
        s.spawn(fn() { ch.send(2) })
    }
    let a = ch.recv() else |_| { return 1 }
    let b = ch.recv() else |_| { return 1 }
    print("{a}{b}")
```

Solution: the two `--seed=0` runs must match — a seed selects the
whole decision stream, and the same seed replays byte-identically.
The `ev:0,0,0` run matches them here because that stream is the one
seed 0 selects. `--seed=3` carries no guarantee relative to the
others; it happens to choose the other order:

```console
$ lupin run ex17-3.lu --seed=0
12
$ lupin run ex17-3.lu --seed=0
12
$ lupin run ex17-3.lu --seed=3
21
$ lupin run ex17-3.lu --schedule=ev:0,0,0
12
```

A schedule is a value. Two runs disagree only if their schedules
disagree, and a schedule you can name is a schedule you can rerun —
that is the entire mechanism this chapter's debugging story stands on.
</details>

<details>
<summary>Exercise 17-4 — [§17.2](../ch17.md#17.2)</summary>

**Exercise 17-4** *(comprehension · lupin)* — The explorer prints the
two schedules of 17-3 as decision streams `ev:0,0,0` and `ev:1,0,0`.
Three decisions, but only the first digit ever differs. What is the
first decision choosing between — and why are the remaining two
decisions no longer choices once it is made?

Solution: the first decision picks which spawned task runs at the
first scheduling point — task one's send or task two's send fires
first. After that, the program has no freedom left: the other send is
the only runnable step, and the two receives in `main` drain the
channel in arrival order. A decision stream records *choices*, not
events; a program's concurrency is measured by how many entries in
that stream could have gone otherwise, and this program has exactly
one.
</details>

<details>
<summary>Exercise 17-5 — [§17.2](../ch17.md#17.2)</summary>

**Exercise 17-5** *(spelunking · lupin)* — Run
`lupin conform-run ex17-3.lu --explore=64` and read the report back:
explain `explored 2 schedule(s)`, `DPOR`, `frontier closed`,
`SCHEDULE-DEPENDENT`, the per-outcome `replay:` seeds, and the process
exit code.

Solution — the run:

```console
$ lupin conform-run ex17-3.lu --explore=64
ex17-3.lu: explored 2 schedule(s) in 2 execution(s) (DPOR; 0 slept, 0 pruned), frontier closed
  outcomes: 2 distinct — SCHEDULE-DEPENDENT
    exit(0) ×1 stdout=12\n leaks=0 forest=ok — replay: --seed=0
      decision stream: ev:0,0,0
    exit(0) ×1 stdout=21\n leaks=0 forest=ok — replay: --seed=4611686018427387905
      decision stream: ev:1,0,0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
$ echo $?
1
```

`explored 2 schedule(s)`: the program has two inequivalent orderings,
and both were run. `DPOR` is dynamic partial-order reduction — the
algorithm that knew the other interleavings were equivalent to these
two, so 2 executions covered the space a naive search would have
enumerated. `frontier closed`: no reachable schedule was left
untried within the budget. `SCHEDULE-DEPENDENT`: the outcomes differ
across schedules — a finding, which is why the exit code is 1 even
though every individual run exited 0. Each outcome carries a
`replay:` seed — the finding arrives with its own reproduction
command, which is the difference between a bug report and an anecdote.
</details>

<details>
<summary>Exercise 17-7 — [§17.3](../ch17.md#17.3)</summary>

**Exercise 17-7** *(comprehension · lupin)* — Rerun 17-3's exploration
with `--explore-preemptions=0`. Predict what the report will claim
about determinism before you run it, then reconcile the claim with
17-5's.

Solution:

```console
$ lupin conform-run ex17-3.lu --explore=64 --explore-preemptions=0
ex17-3.lu: explored 1 schedule(s) in 1 execution(s) (DPOR; 0 slept, 0 pruned), frontier OPEN
  note: preemption bound 0 skipped alternatives; frontier open
  outcomes: 1 distinct — observably deterministic (every schedule agrees)
    exit(0) ×1 stdout=12\n leaks=0 forest=ok — replay: --seed=0
  deadlocks: 0 · races: 0 · max depth: 3 decision(s)
$ echo $?
0
```

With zero preemptions allowed, only the FIFO schedule is explored, and
the report says "observably deterministic — every schedule agrees":
true over the schedules it looked at, and wrong about the program, as
17-5 proved. The tool is honest about the gap — `frontier OPEN` and
the `note:` line say the search was cut short, and the exit code is 0
only because no finding was reached. Read exploration reports the way
you read benchmarks: the verdict is conditional on the budget line,
and "frontier open" is the condition talking.
</details>

<details>
<summary>Exercise 17-8 — [§17.3](../ch17.md#17.3)</summary>

**Exercise 17-8** *(design)* — List three behaviors of a real
concurrent service that seeded schedule exploration, as this chapter
defines it, cannot find — and for each, name the tool or practice
that owns it instead. The chapter's own scope-honesty section (§17.4)
claims v1 promises less than folklore expects; your answer is that
claim, made concrete.

Solution (discussion): first, value nondeterminism — a hash seed, a
random backoff, an id from the OS: exploration permutes *scheduling*
decisions, not data, so property tests and fuzzing own that axis.
Second, real time — a timeout that fires only when a peer takes 30
actual seconds, kernel-buffer pressure, the network: the deterministic
scheduler virtualizes time, so what it validates is your *handling*
of a timeout, never the calibration of one; load tests own the
calibration. Third, anything past the FFI membrane — a C library's
internal threads and its file-descriptor games are invisible to a
scheduler that only sees wolf's blocking points; the audit boundary
of chapter 9 and the C library's own test suite own that. Exploration
proves ordering properties over the events it can see and permute —
that sentence, with both clauses stressed, is §17.4's whole content.
</details>

<details>
<summary>Exercise 17-9 — [§17.3](../ch17.md#17.3)</summary>

**Exercise 17-9** *(extension (break-it-on-purpose) · lupin)* — Construct a
deadlock from two tasks and two rendezvous channels, each task
receiving first and sending second. Predict the trap's roster before
running: how many tasks does it name, and why is the answer three
when you wrote two?

Solution — `ch17/ex17-9.lu`:

```wolf
fn main() -> !int {
    let a = channel[int](0)
    let b = channel[int](0)
    scope s {
        s.spawn(fn() {
            let x = a.recv() else |_| { return }
            b.send(x)
        })
        s.spawn(fn() {
            let y = b.recv() else |_| { return }
            a.send(y)
        })
    }
    0
}
```

```console
$ lupin ex17-9.lu
ex17-9.lu: trap(deadlock): every live task is blocked at a runtime-owned blocking point and no timer is pending; blocked-task roster: `main` (task 0), `task@231` (task 1), `task@336` (task 2) [conc.deadlock.trap] at 213..438
$ echo $?
3
```

Three, because `main` is blocked too — at the scope join, waiting for
children who are waiting for each other. The roster is the trap's gift:
it names every task and where it blocked, which is the state a
production deadlock never hands you. The trap fires because *every*
live task is blocked with no timer pending — a quiet program and a
deadlocked one differ in exactly that clause, and the runtime can tell
them apart. (Compare 12.4: `when (a, b)` exists so lock-order
deadlocks cannot be written; this exercise built the channel-order
equivalent by hand, on purpose.)
</details>

## Chapter 18

<details>
<summary>Exercise 18-1 — [§18.1](../ch18.md#18.1)</summary>

**Exercise 18-1** *(comprehension · wolf)* — One binding keeps this
program out of compile time:

```wolf
comptime fn double(n: int) -> int {
    n + n
}
fn main() -> !int {
    let x = 21
    const Y = double(x)
    if Y == 42 { 0 } else { 1 }
}
```

Predict the compiler's verdict, and name the one-character change that
fixes the program. What may an argument to a `comptime fn` be?

Solution: `let x` is a runtime value, and a `comptime fn` cannot
receive one — the fix is `const x = 21`. Arguments must be literals,
`const`s, types, or results of other comptime calls; the diagnostic
recites the list:

```console
$ wolf conform-run ./ex18-1.lu
error[E0705]: `x` is a runtime value, so this cannot evaluate at compile time
 --> ./ex18-1.lu:9:22
  |
9 |     const Y = double(x)
  |               --------- while evaluating `main`, entered here
  |                      ^ must be comptime-known
  |
  = note: a `comptime fn` runs during compilation: every argument must be a literal, a `const`, a
    type, or the result of another comptime call.
```
</details>

<details>
<summary>Exercise 18-2 — [§18.1](../ch18.md#18.1)</summary>

**Exercise 18-2** *(comprehension · wolf)* — Chapter 3 taught you what
`2147483647 + 1` does at runtime. Predict what it does inside a
`comptime fn`, and predict the decision the diagnostic cites:

```wolf
comptime fn brim() -> i32 {
    let big: i32 = 2147483647
    big + 1
}
```

Solution: the same rule, moved earlier — what would trap at runtime is
a compile error at comptime, and the diagnostic cites X3, the one
checked-arithmetic semantics for every profile and now every phase:

```console
$ wolf conform-run ./ex18-2.lu
error[E0706]: this `+` on `i32` faults at compile time: 2147483647 + 1 leaves `i32`'s range
 --> ./ex18-2.lu:6:5
  |
6 |     big + 1
  |     ^^^^^^^ checked arithmetic, comptime included
...
9 |     const B = brim()
  |               ------ while evaluating `brim`, entered here
  |               ------ while evaluating `main`, entered here
  |
  = note: checked arithmetic has one semantics everywhere (X3): what would trap at runtime is an
    error at comptime — intended wraparound is spelled `wrapping[T]`, never a mode.
```
</details>

<details>
<summary>Exercise 18-4 — [§18.2](../ch18.md#18.2)</summary>

**Exercise 18-4** *(comprehension · wolf)* — `size_of(Vec2)` for a
struct of two `f64` fields is 16 on every target wolf supports. Predict
the verdict of `const S = size_of(Vec2)` anyway, and then explain why a
number that obvious is refused at comptime.

Solution: E0708 — layout belongs to the code generator, and the
checker refuses to promise a number another phase owns. The obviousness
is the trap: field reordering, padding, and target ABIs make aggregate
layout a codegen fact, and a comptime that guessed would have to be
right forever:

```console
$ wolf conform-run ./ex18-4.lu
error[E0708]: the size of `Vec2` is not resolved until codegen lays it out
 --> ./ex18-4.lu:9:15
  |
9 |     const S = size_of(Vec2)
  |               ^^^^^^^^^^^^^ unresolved until codegen
  |               ------------- while evaluating `main`, entered here
  |
  = note: layout (sizes, offsets) is decided by the code generator, not the type checker; comptime
    can answer for fixed-width primitives today, but not yet for aggregates.
```
</details>

<details>
<summary>Exercise 18-6 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-6** *(spelunking · wolf)* — Run `wolf --explain E0701`
and read the entry in full. It names two distinct reasons a comptime
capability can be refused. Name both, and sort these refusals under
them: a clock read, a network fetch, an environment variable.

Solution: the two reasons are *confinement* (compiling a package must
never act on or read the machine that compiles it) and *determinism*
(the same program and target must produce bit-identical comptime
results on every host). The clock is determinism — two identical
builds must not observe different times. The network fetch is
confinement — the entry's own example is that `wolf add` must never
mean arbitrary code runs with your credentials. The environment
variable is both, and the catalog files it under confinement: it reads
the compiling machine, and it also varies host to host. From the real
entry:

```console
$ wolf --explain E0701
E0701: comptime code reached for ambient IO

Comptime evaluation is hermetically sandboxed (D33): no filesystem, no
network, no environment variables, no clock, no randomness, no FFI —
the intrinsics available at compile time are an explicit allowlist,
and nothing ambient is on it. Each refusal names its category and its
reason: confinement (compiling a package must never act on or read
the machine that compiles it — `wolf add` must never mean arbitrary
code runs with your credentials) or determinism (the same program and
target must produce bit-identical comptime results on every host).
Compute the value at runtime instead; file contents belong in
*declared build inputs* through the package manifest, never in an
evaluator capability.
```
</details>

<details>
<summary>Exercise 18-7 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-7** *(comprehension · wolf)* — Five expression tiles.
Sort each onto the comptime side of the boundary or the runtime side
before running anything: `6 * 7`; a function from a type to a type; a
file read; a clock read; a network fetch. Then check the three you
sorted as refused, with three one-line programs. Do the three
diagnostics give the same reason?

Solution: arithmetic and type-to-type functions are admitted — pure
computation over values the compiler already holds. The file read, the
clock read, and the network fetch are refused, all as E0701, but not
for one reason; each refusal names its own:

```console
$ wolf conform-run ./ex18-7a.lu
error[E0701]: `read_text` reaches the filesystem, which comptime code can never touch
  = note: why it is refused — confinement: a build must not read the machine it runs on — and the
    same source would compile differently on different machines.
$ wolf conform-run ./ex18-7b.lu
error[E0701]: `clock_ms` reaches the clock, which comptime code can never touch
  = note: why it is refused — determinism: two identical builds must not observe different times.
$ wolf conform-run ./ex18-7c.lu
error[E0701]: `net_fetch` reaches the network, which comptime code can never touch
  = note: why it is refused — confinement: `wolf add` must never mean arbitrary code talks to the
    network with your credentials.
```

(Each run also prints the span rendering and the shared hermetic-
sandbox note; the lines above are the ones that differ. The full
outputs are in `ex18-7a.lu` through `ex18-7c.lu`'s runs.)
</details>

<details>
<summary>Exercise 18-8 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-8** *(comprehension · wolf)* — A reader decides budgets
are noise and writes `#[budget(fuel = 0)]` to turn the meter off.
Predict what the compiler does with a *trivial* call under that
attribute — a `comptime fn` that returns `10` and computes nothing.

Solution: the rejection is about the attribute, not the workload.
Budgets are raised, never removed; there is no spelling that disables
one, and the trivial body never gets a chance to demonstrate its
innocence:

```console
$ wolf conform-run ./ex18-8.lu
error[E0709]: a comptime budget cannot be turned off — `fuel = 0` would disable the limit
 --> ./ex18-8.lu:8:14
  |
8 |     #[budget(fuel = 0)]
  |              ^^^^^^^^ budgets are raised, never removed
  |
  = note: the sandbox guarantee (D33) includes bounded evaluation: every budget has a default, a
    per-site override, and a hard ceiling — there is no spelling that removes one.
```
</details>

<details>
<summary>Exercise 18-9 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-9** *(comprehension · wolf)* — Two runaway programs, two
different budgets. Before running, match each to the resource it
exhausts and the E-code it earns:

```wolf
// program A
comptime fn dive(n: int) -> int {
    dive(n + 1)
}
// program B
comptime fn spin() -> int {
    while true {}
    0
}
```

Solution: A recurses, so it hits the *depth* budget (E0704, 256 call
frames); B loops in one frame, so it burns *fuel* (E0702, a step
count). Both diagnostics end with the same shape of help — raise the
budget at the use site — because the compiler cannot tell a runaway
from a computation that is merely large; only you can:

```console
$ wolf conform-run ./ex18-9a.lu
error[E0704]: comptime evaluation recursed past 256 call frames
help: raise the budget here: `#[budget(depth = 512)]`
$ wolf conform-run ./ex18-9b.lu
error[E0702]: comptime evaluation ran out of fuel after 1000000 steps
help: raise the budget here: `#[budget(fuel = 2000000)]`
```
</details>

<details>
<summary>Exercise 18-10 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-10** *(extension (break-it-on-purpose) · wolf)* — Earn E0703 — the
*heap* budget — using only a `while` loop and a `var`, without
tripping fuel first. (You will need to grant fuel to get there.)

Solution — `ch18/ex18-10.lu`: grant a large fuel budget so the loop
lives long enough to exhaust the 65536-cell comptime heap instead:

```wolf
comptime fn flood() -> int {
    var n = 0
    while n < 100000000 {
        n = n + 1
    }
    n
}
fn main() -> !int {
    #[budget(fuel = 100000000)]
    const N = flood()
    if N == 0 { 1 } else { 0 }
}
```

```console
$ wolf conform-run ./ex18-10.lu
error[E0703]: comptime evaluation exceeded its heap budget of 65536 cells
  --> ./ex18-10.lu:13:15
   |
13 |     const N = flood()
   |               ^^^^^^^ the allocation that went over happened here
   |               ------- while evaluating `flood`, entered here
   |               ------- while evaluating `main`, entered here
   |
   = note: the comptime heap is capped so evaluation cannot exhaust the machine compiling the
     program (D33); most overruns are unbounded value growth in a loop.
help: raise the budget here: `#[budget(heap = 131072)]`
```

The order of the two limits is the lesson: budgets are independent
meters, and the first one exhausted names the failure.
</details>

<details>
<summary>Exercise 18-12 — [§18.4](../ch18.md#18.4)</summary>

**Exercise 18-12** *(design)* — The sandbox refuses a file read
(E0701) but the catalog entry points at *declared build inputs* through
the package manifest instead. Draw the line between the two designs:
what exactly does declaring an input buy that an ambient read does not
have? Name the failure the ambient read permits in each of: caching,
cross-machine reproducibility, and auditing a dependency you did not
write.

Solution (discussion): a declared input is part of the build's
identity — it is hashed, so the cache can key on it; it is listed, so
another machine can be handed the same bytes; it is visible, so an
auditor reads the manifest instead of the evaluator's traffic. The
ambient read defeats each in turn: a cache cannot know the file
mattered, so it serves stale artifacts; a second machine has a
different file or none, so the "same" build diverges; and an auditor
must now treat every comptime expression as a potential filesystem
probe, which is the exact posture chapter 24 spends a chapter
dismantling. The refusal is not a missing feature; it is the load-
bearing wall of the caching, reproducibility, and audit stories, and
the package manifest is where the need is threaded instead of through
the evaluator. The distinction to hold onto: *what* is read can be
data; *that* it was read must be declaration.
</details>

## Chapter 21

<details>
<summary>Exercise 21-1 — [§21.1](../ch21.md#21.1)</summary>

**Exercise 21-1** *(comprehension · prose)* — Here is saxpy in C:

```c
void saxpy(double a, const double *xs, double *ys, size_t n) {
    for (size_t i = 0; i < n; i++) ys[i] = a * xs[i] + ys[i];
}
```

Without `restrict`, name the specific possibility the C compiler must
plan for, and the optimization it therefore hesitates on. Then state
what a wolf compiler knows about `saxpy(a, xs, mut ys)` from the
signature alone, and who did the work of establishing it.

Solution: the C compiler must assume `xs` and `ys` may overlap — a
store through `ys[i]` could change some later `xs[j]`, so reordering
and vectorizing the loads requires either a runtime overlap check or
giving up the transform. `restrict` is the programmer *promising*
disjointness, unchecked: get it wrong and the program is undefined. In
wolf, `mut ys` is an exclusive claim and `xs` a shared read — chapter
7's rule — so disjointness is a fact the type system already proved at
every call site. Same fact, different laborer: C trusts the
programmer's word; wolf makes the caller demonstrate it, once, at
compile time.
</details>

<details>
<summary>Exercise 21-2 — [§21.1](../ch21.md#21.1)</summary>

**Exercise 21-2** *(fingers · lupin)* — Type the wolf saxpy and run
it: five elements, `a = 2.0`, `ys` all tens. Predict both printed
values first.

Solution — `ch21/ex21-2.lu`:

```wolf
fn saxpy(a: f64, xs: List[f64], mut ys: List[f64]) {
    var i = 0
    while i < xs.len {
        ys[i] = a * xs[i] + ys[i]
        i += 1
    }
}
```

```console
$ lupin ex21-2.lu
12 20
```

2·1 + 10 and 2·5 + 10, printed the way a whole-valued `f64` prints —
shortest round-trip, so `12` rather than `12.0`. The kernel is
deliberately the same one as 21-1: what runs here is the semantics, on
both machines and on the compiler's release tier alike. The suite
§21.4 cites gates this same shape against naive `clang -O3`; the
numbers on the page stay CI's.
</details>

<details>
<summary>Exercise 21-3 — [§21.2](../ch21.md#21.2)</summary>

**Exercise 21-3** *(comprehension · prose)* — A request handler builds
a parse tree of 10,000 nodes, reads it, and discards it. Count the
allocator interactions — calls into allocate and free machinery — for
(a) malloc discipline with individual `free`, (b) malloc discipline
with one arena library, (c) a wolf region. Then name the cost in (c)
that did *not* disappear and where it went.

Solution: (a) 20,000 — every node allocated and freed retail. (b) on
the order of a few dozen — the arena grabs slabs and frees them
wholesale; nodes are pointer bumps, which is the arena's entire trick.
(c) matches (b) at runtime — bump allocation, one wholesale free at
region end — with the checking moved to compile time: the guarantee
that no node pointer outlives the region is the region checker's
proof, not a code review's hope. What did not disappear: the proof
obligation. C's arena has the same lifetime rule and enforces it with
discipline; wolf's region has it as a type fact. The allocator math is
identical — chapter 8 said so — and the difference is who catches the
escapee.
</details>

<details>
<summary>Exercise 21-5 — [§21.4](../ch21.md#21.4)</summary>

**Exercise 21-5** *(comprehension · lupin)* — The bill and the payout
in one program: `sum_to(n)` adds 1,000,000 to an `i32` accumulator `n`
times. Predict both calls' fates — `sum_to(2000)`, then
`sum_to(3000)` — with the arithmetic that decides them.

Solution: 2000 × 1,000,000 = 2.0 × 10⁹ fits under `i32`'s
2,147,483,647 ceiling; 3000 × 1,000,000 crosses it at iteration 2148:

```console
$ lupin ex21-5.lu
2000000000
ex21-5.lu: trap(overflow): `+` produced 2148000000, outside `i32` — checked arithmetic traps in every profile (X3); spell intended overflow `wrapping[i32]` [arith.checked] at 272..286
$ echo $?
3
```

Every one of those two million additions carried the check that made
the last one honest. What the check *costs* after optimization is a
measured number with a date on it, and §21.4 prints it from CI's own
ledger — the checked-adds exception in the suite's gate is that cost
made explicit — rather than asserting it here.
</details>

<details>
<summary>Exercise 21-6 — [§21.4](../ch21.md#21.4)</summary>

**Exercise 21-6** *(spelunking · lupin)* — From exercise 21-5's trap
line alone: name the decision id it cites, the clause tag it enforces,
and the documented spelling for the program that *wanted* wraparound.
Then state, in one sentence, why this trap firing "in every profile"
is the chapter's honesty rather than the chapter's embarrassment.

Solution: X3 is the decision; `[arith.checked]` the clause;
`wrapping[i32]` the intended-overflow spelling — all three are in the
line, which is the point of trap lines. The one sentence: a language
claiming to beat C while quietly disabling its own safety checks in
release builds would be rigging the race, and X3 is wolf agreeing to
be benchmarked with the checks on.
</details>

<details>
<summary>Exercise 21-9 — [§21.4](../ch21.md#21.4)</summary>

**Exercise 21-9** *(comprehension · prose)* — "Beats naive C, and the
claim is a falsifiable CI gate" is a sentence with a specific
engineering content. Name the three artifacts that must exist for the
claim to be falsifiable rather than promotional, and for each say
whether this edition already prints it.

Solution: a pinned, public benchmark suite — the kernels, their C
twins, and the gate that reads them (this edition prints its verdict
line in §21.4, with the repository path and the date); a variance
discipline that can call a delta noise — medians, mean absolute
deviation, a symmetric gate (the instrument that would put that
discipline in your hands is chapter 20's subject, and this edition
does not carry chapter 20); and a dated, regenerated record wired to
CI so the claim expires when the world changes — the colophon's
toolchain pin and the ledger line §21.4 quotes, which names its
commit and its night. Remove any one and the sentence degrades to
advertising: no suite and it is unmeasured, no variance gate and it
is cherry-picked, no date and it is folklore.
</details>

## Chapter 22

<details>
<summary>Exercise 22-1 — [§22.1](../ch22.md#22.1)</summary>

**Exercise 22-1** *(fingers · lupin)* — Build the two-module project:
an entry file and a `stats/` directory exporting `mean`, with a
private `total` helper the entry never sees. Run it. Then move
`total` into a second file inside `stats/` and state what changes for
the entry file.

Solution — `ch22/metrics/`:

```wolf
// metrics/main.lu
use stats

fn main() -> !int {
    var widths = List[int]()
    (mut widths).push(4)
    (mut widths).push(6)
    (mut widths).push(8)
    print("mean {stats.mean(widths)} of {stats.count(widths)}")
    0
}
```

```console
$ lupin metrics/main.lu
mean 6 of 3
```

Moving `total` to another file inside `stats/` changes nothing
anywhere: files are invisible to importers — the module is the
directory, `use stats` names it whole, and the split is a private
reorganization. That non-event is the design.
</details>

<details>
<summary>Exercise 22-2 — [§22.1](../ch22.md#22.1)</summary>

**Exercise 22-2** *(comprehension · lupin)* — `vault/keys.lu` defines
`pub fn count()`, `pub fn loaded()`, and private `fn secrets()` and
`fn total()`. The entry calls `vault.total()`. Predict the diagnostic — including
whether it says the name does not *exist* — and the exit code.

Solution: E0304, exit 2, and the diagnostic is precise about
existence: the name is there and visibility is the objection. A
resolver that pretended otherwise would send you hunting a typo that
is not one:

```console
$ lupin leak/main.lu
leak/main.lu: E0304: `total` exists in `vault`, but it is private; only `pub`/`pub(pkg)` items are visible across modules (D32) [mod.vis.private] at 49..54
$ echo $?
2
```
</details>

<details>
<summary>Exercise 22-3 — [§22.1](../ch22.md#22.1)</summary>

**Exercise 22-3** *(comprehension · lupin)* — `twice/main.lu` and its
sibling `twice/extra.lu` each define `fn describe()`. Neither file
imports the other. Predict the verdict, and say why "neither imports
the other" is a trap in the question.

Solution: E0302 — "file boundaries create no scopes." The trap is
thinking imports are involved at all: sibling files are not two scopes
that could shadow, they are one module with one namespace, and the
second definition is a duplicate wherever it sits:

```console
$ lupin twice/main.lu
twice/main.lu: E0302: the name `describe` is defined twice in this module (defined again in `twice/main.lu`); file boundaries create no scopes (D32) [mod.dup] at 3..11
```
</details>

<details>
<summary>Exercise 22-4 — [§22.1](../ch22.md#22.1)</summary>

**Exercise 22-4** *(comprehension · lupin)* — The entry imports
`tools` and never mentions it again. Predict: warning or error, and
what the diagnostic offers about the fix.

Solution: a hard error, E0305, and the diagnostic notes the fix is
machine-applicable — deleting the line. Wolf takes the Go position
with Go's justification: an unused import is a dependency edge that
slows every build and means nothing, and a warning would be a request:

```console
$ lupin unused/main.lu
unused/main.lu: E0305: the import `tools` is never used in `unused/main.lu`; an unused import is a hard error (D32), and deleting the line is machine-applicable [mod.use.unused] at 4..9
```
</details>

<details>
<summary>Exercise 22-5 — [§22.2](../ch22.md#22.2)</summary>

**Exercise 22-5** *(comprehension + extension · lupin)* — In
`ch22/tangle/`, `store` imports `index` to log entries and `index`
imports `store` to validate them — each import has a reason, which is
how real cycles are born. Predict the diagnostic. Then perform the
interface-extraction refactor in a copy: move the shared vocabulary
into a third module neither imports from, and run the result.

Solution — before, the cycle drawn whole:

```console
$ lupin tangle/main.lu
tangle/main.lu: E0303: this import completes a cycle: `store` → `index` → `store` (in `tangle/index/index.lu`); imports between modules must form a DAG (D32) [mod.cycle] at 70..80
```

After — `ch22/untangled/` adds `kinds/`, which imports nothing;
`index` now consumes `kinds.classify` instead of calling back into
`store`, and the arrows form a DAG:

```console
$ lupin untangled/main.lu
stored 0
```

The refactor's discipline: the extracted module holds what both sides
*needed from each other* and nothing else. If `kinds` starts importing
things, the tangle is reassembling under a new name.
</details>

<details>
<summary>Exercise 22-6 — [§22.2](../ch22.md#22.2)</summary>

**Exercise 22-6** *(comprehension · prose)* — A library refactor
splits one 900-line module file into four files in the same directory,
moves nothing across module boundaries, and changes no `pub` markers.
List everything that changes for the library's importers, then name
the artifact from §22.2 that would prove your answer
mechanically.

Solution: nothing changes — the import path names the directory, the
module's namespace is the union of its files, and the `pub` surface is
untouched. The proof artifact is the module's export hash, which
`wolf interface` prints: a digest over the `pub` surface alone, which
the split leaves bit-identical. Run it before and after and compare the
`export_hash` line; a private helper moving between files does not
appear in the items list, so it cannot appear in the number. A refactor
you can prove invisible is a refactor you can make on a Friday.
</details>

<details>
<summary>Exercise 22-7 — [§22.3](../ch22.md#22.3)</summary>

**Exercise 22-7** *(comprehension · wolf)* — The `init()` idiom §22.3
retires: a plugin system where each module's `init()` registers a handler
into a global table at startup, in whatever order the linker felt like.
Write the comptime replacement for **four** handlers, with a witness that
fails the build if one goes missing, and say what became of the ordering
question.

Solution — `ex22-7.lu`, run by the compiler because a `comptime fn` is
the compiler's to evaluate:

```wolf
struct Ingest  { rows: int }
struct Report  { rows: int }
struct Purge   { rows: int }
struct Reindex { rows: int }

comptime fn handlers(a: type, b: type, c: type, d: type) -> str {
    "{typeinfo(a).name} {typeinfo(b).name} {typeinfo(c).name} {typeinfo(d).name}"
}

comptime fn expect_four(a: type, b: type, c: type, d: type) -> bool {
    assert(handlers(a, b, c, d).len == 27)
    true
}

fn main() -> !int {
    const HANDLERS = handlers(Ingest, Report, Purge, Reindex)
    const CHECKED = expect_four(Ingest, Report, Purge, Reindex)
    print("{HANDLERS}")
    if CHECKED { 0 } else { 1 }
}
```

```console
$ wolf run ex22-7.lu
Ingest Report Purge Reindex
```

What builds the table: `handlers`, during compilation. When: before the
program exists. What became of the ordering question: it was deleted, not
answered. There is no phase in which two registrations could race, no
link order to depend on, and two builds of this file produce the same
table byte for byte — the determinism the sandbox exists to protect
(chapter 18). Drop `Reindex` from either call and the witness fails the
build with E0710 rather than leaving you a table that is quietly one
handler short, which is the second thing `init()` never gave you.
</details>

<details>
<summary>Exercise 22-8 — [§22.3](../ch22.md#22.3)</summary>

**Exercise 22-8** *(design)* — Import cycles are errors (D32). A
colleague argues the compiler should permit cycles and merely warn,
citing a large codebase where breaking them means touching forty
files. Argue wolf's side using what the rule *buys*, then concede the
strongest point on the other side and answer it.

Solution (discussion): the buy is threefold. Builds: a DAG gives every
module a finish order, so compilation parallelizes and incremental
builds have a frontier — cycles collapse that into a single unit that
rebuilds together forever. Comprehension: a DAG means "what does this
depend on" has an answer that terminates; in a cycle, everything
depends on everything, and the forty files were already one file
wearing forty names. Interfaces: E0303 forced this chapter's refactor
to *name* the shared vocabulary (`kinds`), and named seams are where
documentation, testing, and ownership attach. The strongest counter is
real: retrofitting a DAG onto a tangled codebase is expensive, and a
warning would let teams migrate gradually. The answer is the one wolf
gives everywhere: gradual enforcement of a structural rule converts it
into folklore — the warned-about cycle outlives its excuse, new code
grows onto it, and the migration never happens. The forty-file cost is
paid once; the cycle's cost is paid on every build and every read,
indefinitely, by people who did not create it.
</details>

## Chapter 23

<details>
<summary>Exercise 23-1 — [§23.1](../ch23.md#23.1)</summary>

**Exercise 23-1** *(comprehension · prose)* — The manifest §23.1
prints, which is a real file in the book's fixtures:

```wolf
pkg {
    name:    "den/logsearch",
    version: "0.1.0",
    edition: "1",

    deps: {
        rows: { path: "../rows" },
    },
}
```

Answer from the file alone: what may this package do to the machine
that builds it, what may it do to the machine that runs it, and which
of those two answers required reading anything other than this file?

Solution: to the building machine — nothing beyond compilation, and
that answer needed no reading at all: the manifest is data, there is
no script section to audit, and D33 means no other file can smuggle
one in. To the running machine — the manifest declares no
`capabilities`, which is the empty set, so no `net`, no `fs`, no
`env`; chapter 24 covers what enforces that. The second answer needed
one more file, and only one: `../rows`'s own manifest, because
`effective` capabilities are the union over the graph and a dependency
may declare what you did not. `wolf audit` reads that union for you,
which is the mechanical version of this exercise.
</details>

<details>
<summary>Exercise 23-5 — [§23.3](../ch23.md#23.3)</summary>

**Exercise 23-5** *(comprehension · prose)* — An upstream author
force-pushes tag `v1.4.0` of `den/rows` so the same version name now
serves different bytes. Walk the failure: what does your `wolf.sum`
notice, what does the build do about it, and what would you have to run
to tell wolf the change was intended? Then answer the pointed one: what
does `wolf.sum` *not* protect, and who is exposed to that gap?

Solution: the ledger recorded a `b3:` digest over the dependency's
source tree the last time a human fetched it. The next build re-derives
that digest, finds a different number, and refuses with E1506 — "the
bits on disk are not the bits the ledger witnessed". Nothing is
guessed and nothing is trusted except the hash: the tag is a name, the
digest is the content, and the build compares content, which is why a
moved tag is a broken build rather than a silent substitution. Telling
wolf the change was intended is one command, `wolf update`, and the
point of making it a command is that it produces a reviewable diff: the
new digest lands in `wolf.sum` where a human signs off on it.

The gap: `wolf.sum` is a memory, so it protects only what it has
already seen. The first-ever fetcher of a poisoned version has no
recorded digest to compare against — they hash the malicious bytes and
record them as the truth. That is not a flaw in the ledger, it is the
boundary of what a per-project witness can do, and closing it needs a
record the whole world shares rather than one your project keeps.
Content addresses and an append-only record of published versions are
the designed answer, and what a reader can rely on today is the
narrower guarantee: bits cannot change under you without the build
saying so.
</details>

<details>
<summary>Exercise 23-6 — [§23.1](../ch23.md#23.1)</summary>

**Exercise 23-6** *(spelunking · wolf)* — Run `wolf tree` and then
`wolf why` on a project with one dependency, and then run `wolf why` for
a name that is not in the graph. Report all three exit codes, and say
what the third one is for.

Solution — real runs, in the `shelf` fixture §23.1 prints:

```console
$ wolf tree --dir app
capability tree (I13)
den/logsearch 0.1.0 (root) caps=[]
└── rows 1.4.0 caps=[]
effective: []
$ echo $?
0
$ wolf why rows --dir app
den/logsearch (root) -> rows 1.4.0
$ echo $?
0
$ wolf why ghost --dir app
wolf why: `ghost` is not in the resolved graph
$ echo $?
1
```

The third exit code is the one worth having. `wolf why` for a name that
is not there is not a usage error — the command was spelled correctly —
so it is not exit 2; it is a *finding*, and exit 1 is what a finding
gets. That makes both directions scriptable: `wolf why X` succeeding
means X is in your build, and failing means it is not, which is a
one-line check in a pipeline rather than a grep over a tree.
</details>

<details>
<summary>Exercise 23-8 — [§23.3](../ch23.md#23.3)</summary>

**Exercise 23-8** *(design)* — Wolf's determinism has a price: your
users do not receive a dependency's bug fixes until you raise a
minimum by hand. A colleague calls this a security liability and
wants ranges back. Take wolf's side without dodging the liability —
name the mechanism that answers it and the reason auto-upgrading is
the wrong layer for the answer.

Solution (discussion): concede the fact — when versions move only
because a human moved them, a fix you have not asked for is a fix you
do not have. The mechanism that answers it
is advisory tooling above the resolver: an audit that reads your
build list against a vulnerability feed and *tells you* which
minimums to raise turns upgrades into reviewed, attributable diffs —
one line in `wolf.pkg`, one entry in review history. Auto-upgrade
answers at the wrong layer because it converts every upstream publish
into an unreviewed change to your shipped artifact: the same channel
that delivers a fix delivers a compromise (chapter 24's event-stream
is the canonical case — the malicious version arrived as a routine
compatible upgrade nobody read). Determinism does not slow the urgent
fix; raising a minimum is one edit. What it removes is the *silent*
path — and silent is what the attacker was renting.
</details>

## Chapter 24

<details>
<summary>Exercise 24-1 — [§24.1](../ch24.md#24.1)</summary>

**Exercise 24-1** *(spelunking · prose)* — The event-stream incident
(2018), from §24.1's sourcing: a maintainer handed a popular npm
package to a volunteer, who shipped a version whose install-time and
runtime code targeted a specific downstream wallet application. List
the three legs the attack stood on — distribution, execution, and
concealment — and for each leg, name the wolf mechanism from this
part of the book that removes it or forces it into the open, with one
sentence on the residue each mechanism cannot remove.

Solution: distribution — a compatible-version publish flowed
automatically to downstreams; MVS (chapter 23) removes the automatic
part, since a new version reaches you when you raise a minimum, as a
reviewable diff. Residue: you can still raise it without reading.
Execution — install scripts ran arbitrary code on every fetching
machine; the covenant deletes the phase (D33: no scripts), and the
comptime sandbox refuses ambient reach at build time (24-4, 24-5
below). Residue: *runtime* malice in code you call remains possible —
which is what capabilities are for. Concealment — the payload hid in
a minified transitive dep nobody read; capability manifests plus
`wolf audit` (see 24-6) make "this dep now wants `net`"
a surfaced diff instead of archaeology. Residue: a malicious payload
*within* already-granted capabilities. The lesson the three residues
teach together: the covenant shrinks the attack surface to the part a
human must still review, and makes that part small enough to review.
</details>

<details>
<summary>Exercise 24-2 — [§24.1](../ch24.md#24.1)</summary>

**Exercise 24-2** *(comprehension · prose)* — Left-pad (2016) took
thousands of builds down without executing a byte of anyone's code.
State what kind of failure it was, why the comptime sandbox is
irrelevant to it — the pointed half of the question — and which
chapter-23 artifacts answer it instead.

Solution: an availability failure — a published name was withdrawn
and every build that resolved it fresh broke. The sandbox is
irrelevant because nothing malicious ran; no amount of execution
policy helps when the failure is *absence*. The answers are chapter
23's: the immutable registry and transparency log (a published
version cannot be unpublished into a lie), the local module cache,
and vendoring for the paranoid tail. Distinguishing "code I must not
trust" from "infrastructure I must not depend on" is the exercise;
conflating them is how teams buy a sandbox and still go down on a
Tuesday.
</details>

<details>
<summary>Exercise 24-3 — [§24.1](../ch24.md#24.1)</summary>

**Exercise 24-3** *(comprehension · prose)* — `build.rs` is the
mechanism §24.1's third paragraph indicts. Name three legitimate jobs
build scripts do in the Rust ecosystem, and for each, the covenant-
compatible replacement this part of the book offers. Then name the
job for which the honest answer is "v1 cannot vendor that" (§24.4's
subject).

Solution: generating code from a schema — comptime evaluation over the
schema *as a declared build input* (chapter 18's refusal note points
there, and the package manifest is where the declaration lives).
Discovering platform facts —
target metadata in the manifest and comptime conditionals over the
declared target, not probes of the build host. Compiling and linking
a bundled C library — the declarative recipe layer for the common
shapes, and pre-built artifacts through the membrane (chapter 9) for
the rest. The honest residue: the autotools-shaped dependency whose
build is itself a Turing-complete configuration program — ./configure
logic cannot be declared, only executed, and v1 refuses to execute
it. That refusal is priced in §24.4, not hidden.
</details>

<details>
<summary>Exercise 24-4 — [§24.2](../ch24.md#24.2)</summary>

**Exercise 24-4** *(comprehension · wolf)* — The dependency that
phones home at build time, spelled as directly as wolf's syntax
allows. Predict the E-code and *which of the catalog's two refusal
reasons* the diagnostic will cite (they differ by capability —
chapter 18 sorted them):

```wolf
comptime fn latest_ad() -> str {
    net_fetch("https://deps.example.test/banner")
}
```

Solution: E0701, and the **confinement** reason — the diagnostic's first
note says so in those words, and names the scenario outright: `wolf add`
must never mean arbitrary code talks to the network with your
credentials. It arrives from the type checker, at the call inside the
`comptime fn`, with a second span at the `const` that entered the
evaluation. The sample is `ex24-4.lu` and CI checks it as a
`fail(E0701)` with a reviewed snapshot, so the exact text is verified
even where it is not printed.

The covenant is not a policy document; it is this rejection, emitted
before anything runs.
</details>

<details>
<summary>Exercise 24-5 — [§24.2](../ch24.md#24.2)</summary>

**Exercise 24-5** *(comprehension · wolf)* — The build step that reads
your CI secrets: `env_var("CI_DEPLOY_TOKEN")` inside a `comptime fn`.
Predict the refusal reason this one cites — and note before running
that 18-6 filed environment reads under one reason, while the
diagnostic gives this capability a compound answer.

Solution — the environment gets both barrels: the note reads
"determinism and confinement", because environment contents differ per
machine *and* may hold secrets. That is the compound answer the stem
warns about: chapter 18's exercise 18-6 asks a reader to sort a clock
read, a network fetch, and an environment read under the catalog's two
categories, and the catalog files the environment under confinement
while the diagnostic for this capability names both. Both are right
about different things, which is worth noticing: the category is what
the rule is *for*, and the note is why this particular intrinsic is
refused. The sample is `ex24-5.lu`, checked as a `fail(E0701)` with its
own snapshot.

The scenario this kills is the one event-stream normalized: code you
did not read, running at build time, in possession of what your CI
knows. Here that code does not get to run at all.
</details>

<details>
<summary>Exercise 24-6 — [§24.3](../ch24.md#24.3)</summary>

**Exercise 24-6** *(comprehension · wolf)* — Take a project with one
dependency whose manifest declares no capabilities, record the world with
`wolf update`, then edit the dependency's manifest to declare `net` and
run `wolf audit --ci`. Report the exit code and the line that produced
it. Then say which artifact held the *previous* answer, and what would
have happened if that artifact had been refreshed first.

Solution — the walkthrough, in a project shaped like §23.1's:

```console
$ wolf update --dir app
wolf update: wolf.sum refreshed (1 entry)
$ wolf audit --ci --dir app
capability tree (I13)
den/logsearch 0.1.0 (root) caps=[]
└── regex 2.2.0 caps=[net]
effective: [net]
wolf audit: `regex` ACQUIRES capability `net` (was not in wolf.sum)
wolf audit: capability acquisition detected — refusing (--ci)
$ echo $?
1
```

Exit 1, from the ACQUIRES line: a text-matching library now wants the
network, and `--ci` treats acquisition as a finding rather than news.
The artifact holding the previous answer is `wolf.sum` — its last field
is the capability set, recorded the last time a human accepted the
world. That is why the ordering in the tool is not an accident: `wolf
audit` reports the diff *before* any verb rewrites the ledger, so
refreshing first destroys the evidence. Run `wolf update` before you
audit and the ledger now says `caps=net`, the diff is empty, the gate
passes, and the only trace of the change is a line in a file you did not
read. The gate is only a gate while the ledger is behind you.

The question the failing build forces is not "is this library
malicious" — that has no answer you can reach — but "what, concretely,
will you do with `net`". The upgrade will have a plausible reply ready,
and the most plausible one ("it downloads updated Unicode tables") is
exactly the one to refuse: it moves data acquisition from publish time,
where the tables are baked into a hashed artifact anyone can audit, to
run time on your machines with your network.
</details>

<details>
<summary>Exercise 24-7 — [§24.4](../ch24.md#24.4)</summary>

**Exercise 24-7** *(design)* — Pick the strongest real case against
the covenant: a widely-needed C library whose build is a thicket of
`./configure` feature detection (the class §24.4 names as what v1
cannot vendor). The maintainer of a wolf wrapper asks for "one
escape hatch — a sandboxed build script, network off, fs jailed to
the package directory." Argue the refusal, then state what the
covenant's answer costs this maintainer in practice and why the line
holds anyway.

Solution (discussion): the sandboxed-script hatch fails on three
grounds. Precedent — the moment one package may run a jailed script,
every audit answer degrades from "packages cannot run build code" to
"packages cannot run build code except the ones that can," and the
exception list becomes the attack surface. Fidelity — a jail tight
enough to keep the covenant's promises (no host probes, no
environment reads) breaks `./configure` anyway, because host-probing
is that program's entire method; the hatch would be both dangerous
and useless. Determinism — feature detection's *output* depends on
the build host by design, which is the reproducibility hole D33
closed. The real cost lands on the maintainer: they must pre-build
artifacts per target (chapter 9's membrane consumes them), or
translate the feature matrix into declared target metadata by hand —
genuine, unglamorous work, borne by the few. The line holds because
the alternative distributes a worse cost to everyone else: every
consumer of every package re-auditing what build-time execution
might do. The covenant prices the pain onto the package that has the
exotic build, and keeps the ecosystem's default trustable by
reading a manifest.
</details>

<details>
<summary>Exercise 24-8 — [§24.3](../ch24.md#24.3)</summary>

**Exercise 24-8** *(comprehension · prose)* — "It's only a
dev-dependency" — a teammate waves through a test-helper package
whose new version adds comptime code, on the grounds that it ships
nothing to production. Locate the two errors, using this chapter and
one fact from chapter 18.

Solution: error one — build-time compromise does not care about
shipping: comptime code in any dependency evaluates inside your
build, on your CI, where the credentials live (the sandbox is what
stands between them, and 24-5 showed it holding — but the *audit*
question "why does a test helper now need comptime at all" remains a
human's to ask). Error two — test code runs with your project's full
runtime capabilities every time CI executes the suite, and its
output gates merges; code that can fake a green check ships things
to production without ever being in the artifact. The chapter-18
fact that anchors both: comptime is ordinary wolf evaluated by the
compiler — there is no "harmless phase," only phases with different
blast radii. Dev-dependencies are dependencies; the qualifier names
their schedule, not their trust level.
</details>

## Chapter 26

<details>
<summary>Exercise 26-1 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-1** *(fingers · wolf)* — Build `count` as printed and run
it. Then put a tab in the middle of `one.txt`'s first line and predict all
three numbers before running it again.

Solution — `ex26-1.lu`, which is the chapter's `tally` with the tab in
place of a space. Predict: nothing moves. A tab is a word separator in
exactly the way a space is (both are arms of the same `match` pattern), and
it is one byte, as a space is:

```console
$ lupin ex26-1.lu
       2       6      31 one.txt
```

Two lines, six words, thirty-one bytes — the same three numbers §26.1
prints. The exercise is worth doing because the prediction is the whole
point: `32 | 9 | 10` is one arm, so the three characters are one concept,
and a reader who has understood that arm knows the answer without running
anything.
</details>

<details>
<summary>Exercise 26-2 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-2** *(comprehension · lupin)* — `tally` counts a word every
time it crosses from *between* to *inside*. Predict `lines`, `words`, and
`bytes` for the text `"a  b\n\nc"` — two spaces, a blank line, no
trailing newline — and name which of the three people get wrong.

Solution — `ex26-2.lu`. The answer is `2 3 7`:

```console
$ lupin ex26-2.lu
       2       3       7 -
```

Seven bytes: `a`, two spaces, `b`, two newlines, `c`. Three words: the two
spaces are one gap, not two, because the second one finds `inword` already
false. And two lines, which is the number people get wrong — there are
three *rows of text* and only two newlines, and this counter counts
newlines. `wc` does the same thing for the same reason, and the reason is
that a "line" without a terminator is a judgment call while a newline is a
byte.
</details>

<details>
<summary>Exercise 26-3 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-3** *(extension · wolf)* — Give `count` a bytes-only mode: a
second row function that prints the byte column alone, and a `bool` at the
top of `main` that chooses between them. Then say what the same option
costs in the C twin, and count the lines.

Solution — `ex26-3.lu`. The wolf side is four lines: a `bytes_row`
function and one `if` at the call site.

```console
$ lupin ex26-3.lu
      31 one.txt
```

The C twin costs about the same *for the printing* — a second `row`-like
function is four lines there too — and then costs more for the choosing,
because the flag has to reach `main`'s loop from wherever it was decided.
In our twin the choice is a local `int`, so it is one more declaration and
one more `if`: call it six lines against four. That is not an interesting
win, and this exercise exists to make the point that most of the difference
in this chapter's totals is not in the parts that print things.
</details>

<details>
<summary>Exercise 26-4 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-4** *(comprehension · wolf)* — Narrow `count_file`'s row to
`Tally ! {not_found, denied}` and predict the diagnostic's code and the
tags it names, before running it. Then write the full row out by hand and
check that it and `-> !Tally` accept the same program.

Solution — `ex26-4.lu`. The code is E0602, and the tags it names are the
two that are left: `io` and `utf8`. The prediction to get right is that
the diagnostic reports what is *missing* rather than what is present, and
that it prints the corrected row for you:

```console
$ wolf conform-run ./ex26-4.lu
error[E0602]: this can also fail with `io`, `utf8`, which `count_file`'s row does not include
 --> ./ex26-4.lu:7:16
  |
6 | fn count_file(name: str) -> Tally ! {not_found, denied} {
  |                          ------------------------------ the receiving row is declared here
7 |     let text = fs_read_text(name)?
  |                ^^^^^^^^^^^^^^^^^^^ the missing tags escape here
  |
  = note: rows compose by union: `?` re-tags errors into the wider row by injection — there is no
    conversion to write, only tags to admit.
help: extend the row with `io`, `utf8`
  |
6 | fn count_file(name: str) -> Tally ! {not_found, denied, io, utf8} {
  |
```

Write the four tags out and the program compiles; `-> !Tally` compiles the
same program, because the inferred row *is* the union the `help:` line
printed. The choice between them is documentation, not semantics: spell the
row when the set is part of your interface, and infer it when the function
is internal and the caller is going to widen it again anyway.
</details>

<details>
<summary>Exercise 26-5 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-5** *(spelunking · wolf)* — Add a third name that does not
exist, run the program, and read the exit status. Then read the E0602 note
above in full and explain, in two sentences, why the C's `-1` needs a
convention and a row does not.

Solution: the run reports the missing name on standard error, prints the
rows for the two files that opened, prints the total, and exits 1:

```console
$ wolf build count.lu && ./count
       2       6      31 one.txt
       1       1       6 two.txt
count: cannot open gone.txt
       3       7      37 total
$ echo $?
1
```

The complaint lands between the last file row and the total, which is where
the name was in the list — the two streams are separate but they are written
in program order.

The two sentences: `-1` needs a convention because it is an ordinary value
of the return type, so the *only* thing that makes it mean "failure" is an
agreement between the author of `count` and the author of `main` — an
agreement no part of the program states and nothing checks. A row needs no
convention because the failure is not a value of the success type at all:
`Tally ! {…}` is a different type from `Tally`, the compiler will not let a
caller read one as the other, and E0602 is that rule being enforced across
a call boundary rather than remembered across one.
</details>

<details>
<summary>Exercise 26-6 — [§26.5](../ch26.md#26.5)</summary>

**Exercise 26-6** *(design)* — `count` reads each file whole. Sketch the
version that does not: `fs_open`, a loop of `fs_read` over fixed-size
chunks, and a state machine that survives across chunk boundaries. Name
the one thing that gets harder, and say whether you would pay one call to
avoid it.

Solution (discussion): the shape is `let fd = fs_open(name)?`, then a loop
of `fs_read(fd, 8192)` until it answers the `eof` tag, feeding each chunk
to a `tally` that takes the running `Tally` and the `inword` flag as
parameters and returns both — because the state machine's whole point is
that it carries state across bytes, and now the bytes arrive in batches.
`defer fs_close(fd)` on the line after the open, so the descriptor closes
on every path out.

The thing that gets harder is not the state machine. It is the chunk
boundary: `fs_read` hands back a `str`, a `str` is UTF-8, and a fixed-size
read can land in the middle of a multi-byte code point. The whole-file read
never has that problem because a file is a whole document; a chunked reader
either has to be handed bytes rather than text, or has to keep the tail of
each chunk until the next one completes it.

Would we pay one call to avoid it? For `count`, honestly, yes — the version
in the chapter is the one worth writing first, and the chunked version is
worth writing the day somebody points a 40-gigabyte file at it. The general
answer is the one §26.5 gives: know which of the two you copied.
</details>

## Chapter 27

<details>
<summary>Exercise 27-1 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-1** *(fingers · lupin)* — Add `%` to the dispatch. Then
predict what your arm does for `7 0 %` before you run it, and say whether
you had to write anything the `/` arm did not already show you.

Solution — `ex27-1.lu`. Two edits: `37` joins `is_operator`'s list of
bytes, and a `37 =>` arm joins the `match` with the same zero guard the
`47` arm has:

```wolf
                37 => {
                    if b == 0 { return DivZero }
                    a % b
                },
```

```console
$ lupin ex27-1.lu
2
error: division by zero
```

`17 5 %` is 2. `7 0 %` returns `DivZero`, and the prediction to get right
is that it has to be *written* — a modulo by zero is the same defined fault
division by zero is, so the guard is not optional, and nothing about `%`
made a new kind of problem. That is the answer to the second half: no, the
`/` arm showed you everything. Adding an operator to this calculator costs
one byte in `is_operator` and one arm.
</details>

<details>
<summary>Exercise 27-2 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-2** *(comprehension · lupin)* — `eval` returns `Empty` for
both an underflowing operator and an expression that leaves two values on
the stack. Predict the output for the three lines `3 +`, `3 4`, and
`3 4 + 5`, and then argue whether one tag for two situations is the same
mistake §27.1 accused the C of making.

Solution — `ex27-2.lu`. All three are `Empty`:

```console
$ lupin ex27-2.lu
error: the stack does not hold two operands
error: the stack does not hold two operands
error: the stack does not hold two operands
```

`3 +` underflows: one operand, two wanted. `3 4` and `3 4 + 5` both finish
with two values on the stack, so `stack.len != 1` fires.

Is that §27.1's mistake? No, and the distinction is worth being precise
about. The C's fault is that `0.0` from a failed `pop` is *indistinguishable
from a successful answer* — the error and the success share a
representation, and a caller who forgets to look cannot tell them apart.
Here the error is a tag: a caller cannot read it as a number, cannot forget
it, and cannot get an answer out of a failed line. The complaint against
this program is a smaller one — the message is imprecise, because one tag
is doing two jobs and its text has to cover both. That is a wording
problem with a wording fix, and 27-3 is the fix.
</details>

<details>
<summary>Exercise 27-3 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-3** *(extension · lupin)* — Give `Empty` a payload: which
operator ran out of operands, and how many it found. You will have to
change the row, the two `return`s, and one match arm — say what told you
each one.

Solution — `ex27-3.lu`. A second payload struct, `Short { op: str, found:
int }`, and `Empty` becomes `Empty(Short)` in `eval`'s row:

```console
$ lupin ex27-3.lu
error: `+` wanted two operands and found 1
error: `end of line` wanted two operands and found 2
5
```

What told you each edit: the row, because the tag's spelling changed and a
row lists spellings; the `return`s, because `return Empty` no longer
typechecks once `Empty` carries a payload, and the compiler names each one;
and the match arm, because `Empty =>` no longer binds anything the handler
can print and `Empty(s) =>` does. Three edits, all three demanded by a
type — which is the whole reason the row is written down.

Note the last-line case borrowing the word `op` for `"end of line"`. That
is honest but slightly forced, and it is a real design question: two
different failures are still sharing a tag. Splitting them into
`Underflow(Short)` and `Leftover(int)` is one more row entry and one more
arm, and it is the version to write if this were a real calculator.
</details>

<details>
<summary>Exercise 27-4 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-4** *(comprehension · lupin)* — Feed it `007` and `-0` and
`- 3`. Predict all three results before running, then explain which of the
three is handled by `strip_prefix` and which by `words()`.

Solution — `ex27-4.lu`:

```console
$ lupin ex27-4.lu
7
0
error: the stack does not hold two operands
```

`007` is 7 — the digit loop multiplies by ten and adds, and leading zeros
add nothing. `-0` is 0: `strip_prefix("-")` takes the sign, the body `0`
parses to zero, and `0 - 0` is 0 (there is no negative zero in `int`, which
is one of the quiet advantages of not being `double`). `- 3` is the
interesting one: `words()` cut it into two tokens, so the `-` is an
operator with nothing under it, and the answer is `Empty`.

So: `strip_prefix` handles the sign that is *attached*, and `words()`
decides what "attached" means. That division is the whole reason the wolf
column needs no `ungetch`, and this exercise is the smallest program that
shows it.
</details>

<details>
<summary>Exercise 27-5 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-5** *(spelunking · the C twin)* — Take a census of `broken`
in `rpn.c`: count the places that can set it and the places that read it,
and write down the line numbers. Then take the same census of the wolf
column's failure surface — where a tag can be produced, and where one is
handled. Say what the two ratios tell you.

Solution. In `samples/contrast/rpn.c`, `broken` is declared at line 35 and
raised at five sites: 43 (`push` overflow), 53 (`pop` underflow), 69
(`ungetch` overflow), 130 (division by zero) and 146 (unknown command) —
three of them in functions that have no other way to report anything. It is
read once, at line 136, in the `'\n'` arm, and cleared once at 142 so the
next line starts fresh. Five raises, one read, and the read is more than
eighty lines and three functions away from the furthest raise.

In the wolf column, a tag is produced at nine `return`s across `number` and
`eval`, and every one of them arrives in exactly one place: the `match err`
in `main`. Nine produce sites, one handle site, no reset — because there is
no state to clear, the failure having been the value.

The ratios look similar and they are not the same fact. The C's one read is
a *choice* — it is where the author decided to look, and the compiler would
have been equally happy with none. The wolf column's one handler is a
*requirement*: `eval` returns `int ! {…}`, so `main` cannot get an `int`
out of it without either handling the row or propagating it, and if the row
grows a tag the handler stops compiling. One read by convention, one read by
type.
</details>

<details>
<summary>Exercise 27-6 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-6** *(extension · lupin)* — Add two stack words that are not
operators: `dup` duplicates the top value, `swap` exchanges the top two.
Neither touches `number` or the `match`. Predict what `7 2 swap -`
evaluates to before you run it.

Solution — `ex27-6.lu`. Two branches ahead of the `!is_operator(tok)`
test, each with the length guard the operators already use:

```console
$ lupin ex27-6.lu
9
-5
```

`3 dup *` is 9. `7 2 swap -` is `-5`, and the prediction to get right is
the direction: `swap` leaves `2` under `7`, so the subtraction is `2 - 7`.
Neither word needed a new tag, a new payload, or a change to the dispatch,
because a stack word that only moves values around cannot fail in a way the
`Empty` tag does not already cover.
</details>

<details>
<summary>Exercise 27-7 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-7** *(design)* — Wolf's stack holds `int` and K&R's holds
`double`. Argue the other choice: what would the wolf column have to give
up to work in `f64`, what would it gain, and where in this chapter would
the text have to change?

Solution (discussion): what it gains is obvious and real — `7 2 /` becomes
3.5, and a calculator that cannot divide is a strange calculator. What it
gives up is `number`. The digit loop that ends `n * 10 + (b - 48)` produces
an `int` and there is no conversion from that `int` to an `f64`, so a
floating-point `rpn` cannot reach its operands through the same scan: it
needs a parser that builds a mantissa and an exponent, or a library
function that turns text into a float. That is a substantially bigger
`number` — a fractional part, an optional exponent, and a decision about
what to do with `1e400`.

The text that would change: §27.2's paragraph about the stack's type;
§27.3's claim that ten lines of digit loop is a fair trade; and §27.4's
closing accounting, which currently says the `int` costs exactly one
division. The comparison with `getop`'s 28 lines would also get closer,
because most of what makes `getop` long is that it is a *number* parser
rather than a tokenizer.

The defensible position is the one the chapter takes, stated as a choice
rather than as a virtue: this calculator does integer arithmetic, and if
you want fractions you are writing the parser.
</details>

<details>
<summary>Exercise 27-8 — [§27.5](../ch27.md#27.5)</summary>

**Exercise 27-8** *(design)* — Sketch the REPL: read a line, evaluate it,
print the answer, and stop at end of input. `read_line()` returns
`str ! {eof, io, utf8}`, so name the loop's exit condition, and then say
what the calculator would have to remember between lines for `x 3 +` to
mean anything — and what shape that memory wants to be.

Solution (discussion): the loop is
`let line = read_line() else |_| { break }` inside a `while true`, and the
exit condition is the `eof` tag — which is the point worth noticing, because
"input ended" is a failure tag here rather than a sentinel line or a null.
It is the §27.1 argument arriving from the other direction: end of input is
not an empty string, so it cannot be confused with one.

For `x 3 +` to mean anything the calculator needs a name-to-value table
that outlives a line. The shape that wants is a `Map[str, int]` threaded
through `eval` as a `mut` parameter — one more parameter, one more branch in
the token loop (a token that is neither an operator nor a number is a
variable), and one more tag (`Unbound(Bad)`) for a name nobody has assigned.
Assignment wants a syntax decision: `x 3 !` in the K&R style, or a leading
`let`, or `=` as an operator that pops a value and a name.

The honest note about *this* toolchain: `Map` is a name the prelude knows
and neither implementation carries a signature for, so the table above is a
sketch rather than a program. An association list — two parallel `List`s and
a linear scan — is what a reader could build today, and for a calculator's
variables a linear scan over a dozen names is not the part that runs
slowly.
</details>

## Chapter 28

<details>
<summary>Exercise 28-1 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-1** *(fingers · lupin)* — Build the tree as printed and run
it. Then add a word that sorts before `moon` and one that sorts after
`wolf`, and predict where each appears in the output before you run it.

Solution — `ex28-1.lu`, with `zebra apple` added to the text. The
prediction: `apple` first and `zebra` last, whatever order they arrive in.

```console
$ lupin ex28-1.lu
   1 apple
   1 moon
   1 runs
   3 the
   1 watches
   2 wolf
   1 zebra
```

`zebra` was read before `apple` and prints after it, which is the point:
the in-order walk reports the tree's *shape*, and the shape came from
`strcmp`-style comparison rather than from arrival. `apple` became the
left-most leaf and `zebra` the right-most, and neither insertion touched a
node it did not have to.
</details>

<details>
<summary>Exercise 28-2 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-2** *(comprehension · lupin)* — `add` compares with
`w < n.word` on `str`. Predict the order of `Wolf`, `wolf`, `WOLF`, and
`wolfs` in the output *without* the `.lower()` call, and say which two of
the four end up as one node once `.lower()` is back.

Solution — `ex28-2.lu`. Four distinct words, four nodes, ordered by bytes:

```console
$ lupin ex28-2.lu
   1 WOLF
   1 Wolf
   1 wolf
   1 wolfs
```

`WOLF` before `Wolf` before `wolf` because `str` compares byte by byte and
ASCII puts every capital before every lowercase letter — `O` is 79, `o` is
111. `wolf` before `wolfs` because a prefix sorts before what extends it.
Put `.lower()` back and three of the four collapse into one node with a
count of 3, and `wolfs` stays where it is: it is a different word in every
case folding.

The lesson worth keeping is that byte order is not alphabetical order, and
the book says so plainly in chapter 2 rather than pretending otherwise.
`lower()` is enough for ASCII words and it is not a collation.
</details>

<details>
<summary>Exercise 28-3 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-3** *(extension · lupin)* — Add a `-n` mode: print the words
in descending order of count instead of alphabetically, with ties broken
alphabetically. The tree is already sorted by word, so the shape of the
answer is a second pass — say what you collect on the first pass, and what
it costs in lines.

Solution — `ex28-3.lu`. The first pass is `collect`, an in-order walk that
pushes each node's word and count onto two parallel `List`s — so it arrives
already sorted alphabetically, which is what makes the tie-breaking free.
The second pass is a selection walk: `next_after` finds the largest
remaining `(count, word)` after the last one printed, which needs no
mutation of either list.

```console
$ lupin ex28-3.lu
   3 the
   2 wolf
   1 moon
   1 runs
   1 watches
```

The cost is 35 lines of code: six for `collect`, sixteen for `next_after`,
and thirteen for the driving loop and its two lists. That is a real number
and it is bigger than it should be, for a reason worth knowing: the
obvious implementation marks each printed entry in a `List[bool]` and
skips the marked ones, and writing a single
element of a list is not something this toolchain does — so the selection
carries a cursor (`the last pair printed`) instead of a mark. The cursor
version is arguably the better program anyway, since it allocates nothing
per iteration, but it is not the one a reader would reach for first.
</details>

<details>
<summary>Exercise 28-4 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-4** *(comprehension · lupin)* — Delete the `is_empty()` guard
in `walk` and predict the exact failure: which trap kind, which exit code,
and at which of the two `walk` calls. Then check it.

Solution — `ex28-4.lu`, with `walk(n.left[0])` unguarded. The prediction:
a `bounds` trap, exit 3, at the *left* call — and it fires on the first
leaf the walk reaches, before anything is printed at all.

```console
$ lupin ex28-4.lu
ex28-4.lu: trap(bounds): index 0 is outside a collection of 0 element(s) [mem.ub.defined] at 708..717
$ echo $?
3
```

The C twin's equivalent mistake — dropping `if (p != NULL)` from
`treeprint` — dereferences a null pointer, which is undefined behavior: it
may segfault, it may print garbage, and on some targets it may appear to
work. Here it is a defined fault with a kind, a clause tag, and a byte
span, and it happens at the same place every time. That is the difference
the whole book is about, arriving in a two-line function.
</details>

<details>
<summary>Exercise 28-5 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-5** *(spelunking · the C twin)* — Count, in `wordtree.c`,
every line that would disappear if `malloc` could not fail, and then every
line that would disappear if the program never had to free. Give both
numbers and say which of the two the `region` brace replaced.

Solution. Counting code lines only, in `samples/contrast/wordtree.c`:

*If `malloc` could not fail* — 16 lines go. `addtree`'s two null tests with
their `nomem` sets, early returns and the half-built `free(p)` come to 10
(lines 50–59); the `nomem` declaration is 1 (line 39); `main`'s
out-of-memory branch is 5 (lines 153–157). What survives is `talloc`,
`dupstr`, and the two assignments that call them, because the allocation
still has to happen.

*If the program never had to free* — 10 lines go: `treefree` is 9 (lines
85–93) and its call in `main` is 1 (line 160). Note that the `free(p)`
inside `addtree` is counted in the first number rather than this one; it is
there because the *failure* path has to clean up, not because the program
ends.

Both together are 26 of the file's 115 code lines, and `talloc` and
`dupstr` — 12 more — exist only because the memory has to come from
somewhere nameable.

Which did the brace replace? The second, exactly: `region words { … }` is
`treefree`, and the ten lines are the ten it replaced. The first sixteen
were not replaced by anything — they were removed by the fact that a wolf
allocation failure is not a value a program is handed, which is a different
argument and a stronger one. The brace is the visible half. The invisible
half is bigger.
</details>

<details>
<summary>Exercise 28-6 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-6** *(extension · lupin)* — Take the tree's census: write
`nodes` and `depth` and print both after the walk. Then multiply the node
count by two and say what that number is in the C column, and what it is
in the wolf one.

Solution — `ex28-6.lu`. Two recursions in the same shape as `walk`:

```console
$ lupin ex28-6.lu
   1 moon
   1 runs
   3 the
   1 watches
   2 wolf
5 nodes, depth 3
```

Five nodes, depth 3 — the tree is not balanced and nothing balances it,
which is true of K&R's as well; feed either program a sorted word list and
you get a linked list with a tree's memory layout.

Ten is the number. In the C column it is the count of `malloc` calls (two
per node: one for the `struct tnode`, one for `dupstr`'s copy of the text)
and therefore also the count of `free` calls `treefree` has to make, in the
right order. In the wolf column ten is not a count of anything the program
does: the nodes and their strings are allocations in the region, the region
frees once, and the only place the number ten appears is in an exercise
about the other column.
</details>

<details>
<summary>Exercise 28-7 — [§28.5](../ch28.md#28.5)</summary>

**Exercise 28-7** *(design)* — K&R's `addtree` returns the new subtree
root; wolf's `add` takes `mut n` and returns nothing. Both are answers to
"how does a recursive insert report where the tree went." Argue which is
easier to get wrong, and then say what the wolf version does about the
empty tree that the C version does not have to.

Solution (discussion): `addtree`'s convention is `p = addtree(p, w)` at
every call site, including the two recursive ones — `p->left =
addtree(p->left, w)` and its mirror. It goes wrong in one specific way,
and the way is silent: write `addtree(p->left, w)` without
the assignment and the program compiles, runs, and quietly loses every word
that would have become a new left child. Nothing in C's type system objects
to a discarded return value.

`add`'s convention cannot be dropped, because there is no return value to
drop. Passing `mut n.left[0]` is a claim on that place, spelled at the call
site, and the mutation lands where the caller can see it was going to. What
it costs is that the claim is *visible* — `mut` at every recursive call, and
a reader who wants the mutation surface of this program greps for one word.
That is X1's whole argument, and this is a program small enough to check it
on.

What the wolf version has to do about the empty tree: `addtree` handles it
for free, because `NULL` is a `struct tnode *` and the recursion's base case
and the empty-tree case are the same case. `add` cannot be handed a `Node`
that does not exist, so `main` carries `forest`, a `List[Node]` holding
nothing or one thing, and the first word is a `push` rather than an `add`.
That is three lines of `main` the C does not need — and the same three
lines are what make the *children* need no null, so the language is
charging once for something the C column pays for at every dereference.
</details>

## Chapter 30

<details>
<summary>Exercise 30-1 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-1** *(fingers · wolf)* — Build both programs and run them
against the same two files with three patterns of your own. Then run
`wc -l` on both and write down, in one sentence, what you would tell a
colleague who proposed the parallel one for a log directory of four files.

Solution — no new program; the two are `samples/projects/seqgrep/seqgrep.lu`
and `samples/projects/pargrep/pargrep.lu`, both printed in the chapter.

```text
$ wc -l samples/projects/seqgrep/seqgrep.lu samples/projects/pargrep/pargrep.lu
  57 samples/projects/seqgrep/seqgrep.lu
  97 samples/projects/pargrep/pargrep.lu
```

The sentence: for four files, ship the sequential one — the parallel
version costs forty lines and thirty-seven of them are the fan-out being
written out by hand, so the version worth arguing about is the one that
takes its shard count from the input, and that is not the version on the
page.
</details>

<details>
<summary>Exercise 30-2 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-2** *(comprehension · wolf)* — `hits` and `done` are both
`channel[int](0)`. Give `hits` a buffer — `channel[int](64)` — and predict
what happens before you run it. Then run the binary twenty times and count
the report lines each time. Two questions: what can `done.send(1)` do now
that it could not do before, and which line of the collector is the one
that loses the hits?

Solution — one character of the program changes and it stops working.
Twenty runs of the modified binary, counting report lines:

```text
$ for i in $(seq 1 20); do ./pargrep | wc -l; done | sort -n | uniq -c
     11 1
      2 2
      7 4
```

Four is the correct answer and it came up seven times.

What `done.send(1)` can do now: finish first. With `hits` unbuffered, a
task's send blocks until the collector has taken the value, so everything
a task found is already in `found` by the time its completion is sent.
Buffered, the send returns immediately, and a task can queue four hits and
then report itself done while all four are still in the channel.

The line that loses them is `while live > 0`. The collector stops the
moment the fourth completion arrives, and anything still sitting in the
buffer is never received. The report then walks the input and finds `found`
missing most of it.

This program is not in this directory. A sample whose output changes
between runs cannot be a CI sample, and a book that shipped one would be
teaching its own alarm to cry wolf. The measurement above is what the
exercise is for; the program is three seconds of your own editing.
</details>

<details>
<summary>Exercise 30-3 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-3** *(comprehension · wolf)* — Delete `freeze` from the
`needles` binding, leaving `let needles = pattern.split("|")`. Predict
whether the program still compiles before you try it. Then explain, in two
sentences, what the four tasks are allowed to do with `needles` in each
version.

Solution — `ex30-3.lu`. It compiles, and it runs correctly:

```console
$ wolf build ex30-3.lu && ./ex30-3
a.log:1: 06:12 the wolf runs
a.log:3: 07:02 the wolf howls
b.log:2: 08:52 the wolf sleeps
b.log:4: 09:44 the wolf wakes
```

The prediction most readers write down is "it will not compile", and the
useful part of this exercise is being wrong about it. `needles` is bound
with `let` and never written, so the four tasks read a value nobody
mutates and the program is correct — by inspection, and only by
inspection.

The two sentences. With `freeze`, the tasks read an `imm` graph: nothing
anywhere can write it, the compiler knows that, and adding a write
somewhere else in the program is a compile error rather than a race
(§30.4's E1012 is that error). Without `freeze`, the tasks read ordinary
data that happens not to be written today, and the guarantee is a property
of the current text rather than of the type — which is exactly the
distinction the whole of Part 3 is about.
</details>

<details>
<summary>Exercise 30-4 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-4** *(extension · wolf)* — Instrument the collector: print
each index as it arrives, before pushing it. Run the binary twenty times
and count the distinct arrival orders you see; then confirm that the four
report lines never move. Which of the two outputs would you put in a test?

Solution — `ex30-4.lu`, one `print` added to the `hits` arm.

```console
$ wolf build ex30-4.lu && ./ex30-4
arrive 0
arrive 2
arrive 5
arrive 7
a.log:1: 06:12 the wolf runs
a.log:3: 07:02 the wolf howls
b.log:2: 08:52 the wolf sleeps
b.log:4: 09:44 the wolf wakes
```

That is one run. Over twenty, the four `arrive` lines came out in five
different orders — `0 2 5 7` most often, then `0 2 7 5`, `0 5 2 7`,
`0 5 7 2`, `0 7 2 5` — and the four report lines hashed to one value
twenty times out of twenty. Which one goes in a test is therefore not a
matter of taste: the report is a property of the program, the arrival
order is a property of the afternoon.

Index `0` arrives first on nearly every run, which is worth not
over-reading. The first shard's first line matches, and its task has the
shortest path to a rendezvous. That is a bias, not a guarantee, and a test
that assumed it would fail on a loaded machine.
</details>

<details>
<summary>Exercise 30-5 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-5** *(extension · wolf)* — Make the report order-dependent
on purpose: delete both channels and the collector, and have each task
print its own matches directly. Run the binary twenty times and hash the
output. You will get more damage than you predicted — say what the extra
damage is, and then say what you have broken in terms of §30.5's rule
rather than in terms of tasks.

Solution — `ex30-5.lu`. Twenty runs, twenty distinct outputs. Here is one:

```text
$ ./ex30-5
a.loga.log:1: 06:12 the wolf runs
:3: 07:02 the wolf howls
b.log:4: b.log:2: 08:52 the wolf sleeps
09:44 the wolf wakes
```

The extra damage is that the lines are not merely out of order, they are
*torn*: `a.log` from one task and `:3: 07:02 …` from another arrived in
the same line of output. `print` is not atomic across tasks, and four
tasks writing one stream interleave inside a line as readily as between
two.

In §30.5's terms: the four report lines used to be a fact about the
program, and now they are a fact about the run. Nothing was added to the
program to break this — something was removed. The collector was not
overhead; it was the single owner of the output, and a single owner is
what made the output reproducible. The general form of the rule is that
the last stage of a concurrent pipeline should be sequential, and the
cheapest way to obey it is to let exactly one task print.
</details>

<details>
<summary>Exercise 30-6 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-6** *(spelunking · wolf)* — Read the E1012 note in §30.4 in
full. It offers two ways out — build the value completely before freezing,
or keep a mutable `copy` alongside. Say which one `pargrep` uses and what
the other one would cost in a four-task program.

Solution — `pargrep` uses the first, twice, and the shape is visible in
both `freeze region { … }` blocks: everything the value will ever contain
is pushed inside the block, and the block's last expression is the finished
value. The pattern list is built by `split` in one call; the table is
accumulated by a loop over the files and then handed out as a `Table`.
Neither is touched again.

The second way out — keeping a mutable `copy` beside the frozen one — is a
correct answer to a different question, and in a four-task program it is
usually the wrong one. The copy is not shareable, so it cannot cross into
a task, so the only place it can be used is the parent; and the moment
what the parent has and what the tasks have can differ, the program has a
consistency question it did not have before. The cost is not the memory.
It is that "the frozen table" stops being a single noun.
</details>

<details>
<summary>Exercise 30-7 — [§30.5](../ch30.md#30.5)</summary>

**Exercise 30-7** *(design)* — The shard count is a constant. Sketch the
version that takes it from the input: what the ranges become, what the
collector's `live` counter becomes, and what `pargrep` would need from the
language to spell the fan-out in one loop instead of four spawns. Then say
whether four shards on two files was ever the right number.

Solution — the arithmetic is the easy third of it. With `w` shards, the
`k`th range is `(n * k) / w` to `(n * (k + 1)) / w`, which is the same
integer division §30.1 already does and needs no boundary variables at
all; `live` starts at `w` instead of `4`; and the report loop does not
change, because it never knew how many tasks there were. The whole
difference is that three `let b`s become one expression evaluated inside a
loop.

What the language has to give is the loop: `for k in 0..w { s.spawn(fn() {
… }) }`, with each closure capturing its own `k`. That is one construct,
and everything else in the sketch is already spelled.

Whether four was ever right: no, and the honest reason is that nobody
chose it for this input. Four is a plausible default for a machine and a
poor one for two files of four lines, where a single task would finish
before a second one started. The number a real version wants is the
smaller of the worker count and the input's shard count, with a floor of
one — and the interesting part is that the program cannot ask for the
first of those two either. A tool that shards should take `-j` from the
command line, the way `make` does, which makes this exercise's answer one
more argument for the loop.
</details>
