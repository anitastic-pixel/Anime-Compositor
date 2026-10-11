# B-388: frame times with Energy Stroke (D-508) — PROVISIONAL

Measured on 2026-10-11, built with `cargo test --release --test
b388_energy_stroke` from 90490e27 (B-388's code), and run with `--ignored
b388_energy_stroke_timing`: once on the card, then once with `B388_CPU` set on the processor.
**The machine was not quiet.** Another lane's `b155_gpu_chain` test was drawing on the same card
and other cargo and rustc processes were running before, during and after both runs (five were
still running when the second run ended). The numbers below are recorded as they came out. They
are not a measurement of Energy Stroke: the card's Noise alone took 262 ms a frame here, against
11.8 ms on the quiet machine in B-387's table. A quiet re-measure is owed.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Energy
Stroke, every eighth frame asked for whole at Full. **Again** is the median of the loops after
the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 262.4 | 83.6 |
| Noise, then Energy Stroke as added | 157.2 | 70.8 |
| Noise, then Energy Stroke, orange, intensity 300, spread 120 | 159.5 | 321.0 |
| Noise, then Energy Stroke, distortion 20, flicker 60 | 282.7 | 219.1 |

First loops (empty caches): card 268.6, 226.4, 217.6, 283.6; processor 78.4, 94.5, 259.1, 537.4.

**Reading it.** Nothing can be read from these numbers: shots with the effect ran faster than
the shot without it, and the processor's Noise alone ran slower again than first. They show
only that every shot draws to the end on both. Re-run the two commands above when no other
cargo, rustc or test process is running and the card is idle, and replace this table.
