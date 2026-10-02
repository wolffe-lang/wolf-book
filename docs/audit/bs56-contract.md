# bs56 — wolf-boot.js under a query string, and a 304 is a landing

**Class:** small (wolf-book), **Fable**. **Wave:** 52. **Authored
2026-10-01 by the lane**, from the wave-52 row and bs55's contract
(`docs/audit/bs55-contract.md`) as template. Two issues, one oracle each,
both ww35's findings against bs55's work:

- **wolf-book#66.** `book/wolf-boot.js` works out `path_to_root` by
  cutting `wolf-boot.js` off the end of its own `src`. With
  `src="../wolf-boot.js?v=bd3484e"` the suffix test fails, the root is
  `""`, and every sidebar link from a page in `front/` or `back/` lands
  one folder too deep — the maintainer's original 404, back again the
  moment anyone cache-busts the file. Oracle: a unit test that runs the
  file with a stub `document` and reads `path_to_root` for
  `wolf-boot.js?v=abc`, a fragment, and nested paths; and the contents
  gate serving the render with every script reference versioned,
  `wolf-boot.js` included. Both seen red first.
- **wolf-book#67.** `tests/contents/contents.mjs`'s `judge()` counts any
  landing that is not exactly HTTP 200 as a `status` fault. A browser
  that revalidates a cached page gets 304 and renders the right page;
  Firefox (and WebKit, sometimes) report that 304 as the navigation's
  status. lupp.us now sends `Cache-Control: no-cache` with an `ETag`
  under `/book/` (ww35's config, installed), so the live gate reads red
  on correct landings. A 304 on the right page is a landing; a 304 on
  the wrong page is still a fault. Oracle: `judge()`'s unit test, and
  the gate against a server that answers `If-None-Match` with 304 the
  way nginx does. Both seen red first.

Then the gate and the maintainer's retry against the live
`https://lupp.us/book/`, three engines, phone and desktop, every entry
from every page including the second hop from `front/` and `back/`,
with the numbers recorded either way.

## 1. Forbidden, absolutely
- No `rm` outside `~/lanes/bs56/` on kasumi, `/private/tmp/bs56`, and this lane's `bs56-*` files in the shared scratchpad; no deletion in any tree this lane did not create.
- No `git add -A`; no edit to another lane's file; no `~/.claude`; no edit to the planning repo or to wolf-web (versioning `wolf-boot.js` on the site is wolf-web's call once this merges).
- No `cargo build` on nomad-1 (this Mac): every cargo and render run is on kasumi. Playwright runs here only, from the cached `playwright-core` 1.63.0, against a static serve of a kasumi render or the live site; nothing is installed on the pool.
- No deploy to almanta; no GitHub Pages deploy from the branch.
- No merge, no rebase-merge; no `2>/dev/null` on a checkout; assert the branch before every commit.
- No "seen red" without a run id, sha, path or digest in the same paragraph.
- No change to what the gate asserts beyond the two issues: the title and number tests stay the judge; a 304 that lands on the wrong page is a fault; no page, label, anchor or file name moves.
- No cache disabled in the gate to make Firefox green: the gate meets the site's headers as the site sends them.
- No `ssh -f`; long jobs under `setsid` with the pid recorded; kill only this lane's own pids, never a pattern, never a process group.
- Wait in printing loops; poll CI with `gh run view`, never `gh run watch` without `--interval 60`.
- Predict before measuring: the reds' numbers by class and count before any gate runs on either server; the live numbers before the live run.
- No attribution trailers on any commit or PR.

## 2. Inputs, verified (re-derived 2026-10-01, first act)

| claim | command | result |
|---|---|---|
| wolf-book trunk `bd3484e` | `git rev-parse origin/trunk` | `bd3484edd6c3ee38d468167fe04d3f1d7572740f` ✓ (bs55's merge, PR #65) |
| the root is cut off the `src` by a suffix test | `book/wolf-boot.js:28` | `src.slice(-name.length) === name ? src.slice(0, -name.length) : ""` ✓ — a `?` or `#` after the name gives `""` |
| the file is linked bare by the theme | `theme/index.hbs:60` | `<script src="{{ path_to_root }}wolf-boot.js"></script>` ✓; the 404 page's copy is absolutized to `/book/wolf-boot.js` (`xtask/src/render.rs:1068`) |
| `judge()` demands exactly 200 | `tests/contents/contents.mjs:78` | `if (landed.status !== 200) faults.push("status")` ✓; the retry counts `=== 200` at lines 356, 359, 364 |
| `serve.mjs` never answers 304 | `tests/contents/serve.mjs:82-87` | no `ETag`, no `Last-Modified`, no conditional branch ✓ — CI has never seen a 304 |
| ww35's 585 | wolf-web `docs/audit/ww35-evidence/gate-staged-wolfboot-red.txt` | chromium phone `2205 clicks, 1620 at 200 — FAIL {"status":585,…}`, e.g. `front/how-to-read.html -> "1. Hello, Wolf": HTTP 404 …/book/front/ch01.html` ✓ |
| ww35's 304s | wolf-web `ww35-evidence/gate-staged.txt`, `summary.txt` | staged `no-cache` config: firefox 2042/2038, webkit 1818/1817 status faults, chromium 0; every 304 on the right title and number (`ww35-faults.py`) ✓ |
| wolf-web trunk `35bbd3c` versions every book script but `wolf-boot.js` | `curl -s https://lupp.us/book/front/how-to-read.html \| grep -o '<script[^>]*>'` | `../toc.js?v=bd3484e`, `../wolf-boot.js` (bare), `../book.js?v=bd3484e` … ✓ (ww35 `d5c9a9b`) |
| lupp.us's headers under `/book/` today | `curl -sI https://lupp.us/book/ch07.html` | `cache-control: no-cache`, `etag: "6abd70b4-17513"`, `last-modified: Wed, 30 Sep 2026 20:27:32 GMT`, the CSP of `nginx/lupp.us.conf:122` ✓ |
| lupp.us answers a conditional GET with 304 | `curl -o /dev/null -w '%{http_code}' -H 'If-None-Match: "6abd70b4-17513"' …/ch07.html` | **304** ✓ |
| a miss under `/book/` answers the book's 404 page | `curl -sI https://lupp.us/book/front/no-such-page.html` | `HTTP/2 404`, `content-length: 5667` (the book's `404.html`, 5,667 B per ww35's `staged-headers.txt`) ✓ |
| wolf-book#66, #67 | `gh api repos/wolffe-lang/wolf-book/issues/6{6,7}` | both OPEN, no comments |
| the gate's CI shape | `.github/workflows/book.yml:294-333` | `contents` needs `render`, matrix chromium/firefox/webkit, `npm test` then `node contents.mjs --root ../../target/render/web --engines …`; no `concurrency` block, so a push does not cancel the previous run |
| bs55's CI timings | `gh run view 36756426979 --json jobs` | render 62 s; contents chromium 10 min, firefox 16 min, webkit 15 min |
| Playwright here | `~/.npm/_npx/e41f203b7505f1fb/node_modules/playwright-core/package.json`; `~/Library/Caches/ms-playwright` | 1.63.0; chromium-1243, firefox-1543, webkit-2359 present; node v26.7.0 |
| kasumi | `ssh kasumi uptime; df -h ~` | up 17 d, load 2.8; `/home` **97 % used, 29 G free**; node v26.8.1, cargo 1.96.1; no `~/lanes/bs56` yet |

**Drift, reported not absorbed.**

1. **The maintainer has installed ww35's nginx config.** ww35's PR left it
   staged ("needs the maintainer's sudo"); today `/book/` sends
   `cache-control: no-cache` with an `ETag`, answers a conditional GET
   with 304, and a miss gets the book's own 404 page. So the live site
   is the #67 posture now, not the "today's config, HTML heuristically
   cached" that gave ww35 139 Firefox faults: trunk's gate against live
   Firefox should read red by the thousands, and the two miss pages now
   draw the sidebar (ww35's `entries 2, state 6` should be gone).
2. **kasumi is at 97 %, 29 G free**, not the "90+ GB free" of the
   wave-45 rule. One render clone fits; the clone and its `target/` are
   pruned the moment the renders are copied here.
3. nginx never revalidates an error page (its not-modified filter runs
   on 200 only), so the server mirror answers 304 for hits only; a miss
   is always a full 404. Not in either issue; it decides what the gate
   may see on the two miss pages.

## 3. Prediction, committed before measuring
`docs/audit/bs56-prediction.md`, in its own commit, before any gate
runs on either server or against the live site: for #66, the unit
test's red cases by name and the gate's red by count per cell; for #67,
the same, per engine; after both fixes, the local six cells; and the
live six cells and the retry, with the 304 counts by engine.

## Items
1. **#66, the gate first.** `serve.mjs` learns to serve the render with
   every script and stylesheet reference versioned (`?v=<tag>`),
   `wolf-boot.js` included — the way wolf-web's build publishes the
   book, plus the one file it had to leave bare; `contents.mjs` serves
   that way by default. Pushed and run red in CI before any fix (585
   per cell, all 404).
2. **#66, the unit test.** `tests/contents/wolf-boot.test.mjs` runs
   `book/wolf-boot.js` in a `vm` context with a stub `document` whose
   `currentScript.src` is each case, and reads `path_to_root`:
   `wolf-boot.js`, `../wolf-boot.js`, `../../wolf-boot.js?v=abc`,
   `/book/wolf-boot.js?v=abc`, `wolf-boot.js#top`,
   `../wolf-boot.js?v=1#x`. Red first (local log committed; CI's
   `npm test` red cited).
3. **#66, the fix.** The file cuts a `?…` or `#…` tail off the `src`
   before the suffix test. Nothing else in the file moves.
4. **#67, the server first.** `serve.mjs` sends `Cache-Control:
   no-cache` (lupp.us `nginx/lupp.us.conf:118`), an `ETag` and
   `Last-Modified` from the file, and answers `If-None-Match` /
   `If-Modified-Since` with 304 on a hit, as nginx does. Pushed and run
   red in CI before the judge moves: Firefox and WebKit red on status
   faults that are all 304 and all on the right page; Chromium green.
5. **#67, the judge.** `judge()` accepts 200 or 304 as a landing and
   keeps the title and number tests; the retry counts a hop as landed
   the same way; the record and the summary line carry `at 200` and
   `at 304` separately. Unit test red first: a 304 on the right page
   gives no fault, a 304 on the wrong title gives `title`, a 304 on the
   wrong number gives `number`, a 404 is still `status`.
6. **Live.** With the head's gate: the full gate (every entry from every
   page, which includes every `front/` and `back/` page) and the retry
   (from ch07, every entry, then a second click from wherever it landed,
   which includes the 12 pages one level down), three engines, phone
   and desktop, against `https://lupp.us/book/`; one Firefox phone cell
   with trunk's gate first, as the before. Numbers recorded either way.
7. CHANGELOG, `docs/audit/bs56-results.md`, the evidence directory.

## 4. Evidence index (required)
The prediction commit; the red CI run ids with the job lines (the
versioned serve at trunk's `wolf-boot.js`; the 304 server at trunk's
`judge()`); the local red logs for the two unit tests; the local six
cells green on the fixed render against the fixed server; the live
gate's six lines and its `--json` record, the live retry's log, the live
headers probed; the CI run id at the head sha on three engines. Every
file under `docs/audit/bs56-evidence/`, named in the PR.

## 5. Done-when
Branch `bs56` on origin; PR open, unmerged; #66 and #67 named in the
body; CI green on all jobs at the head sha (`gh run view`), including
`contents` on chromium, firefox and webkit; each fix's witness seen red
in CI first, run ids cited; five sections in the PR body; §2 drift
reported; §3 precedes every fix commit; kasumi's clone and `target/`
pruned once the renders are copied; no orphans; the worktree removed
after the report.
