# B-283: frame times with Tiles

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b283_tiles --no-run` and run with `--ignored b283_tiles_timing` (`tests/b283_tiles.rs`): once on the card, then once with `B283_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program, the card was idle (0 to 4% busy; about 4.1 GB of its memory held by the desktop and open programs) and the processor under 2% busy; the same after each run. Before the card run another program kept the card 13 to 29% busy, and the run waited about 4 minutes for it to settle. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier figures, again (2026-10-09, also quiet, at 5f297f6c): Noise alone card 11.8, processor 40.9; Motion Tile 25 card 138.5, processor 124.6; Tiles scale 25 card 49.8, processor 124.3; Tiles scale 7 card 517.6, processor 1662.3 ms. The first, provisional run (another lane building) had Noise alone on the card at 16.7.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** As B-273: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 19.7 | 13.0 | 43.0 | 45.9 |
| Noise, then Motion Tile, tiles 25 per cent (its sized tile is drawn by the processor) | 126.9 | 126.6 | 125.6 | 126.8 |
| Noise, then Tiles, scale 25 | 54.0 | 50.5 | 127.0 | 126.3 |
| Noise, then Tiles, scale 7, blend 20 | 521.1 | 519.5 | 1654.7 | 1670.0 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Tiles, scale 25 (4 by 4 points a pixel) | about 13 ms (50.5 against 13.0) | over Target P2's 4 ms | about 27 ms (126.3 against 45.9) |
| Tiles, scale 7 (15 by 15 points a pixel) | about 169 ms (519.5 against 13.0) | over P2 | about 541 ms (1670.0 against 45.9) |
| Motion Tile, tiles 25 per cent, for comparison | about 38 ms (the layer goes to the processor and back) | not on the card | about 27 ms |

The quiet re-run of 2026-10-13 matches the quiet run of 2026-10-09 to within a few per cent on
every row, so the reading below stands.

The cost follows the number of points each pixel averages, ceil(100 / scale) squared, so it grows
quickly below a scale of about 10. Motion Tile's rule fixes that count (D-304); an area average
would bound the cost but would change Motion Tile's pictures, so it is not done here.
