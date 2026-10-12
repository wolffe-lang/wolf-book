# bs65 — the prediction, committed before either new archive was downloaded

Wave 53. Written 2026-10-12 against wolf-book `f8e73fa` (the contract, an
empty commit on bs64's merged trunk `2504a0f`), **before**
`wolf-0.2.28-*.tar.gz` or `lupin-0.1.51-*.tar.gz` was downloaded on any
host. DERIVED means computed from the two upstream git histories at the
tags (private blobless clones under `~/lanes/bs65/`); PREDICTED is a claim
about what the release binaries will do, with what falsifies it.

What ran before this file, and is allowed to: the **control** pair, the
pin the book carries (wolf 0.2.26 / lupin 0.1.49), fetched by digest from
the release API on three hosts (`bs65-evidence/setup.sh`): linux x86-64
`05acdc5e…` / `84911a35…` on kasumi and hasu, macOS arm64 `8ea7ef3b…` /
`ad188d58…` on nomad-1. On the unedited head `f8e73fa`, kasumi: **515
samples, 512 passed, 3 pending, 0 failed, 0 flips; 488 of 508 console
blocks**; ledger **126 open (119 filed, 7 waived), 109 closed**;
verify-docs ok (corpus 266); diagrams 6 from 4 traces, checked. And the
**580-program sweep** (`allprogs.py`: every book fence outside `back/`,
`part()` fences assembled, every corpus `.lu`, each alone) on nomad-1
from the macOS archives (lupin, `conform-run --json`, `--checked`, and a
`--release` build and run for every native `pass` with a `main`); the
kasumi and hasu sweeps were still running when this was written.

## 1. The pin (DERIVED)

```
[wolf]         rev 89dc139443da38078df6093568da87bc6d0ee6f9 -> f71ece8e1ad2234940040d300359888932ad67af
               impl_version 0.2.26 -> 0.2.28
[lupin]        rev f516a5f4ea4341acd3a30f3e5cdd4327aede1912 -> f72e78d7a9d0ceb39d80380389ad6a2072991c63
[lupin-trace]  rev f516a5f4ea4341acd3a30f3e5cdd4327aede1912 -> f72e78d7a9d0ceb39d80380389ad6a2072991c63
```

Two compiler releases (0.2.27, release 409367590, and 0.2.28,
release 410118732; 328 commits `v0.2.26..v0.2.28`) and two interpreter
releases (0.1.50, 0.1.51, release 409991647; 51 commits). The trace pin
stays collapsed onto `[lupin].rev`: `src/eval/placetrace.rs` is
byte-identical between v0.1.49 and v0.1.51.

## 2. The stamps (PREDICTED, to the character)

```
$ wolf --version
wolf 0.2.28 (wolfgang, pin f71ece8)
paired with lupin 0.1.51 (reference interpreter), pin 294d626
$ lupin --version
lupin 0.1.51 (wolf-interp, reference interpreter at pin 294d626)
```

From `PAIRING` at `f71ece8e` (`lupin-version = 0.1.51`, `lupin-pin =
294d626`), `vendor/upstream/PIN` at `v0.1.51` (`294d626d…`) and D57.
**The interpreter's clause is now FOUR releases behind**: `294d626` is
v0.2.24's tag, an ancestor of `f71ece8e`, **541** commits and four
releases (v0.2.25 … v0.2.28). ch01 §1.2's "from two releases back … you
get 213" and the colophon's matching sentence are rewritten to four and
541. Falsified by any character of the stamps that differs.

## 3. Spec artifacts (DERIVED both ways)

- **Anchors 607 → 634**, none dropped. Added (27):
  `abi.target.none.ambient`, `exec.checked.race`, `exec.checked.sched`,
  `exec.checked.task`, `os.env.unset`, `os.fs.change.note`,
  `os.fs.create.mode`, `os.fs.link`, `os.fs.mode`, `os.fs.readdir`,
  `os.fs.readlink`, `os.fs.stat`, `os.fs.symlink`, `os.fs.times`,
  `os.fs.truncate`, `os.fs.umask`, `os.json`, `os.json.dup`,
  `os.proc.exec`, `os.proc.job`, `os.proc.status`, `os.signal.disp`,
  `os.signal.poll`, `os.term`, `os.term.mode`, `os.term.note`,
  `type.wrap.div`. Appendix D's tags hold.
- **`grammar.ebnf` does not move** (byte-equal to the vendored copy at
  v0.2.28). Appendix A does not regenerate.
- **Catalogue 187 → 187** (no `## E…`/`## W…` heading added). Appendix
  C's count holds.
- Registry text moves for **E0301** (the archive's std), **E1010** (kept
  and sent arguments) and **E1504** (code that reaches: a host builtin, a
  std module's own reach, a C declaration — wolf-lang#639's complaint
  answered). The book prints `wolf --explain E1504` (ch24 §24.3); it
  prints neither E0301's nor E1010's.
- No existing `wolf_mem` E1010 snapshot moves between the tags (17 new
  ones only), so the book's three E1010 transcripts (ch08 s3, s4,
  ex8-3) **hold**.

## 4. Members and floors (PREDICTED)

- wolf archive: 0.2.26's eleven members **plus `std/`** (s204, ruling
  #59): 53 `.lu` files, `STD-REV` = `98e1b7bc…`, the two std licences
  and a README. Byte-identical to 0.2.26's: `LICENSE`, `LICENSE-EXCEPTION`
  (`3972dc97…`, `a0eec20d…`). `_wolf`, `wolf.bash`, `wolf.fish` are
  predicted to MOVE only if a subcommand or flag was added (none is in
  `main.rs`'s diff: no new subcommand), so **byte-identical** too. Move:
  `wolf`, `wolf.1` (the std paragraph and version), `README.md`,
  `wolf-cimport-worker`, `libwolf_rt.a` (s228, s230), `libwolf_rt_none.a`
  (s223's two symbols).
- lupin archive: `lupin`, `CHANGELOG.md`, `LICENSE`, `README.md`.
- Both linux x86-64 binaries: no `GLIBC_` symbol above **2.34**.

## 5. The subject run: the unedited tree, the new pair (PREDICTED)

**0 flips. 515 samples: 512 passed, 3 pending, 6 failed; 482 of 508
console blocks.** No snapshot and no `diagnostic,from` block moves. The
six failures are console blocks, every one a drift of text, no verdict:

1. `book/ch01.md` `--version` (§2's stamps).
2. `book/back/colophon.md` `--version`.
3. `book/ch22.md` `wolf interface`: `toolchain 0.2.26` → `0.2.28`, both
   export hashes hold (wolf-lang's own `interface_pretty` snapshot moves
   only its toolchain line between the tags).
4. `principles/exercises/ch22/EXERCISES.md`, 22-13's interface: the same.
5. `book/ch24.md` `wolf --explain E1504`: the registry's new paragraph
   ("code in the owning package that reaches a capability … an import …
   `import c` … a C declaration … a host builtin … a std module whose own
   code reaches one ([pkg.caps])").
6. `book/back/appendix-e.md` `wolf --help`: the closing std paragraph
   ("The standard library, wolf-std, ships in the release archive as
   `std/` beside the `wolf` binary, and that copy is the default …").
   The subcommand list does not move (twenty-three).

The same six, and only those, on a **bare** `wolf` (the binary and its
runtime libraries copied out of the archive with no `std/` beside them,
which is what the book's CI builds from source): the archive's std
reaches no book sample, because **no book program writes `use std.…`**
(grep over the 580: 0) and the three pending rows fail for reasons that
are not the std root (ex5-8's compiler half is `member access on a value
whose type is still being inferred`, its lupin half the std root lupin
still lacks — lupin is unpacked from its own archive, not beside
`wolf`). Pending **3 → 3** (ex5-8, ex7-5, ex8-7) in the same words.

What does **not** fail, by category, and why:

- **E1010's four new shapes** (s227 #660, s229 #665/#678's direct half,
  s228's `net_read`): **no book program is newly refused**. The
  compiler lanes' censuses over the book (s227: `2504a0f`, s229, s228's
  357 samples and exercises) moved 0 rows; my own scan: 11 programs use
  `in r { … }`, none hands a region-built `str` to a call beside a `mut`
  argument; no program hands a `channel[str]` to a helper inside a
  region block; ch33's seven `net_read` programs read into the root, none
  returns a read out of a region block. Falsified by any native or
  checked `fail(E1010)` outside the five control rows (ch08 s3 at
  `ch08.md:146`, s4 at `ch08.md:246`, ex8-3, and ch16's two fences at
  `ch16.md:13` and `:27`, extracted alone).
- **The new prelude names** (17 at 0.2.27, 8 at 0.2.28): no book program
  declares any (both CHANGELOGs' greps over `2504a0f`), so no W0304.
- **s220** (u64 printing, `-x` on unsigned): no book program prints an
  unsigned value above `i64::MAX` or negates an unsigned (the 10 that
  name `uint`/`wrapping` use `wrapping[i32]` and `as uint` sizes).
- **s224** (json last-wins), **s225** (exec, env unset), **s218**
  (stat), **s219** (signals, terminal), **s230** (modes, links, umask),
  **#688** (`0o`/`0b` literals): no book program reaches them (grep:
  no `json_`, no `0o`/`0b` literal, no new builtin).
- **s221** (`ffi` at a C declaration): ch32's three `import c` programs
  are single files, no manifest; `samples/pkg` declares no C. No E1504.
- **s228's `List` slice** charges its own bytes: no book `cap:` budget
  sits within the doubling slack of a slice (the five `cap:` programs
  measure regions of their own allocations); 0 rows.

## 6. The sweep: subject against control (PREDICTED)

- **lupin: 0 rows** outside the control's own noise (0.1.50 and 0.1.51
  mirror builtins no book program calls; json last-wins meets none).
- **native: 0 rows** (verdict, stdout, codes).
- **release: noise only** (bs64's set: B-11 and ex9-4's exit after UB,
  ch17's balances, ex11-5's worker order, ex30-4/ex30-5's arrival order,
  ch10's first scope program).
- **checked: exactly the 83 rows of `bs65-evidence/predicted-checked.txt`**
  (77 «structured concurrency in checked execution (C1 deferred)», 6
  «closures in checked execution» at 0.2.26) move from `unsupported` to
  a verdict — **lupin's verdict class, and on `exit(0)` lupin's stdout
  byte for byte**. The checked machine takes lupin's scheduler decisions
  (`[exec.checked.sched]`, ruling #63), so an unseeded run on both is the
  same schedule. Exceptions allowed by name only: a row that reaches a
  construct the checked machine still declines (a nested `fn`), or one
  of the four measured partings (wolf-interp#227 rendezvous back-edge,
  #228 UB inside a proc, #229 a task's module state, #230 a `when`
  through a closure). Falsified by any of the 83 that answers another
  verdict or other bytes outside those, and by any checked row outside
  the 83 that moves.
- **ex16-7 and ex16-8** are two of the 83: checked `exit(0)` with
  `distance=18` and `distance=10` (lupin's and native's bytes at both
  pins; ex16-8 is 16-7 with carve seed 2).

## 7. Sentences the release makes false (found by reading, before any archive)

1. **ch01 §1.2** and **the colophon**: "from two releases back … 213" →
   four releases, 541.
2. **ch01 §1.2** "The compiler's folder holds the `wolf` binary, the
   C-import worker, and the runtime library": since 0.2.27 it also holds
   the standard library, `std/`, the default std root. One clause.
3. **ch24 §24.3**: "'Reach' means the code, not only the imports, and
   the explain text above tells only the import half" — the explain now
   tells all of it (item 5 above). The sentence is rewritten to read the
   regenerated block; the paragraph's measured facts hold.
4. **principles/exercises/ch11/EXERCISES.md** header: "the checked
   machine (`wolf conform-run --checked`) reports `unsupported` for the
   concurrency surface" — false at 0.2.28 (s226). Rewritten to the
   measured answer for that chapter's programs.
5. **ch25**'s reader-facing paragraph: "a standard library on the page to
   depend on: the toolchain this edition names carries neither in a
   form a reader can run … and `use std.io` resolves to nothing" — the
   toolchain's archive now carries wolf-std. PREDICTED (to be measured on
   the archive whole): `use std.io` from the 0.2.28 archive RESOLVES on
   the compiler and lupin (unpacked alone) still answers `unsupported`.
   The sentence is corrected to what is measured; §25.2 stays held (the
   section's subject, the stdlib *posture* — core, penumbra, `std.x` —
   is unchanged by a bundled copy) and its HOLD note gains the
   re-measure.
6. The version-literal ledger (`audit/version-literals.txt`): every
   `audited-at` stamp re-read at 0.2.28 / 0.1.51 (`verify-docs` forces it).
7. The dated ledger rows in ch11, ch17, ch30, ch32 and ch33 that quote
   the C1 refusal are re-verifies at named pins and stay true as history;
   none is reworded.

## 8. Back matter and counts at head (PREDICTED)

- No snapshot re-blessed. Six console blocks spliced from the run's
  `actual:`; `solutions.md` regenerated by `backmatter` (22-13's
  toolchain line).
- `ledger --check`: **126 open, 109 closed**, unchanged (no ledger row
  names #639 or the checked machine as open debt).
- `verify-docs` ok after the version-literal re-read; tiers unchanged;
  Appendix C 187.
- `backmatter --check` against wolf-lang at v0.2.28: `anchors.json`
  re-vendored (634); `diagnostic-codes.txt` and `grammar.ebnf` unchanged.
- `diagrams --check` 6 from 4 with the release lupin 0.1.51 as
  `LUPIN_TRACE`: byte-identical.
- Head: **515 / 512 / 3 / 0 / 0, 488 of 508**, on the archive whole and
  on the bare `wolf`, kasumi and hasu; CI green on three hosts.

## 9. Method

Control (above) and subject on the same unedited tree, the toolchain the
one variable, on kasumi and hasu (linux x86-64, gates and sweep) and
nomad-1 (macOS arm64, sweep only — no cargo build there); the sweep on
each pair twice for the noise floor. Then the edits (console blocks
spliced from the run by `splice.py`, never typed), then the head gate
(`samples`, also under `taskset -c 0-3` and on the bare `wolf`,
`--self-test`, `verify-docs`, `ledger --check`, `backmatter --check`,
`contrast`, `diagrams --check` with `LUPIN_TRACE`, `render web`, `tiers
--check`, xtask's tests with `--nocapture`), every log full `2>&1` with
SKIPs counted. A plant (ch01's stamp put back to 0.2.26) must go red on
CI's three hosts and is reverted.
