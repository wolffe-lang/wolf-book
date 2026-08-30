# Chapter 27 — `rpn`, twice

Eight exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`. `rpn` is the project in the part that both
implementations run, so any of these programs can be checked either way;
the transcripts below are the interpreter's, because that is the loop this
chapter tells you to develop in.

## The chapter batch

**Exercise 27-1** *(fingers · lupin)*. Add `%` to the dispatch. Then
predict what your arm does for `7 0 %` before you run it, and say whether
you had to write anything the `/` arm did not already show you.

Solution. `ex27-1.lu`. Two edits: `37` joins `is_operator`'s list of
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
is that it has to be *written*: a modulo by zero is the same defined fault
division by zero is, so the guard is not optional, and nothing about `%`
made a new kind of problem. That is the answer to the second half: no, the
`/` arm showed you everything. Adding an operator to this calculator costs
one byte in `is_operator` and one arm.

**Exercise 27-2** *(comprehension · lupin)*. `eval` returns `Empty` for
both an underflowing operator and an expression that leaves two values on
the stack. Predict the output for the three lines `3 +`, `3 4`, and `3 4 +
5`, and then argue whether one tag for two situations is the same mistake
§27.1 accused the C of making.

Solution. `ex27-2.lu`. All three are `Empty`:

```console
$ lupin ex27-2.lu
error: the stack does not hold two operands
error: the stack does not hold two operands
error: the stack does not hold two operands
```

`3 +` underflows: one operand, two wanted. `3 4` and `3 4 + 5` both finish
with two values on the stack, so `stack.len != 1` fires.

Is that §27.1's mistake? No. The C's fault is that `0.0` from a failed
`pop` is *indistinguishable
from a successful answer*: the error and the success share a
representation, and a caller who forgets to look cannot tell them apart.
Here the error is a tag: a caller cannot read it as a number, cannot forget
it, and cannot get an answer out of a failed line. The complaint against
this program is a smaller one: the message is imprecise, because one tag
is doing two jobs and its text has to cover both. That is a wording
problem with a wording fix, and 27-3 is the fix.

**Exercise 27-3** *(extension · lupin)*. Give `Empty` a payload: which
operator ran out of operands, and how many it found. You will have to
change the row, the two `return`s, and one match arm. Say what told you
each one.

Solution. `ex27-3.lu`. A second payload struct, `Short { op: str, found:
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
type, which is the reason the row is written down.

Note the last-line case borrowing the word `op` for `"end of line"`. That
is honest but slightly forced, and it is a real design question: two
different failures are still sharing a tag. Splitting them into
`Underflow(Short)` and `Leftover(int)` is one more row entry and one more
arm, and it is the version to write if this were a real calculator.

**Exercise 27-4** *(comprehension · lupin)*. Feed it `007` and `-0` and
`- 3`. Predict all three results before running, then explain which of the
three is handled by `strip_prefix` and which by `words()`.

Solution. `ex27-4.lu`:

```console
$ lupin ex27-4.lu
7
0
error: the stack does not hold two operands
```

`007` is 7: the digit loop multiplies by ten and adds, and leading zeros
add nothing. `-0` is 0: `strip_prefix("-")` takes the sign, the body `0`
parses to zero, and `0 - 0` is 0 (there is no negative zero in `int`, which
is one of the quiet advantages of not being `double`). `- 3` is the
interesting one: `words()` cut it into two tokens, so the `-` is an
operator with nothing under it, and the answer is `Empty`.

So: `strip_prefix` handles the sign that is *attached*, and `words()`
decides what "attached" means. That division is why the wolf
column needs no `ungetch`, and this exercise is the smallest program that
shows it.

**Exercise 27-5** *(spelunking · the C twin)*. Take a census of
`broken` in `rpn.c`: count the places that can set it and the places
that read it, and write down the line numbers. Then take the same census
of the wolf column's failure surface: where a tag can be produced, and
where one is handled. Say what the two ratios tell you.

Solution. In `samples/contrast/rpn.c`, `broken` is declared at line 35 and
raised at five sites: 43 (`push` overflow), 53 (`pop` underflow), 69
(`ungetch` overflow), 130 (division by zero) and 146 (unknown command).
Three of them are in functions that have no other way to report anything.
It is read once, at line 136, in the `'\n'` arm, and cleared once at 142 so the
next line starts fresh. Five raises, one read, and the read is more than
eighty lines and three functions away from the furthest raise.

In the wolf column, a tag is produced at nine `return`s across `number` and
`eval`, and every one of them arrives in exactly one place: the `match err`
in `main`. Nine produce sites, one handle site, no reset: there is
no state to clear, the failure having been the value.

The ratios look similar and they are not the same fact. The C's one read is
a *choice*: it is where the author decided to look, and the compiler would
have been equally happy with none. The wolf column's one handler is a
*requirement*: `eval` returns `int ! {…}`, so `main` cannot get an `int`
out of it without either handling the row or propagating it, and if the row
grows a tag the handler stops compiling. One read by convention, one read by
type.

**Exercise 27-6** *(extension · lupin)*. Add two stack words that are not
operators: `dup` duplicates the top value, `swap` exchanges the top two.
Neither touches `number` or the `match`. Predict what `7 2 swap -`
evaluates to before you run it.

Solution. `ex27-6.lu`. Two branches ahead of the `!is_operator(tok)`
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

**Exercise 27-7** *(design)*. Wolf's stack holds `int` and K&R's holds
`double`. Argue the other choice: what would the wolf column have to give
up to work in `f64`, what would it gain, and where in this chapter would
the text have to change?

Solution (discussion): what it gains is obvious and real: `7 2 /` becomes
3.5, and a calculator that cannot divide is a strange calculator. What it
gives up is `number`. The digit loop that ends `n * 10 + (b - 48)` produces
an `int` and there is no conversion from that `int` to an `f64`, so a
floating-point `rpn` cannot reach its operands through the same scan: it
needs a parser that builds a mantissa and an exponent, or a library
function that turns text into a float. That is a substantially bigger
`number`: a fractional part, an optional exponent, and a decision about
what to do with `1e400`.

The text that would change: §27.2's paragraph about the stack's type;
§27.3's claim that ten lines of digit loop is a fair trade; and §27.4's
closing accounting, which currently says the `int` costs exactly one
division. The comparison with `getop`'s 28 lines would also get closer,
because most of what makes `getop` long is that it is a *number* parser,
not a tokenizer.

The defensible position is the one the chapter takes, stated as a choice
rather than as a virtue: this calculator does integer arithmetic, and if
you want fractions you are writing the parser.

**Exercise 27-8** *(design)*. Sketch the REPL: read a line, evaluate it,
print the answer, and stop at end of input. `read_line()` returns
`str ! {eof, io, utf8}`, so name the loop's exit condition, and then say
what the calculator would have to remember between lines for `x 3 +` to
mean anything, and what shape that memory wants to be.

Solution (discussion): the loop is
`let line = read_line() else |_| { break }` inside a `while true`, and the
exit condition is the `eof` tag: "input ended" is a failure tag here,
not a sentinel line or a null.
It is the §27.1 argument arriving from the other direction: end of input is
not an empty string, so it cannot be confused with one.

For `x 3 +` to mean anything the calculator needs a name-to-value table
that outlives a line. The shape that wants is a `Map[str, int]` threaded
through `eval` as a `mut` parameter: one more parameter, one more branch in
the token loop (a token that is neither an operator nor a number is a
variable), and one more tag (`Unbound(Bad)`) for a name nobody has assigned.
Assignment wants a syntax decision: `x 3 !` in the K&R style, or a leading
`let`, or `=` as an operator that pops a value and a name.

The honest note about *this* toolchain: `Map` is a name the prelude knows
and neither implementation carries a signature for, so the table above is a
sketch rather than a program. An association list (two parallel `List`s and
a linear scan) is what a reader can build with the chapters so far, and for
a calculator's
variables a linear scan over a dozen names is not the part that runs
slowly.

## Stats

Eight exercises: 1 fingers, 2 comprehension, 2 extension, 1 spelunking,
2 design. Checkers: 5 under lupin (all five programs on disk ran with the
outputs shown), 1 read of a vendored C twin, 2 discussion. No exercise in
this batch is pending.
