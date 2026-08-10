# N. Chapter title carries its number

<!-- TEMPLATE (bs00, contract target 5). Copy this file to chNN.md's
     stub when the chapter sprint starts; delete what the chapter does
     not need, in this order of reluctance: sidebar last. The section
     numbers N.M come from principles/TOC.md and are the page anchors —
     do not invent numbering here. -->

<!-- Optional epigraph, TONE.md §4 register rules apply. At most one;
     most chapters have none. If it quotes a lyric, it goes on
     PERMISSIONS.md the same commit. -->

*Epigraph in italics, if the chapter earned one.*

Cold open: a program, not a definition. The first thing on the page is
code the reader wants to have written, run for real:

```wolf,run(exit=0, stdout="…")
fn main() -> !int {
    // the cold open earns the chapter
    0
}
```

## N.1 The running example advances

<!-- Every chapter advances its part's running example. Say where it
     was, move it forward, keep the diff small enough to hold. -->

## N.2 Concept, code-first

<!-- Code speaks first (TONE.md §1). A concept is introduced by a
     program that needs it, then explained. Fail blocks show the real
     diagnostic and are snapshot-checked:

```wolf,fail(E1001)
// the smallest program that earns the diagnostic
```

     Split one program across prose with part blocks:

```wolf,part(example)
// first slice
```

```wolf,part(example, cont),run(exit=0)
// the extractor stitches the parts and runs the whole
```
-->

## N.M What the machine does

<!-- The T1 cost-model sidebar, one per chapter: what this chapter's
     constructs cost — words, allocations, instructions — stated
     plainly, from measurements where the claim needs one. -->

## Exercises

<!-- 2–4 challenges, Crafting Interpreters style, numbered N-1… per
     EXERCISES.md: stems here, solutions on the solutions pages,
     programs under the sample pipeline. -->

<!-- AUDIT LEDGER (unpublished): every papercut hit while writing this
     chapter — tool friction, diagnostic wording, std gaps — one line
     each, filed to the bs01 issue loop. Writing the chapter is the
     ergonomics audit; an empty ledger on a nontrivial chapter is
     suspicious, not clean.
     - [ ] …
-->
