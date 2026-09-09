# P-11: what the effect stack costs when it is not run twice

Document 15's P-11. This page is the timing half of the unit; the correctness half — that the
cache changes no pixel and loses no warning — is `verification/P-11_effect_cache_table.md`, which
is checked on every build.

**A first playthrough of the declared ten-layer fixture falls from 172.498 ms a frame to 46.729,
49.478 and 50.817 ms across three runs — 4.14x the 24 fps budget down to 1.12x to 1.22x — and at
full resolution from 206.926 ms to 71.660, 72.037 and 68.958, which is 4.97x down to 1.65x to
1.73x. The 960-frame manifest is identical to the one P-10 left, byte for byte.**

## What the timer was asked first

Document 15's entry opens with a condition, not a plan: "The first thing this unit does is split
the effect stack in `src/perf.rs` by instance and re-run P-01's harness, because P-09's claim that
the blur is the expensive one is an argument from what the three effects do and not a measurement;
if the timer disagrees, the unit buys whatever the timer names instead."

`src/perf.rs` now has five stages where it had one — the copy the stack writes into, the exposure,
the tint, the blur, and the cache this unit adds — and the split run says this, on the declared
fixture's six P-01 rows:

| Stage | p50 across the declared fixture's rows |
|---|---|
| effect: gaussian blur | 111.749 to 123.073 ms |
| effect stack: the copy it writes into | 10.354 to 13.437 ms, on the two warm rows only |
| effect: exposure | 0.970 to 1.183 ms |
| effect: tint | 0.846 to 0.892 ms |

The timer agreed with P-09. **The blur is 28.4% to 80.3% of a frame on its own, and the exposure
and the tint together are under two milliseconds**, so the cache is built for the blur, which is
what the entry directs: "Per instance, and the blur first."

The reference shot's six rows read 0.000 on all four, because no layer of it has an effect. Those
rows are this unit's control and they are used as one below.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Harness: `tests/p01_frame_trace.rs`, the `p03b_first_playthrough` measurement — twenty frames
  spread across each fixture's 240-frame work area, every one a cache miss on every layer, with
  the viewer's own budget
- Before: the build P-10 left, `verification/P-03b_first_playthrough.md` as re-run on it
- After: three runs of the same harness on this build, reported separately rather than averaged

## What it did to a first playthrough

| Workload | Quality | Before (ms) | After, three runs (ms) | Against the 24 fps budget |
|---|---|---|---|---|
| the declared ten-layer fixture | Draft | 172.498 | **46.729, 49.478, 50.817** | 4.14x → 1.12x, 1.19x, 1.22x |
| the declared ten-layer fixture | Full | 206.926 | **71.660, 72.037, 68.958** | 4.97x → 1.72x, 1.73x, 1.65x |
| the reference shot | Draft | 22.960 | 23.953, 24.477, 23.891 | 0.55x → 0.57x, 0.59x, 0.57x |
| the reference shot | Full | 32.988 | 35.242, 34.725, 34.856 | 0.79x → 0.85x, 0.83x, 0.84x |

**The reference shot is the control, and it is worth reading before the result.** No layer of it
has an effect, so no part of this change can touch it — and it reads 3% to 6% *slower* after. That
is this machine's run-to-run spread on the day, measured on a path the code could not have moved,
and P-01's twelve rows re-run on the same build agree: every one of them, including the six with
no effects at all, came back between 0.8% and 15.3% slower than the same build measured earlier.
So the honest reading of the table is that the noise this session runs a few percent *against* the
result, and the declared fixture's 73% is what remains after that.

Where the time went, on the declared fixture at Draft:

| Stage | Before (p50 ms) | After (p50 ms) | After (p95 ms) |
|---|---|---|---|
| effect stack (one stage before this unit) | 119.574 | — | — |
| effect: gaussian blur | — | **0.000** | 111.269 |
| effect: exposure | — | 1.301 | 1.875 |
| effect: tint | — | 0.000 | 1.046 |
| effect stack: the copy it writes into | — | 3.953 | 11.381 |
| effect result cache: lookup and admit | — | 1.061 | 3.070 |
| decode this frame's cels in parallel | 29.428 | 32.472 | 41.023 |
| tile loop: sample and blend | 4.130 | 4.242 | 4.951 |

**The blur's median is zero and its 95th percentile is the whole 111 ms.** That is the shape of a
cache working: more than half the frames of a playthrough do not run the blur at all, and the
frames that do run it pay what they always paid. The p95 frame time barely moves — 190.032 ms
before, 177.923 after — because the slowest frame in a pass is by definition one that missed.

The cache's own stage, the lookup and the admission together, is 1.061 ms p50 and 3.070 ms p95.
That is what holding the key by value rather than as a hash costs, and it is 2% of what it saves.

## Why the pass is not faster still

`Effect results over the pass: 18 hits, 38 evaluations, 24 dropped to stay inside the budget.`

A full-resolution result is 33 MB before a blur's margin is added, so 448 MiB is room for about a
dozen. The pass wants 38 distinct ones and gets 18 of them back. **The number of evictions is
larger than the number of hits, and that looks like the budget being spent badly** — so it was
measured rather than assumed.

The rule tested was to admit only stacks containing an effect that expands its bounds, which is
document 21's own declaration and is exactly the class of effect that reads a neighbourhood and
therefore costs a multiple of its image. On this fixture that means the blur and nothing else. It
gave a clean cache — **9 hits, 9 evaluations, nothing dropped at all** — and it made the
playthrough *slower*:

| Workload | Quality | Admitting everything | Admitting only the blur |
|---|---|---|---|
| the declared ten-layer fixture | Draft | 46.729, 49.478, 50.817 | **55.406, 54.830** |
| the declared ten-layer fixture | Full | 71.660, 72.037, 68.958 | 72.396, 71.404 |

**It is not in this build.** Two things the rule missed. The blur's own hit rate hardly changed —
nine hits in twenty frames against a median of zero either way — so the evictions were not costing
the thing they appeared to be costing; least-recently-used was already keeping the entries that
were about to be asked for. And an exposure or a tint hit does not only skip 1.2 ms of arithmetic:
it skips the copy the stack writes into, which is 3.953 ms p50 and 11.381 ms p95 on this row.
Refusing to cache the cheap effects gave that copy back on every frame of two layers.

So the eviction count is reported here as a fact about the budget and **not** as a fault to fix.
The question of a larger effect budget is a different question, and it is D-40's to answer, not
this unit's: the gibibyte is split here, never enlarged.

## The proof that no pixel moved

`tests/p03_byte_equality.rs`, run on this build: 960 frames across both fixtures at both
qualities, each rendered with no cache and again with the viewer's own budget, every pair equal,
and the manifest of all 960 lines **identical to the one P-10 left**, over 4,230,144,000 bytes.
This is the first run of that test in which the budgeted half of every pair was rendered through
the effect cache, because `CelCache::viewer()` is what that half now builds.

`verification/P-11_effect_cache_table.md` is the rest of it, and it runs in the normal suite
rather than on request: the key notices a blur of 4.5 pixels where it stored a blur of 4.0, it
notices a corner of the mask moving, it notices a mask being removed, and an effect this build
cannot draw raises its warning on the frame served from the cache exactly as on the frame that
filled it. That last one is document 27's sentence made a test — a cache "must never define
correctness" — and a diagnostic that appeared only on a miss would be a cache defining one.

## What this does not touch

**Export.** ADR-015's third bound is that an export neither reads the cache nor writes it. An
export builds `CelCache::none`, which has no effect budget, so `effect_result` returns before it
builds a key and `store_effect` returns before it takes a byte. Every figure on this page is a
preview.

**The gibibyte.** `DEFAULT_EFFECT_BUDGET_BYTES` is taken out of `DEFAULT_BUDGET_BYTES`, not added
to it. The window holds 1 GiB, as document 40 said it does; 448 MiB of it is now effect results
and the remaining 576 MiB is cels.

**The cel cache's own numbers.** They are unchanged across every run above — 86 hits, 148 misses —
because the cel stream is unchanged. Only the second list is new, and it is a second list for the
reason `verification/P-09_effect_reuse.md` measured: one least-recently-used list holding both
would let the cel stream, which cycles through far more distinct drawings than any budget holds,
drop the effect results just before they are asked for.

**The five artifacts that record a run of an earlier build.** `verification/P-01_frame_trace.md`,
`verification/P-03b_first_playthrough.md`, `verification/B-08b_cache_budget.md`,
`verification/T-06_declared_fixture.md` and `verification/T-06_performance_envelope.md` are dated
records and are not rewritten here. Two of them — the T-06 pair — measure the viewer with
`CelCache::with_budget(DEFAULT_BUDGET_BYTES)`, which is now the cel-only cache rather than the
viewer's, so the next deliberate run of the acceptance envelope should be the one that decides
whether they move to `CelCache::viewer()`. That is an acceptance decision, not a performance one.

## What is still open

The declared fixture's first playthrough is 1.12x to 1.22x of the 24 fps budget at Draft. It was
4.14x. It is not yet inside it, and the largest stage left is the decode at 32.472 ms p50 — which
is P-04's and P-05's territory, not this unit's.
