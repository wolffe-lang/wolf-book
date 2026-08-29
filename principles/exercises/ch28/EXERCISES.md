# Chapter 28 — `wordtree`, twice

Seven exercises. Programs are in this directory; commands are as run from
here, and every output below is pasted from a real run at the pins in
`wolf-toolchain.toml`. This chapter's programs are the interpreter's, as
the chapter's own transcripts are.

## The chapter batch

**Exercise 28-1** *(fingers · lupin)*. Build the tree as printed and run
it. Then add a word that sorts before `moon` and one that sorts after
`wolf`, and predict where each appears in the output before you run it.

Solution. `ex28-1.lu`, with `zebra apple` added to the text. The
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

**Exercise 28-2** *(comprehension · lupin)*. `add` compares with
`w < n.word` on `str`. Predict the order of `Wolf`, `wolf`, `WOLF`, and
`wolfs` in the output *without* the `.lower()` call, and say which two of
the four end up as one node once `.lower()` is back.

Solution. `ex28-2.lu`. Four distinct words, four nodes, ordered by bytes:

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

**Exercise 28-3** *(extension · lupin)*. Add a `-n` mode: print the words
in descending order of count instead of alphabetically, with ties broken
alphabetically. The tree is already sorted by word, so the shape of the
answer is a second pass — say what you collect on the first pass, and what
it costs in lines.

Solution. `ex28-3.lu`. The first pass is `collect`, an in-order walk that
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

**Exercise 28-4** *(comprehension · lupin)*. Delete the `is_empty()` guard
in `walk` and predict the exact failure: which trap kind, which exit code,
and at which of the two `walk` calls. Then check it.

Solution. `ex28-4.lu`, with `walk(n.left[0])` unguarded. The prediction:
a `bounds` trap, exit 3, at the *left* call — and it fires on the first
leaf the walk reaches, before anything is printed at all.

```console
$ lupin ex28-4.lu
ex28-4.lu: trap(bounds): index 0 is outside a collection of 0 element(s) [mem.ub.defined] at 29:10
$ echo $?
3
```

The C twin's equivalent mistake — dropping `if (p != NULL)` from
`treeprint` — dereferences a null pointer, which is undefined behavior: it
may segfault, it may print garbage, and on some targets it may appear to
work. Here it is a defined fault with a kind, a clause tag, and a byte
span, and it happens at the same place every time. That is the difference
the whole book is about, arriving in a two-line function.

**Exercise 28-5** *(spelunking · the C twin)*. Count, in `wordtree.c`,
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

**Exercise 28-6** *(extension · lupin)*. Take the tree's census: write
`nodes` and `depth` and print both after the walk. Then multiply the node
count by two and say what that number is in the C column, and what it is
in the wolf one.

Solution. `ex28-6.lu`. Two recursions in the same shape as `walk`:

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

**Exercise 28-7** *(design)*. K&R's `addtree` returns the new subtree
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

## Stats

Seven exercises: 1 fingers, 2 comprehension, 2 extension, 1 spelunking,
1 design. Checkers: 5 under lupin (all five programs on disk ran with the
outputs shown, one of them to a trap), 1 read of a vendored C twin, 1
discussion. No exercise in this batch is pending.
