// The Wolf Book — first-paint state (bs55). Loaded synchronously from
// the top of <body>, right after the sidebar's checkbox and before the
// sidebar itself, so everything here happens before anything is drawn.
//
// This was three inline <script>s in theme/index.hbs. lupp.us serves
// the book under `Content-Security-Policy: script-src 'self'` (wolf-web
// nginx/lupp.us.conf), which refuses every inline script, so none of
// them ever ran there:
//   * `path_to_root` was never defined, mdBook's toc.js threw on the
//     first sidebar link, and every link stayed relative — right from a
//     page at the book's root, a 404 from every page in front/ or back/;
//   * the checkbox that draws the sidebar was never checked while the
//     template's static `sidebar-visible` class said it was open, so at
//     desktop width the state said "open" and the drawing said "closed";
//   * search never found its index.
// A same-origin file is `'self'`. Keep it that way: no inline script in
// the theme, ever (tests/contents holds every page to the site's CSP).
"use strict";

// mdBook's toc.js and searcher.js read a global `path_to_root`. This
// file is linked as `{{ path_to_root }}wolf-boot.js`, so its own src
// attribute carries the value; the 404 page's rewrite to `/book/…`
// (render.rs) reaches it the same way.
var path_to_root = (function () {
    var me = document.currentScript;
    var src = (me && me.getAttribute("src")) || "";
    var name = "wolf-boot.js";
    return src.slice(-name.length) === name ? src.slice(0, -name.length) : "";
})();

document.documentElement.classList.add("js");

// The sidebar's state, decided once, and written to the checkbox (which
// draws it, chrome.css) and the class (which names it) together, so the
// two cannot disagree. Wide screens open it unless the reader closed it
// last time; narrow screens start closed.
(function sidebarFirstPaint() {
    var toggle = document.getElementById("sidebar-toggle-anchor");
    if (!toggle) {
        return;
    }
    var open = false;
    if (document.documentElement.clientWidth >= 1080) {
        var stored = null;
        try {
            stored = localStorage.getItem("mdbook-sidebar");
        } catch (e) {
            /* storage unavailable: the default holds */
        }
        open = (stored || "visible") === "visible";
    }
    toggle.checked = open;
    document.documentElement.classList.toggle("sidebar-visible", open);
})();
