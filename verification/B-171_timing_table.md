# B-171: a composition inside another, before and after

Measured on 2026-09-30 by `b171_timing` (in `tests/b171_precomp.rs`, run with
`cargo test --release --test b171_precomp b171_timing -- --ignored`), three runs before the build
(the source of the checks-first commit e145371) and three after. No other cargo process was running
at the start of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the graphics card, with the viewer's
memory at what Automatic gives on this machine (16 GB) and its disk folder emptied first.

The reference shot, with a Gaussian Blur (24 px) on its first layer and another (16 px) on its
third, shown whole in an outer composition. Each pass draws frames 0 to 23 of the outer
composition through one viewer, as the window does; each figure is the median frame of a pass, in
milliseconds, and the median of the three runs. The passes, in order: the first play; the same
frames again; the same frames with the outer layer's opacity at 80%; and the same frames through a
new viewer with the same disk folder, which is the program closed and opened again.

| Quality | Pass | Before (ms) | After (ms) |
|---|---|---:|---:|
| Full | First play | 40.5 | **46.7** |
| Full | Played again | 16.1 | **2.0** |
| Full | Outer layer edited | 15.8 | **1.9** |
| Full | Program opened again | 31.8 | **13.5** |
| Draft | First play | 20.3 | **18.9** |
| Draft | Played again | 5.0 | **0.3** |
| Draft | Outer layer edited | 5.4 | **0.3** |
| Draft | Program opened again | 15.2 | **1.6** |

Before, only the drawings and their effects were kept, so every pass drew the inner composition
again. After, it is drawn once. The first play at Full is slower, by the time it takes to write
each frame to the disk folder (33 MB a frame); the three after runs were 46.7, 43.0, 55.6 ms against
45.3, 40.4, 40.5 before. At Draft a frame is a quarter the size and the writes did not show.
