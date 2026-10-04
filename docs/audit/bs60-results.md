# bs60 — results against the prediction

Prediction committed at `aaf28b1` (`docs/audit/bs60-prediction.md`),
before `wolf-0.2.23-*.tar.gz` or `lupin-0.1.46-*.tar.gz` was downloaded.
Measured 2026-10-04 on kasumi (linux x86-64) from the release archives,
each matched against the release API the same minute
(`bs60-evidence/archive-digests.txt`, read 19:46Z): wolf 0.2.23
`6f505eb5…`, lupin 0.1.46 `d13a0379…`; control wolf 0.2.22 `df0f2fea…`,
lupin 0.1.45 `907cfb1a…` (read 19:30Z). Every member hashed by name
(`members.txt`). The control's gate is `gate-control.txt` (git status
after it: 0 changed paths; 4 SKIP lines, the four the head gate also
prints).

## The flip table

| sample | predicted | measured (subject run, unedited tree `aaf28b1`) | |
|---|---|---|---|
| flips, all kinds | 0 | **0** | **held** |
| `ch18/ex18-4` | FAIL: snapshot drift, E0708 «the **layout** of `Vec2` …» and the `#[repr(c)]` note | `FAIL ch18/ex18-4: diagnostic drifted from reviewed snapshot`; the blessed diff is exactly that headline and note | **held** |
| `ch20/ex20-5` | FAIL: snapshot drift, E0817's implemented list gains `repr(c, packed)`, `repr(c, align(N))`, `section(".name")` | the same drift, those three items and nothing else | **held** |
| `ch20/ex20-8` | FAIL: the same | the same | **held** |
| `book/ch18/s2`, `book/ch18/s10` | hold (module-level `fail(E0710)`; "the two E0710 snapshots hold") | **FAIL ×2**, and their page `diagnostic,from(…)` blocks **FAIL ×2**: one failed assert is now TWO E0710 records | **missed** (below) |
| console: four stamp sites | FAIL (`colophon.md:7`, `ch01.md:153`, `ch22.md:288`, `ch22/EXERCISES.md:218`) | the same four; both export hashes held (`b7a67a58…`, `05a012a2…`) | **held** |
| console: 18-4's `wolf conform-run` | FAIL | `ch18/EXERCISES.md:111` | **held** |
| console: 20-5's and 20-8's `$ lupin` | FAIL: E0817 «`#[noalloc]` is not an attribute wolf implements …», exit 2 | `ch20/EXERCISES.md:136` and `:210`; lupin prints exactly that, exit 2, at 10:3 and 9:3 | **held** |
| the 19 non-trap `lupin-run` fences | hold | every verdict identical on both pairs (`probes-control.txt` vs `probes-subject.txt`) | **held** |
| 6 `wolf-run` fences | hold | lupin exit 4 on all six | **held** |
| 3 pending rows | hold, same words, same spans | identical (@300..303, @365..367, @1926..1929) | **held** |
| ch01, ch12, ch14, ex1-6, ex12-9 lupin parse-error transcripts (#175 risk) | hold | replayed byte for byte | **held** |

Runner arithmetic: predicted **509 / 503 passed / 3 pending / 10 failed /
0 flips**, measured **509 / 501 / 3 / 14 / 0**
(`gate-control-subject.txt`). Console blocks predicted 475 of 502,
measured **475 of 502**.

## Everything else

| figure | predicted | measured |
|---|---|---|
| stamps | `wolf 0.2.23 (wolfgang, pin 8edac3e)` / `paired with lupin 0.1.46 (reference interpreter), pin 8e36bc1` / `lupin 0.1.46 (wolf-interp, reference interpreter at pin 8e36bc1)` | **byte-identical** (`versions.txt`) |
| distance | 200 commits, one release | 200 |
| members | the same ten; `_wolf`, `wolf.bash`, `wolf.fish`, `LICENSE`, `LICENSE-EXCEPTION` unchanged; `wolf.1`, `README.md` move | **held**: `_wolf` `2d1e4801…`, `wolf.bash` `29eb2d75…`, `wolf.fish` `51a1b6a8…`, `LICENSE` `3972dc97…`, `LICENSE-EXCEPTION` `a0eec20d…`; `wolf.1` `bf1c9e37…` → `91260e8f…`, `README.md` `a204a622…` → `f1871d98…` |
| glibc ceiling | ≤ 2.34 | **GLIBC_2.34** on both binaries, both pairs |
| anchors | 569 → 586, +17, none dropped | **586**, the 17 named, none dropped, no key changed document (`anchors-diff.txt`) |
| grammar | moves: `extern_let_item`, two hunks | **held**; Appendix A regenerated with exactly those two lines |
| catalogue | 181 → 185 | 185; verify-docs reads Appendix C at 185 / 60 |
| witnesses | §6's table | **held, all five** (`probes-subject.txt`): private `*T` exit 0 on lupin 0.1.46; else-less `if` value E0401 on all four tools; module `const HANDLERS` prints `Ingest Report Purge`; `#[repr(c)] Vec2` prints `16`; typed module `let` compiles (but see wolf-lang#585 below) |
| head run | 509 / 506 / 3 / 0 / 0; 482 of 502 | **509 / 506 / 3 / 0 / 0; 482 of 502** |
| `ledger --check` | 132 → 129 open; closed 101 → 104 | **128 open (121 filed, 7 waived, 0 unfiled), 106 closed** (below) |
| verify-docs | 345 (296 printed), tiers 191 / 9 / 31 / 28 / 78 / 8 | **held**; 30 version literals audited at wolf 0.2.23 / lupin 0.1.46 |
| export | 377 | **377** |
| backmatter | Appendix A regenerates; solutions.md drifts on 22-13 and edited solutions | **held**: 18-4 and 22-13 regenerated; `--check` clean against v0.2.23 |
| self-test | — | 14/14 |
| xtask tests | — | 178 passed, `--nocapture` |
| render | — | web, CSP guard, internal links none dead |

## The miss, plainly

**Chapter 18's two module-level witnesses.** I read the five module
`const`s in chapter 18 and concluded the E0710 snapshots would hold
because the fault text had not changed. It had not. What changed is
how many times the fault is reported: kw09 added a second evaluation of
every module initializer (`[mem.static.3]`, `ctfe/mod.rs`'s second
loop), and it reports its fault without asking whether the comptime
call-site pass already reported the same one. So `const SIXTEEN_SHARDS:
bool = expect_mask(16, 14)` prints the 0.2.22 record and then a second
record at the same span, «while evaluating `const SIXTEEN_SHARDS`»,
with the note «`SIXTEEN_SHARDS` is module state: its initializer is
evaluated at compile time and its value is part of the image
([mem.static.3])». `conform-run --json` lists `["E0710", "E0710"]`
where 0.2.22 listed one (`probes-584-585.txt`, each program alone). My
string-level scan could not see it: it compared message fragments, and
no fragment moved. The check that would have seen it is "does any
module initializer in the book fail at comptime", which this file's §5
answered with the wrong half of the evidence.

Filed upstream as **wolf-lang#584** with the witness. The book prints
what the compiler prints: both snapshots and both page blocks carry
the two records, §18.1 says in one paragraph why there are two, and a
new chapter 18 ledger row names the issue.

## Ledger, re-counted: 128 open, not 129

Predicted closures held (bs59's #181 row in ch09; ch03's #153 row;
ch32's papercut). Three moves the prediction did not name, each
measured:

- **ch32's inventory row** said "The row closes" at bs51 and was never
  ticked. Re-measured on both pairs (`probes-stale-rows.txt`) and
  ticked: ch32 has no open row.
- **ch18's E0708 schedule row** (bs11, "a diagnostic that states a
  schedule"): 0.2.23's E0708 note is present tense, names the rule and
  the `#[repr(c)]` repair, and defers nothing. Closed; wolf-lang#157,
  the umbrella, stays open.
- **ch18's #584 row**: added, filed.

132 − 5 + 1 = **128 open**; 101 + 5 = **106 closed**.

## Sentences the release made false (fixed)

1. ch09 §9.2: "lupin 0.1.45 refuses the private `header_len` with
   E1302 too (wolf-interp#181), so a private `*T` signature is one of
   the places this edition's two machines disagree." Now: lupin 0.1.46
   draws the same line; the machines agree. The #181 ledger row closes.
2. ch09's ledger, the cast row: 9-4 and B-11 «answer raw casts that
   change the machine shape». kw06 compiles both; native `pass`,
   checked `ub(mem.ub)`, lupin L2, no fence moves.
3. ch18 §18.2: "Ask for a size and the compiler declines, because
   layout … belongs to the code generator". Now true of a native-layout
   struct; a `#[repr(c)]` one answers at comptime (`[abi.layout.query]`).
4. 18-4's solution: its transcript and "make aggregate layout a codegen
   fact", now naming the `#[repr(c)]` way out.
5. ch18 §18.1 and §18.3: the two E0710 blocks (the miss above).
6. ch20's contracts corpus: the set header, the honesty paragraph,
   20-5's transcript and "lupin executes the program — attributes are
   inert", 20-8's transcript and "The two machines claim different
   things … lupin prints 13", and both `.lu` headers. Both machines
   refuse by name now; 20-8 names the one place their two notes still
   differ (the compiler implements `repr(c, packed)`, `repr(c,
   align(N))` and `section`, lupin does not yet: wolf-interp#188, #190).
7. ch20's and ch18's ledgers, EXERCISES-PENDING.md's 20-5 row, 25-5 (c):
   lupin's half re-measured; #180's rows stay open.
8. ch22's ledger: "a module-level `const` … is refused with
   `item-initializer lowering (globals)`". The globals gap is closed;
   the `List` gap holds the sample in `main`. §22.3 gains one sentence:
   module state keeps "no life before main" because its initializers
   are evaluated into the image.
9. The stamps: ch01 (both blocks, "222" → "200", the Windows archive
   name, the `+dev.unknown` line), the colophon, ch22's and 22-13's
   `toolchain` line, solutions.md.
10. Appendix C: 181 → 185 codes; 60 shown. Appendix A: regenerated.

Stale before this release, found here, fixed with measurements on both
pairs: ch03's #153 row (lupin has refused the else-less `if` value
E0401 since before 0.1.45), ch32's papercut and inventory rows (the
allocator has built and run natively since s173).

## Filed upstream

- **wolf-lang#584**: one failed comptime assert under a module `const`
  is reported twice (above).
- **wolf-lang#585**: a module-level `let`/`const` holding a `"""` string
  prints the raw literal (a `""` pair at each end, indentation kept) on
  `wolf run` and the checked machine; lupin dedents, and the same
  literal inside a fn is right everywhere. A wrong answer with no
  diagnostic, new in 0.2.23 (0.2.22 declined the program). No book
  sample prints it: chapter 6's `USAGE` is untyped and still declined.

## After the prediction, from the coordinator

A heads-up arrived after `aaf28b1`: 0.2.23's prelude gains `size_of`,
`align_of` and `offset_of`, so a program DEFINING a function by one of
those names draws W0304. Checked: no book program or exercise defines
any of the three (the one hit is 18-4's call to `size_of`), and the
subject log carries no W0304. wolf-lang#583 (a macOS release-tier ICE
when two units share a temp directory) is noted for CI.

## Open issues the pin moves

None of wolf-book #60, #58, #57, #48, #45, #43, #29: their programs
answer in the same words on both pairs. #48 is touched in passing: the
20-8 block still quotes its file's body byte for byte.

## Gate at head

`gate-summary.txt` (kasumi, head `4c38c7e`, every gate exit 0, git
status clean after) and the PR's CI runs on three hosts.
