# Chapter 2 — Strings, honestly: exercises

Exercises 2-1 through 2-4 are the doctrine's exemplar batch and live in
`principles/EXERCISES.md` §5; numbering continues from them. Commands
run from this directory; outputs are pasted from real runs.

## §2.2 — Multiline and raw

**Exercise 2-5** *(comprehension · lupin REPL)*. Predict all three
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

## §2.3 — Bytes, honestly

**Exercise 2-6** *(comprehension · lupin REPL)*. `"wolf"` has four
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
this section is the last: `"e\u{301}".chars().len` is 2 — a combining
accent is its own scalar, so the glyph a reader sees as one letter is
two chars in a three-byte string. Bytes count storage, chars count
scalars, and neither counts what the reader sees.

## §2.5 — What the machine does

**Exercise 2-8** *(comprehension · lupin REPL)*. `s` is
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
