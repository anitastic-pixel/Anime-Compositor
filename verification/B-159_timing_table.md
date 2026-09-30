# B-159: an edit drawn again, before and after

Measured on 2026-09-30 by `b159_timing` (in `tests/b159_below.rs`, run with
`cargo test --release --test b159_below b159_timing -- --ignored`), three runs before the build
(the checks-first commit c916d8a, where nothing was kept) and three after. No other cargo process
was running at the start of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the processor.

Each run draws frame 10 through the viewer's cache, then sets one layer's opacity to 80% and 60% by
turns twenty times, as a slider being dragged does; its figure is the median of the twenty draws,
in milliseconds, planning and drawing. Each figure below is the median of the three runs. The
layer edited is counted from the bottom.

| Shot | Quality | Layer edited | Before | After |
|---|---|---|---:|---:|
| reference shot | Full | top (4 of 4) | 11.2 | **7.2** |
| reference shot | Full | middle (3 of 4) | 12.0 | **8.9** |
| reference shot | Full | second from bottom (2 of 4) | 11.8 | **10.0** |
| reference shot | Draft | top (4 of 4) | 2.0 | **1.0** |
| reference shot | Draft | middle (3 of 4) | 2.0 | **1.1** |
| reference shot | Draft | second from bottom (2 of 4) | 1.9 | **1.5** |
| declared ten-layer fixture | Full | top (10 of 10) | 24.5 | **7.6** |
| declared ten-layer fixture | Full | middle (6 of 10) | 23.8 | **14.9** |
| declared ten-layer fixture | Full | second from bottom (2 of 10) | 24.0 | **22.4** |
| declared ten-layer fixture | Draft | top (10 of 10) | 7.9 | **4.1** |
| declared ten-layer fixture | Draft | middle (6 of 10) | 8.9 | **5.0** |
| declared ten-layer fixture | Draft | second from bottom (2 of 10) | 8.4 | **6.4** |

The higher the layer edited, the more is kept below it and the bigger the saving. Editing the
second layer from the bottom saves least, since only the bottom layer is kept. Editing the bottom
layer keeps nothing and costs what it did. A frame drawn for the first time, or any other frame,
is drawn as before.
