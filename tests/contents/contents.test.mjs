// Unit tests for the contents gate's two judges and its server
// (bs55). `node --test` runs them; no browser is needed.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { parseLabel, judge, isLanding } from "./contents.mjs";
import { resolve, serve, version, validators, notModified, LUPP_HEADERS } from "./serve.mjs";

test("a label splits into its number and its name", () => {
    assert.deepEqual(parseLabel("33. The serving loop"), { num: "33", name: "The serving loop" });
    assert.deepEqual(parseLabel("Appendix A — Grammar summary"), { num: null, name: "Appendix A — Grammar summary" });
    assert.deepEqual(parseLabel("1. Hello, Wolf"), { num: "1", name: "Hello, Wolf" });
});

test("a click lands when the status, the title and the number agree", () => {
    const ok = { status: 200, title: "The serving loop - The Wolf Book", h1: "33. The serving loop" };
    assert.deepEqual(judge("33. The serving loop", ok), []);
    // Today's sidebar: mdBook's positional 26 over chapter 33.
    assert.deepEqual(judge("26. The serving loop", ok), ["number"]);
    assert.deepEqual(judge("33. The serving loop", { ...ok, status: 404 }), ["status"]);
    assert.deepEqual(judge("33. The serving loop", { ...ok, title: "tinyvm - The Wolf Book" }), ["title"]);
    // Smart punctuation curls the h1's apostrophe, not the title's: the
    // name is held to the title, the number to the h1.
    assert.deepEqual(
        judge("Appendix E — The driver's surface", {
            status: 200,
            title: "Appendix E — The driver's surface - The Wolf Book",
            h1: "Appendix E — The driver’s surface",
        }),
        [],
    );
    // An unnumbered label may not land on a numbered chapter.
    assert.deepEqual(judge("Errata", { status: 200, title: "Errata - The Wolf Book", h1: "7. Errata" }), ["number"]);
});

test("a revalidated landing (304) is a landing, judged by its title and number (#67)", () => {
    // lupp.us sends `Cache-Control: no-cache` under /book/, so a page the
    // browser holds is asked for again and answered 304; Firefox and
    // WebKit report that as the navigation's status while rendering the
    // right page. The page, not the code, decides.
    const ok = { status: 304, title: "The serving loop - The Wolf Book", h1: "33. The serving loop" };
    assert.deepEqual(judge("33. The serving loop", ok), []);
    assert.deepEqual(judge("33. The serving loop", { ...ok, title: "Notation - The Wolf Book" }), ["title"]);
    assert.deepEqual(judge("33. The serving loop", { ...ok, h1: "26. The serving loop" }), ["number"]);
    // A 304 that lands on a wrong page is that page's faults, never a pass.
    assert.deepEqual(judge("Errata", { status: 304, title: "Errata - The Wolf Book", h1: "7. Errata" }), ["number"]);
    // Anything else is still a status fault, whatever the page says.
    assert.deepEqual(judge("33. The serving loop", { ...ok, status: 404 }), ["status"]);
    assert.deepEqual(judge("33. The serving loop", { ...ok, status: 301 }), ["status"]);
    assert.deepEqual(judge("33. The serving loop", { ...ok, status: null }), ["status"]);
    assert.deepEqual([200, 304, 404, 301, 500, null].map(isLanding), [true, true, false, false, false, false]);
});

function tree() {
    const d = fs.mkdtempSync(path.join(os.tmpdir(), "bs55-serve-"));
    fs.mkdirSync(path.join(d, "front"));
    fs.writeFileSync(path.join(d, "index.html"), "<h1>index</h1>");
    fs.writeFileSync(path.join(d, "ch07.html"), "<h1>7. Who owns this?</h1>");
    fs.writeFileSync(path.join(d, "front", "notation.html"), "<h1>Notation</h1>");
    fs.writeFileSync(path.join(d, "404.html"), "<h1>Document not found (404)</h1>");
    return d;
}

test("resolve is nginx's try_files: the file, the directory's index, then .html", () => {
    const d = tree();
    assert.equal(resolve(d, "/book/ch07.html"), path.join(d, "ch07.html"));
    assert.equal(resolve(d, "/book/ch07"), path.join(d, "ch07.html"));
    assert.equal(resolve(d, "/book/"), path.join(d, "index.html"));
    assert.equal(resolve(d, "/book/front/notation"), path.join(d, "front", "notation.html"));
    assert.equal(resolve(d, "/book/front/ch01.html"), null);
    assert.equal(resolve(d, "/book/../etc/passwd"), null);
});

test("the server sends lupp.us's CSP and the book's 404 page on a miss", async () => {
    const d = tree();
    const s = await serve(d, 0);
    try {
        const hit = await fetch(s.url + "ch07.html");
        assert.equal(hit.status, 200);
        assert.equal(hit.headers.get("content-security-policy"), LUPP_HEADERS["Content-Security-Policy"]);
        assert.match(hit.headers.get("content-security-policy"), /script-src 'self' 'wasm-unsafe-eval';/);
        assert.match(hit.headers.get("content-security-policy"), /base-uri 'none'/);
        const miss = await fetch(s.url + "front/ch01.html");
        assert.equal(miss.status, 404);
        assert.match(await miss.text(), /Document not found/);
        assert.equal(miss.headers.get("content-security-policy"), LUPP_HEADERS["Content-Security-Policy"]);
    } finally {
        await s.close();
    }
});

test("the versioned serve suffixes every script and stylesheet, wolf-boot.js included", async () => {
    const page =
        '<link rel="stylesheet" href="../css/chrome.css"><script src="../toc.js"></script>' +
        '<script src="../wolf-boot.js"></script><script src="/book/wolf-boot.js"></script>' +
        '<a href="../ch07.html">7</a><script src="x.js?v=old"></script><img src="a.png">';
    assert.equal(
        version(page, "abc"),
        '<link rel="stylesheet" href="../css/chrome.css?v=abc"><script src="../toc.js?v=abc"></script>' +
            '<script src="../wolf-boot.js?v=abc"></script><script src="/book/wolf-boot.js?v=abc"></script>' +
            '<a href="../ch07.html">7</a><script src="x.js?v=old"></script><img src="a.png">',
    );
    const d = tree();
    fs.writeFileSync(path.join(d, "ch07.html"), '<script src="wolf-boot.js"></script><h1>7. Who owns this?</h1>');
    const s = await serve(d, 0, { version: "t1" });
    try {
        const hit = await fetch(s.url + "ch07.html");
        assert.equal(hit.status, 200);
        assert.match(await hit.text(), /src="wolf-boot\.js\?v=t1"/);
        const bare = await serve(d, 0);
        try {
            assert.match(await (await fetch(bare.url + "ch07.html")).text(), /src="wolf-boot\.js"/);
        } finally {
            await bare.close();
        }
    } finally {
        await s.close();
    }
});

test("the server revalidates like nginx under /book/: no-cache, validators, 304 on a hit, never on a miss", async () => {
    const d = tree();
    const s = await serve(d, 0);
    try {
        const hit = await fetch(s.url + "ch07.html");
        assert.equal(hit.status, 200);
        assert.equal(hit.headers.get("cache-control"), "no-cache");
        const etag = hit.headers.get("etag");
        const lm = hit.headers.get("last-modified");
        assert.match(etag, /^"[0-9a-f]+-[0-9a-f]+"$/);
        assert.equal(Number.isNaN(Date.parse(lm)), false);
        const again = await fetch(s.url + "ch07.html", { headers: { "If-None-Match": etag } });
        assert.equal(again.status, 304);
        assert.equal(await again.text(), "");
        assert.equal(again.headers.get("content-security-policy"), LUPP_HEADERS["Content-Security-Policy"]);
        assert.equal(again.headers.get("etag"), etag);
        const since = await fetch(s.url + "ch07.html", { headers: { "If-Modified-Since": lm } });
        assert.equal(since.status, 304);
        const stale = await fetch(s.url + "ch07.html", { headers: { "If-None-Match": '"0-0"' } });
        assert.equal(stale.status, 200);
        const miss = await fetch(s.url + "front/ch01.html");
        assert.equal(miss.status, 404);
        const missAgain = await fetch(s.url + "front/ch01.html", { headers: { "If-None-Match": miss.headers.get("etag") } });
        assert.equal(missAgain.status, 404);
        assert.match(await missAgain.text(), /Document not found/);
    } finally {
        await s.close();
    }
    const v = validators({ mtimeMs: 1790800052000, size: 95507 });
    assert.deepEqual(v, { ETag: '"6abd70b4-17513"', "Last-Modified": "Wed, 30 Sep 2026 20:27:32 GMT" });
    assert.equal(notModified({ "if-none-match": 'W/"6abd70b4-17513"' }, v), true);
    assert.equal(notModified({ "if-none-match": '"x", "6abd70b4-17513"' }, v), true);
    assert.equal(notModified({ "if-none-match": '"x"' }, v), false);
    assert.equal(notModified({ "if-modified-since": "Wed, 30 Sep 2026 20:27:31 GMT" }, v), false);
    assert.equal(notModified({ "if-modified-since": "Wed, 30 Sep 2026 20:27:32 GMT" }, v), true);
    assert.equal(notModified({ "if-none-match": "*" }, v), true);
    assert.equal(notModified({}, v), false);
});
