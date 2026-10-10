# B-326: frame times with Curl Noise (D-446)

Measured on 2026-10-10, built with `cargo test --release --test b326_curl_noise` from 649598ba
(B-326's code), and run with `--ignored b326_curl_noise_timing`: once on the card, then once with
`B326_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 15.5 GB of its memory in use by another program and these build or test programs were running: b333_dust_scratches-8a57c, cargo.exe. Between the card and processor runs, the graphics card had 7.5 GB of its memory in use by another program. After the processor run, the graphics card had 11.1 GB of its memory in use by another program. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Curl Noise, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 420.0 | 46.1 |
| Noise, then Curl Noise as added (radius 30, 12 samples) | 66.6 | 271.7 |
| Noise, then Curl Noise Input Noise (no lines) | 107.3 | 196.4 |
| Noise, then Curl Noise radius 100, 24 samples (the longest lines) | 82.6 | 379.9 |

First loops (empty caches): card 1116.6, 99.3, 64.4, 84.6; processor 44.5, 267.3, 202.5, 393.4.

**Reading it.** On the card, Curl Noise costs an unknown amount a frame (the machine was busy) over the Noise alone as added
(420.0 to 66.6, three layers): one pass draws the flow and the seed over the layer
and a margin of the sample radius, a second follows the flow both ways from every pixel, so the
cost grows with Sample Count. At these settings a busy machine cannot say whether the card is quicker than the processor here.
