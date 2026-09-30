// The Wolf Book — page behavior (bs00). Deliberately small: keyboard
// prev/next, sidebar state persistence, and the section rail (the
// Part → chapter → section level of DESIGN.md §3). No themes, no
// clipboard, no playground, no telemetry.
"use strict";

// The first-paint state is wolf-boot.js's (bs55); this keeps the class,
// the ARIA attributes and the links' tab order in step with the checkbox
// from then on. The ARIA half was an inline script the site's CSP never
// ran, so assistive tech was told nothing on lupp.us.
(function sidebarState() {
    const checkbox = document.getElementById("sidebar-toggle-anchor");
    if (!checkbox) {
        return;
    }
    function reflect() {
        const open = checkbox.checked;
        document.documentElement.classList.toggle("sidebar-visible", open);
        const toggle = document.getElementById("sidebar-toggle");
        if (toggle) {
            toggle.setAttribute("aria-expanded", String(open));
        }
        const sidebar = document.getElementById("sidebar");
        if (sidebar) {
            sidebar.setAttribute("aria-hidden", String(!open));
            sidebar.querySelectorAll("a").forEach(function (link) {
                link.setAttribute("tabIndex", open ? 0 : -1);
            });
        }
    }
    reflect();
    checkbox.addEventListener("change", function () {
        try {
            localStorage.setItem("mdbook-sidebar", checkbox.checked ? "visible" : "hidden");
        } catch (e) { /* storage unavailable: state simply does not persist */ }
        reflect();
    });
})();

(function keyboardNav() {
    document.addEventListener("keydown", function (e) {
        if (e.altKey || e.ctrlKey || e.metaKey || e.shiftKey) {
            return;
        }
        if (/^(?:input|textarea|select)$/i.test(document.activeElement.tagName)) {
            return;
        }
        if (e.key === "ArrowRight") {
            const next = document.querySelector(".nav-inline a.next");
            if (next) {
                window.location.href = next.href;
            }
        } else if (e.key === "ArrowLeft") {
            const prev = document.querySelector(".nav-inline a.previous");
            if (prev) {
                window.location.href = prev.href;
            }
        }
    });
})();

// The ToC rail shows Part → chapter from SUMMARY; this adds the third
// level — the active chapter's sections — from the page's own numbered
// headings, whose ids are the stable section-number anchors.
(function sectionRail() {
    function mount() {
        const active = document.querySelector(".sidebar a.active");
        if (!active || active.parentElement.querySelector(".rail-sections")) {
            return;
        }
        const heads = Array.from(document.querySelectorAll("main h2[id]"))
            .filter(function (h) { return /^\d+\.\d+$/.test(h.id); });
        if (heads.length === 0) {
            return;
        }
        const list = document.createElement("ol");
        list.className = "rail-sections";
        heads.forEach(function (h) {
            const li = document.createElement("li");
            const a = document.createElement("a");
            a.href = "#" + h.id;
            a.textContent = h.textContent;
            li.appendChild(a);
            list.appendChild(li);
        });
        active.parentElement.appendChild(list);
    }
    // The sidebar scrollbox is a custom element populated by toc.js;
    // mount after it has rendered.
    if (customElements && customElements.get("mdbook-sidebar-scrollbox")) {
        mount();
    }
    document.addEventListener("DOMContentLoaded", mount);
    window.addEventListener("load", mount);
})();
