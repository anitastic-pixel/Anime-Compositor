# B-326: frame times with Curl Noise (D-446)

**Measured again on a quiet machine** on 2026-10-14 at 91452ccd (after D-680, which makes Curl Noise's walk read up to ceil(radius / steps) points a step), built with `cargo test --release --test b326_curl_noise --no-run` and run with `--ignored b326_curl_noise_timing` on its own: once on the card, then once with `B326_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program, the owner's app was not open, and two looks 5 s apart found the card idle (1% and 5% busy; 3.8 to 3.9 GB of its memory held by the desktop and open programs) and the processor 5% and 4% busy; after each run the card was 0 to 1% busy and the processor 2% busy. One run each, the Again column a median of the loops after the first. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build.

Earlier figures (2026-10-10, at 649598ba, before D-680, on a machine that was not quiet), Again, card / processor: Noise alone 420.0 / 46.1; as added 66.6 / 271.7; Input Noise 107.3 / 196.4; radius 100, 24 samples 82.6 / 379.9 ms.

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
| Noise alone | 13.2 | 43.4 |
| Noise, then Curl Noise as added (radius 30, 12 samples) | 112.4 | 362.2 |
| Noise, then Curl Noise Input Noise (no lines) | 43.1 | 205.5 |
| Noise, then Curl Noise radius 100, 24 samples (the longest lines) | 264.6 | 713.5 |

First loops (empty caches): card 17.7, 116.7, 46.2, 270.1; processor 41.9, 362.3, 215.6, 713.0.

**Reading it.** On the card, Curl Noise as added costs about 99 ms a frame over the Noise alone
(13.2 to 112.4, three layers), and at radius 100 with 24 samples about 251 ms. On the processor
it costs about 319 ms as added and about 670 ms at radius 100. The card is about three times
quicker than the processor here.

**Slower after D-680, as expected.** The new rule is slower than the earlier figures even though
those were taken on a busy machine and so were too high: as added, the card went from 66.6 to
112.4 ms (+46) and the processor from 271.7 to 362.2 (+91); at radius 100, 24 samples, the card
went from 82.6 to 264.6 (+182, about 3.2 times) and the processor from 379.9 to 713.5 (+334,
about 1.9 times). Because the earlier figures were inflated, the true slowdown is at least this
much. Input Noise draws no lines, so D-680 should not touch it; its processor figure (205.5
against 196.4) is about where it was.

## After audits P2 and P9 (single precision 2928a020; field kept 0bd82a03)

**Measured** on 2026-10-14 at 508a1d3d, built with `cargo test --release --test b326_curl_noise --no-run` (own target folder) and run with `--ignored --exact b326_curl_noise_timing`, one test at a time, nothing else of this work running, on the card only (P2 and P9 changed only the card; the processor's figures above still stand). Just before the run the card was 2% busy and the processor about 3%. Same machine as above. One run, eight loops; **Again** is the median of the seven after the first.

| Shot | Card before (above) | Card after |
|---|---:|---:|
| Noise alone | 13.2 | 12.3 |
| Noise, then Curl Noise as added (radius 30, 12 samples) | 112.4 | 32.6 |
| Noise, then Curl Noise Input Noise (no lines) | 43.1 | 19.8 |
| Noise, then Curl Noise radius 100, 24 samples (the longest lines) | 264.6 | 48.6 |

First loops: 16.4, 35.0, 23.4, 52.4.

**Reading it.** On the card, Curl Noise as added now costs about 20 ms a frame over the Noise alone (12.3 to 32.6), down from about 99; the longest lines about 36, down from about 251. Every shot but the longest lines (48.6) now fits in real time (41.7 ms at 24 a second). The two changes are timed together here, not apart. The pictures are the same within 1 level of 255 (`D-446_curl_noise_table.md`), and a kept field gives byte-identical pictures (`P9_curl_field_table.md`, 12 of 12).
