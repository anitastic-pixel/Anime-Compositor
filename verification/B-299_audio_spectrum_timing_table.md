# B-299: frame times with Audio Spectrum (D-420)

Measured on 2026-10-10, built with `cargo test --release --test b299_audio_spectrum` from 102dd908
(B-299's code), and run with `--ignored b299_audio_spectrum_timing`: once on the card, then once with
`B299_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Audio Spectrum listening to a sound layer of music the test writes (ten seconds, eight
notes from 110 to 1760 hertz), every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 27.9 | 54.7 |
| Noise, then Audio Spectrum as added, listening to the music (64 bands, 90 ms) | 49.8 | 56.5 |
| Noise, then Audio Spectrum, 1024 bands to 4000 Hz, hairlines, 2000 tall | 60.0 | 80.8 |
| Noise, then Audio Spectrum, Analog Dots round the middle, 128 bands, averaged over 200 ms | 75.7 | 66.7 |

First loops (empty caches): card 37.8, 39.1, 65.2, 82.3; processor 53.3, 60.7, 83.7, 65.0.

**Reading it.** As added, the spectrum costs about 22 ms a frame on the card over the Noise alone
(27.9 to 49.8), and about 2 ms on the processor (54.7 to 56.5). The levels are worked out on the
processor every frame either way (one sum per band over the window); the card then draws the marks
with its own "marks" pass, which looks at every piece in a pixel's 16-row band, so many small dots
(Analog Dots round the middle) are slower on the card (75.7) than on the processor (66.7). At
these settings the card is not quicker than the processor for this effect.
