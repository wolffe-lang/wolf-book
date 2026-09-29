# bs54 — results, scored against the prediction

The prediction is `docs/audit/bs54-prediction.md`, committed at
`64a8e35` and pushed to origin before either archive was downloaded;
the contract is `docs/audit/bs54-contract.md` at `974bf4f`. Every run
below is on kasumi (linux x86-64) against release archives verified by
digest (`bs54-evidence/archive-digests.txt`, `members.txt`,
`versions.txt`). Full logs stay on kasumi under `~/lanes/bs54/logs/`
until the prune; the lines that carry each number are in
`bs54-evidence/gate-summary.txt`.

## 1. The archives

| archive | release digest = computed sha256 | member of record |
|---|---|---|
| `wolf-0.2.18-x86_64-unknown-linux-gnu.tar.gz` | `da027bf9…` | `wolf` `a9556245…` |
| `lupin-0.1.41-x86_64-unknown-linux-gnu.tar.gz` | `18848901…` | `lupin` `c5a65edf…` |
| `wolf-0.2.17-x86_64-unknown-linux-gnu.tar.gz` (control) | `a95d0f0f…` | `wolf` `5cdd936e…` |
| `lupin-0.1.40-x86_64-unknown-linux-gnu.tar.gz` (control) | `509929e6…` | `lupin` `18d64444…` |

The control pair was copied from `~/lanes/bs53/archives/` and re-hashed
against the v0.2.17 / v0.1.40 release APIs; its members hash as bs53
recorded. `_wolf`, `wolf.bash` and `wolf.fish` hash the same at both
pins (`2d1e4801…`, `29eb2d75…`, `51a1b6a8…`).

## 2. The prediction, scored

| § | predicted | measured | verdict |
|---|---|---|---|
| 2 | stamps `wolf 0.2.18 (wolfgang, pin ec56a08)` / `paired with lupin 0.1.41 …, pin 93a5fe5` / `lupin 0.1.41 (… at pin 93a5fe5)`; distance 152, two releases | byte-identical (`versions.txt`) | ✅ |
| 3 | anchors 541 → 542, +`mem.model.place.elem`, none dropped or remapped | exactly (`anchors-diff.txt`); `backmatter --check` matches the v0.2.18 sibling | ✅ |
| 3 | grammar holds; Appendix A does not move | `910ff9d5…` both; appendix A up to date | ✅ |
| 3 | catalogue 176 → 176; Appendix D moves in its counts only | 176, same set ✅; Appendix D prints no anchor count, so nothing moved | ⚠️ right that nothing breaks, wrong that anything moves |
| 4 | **0 flips** | 0 flips (subject run) | ✅ |
| 5 | **6 failures**, all console, by name: colophon:7, ch01:153, ch22:288, ch22 EXERCISES:218, 7-10's and C-1's `--explain E1001` | exactly those six (`appx/EXERCISES.md:177`, `ch07/EXERCISES.md:486`) | ✅ 6/6 |
| 5 | **0 program failures**; #460, #464, #452, #141 reach no sample | 0; 503 passed at control and at subject | ✅ |
| 5 | totals 506 / 503 / 3 pending; console 475 of 501 at subject, 481 after; export 374 | exactly | ✅ |
| 6 | 0 graduations by respelling | 0 (every `copy xs[0]` still guards a run-time `xs[i]`) | ✅ |
| 7 | pending 3 → 3, same words | 3; all three identical on both pairs, same spans (`probes-rows.txt`) | ✅ |
| 8 E1 | uncopied `best`: 0.2.17 names `xs.len`; 0.2.18 still refuses, not at `xs.len`, at `xs[i]`; lupin `340 espresso` both | exactly; 0.2.17 gives four diagnostics (E1001 ×3 + E1002), 0.2.18 three (E1001 ×2 at `xs[i]` + E1002) | ✅ |
| 8 E2 | shard loop: 0.2.17 two E1001s; 0.2.18 one, `shards[i]`; lupin runs both | exactly (8:17, move site `shards[i]`); lupin `north`/`south` exit 0 on both | ✅ |
| 9 | #460 and #464 witnesses `fail(E1001)` on default and `--checked` at 0.2.18; lupin 0.1.41 traps the first, prints `1` on the second; both run at 0.2.17 | exactly; at 0.2.17 `wolf run` printed `2 2 1` and `2`, lupin 0.1.40 `2 1 1` and `1` | ✅ |
| 10 | 7-23 `fail(E1002)` byte-identical; lupin exit 0; 7-23b `exit(0)` on all three | exactly, identical against the old pair too | ✅ |
| 11 | re-records by name; `solutions.md` regenerated for 7-10, C-1 and 22-13 | 7-10 and 22-13 ✅; C-1 is an appendix exercise the backmatter does not publish, so `solutions.md` moved by those two only | ⚠️ one name too many |
| 11 | index tiers 192 / 31 / 26; ledger 132 open | `verify-docs` 192 / 31 / 26; `ledger --check` 132 open (125 filed, 7 waived) | ✅ |

Not predicted, and measured:

- **A declined-block row is keyed by line number.** 7-10's transcript
  grew by eight lines, so exercise 7-12's REPL session (declined in
  `samples-declined.toml`, wolf-book#29) moved from line 542 to 550,
  and the first head gate (at `7692829`) failed with two lines for it
  — a row naming no block, and a block with no row — until the row
  followed it (`21c7cc3`). bs53 met the same shape in byte offsets (ex11-2's
  roster); a transcript that grows moves every line-keyed record below
  it in the same file.
- **§5.3's "two errors" was already stale at 0.2.17.** The page said
  the uncopied `best` drew two errors; 0.2.17 drew four, one of them
  the lend rule's E1002 on the returned `b`, which reached this program
  at 0.2.16 (#366). The rewrite names it.

## 3. What moved on the page

| site | at 0.2.17 / 0.1.40 | at 0.2.18 / 0.1.41 | change |
|---|---|---|---|
| §1.2, colophon `--version` | 0.2.17 / 0.1.40 | 0.2.18 / 0.1.41 | re-recorded |
| §1.2, colophon pair prose | 72 commits, one release, "previous" tag | 152 commits, two releases, the tag two releases back | rewritten |
| ch22, ex22-13 interface line | `toolchain 0.2.17` | `toolchain 0.2.18` | re-recorded; both export hashes hold |
| 7-10, C-1 `--explain E1001` | one paragraph | two (the `mut` parameter at return) | re-recorded |
| §5.3 `best` paragraph | "a refusal … at that `xs.len`", "two errors" | refused at `xs[i]`, `len` readable, E1002 named, three errors | rewritten |
| ch31 shard papercut (open, wolf-lang#153) | E1001 ×2 | E1001 ×1, `shards[i]` | re-measured, stays open |
| ch07 (7-23), ch08 (8-7), ch28 (six) ledgers; the three pending rows | — | unchanged, same words and spans | re-measured |

The checked machine (`wolf conform-run --json --checked`) agrees with
the default lane on every refusal above; on the printed `best` it
answers `unsupported` («closures in checked execution»), pre-existing
and the same on both pairs.

## 4. The gate at head

`21c7cc3`, new pair, kasumi: `samples` 506 / 503 passed / 3 pending /
0 failed / 0 flips; console 481 of 501; `samples --self-test` 13/13;
`verify-docs` ok, 27 version literals audited against 0.2.18 / 0.1.41;
`ledger --check` ok (132 open, 0 unfiled); `backmatter --check` up to
date and matching the v0.2.18 sibling. The commits after `21c7cc3`
touch `docs/audit/` only. CI on three hosts is the PR's.
