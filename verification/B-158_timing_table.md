# B-158: the part of the picture on screen, before and after

Measured on 2026-09-30 by `b158_part_timing` (in `app/src/main.rs`, run with
`cargo test --release --manifest-path app/Cargo.toml b158_part_timing -- --ignored`), three runs
before the build (the checks-first commit ffa59f6, where the window took the part and ignored it)
and three after. No other cargo process was running at the start of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release.

Each figure is the median of three runs; each run's figure is the median of frames 1 to 48, each
made (not from memory) by a viewer that remembers nothing, after frame 0 was made once to read the
drawings. It is the window's whole work for a frame: planning, drawing, the eight-bit encode, and
cutting out the part. "Middle quarter" is the part a 200% zoom shows of a picture fitted to it
(fractions 0.25 to 0.75 across and down), with its spare pixel and no margin.

| Shot | Drawn on | Quality | Whole frame, before | Whole frame, after | Middle quarter, before | Middle quarter, after |
|---|---|---|---:|---:|---:|---:|
| reference shot | processor | Full | 20.9 | 21.8 | 21.6 | **11.3** |
| reference shot | processor | Draft | 8.8 | 9.0 | 9.5 | **7.3** |
| declared ten-layer fixture | processor | Full | 40.1 | 41.0 | 40.8 | **21.4** |
| declared ten-layer fixture | processor | Draft | 20.3 | 19.9 | 19.5 | **17.0** |
| reference shot | card | Full | 13.9 | 12.8 | 9.7 | 10.2 |
| reference shot | card | Draft | 7.2 | 7.2 | 7.6 | 7.5 |
| declared ten-layer fixture | card | Full | 27.9 | 26.4 | 20.3 | 18.6 |
| declared ten-layer fixture | card | Draft | 17.3 | 17.8 | 20.2 | 21.6 |

**What it says.** On the processor, the middle quarter now takes about half the time of the
whole frame at Full (21.6 to 11.3 ms on the reference shot, 40.8 to 21.4 on the ten-layer fixture),
and about a fifth less at Draft, where the picture is small and planning is a larger share. The
whole frame is unchanged (the differences are run-to-run spread).

**The card** is not faster: it still draws the whole frame, and only the pixels sent are cut. Its
"middle quarter" column differs from its "whole frame" column the same way before the build as
after, when both did exactly the same work, so that difference is the order the two were measured
in (the part is measured second, on a card already warm), not a saving.

Why not a quarter of the time: a part is still planned whole, and effects run on the whole layer
before the picture is laid (ADR-017), so only the laying and the encode shrink.

The three runs of each: before, `b158_before_1..3`; after, `b158_after_1..3` (raw files kept outside
the repository, as `verification/B-158_timing_raw.md` is one run and is not committed).
