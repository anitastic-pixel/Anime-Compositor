# B-525: frame times with Curl Noise's Rotation, Scale and Perspective Offset (D-646)

**Measured again on a quiet machine** on 2026-10-14 at 91452ccd (after D-680, which makes Curl Noise's walk read up to ceil(radius / steps) points a step), built with `cargo test --release --test b525_curl_transform --no-run` and run with `--ignored b525_curl_transform_timing` on its own: once on the card, then once with `B525_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. Before the card run the check waited 516 s for the card to settle (it read 12 to 21% busy from desktop programs meanwhile) and started with the card 6% busy and the processor 4%; before the processor run the card was 4% busy and the processor 4%. After the runs the card was 1% and 11% busy and the processor 5% and 2%. One run each, the Again column a median of the loops after the first. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build.

Earlier figures (2026-10-13, from 7b7655a8, before D-680, PROVISIONAL on a machine that was not quiet), Again, card / processor: Noise alone 27.1 / 57.7; as added 76.2 / 394.5; turned, scaled, Perspective Offset 76.7 / 316.7 ms.

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
| Noise alone | 13.0 | 41.8 |
| Noise, then Curl Noise as added | 111.8 | 356.5 |
| the same with Rotation 45, Scale 150, Perspective Offset on | 110.2 | 371.8 |

First loops (empty caches): card 17.0, 115.0, 116.6; processor 42.0, 357.0, 377.0.

**Reading it.** On the card, Curl Noise costs about 99 ms a frame over the Noise alone
(13.0 to 111.8, three layers), and turning, scaling and Perspective Offset add nothing measurable
(110.2). On the processor Curl Noise costs about 315 ms a frame (41.8 to 356.5), and the Transform
adds about 15 ms (371.8). The earlier, lower card figure for Curl Noise (76.2) was before D-680;
B-326 shows the same rise for the plain effect.
