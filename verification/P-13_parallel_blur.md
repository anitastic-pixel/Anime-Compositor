# P-13: the blur spread across the pool the renderer already owns

Document 15's P-13. This page is the timing half of the unit; the correctness half - that
spreading the blur moves no bit - is `verification/P-13_parallel_blur_table.md`, which is written
and checked on every build.

**A warm draft frame of the declared ten-layer fixture falls from 132.8 ms to 42.0 and 44.2 ms
across two runs, which is 3.19x the 41.667 ms a 24 fps clock allows down to 1.01x and 1.06x, and
a warm full-resolution frame from 153.6 ms to 62.5 and 59.8. The blur's own p95 falls from 116.4
to 153.1 ms across the fixture's six rows to 13.7 to 17.3 ms. No pixel moved, and the fixtures
are untouched.**

## Why this loop and not another

The owner asked on 2026-09-12 whether precision could be lowered - "say, the hundreths" - to buy
performance. The answer written back was that it could not, for three reasons recorded here
because they are what sent this unit at the blur instead:

- Rounding a number to two places makes no arithmetic faster. A multiply costs what it costs
  whatever the operand is, and quantising is an operation added rather than removed.
- The pixels are already at the lower precision. Every buffer in the renderer is f32; f64 lives
  in the geometry and the effect parameters, which is a handful of values per layer. The one
  place it runs per pixel is the tile loop, and that loop costs 4.2 to 4.3 ms of a declared-fixture
  frame whichever row of `verification/P-01_frame_trace.md` is read. The f64 arithmetic is a few
  operations of that against a bilinear gather that goes to memory for four pixels, so narrowing
  it would win a fraction of four milliseconds and would move the pixels while doing it.
- Hundredths is 1e-2. Document 25 line 57 allows simple arithmetic 1e-5 of error and a filter
  2e-5, and line 59 says not to loosen a tolerance to hide an error. Quantising a result to 1e-2
  fails the declared fixtures, and `Fixtures/` is read-only to implementation work, so that is a
  specification decision for the owner and not a step available here.

What the same measurement did say is that the blur was a plain `for` loop on one thread inside the
stage that dominates every declared-fixture row, while the tile loop beside it has been spread
across the rayon pool since B-08a. That costs no accuracy at all to fix, which is why it was
taken first.

## What changed

`convolve` in `src/effects.rs` hands out one destination row per task with `par_chunks_mut`
instead of walking the rows itself. A row reads the source and writes only itself, on either axis
of the separable pass, so the rows are independent and the borrow checker is what says so rather
than a comment. **The arithmetic is untouched**: a destination pixel is still the same taps
accumulated into the same `acc` in the same order, so the result is the same bits on one thread
or on twenty-four. Nothing else in the effect stack moved.

## The measurement

`tests/p01_frame_trace.rs`, `#[ignore]`d and run deliberately in release, on the same build with
and without the change. The before row is the current `main` at `29cb3f4`; the after rows are two
separate runs, both reported, because `verification/P-10_encode.md` established that one row of
this harness can move 19 ms between repeats and a single run is not a result.

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Date: 2026-09-12

### The declared ten-layer fixture

Frame time, p50 milliseconds, and the budget multiple in brackets.

| Row | Before | After, run 1 | After, run 2 |
|---|---|---|---|
| Draft, cache cold, files first read | 346.4 (8.31x) | 259.9 (6.24x) | 250.1 (6.00x) |
| Draft, cache cold, file cache warm | 344.1 (8.26x) | 249.0 (5.98x) | 250.0 (6.00x) |
| Draft, everything warm | 132.8 (3.19x) | 42.0 (1.01x) | 44.2 (1.06x) |
| Full, cache cold, files first read | 371.4 (8.91x) | 268.3 (6.44x) | 271.5 (6.52x) |
| Full, cache cold, file cache warm | 369.3 (8.86x) | 276.7 (6.64x) | 288.2 (6.92x) |
| Full, everything warm | 153.6 (3.69x) | 62.5 (1.50x) | 59.8 (1.44x) |

The blur's own stage, p95 milliseconds, which is the row to read rather than its p50: P-11's cache
answers more than half the frames of a pass without running the blur at all, so its p50 is 0.000
on the warm rows and the frames that do run it are the p95.

| Row | Before | After, run 1 | After, run 2 |
|---|---|---|---|
| Draft, cache cold, files first read | 116.4 | 15.1 | 15.5 |
| Draft, cache cold, file cache warm | 117.5 | 14.2 | 14.3 |
| Draft, everything warm | 141.0 | 15.1 | 17.3 |
| Full, cache cold, files first read | 124.6 | 13.7 | 14.2 |
| Full, cache cold, file cache warm | 118.4 | 14.5 | 16.7 |
| Full, everything warm | 153.1 | 17.0 | 17.1 |

Between seven and nine times, against twenty-four hardware threads. A 1920 by 1080 buffer of f32
RGBA is 33 MB in and 33 MB out per pass and there are two passes, so this is memory-bound past a
certain thread count in the same way `verification/P-10_encode.md` recorded for the encode, and
the factor is reported rather than a thread count claimed.

### The reference shot

Unmoved, and that is the check that the figures above are the blur and not the weather: the
reference shot has no effect instances at all, so no row of it may move. Frame p50, before against
the two after runs: draft cold 81.5 against 82.7 and 83.6, draft warm 2.5 against 2.3 and 2.5,
full warm 11.8 against 13.3 and 13.5. Every difference is inside the run-to-run spread of a stage
this unit does not touch.

## What did not move

**No pixel.** `verification/P-13_parallel_blur_table.md` blurs a cel-sized buffer at the declared
fixture's own sigma of 4.0 inside a pool of one thread and on the pool this machine would use, and
compares all 8,584,704 samples **by their bits** rather than against a tolerance. Zero differ. It
runs on every build, so the claim is re-checked rather than remembered.

`tests/b07_effects.rs` still checks the blur against weights generated a second way, unchanged.
`Fixtures/` is untouched. `verification/B-07_effects_table.md` is untouched.

## What this leaves open

D-47 stays open. A warm draft frame of the declared fixture is now at 1.01x and 1.06x of the 24
fps budget, which is the first time any row of this fixture has reached it, but a **cold** frame
is still 6.00x and the full-resolution warm frame is 1.44x. The largest stage left on the cold
rows is the decode, which is where P-03 and P-05 already are.

The precision trade the owner's question was really about is **not** taken here and remains
theirs to decide: cutting the blur kernel from the `ceil(3*sigma)` document 21 specifies to
something tighter, which would drop about a sixth of the taps at two and a half sigma. It changes
pixels by roughly 1e-3, fifty times the tolerance document 25 allows a filter, so it is an
amendment to document 21 rather than an optimisation. The other candidate that question suggested
- holding cached cels at half precision - is worth nothing now and the fresh table is why: with
P-04's parallel decode and P-11's effect cache in, `cache lookup and its copy` reads 0.0% to 0.1%
of every declared-fixture row.

What the fresh table does name next is two stages this unit did not touch. On the cold rows the
transfer function and the premultiply are 46.8% to 52.5% of a frame, which is P-03(c)'s territory
and where it was deliberately left. On the warm draft row the copy the effect stack writes into is
29.6%, second only to the blur that was just spread, and it is a copy rather than arithmetic.
