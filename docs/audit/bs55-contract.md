# bs55 — the book at 0.2.19 / 0.1.42, and the contents

**Class:** medium (wolf-book), **Opus**. **Wave:** 52. **Authored
2026-09-30 by the lane**, from the wave-52 row and bs54's contract
(`docs/audit/bs54-contract.md`) as template. Two oracles, one per half:

- **The pin:** the samples gate (`cargo xtask samples`) on kasumi against
  the release archives by digest, control (0.2.18 / 0.1.41) then subject
  (0.2.19 / 0.1.42) on the unedited tree, and on three CI hosts.
- **The contents:** a Playwright gate — WebKit, Chromium and Firefox, a
  390px phone and a 1440px desktop — that serves the render under
  `/book/` **with lupp.us's own response headers** (the CSP in
  wolf-web `nginx/lupp.us.conf:58`), clicks every sidebar entry from every
  page, and asserts HTTP 200 and that the landing page is the page the
  label names. Seen red against today's render in CI before any fix.

Deliverables, from the row: (0) the pin; (1) every sidebar label carries
the chapter's own number (33 for "The serving loop"), anchors and file
names unchanged; (2) `404.html`'s links resolve under `/book/`; (3) at
desktop width the sidebar's state and drawing agree on first load, with
and without stored state; (4) `book-pages.yml` stops describing Pages as
production and serves per-page redirects to `https://lupp.us/book/<same
path>` (meta refresh + canonical); (5) the gate, in the book's CI; and
the maintainer's 404 from `ch07.html` retried under it, recorded either
way.

## 1. Forbidden, absolutely
- No `rm` outside `~/lanes/bs55/` on kasumi, `/private/tmp/bs55`, and this lane's scratchpad folder; no deletion in any tree this lane did not create.
- No `git add -A`; no edit to another lane's file; no `~/.claude`; no edit to the planning repo or to wolf-web (the nginx half is ww35's).
- No `cargo build` on nomad-1 (this Mac): every cargo, render and sample run is on kasumi. Playwright may run here, from the cached `playwright-core`, against the live site or a static serve of a kasumi render; nothing is installed on the pool.
- No deploy to almanta; no GitHub Pages deploy from the branch (the Pages workflow runs on trunk only).
- No merge, no rebase-merge; no `2>/dev/null` on a checkout; assert the branch before every commit.
- No "seen red" without a run id, sha, path or digest in the same paragraph.
- The pin is taken from the RELEASE ARCHIVE by digest, never from a clone or `~/.local/bin`; every member hashed by name.
- The checked machine answers only via `wolf conform-run --json --checked`; `wolf run --checked` is the native build.
- No renumbering of any chapter, section, exercise, anchor or file name (how-to-read's rule: numbers are permanent links).
- Kill only this lane's own pids — never a pattern, never a process group; long jobs `setsid` with the pid recorded.
- Predict before measuring: the flips and stamps before either archive is downloaded; the gate's red before it runs in CI.
- No attribution trailers on any commit or PR.

## 2. Inputs, verified (re-derived 2026-09-30, first act)

| claim | command | result |
|---|---|---|
| wolf-book trunk `dadc38be` | `git rev-parse origin/trunk` | `dadc38bee9eb8d5570cfe638bf9fb79bfe977ab0` ✓ (bs54's merge) |
| book pins wolf 0.2.18 `ec56a08f` / lupin 0.1.41 `0cfc0cf` | `wolf-toolchain.toml` `[wolf].rev`, `[lupin].rev` | ✓ |
| wolf 0.2.19 = `v0.2.19` = `c2401f05` | `git rev-parse v0.2.19^{commit}` (wolf-lang) | `c2401f05f37794a078d2acf62f837dad98e5950d` ✓ |
| release 400208356, four assets, not draft/prerelease | `gh release view v0.2.19 -R wolffe-lang/wolf-lang` | ✓ linux-x64 `9f3873d8…`, darwin-arm64 `8e9a9653…`, linux-arm64 `dcb418a9…`, windows-x64 `9b93ce77…` |
| lupin 0.1.42 = `v0.1.42` = `8e2516dc` | `git rev-parse v0.1.42^{commit}` (wolf-interp) | `8e2516dc47bf808512388cc687e070981d331d98` ✓ |
| release 400022505, five assets | `gh release view v0.1.42 -R wolffe-lang/wolf-interp` | ✓ linux-x64 `9856335a…`, darwin-arm64 `756d6498…`, linux-arm64 `0cb937ec…`, windows zip `b17302d9…`, bare `lupin.exe` `1c2c9b59…` |
| lupin 0.1.42's conformance pin | `git show v0.1.42:vendor/upstream/PIN` | `ec56a08f…` = **v0.2.18's tag** (moved from `93a5fe5`, v0.2.16) |
| the compiler names the lupin the book pins | `git show v0.2.19:crates/wolf_driver/PAIRING` | `lupin-version = 0.1.42`, `lupin-pin = ec56a08` ✓ — EXACT on both fields |
| the interpreter's half is a distance | `git merge-base --is-ancestor ec56a08f c2401f05`; `git rev-list --count ec56a08f..c2401f05` | ancestor ✓; **105** commits, **one** release (0.2.18 → 0.2.19) |
| anchors | `spec/anchors.json` at both tags, key→document maps diffed | **542 → 542, identical** |
| grammar | `spec/grammar.ebnf` sha256 at both tags | **holds**: `910ff9d5…` both |
| catalogue | `git diff v0.2.18 v0.2.19 -- docs/diagnostics.md` | three `Fixtures:` lines only (E1001, E1002, E1013); no prose |
| driver | `git diff --stat v0.2.18 v0.2.19 -- crates/wolf_driver/src/main.rs help.rs crates/wolf_pkg/` | `main.rs` +88 −21 (#469: `--deny-warnings`), `help.rs` and `wolf_pkg` empty |
| what 0.2.19 changes | `CHANGELOG.md` at `v0.2.19`, §0.2.19 | EG2 (`mut` claims element-granular: two literal elements, member read beside an element claim), header reads (`len`/`count`/`is_empty`) beside a moved element, #470 (release-tier ICE, `mut` two fields one region), #469, #471 |
| what 0.1.42 changes | `CHANGELOG.md` at `v0.1.42`, §0.1.42 | re-pin to v0.2.18; #143 (whole read of a place holding a moved part traps), #144 (a `Map` read moves a non-`Copy` value out), #145, #146, #149, #151, #152 |
| the site's CSP | wolf-web `origin/trunk:nginx/lupp.us.conf:58` and `curl -sI https://lupp.us/book/ch07.html` | `script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; … base-uri 'none'` — **no inline script runs on lupp.us** |
| nginx under `/book/` | wolf-web `nginx/lupp.us.conf:86,99` | `try_files $uri $uri/ $uri.html =404`; `error_page 404 /404.html` (the SITE's 404, not the book's; ww35's half) |
| mdBook | `Cargo.lock` | `mdbook 0.4.52`, as a library in xtask; the theme is a full replacement (`theme/index.hbs`, `theme/book.js`) |
| GitHub Pages | `gh api repos/wolffe-lang/wolf-book/pages`; `book-pages.yml:1-6` | live, `build_type: workflow`; the workflow's header calls it "the web edition's production home" |
| wolf-book#45, #57, #58, #60 | `gh issue list` | all OPEN |

**Drift, reported not absorbed.**

1. **The orchestrator's three findings have one cause the brief does not
   name: lupp.us's CSP blocks every inline `<script>` in the theme.**
   Measured before any edit, on the live site, all three engines at
   1440px (Playwright from the cached package): each page logs four CSP
   refusals and `path_to_root is not defined`; `document.documentElement`
   is `wolf sidebar-visible` (the template's static class, never removed),
   `#sidebar-toggle-anchor` is unchecked (the inline script that checks
   it never ran), and the sidebar draws at `matrix(1,0,0,1,-288,0)`. That
   is finding (3) exactly.
2. **That same refusal is the maintainer's 404, which the orchestrator's
   crawl could not see.** `toc.js` rewrites every sidebar link as
   `path_to_root + href`; with `path_to_root` undefined it throws on the
   first link and leaves every href relative. From `ch07.html` (the book's
   root) relative is right, so every entry lands. From any page one level
   down — `front/how-to-read.html`, `front/notation.html`, every appendix,
   Solutions, Glossary, Index, Errata, Colophon — the sidebar's links
   resolve to `/book/front/ch01.html`, `/book/front/front/notation.html`
   and so on: 404. A reader who opens the Contents from ch07, clicks "How
   to read this book", then any other entry, has met it. A curl crawl
   reads the server's hrefs (which are right) and GitHub Pages sends no
   CSP (so a Playwright run there is right too); only a browser under
   lupp.us's headers sees it. The gate therefore serves with those headers.
3. **Finding (2) is also bent by the CSP**: `base-uri 'none'` means the
   browser ignores `404.html`'s `<base>` on lupp.us whatever it says, so
   setting `site-url` alone would not move a single link there. The 404
   page's links and resources must be written absolute under `/book/`.
4. Search is dark on lupp.us for the same reason
   (`window.path_to_searchindex_js` is set inline). Not in the row; the
   fix for (1)–(3) moves it out of line with the rest, and the gate
   checks it loads.
5. The interpreter's half is now **one** release and 105 commits behind,
   not two and 152 (lupin 0.1.42 re-pinned on v0.2.18). §1.2 and the
   colophon say "152 commits, two releases".
6. `backmatter --check` compares the vendored spec with a sibling
   wolf-lang checkout (`WOLF_LANG_PATH`); bs54's kasumi archives and
   clones were pruned, so the control pair (0.2.18 / 0.1.41) is
   re-downloaded and re-hashed against its release digests.

## 3. Prediction, committed before measuring
`docs/audit/bs55-prediction.md`, in its own commit, before either
archive is downloaded and before the gate runs anywhere: the stamps to
the character, flips by class, console failures by name, pending rows,
spec artifacts; and for the contents, the gate's red on today's render
by class and count (label mismatches, 404s, sidebar disagreement) and
its green after.

## Items
1. **The pin**, both halves by digest; control and subject samples runs
   on the unedited tree; stamps, the `--version` transcripts, the pair
   prose, the version-literal ledger, backmatter, CHANGELOG.
2. **The gate first** (`tests/contents/`): a Node server that serves
   `target/render/web` under `/book/` with lupp.us's headers and the
   book's `404.html` for a miss; a Playwright script over three engines
   and two viewports; a CI job fed by the render job's artifact. Pushed
   and run red before any fix.
3. **The numbers** (1): the wolf preprocessor sets each chapter's
   `number` from its own `# N.` heading, so mdBook's sidebar prints 33
   for ch33. No file, anchor or heading changes.
4. **Out of line** (3, and the 404): every inline script in
   `theme/index.hbs` moves to a same-origin file; `path_to_root` is read
   from that file's own `src`; the static `sidebar-visible` class goes,
   so with no script the state and drawing still agree (hidden).
5. **The 404 page** (2): `site-url = "/book/"`, and the render rewrites
   `404.html`'s relative links and resources to `/book/…`, because
   lupp.us ignores `<base>`.
6. **Pages** (4): `book-pages.yml` publishes one redirect stub per page
   (meta refresh + `rel=canonical` to `https://lupp.us/book/<same
   path>`), plus a `404.html` that forwards any other path.
7. **The retry**: the maintainer's report under the gate — from ch07,
   every entry, all engines, both viewports, with and without the sidebar
   toggled — against the live site (today) and the fixed render; recorded.

## 4. Evidence index (required)
Archive digests and member hashes by name; the prediction commit; the
control and subject gate logs on kasumi (`~/lanes/bs55/logs/`) with the
count lines in `docs/audit/bs55-evidence/gate-summary.txt`; the gate's
red CI run id and its log lines; the live-site retry log
(`bs55-evidence/retry-live.txt`) and the fixed-render retry
(`retry-fixed.txt`); CI run ids on three hosts at head.

## 5. Done-when
Branch `bs55` on origin; PR open, unmerged; CI green on all jobs at the
head sha (`gh run view`), including the Playwright gate on three engines;
the gate seen red in CI first, run id cited; five sections in the PR
body; §2 drift reported; §3 precedes the pin commit and the fixes; kasumi
archives pruned once the evidence is written; no orphans; the worktree
removed after the report.
