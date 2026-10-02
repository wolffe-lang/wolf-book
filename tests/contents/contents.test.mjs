// Unit tests for the contents gate's two judges and its server
// (bs55). `node --test` runs them; no browser is needed.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { parseLabel, judge } from "./contents.mjs";
import { resolve, serve, version, LUPP_HEADERS } from "./serve.mjs";

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
