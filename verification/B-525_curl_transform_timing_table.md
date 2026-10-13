# B-525: frame times with Curl Noise's Rotation, Scale and Perspective Offset (D-646)

Measured on 2026-10-13, built with `cargo test --release --test b525_curl_transform` from 7b7655a8
(B-525's code) plus the timing test, and run with `--ignored b525_curl_transform_timing`: once on
the card, then once with `B525_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was not quiet. Before the card run, the graphics card had 7.8 GB of its memory in use by other programs and these build or test programs were running: b155_gpu_chain, b532_warp_stabilizer, b533_rolling_shutter_repair, cargo.exe, rustc.exe. Between the card and processor runs, 7.9 GB, with b490_particle_space, b532_warp_stabilizer, b533_rolling_shutter_repair, cargo.exe. After the processor run, 8.2 GB, with b493_particle_playground, b495_particle_emitter, b532_warp_stabilizer, b533_rolling_shutter_repair, cargo.exe. Treat them as an upper bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Curl Noise on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 27.1 | 57.7 |
| Noise, then Curl Noise as added | 76.2 | 394.5 |
| the same with Rotation 45, Scale 150, Perspective Offset on | 76.7 | 316.7 |

First loops (empty caches): card 31.3, 87.9, 79.4; processor 54.4, 294.2, 341.7.

**Reading it.** On the card, Curl Noise costs about 49 ms a frame over the Noise alone
(27.1 to 76.2, three layers), and turning, scaling and Perspective Offset add nothing measurable
(76.7). On the processor Curl Noise costs roughly 260 to 340 ms a frame; the two processor shots
differ by more than the Transform could explain (the turned one is the quicker), so that spread is
the busy machine, not the setting.
