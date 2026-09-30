// The book as lupp.us serves it (bs55), for the contents gate.
//
// The render is served under `/book/`, with the response headers
// wolf-web's `nginx/lupp.us.conf` adds to every response (line 58 is
// the CSP), and nginx's `try_files $uri $uri/ $uri.html =404` for the
// book's location. A miss under `/book/` answers the book's own
// `404.html` with status 404 — the posture ww35 gives nginx.
//
// The headers are the point. GitHub Pages sends no CSP, and a curl
// crawl reads hrefs without running a script, so neither saw what a
// reader on lupp.us sees: `script-src 'self'` refuses every inline
// `<script>`, and `base-uri 'none'` makes a `<base>` inert. A gate that
// serves the render without them tests a site nobody reads.
//
// Usage: node serve.mjs <render-dir> [port]   (port 0 picks one)
// As a module: `await serve(root, port)` resolves { url, close }.

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

export function serve(root, port = 0) {
    const server = http.createServer((req, res) => {
        const urlPath = new URL(req.url, "http://x").pathname;
        const send = (status, file) => {
            const body = fs.readFileSync(file);
            const type = TYPES[path.extname(file)] || "application/octet-stream";
            res.writeHead(status, { ...LUPP_HEADERS, "Content-Type": type });
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
    const s = await serve(root, Number(process.argv[3] || 8055));
    console.log(`serving ${root} at ${s.url} with lupp.us's headers`);
}
