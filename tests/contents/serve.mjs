// The book as lupp.us serves it (bs55), for the contents gate.
//
// The render is served under `/book/`, with the response headers
// wolf-web's `nginx/lupp.us.conf` adds to every response (line 58 is
// the CSP), and nginx's `try_files $uri $uri/ $uri.html =404` for the
// book's location. A miss under `/book/` answers the book's own
// `404.html` with status 404 — the posture ww35 gives nginx.
//
// It revalidates the way nginx does under `/book/` (bs56, wolf-book#67):
// `Cache-Control: no-cache` (lupp.us.conf:118), an `ETag` and a
// `Last-Modified` from the file, and a hit whose `If-None-Match` or
// `If-Modified-Since` still holds answers 304 with no body. Firefox and
// WebKit report that 304 as the navigation's status while they render
// the cached copy — the right page — which the gate must not read as a
// miss. A miss never revalidates (nginx's not-modified filter runs on
// 200 only): a 404 is always a full 404.
//
// The headers are the point. GitHub Pages sends no CSP, and a curl
// crawl reads hrefs without running a script, so neither saw what a
// reader on lupp.us sees: `script-src 'self'` refuses every inline
// `<script>`, and `base-uri 'none'` makes a `<base>` inert. A gate that
// serves the render without them tests a site nobody reads.
//
// Usage: node serve.mjs <render-dir> [port] [tag]   (port 0 picks one;
// a tag versions every script and stylesheet reference, `?v=<tag>`)
// As a module: `await serve(root, port, { version })` resolves { url, close }.

import http from "node:http";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// wolf-web nginx/lupp.us.conf:51-58, verbatim. Keep in step with it.
export const LUPP_HEADERS = {
    "X-Content-Type-Options": "nosniff",
    "Referrer-Policy": "strict-origin-when-cross-origin",
    "X-Frame-Options": "SAMEORIGIN",
    "Content-Security-Policy":
        "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; " +
        "img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'none'; " +
        "frame-ancestors 'self'",
};

export const PREFIX = "/book/";

// lupp.us.conf:118, the book's location only.
export const BOOK_CACHE = { "Cache-Control": "no-cache" };

/// nginx's validators for a file: `"<mtime hex>-<size hex>"`, and the
/// mtime to the second.
export function validators(st) {
    const sec = Math.floor(st.mtimeMs / 1000);
    return {
        ETag: `"${sec.toString(16)}-${st.size.toString(16)}"`,
        "Last-Modified": new Date(sec * 1000).toUTCString(),
    };
}

/// The request's conditions still hold for a file with these
/// validators: `If-None-Match` decides when present (any listed tag,
/// weak or strong, or `*`), else `If-Modified-Since`.
export function notModified(headers, v) {
    const inm = headers["if-none-match"];
    if (inm !== undefined) {
        return inm
            .split(",")
            .map((t) => t.trim().replace(/^W\//, ""))
            .some((t) => t === "*" || t === v.ETag);
    }
    const ims = headers["if-modified-since"];
    if (ims !== undefined) {
        const since = Date.parse(ims);
        return !Number.isNaN(since) && Date.parse(v["Last-Modified"]) <= since;
    }
    return false;
}

/// Every script and stylesheet reference in a page, versioned the way
/// wolf-web's build publishes the book: `toc.js` becomes `toc.js?v=<tag>`
/// (`scripts/version-book-assets.py`, ww35). The build had to leave
/// `wolf-boot.js` bare because the file read its root off its own `src`
/// and a query string broke it (wolf-book#66); here nothing is left
/// bare, so the gate holds the file to a versioned `src` from every
/// page, the 404 page's absolute `/book/wolf-boot.js` included. A
/// reference that already carries a `?` or `#` is left alone.
export function version(html, tag) {
    return html.replace(/\b(src|href)="([^"?#]+\.(?:js|css))"/g, `$1="$2?v=${tag}"`);
}

const TYPES = {
    ".html": "text/html",
    ".js": "application/javascript",
    ".css": "text/css",
    ".json": "application/json",
    ".svg": "image/svg+xml",
    ".png": "image/png",
    ".woff2": "font/woff2",
    ".woff": "font/woff",
    ".ttf": "font/ttf",
    ".pdf": "application/pdf",
    ".md": "text/plain; charset=utf-8",
};

function isFile(p) {
    try {
        return fs.statSync(p).isFile();
    } catch {
        return false;
    }
}

/// nginx's `try_files $uri $uri/ $uri.html =404` with `index index.html`.
export function resolve(root, urlPath) {
    let rel;
    try {
        rel = decodeURIComponent(urlPath.slice(PREFIX.length));
    } catch {
        return null;
    }
    const base = path.resolve(root);
    const at = path.resolve(base, rel);
    if (at !== base && !at.startsWith(base + path.sep)) {
        return null;
    }
    for (const cand of [at, path.join(at, "index.html"), at + ".html"]) {
        if (isFile(cand)) {
            return cand;
        }
    }
    return null;
}

export function serve(root, port = 0, opts = {}) {
    const tag = opts.version || null;
    const server = http.createServer((req, res) => {
        const urlPath = new URL(req.url, "http://x").pathname;
        const send = (status, file) => {
            const v = validators(fs.statSync(file));
            if (status === 200 && notModified(req.headers, v)) {
                res.writeHead(304, { ...LUPP_HEADERS, ...BOOK_CACHE, ...v });
                res.end();
                return;
            }
            let body = fs.readFileSync(file);
            const type = TYPES[path.extname(file)] || "application/octet-stream";
            if (tag && type === "text/html") body = Buffer.from(version(body.toString("utf8"), tag));
            res.writeHead(status, { ...LUPP_HEADERS, ...BOOK_CACHE, ...v, "Content-Type": type });
            res.end(req.method === "HEAD" ? undefined : body);
        };
        if (urlPath === "/book") {
            res.writeHead(301, { ...LUPP_HEADERS, Location: PREFIX });
            res.end();
            return;
        }
        if (!urlPath.startsWith(PREFIX)) {
            res.writeHead(404, { ...LUPP_HEADERS, "Content-Type": "text/plain" });
            res.end("not the book\n");
            return;
        }
        const hit = resolve(root, urlPath);
        if (hit) {
            send(200, hit);
        } else if (isFile(path.join(root, "404.html"))) {
            send(404, path.join(root, "404.html"));
        } else {
            res.writeHead(404, { ...LUPP_HEADERS, "Content-Type": "text/plain" });
            res.end("404\n");
        }
    });
    return new Promise((ok) => {
        server.listen(port, "127.0.0.1", () => {
            const { port: p } = server.address();
            ok({
                url: `http://127.0.0.1:${p}${PREFIX}`,
                close: () => new Promise((done) => server.close(done)),
            });
        });
    });
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
    const root = process.argv[2];
    if (!root || !fs.existsSync(path.join(root, "index.html"))) {
        console.error("usage: node serve.mjs <render-dir> [port]  (no index.html there)");
        process.exit(2);
    }
    const tag = process.argv[4] || null;
    const s = await serve(root, Number(process.argv[3] || 8055), { version: tag });
    console.log(`serving ${root} at ${s.url} with lupp.us's headers${tag ? `, scripts versioned ?v=${tag}` : ""}`);
}
