# B-348: frame times with Channel Threshold (D-468)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b348_channel_threshold --no-run` and run with `--ignored b348_channel_threshold_timing`: once on the card, then once with `B348_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 29.5, processor 123.4; Noise, then Channel Threshold as added (luminance at 128) card 47.0, processor 98.7 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Channel Threshold on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 41.6 |
| Noise, then Channel Threshold as added (luminance at 128) | 19.2 | 47.5 |
| Noise, then Channel Threshold, RGB at 200 turned over at blend 30 | 19.3 | 48.0 |

First loops (empty caches): card 16.4, 23.2, 22.5; processor 41.8, 47.4, 47.4.

**Reading it.** On the card, Channel Threshold adds 7.0 ms a frame over the Noise alone as added (12.2 to 19.2, three layers), about 2.3 ms a 1080p layer, and 7.1 ms in the second setting (19.3): one tone pass, each pixel read once, whatever the channel. On the processor it adds about 6 ms (41.6 to 47.5 and 48.0). The whole frame is about 2.5 times quicker on the card (19.2 against 47.5). The busy run could not say; this one can.
