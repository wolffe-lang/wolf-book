// Unit test for book/wolf-boot.js's root (bs56, wolf-book#66). The
// file works out `path_to_root` from its own `src` attribute; a
// cache-busting query string on that src (`wolf-boot.js?v=bd3484e`,
// what wolf-web's build adds to every other book script) made the
// suffix test fail, the root `""`, and every sidebar link from a page
// in front/ or back/ land one folder too deep. The file is run as a
// browser would, with a stub document whose current script carries
// each src; no browser is needed.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import vm from "node:vm";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const boot = fs.readFileSync(path.join(here, "..", "..", "book", "wolf-boot.js"), "utf8");

/// Run the file with `src` as its script's src; answer `path_to_root`.
export function rootOf(src) {
    const classes = new Set();
    const document = {
        currentScript: { getAttribute: (n) => (n === "src" ? src : null) },
        documentElement: {
            clientWidth: 0,
            classList: {
                add: (c) => classes.add(c),
                toggle: (c, on) => (on ? classes.add(c) : classes.delete(c)),
            },
        },
        getElementById: () => null,
    };
    const ctx = vm.createContext({ document, localStorage: { getItem: () => null } });
    vm.runInContext(boot, ctx, { filename: "wolf-boot.js" });
    return ctx.path_to_root;
}

test("the root is what precedes the file's name, from any depth", () => {
    assert.equal(rootOf("wolf-boot.js"), "");
    assert.equal(rootOf("../wolf-boot.js"), "../");
    assert.equal(rootOf("../../wolf-boot.js"), "../../");
    assert.equal(rootOf("/book/wolf-boot.js"), "/book/");
});

test("a query string or a fragment on the src does not move the root (#66)", () => {
    assert.equal(rootOf("wolf-boot.js?v=abc"), "");
    assert.equal(rootOf("../wolf-boot.js?v=bd3484e"), "../");
    assert.equal(rootOf("../../wolf-boot.js?v=abc"), "../../");
    assert.equal(rootOf("/book/wolf-boot.js?v=abc"), "/book/");
    assert.equal(rootOf("wolf-boot.js#top"), "");
    assert.equal(rootOf("../wolf-boot.js?v=1#x"), "../");
});

test("a src that is not this file gives no root rather than a wrong one", () => {
    assert.equal(rootOf("../toc.js?v=abc"), "");
    assert.equal(rootOf(""), "");
});
