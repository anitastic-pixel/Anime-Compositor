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

## After audits P2 and P9 (2928a020, 0bd82a03)

**Measured** on 2026-10-14 at 508a1d3d, built with `cargo test --release --test b525_curl_transform --no-run` (own target folder) and run with `--ignored --exact b525_curl_transform_timing`, one test at a time, nothing else of this work running, on the card only. One reading of the card just before the run said 81% busy; it was taken the moment the previous timing test (B-461) ended, so it probably caught that test's last frames, and the card read 1% a few minutes later. The as-added figure agrees with B-326's own run of the same shot that day (32.6 ms). Same machine as above. One run, eight loops; **Again** is the median of the seven after the first.

| Shot | Card before (above) | Card after |
|---|---:|---:|
| Noise alone | 13.0 | 12.3 |
| Noise, then Curl Noise as added | 111.8 | 32.4 |
| the same with Rotation 45, Scale 150, Perspective Offset on | 110.2 | 32.3 |

First loops: 15.9, 36.1, 36.2.

**Reading it.** On the card, Curl Noise now costs about 20 ms a frame over the Noise alone, down from about 99, and Rotation, Scale and Perspective Offset still add nothing measurable. Both shots fit in real time (41.7 ms at 24 a second). The pictures are the same within 1 level of 255 (`D-646_curl_transform_table.md`).
