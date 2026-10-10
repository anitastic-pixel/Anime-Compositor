# B-304: frame times with Threads (D-425)

Measured on 2026-10-10, built with `cargo test --release --test b304_threads` from cf225c53
(B-304's code), and run with `--ignored b304_threads_timing`: once on the card, then once with
`B304_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Threads, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.6 |
| Noise, then Threads as added (50 apart, coverage 90, shadowing 50) | 16.4 | 48.3 |
| Noise, then Threads, 12 by 8, overlaps 3, turned 30, shadowing 100, texture 60 | 16.2 | 48.1 |
| Noise, then Threads, 4 by 4, coverage 50, texture 100 | 16.4 | 56.6 |

First loops (empty caches): card 16.4, 20.8, 20.3, 19.3; processor 40.2, 48.5, 47.9, 48.5.

**Reading it.** On the card, Threads costs about 4 ms a frame over the Noise alone as added
(12.2 to 16.4, three layers); one pass a layer, each pixel worked on its own, so the
settings barely change the cost. At these settings the card is quicker than the processor here.
