# P-05: what the skip is worth

Written by `p05_culling_cost` in `tests/p05_culling.rs`, `#[ignore]`d and run deliberately in release. Forty frames spread across each fixture's work area, the tile loop only - no decode, no effect stack, no encode - each rendered both ways, in an order that alternates frame by frame.

The correctness half is `verification/P-05_culling_table.md`, which runs on every build.

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Date: 2026-09-09

| Fixture | Quality | Without culling (ms) | With culling (ms) | Change |
|---|---|---|---|---|
| reference shot | Draft | 1.693 | 1.751 | +3.4% |
| reference shot | Full | 8.016 | 7.983 | -0.4% |
| declared fixture | Draft | 4.163 | 4.145 | -0.4% |
| declared fixture | Full | 20.431 | 20.575 | +0.7% |

Document 15's entry said so in advance - "it is now expected to report a saving too small to matter" - and the measured answer on these two fixtures is that there is no saving at all: the box excludes none of the 7,020 layer-and-tile pairs `verification/P-05_culling_table.md` counts, because every cel here is a full-frame drawing with transparent margins and a geometric box cannot see transparency.

What the table does say is that the test costs nothing to carry: under 3% of the tile loop, which P-01 measured at 1.3% to 7.8% of a frame, so under a fifth of one percent of a frame at the worst row above. It is kept on that basis and not on a saving. The skip itself works - the same page excludes 120 of 135 pairs for a quarter-size layer in a corner - so it will pay on layers smaller than the frame, and a box drawn from alpha rather than geometry is the entry that would pay here.

The two renders alternate frame by frame. They did not at first, and with the culled render always going first on a fresh plan the table read +24% at draft; that was the cold cache of whichever ran first, not the box.
