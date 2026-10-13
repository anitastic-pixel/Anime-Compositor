# B-550: frame times with Bubbles (D-671)

Measured on 2026-10-13, built with `cargo test --release --test b550_bubbles` from lane C's
commit 9f468c19 with later lane C code not yet committed, which holds e073f346 (B-550's code), and run with `--ignored b550_bubbles_timing`: once on the card, then once
with `B550_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, nothing else was running. After the processor run, the graphics card had 10.3 GB of its memory in use by another program and these build or test programs were running: b492_particle_plane-7a9cb, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Bubbles, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 16.7 | 52.6 |
| Noise, then Bubbles as added | 23.6 | 82.1 |
| Noise, then Bubbles, 1000 of size 3, World, Lighten | 62.0 | 295.4 |
| Noise, then Bubbles, speed keyed 0 to 3 (the settings read back to the in point) | 21.0 | 92.1 |

First loops (empty caches): card 21.8, 26.1, 62.2, 23.8; processor 49.1, 80.0, 290.6, 96.4.

**Reading it.** On the card, Bubbles costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(16.7 to 23.6, three layers); the cost grows with the number of bubbles and their size. Here a busy machine cannot say whether the card is quicker.
