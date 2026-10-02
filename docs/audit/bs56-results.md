# bs56 — results, scored against the prediction

The prediction is `docs/audit/bs56-prediction.md`, committed at
`5df61ef` and pushed before the versioned serve, the 304 server,
either unit test or either fix existed, and before any gate ran; the
contract is `docs/audit/bs56-contract.md` at `61189e6`. Renders are
kasumi's (`cargo xtask render web` at `bd3484e` and at `2dec44a`; the
two differ in `wolf-boot.js` only, `diff -rq`). Browser runs are
Playwright 1.63 (Chromium, Firefox, WebKit) in CI and on nomad-1,
against the kasumi renders served by `serve.mjs` and against the live
`https://lupp.us/book/`. Every line quoted below is in
`docs/audit/bs56-evidence/`.

## 1. #66 — wolf-boot.js under a query string

| § | predicted | measured | verdict |
|---|---|---|---|
| 1a | the unit test red on 3 of its 6 named cases (the three with a `?`); `wolf-boot.js#top` passes by accident | `wolfboot-unit-red.txt`: `3 red` of the six, the same three; the fragment case passes by accident; `4 of 12` over the test's full list | ✅ |
| 1b | the gate on today's render, every script reference versioned: **585** status faults per cell, all 404 under `front/` or `back/`, 1,620 at 200, 0 of every other class, all six cells | CI run **36953219958** at `477cada` (`ci-red-66.txt`): `2205 clicks, 1620 at 200 — FAIL {"status":585,…}` on chromium, firefox and webkit, phone and desktop; locally (`gate-versioned-red.txt`) 585 = 135 under `front/` + 450 under `back/`, from the 13 predicted source pages | ✅ exact |
| 1c | after the fix the six cases pass; the gate 2,205 at 200, 0 faults, every cell; only `wolf-boot.js` changes in the render | unit test 3/3 (10/10 with the rest); CI run 36953465335 at `2dec44a` green on three engines; `diff -rq web-before web-after`: `wolf-boot.js` only | ✅ |

**The fix** (`2dec44a`): the file cuts a `?…` or `#…` tail off its
`src` before comparing the name — one `.replace(/[?#].*$/, "")`.
**The hold** (`477cada`): `serve.mjs` serves every script and
stylesheet reference versioned (`?v=gate`), `wolf-boot.js` included,
by default, so the gate in CI meets the posture wolf-web's build had to
avoid. wolf-web may now version `wolf-boot.js` like the rest.

## 2. #67 — a 304 is a landing

| § | predicted | measured | verdict |
|---|---|---|---|
| 2a | the judge's unit test red on 3 of 4 cases (every 304 case reads `["status"]`; the 404 case passes) | `judge-unit-red.txt`: `3 of 4 cases red`, exactly those | ✅ |
| 2b | the gate against a server that revalidates, judge unchanged: Chromium 0 faults; Firefox 1,950–2,060 status faults, all 304, all on the right page; WebKit 1,700–1,900, the same; 0 of every other class | CI run **36953625229** at `e484b28` (`ci-red-67.txt`): chromium `2205 at 200 — ok` ×2; firefox `168 at 200 — FAIL {"status":2037,…}` and `167 / 2038`; webkit `383 / 1822` ×2. Locally (`gate-304-red.txt`, phone): chromium 0; firefox 2,041; webkit 1,824 — every 304 on the right title and number (`bs56-classify.js`), 0 wrong | ✅ in band |
| 2c | after the fix, every cell 0 faults; landings split 200 / 304: Chromium 2,205 / 0; Firefox 150–250 at 200; WebKit 300–500 at 200 | CI run **36953773273** at `d7f0cd5` (`ci-green-judge.txt`): chromium `2205 at 200, 0 at 304`; firefox `165 / 2040`, `168 / 2037`; webkit `388 / 1817`, `378 / 1827`; all `— ok {…0…}` | ✅ in band |

**The fix** (`d7f0cd5`): `isLanding(status)` is 200 or 304;
`judge()` keeps the title and number tests as the judge, so a 304 on
the wrong page is that page's faults; the retry counts hops the same
way; the record and the summary line carry `at 200` and `at 304`
apart. **The hold** (`e484b28`): `serve.mjs` sends `Cache-Control:
no-cache` (lupp.us.conf:118), nginx's `ETag` (`"<mtime hex>-<size
hex>"`, the live `"6abd70b4-17513"` reproduced in the test) and
`Last-Modified`, and answers `If-None-Match` / `If-Modified-Since`
with 304 on a hit and never on a miss, so CI's Firefox and WebKit now
see what lupp.us sends.

## 3. Live, `https://lupp.us/book/`

The site today: `cache-control: no-cache`, `etag`, the CSP, a 304 on a
conditional GET, the book's own 404 page (5,667 B) on a miss under
`/book/`, every book script versioned `?v=bd3484e` but `wolf-boot.js`
(`live-headers.txt`). Nothing in this lane changed the site.

| § | predicted | measured | verdict |
|---|---|---|---|
| 3a | before, trunk's gate, Firefox phone: status faults 1,900–2,100, all 304, all right; entries 0, state 0; click 0–2 | `live-before.txt`: `2205 clicks, 163 at 200 — FAIL {"status":2042,…}`; classified `{"304":2042}`, 2,042 landed right, 0 wrong; every other class 0 | ✅ |
| 3b | after, the head's gate, six cells: 2,205 clicks; status, title, number, load, entries, state, search 0; click 0 on Chromium and Firefox, 0–3 on WebKit; Chromium 2,205 at 200; Firefox 150–300 at 200; WebKit 300–600 at 200, the rest at 304 | `live-gate.txt`: chromium phone and desktop `2205 at 200, 0 at 304 — ok`; firefox phone and desktop `162 at 200, 2043 at 304 — ok`; webkit phone `2203 clicks, 381 at 200, 1822 at 304`, **click 2**; webkit desktop `2204 clicks, 379 at 200, 1825 at 304`, **click 1** — the three are Playwright timeouts over the public internet (`ch29.html` entries 27 and 30, `ch02.html` entry 20 `page.goto`), none a wrong landing; every other class 0 on all six | ✅ (WebKit's clicks in the 0–3 band, as ww35 saw) |
| 3c | the retry, six cells: hop 1 90/90, hop 2 90/90, 0 number mismatches, 0 `HTTP 404` lines; some hop-2 lines 304 on Firefox and WebKit | `live-retry.txt`: every cell `hop1 90/90 landed … (0 number mismatches); hop2 90/90 landed`; chromium 0 at 304; firefox 47 and 89 at 304; webkit 45 and 89 at 304; `live-retry.log`: **0 `HTTP 404` lines, 0 ERROR lines** in 1,080 hop lines, 356 hop-2 lines at 304 | ✅ |

So the maintainer's path, with the second hop from every page one
level down, lands 180 of 180 hops per cell on all six cells; the full
gate lands every click that completed, 13,227 of 13,230 across the six
cells (the 3 WebKit timeouts and no other shortfall).

## 4. Not predicted, and measured

- **`wolf-boot.test.mjs` was dark in CI for one push.** `package.json`'s
  `test` script named `contents.test.mjs` alone, so run 36953386790 at
  `2cc0f7a` (the red test's commit) went red at the *gate* step (a
  second #66 integration red), not at `npm test`. Wired at `fe2c97a`;
  the unit test's red is the committed local log
  (`wolfboot-unit-red.txt`), its CI green is run 36954643067 (10 unit
  tests). The judge test lives in `contents.test.mjs` and did go red in
  CI (run 36953697042 at `e1190f2`, step "the gate's own tests").
- The live cells take 250–620 s here against 150–190 s for a local
  serve; WebKit phone at 622 s is where the two timeouts fell.
- Chromium reports a revalidated navigation as the cached response's
  200, so no Chromium cell anywhere shows a 304: `0 at 304` on every
  Chromium line, live and local.

## 5. Where the figures are

`docs/audit/bs56-evidence/`: `wolfboot-unit-red.txt`,
`judge-unit-red.txt` (the two unit reds, per case); `gate-versioned-red.txt`,
`gate-304-red.txt` (local gate reds with their classification);
`ci-red-66.txt`, `ci-red-67.txt`, `ci-green-judge.txt` (the CI job
lines by run and job id); `live-before.txt`, `live-gate.txt` +
`live-gate.json`, `live-retry.txt` + `live-retry.log` +
`live-retry.json`, `live-headers.txt`; `bs56-classify.js` (the
classifier, ww35's `ww35-faults.py` in the gate's own terms).
