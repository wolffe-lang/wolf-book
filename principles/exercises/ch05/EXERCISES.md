# Chapter 5 — Collections and generics without fear: exercises

Exercises 5-1 through 5-5 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §5.1 — `List`, `Map`, `Set`, tuples

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
            seen.push(line)
            counts.push(1)
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

## Chapter batch

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
            let b = stack.pop()
            let a = stack.pop()
            if t == "+" { stack.push(a + b) } else if t == "-" { stack.push(a - b) } else if t == "*" { stack.push(a * b) } else { stack.push(a / b) }
        } else {
            let n = t.to_int() else { return BadToken }
            stack.push(n)
        }
    }
    if stack.len != 1 { return Underflow }
    stack.pop()
}
fn main() -> !int {
    var tokens = List[str]()
    tokens.push("3")
    tokens.push("4")
    tokens.push("+")
    tokens.push("2")
    tokens.push("*")
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

## §5.2 — The combinator style

**Exercise 5-8** *(comprehension · pending — blocker: `sorted_by` /
`take` absent from the interp std subset; owner: s37-core-types (std
surface pinning))* — The chapter's signature chain, applied to a score
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
`marmot 5` — sort descending by the pair's second element, keep one.
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
