# B-554: frame times with Polygon Shards (D-675)

**Measured again on a quiet machine** on 2026-10-14 at 90a33e9a, built with `cargo test --release --test b554_polygon_shards --no-run` and run with `--ignored b554_polygon_shards_timing`: once on the card, then once with `B554_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 2% busy; 3.8 GB of its memory held by the desktop and open programs); after a run no other build or test was running, but the card was 11% busy with another program (the timing test had ended). The processor was 2% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-13, on a machine that was not quiet), again: Noise alone card 39.4, processor 58.3; Noise, then Polygon Shards as added (the Noise on the processor, as the shards begin the card's run) card 44.3, processor 133.1 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Polygon Shards, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.0 |
| Noise, then Polygon Shards as added (the Noise on the processor, as the shards begin the card's run) | 32.2 | 108.5 |
| Noise, then Polygon Shards, flat triangles on a grid of 8 pixels | 32.3 | 285.9 |
| Noise, then Polygon Shards, force keyed 0 to 600 | 32.5 | 83.0 |

First loops (empty caches): card 16.8, 87.3, 194.7, 70.1; processor 40.8, 110.0, 262.5, 81.6.

**Reading it.** On the card, once warm, Polygon Shards costs about 20 ms a frame over the Noise alone at every setting here (32.2, 32.3 and 32.5 against 12.2, three layers, about 7 ms a 1080p layer); the grid of 8 pixels costs no more there once warm, though its first loop from empty caches is much slower (194.7 against 87.3 as added). On the processor it costs about 68 ms as added, 42 with force keyed and 245 on the grid of 8 pixels (108.5, 83.0 and 285.9 against 41.0): there the cost grows with the number of shards. So the card is about two to twelve times quicker for what the effect adds, the most on the fine grid. The busy run could not say that; the quiet run can.
