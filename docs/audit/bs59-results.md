# bs59 — results against the prediction

Prediction committed at `a8a88d3` (`docs/audit/bs59-prediction.md`),
before `wolf-0.2.22-*.tar.gz` or `lupin-0.1.45-*.tar.gz` was downloaded.
Measured 2026-10-04 on kasumi (linux x86-64) from the release archives,
each matched against the release API the same minute
(`bs59-evidence/archive-digests.txt`, read 02:08Z): wolf 0.2.22
`df0f2fea…`, lupin 0.1.45 `907cfb1a…`; control wolf 0.2.21 `09e0a6f5…`,
lupin 0.1.44 `e44aae06…`. Every member hashed by name (`members.txt`).

## The flip table

| sample | predicted | measured (subject run, unedited tree `a8a88d3`) | |
|---|---|---|---|
| flips, all kinds | 0 | **0** | **held** |
| `ch20/ex20-5` | FAIL: `wolf run` E0817 «`noalloc` is not implemented yet», exit 2; lupin `3` | `FAIL ch20/ex20-5: wolf run: exited Some(2) … error[E0817]: \`noalloc\` is not implemented yet`; lupin `3`; both `conform-run` lanes `fail(E0817)` | **held** |
| `ch20/ex20-8` | FAIL: the same; lupin `13` | the same; lupin `13` | **held** |
| `book/ch09/s3` | FAIL: compiler verdict `pass` (a module-private `*T` signature); lupin E1302 | `FAIL book/ch09/s3: wolf verdict \`pass\`, expected \`fail(E1302)\``; `wolf run` exit 0; lupin E1302, exit 2 | **held** |
| `book/ch06/part-wordcount` | FAIL: lupin E0402 at resolve (exit 2) or a by-name decline (exit 4) | lupin exit **4**, «`trim` takes 0 arguments, but this call passes 1 — `[mem.str.ws]`'s family takes no argument (E0402, which this machine decides at resolve only where it can see the receiver is a `str`)»; the compiler still declines at `resolve` («an item without a declared type») | **held** (the exit-4 branch) |
| the other 18 non-trap `lupin-run` fences | hold | every verdict identical on both pairs (`probes-control.txt` vs `probes-subject.txt`) | **held** |
| 6 `wolf-run` fences | hold | lupin exit 4 on all six | **held** |
| 3 pending rows | hold, same words, same spans | identical | **held** |
| console failures | 5: the four stamp sites and `ch06.md:900` | the same five | **held** |
| `diagnostic,from(book/ch09/s3)` | — | **FAIL**: the page's diagnostic block has nothing to compare against once s3 passes | **missed** (below) |

Runner arithmetic: predicted **509 / 502 passed / 3 pending / 9 failed /
0 flips**, measured **509 / 502 / 3 / 10 / 0**
(`gate-control-subject.txt`). Console blocks predicted 477 of 502,
measured **477 of 502**.

## Everything else

| figure | predicted | measured |
|---|---|---|
| stamps | `wolf 0.2.22 (wolfgang, pin 8e36bc1)` / `paired with lupin 0.1.45 (reference interpreter), pin dfcc2f1` / `lupin 0.1.45 (wolf-interp, reference interpreter at pin dfcc2f1)` | **byte-identical** (`versions.txt`) |
| distance | 222 commits, one release | 222 |
| members | the same ten; `_wolf`, `wolf.bash`, `wolf.fish`, `LICENSE` unchanged; `wolf.1` moves | **held**: `_wolf` `2d1e4801…`, `wolf.bash` `29eb2d75…`, `wolf.fish` `51a1b6a8…` unchanged; `wolf.1` `04c95206…` → `bf1c9e37…` |
| glibc ceiling | ≤ 2.34 | **GLIBC_2.34** on both binaries, both pairs |
| anchors | 546 → 569, +23, none dropped | **569**, the 23 named, none dropped (`anchors-diff.txt`) |
| grammar | holds | holds (`910ff9d5…`) |
| catalogue | 178 → 181 | 181; verify-docs reads Appendix C at 181 / 60 |
| E0817 phase | — | `phase_reached` `typecheck` (both files' `//! phase:`) |
| ch22 export hash | holds | `b7a67a58…` on the 0.2.22 archive |
| head run | 509 / 506 / 3 / 0 / 0; 482 of 502 | **509 / 506 / 3 / 0 / 0; 482 of 502** |
| `ledger --check` | 131 → 132 open, none closed | **132 open (125 filed, 7 waived), 101 closed** |
| verify-docs | 345 (296 printed), tiers 191 / 9 / 31 / 28 / 78 / 8 | **held**; 30 version literals audited (27 → 30: ch09 ×2, ch32 ×1) |
| export | 377 | **377** |
| backmatter | solutions.md drifts on 22-13's toolchain line | **held**; regenerated |
| self-test | — | 14/14 |
| render | — | web, 48 pages, 1328 links none dead, CSP guard |

## The miss, plainly

**The tenth failure.** I counted s3 as one failure. The runner counts
two: the sample (`fail(E1302)` not met) and the page's
`diagnostic,from(book/ch09/s3)` block, which is compared to the
captured diagnostic and has none to compare against once the program
passes. Every `fail(…)` sample with a page block beside it fails twice
when its verdict moves. A counting miss, not a tool change.

## The `#[noalloc]` sites (item 2)

- **ex20-5 → `fail(E0817)`.** The exercise is the broken promise; the
  attribute is its subject, so the attribute stays and the directive
  reads what the compiler does today. It stays the pending row in
  EXERCISES-PENDING.md: E0817 refuses the *attribute*, the verifying
  compiler will refuse the *body*, with a code #180 has not assigned.
- **ex20-8 → `fail(E0817)`.** The stem asks what the toolchain claimed
  about the attribute; that answer is exactly what 0.2.22 changed, so
  the attribute stays and the solution answers per machine (the
  compiler refuses a kept promise because nothing can tell it from a
  broken one; lupin prints 13, reading no attribute, wolf-interp#174).
  Checker `wolf + lupin` in the stem and the index.
- **EXERCISES.md's 20-8 block** still quotes the file's body
  byte-for-byte; its `$ lupin ex20-8.lu` transcript replays (13).

## Sentences the release made false (item 3)

1. ch09 §9.2: "A function's signature is safe or the program does not
   compile" — and its sample, which stopped being refused. Now `pub fn
   header_len` (with its `///`, so no W0313), the new E1302 note, and a
   paragraph on the module-private and C-membrane places a `*T` may
   stand, with lupin's split (wolf-interp#181) named in prose and in a
   new ledger row.
2. ch09 §9.5: "`pack`'s signature is fully safe. It has to be".
3. ch32 §32.2: "no `*T` crosses a function signature".
4. ch06 §6.5: "`trim(".,;!?")` removes any of those characters" — the
   capstone's `count` uses a `bare` helper; lupin 0.1.45 prints the
   same five lines; "Fifty lines, four functions" is now sixty and six.
5. ch20 corpus: the set header, the honesty paragraph, 20-5's
   `verdict=unsupported` (already `pass` at the control), 20-8's
   "claimed nothing", and 20-10's quotation of `corpus/comptime.lu`,
   which at v0.2.22 says the opposite.
6. ch18 and ch20 ledgers ("parse and are verified by nothing"),
   EXERCISES-PENDING.md (20-5's row, "19-1: `#[noalloc]` parses"),
   `samples-pending.toml`'s header, 25-5's (c).
7. Added, not corrected: §20.1's one sentence naming the E0817 refusal
   (Appendix C 59 → 60, the index gains `E0817` and `#[noalloc]`).

## Open issues the pin moves

None of wolf-book #60, #58, #57, #48, #45, #43, #29: their programs
answer in the same words on both pairs.

## Gate at head

`gate-summary.txt` (kasumi, the head before this file) and the PR's CI
runs on three hosts.
