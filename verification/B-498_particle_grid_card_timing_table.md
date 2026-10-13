# B-498: frame times with Particle Grid on the card (D-618) — PROVISIONAL

Measured on 2026-10-13, built with `cargo test --release --test b496_particle_grid` from
79923304 (B-498's code), and run with `--ignored b496_particle_grid_timing`.
**The machine was not quiet.** Other lanes' tests (`b525_curl_transform`, `b532_warp_stabilizer`,
`b533_rolling_shutter_repair`, and a `b155_gpu_chain` that finished during the run) and other
lanes' builds ran through the whole measurement. The numbers are recorded as they came out. A
quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with Particle
Grid on its first layer. Each listed frame is drawn whole at Full, three times, and the table
gives the median of the three, in milliseconds. Every draw starts with empty caches: a new frame
cache each time, and the card's kept pictures dropped (`gpu.forget()`) before each card draw.
Each shot first draws one frame on the card, not timed, so the card's passes are built. "card" is
the frame the page receives from the card; "processor" is the same frame drawn on the processor.
On the card, the points are still worked out on the processor and only the drawing is on the
card.

| Shot | Drawn by | Frame 0 | Frame 60 | Frame 120 | Frame 239 |
|---|---|---:|---:|---:|---:|
| no effect | card | 60.1 ms | 45.8 ms | 54.5 ms | 56.4 ms |
| no effect | processor | 46.8 ms | 45.2 ms | 46.6 ms | 51.1 ms |
| Particle Grid as added (1271 points) | card | 58.8 ms | 63.7 ms | 56.7 ms | 59.2 ms |
| Particle Grid as added (1271 points) | processor | 63.9 ms | 61.0 ms | 56.4 ms | 59.1 ms |
| 100 by 100 glowing points, rippling and drifting, added | card | 68.3 ms | 61.5 ms | 75.0 ms | 64.6 ms |
| 100 by 100 glowing points, rippling and drifting, added | processor | 64.5 ms | 80.9 ms | 62.6 ms | 63.4 ms |
| 200 by 100 by 5 points tipped, coloured and pushed by a layer | card | 87.7 ms | 70.0 ms | 74.7 ms | 74.3 ms |
| 200 by 100 by 5 points tipped, coloured and pushed by a layer | processor | 80.5 ms | 92.4 ms | 73.1 ms | 88.9 ms |

**Reading it.** Card and processor are close in every shot. The differences (up to about 20 ms
either way) are no larger than the swings the other tests caused, so these numbers do not show
whether the card is faster. With 100,000 points (the last shot), the card was a little faster at
three of the four frames. Working the points out is still done on the processor, which these
times include. Re-run the command above when no other cargo, rustc or test process is running,
and replace this table.
