# B-461: frame times with Curl Noise, Source This Layer and Other Layer (D-583)

Measured on 2026-10-12, built with `cargo test --release --test b461_curl_sources` from dea24e0a
(B-461's code), and run with `--ignored b461_curl_sources_timing`: once on the card, then once
with `B461_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 14.7 GB of its memory in use by another program and these build or test programs were running: b221_gpu_mix-7be658513acc, cargo.exe, rustc.exe. Between the card and processor runs, the graphics card had 7.8 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b221_gpu_mix-7be658513acc, b490_particle_space-c0ea6, cargo.exe. After the processor run, the graphics card had 13.1 GB of its memory in use by another program and these build or test programs were running: b155_gpu_chain-0784c04478, b221_gpu_mix-7be658513acc, cargo.exe, rustc.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Curl Noise on Source This Layer on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 133.5 | 92.8 |
| Noise, then Curl Noise This Layer as added | 147.0 | 286.2 |
| the same with Softness 50 | 79.1 | 332.8 |

First loops (empty caches): card 233.9, 308.3, 74.2; processor 89.7, 281.6, 379.9.

**Reading it.** On the card, Curl Noise, Source This Layer and Other Layer costs an unknown amount a frame (the machine was busy) over the Noise alone in the first setting
(133.5 to 147.0, three layers), and the second setting takes 79.1: the same two passes as Internal plus the card's lightness pass, and with Softness two blur passes.
At these settings a busy machine cannot say whether the card is quicker than the processor here.
