# B-324: frame times with Vegas (D-444)

Measured on 2026-10-10, built with `cargo test --release --test b324_vegas` from 08411bc5
(B-324's code), and run with `--ignored b324_vegas_timing`: once on the card, then once with
`B324_CPU` set on the processor. **Both runs are PROVISIONAL**: `tasklist` showed another lane's test program running as the card run ended and the processor run began, so the numbers may be high.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Vegas on the layer's masks, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.0 | 45.9 |
| Noise, then Vegas as added (mask 1, 32 segments, Width 2) | 24.9 | 48.2 |
| Noise, then Vegas, All Masks, 64 segments, Width 24 | 65.3 | 55.3 |
| Noise, then Vegas, All Masks, 8 segments, Length 0.5, Even, Width 60 | 92.5 | 59.5 |

First loops (empty caches): card 16.4, 27.2, 65.8, 94.7; processor 46.2, 47.7, 55.4, 59.2.

**Reading it.** On the card, Vegas costs about 12 ms a frame over the Noise alone as added
(13.0 to 24.9, three layers); the dashes' straight bits are worked out on the
processor, then one pass a layer, each pixel looking only at the bits near its band of rows.
At these settings the card is not quicker than the processor at every setting.
