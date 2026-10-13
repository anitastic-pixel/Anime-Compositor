# B-526: frame times with Turbulent Noise's Overflow, Transform and Sub Settings (D-647)

Measured on 2026-10-13, built with `cargo test --release --test b526_turbulent_more` from 0b39e8b8
(B-526's code), and run with `--ignored b526_turbulent_more_timing`: once on the card, then once
with `B526_CPU` set on the processor. **These numbers are PROVISIONAL**: the machine was busy.
Before the card run, the graphics card had 13.7 GB of its memory in use by other programs and these
build or test programs were running: two b155_gpu_chain, b461_curl_sources, b532_warp_stabilizer,
cargo.exe. Between the card and processor runs, 10.9 GB, with two b155_gpu_chain,
b496_particle_grid, b532_warp_stabilizer, cargo.exe. After the processor run, 12.2 GB, with two
b155_gpu_chain, b530_pixel_motion_blur, b532_warp_stabilizer, cargo.exe. Treat them as an upper
bound, to be measured again on a quiet machine.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then a
Turbulent Noise on each, every eighth frame asked for whole at Full. **Again** is the median of the
loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 58.5 | 111.8 |
| Noise, then Turbulent Noise as added | 60.9 | 186.1 |
| the same with Rotation 30, Offset (40, -25), Perspective Offset, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (3, 2), Center Subscale, Soft Clamp, Hard Light | 78.0 | 200.3 |

First loops (empty caches): card 110.1, 53.0, 63.9; processor 138.8, 184.0, 199.3.

**Reading it.** With every D-647 setting moved, Turbulent Noise costs about 17 ms a frame more on
the card (60.9 to 78.0, three layers) and about 14 ms more on the processor (186.1 to 200.3): the
turned point is worked per octave per pixel. With the settings as added the old path is taken. The
Noise alone took 58.5 on the card here against 24 to 27 in earlier runs, so the busy card inflates
every number.
