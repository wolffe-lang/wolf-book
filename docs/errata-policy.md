# Errata policy

The reader-facing page is `book/back/errata.md`. This file is the
maintenance side of it: who acts on a report, on what clock, and what
each kind of fix costs.

## Taxonomy

Three kinds, matching the three issue templates.

**technical** — a sample or a claim is wrong. The fix goes through
`docs/review-checklist.md` in full: trace the claim, run the sample, and
re-capture any transcript the fix touches. A technical report that turns
out to be a toolchain defect is filed upstream in wolf-lang or
wolf-interp and linked from the book issue, and the book waits for the
fix rather than teaching around it.

**editorial** — typos, wording, a cross-reference pointing at the wrong
section, a term used two ways. Fast path. No pin, no re-run, one commit.

**drift** — a sample that no longer holds on a newer toolchain than the
pin. The nightly lane finds most of these first. Drift is fixed at the
next pin bump, in one commit with the bump, with every flip named in the
pin note. A drift report is never fixed by loosening a check.

## Clock

- editorial: next commit that touches the book.
- technical: triaged within a week; a wrong sample is either fixed or the
  page carries the correction under "Known corrections" until it is.
- drift: at the next pin bump. A red nightly is information and not an
  emergency, because the pin is what the book claims.

## Ownership

The repository maintainer triages. There is one, and the honest form of
that sentence is on the errata page: reports go to the issue tracker and
are read there. The book re-verifies itself on every commit and on every
toolchain release, so the failure mode this policy guards against is not
an unread report; it is a report nobody wrote down.

## Release rebuild

`.github/workflows/book.yml` accepts a `repository_dispatch` event of
type `toolchain-release`. A release in wolf-lang or wolf-interp fires it,
the book's whole suite runs against the pin, and the web edition
redeploys if it passes. The nightly lane does the same thing against the
compiler's main branch, report-only, so drift is visible before a release
makes it urgent. A dry run of the release path is a
`workflow_dispatch` on the same workflow.

## What is not errata

A missing chapter is not an erratum. Five chapters and three sections in
this edition are reserved, each saying on its own page what it covers and
why it is not written. A report that one of them is missing gets closed
with a pointer to the page, which already says so.
