# B-283: frame times with Tiles

Measured again on 2026-10-09 on a quiet machine, on the Magnify code commit (5f297f6c, 56d0c4d5 once rebased onto main), which
leaves Tiles as its code commit (fd577a37, 66942d34 once rebased onto main) wrote it. No other cargo or rustc process was running:
checked before the card round and after each round, none; the owner's app was not touched. The
first measurement, PROVISIONAL because another lane was building, is replaced by this one ("Noise
alone" on the card was 16.7 ms then, 11.8 now, against 12.0 on the quiet B-273 run). The timing
test is `b283_tiles_timing` in `tests/b283_tiles.rs`, built with
`cargo test --release --test b283_tiles` and run with `--ignored b283_tiles_timing` (`B283_CPU`
set for the processor). One round each, so these are single figures, not medians of rounds.

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
| Noise alone | 16.1 | 11.8 | 40.4 | 40.9 |
| Noise, then Motion Tile, tiles 25 per cent (its sized tile is drawn by the processor) | 127.6 | 138.5 | 124.6 | 124.6 |
| Noise, then Tiles, scale 25 | 62.5 | 49.8 | 123.7 | 124.3 |
| Noise, then Tiles, scale 7, blend 20 | 520.5 | 517.6 | 1643.1 | 1662.3 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Tiles, scale 25 (4 by 4 points a pixel) | about 13 ms (49.8 against 11.8) | over Target P2's 4 ms | about 28 ms (124.3 against 40.9) |
| Tiles, scale 7 (15 by 15 points a pixel) | about 169 ms (517.6 against 11.8) | over P2 | about 541 ms (1662.3 against 40.9) |
| Motion Tile, tiles 25 per cent, for comparison | about 42 ms (the layer goes to the processor and back) | not on the card | about 28 ms |

The cost follows the number of points each pixel averages, ceil(100 / scale) squared, so it grows
quickly below a scale of about 10. Motion Tile's rule fixes that count (D-304); an area average
would bound the cost but would change Motion Tile's pictures, so it is not done here.
