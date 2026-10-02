# bs57 — results against the contract's §3

Prediction committed at `6981413` (contract §3) before any archive was
downloaded or any probe ran. Measured 2026-10-02 on kasumi.

## The pin (wolf 0.2.19 `9f3873d8…`, lupin 0.1.42 `9856335a…`, by digest)

| program | predicted | measured | verdict |
|---|---|---|---|
| `book/ch06/part-rowmatch` on wolf | `unsupported`, the `check.rs` text, `resolve`, exit 4 | exactly that, @374..562, default and `--checked` | **held** |
| same on lupin | runs, exit 0, the four lines | runs, exit 0, `no comma` / `no comma` / `` `lots` at byte 7 is not a number `` / `no comma` — a bare tag arm binds a success value first-arm-wins | **missed** (drift 4's second branch) |
| `book/ch06/s7` on wolf | `unsupported`, same text, not E0801 | exactly that, @374..459 | **held** |
| same on lupin | a no-arm error, nonzero | runs, exit 0, prints `no_comma` — `n => n` binds the raw tag | **missed** |
| `ch06/ex6-15` on wolf | `unsupported`, same text | exactly that, @144..233 | **held** |
| same on lupin | `7 -4 -99` | `7 -4 Weird` — `v => v` binds the unnamed tag, `_` never reached | **missed** |

The three lupin misses are one fact: lupin 0.1.42 runs the form with
no clause and sorts no arm by half — a bare identifier arm binds
whatever arrives, success value or raw tag. That is the value half
is67 owns, and it is why the console lane learned the pending line
(the contract's drift 4 named both branches; the second held).

## The gate at head (`999f5e00`, the pin)

| figure | predicted | measured |
|---|---|---|
| samples | 509 (250 roots + 16 members, 259 book blocks) | **509** (250 + 16, 259) |
| passed / pending / failed / flips | 503 / 6 / 0 / 0 | **503 / 6 / 0 / 0** |
| console blocks | 503 declared, 481 replayed (second branch) | **502 declared, 481 replayed** — 6-15's solution prints no transcript (7-5's precedent; lupin's real one is a wrong answer) |
| verify-docs | corpus 266; index 345, 9 pending; ch06 15 | **266; 345 (296 printed); 9 pending; ch06 15** — after the tense lint caught the pending apparatus on the page (`ae514c0`) |
| ledger | 133 open | **133** (126 filed, 7 waived, 0 unfiled) |
| backmatter | 296 / 331 | **296 / 331** |
| self-test | 15/15 | **14/14** — one plant (the console lane); the diagnostic path has unit tests instead |
| render | clean | web 48 pages, 1328 links none dead, CSP guard; md |

## The branch build (wolf-lang `s197` at `78753bcf`, `0.2.20+dev.78753bc`)

| program | predicted | measured |
|---|---|---|
| `part-rowmatch` | the four lines, native and checked | **the four lines**, `wolf run`, `--native`, `--checked` (sha `3b787e1f…`) |
| `s7` | `fail(E0801)` naming `no_comma` | **`fail(E0801)`: "this `match` does not cover `no_comma`"**, every lane; snapshot blessed from this run |
| `ex6-15` | `7 -4 -99`; `v` then `_` accepted silently or E0802 | **`7 -4 -99`**, no warning (`v` before `_` is the clause's own shape); the checked lane refuses `Weird` as "module items in checked execution", a checked-machine limit 6-6 shares |

With this build as `WOLF` and lupin 0.1.42 as `LUPIN`, the runner
reports **2 flips** (`part-rowmatch`, `s7`), keeps `ex6-15` pending
(lupin's half), and fails the four `--version` transcripts (a branch
stamp, as it must). One consequence for the lane that retires the
rows: `part-rowmatch`'s `run(exit=0)` flips on the compiler alone,
because lupin's wrong bytes exit 0 too; its console block is the byte
claim, and the runner compares that block the moment the row leaves.
