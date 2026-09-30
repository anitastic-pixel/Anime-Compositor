# B-170: a held masked drawing, before and after

Measured on 2026-09-30 by `b170_timing` (in `tests/b170_held.rs`, run with
`cargo test --release --test b170_held b170_timing -- --ignored`), three runs before the build
(the checks-first commit f1e02d3) and three after. No other cargo process was running at the start
of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the graphics card.

The reference shot with every layer held on twos and its second layer masked (a four-corner mask,
feather 4), with no effect, with a Gaussian Blur (6 px) and with an Exposure (+0.5). Each run draws
frames 0 to 23 through one viewer in turn; its figures are the medians of frames 4 to 23, a new
drawing on even frames and the same drawing held on odd ones, in milliseconds, planning and
drawing. Each figure below is the median of the three runs.

| Shot | Quality | Held frame, before | Held frame, after | New drawing, before | New drawing, after |
|---|---|---:|---:|---:|---:|
| mask | Full | 20.5 | **2.3** | 36.6 | 36.8 |
| mask | Draft | 18.4 | **0.7** | 33.2 | 33.9 |
| mask and blur | Full | 21.4 | **2.3** | 37.7 | 37.7 |
| mask and blur | Draft | 2.7 | **1.8** | 19.1 | 18.7 |
| mask and exposure | Full | 21.9 | **2.2** | 47.2 | 37.6 |
| mask and exposure | Draft | 3.0 | **1.7** | 22.4 | 18.9 |

A held frame no longer masks its drawing again or sends it to the card again. A frame with a new
drawing costs what it did: the mask has to be drawn once. (The "mask and exposure" new-drawing
figures before the build wandered from 40 to 54 ms across the three runs; the change does not touch
that path, and the after figures were steady.)
