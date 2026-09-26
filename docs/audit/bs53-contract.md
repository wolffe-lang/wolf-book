# bs53 — the book at 0.2.17 / 0.1.40, and the rows the pair unblocked

**Class:** medium (wolf-book), **Opus**. **Wave:** 48. **Authored
2026-09-26 by the lane**, from the wave-48 row and bs51's contract
(`wolf/sprints/book/bs51-the-book-at-0216.md`) as template. One oracle:
the samples gate (`cargo xtask samples`) on three hosts, and on kasumi
against the release archives. One deliverable: the book at wolf 0.2.17 /
lupin 0.1.40 with every pending and one-machine row the pair could have
moved re-measured, graduated where both machines now pass, and chapter
7's lend-rule exercise (7-23) re-measured on both.

## 1. Forbidden, absolutely
- No `rm` outside `~/lanes/bs53/` on kasumi and `/private/tmp/bs53`; no deletion in any tree this lane did not create.
- No `git add -A`; no edit to another lane's file; no `~/.claude`; no edit to the planning repo.
- No build on nomad-1 (this Mac): every `cargo`, every sample run, on kasumi.
- No merge, no rebase-merge; no `2>/dev/null` on a checkout; assert the branch before every commit.
- No "seen red" without a run id, sha, path or digest in the same paragraph.
- The pin is taken from the RELEASE ARCHIVE by digest, never from a clone or `~/.local/bin`; every member hashed BY NAME (`_wolf` is the zsh completion script and hashes the same at every pin).
- The checked machine answers only via `wolf conform-run --json --checked`; `wolf run --checked` is the native build.
- No exercise renumbering (EXERCISES.md §1; chapter 7's bs52 exception is not extended).
- Kill only this lane's own pids on kasumi — never a pattern, never a process group.
- Predict the flips **before** either archive is downloaded, in a committed file (`docs/audit/bs53-prediction.md`).
- No attribution trailers on any commit or PR.

## 2. Inputs, verified (re-derived 2026-09-26, first act)

| claim | command | result |
|---|---|---|
| wolf-book trunk `b91b75d` | `git rev-parse origin/trunk` | `b91b75d870fcabbdace9f7a9a1f75efed2958709` ✓ |
| book pins wolf 0.2.16 `93a5fe50` / lupin 0.1.38 `ba357aa` | `wolf-toolchain.toml` `[wolf].rev`, `[lupin].rev` | ✓ |
| wolf 0.2.17 = `v0.2.17` = `02afce84` | `git rev-parse v0.2.17^{commit}` (wolf-lang) | `02afce84f05c7841856a10671b6d7924f79193cc` ✓ |
| release 397045016, four assets, not draft/prerelease | `gh release view v0.2.17 -R wolffe-lang/wolf-lang` | ✓ darwin-arm64 `525c9143…`, linux-arm64 `d3d22623…`, windows-x64 `a1708837…`, linux-x64 `a95d0f0f…` |
| lupin 0.1.40 = `v0.1.40` = `54f85e6` | `git rev-parse v0.1.40^{commit}` (wolf-interp) | `54f85e694d4c03e5cd40bef461f85ca0ac373332` ✓ |
| release 397033025, five assets | `gh release view v0.1.40 -R wolffe-lang/wolf-interp` | ✓ darwin-arm64 `197f1957…`, linux-arm64 `c6a9b30f…`, windows zip `10466658…`, linux-x64 `509929e6…`, bare `lupin.exe` `1daf174c…` |
| lupin 0.1.40's conformance pin | `git show v0.1.40:vendor/upstream/PIN` | `93a5fe50…` = **v0.2.16's tag** — on the released line |
| the compiler names the lupin the book pins | `git show v0.2.17:crates/wolf_driver/PAIRING` | `lupin-version = 0.1.40`, `lupin-pin = 93a5fe5` ✓ — the compiler's half is EXACT |
| the interpreter's half is a distance | `git merge-base --is-ancestor 93a5fe50 v0.2.17`; `git rev-list --count 93a5fe50..02afce84` | ancestor ✓; **72** commits, one release |
| anchors | `spec/anchors.json` at both tags, key sets diffed both ways | **539 → 541**: `mem.model.place.rhs`, `os.fs.path.domain` added; none dropped; none remapped. The book's vendored copy is byte-identical to v0.2.16's |
| grammar | `spec/grammar.ebnf` sha256 | **moves**: `2b6269d7…` → `910ff9d5…`, +2 lines (`index_place '=' 'take' expr`, `index_place ::= expr '[' expr ']'`); the vendored copy is v0.2.16's |
| catalogue | code headings in `docs/diagnostics.md` at both tags | **176 → 176**, same set; only `Fixtures:` lines differ |
| `wolf --help`, package hashes | `git diff --stat v0.2.16 v0.2.17 -- crates/wolf_driver/src/help.rs crates/wolf_pkg/` | both empty — Appendix E and ch22/ch25's hashes are predicted to hold |
| the std pin (B151) | `gh release list -R wolffe-lang/wolf-std` | **empty** — still nothing to pin by digest; `[wolf-std]` keeps its posture; nothing to re-derive against 0.2.17 |
| wolf-lang#431 (scope handle native hang) | `gh issue view 431` | CLOSED 2026-09-24 by s179 (`__wolf_rt_scope_env_copy`), in v0.2.17 |
| wolf-interp#130 (`Scope`/`Proc[T]` on lupin, typed join) | `gh issue view 130 -R wolffe-lang/wolf-interp` | CLOSED 2026-09-25, **shipped in lupin 0.1.39** — the book has never pinned 0.1.39, so it arrives here |
| wolf-lang#438 (index store copies, `take` moves) | issue + both CHANGELOGs | CLOSED; compiler half in 0.2.17 (s180), interpreter half in 0.1.40 (is55) |
| wolf-lang#437 (file index) | issue + both CHANGELOGs | CLOSED; lupin half 0.1.39, compiler half 0.2.17 |
| wolf-lang#386 / `lupin conform-run` cwd | wolf-interp CHANGELOG 0.1.40 | both in 0.1.40; `conform-run` observes in the process cwd; the explorer keeps its private root |
| open and relevant | issues | wolf-lang#452 (checked machine's store operand order) OPEN, #446 OPEN; wolf-book#60 (ex8-7), #58 (ex5-8), #57 OPEN |

**Drift, reported not absorbed.**

1. **The wave row names "the index-store and file-index samples the
   release unblocked". No such rows exist in this book.** Searched
   `book/`, `principles/`, `samples-*.toml` for `#438`, `#437`, `#386`,
   `#452`, "index store", "file index", `] = take`/`copy`/`move`: zero
   hits in any pending row, one-machine fence, declined row or ledger.
   The search is shown to fire: the index-store pattern
   `\w+\[[a-z0-9_]+\](\.\w+)*\s*=\s*[^=]` hits every index store in
   the tree (ch05, ch06, ch07, ch09, ch13, ch16, ch20, ch21, ch22, ch32
   and the pool LRU of ex8-7), and read one by one each stores a
   `Copy` value (`int`, a byte, a masked wall), writes through a raw
   pointer, writes a field through an element or a pool handle, or —
   the one non-`Copy` case, ex7-18's `xs[i] = xs[i].upper()` — stores
   a call's fresh result, not a place. #438 changes what a *place* on
   the right does; none of these has one. No index or value operand
   has a side effect, so #452's operand order reaches none either. The
   book's runner reads no record span and replays `wolf conform-run`
   minus its JSON line, so #437 reaches no check; it invokes
   `lupin conform-run` only in explorer console blocks, which keep the
   private root, so #386's cwd change reaches no check either. These
   are re-measured anyway (the gate runs every sample) and predicted
   zero.
2. **What the pair actually unblocks for the book is lupin 0.1.39's
   `Scope`/`Proc[T]`** (wolf-interp#130), which the wave row does not
   name, together with #431's native fix — ch11's four scope-handle
   programs and ch14's three proc-handle programs.
3. `samples-pending.toml`'s ex5-8 owner names wolf-book#39, CLOSED
   2026-09-12 (successor #45, open). Pre-existing; corrected in passing.

## 3. Prediction, committed before either archive is downloaded
`docs/audit/bs53-prediction.md`: flips by name (automatic and by
respelling); pending before/after; sample totals; anchors +2 none
dropped; `grammar.ebnf` re-vendored; the stamps to the character; the
console re-records by name; 7-23's verdict on both machines.

## Items
1. **The pin**, both halves by digest; spec artifacts re-vendored.
2. **The one-machine rows the pair moved**, graduated where both
   machines pass (measured on each, never inferred from a flip), or
   re-measured with the refusal's words in the ledger.
3. **Pending rows** re-measured: ex5-8, ex7-5, ex8-7.
4. **7-23** on both machines at the new pair.
5. Ledgers, version literals, index, backmatter, CHANGELOG.

## 4. Evidence index (required)
Archive digests and member hashes by name; the prediction commit; the
control and subject gate logs on kasumi (`~/lanes/bs53/logs/`); the flip
table; anchor key-set diffs both ways; per-graduation runs on each
machine; 7-23's runs; CI run ids on three hosts at head.

## 5. Done-when
Branch `bs53` on origin; PR open, unmerged; CI green on all three hosts
at the head sha (`gh run view`); five sections in the PR body; §2 drift
reported; §3 precedes the pin commit; kasumi build dirs pruned; no
orphans; the worktree removed after the report.
