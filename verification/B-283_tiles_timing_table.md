# B-283: frame times with Tiles

**PROVISIONAL.** Measured on 2026-10-09 on the code commit (fd577a37). Another lane's cargo
builds and tests were running when the card round started and again when the processor round
ended, so the machine was not quiet; the owner's app was not touched. "Noise alone" on the card
came out at 16.7 ms against 12.0 on the quiet B-273 run, which shows the load. To be measured
again on a quiet machine. The timing test is `b283_tiles_timing` in `tests/b283_tiles.rs`, built
with `cargo test --release --test b283_tiles` and run with `--ignored b283_tiles_timing`
(`B283_CPU` set for the processor).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-273: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 25.5 | 16.7 | 39.6 | 40.5 |
| Noise, then Motion Tile, tiles 25 per cent (its sized tile is drawn by the processor) | 221.2 | 259.6 | 124.8 | 124.3 |
| Noise, then Tiles, scale 25 | 61.8 | 65.6 | 122.9 | 125.0 |
| Noise, then Tiles, scale 7, blend 20 | 539.3 | 536.4 | 1704.8 | 1808.5 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Tiles, scale 25 (4 by 4 points a pixel) | about 16 ms (65.6 against 16.7) | over Target P2's 4 ms | about 28 ms (125.0 against 40.5) |
| Tiles, scale 7 (15 by 15 points a pixel) | about 173 ms (536.4 against 16.7) | over P2 | about 589 ms (1808.5 against 40.5) |
| Motion Tile, tiles 25 per cent, for comparison | about 81 ms (the layer goes to the processor and back) | not on the card | about 28 ms |

The cost follows the number of points each pixel averages, ceil(100 / scale) squared, so it grows
quickly below a scale of about 10. Motion Tile's rule fixes that count (D-304); an area average
would bound the cost but would change Motion Tile's pictures, so it is not done here.
