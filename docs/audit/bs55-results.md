# bs55 — results, scored against the prediction

The prediction is `docs/audit/bs55-prediction.md`, committed at
`51725fe` and pushed before either archive was downloaded and before
the contents gate existed; the contract is `docs/audit/bs55-contract.md`
at `50ab661`. Sample runs are on kasumi (linux x86-64) against release
archives verified by digest (`bs55-evidence/archive-digests.txt`,
`members.txt`, `versions.txt`); the lines that carry each number are in
`bs55-evidence/gate-summary.txt`. Browser runs are Playwright 1.63
(Chromium, Firefox, WebKit) in CI and on nomad-1 against a kasumi render.

## 1. The maintainer's report, retried — REPRODUCED, and the cause

From `https://lupp.us/book/ch07.html`, every sidebar entry, all three
engines, phone and desktop, with and without the sidebar toggled
(`bs55-evidence/retry-live.txt`): **hop 1, 90 of 90 land at 200** on
every engine and viewport, 16 with the wrong number (8 per variant).
**Hop 2 — one more click from wherever hop 1 landed — 66 of 90**: all
24 misses are from the 12 pages one directory down, e.g.

    chromium phone untoggled hop2 from /book/front/how-to-read.html "1. Hello, Wolf" -> HTTP 404 /book/front/ch01.html

That is the 404. lupp.us sends `Content-Security-Policy: script-src
'self'` (wolf-web `nginx/lupp.us.conf:58`), every page logs the refusal
of the theme's inline scripts and then `path_to_root is not defined`
(first line of each block in `retry-live.txt`), and mdBook's `toc.js`,
which rewrites every sidebar link as `path_to_root + href`, throws on the
first one and leaves them all relative. Relative is right at the book's
root (ch07), wrong one directory down. The orchestrator's crawl could
not see it: curl reads hrefs without running scripts, and GitHub Pages
sends no CSP. The same refusal is the desktop sidebar's disagreement
(the static `sidebar-visible` class, a checkbox no script checked) and
a dark search box.

**Fixed** (`f6f70aa`): no inline script is left in the theme (there were four on every page).
`path_to_root` comes from `wolf-boot.js`'s own `src` attribute, the
sidebar's first paint sets checkbox and class together, and the ARIA
sync moved to `book.js`; `render` refuses any page with an inline
script (`2186000`). Retried on the fixed render, same script, same
matrix (`retry-fixed.txt`): **hop 1 90/90, hop 2 90/90, 0 number
mismatches**, all six engine × viewport cells.

## 2. The prediction, scored

### Part A — the pin

| § | predicted | measured | verdict |
|---|---|---|---|
| A2 | stamps `wolf 0.2.19 (wolfgang, pin c2401f0)` / `paired with lupin 0.1.42 …, pin ec56a08` / `lupin 0.1.42 (… at pin ec56a08)`; distance 105, one release | byte-identical (`versions.txt`) | ✅ |
| A3 | anchors 542 → 542, grammar holds, catalogue `Fixtures:` only | `backmatter --check` up to date and matching the v0.2.19 sibling; `vendor/spec/` untouched | ✅ |
| A4 | **0 flips** | 0 (subject run) | ✅ |
| A4 | EG2 is real and reaches no sample | the planted `mut_elem_excl.lu` is E1002 at 0.2.18 and prints `2` at 0.2.19 on every machine (`probes.txt`); ch28's six unchanged | ✅ |
| A5 | **4 failures**, all console: colophon:7, ch01:153, ch22:288, ch22 EXERCISES:218 | exactly those four | ✅ 4/4 |
| A5 | 0 program failures; 506 / 503 / 3; console 477 → 481 of 501; export 374 | exactly, control and subject | ✅ |
| A6 | pending 3 → 3, same words, same spans | identical on both pairs, word for word (`probes.txt`) | ✅ |

### Part B — the contents (per engine and viewport)

| § | predicted (today's render) | measured | verdict |
|---|---|---|---|
| B1 | 8 entries mislabelled (ch33 as 26, ch26…32 as 27…33) | 8 | ✅ |
| B1 | 540 404s = 12 pages × 45 | **540** on all three engines, both viewports (CI 36749726580; local Firefox) | ✅ |
| B1 | 280 number mismatches = 35 root pages × 8 | **288** | ⚠️ the gate's page set is 49, not 47: it also loads two misses, and the one at the root (`/book/no-such-page.html`) draws a working sidebar, so +8 |
| B1 | 47 of 47 pages disagree on sidebar state, three stored states | **147** = 49 × 3 | ⚠️ same two extra pages; every one disagrees, as predicted |
| B1 | the miss one level down shows 0 entries | 0 | ✅ |
| B1 | search never requests its index | index never requested, `path_to_root is not defined` | ✅ |
| B1 | same on all engines | same on all three and both viewports | ✅ |
| B2 | live, from ch07: 45/45 land, 8 mismatches; the 404 on hop 2 from 12 pages | 90/90 (two variants), 16 = 2 × 8; hop 2 misses 24 = 2 × 12 | ✅ |
| B3 | after: 0 of every class, all engines and viewports | 0 of every class, 2,205 clicks per cell at 200 | ✅ |
| B3 | no id, anchor or file name changes | 1,130 `id`s on 48 pages, 0 changed; files 77 → 78, `wolf-boot.js` added (`ids-diff.txt`) | ✅ |

Not predicted, and measured:

- **The first gate hung Firefox for seventeen minutes** (CI job
  110005253755, cancelled): its wait for the sidebar's transition was
  unbounded, and several pages shared one context, where Firefox does
  not tick a background page's animation. Bounded waits and one page
  per worker (`786a515`); the red on today's render was then measured
  on Firefox locally, the same numbers as the other two
  (`contents-before.txt`).
- **`fill()` searches nothing** on any engine: mdBook's searcher answers
  key events. The gate types (`d2c23cc`). WebKit's first green-side run
  read that as a broken search.
- **`base-uri 'none'` makes `site-url` inert on lupp.us**, so the 404
  page's links are written absolute under `/book/` by the render
  (`root_the_404`), with `site-url` kept for any host that honours
  `<base>`.

## 3. What moved on the page

| site | before | after |
|---|---|---|
| sidebar labels, ch33 and ch26–32 | 26, 27…33 (position in SUMMARY) | 33, 26…32 (each chapter's own `# N.`; `f226c34`) |
| every page's scripts | 4 inline, all refused on lupp.us | 0 inline; `wolf-boot.js` |
| desktop first paint | class open, checkbox closed, drawn closed | agree: open unless stored `hidden`; phone closed |
| `404.html` | `<base href="/">`, relative links | `<base href="/book/">`, every link `/book/…` |
| GitHub Pages | a second full copy, "production home" | 47 per-page redirects + a forwarding 404 (`3f40bb4`; `pages-probe.txt`: all three engines, JS on and off; the fragment survives with JS) |
| §1.2, colophon | 0.2.18 / 0.1.41, 152 commits, two releases | 0.2.19 / 0.1.42, 105 commits, one release |
| ch22, ex22-13 | `toolchain 0.2.18` | `toolchain 0.2.19`, both hashes hold |

## 4. For ww35 (the deploy), not done here

- nginx: `error_page 404 /book/404.html` inside `location /book/`
  (today a miss under `/book/` gets the SITE's `/404.html`). The book's
  404 page now works at any depth.
- `location ~* \.(js|css|woff2?)$` caches for 7 days, and `toc.js` and
  `book.js` are not fingerprinted: a returning reader can keep the old
  `toc.js` (old numbers) for up to a week after the deploy. The links
  are right either way (`wolf-boot.js` is a new file); only the labels
  lag.
