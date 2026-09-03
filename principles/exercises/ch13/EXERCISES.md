# Chapter 13 — Dividing one job: exercises

Commands run from this directory; outputs are pasted from real runs.
Two exercises in this chapter are Tier-PENDING: their programs are on
disk with the outcome their directive headers claim, and the manifest
names what blocks them. Nothing below shows an output that was not
produced.

Seven of the ten are printed in the chapter: 13-2, 13-3 and 13-4 in
§13.2, and 13-5, 13-7, 13-9 and 13-10 in the chapter batch. The other
three are `par`'s — 13-1 and 13-6 hold pending rows, and 13-8's
subject is `par`'s decomposition contract — so all three are written,
on file, and not printed. They land with §13.1.

## §13.1 — `par`

**Exercise 13-1** *(comprehension · pending — blocker: `par` absent
from the interpreter's std subset; owner: s32-tasks-scheduler /
s37-core-types)*. Nine numbers, squared and summed. The parallel
spelling replaces the squaring loop with one call:

```wolf
fn main() -> !int {
    var xs = List[int]()
    for i in 1..=9 { (mut xs).push(i) }
    let sq = xs.par(fn(x) x * x)
    var sum = 0
    for v in sq { sum += v }
    print("{sum}")
    0
}
```

Before any tool can run this: what must `par` guarantee about the
*order* of `sq` relative to `xs` for the surrounding program to stay
correct without edits, and where do the tasks it spawns join?

Solution (prose): `par` must return results in input order (position
`i` of `sq` is `f(xs[i])`) or the "change one call" promise is false,
because downstream code may be order-sensitive even when a sum is not.
The tasks join *inside* the call: `par` is structured concurrency in
an expression, a scope that opens and closes between the parentheses,
which is why `?` can propagate a child's error out of it (the join has
already happened when `?` looks). The directive header pins
`stdout="285"`; CI verifies it the day `par` lands.

Today:

```console
$ lupin ex13-1.lu
ex13-1.lu: unsupported: `List` has no method `par` in this machine's std subset
$ echo $?
4
```

**Exercise 13-2** *(fingers · lupin)*. Nine numbers, squared and
summed, on one core: build the list, square each into a second list, add
them up, print the total. Run it and keep the program. The number it
prints is the number every divided version of this job has to agree
with, and a divided job that does not reproduce its sequential answer
is wrong, not faster.

Solution. `ch13/ex13-2.lu`:

```wolf
fn main() -> !int {
    var xs = List[int]()
    for i in 1..=9 { (mut xs).push(i) }
    var sq = List[int]()
    for x in xs { (mut sq).push(x * x) }
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

## §13.2 — The race that does not compile

**Exercise 13-3** *(comprehension · wolf + lupin)*. Two tasks
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
in what they print: the codes and the spans agree, and the amount of
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
question asks about: it concerns not this closure's write but the
*previous* closure, which took its copy of `hits` before this line
ran and will never see it. Five diagnostics for one mistake with two
instances.

The interpreter refuses the same program, and says it once:

```console
$ lupin ex13-3.lu
ex13-3.lu: E1101: this task writes to `hits`, which it captures from the enclosing function: unsynchronized mutable capture across tasks (D14 — copy, share `imm`, or `move`; a `sync` type mediates shared writes) [conc.task.spawn] at 9:24
$ echo $?
2
```

Same code, same headline, same span as the compiler's first error:
`318..322` is the `hits` at line 9. What differs is volume, not verdict:
the compiler reports every offending spawn and both warnings, the
interpreter reports the first refusal and stops. Neither runs it.

An earlier interpreter ran this program to exit 0: capture by value
meant both increments landed on private copies, `hits` printed as 0,
and a silently wrong answer wore a clean exit code. That contrast was
the argument for making the capture a compile error. The argument won;
the demonstration is gone, and W1101's note is where the lost-update
story now lives.

**Exercise 13-4** *(spelunking · lupin)*. Two programs differ in one
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
(`races: 0`: channel operations synchronize; nothing here is a data
race); 4a copies the arrival order into stdout, so its two schedules
produce two outputs and the verdict is schedule-dependent, while 4b
folds the values with `+`, which commutes, collapsing both schedules
into one observable outcome. This is the design rule for `par`
reductions in one sentence: combine with operations whose result does
not encode arrival order, and the whole parallel program stays
observably deterministic. That is the property `--explore` exists to
certify, and the property 13-1's `par` preserves by returning results
in input order.

## Chapter batch

**Exercise 13-5** *(extension · lupin)*. grep, wolfished: write
`grep(text, pattern) -> List[str] ! {EmptyPattern}` returning the
matching lines. Substring search is yours to write with byte slices. Why
is the empty pattern an *error* here, when POSIX grep happily matches it
everywhere?

Solution. `ch13/ex13-5.lu`:

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
        if contains(line, pattern) { (mut hits).push(line) }
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
almost never intends: it is usually a variable that turned out
blank. The row makes the caller say which they meant: handle
`EmptyPattern` with "all lines" if that is truly the wish. An API's
defaults should serve its likeliest accident, not its cleverest use.

**Exercise 13-6** *(extension · pending — blocker: `par` absent from
the interpreter's std subset; owner: s32-tasks-scheduler /
s37-core-types)*. Parallelize 13-5: collect the lines into a list and
map `contains` over them with `par`, counting matches from the
returned flags. `ch13/ex13-6.lu` is on disk with the one-call
spelling. Why is `contains` an ideal `par` body, and what about
13-5's `grep` had to change shape before `par` could apply?

Solution (prose): `contains` is pure (reads its arguments, touches
nothing shared), so lines can be tested in any order and E1101 has
nothing to object to. The shape change is the honest cost of the one
free call: the `for` loop *filtered* (pushing only hits), while `par`
*maps*, so the parallel version computes a flag per line and counts
afterward, keeping the reduction order-insensitive per 13-4's rule.

Today:

```console
$ lupin ex13-6.lu
ex13-6.lu: unsupported: `List` has no method `par` in this machine's std subset
$ echo $?
4
```

**Exercise 13-7** *(comprehension · lupin)*. One Euler step for two
bodies on a line, gravity only, equal masses. Before running: what is
`v1 + v2` after the step, and is your answer exact or approximate for
f64 arithmetic?

Solution. `ch13/ex13-7.lu` (excerpt):

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
property of sharing one rounding, not of infinite precision: compute
the two accelerations separately with different roundings and the
symmetry is gone. That observation is the seed of every reproducible
n-body benchmark in Part 4.

**Exercise 13-8** *(design)*. A million elements, a `par` map, and a
machine with eight cores. Task-per-element is a million tasks;
task-per-chunk is eight. Discuss: what does each choice cost, which
one does a *structured* runtime prefer, and why must the answer never
change the program's result?

Solution (discussion): task-per-element pays scheduling overhead per
element (for a cheap body, more bookkeeping than work) but exposes
maximal parallelism and makes an uneven workload self-balancing.
Task-per-chunk amortizes overhead to nearly nothing but invites
stragglers: one slow chunk idles seven cores, and choosing chunk size
is a tuning job that outlives its hardware. Runtimes therefore prefer
neither statically: work-stealing splits eagerly while queues are
hungry and coarsens when they are not, and `par` deliberately does
not let you spell the split in the program. The last question is the
contract doing its work: because `par` returns results in input order
and joins inside the call, the decomposition is unobservable (13-4's
lesson as an API guarantee), so the runtime may re-decide it per run,
per machine, per load, without changing the program's result.

**Exercise 13-9** *(extension · lupin)*. The caesar shift, sequential
on purpose: `shift(s, k)` moves each lowercase letter `k` places with
wraparound (`chars`, casts, `% 26`) and passes everything else
through. Encode a line with `k = 3`, decode with the *same function*,
and make `main` exit nonzero if the round trip misses. Two questions:
what number decodes for `k = 3`, and — the extension — if the shift
instead came from the letters of a key word repeating along the
message, what would decode have to do that it does not do now?

Solution. `ch13/ex13-9.lu`:

```wolf
fn shift(s: str, k: int) -> str {
    var out = ""
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            let at = (c as int) - ('a' as int)
            let moved = (at + k) % 26
            out = "{out}{(moved + ('a' as int)) as char}"
        } else {
            out = "{out}{c}"
        }
    }
    out
}
fn main() -> !int {
    let plain = "the wolf runs at dusk"
    let coded = shift(plain, 3)
    let back = shift(coded, 23)
    print(coded)
    print(back)
    if back != plain { return 1 }
    0
}
```

```console
$ lupin ex13-9.lu
wkh zroi uxqv dw gxvn
the wolf runs at dusk
```

23 decodes, because 3 + 23 is 26 and `% 26` makes 26 the identity —
decode is encode with the complementary shift, one function doing
both jobs. Under a repeating key the complement is *per position*:
decode must walk the key alongside the message and complement each
letter's own shift, so the one thing it needs that the fixed-shift
version lacks is an index — the loop over `chars()` grows a counter,
and everything else survives unchanged. (That per-position version is
the Vigenère cipher, and the counter is the whole difference.)

**Exercise 13-10** *(comprehension + extension · lupin)*. 13-5's byte
scan found lines; this one counts hits. Write
`count(hay, needle, overlapping)` — the same
`hay[i..i + needle.len] == needle` probe — where the flag decides
whether a hit advances `i` by one byte or by the needle's length.
Predict both answers for `"aaaa"` / `"aa"` before running. Which
convention does a text editor's find-and-replace need, and what goes
wrong under the other one?

Solution. `ch13/ex13-10.lu`:

```wolf
fn count(hay: str, needle: str, overlapping: bool) -> int {
    var hits = 0
    var i = 0
    while i + needle.len <= hay.len {
        if hay[i..i + needle.len] == needle {
            hits += 1
            i += if overlapping { 1 } else { needle.len }
        } else {
            i += 1
        }
    }
    hits
}
fn main() -> !int {
    print("{count("aaaa", "aa", true)} {count("aaaa", "aa", false)}")
    print("{count("the wolf the ridge the creek", "the", true)}")
    0
}
```

```console
$ lupin ex13-10.lu
3 2
3
```

Overlapping sees hits at offsets 0, 1, 2; non-overlapping consumes
two bytes per hit and sees 0 and 2. Find-and-replace needs the
non-overlapping convention: each replacement consumes its match, so
the next search resumes *after* it. Under the overlapping count a
replace of `"aa"` in `"aaaa"` would claim three sites where only two
disjoint replacements exist — the second "hit" overlaps bytes the
first replacement already rewrote. A count is only meaningful with
its consumption rule attached, which is why the flag is in the
signature and not in a comment.

**Exercise 13-11** *(extension · lupin)*. 13-5 searched a `str` with
`str` slices. Do it a layer down, over bytes, and lend the haystack
rather than handing it over: `find(hay, needle) -> int` takes two
`List[byte]` byte views and answers the first index where the needle
starts, or `-1`. The window comparison belongs in its own function, so
the slice happens inside a callee working on a view it does not own.
Two questions: what does the caller still hold after the call, and how
many bytes does one window cost?

Solution. `ch13/ex13-11.lu`:

```wolf
fn matches_at(hay: List[byte], needle: List[byte], i: int) -> bool {
    let window = hay[i..i + needle.len]
    var k = 0
    while k < needle.len {
        if window[k] != needle[k] { return false }
        k += 1
    }
    true
}
fn find(hay: List[byte], needle: List[byte]) -> int {
    if needle.len == 0 { return -1 }
    var i = 0
    while i + needle.len <= hay.len {
        if matches_at(hay, needle, i) { return i }
        i += 1
    }
    -1
}
fn main() -> !int {
    let text = "the wolf runs the long ridge"
    let hay = text.bytes()
    print("{find(hay, "wolf".bytes())}")
    print("{find(hay, "ridge".bytes())}")
    print("{find(hay, "moon".bytes())}")
    print("{hay.len}")
    0
}
```

```console
$ lupin ex13-11.lu
4
23
-1
28
```

`hay` crosses two signatures without a `take` and without a `mut`, so
both callees borrow it and `main` still owns it — the last line proves
that by reading `hay.len` after every search has finished with it. The
slice inside `matches_at` is the point: a lent view can be sliced, and
the subscript range is the same endpoint surface a `str` slice takes.

What a window costs is the second half of the answer, and it is not
free. `hay[i..i + needle.len]` is a fresh `List` of that many
elements, so the scan allocates one window per position tried rather
than walking the haystack in place. That is the honest price of
spelling the comparison as a slice, and it is why a real substring
search compares element by element from `i` without cutting anything
out. Write that version too and the loop body loses its allocation
entirely; the answer it prints does not change.
