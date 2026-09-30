# B-171b: composition frames written to disk by a worker, before and after

Measured on 2026-09-30 by `b171_timing` (in `tests/b171_precomp.rs`, run with
`cargo test --release --test b171_precomp b171_timing -- --ignored`), the same shot, passes and
method as `verification/B-171_timing_table.md`. Three runs of each of three builds, all on the same
day: the program before B-171 (the source of e145371, B-171's checks-first commit, with this
test), B-171 as built, writing each frame's copy inside the frame (6d8f042), and B-171b. No other
cargo process was running at the start of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the graphics card, with the viewer's
memory at what Automatic gives on this machine (16 GB) and its disk folder emptied first.

Each figure is the median frame of a pass, in milliseconds, and the median of the three runs.

| Quality | Pass | Before B-171 (ms) | B-171, writes in the frame (ms) | B-171b (ms) |
|---|---|---:|---:|---:|
| Full | First play | 34.8 | 55.1 | **27.5** |
| Full | Played again | 15.0 | 2.0 | **2.0** |
| Full | Outer layer edited | 15.1 | 2.0 | **1.9** |
| Full | Program opened again | 25.5 | 13.1 | **13.0** |
| Draft | First play | 18.1 | 19.5 | **17.0** |
| Draft | Played again | 4.5 | 0.4 | **0.3** |
| Draft | Outer layer edited | 4.4 | 0.4 | **0.3** |
| Draft | Program opened again | 13.9 | 2.0 | **1.6** |

The three first plays at Full: before B-171 30.2, 34.8, 35.0; B-171 54.7, 56.5, 55.1; B-171b
28.3, 27.5, 26.2. At Draft, B-171b 17.2, 17.0, 16.5.

**The first play is no longer slower.** B-171 wrote each 33 MB copy inside the frame; B-171b hands
it to one worker and goes on. It is now faster than before B-171 (27.5 against 34.8 measured the
same day, and against the 40.5 in B-171's own table), because the frames drawn after a frame is
kept find it in memory. Every other pass is as B-171 made it.

**The trade.** At most eight frames wait for the worker (265 MB at Full). A frame kept while eight
are waiting is kept in memory only, not on disk, rather than holding up the picture; with this
disk, the program opened again still read its frames back in 13.0 ms at Full, as B-171 did.

**Caveats.** Frame times vary with what the disk is still flushing from the run before. While
settling the worker (a queue of 4 that waited when full gave 36.4 ms, a queue of 64 gave 26.4) the
first run after each build was often the fastest of its three. The B-171b figures above are a
separate batch of three run after those trials, not in the same scripted sequence as the other two
columns.
