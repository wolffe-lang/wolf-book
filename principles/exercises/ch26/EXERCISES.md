# Chapter 26 — `count`, twice

Six exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`.

One note on checkers, because this chapter is the book's first that is
compiled rather than interpreted. `count` reads files, and the reference
interpreter has no filesystem — it declines the effect rather than
mocking it — so the transcripts for anything touching `fs_read_text` are
`wolf build` runs. The solution programs on disk here are the ones the
interpreter can run, plus one whose whole answer is a static verdict and
therefore belongs to `wolf conform-run` either way.

## §26.3 — The same machine as a `match`

**Exercise 26-1** *(fingers · wolf)*. Build `count` as printed and run
it. Then put a tab in the middle of `one.txt`'s first line and predict all
three numbers before running it again.

Solution. `ex26-1.lu`, which is the chapter's `tally` with the tab in
place of a space. Predict: nothing moves. A tab is a word separator in
exactly the way a space is (both are arms of the same `match` pattern), and
it is one byte, as a space is:

```console
$ lupin ex26-1.lu
       2       6      31 one.txt
```

Two lines, six words, thirty-one bytes: the same three numbers §26.1
prints. The prediction is the exercise: `32 | 9 | 10` is one arm, so the
three characters are one concept, and a reader who has understood that
arm knows the answer without running anything.

**Exercise 26-2** *(comprehension · lupin)*. `tally` counts a word every
time it crosses from *between* to *inside*. Predict `lines`, `words`, and
`bytes` for the text `"a  b\n\nc"` (two spaces, a blank line, no
trailing newline) and name which of the three people get wrong.

Solution. `ex26-2.lu`. The answer is `2 3 7`:

```console
$ lupin ex26-2.lu
       2       3       7 -
```

Seven bytes: `a`, two spaces, `b`, two newlines, `c`. Three words: the two
spaces are one gap, not two, because the second one finds `inword` already
false. And two lines, which is the number people get wrong: there are
three *rows of text* and only two newlines, and this counter counts
newlines. `wc` does the same thing for the same reason, and the reason is
that a "line" without a terminator is a judgment call while a newline is a
byte.

## §26.4 — Per file, and a total

**Exercise 26-3** *(extension · wolf)*. Give `count` a bytes-only mode: a
second row function that prints the byte column alone, and a `bool` at the
top of `main` that chooses between them. Then say what the same option
costs in the C twin, and count the lines.

Solution. `ex26-3.lu`. The wolf side is four lines: a `bytes_row`
function and one `if` at the call site.

```console
$ lupin ex26-3.lu
      31 one.txt
```

The C twin costs about the same *for the printing* (a second `row`-like
function is four lines there too) and then costs more for the choosing,
because the flag has to reach `main`'s loop from wherever it was decided.
In our twin the choice is a local `int`, so it is one more declaration and
one more `if`: call it six lines against four. The win is small because
most of the difference in this chapter's totals is not in the parts that
print things.

**Exercise 26-4** *(comprehension · wolf)*. Narrow `count_file`'s row to
`Tally ! {not_found, denied}` and predict the diagnostic's code and the
tags it names, before running it. Then write the full row out by hand and
check that it and `-> !Tally` accept the same program.

Solution. `ex26-4.lu`. The code is E0602, and the tags it names are the
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

**Exercise 26-5** *(spelunking · wolf)*. Add a third name that does not
exist, run the program, and read the exit status. Then read the E0602
note above in full and explain, in two sentences, why the C's `-1` needs a
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
the name was in the list: the two streams are separate but they are written
in program order.

The two sentences: `-1` needs a convention because it is an ordinary value
of the return type, so the *only* thing that makes it mean "failure" is an
agreement between the author of `count` and the author of `main`, an
agreement no part of the program states and nothing checks. A row needs no
convention because the failure is not a value of the success type at all:
`Tally ! {…}` is a different type from `Tally`, the compiler will not let a
caller read one as the other, and E0602 is that rule being enforced across
a call boundary rather than remembered across one.

**Exercise 26-6** *(design)*. `count` reads each file whole. Sketch the
version that does not: `fs_open`, a loop of `fs_read` over fixed-size
chunks, and a state machine that survives across chunk boundaries. Name
the one thing that gets harder, and say whether you would pay one call to
avoid it.

Solution (discussion): the shape is `let fd = fs_open(name)?`, then a loop
of `fs_read(fd, 8192)` until it answers the `eof` tag, feeding each chunk
to a `tally` that takes the running `Tally` and the `inword` flag as
parameters and returns both, because the machine's job is to carry state
across bytes and the bytes now arrive in batches.
`defer fs_close(fd)` on the line after the open, so the descriptor closes
on every path out.

The thing that gets harder is not the state machine. It is the chunk
boundary: `fs_read` hands back a `str`, a `str` is UTF-8, and a fixed-size
read can land in the middle of a multi-byte code point. The whole-file read
never has that problem because a file is a whole document; a chunked reader
either has to be handed bytes, not text, or has to keep the tail of
each chunk until the next one completes it.

Would we pay one call to avoid it? For `count`, honestly, yes. Write the
chapter's version first; write the chunked one the day somebody points a
40-gigabyte file at it. The general
answer is the one §26.5 gives: know which of the two you copied.

## Stats

Six exercises: 1 fingers, 3 comprehension, 1 extension (26-3, tagged
comprehension-adjacent in the index as extension), 1 spelunking, 1 design.
Checkers: 3 under lupin, 1 under `wolf conform-run` with a reviewed
snapshot, 2 discussion. Four solution programs are on disk and ran with the
outputs shown.
