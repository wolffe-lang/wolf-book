# bs56 — the prediction, committed before measuring

Wave 52. Written 2026-10-01 against wolf-book `61189e6` (the contract),
**before** the versioned serve, the 304 server, either unit test or
either fix was written, and before any gate ran on a local server or
against the live site. DERIVED means read off the tree; PREDICTED is a
claim about what a browser or a test runner will do, with the number
that falsifies it.

Definitions, as bs55's: a **cell** is one engine at one viewport; the
gate's page set is **49** (45 sidebar entries, `index.html`, `404.html`,
and two misses, `no-such-page.html` and `front/no-such-page.html`); a
cell makes **2,205** clicks (49 × 45). The **12 pages one level down**
are `front/how-to-read.html`, `front/notation.html`, and the ten
`back/` pages (five appendices, Solutions, Glossary, Index, Errata,
Colophon). The front miss is a thirteenth page one level down.

## 1. #66 — wolf-boot.js under a query string

### 1a. The unit test on today's file (PREDICTED)

Six cases, `path_to_root` read after the file runs with the given
`src`:

| src | want | today |
|---|---|---|
| `wolf-boot.js` | `` | passes |
| `../wolf-boot.js` | `../` | passes |
| `../../wolf-boot.js?v=abc` | `../../` | **red**: `` |
| `/book/wolf-boot.js?v=abc` | `/book/` | **red**: `` |
| `wolf-boot.js#top` | `` | passes **by accident**: the suffix test fails on `#top` and gives ``, which is the want; the case holds the fix to a fragment at the root |
| `../wolf-boot.js?v=1#x` | `../` | **red**: `` |

**3 of 6 red** today (the three with a `?`); 3 pass, one of them by
accident. Falsified by any other count.

### 1b. The gate on today's render, served with versioned references (PREDICTED)

`serve.mjs` rewrites every `src`/`href` ending `.js` or `.css` in a
served page to carry `?v=<tag>`, `wolf-boot.js` included. Then on every
page one level down `path_to_root` is `""` and every sidebar link is
relative to that page's directory:

- **status faults 585 per cell** = 13 pages × 45 (the 12 pages one
  level down and the front miss, whose `404.html` carries
  `/book/wolf-boot.js?v=…`), every one `HTTP 404` landing at
  `/book/front/<file>` or `/book/back/<file>`; **1,620 at 200**.
- title, number, click, load, entries, state, search: **0** each. The
  root pages and the root miss still resolve relative links correctly;
  the first-paint state does not read the root; search runs from
  `ch07.html` at the root.
- The same 585 on all three engines and both viewports (ww35 measured
  one cell, chromium phone: 585).

Falsified by any cell off 585, or by any fault class other than
`status`.

### 1c. After the fix (PREDICTED)

The six cases pass; the gate against the versioned serve: 2,205 clicks,
2,205 at 200, 0 faults of every class, every cell. No file in the
render changes but `wolf-boot.js`.

## 2. #67 — a 304 is a landing

### 2a. The judge's unit test on today's `judge()` (PREDICTED)

| case | want | today |
|---|---|---|
| 304, right title and number | `[]` | **red**: `["status"]` |
| 304, wrong title | `["title"]` | **red**: `["status"]` |
| 304, wrong number | `["number"]` | **red**: `["status"]` |
| 404, right title and number | `["status"]` | passes |

**3 of 4 red**. Falsified by any other count.

### 2b. The gate against a server that revalidates (PREDICTED)

`serve.mjs` sends `Cache-Control: no-cache`, an `ETag` and
`Last-Modified` for every hit, and answers a matching `If-None-Match`
or `If-Modified-Since` with 304 and no body; a miss is always a full
404 (nginx revalidates 200s only). With the #66 fix in and today's
`judge()`, per cell:

- **Chromium: 0 faults**, 2,205 at 200 — it reports a revalidated
  navigation as the cached response's 200.
- **Firefox: status faults between 1,950 and 2,060**, every one
  `HTTP 304`, every one on the right title and number (the gate's own
  test applied to the fault line). The 200s are each worker's first
  visit to each target: 4 workers × about 45 targets, so 150–250 at
  200. ww35 measured 2,042 and 2,038 on the staged nginx.
- **WebKit: status faults between 1,700 and 1,900**, every one 304 on
  the right page; ww35 measured 1,818 and 1,817.
- title, number, click, load, entries, state, search: 0 on every cell.

Falsified by a Chromium status fault, a Firefox or WebKit cell outside
its band, any 304 fault line whose title or h1 is not the label's, or
any fault in another class.

### 2c. After the fix (PREDICTED)

The four cases pass. Every cell: 2,205 clicks, 0 faults of every class;
the summary line splits the landings: Chromium 2,205 at 200 and 0 at
304; Firefox 150–250 at 200 and the rest at 304; WebKit 300–500 at 200
and the rest at 304. A 404 is still a `status` fault, held by the two
miss pages' `load` check and the unit test.

## 3. Live, `https://lupp.us/book/` (PREDICTED)

The site serves bs55's render (`?v=bd3484e` on every script but
`wolf-boot.js`) with ww35's config installed: `no-cache`, `ETag`, the
book's 404 page for a miss under `/book/`. Nothing in this lane changes
the site; the gate is what moves.

### 3a. Before — trunk's gate, Firefox phone, one cell

2,205 clicks; **status faults 1,900–2,100, all 304, all on the right
page**; title, number, entries, state, search 0 (the two misses now
draw the book's sidebar, so ww35's `entries 2, state 6` is gone);
click 0–2 (the public internet).

### 3b. After — the head's gate, six cells

Per cell: 49 pages, 45 entries, 2,205 clicks; **status 0, title 0,
number 0, load 0, entries 0, state 0, search 0**; click **0** on
Chromium and Firefox, **0–3** on WebKit (ww35 saw 1 and 2, all
timeouts over the internet, none a wrong landing). Landings: Chromium
2,205 at 200; Firefox 150–300 at 200, the rest at 304; WebKit 300–600
at 200, the rest at 304. Any landing not at 200 or 304 on a real page,
or any 304 on a wrong page, is a fault and falsifies this.

### 3c. The maintainer's retry — the head's gate, six cells

From `ch07.html`, every entry, untoggled and toggled, then a second
click from wherever the first landed (which is each of the 12 pages one
level down, twice): **hop 1 90 of 90 landed, hop 2 90 of 90 landed, 0
number mismatches**, every cell; **0 `HTTP 404` lines**; on Firefox
and WebKit some hop-2 lines read `HTTP 304`, all on the right page.
Falsified by any 404 line, any mismatch, or any hop not landed.
