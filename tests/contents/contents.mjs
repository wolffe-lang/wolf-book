// The contents gate (bs55). From every page of the web edition, click
// every sidebar entry, and hold each click to three things: the landing
// response is HTTP 200, the landing page's <title> is the label's name,
// and the landing page's first <h1> carries the label's number. It also
// holds the sidebar's state to its drawing on first load (the checkbox,
// the `sidebar-visible` class and where the sidebar is actually drawn),
// with no stored state and with `mdbook-sidebar` stored as `visible` and
// as `hidden`; checks that a miss one directory down still draws a
// working sidebar; and that search loads its index.
//
// It runs WebKit, Chromium and Firefox at a 390px phone and a 1440px
// desktop, against the render served the way lupp.us serves it
// (serve.mjs: the site's CSP, `/book/`, nginx's try_files) — or, with
// --base, against any live URL.
//
//   node contents.mjs --root ../../target/render/web
//   node contents.mjs --base https://lupp.us/book/ --engines webkit
//   node contents.mjs --retry --base https://lupp.us/book/   (the maintainer's report)
//
// Options: --engines a,b  --viewports phone,desktop  --workers N
//          --json FILE (the full record)  --log FILE (the retry's lines)
//
// Exit 0 only when every check on every engine and viewport held.
// PLAYWRIGHT_MODULE names a playwright(-core) to load instead of the
// `playwright` package this directory installs.

import { createRequire } from "node:module";
import fs from "node:fs";
import { fileURLToPath } from "node:url";
import { serve } from "./serve.mjs";

const require = createRequire(import.meta.url);

export const VIEWPORTS = {
    phone: { width: 390, height: 844 },
    desktop: { width: 1440, height: 900 },
};
const TITLE_SUFFIX = " - The Wolf Book";
const ENTRY = "#sidebar li.chapter-item > a";
const MISSES = ["no-such-page.html", "front/no-such-page.html"];

function args(argv) {
    const o = {
        engines: ["chromium", "firefox", "webkit"],
        viewports: ["phone", "desktop"],
        workers: 4,
        retry: false,
    };
    for (let i = 0; i < argv.length; i++) {
        const a = argv[i];
        const next = () => argv[++i];
        if (a === "--root") o.root = next();
        else if (a === "--base") o.base = next();
        else if (a === "--engines") o.engines = next().split(",");
        else if (a === "--viewports") o.viewports = next().split(",");
        else if (a === "--workers") o.workers = Number(next());
        else if (a === "--json") o.json = next();
        else if (a === "--log") o.log = next();
        else if (a === "--retry") o.retry = true;
        else throw new Error(`unknown option ${a}`);
    }
    if (!o.root === !o.base) throw new Error("exactly one of --root DIR or --base URL");
    return o;
}

/// "26. The serving loop" -> { num: "26", name: "The serving loop" }.
export function parseLabel(text) {
    const m = /^(\d+)\.\s+(.*)$/.exec(text);
    return m ? { num: m[1], name: m[2] } : { num: null, name: text };
}

/// A click lands on the page its label names: 200, the title is the
/// label's name, and the h1's number is the label's number.
export function judge(label, landed) {
    const want = parseLabel(label);
    const h1num = landed.h1 ? parseLabel(landed.h1).num : null;
    const faults = [];
    if (landed.status !== 200) faults.push("status");
    else {
        if (landed.title !== want.name + TITLE_SUFFIX) faults.push("title");
        if (h1num !== want.num) faults.push("number");
    }
    return faults;
}

const norm = (s) => (s || "").replace(/\s+/g, " ").trim();

/// Let the sidebar's transition finish. Bounded: a page the browser
/// does not consider visible may never tick an animation (Firefox), and
/// an unbounded wait here hung the first CI run for seventeen minutes.
async function settle(page) {
    await page.evaluate(() =>
        Promise.race([
            Promise.all(document.getAnimations().map((a) => a.finished.catch(() => null))),
            new Promise((ok) => setTimeout(ok, 1000)),
        ]),
    );
}

/// No single click may take the run with it.
function bounded(ms, what, work) {
    let timer;
    return Promise.race([
        work.finally(() => clearTimeout(timer)),
        new Promise((_, no) => {
            timer = setTimeout(() => no(new Error(`${what}: no answer in ${ms / 1000}s`)), ms);
        }),
    ]);
}

async function sidebarState(page) {
    await settle(page);
    return page.evaluate(() => {
        const t = document.getElementById("sidebar-toggle-anchor");
        const s = document.getElementById("sidebar");
        if (!t || !s) return null;
        const r = s.getBoundingClientRect();
        return {
            checked: t.checked,
            cls: document.documentElement.classList.contains("sidebar-visible"),
            drawn: r.width > 0 && r.right > 1 && r.left > -1 && r.left < window.innerWidth,
        };
    });
}

async function entries(page) {
    return page.$$eval(ENTRY, (as) =>
        as.map((a) => ({ text: a.textContent.replace(/\s+/g, " ").trim(), href: a.getAttribute("href") })),
    );
}

async function landedOn(page, resp) {
    const got = await page.evaluate(() => {
        const h = document.querySelector("main h1");
        return { title: document.title, h1: h ? h.textContent : null };
    });
    return { status: resp ? resp.status() : null, url: page.url(), title: got.title, h1: norm(got.h1) };
}

async function openSidebar(page) {
    const st = await sidebarState(page);
    if (st && (!st.checked || !st.drawn)) {
        await page.click("#sidebar-toggle");
        await settle(page);
    }
}

async function clickEntry(page, i) {
    const link = page.locator(ENTRY).nth(i);
    const label = norm(await link.textContent());
    const [resp] = await Promise.all([
        page.waitForNavigation({ waitUntil: "domcontentloaded", timeout: 15000 }),
        link.click({ timeout: 10000 }),
    ]);
    return { label, landed: await landedOn(page, resp) };
}

/// The page list: every sidebar entry as the index page lists it, the
/// index and the 404 page themselves, and two misses.
async function sources(browser, base) {
    const ctx = await browser.newContext();
    const page = await ctx.newPage();
    await page.goto(base + "index.html");
    // The href attributes as authored, relative to the book's root: the
    // index page sits at the root, so these are the files.
    const raw = await page.$$eval(ENTRY, (as) => as.map((a) => a.getAttribute("href")));
    await ctx.close();
    const files = raw.map((h) => h.replace(/^(?:\.\/)+/, "").replace(/^https?:\/\/[^/]+/, ""));
    const rel = files.map((h) => (h.startsWith(new URL(base).pathname) ? h.slice(new URL(base).pathname.length) : h));
    return {
        count: raw.length,
        list: [
            ...rel.map((p) => ({ path: p, status: 200 })),
            { path: "index.html", status: 200 },
            { path: "404.html", status: 200 },
            ...MISSES.map((p) => ({ path: p, status: 404 })),
        ],
    };
}

/// Run `tasks` on `n` workers; `fn(task, w)` learns which worker it is.
async function pool(n, tasks, fn) {
    let next = 0;
    await Promise.all(
        Array.from({ length: Math.min(n, tasks.length) }, async (_, w) => {
            while (next < tasks.length) {
                const t = tasks[next++];
                await fn(t, w);
            }
        }),
    );
}

async function gateOne(browser, base, vpName, opt) {
    const vp = VIEWPORTS[vpName];
    const { count, list } = await sources(browser, base);
    const r = {
        engine: browser.browserType().name(),
        viewport: vpName,
        entries: count,
        pages: list.length,
        clicks: 0,
        landed200: 0,
        faults: { status: [], title: [], number: [], click: [], load: [], entries: [], state: [], search: [] },
    };

    // 1. First-load state, under each stored state.
    for (const stored of [null, "visible", "hidden"]) {
        const ctx = await browser.newContext({ viewport: vp });
        await ctx.addInitScript((v) => {
            try {
                if (v === null) localStorage.removeItem("mdbook-sidebar");
                else localStorage.setItem("mdbook-sidebar", v);
            } catch (e) {
                /* storage refused */
            }
        }, stored);
        const page = await ctx.newPage();
        for (const src of list) {
            await page.goto(base + src.path, { waitUntil: "load" });
            const st = await sidebarState(page);
            const expectOpen = vpName === "desktop" && stored !== "hidden";
            if (!st || st.checked !== st.cls || st.checked !== st.drawn || st.checked !== expectOpen) {
                r.faults.state.push(
                    `${src.path} stored=${stored} want open=${expectOpen} got ${JSON.stringify(st)}`,
                );
            }
        }
        await ctx.close();
    }

    // 2. Every entry from every page. One context and one page per
    // worker, reused: each worker's page is its context's only page, so
    // no engine treats it as a background tab.
    const ctx = await browser.newContext({ viewport: vp });
    const tasks = [];
    const perSource = new Map();
    {
        const page = await ctx.newPage();
        for (const src of list) {
            const resp = await page.goto(base + src.path, { waitUntil: "load" });
            const status = resp ? resp.status() : null;
            if (status !== src.status) r.faults.load.push(`${src.path}: HTTP ${status}, want ${src.status}`);
            const es = await entries(page);
            if (es.length !== count) {
                r.faults.entries.push(`${src.path}: ${es.length} sidebar entries, want ${count}`);
            }
            perSource.set(src.path, es.length);
            for (let i = 0; i < es.length; i++) tasks.push({ src: src.path, i });
        }
        await page.close();
    }
    await ctx.close();
    const workers = [];
    for (let w = 0; w < opt.workers; w++) {
        const wctx = await browser.newContext({ viewport: vp });
        workers.push({ ctx: wctx, page: await wctx.newPage() });
    }
    await pool(opt.workers, tasks, async ({ src, i }, w) => {
        const page = workers[w].page;
        try {
            await bounded(60000, `${src} entry ${i}`, (async () => {
                await page.goto(base + src, { waitUntil: "load" });
                await openSidebar(page);
                const { label, landed } = await clickEntry(page, i);
                r.clicks++;
                if (landed.status === 200) r.landed200++;
                for (const f of judge(label, landed)) {
                    r.faults[f].push(
                        `${src} -> "${label}": HTTP ${landed.status} ${landed.url} title="${landed.title}" h1="${landed.h1}"`,
                    );
                }
            })());
        } catch (e) {
            r.faults.click.push(`${src} entry ${i}: ${String(e.message).split("\n")[0]}`);
        }
    });
    for (const w of workers) await w.ctx.close();

    // 3. Search loads its index.
    {
        const sctx = await browser.newContext({ viewport: vp });
        const page = await sctx.newPage();
        const errors = [];
        page.on("pageerror", (e) => errors.push(e.message));
        let index = null;
        page.on("response", (resp) => {
            if (/searchindex\.js(on)?$/.test(new URL(resp.url()).pathname)) index = resp.status();
        });
        await page.goto(base + "ch07.html", { waitUntil: "load" });
        try {
            await page.click("#search-toggle", { timeout: 5000 });
            // Keys, as a reader types them: mdBook's searcher listens for
            // key events, so a programmatic fill() searches nothing on
            // any engine (measured; WebKit's first run read that as a
            // broken search).
            await page.locator("#searchbar").pressSequentially("region", { delay: 20 });
            await page.waitForSelector("#searchresults li", { timeout: 10000 });
        } catch (e) {
            /* judged below */
        }
        const hits = await page.$$eval("#searchresults li", (l) => l.length).catch(() => 0);
        if (index !== 200 || hits === 0) {
            r.faults.search.push(`searchindex HTTP ${index}, ${hits} result(s); page errors: ${errors.join(" | ")}`);
        }
        await sctx.close();
    }
    return r;
}

/// The maintainer's report, retried: from ch07, every entry, with and
/// without the sidebar toggled; then one more click (entry "1.") from
/// wherever the first landed — the second hop is where a relative link
/// written for one directory is read in another.
async function retryOne(browser, base, vpName, lines) {
    const vp = VIEWPORTS[vpName];
    const eng = browser.browserType().name();
    const ctx = await browser.newContext({ viewport: vp });
    const page = await ctx.newPage();
    const errors = [];
    page.on("pageerror", (e) => errors.push(e.message));
    await page.goto(base + "ch07.html", { waitUntil: "load" });
    const count = (await entries(page)).length;
    const first = await sidebarState(page);
    lines.push(`# ${eng} ${vpName}: ch07 has ${count} entries; first load ${JSON.stringify(first)}; page errors: ${[...new Set(errors)].join(" | ") || "none"}`);
    const tally = { hop1: 0, hop1ok: 0, hop2: 0, hop2ok: 0, numberMismatch: 0 };
    for (const variant of ["untoggled", "toggled"]) {
        for (let i = 0; i < count; i++) {
            await page.goto(base + "ch07.html", { waitUntil: "load" });
            let label, landed;
            try {
                if (variant === "toggled") {
                    await page.click("#sidebar-toggle");
                    await settle(page);
                    await openSidebar(page);
                    ({ label, landed } = await clickEntry(page, i));
                } else {
                    // As first drawn: no touch on the toggle. A link the
                    // sidebar does not draw is activated as the DOM would
                    // (keyboard, assistive tech): its own click().
                    const st = await sidebarState(page);
                    const link = page.locator(ENTRY).nth(i);
                    label = norm(await link.textContent());
                    const [resp] = await Promise.all([
                        page.waitForNavigation({ waitUntil: "domcontentloaded", timeout: 15000 }),
                        st && st.drawn ? link.click({ timeout: 10000 }) : link.evaluate((a) => a.click()),
                    ]);
                    landed = await landedOn(page, resp);
                }
            } catch (e) {
                lines.push(`${eng} ${vpName} ${variant} hop1 entry ${i}: ERROR ${String(e.message).split("\n")[0]}`);
                continue;
            }
            tally.hop1++;
            const faults = judge(label, landed);
            if (landed.status === 200) tally.hop1ok++;
            if (faults.includes("number")) tally.numberMismatch++;
            lines.push(`${eng} ${vpName} ${variant} hop1 "${label}" -> HTTP ${landed.status} ${new URL(landed.url).pathname} h1="${landed.h1}"${faults.length ? " FAULT " + faults.join(",") : ""}`);
            if (landed.status !== 200) continue;
            try {
                await openSidebar(page);
                const second = await clickEntry(page, 2);
                tally.hop2++;
                if (second.landed.status === 200) tally.hop2ok++;
                lines.push(`${eng} ${vpName} ${variant} hop2 from ${new URL(landed.url).pathname} "${second.label}" -> HTTP ${second.landed.status} ${new URL(second.landed.url).pathname}`);
            } catch (e) {
                lines.push(`${eng} ${vpName} ${variant} hop2 from ${new URL(landed.url).pathname}: ERROR ${String(e.message).split("\n")[0]}`);
            }
        }
    }
    await ctx.close();
    lines.push(`# ${eng} ${vpName}: hop1 ${tally.hop1ok}/${tally.hop1} at 200 (${tally.numberMismatch} number mismatches); hop2 ${tally.hop2ok}/${tally.hop2} at 200`);
    return tally;
}

async function main() {
    const opt = args(process.argv.slice(2));
    const pw = require(process.env.PLAYWRIGHT_MODULE || "playwright");
    let server = null;
    let base = opt.base;
    if (opt.root) {
        server = await serve(opt.root, 0);
        base = server.url;
    }
    if (!base.endsWith("/")) base += "/";
    console.log(`contents: ${opt.retry ? "retry" : "gate"} against ${base} (${opt.root ? "render served with lupp.us's headers" : "live"})`);
    let failed = false;
    const record = [];
    const lines = [];
    for (const name of opt.engines) {
        const browser = await pw[name].launch();
        for (const vp of opt.viewports) {
            const t0 = Date.now();
            if (opt.retry) {
                const t = await retryOne(browser, base, vp, lines);
                record.push({ engine: name, viewport: vp, ...t });
                console.log(lines[lines.length - 1]);
                if (t.hop1ok !== t.hop1 || t.hop2ok !== t.hop2 || t.numberMismatch) failed = true;
                continue;
            }
            const r = await gateOne(browser, base, vp, opt);
            record.push(r);
            const n = Object.fromEntries(Object.entries(r.faults).map(([k, v]) => [k, v.length]));
            const bad = Object.values(n).some((v) => v > 0);
            failed ||= bad;
            console.log(
                `contents: ${name} ${vp}: ${r.pages} pages, ${r.entries} entries, ${r.clicks} clicks, ` +
                    `${r.landed200} at 200 — ${bad ? "FAIL" : "ok"} ${JSON.stringify(n)} in ${((Date.now() - t0) / 1000).toFixed(0)}s`,
            );
            for (const [k, v] of Object.entries(r.faults)) {
                for (const line of v.slice(0, 12)) console.log(`  ${k}: ${line}`);
                if (v.length > 12) console.log(`  ${k}: … ${v.length - 12} more`);
            }
        }
        await browser.close();
    }
    if (opt.json) fs.writeFileSync(opt.json, JSON.stringify(record, null, 1));
    if (opt.log) fs.writeFileSync(opt.log, lines.join("\n") + "\n");
    if (server) await server.close();
    console.log(`contents: ${failed ? "FAILED" : "ok — every entry from every page lands where its label says"}`);
    process.exit(failed ? 1 : 0);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    main().catch((e) => {
        console.error(e);
        process.exit(2);
    });
}
