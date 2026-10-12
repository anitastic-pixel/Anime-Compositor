# B-450: frame times with Color Grade's third unit (D-571)

Measured on 2026-10-12, built with `cargo test --release --test b450_color_grade_3` from 3aec1083, and run with `--ignored b450_color_grade_3_timing`: once on the card, then once with `B450_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. During the runs other lanes' build and test programs were running (ten cargo.exe, two b155_gpu_chain, b461_curl_sources and an app test program), two of them drawing on the same card, and the Noise alone came out at 107.4 ms a frame on the card where a quiet run gives about 30. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a Color Grade on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 107.4 | 111.0 |
| Noise, then Color Grade: the four new curves and three secondary wheels | 136.4 | 259.9 |
| Noise, then Color Grade: an HSL secondary, its key denoised 12 and blurred 8 (drawn by the processor either way) | 737.1 | 348.6 |

First loops (empty caches): card 444.0, 135.4, 708.1; processor 72.7, 338.1, 470.0.

**Reading it.** The four curves and the three wheels add about 29 ms a frame on the card over the Noise alone, about 10 ms a 1080p layer, against about 50 ms a layer on the processor, three layers, on a busy machine. A key with Denoise or Blur is worked out by the processor (D-571): on the processor's own path the whole secondary with its refined key adds about 80 ms a layer; on the card's path the frame took 737.1 ms against the processor path's 348.6, so with the card on, a refined key is slower than with it off. That is the hand-over to the processor and back for each such layer, on a card two other programs were also using; it is the first thing to measure again on a quiet machine, and the reason to teach the card the key's median and blur if it holds (the `ponytail:` note in `src/compose.rs`).
