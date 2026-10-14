# B-461: frame times with Curl Noise, Source This Layer and Other Layer (D-583)

**Measured again on a quiet machine** on 2026-10-14 at 91452ccd (after D-680, which makes Curl Noise's walk read up to ceil(radius / steps) points a step), built with `cargo test --release --test b461_curl_sources --no-run` and run with `--ignored b461_curl_sources_timing` on its own: once on the card, then once with `B461_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program, the owner's app was not open, and two looks 5 s apart found the card idle (2% and 0% busy; 4.0 GB of its memory held by the desktop and open programs) and the processor 2% busy. After the card run the card was 0% busy; after the processor run it was 11% busy (another program on the desktop, not a build or test) and the processor 2%. One run each, the Again column a median of the loops after the first. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build.

Earlier figures (2026-10-12, at dea24e0a, before D-680, on a machine that was not quiet), Again, card / processor: Noise alone 133.5 / 92.8; This Layer as added 147.0 / 286.2; with Softness 50 79.1 / 332.8 ms.

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
| Noise alone | 12.6 | 42.3 |
| Noise, then Curl Noise This Layer as added | 83.8 | 263.8 |
| the same with Softness 50 | 87.8 | 298.3 |

First loops (empty caches): card 16.1, 88.0, 92.8; processor 41.4, 261.4, 306.2.

**Reading it.** On the card, Curl Noise on Source This Layer costs about 71 ms a frame over the
Noise alone (12.6 to 83.8, three layers), and Softness 50 adds about 4 ms more (87.8): the same
two passes as Internal plus the card's lightness pass, and with Softness two blur passes. On the
processor it costs about 222 ms as added and about 256 ms with Softness 50. The card is about
three times quicker than the processor here.

**Against the earlier figures.** These quiet figures are lower than the earlier ones (card 147.0
to 83.8, processor 286.2 to 263.8 as added), but the earlier ones were taken on a busy machine
(the card's Noise alone was 133.5 then against 12.6 now), so this table cannot say how much
D-680 slowed this setting. B-326's table shows the slowdown on a like-for-like shot.

## After audits P2 and P9 (2928a020, 0bd82a03)

**Measured** on 2026-10-14 at 508a1d3d, built with `cargo test --release --test b461_curl_sources --no-run` (own target folder) and run with `--ignored --exact b461_curl_sources_timing`, one test at a time, nothing else of this work running, on the card only. Just before the run the card was 0% busy and the processor about 3%. Same machine as above. One run, eight loops; **Again** is the median of the seven after the first.

| Shot | Card before (above) | Card after |
|---|---:|---:|
| Noise alone | 12.6 | 12.4 |
| Noise, then Curl Noise This Layer as added | 83.8 | 85.8 |
| the same with Softness 50 | 87.8 | 89.1 |

First loops: 16.1, 91.2, 93.4.

**Reading it.** No change, as planned: Source This Layer and Other Layer keep double precision (in single they were 7 levels off) and their field follows the picture, so it is drawn every time. The 1 to 2 ms differences are too small for one run to tell apart.
