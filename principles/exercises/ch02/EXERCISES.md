# Chapter 2 — Strings, honestly: exercises

Exercises 2-1 through 2-4 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §2.1 — Literals, methods, interpolation

**Exercise 2-12** *(comprehension + extension · lupin)*. Center
`"DEN LOG"` in twenty columns of stars, twice: once with a fill-align
spec, once by hand with `repeat` and `len`. The padding is thirteen,
which does not halve. Predict which side gets the extra star under
each spelling before you run them, then state the rule.

Solution. `ch02/ex2-12.lu`:

```wolf
fn main() -> !int {
    let title = "DEN LOG"
    print("{title:*^20}")
    let pad = 20 - title.len
    let left = pad / 2
    let right = pad - left
    print("{"*".repeat(left)}{title}{"*".repeat(right)}")
    0
}
```

```console
$ lupin ex2-12.lu
******DEN LOG*******
******DEN LOG*******
```

Both give the extra star to the right. The spec's rule is exactly the
hand version's arithmetic: left gets `pad / 2`, truncation rounds the
left side down, and the remainder lands on the right. The two lines
matching is the point — `^` is not magic, it is the division you would
have written.

## §2.2 — Multiline and raw

**Exercise 2-13** *(fingers · lupin)*. Make the invisible visible:
print `"howl\tat\nthe moon"` as one line in which each tab shows as
`<tab>` and each newline as `<nl>`. Two `replace` calls chain. Why
must the program spell its markers in ordinary literals but would
have needed raw ones to *match* `\t` if the text had carried a real
backslash?

Solution. `ch02/ex2-13.lu`:

```wolf
fn main() -> !int {
    let mixed = "howl\tat\nthe moon"
    let visible = mixed.replace("\t", "<tab>").replace("\n", "<nl>")
    print(visible)
    0
}
```

```console
$ lupin ex2-13.lu
howl<tab>at<nl>the moon
```

`"\t"` in the pattern position is the one-byte tab itself, which is
what the text contains, so the ordinary literal is the right spelling.
Had the text carried a literal backslash-t — two bytes — the pattern
would need `r"\t"` to mean those two bytes, because in an ordinary
literal that spelling collapses to the tab. The escape table applies
where the literal is written, not where the string is used.

**Exercise 2-5** *(comprehension · lupin REPL)*. Predict all three
lengths before evaluating: `"\n".len`, `r"\n".len`, `r"C:\temp".len`.

Solution: 1, 2, 7. In an ordinary literal `\n` is one byte, a newline.
In a raw literal it is two bytes, a backslash and an `n`: raw means
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

## §2.3 — Bytes, honestly

**Exercise 2-6** *(comprehension · lupin REPL)*. `"wolf"` has four
bytes. Predict each of these, precisely, as a value or an event: `"wolf"[..2]`,
`"wolf"[2..]`, `"wolf"[4..4]`, `"wolf"[3..2]`.

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

`[4..4]` does not trap: the boundary after the last byte is a real
position, and it is where appending happens.

## §2.4 — Iterating meaning

**Exercise 2-7** *(extension · lupin)*. Write `encode`, a run-length
encoder over bytes: `"aaabcc"` becomes `"a3b1c2"`. Walk the string with
byte slices and equality only. What does your encoder do with the empty
string, and did you have to write a special case for it?

Solution. `ch02/ex2-7.lu`:

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

**Exercise 2-10** *(fingers · lupin)*. Reverse each word of
`"the wolf runs"`, keeping the words in order. Build each reversal with
`chars()` and string joins alone. Then say why reversing the *bytes*
instead would wreck `"éland"` while this version does not.

Solution. `ch02/ex2-10.lu`:

```wolf
fn main() -> !int {
    var out = ""
    for w in "the wolf runs".words() {
        var r = ""
        for c in w.chars() {
            r = "{c}" + r
        }
        if out.is_empty() { out = r } else { out = out + " " + r }
    }
    print(out)
    0
}
```

```console
$ lupin ex2-10.lu
eht flow snur
```

`é` is two bytes in a fixed order; reversing bytes would emit them
backward, and the result is not UTF-8 at all. `chars()` yields whole
scalars, so prepending each one keeps every multi-byte sequence intact.
Reversal is the smallest program that shows why "a string is bytes" and
"a string is text" need different loops.

**Exercise 2-11** *(fingers · lupin)*. Squeeze: delete from
`"howling at the moon"` every character that appears in a second
string, here `"aeiou"`. One pass, `contains`, and a growing result.
Which of the two strings does your loop walk, and what happens to the
program's cost if you walk the other one?

Solution. `ch02/ex2-11.lu`:

```wolf
fn main() -> !int {
    let s = "howling at the moon"
    let drop = "aeiou"
    var t = ""
    for c in s.chars() {
        if drop.contains("{c}") == false {
            t += "{c}"
        }
    }
    print(t)
    0
}
```

```console
$ lupin ex2-11.lu
hwlng t th mn
```

The loop walks `s` and probes `drop`, so the work is the text's length
times the (tiny, fixed) drop set. Walked the other way — for each
character of `drop`, scan and rebuild `s` — the cost multiplies by the
number of rebuilds and the code needs a mutable copy per pass. Same
answer, different bill; the loop order is the algorithm.

**Exercise 2-9** *(comprehension · wolf)*. Predict all six numbers
before running: for each of `"wolf"`, `"é"`, and `"e\u{301}"`, both
`.len` and `.chars().len`. Then the pointed half: which of the six
could §2.3 have told you, and which one needed this section?

Solution. `ch02/ex2-9.lu`:

```wolf
fn main() -> !int {
    print("{"wolf".len} {"wolf".chars().len}")
    print("{"é".len} {"é".chars().len}")
    print("{"e\u{301}".len} {"e\u{301}".chars().len}")
    0
}
```

```console
$ wolf run ex2-9.lu
4 4
2 1
3 2
```

Five of the six are §2.3 material: `.len` counts bytes (4, 2, 3), and
ASCII is the case where every scalar is one byte, so `"wolf"` counts 4
either way and `"é"`'s two bytes are one scalar. The number that needed
this section is the last: `"e\u{301}".chars().len` is 2. A combining
accent is its own scalar, so the glyph a reader sees as one letter is
two chars in a three-byte string. Bytes count storage, chars count
scalars, and neither counts what the reader sees.

**Exercise 2-14** *(fingers · lupin)*. Detab: replace each tab in
`"a\tbb\tccc"` with the spaces that carry the column to the next tab
stop, stops every four columns. Print the result in brackets so the
spacing is checkable. Why is "replace each tab with four spaces"
wrong, and which input proves it?

Solution. `ch02/ex2-14.lu`:

```wolf
fn main() -> !int {
    let line = "a\tbb\tccc"
    var out = ""
    var col = 0
    for c in line.chars() {
        if c == '\t' {
            let stop = (col / 4 + 1) * 4
            out += " ".repeat(stop - col)
            col = stop
        } else {
            out += "{c}"
            col += 1
        }
    }
    print("[{out}]")
    0
}
```

```console
$ lupin ex2-14.lu
[a   bb  ccc]
```

A tab's width depends on where it sits: after `a` (column 1) it is
three spaces; after `bb` (column 6) it is two. "Four spaces" is right
only when the tab lands on a stop already — the input above proves it
by needing two different widths for its two tabs. The tab stop is a
*destination*, not a distance.

**Exercise 2-15** *(extension · lupin)*. Entab — 2-14's inverse: fold
runs of blanks back into tabs where a run reaches a four-column stop,
keeping single blanks that do not. Feed it 2-14's output and print the
tabs visibly. One rule in your loop decides whether a run becomes
`\t` or stays a space; state the rule.

Solution. `ch02/ex2-15.lu`:

```wolf
fn main() -> !int {
    let line = "a   bb  ccc"
    var out = ""
    var col = 0
    var blanks = 0
    for c in line.chars() {
        if c == ' ' {
            blanks += 1
            col += 1
            if col % 4 == 0 {
                if blanks == 1 { out += " " } else { out += "\t" }
                blanks = 0
            }
        } else {
            out += " ".repeat(blanks)
            blanks = 0
            out += "{c}"
            col += 1
        }
    }
    out += " ".repeat(blanks)
    print("[{out.replace("\t", "<tab>")}]")
    0
}
```

```console
$ lupin ex2-15.lu
[a<tab>bb<tab>ccc]
```

The rule: a run of blanks ending exactly on a stop becomes a tab only
when it is longer than one blank — a single blank at a stop stays a
blank, because `\t` would claim a column the original never spent.
Everything else is bookkeeping: count blanks, flush them at a stop or
at the next visible character. Round-tripping 2-14's output back to
its input is the check that both programs mean the same columns.

## §2.5 — What the machine does

**Exercise 2-8** *(comprehension · lupin REPL)*. `s` is `"wolfpack"`.
Predict all four values, then say what slicing `s` cost: did any of
these lines copy eight bytes?

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

Solution: `wolf`, 4, 8, 4, and nothing copied. A slice is a *view*:
two words, a pointer and a length, aimed into bytes that already
exist. `s` is untouched by every line here, which is why `s.len` is
still 8 after `t` was made from it. The chapter's cost claim is
checkable from the values alone: if slicing copied, substring-heavy
code would pay by the byte; because it is two words, `t = s[..4]`
costs the same whether `s` is eight bytes or eight megabytes.
