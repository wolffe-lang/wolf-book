# bs55 — the prediction, committed before measuring

Wave 52. Written 2026-09-30 against wolf-book `50ab661` (the contract),
**before** `wolf-0.2.19-*.tar.gz` or `lupin-0.1.42-*.tar.gz` was
downloaded, before the contents gate existed or ran anywhere, and before
any other file in this tree was edited. DERIVED means computed from the
upstream histories; PREDICTED is a claim about what a binary or a
browser will do, with the number or name that falsifies it.

## Part A — the pin

### A1. The pin (DERIVED)

```
[wolf]   rev ec56a08f04ff318ea659fd58683f7ae4f22dc7a5 -> c2401f05f37794a078d2acf62f837dad98e5950d
         impl_version 0.2.18 -> 0.2.19
[lupin]  rev 0cfc0cfc89af5fd2aeb71d46c86742745b902869 -> 8e2516dc47bf808512388cc687e070981d331d98
```

### A2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.19 (wolfgang, pin c2401f0)
paired with lupin 0.1.42 (reference interpreter), pin ec56a08
$ lupin --version
lupin 0.1.42 (wolf-interp, reference interpreter at pin ec56a08)
```

From `crates/wolf_driver/PAIRING` at `c2401f05`, `vendor/upstream/PIN`
at `v0.1.42`, and D57. The compiler's half is **exact** (0.1.42, and
the pin it names is the pin lupin prints). The interpreter's half
**closes**: `ec56a08` is v0.2.18's tag, **105** commits and **one**
release behind `c2401f05`, where it was 152 and two. §1.2's and the
colophon's "152 commits, two releases" and "the tag two releases back"
become 105, one, and "the compiler's previous release tag". Falsified
by any character that differs, or a count other than 105.

### A3. Spec artifacts (DERIVED both ways)

Anchors **542 → 542**, key→document maps identical; grammar holds
(`910ff9d5…`); catalogue moves in three `Fixtures:` lines only. So
Appendices A, C and D do not move and `vendor/spec/anchors.json` does
not change.

### A4. Flips at the subject run — **0** (PREDICTED)

- **EG2 (element-granular `mut` claims).** The only element claims in
  any book program are ch28's `add(mut n.left[0], w)` /
  `add(mut n.right[0], w)` / `add(mut forest[0], w)` — **one** claim per
  call — in the six `lupin-run` wordtree fences and exercises 28-1…28-6.
  Their compiler refusal is «`copy` of a value nested this deep»
  (push's plain copy, wolf-book#57), which EG2 does not touch. No
  program passes two elements `mut` in one call (search: `mut X[...]`
  over `book/` and `principles/exercises/`, 21 hits, all ch28).
- **Header reads beside a moved element (#474).** bs54's search found
  no non-`Copy` element moved in any of the 573 program texts; there is
  no moved element for a header read to sit beside.
- **Trap rows and `wolf-run` rows** carry no automatic flip.

**Falsified by any flip.**

### A5. Failures at the subject run — **4**, all console (PREDICTED)

The four stamp sites and nothing else: `book/back/colophon.md:7`,
`book/ch01.md:153` (the `--version` pair), `book/ch22.md:288` and
`principles/exercises/ch22/EXERCISES.md:218` (the interface's
`toolchain 0.2.18` line). The catalogue's prose does not move, so no
`--explain` transcript drifts (bs54 moved six because E1001 gained a
paragraph; this release adds none). **0 program failures**:

- lupin 0.1.42's new traps need a moved part (#143, #149 — none, A4)
  or a `Map` whose values are not `Copy` (#144): every spelled `Map` in
  the book is `Map[str, int]` (28 sites), so a read copies.
- #146 (the caller sees a callee's move-out) needs a `mut` parameter
  left moved-out; bs54's search found none.
- #469 (`--deny-warnings`) — no book transcript passes the flag.
- #470 (release-tier ICE) — fixed code, nothing refused newly.

Totals: **506 samples / 503 passed / 3 pending / 0 failed** at control
(0.2.18 / 0.1.41) and at the head; **477 of 501** console blocks at the
subject run, **481 of 501** after the re-record. Export **374**.
Falsified by any other count.

### A6. Pending rows — 3 → 3, same words, same spans

ex5-8 (`member access on a value whose type is still being inferred`
@300..303), ex7-5 (`borrow expressions` @365..367), ex8-7 (``==` on a
container, tuple or handle`` @1926..1929 on wolf; `handle is not a Map
key` on lupin). Nothing in either release is about std roots, borrow
expressions, `Eq`, or map keys.

## Part B — the contents

Definitions, fixed before the gate is written. A **page** is every
HTML file the render writes that carries the sidebar: the 45 SUMMARY
entries, `index.html` and `404.html` = **47**. An **entry** is each of
the sidebar's 45 links. A click **matches** when the response is HTTP
200, the landing page's `<title>` is `<label without its number> - The
Wolf Book`, and the landing page's first `<h1>` carries the label's
number (both numbered and equal, or both unnumbered). The server serves
`target/render/web` under `/book/` with lupp.us's response headers
(the CSP of wolf-web `nginx/lupp.us.conf:58`) and answers a miss with
the book's `404.html` at status 404.

### B1. Today's render (`dadc38b`), per engine and viewport (PREDICTED)

- **Number mismatches: 8 entries** — `ch33` labelled `26.`, and `ch26`…
  `ch32` labelled `27.`…`33.` — on every page where the click lands.
- **404s: every click from the 12 pages one level down** (2 in `front/`,
  10 in `back/`): **12 × 45 = 540**, because the CSP refuses the inline
  script that defines `path_to_root`, `toc.js` throws, and the links
  stay relative. Every click from the 35 root pages (33 chapters,
  `index.html`, `404.html`) lands: **35 × 45 = 1,575** at 200, of which
  **35 × 8 = 280** mismatch on the number.
- **Sidebar state: 47 of 47 pages disagree** on first load at both
  viewports and under all three stored states (none, `visible`,
  `hidden`): `<html>` carries `sidebar-visible` from the template and
  the checkbox is unchecked because the inline script never runs. At
  1440px the sidebar draws at `translateX(-288px)`.
- **A miss one level down** (`/book/front/no-such-page.html`): the 404
  page loads `toc.js` and its CSS relative to `/book/front/` and gets
  neither: **0** sidebar entries.
- **Search**: `searchindex.js` never requested.
- Same numbers on all three engines and both viewports. Falsified by
  any other count, or by any engine that runs the inline scripts under
  the CSP.

### B2. The maintainer's report on the live site (PREDICTED)

From `https://lupp.us/book/ch07.html`, every entry, all engines, both
viewports, with and without the sidebar toggled: **45 of 45 land (200)**
and 8 mismatch on the number; the 404 appears on the **second** click,
from any page under `front/` or `back/`, 45 of 45. The retry records
both hops.

### B3. After the fixes (PREDICTED)

0 mismatches, 0 non-200s, 47 of 47 pages agree on sidebar state under
all three stored states, the miss page shows 45 entries whose links all
land, search loads its index — on all three engines, both viewports.
No heading id, anchor, or file name in `target/render/web` changes
(the set of `id=` values and the file list, diffed before and after).
