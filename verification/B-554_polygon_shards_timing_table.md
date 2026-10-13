# B-554: frame times with Polygon Shards (D-675)

Measured on 2026-10-13, built with `cargo test --release --test b554_polygon_shards` from lane C's
commit 52ffce8b, which holds 52ffce8b (B-554's code), and run with `--ignored b554_polygon_shards_timing`: once on the card, then once
with `B554_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, these build or test programs were running: anime_compositor_app-7f89, b532_warp_stabilizer-b6a2, cargo.exe. Between the card and processor runs, these build or test programs were running: anime_compositor_app-7f89, b532_warp_stabilizer-b6a2, cargo.exe, rustc.exe. After the processor run, these build or test programs were running: anime_compositor_app-7f89, b532_warp_stabilizer-b6a2, cargo.exe, rustc.exe. Treat them as an upper bound, to be measured again on a quiet machine.

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
| Noise alone | 39.4 | 58.3 |
| Noise, then Polygon Shards as added (the Noise on the processor, as the shards begin the card's run) | 44.3 | 133.1 |
| Noise, then Polygon Shards, flat triangles on a grid of 8 pixels | 44.7 | 336.5 |
| Noise, then Polygon Shards, force keyed 0 to 600 | 50.0 | 111.2 |

First loops (empty caches): card 116.1, 162.4, 322.3, 83.8; processor 56.8, 211.3, 359.7, 100.2.

**Reading it.** On the card, Polygon Shards costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(39.4 to 44.3, three layers); the cost grows with the number of shards (the layer's size over the grid spacing, squared) and how far they reach. Here a busy machine cannot say whether the card is quicker.
