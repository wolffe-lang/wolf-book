# bs54 — the book at 0.2.18 / 0.1.41, and the rows the point release could move

**Class:** medium (wolf-book), **Opus**. **Wave:** 50. **Authored
2026-09-28 by the lane**, from the wave-50 row and bs53's contract
(`docs/audit/bs53-contract.md`) as template. One oracle: the samples
gate (`cargo xtask samples`) on kasumi against the release archives by
digest, and on three CI hosts. One deliverable: the book at wolf 0.2.18
/ lupin 0.1.41, every sample, pending row and ledger entry the pair
could have moved re-measured — element places (EG1), #460's and #464's
refusals, #452's store order, lupin's moved-element traps
(wolf-interp#141) — graduated by name where both machines now pass, and
any sample 0.2.18 newly refuses named.

## 1. Forbidden, absolutely
- No `rm` outside `~/lanes/bs54/` on kasumi and `/private/tmp/bs54`; no deletion in any tree this lane did not create.
- No `git add -A`; no edit to another lane's file; no `~/.claude`; no edit to the planning repo.
- No build on nomad-1 (this Mac): every `cargo`, every sample run, every probe, on kasumi.
- No merge, no rebase-merge; no `2>/dev/null` on a checkout; assert the branch before every commit.
- No "seen red" without a run id, sha, path or digest in the same paragraph.
- The pin is taken from the RELEASE ARCHIVE by digest, never from a clone or `~/.local/bin`; every member hashed BY NAME (`_wolf` is the zsh completion script and hashes the same at every pin).
- The checked machine answers only via `wolf conform-run --json --checked`; `wolf run --checked` is the native build.
- No exercise renumbering (EXERCISES.md §1; chapter 7's bs52 exception is not extended).
- Kill only this lane's own pids on kasumi — never a pattern, never a process group.
- Predict the flips **before** either archive is downloaded, in a committed file (`docs/audit/bs54-prediction.md`).
- No attribution trailers on any commit or PR.

## 2. Inputs, verified (re-derived 2026-09-28, first act)

| claim | command | result |
|---|---|---|
| wolf-book trunk `f2f4280` | `git rev-parse origin/trunk` | `f2f4280…` ✓ (bs53's merge) |
| book pins wolf 0.2.17 `02afce84` / lupin 0.1.40 `54f85e6` | `wolf-toolchain.toml` `[wolf].rev`, `[lupin].rev` | ✓ |
| wolf 0.2.18 = `v0.2.18` = `ec56a08f` | `git rev-parse v0.2.18^{commit}` (wolf-lang) | `ec56a08f04ff318ea659fd58683f7ae4f22dc7a5` ✓ |
| release 397723077, four assets, not draft/prerelease | `gh release view v0.2.18 -R wolffe-lang/wolf-lang` | ✓ darwin-arm64 `b8f36045…`, linux-arm64 `fe6adec8…`, windows-x64 `6a133a42…`, linux-x64 `da027bf9…` |
| lupin 0.1.41 = `v0.1.41` = `0cfc0cf` | `git rev-parse v0.1.41^{commit}` (wolf-interp) | `0cfc0cfc89af5fd2aeb71d46c86742745b902869` ✓ |
| release 397709135, five assets | `gh release view v0.1.41 -R wolffe-lang/wolf-interp` | ✓ darwin-arm64 `2b8c14b0…`, linux-arm64 `58028bc9…`, windows zip `40b3ee2d…`, linux-x64 `18848901…`, bare `lupin.exe` `0c00b1a1…` |
| lupin 0.1.41's conformance pin | `git show v0.1.41:vendor/upstream/PIN` | `93a5fe50…` = **v0.2.16's tag**, unchanged from 0.1.40 |
| the compiler names the lupin the book pins | `git show v0.2.18:crates/wolf_driver/PAIRING` | `lupin-version = 0.1.41`, `lupin-pin = 93a5fe5` ✓ — the compiler's half is EXACT |
| the interpreter's half is a distance | `git merge-base --is-ancestor 93a5fe50 ec56a08f`; `git rev-list --count 93a5fe50..ec56a08f` | ancestor ✓; **152** commits (72 + 80 since 0.2.17), **two** releases |
| anchors | `spec/anchors.json` at both tags, key sets diffed both ways | **541 → 542**: `mem.model.place.elem` added; none dropped; none remapped |
| grammar | `spec/grammar.ebnf` sha256 at both tags | **holds**: `910ff9d5…` both |
| catalogue | `git diff v0.2.17 v0.2.18 -- docs/diagnostics.md` | same code set; **E1001's entry gains one paragraph** (the `mut` parameter must hold a value at every return, `[mem.tier0.mode.mut]`); E1002 moves only in `Fixtures:` |
| `wolf --help`, package hashes | `git diff --stat v0.2.17 v0.2.18 -- crates/wolf_driver/src/help.rs crates/wolf_pkg/` | both empty; `crates/wolf_driver/src/main.rs` +5 (W1002 suppressed beside the at-return E1001) |
| diagnostics that move | `git diff --name-status v0.2.17 v0.2.18 -- '*.snap'` | modified: only element-naming (`xs[_]` → `xs[0]`), E1002's two-element note, pool CFG dumps, and the interface `toolchain 0.2.18` line |
| the std pin (B151) | `gh release list -R wolffe-lang/wolf-std` | **empty** — nothing to pin by digest; `[wolf-std]` keeps its posture |
| wolf-lang#460, #464, #452 | `gh issue view` | all CLOSED 2026-09-27, all in v0.2.18 |
| wolf-lang#446 | `gh issue view 446` | OPEN (ruling owed); EG1 (literal indices distinct) shipped in 0.2.18 regardless |
| wolf-interp#141 | `gh issue view 141 -R wolffe-lang/wolf-interp` | CLOSED, in 0.1.41; #143, #144, #145, #146 OPEN (pinned by version in 0.2.18's gates) |
| wolf-book#45, #57, #58, #60 | `gh issue view` | all OPEN |

**Drift, reported not absorbed.**

1. **The row's premise is probably empty again, and this time the lane
   says so before measuring rather than after** (bs53's lesson). A
   static search over all 573 program texts (every ` ```wolf ` fence
   in `book/*.md`, 308, plus the 265 `.lu` files under
   `principles/exercises/`) finds **no** non-`Copy` element moved
   anywhere — the 27 element bindings read an `int`, `char`, `str`
   slice, handle or raw pointer, all `Copy` on both machines
   (lupin's `is_copy` at `v0.1.41:src/eval/mod.rs:8620` lists `Str`,
   `Handle`, `Raw`) — and **no** `mut` parameter left moved-out: the
   seven callee moves (`swap` ×2 on ch07's page, `retitle`, ex7-6's
   `grow`, ex7-21's `remove`, ex7-7's `int` swap, ch32's `h.head`)
   each store back before every return or move a `Copy` value. The
   search is shown to fire first: on wolf-lang's own witnesses
   (`corpus/memory/mut_param_moveout_*.lu`, `elem_*no_revive*.lu`,
   `elem_move_*.lu` at `v0.2.18`) it hits 5 of 5 #464 shapes and 10
   of 10 element moves. No index store has a call in its index or a
   `mut` in its value, so #452's order is unobservable. So #460, #464,
   #452 and #141 are predicted to reach **no sample**; the gate runs
   every sample anyway, and the witnesses are planted on the archive.
2. **What the release does reach, which the row does not name**: the
   catalogue text of E1001 (exercise 7-10's and C-1's `wolf --explain
   E1001` transcripts gain a paragraph), and two prose claims that an
   element binding makes the container's `len` unreadable — §5.3's
   `best` paragraph ("the next line's `xs.len` reads a value that
   moved away … a refusal at compile time, at that `xs.len`") and
   ch31's open papercut row (`shards.len` after `let s = shards[i]`).
   EG1 item 1 says "an element move leaves the container's `len`
   readable", so both are re-measured with a witness on both pairs.
3. The interpreter's half is **two** releases and 152 commits behind
   now, not one: lupin 0.1.41 kept 0.1.40's pin. §1.2 and the colophon
   say "72 commits, one release".

## 3. Prediction, committed before either archive is downloaded
`docs/audit/bs54-prediction.md`: flips by name (predicted zero, with
the reason per class); failures at the subject run by name; pending
before/after; sample totals; anchors +1; the stamps to the character;
the console re-records by name; 7-23's verdict; the two element-`len`
claims on both machines; the two planted witnesses.

## Items
1. **The pin**, both halves by digest; `anchors.json` re-vendored.
2. **Every sample re-measured**: control (old pair) and subject (new
   pair) on the unedited tree; any flip graduated only after both
   machines are measured; any newly refused sample named.
3. **Planted witnesses** (#460, #464) on the subject archive, so the
   zero in item 2 is a zero from a search that fires.
4. **Pending rows** ex5-8, ex7-5, ex8-7 re-measured on both pairs.
5. **Ledger entries** the pair could move: §5.3's `best` claim,
   ch31's shard papercut, ch28's six, 7-23, ex8-7.
6. Console re-records, ledgers, version literals, backmatter, CHANGELOG.

## 4. Evidence index (required)
Archive digests and member hashes by name; the prediction commit; the
control and subject gate logs on kasumi (`~/lanes/bs54/logs/`) with the
count lines copied to `docs/audit/bs54-evidence/gate-summary.txt`; the
static detector and its outputs (`bs54-evidence/detect.py`,
`detect-book.txt`, `detect-plant.txt`); probe transcripts on both pairs;
anchor key-set diffs both ways; CI run ids on three hosts at head.

## 5. Done-when
Branch `bs54` on origin; PR open, unmerged; CI green on all three hosts
at the head sha (`gh run view`); five sections in the PR body; §2 drift
reported; §3 precedes the pin commit; kasumi archives pruned once the
evidence is written; no orphans; the worktree removed after the report.
