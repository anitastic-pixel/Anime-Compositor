# B-332: frame times with Noise HLS Auto (D-452)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b332_noisehlsauto --no-run` and run with `--ignored b332_noisehlsauto_timing`: once on the card, then once with `B332_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (1 and 4% busy; 4.2 GB of its memory held by the desktop and open programs); the same after each. The processor was 8% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 15.8, processor 46.7; Noise, then Noise HLS Auto as added (Uniform, lightness 10, speed 1) card 21.2, processor 69.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Noise HLS Auto on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.4 | 41.8 |
| Noise, then Noise HLS Auto as added (Uniform, lightness 10, speed 1) | 16.1 | 62.3 |
| Noise, then Noise HLS Auto Squared, hue 40, lightness 20, saturation 40 | 17.1 | 63.0 |
| Noise, then Noise HLS Auto Grain, grain size 2.5, hue 40, lightness 20, saturation 40 | 18.7 | 66.9 |

First loops (empty caches): card 16.6, 20.8, 21.4, 22.1; processor 41.4, 61.8, 62.6, 66.7.

**Reading it.** Three Noise HLS Autos add 3.7 to 6.3 ms a frame on the card over Noise alone (16.1
to 18.7 against 12.4), one grade pass each on a 1920 by 1080 layer: about 1.2 to 2.1 ms a layer,
a little over Target P1's 1 ms for one effect (three value noises and an HSL round trip a pixel,
in the grade pass's double precision, as Noise HLS). On the processor they cost about 7 to 8 ms a
layer (62.3 to 66.9 against 41.8). The busy run's 1.8 to 3.8 ms a layer on the card were partly
the machine; the reading, just over P1, stands.
