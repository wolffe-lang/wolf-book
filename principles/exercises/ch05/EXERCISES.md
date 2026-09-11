# Chapter 5 — Collections and generics: exercises

Exercises 5-1 through 5-5 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §5.1 — `List`, `Map`, `Set`, tuples

**Exercise 5-6** *(extension · wolf)*. Count lines the way the head
of the chapter counted categories: a `Map[str, int]` and one line of
counting per row. Then print each distinct line once, with its count,
in the order the lines first appeared. The `Map` alone does not give
you that order. Which container does, and why is it not the `Map`?

Solution. `ch05/ex5-6.lu`, which runs under `wolf`:

```wolf
fn seen_at(xs: List[str], s: str) -> int {
    var i = 0
    while i < xs.len {
        if xs[i] == s { return i }
        i += 1
    }
    -1
}
fn main() -> !int {
    let log = """
        howl
        howl
        scratch
        howl
        scratch
        """
    var order = List[str]()
    var counts = Map[str, int]()
    for line in log.lines() {
        if seen_at(order, line) < 0 { (mut order).push(line) }
        counts[line] = (counts[line] else 0) + 1
    }
    for s in order {
        print("{counts[s] else 0:>4} {s}")
    }
    0
}
```

```console
$ wolf run ex5-6.lu
   3 howl
   2 scratch
```

The count is the map alone: `counts[line] = (counts[line] else 0) + 1`
is the whole tally, one line per row, and nothing decides insert versus
update, because `else 0` reads a missing entry as zero and the
assignment inserts it. What the map does not give is the order. `pairs()`
walks the entries in an order the language leaves unspecified, and
nothing in a map remembers which key came first; the `order` list
remembers exactly that and nothing else, at the cost of one `seen_at`
scan per line, and the printing loop walks the list and asks the map.

**Exercise 5-12** *(fingers · lupin)*. Histogram the word lengths of a
two-line block, bars of `#` growing sideways, one row per length. A
`List[int]` indexed by length is the whole data structure — grow it to
fit as you go. One of your rows prints an empty bar. Which, and why
does it print at all?

Solution. `ch05/ex5-12.lu`:

```wolf
fn main() -> !int {
    let text = """
        the moon watches the long ridge
        a wolf runs past the frozen creek
        """
    var tally = List[int]()
    for w in text.words() {
        while tally.len < w.len + 1 { (mut tally).push(0) }
        tally[w.len] = tally[w.len] + 1
    }
    for n in 1..tally.len {
        print("{n:>2} {"#".repeat(tally[n])}")
    }
    0
}
```

```console
$ lupin ex5-12.lu
 1 #
 2 
 3 ###
 4 #####
 5 ##
 6 #
 7 #
```

Length 2: the block has one one-letter word (`a`) and words of three
letters and up, but nothing two letters long. The row prints because
the list was grown by *maximum length seen*, not by lengths seen — the
gap is a real zero, and `repeat(0)` spells it as an empty bar. A
histogram that skipped empty rows would hide the shape it exists to
show.

**Exercise 5-14** *(fingers · lupin)*. A comma-separated ledger, three
rows of `name,cents`. Split each row, table it with format specs, and
end with a total row. What does your program do with a row that has no
comma, and which single spelling in your source decided that?

Solution. `ch05/ex5-14.lu`:

```wolf
fn main() -> !int {
    let ledger = """
        drink,340
        pastry,275
        stew,900
        """
    var total = 0
    for row in ledger.lines() {
        var name = ""
        var cents = 0
        var i = 0
        for field in row.split(",") {
            if i == 0 { name = field }
            if i == 1 { cents = field.to_int() else 0 }
            i += 1
        }
        total += cents
        print("{name:<10}{cents:>6}")
    }
    print("{"total":<10}{total:>6}")
    0
}
```

```console
$ lupin ex5-14.lu
drink        340
pastry       275
stew         900
total       1515
```

A commaless row arrives as one field: `name` takes the whole row,
`cents` keeps its initial 0, and the row costs the total nothing. The
deciding spelling is `var cents = 0` — the initializer *is* the
missing-field policy. If the right answer for your ledger is "refuse
the row instead", that policy has a chapter of its own next.

## Chapter batch

**Exercise 5-7** *(comprehension + extension · lupin)*. An RPN
evaluator is a loop and a stack, and the stack is a `List`. Given the
tokens `3 4 + 2 *`, trace the stack contents after each token on paper,
then run. Then answer from your trace, not from the code: which input
would make `stack.len < 2` true at an operator, and what does your
evaluator do about it?

Solution. `ch05/ex5-7.lu`:

```wolf
fn eval_rpn(tokens: List[str]) -> int ! {Underflow, BadToken} {
    var stack = List[int]()
    for t in tokens {
        if t == "+" || t == "-" || t == "*" || t == "/" {
            if stack.len < 2 { return Underflow }
            let b = (mut stack).pop() else { return Underflow }
            let a = (mut stack).pop() else { return Underflow }
            if t == "+" { (mut stack).push(a + b) } else if t == "-" { (mut stack).push(a - b) } else if t == "*" { (mut stack).push(a * b) } else { (mut stack).push(a / b) }
        } else {
            let n = t.to_int() else { return BadToken }
            (mut stack).push(n)
        }
    }
    if stack.len != 1 { return Underflow }
    (mut stack).pop() else { return Underflow }
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
evaluator returns `Underflow` instead of trapping on `pop`: the error
row is doing bounds-checking's job one level up, where the caller can
do something about it. (The row previews chapter 6; reading it is
enough here.) `pop` carries a row of its own, which is why each call
takes an `else` even under the guard: the compiler checks the call, not
the reasoning around it.

**Exercise 5-11** *(extension · lupin)*. Exercise 2-7 encoded runs
with byte slices; a coder without a decoder is half a tool. Respell
`encode` over `chars()`, growing the output with `"{prev}{run_len}"`,
then write `decode`, and round-trip both `"wwwwoooolf"` and a string
whose runs reach two digits. Make `main` exit nonzero on any
mismatch, so the round trip is a check the machine performs rather
than a claim. Where in `decode` does the two-digit case live?

Solution. `ch05/ex5-11.lu`:

```wolf
fn encode(s: str) -> str {
    var out = ""
    var run_len = 0
    var prev = ' '
    for c in s.chars() {
        if run_len > 0 && c == prev {
            run_len += 1
        } else {
            if run_len > 0 { out += "{prev}{run_len}" }
            prev = c
            run_len = 1
        }
    }
    if run_len > 0 { out += "{prev}{run_len}" }
    out
}
fn decode(s: str) -> str {
    var out = ""
    var cur = ' '
    var seen = false
    var n = 0
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            n = n * 10 + (c as int) - ('0' as int)
        } else {
            if seen {
                for _ in 0..n { out += "{cur}" }
            }
            cur = c
            seen = true
            n = 0
        }
    }
    if seen {
        for _ in 0..n { out += "{cur}" }
    }
    out
}
fn main() -> !int {
    var inputs = List[str]()
    (mut inputs).push("wwwwoooolf")
    (mut inputs).push("aaaaaaaaaaab")
    for plain in inputs {
        let coded = encode(plain)
        let back = decode(coded)
        print("{plain} -> {coded} -> {back}")
        if back != plain { return 1 }
    }
    0
}
```

```console
$ lupin ex5-11.lu
wwwwoooolf -> w4o4l1f1 -> wwwwoooolf
aaaaaaaaaaab -> a11b1 -> aaaaaaaaaaab
```

The two-digit case lives in one line: `n = n * 10 + …`. A decoder that
read "the digit after the letter" instead of accumulating would pass
the first input and shred the second — which is why the round trip
tests both, and why the exit code carries the verdict: a printed
`a11b1` looks right to a reader skimming; `!=` does not skim.

**Exercise 5-13** *(extension · lupin)*. Write `any_index(s, set)`:
the byte index in `s` of the first character that appears in `set`,
or −1 for none. Probe it with a vowel set against a line that has
vowels, one that does not, and the empty string. Why do the last two
answers have to be the same, and what would distinguishing them cost
your callers?

Solution. `ch05/ex5-13.lu`:

```wolf
fn any_index(s: str, set: str) -> int {
    var i = 0
    for c in s.chars() {
        if set.contains("{c}") { return i }
        i += 1
    }
    -1
}
fn main() -> !int {
    print("{any_index("the wolf runs", "aeiou")}")
    print("{any_index("dry glyph", "aeiou")}")
    print("{any_index("", "aeiou")}")
    0
}
```

```console
$ lupin ex5-13.lu
2
-1
-1
```

Both mean "no position holds a hit", and a position is the only thing
the function promises. Splitting them ("empty input" versus "searched
and missed") would force every caller to handle a case that changes
nothing about what they can do next — the chapter after this one is
about rows, and the first lesson there is that a distinction worth a
tag is one the caller would *branch on*. This one is not.

**Exercise 5-15** *(extension · lupin)*. Fold a long line at twelve
columns: greedy fill, words never split, each output line as full as
the width allows. Return the lines as a `List[str]` and print them.
One comparison in your loop encodes the whole policy — which one, and
what single change makes ragged-right into one-word-per-line?

Solution. `ch05/ex5-15.lu`:

```wolf
fn fold(text: str, width: int) -> List[str] {
    var out = List[str]()
    var line = ""
    for w in text.words() {
        if line.is_empty() {
            line = w
        } else if line.len + 1 + w.len <= width {
            line = line + " " + w
        } else {
            (mut out).push(line)
            line = w
        }
    }
    if line.is_empty() == false { (mut out).push(line) }
    out
}
fn main() -> !int {
    let text = "the wolf runs the long ridge past the frozen creek at dusk"
    for line in fold(text, 12) {
        print(line)
    }
    0
}
```

```console
$ lupin ex5-15.lu
the wolf
runs the
long ridge
past the
frozen creek
at dusk
```

The policy is `line.len + 1 + w.len <= width` — "does this word, plus
its separating space, still fit". Replace `width` with 0 (or the test
with `false`) and every word overflows immediately: one word per
line. The greedy fill and the degenerate layout are the same loop
with one comparison's verdict flipped, which is the honest way to see
that a formatter is a policy wearing a loop.

## §5.2 — The combinator style

**Exercise 5-8** *(comprehension · pending — blocker: `sorted_by` /
`take` absent from the interp std subset; owner: s37-core-types (std
surface pinning))*. The chapter's signature chain, applied to a score
table:

```wolf
for (name, n) in scores.pairs().sorted_by(fn(a, b) b.1 <=> a.1).take(1) {
    print("{name} {n}")
}
```

With `wolf → 4`, `marmot → 5`, `elk → 2` in the map, state what this
prints, then desugar the chain by hand: write the loop-and-locals
version that produces the same line using only `pairs()`, a `var`
best-so-far, and one comparison. Which of the two versions says *what*
it wants, and which says *how* to get it?

Solution (prose, pending the combinator surface): it prints
`marmot 5`: sort descending by the pair's second element, keep one.
The desugared version walks `pairs()` holding the best pair seen and
compares as it goes; it allocates nothing and sorts nothing, which for
top-1 is strictly less work than the chain performs. That is the
trade the combinator style makes on purpose: the chain states the
result's shape and lets the library choose the work; the loop states
the work and leaves the reader to infer the shape. The solution
program is on disk with its expected output in the header; CI runs it
the day the std surface lands.

Today:

```console
$ lupin ex5-8.lu
ex5-8.lu: unsupported: `List` has no method `sorted_by` in this machine's std subset
$ echo $?
4
```

## §5.5 — Structs

**Exercise 5-16** *(fingers · lupin)*. Respell §5.1's `split_at` to
answer a `Row` instead of a `(str, int)`, and use it to parse the three
rows at the head of the chapter and print them through `text`. One line
of the caller gets shorter and one line of the function gets longer.
Name both, and say what each reader gained.

Solution. `ch05/ex5-16.lu`:

```wolf
struct Row { kind: str, cents: int }
fn split_at(row: str, i: int) -> Row {
    Row { kind: row[..i], cents: row[i + 1..].to_int() else 0 }
}
fn text(r: Row) -> str { "{r.kind:<10}{r.cents:>5}" }
fn main() -> !int {
    print(text(split_at("espresso,340", 8)))
    print(text(split_at("pastry,275", 6)))
    print(text(split_at("tip,100", 3)))
    0
}
```

```console
$ lupin ex5-16.lu
espresso    340
pastry      275
tip         100
```

The caller's line got shorter: `let (name, cents) = split_at(…)` is
gone, because nothing has to be taken apart before it is used, and the
value goes straight into `text`. The function's line got longer: the
tuple `(row[..i], …)` became `Row { kind: row[..i], cents: … }`, two
words more. The function's reader gained nothing they did not already
know, since the tuple's two positions were named by the signature one
line up; the caller's reader gained the two words, because `r.cents`
says what `pair.1` made them look up.

**Exercise 5-17** *(extension · lupin)*. A struct holds a list as
easily as an `int`. Declare `Receipt { rows: List[Row] }`, build one
from the three rows, and write `total(r: Receipt) -> int` as a plain
function with no `impl`. Print the count of rows and the total. Then
read `total`'s signature and say what it tells a caller about `r.rows`:
does the call change the list, and which character on the line says so?

Solution. `ch05/ex5-17.lu`:

```wolf
struct Row { kind: str, cents: int }
struct Receipt { rows: List[Row] }
fn total(r: Receipt) -> int {
    var sum = 0
    for row in r.rows { sum += row.cents }
    sum
}
fn main() -> !int {
    var rows = List[Row]()
    (mut rows).push(Row { kind: "drink", cents: 340 })
    (mut rows).push(Row { kind: "food", cents: 275 })
    (mut rows).push(Row { kind: "gratuity", cents: 100 })
    let receipt = Receipt { rows: rows }
    print("{receipt.rows.len} rows, {total(receipt)} cents")
    0
}
```

```console
$ lupin ex5-17.lu
3 rows, 715 cents
```

The call does not change the list, and no character on the line says
so: `r: Receipt` carries no word in front of it, and that absence is
the claim. A parameter with nothing written before it is read for the
call and handed back whole, which is why `main` can print
`receipt.rows.len` and call `total(receipt)` in one line. A function
that did change its argument would have to say so at both ends, and
chapter 7 is where that word is introduced.

## §5.6 — Traits

**Exercise 5-18** *(design)*. The alias `Num` names seven traits, and
a call under `[T: Num]` is checked against each of them by name. Why
does the refusal name the missing trait and never the alias? Say what a
reader would lose if it read "`f64` is not a `Num`", and what the
author of a seven-trait alias gains from the compiler never mentioning
it.

Solution. The alias has no members and no impls, so "`f64` is not a
`Num`" names nothing the reader can write: there is no `impl Num for
f64` to add, and the sentence sends them to the alias's definition to
work out which of seven traits is the one they lack. Naming `Eq`
instead names the impl that fixes it, and the fix is the same whether
the bound was written `Eq`, `Add + Eq`, or `Num`. What the alias's
author gains is that the alias stays a spelling: it can grow an eighth
trait, or be split in two, and every refusal it ever produced was
already about a trait and not about the name, so nothing a reader
learned from a diagnostic goes stale. The one thing an alias is for is
saving the reader from writing the list; it would be a poor trade to
make them read it back out of an error message.

Exercises 5-9 and 5-10 (the third `Draw` shape; the cast-a-binding
rule) moved to chapter 7 with the material they belong to, as 7-17 and
7-18 (bs42). The numbers are not reused.
