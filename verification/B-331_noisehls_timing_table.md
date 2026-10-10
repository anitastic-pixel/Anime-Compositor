# B-331: frame times with Noise HLS (D-451)

Measured on 2026-10-10, built with `cargo test --release --test
b331_noisehls` from 0d14efde (B-331's code), and run with `--ignored
b331_noisehls_timing`: once on the card, then once with `B331_CPU` set on the processor.
The machine was quiet: no other cargo, rustc or test process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Noise HLS on each, every eighth frame asked for whole at Full. **Again** is the median
of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.9 | 44.6 |
| Noise, then Noise HLS as added (Uniform, lightness 10) | 18.1 | 68.2 |
| Noise, then Noise HLS Squared, hue 40, lightness 20, saturation 40 | 18.3 | 68.6 |
| Noise, then Noise HLS Grain, grain size 2.5, hue 40, lightness 20, saturation 40 | 20.3 | 72.8 |

First loops (empty caches): card 17.2, 23.4, 22.9, 24.8; processor 43.8, 69.0, 68.5, 74.8.

**Reading it.** Three Noise HLSs add 5.2 to 7.4 ms a frame on the card over Noise alone,
one grade pass each on a 1920 by 1080 layer; Target P1 asks 1 ms or less for one effect, so
this is over Target P1: about 1.7 to 2.5 ms a layer on the card, against 1 ms (three value noises and an HSL round trip a pixel, in the grade pass's double precision).
