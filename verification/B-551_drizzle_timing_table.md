# B-551: frame times with Drizzle (D-672)

Measured on 2026-10-13, built with `cargo test --release --test b551_drizzle` from lane C's
commit 9f468c19 with later lane C code not yet committed, which holds 65b15745 (B-551's code), and run with `--ignored b551_drizzle_timing`: once on the card, then once
with `B551_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, nothing else was running. Between the card and processor runs, nothing else was running. After the processor run, these build or test programs were running: b492_particle_plane-7a9cb, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Drizzle, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 14.0 | 58.2 |
| Noise, then Drizzle as added | 26.4 | 96.5 |
| Noise, then Drizzle, thirty drops a second, Rippling 720, Spreading 150, a point light | 26.7 | 112.9 |
| Noise, then Drizzle, drip rate keyed 0 to 40 (the settings read back to the in point) | 26.8 | 139.2 |

First loops (empty caches): card 18.0, 30.0, 31.5, 26.4; processor 60.2, 97.5, 109.8, 142.2.

**Reading it.** On the card, Drizzle costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(14.0 to 26.4, three layers); the cost grows with the drops alive at once and how far their ripples spread. Here a busy machine cannot say whether the card is quicker.
