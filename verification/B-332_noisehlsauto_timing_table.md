# B-332: frame times with Noise HLS Auto (D-452)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b332_noisehlsauto` from fe54593f (B-332's code), and run with `--ignored
b332_noisehlsauto_timing`: once on the card, then once with `B332_CPU` set on the processor.
The machine was not quiet: no cargo or test process was running when it started, but the card was
shared with other lanes' tests (Noise alone 15.8 ms against 12.9 in B-331's quiet run), and half an
hour's wait found no quiet minute. To be measured again on a quiet machine.

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
| Noise alone | 15.8 | 46.7 |
| Noise, then Noise HLS Auto as added (Uniform, lightness 10, speed 1) | 21.2 | 69.6 |
| Noise, then Noise HLS Auto Squared, hue 40, lightness 20, saturation 40 | 24.5 | 68.8 |
| Noise, then Noise HLS Auto Grain, grain size 2.5, hue 40, lightness 20, saturation 40 | 27.2 | 73.4 |

First loops (empty caches): card 20.3, 26.6, 28.8, 31.2; processor 47.4, 69.1, 71.5, 72.6.

**Reading it.** Three Noise HLS Autos add 5.4 to 11.4 ms a frame on the card over Noise alone,
one grade pass each on a 1920 by 1080 layer; Target P1 asks 1 ms or less for one effect, so this is
over Target P1: about 1.8 to 3.8 ms a layer on the card, against 1 ms (three value noises and an HSL round trip a pixel, in the grade pass's double precision, as Noise HLS).
