# B-352: frame times with Gaussian Blur (Legacy) (D-472)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b352_gaussian_blur_legacy --no-run` and run with `--ignored b352_gaussian_blur_legacy_timing`: once on the card, then once with `B352_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 5% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 29.7, processor 124.1; Noise, then Gaussian Blur (Legacy) Blurriness 10 card 63.5, processor 251.9 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
a Gaussian Blur (Legacy) on each, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 13.4 | 44.8 |
| Noise, then Gaussian Blur (Legacy) Blurriness 10 | 16.1 | 72.1 |
| the same at Blurriness 100 | 15.6 | 133.5 |

First loops (empty caches): card 17.2, 19.4, 18.3; processor 44.3, 72.0, 133.8.

**Reading it.** On the card, Gaussian Blur (Legacy) at Blurriness 10 costs about 2.7 ms a frame
over the Noise alone (13.4 to 16.1, three layers), under 1 ms a 1080p layer, and at Blurriness 100
about 2.2 ms (15.6), no more than at 10 in this run: Gaussian Blur's two passes, each pixel reading
2 ceil(6.5 x 0.3 B) + 1 neighbours a pass. (The earlier, busy run's 77.7 at Blurriness 100 was the
machine, not the blur.) On the processor the same settings cost about 27 and 89 ms a frame (72.1
and 133.5 against 44.8). At these settings the card is quicker than the processor here, about
four to eight times.
