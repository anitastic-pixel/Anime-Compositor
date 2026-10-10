# B-300: frame times with Audio Waveform (D-421)

Measured on 2026-10-10, built with `cargo test --release --test b300_audio_waveform` from d2dbe8e5
(B-300's code), and run with `--ignored b300_audio_waveform_timing`: once on the card, then once with
`B300_CPU` set on the processor. `tasklist` showed no other cargo, rustc or test program before, between and after the two runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-299's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then an Audio Waveform listening to a sound layer of B-299's music (ten seconds, eight notes from
110 to 1760 hertz), every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.7 | 43.4 |
| Noise, then Audio Waveform as added, listening to the music (32 points, 90 ms) | 15.9 | 49.2 |
| Noise, then Audio Waveform, Digital, 1024 samples over 200 ms, hairlines, 300 tall | 25.1 | 70.7 |
| Noise, then Audio Waveform, Analog Dots, 128 samples, 800 tall, thickness 8 | 15.9 | 54.9 |

First loops (empty caches): card 16.2, 20.5, 28.6, 22.0; processor 42.3, 48.8, 68.2, 55.2.

**Reading it.** On the card, Audio Waveform costs about 3 ms a frame over the Noise alone as added
(12.7 to 15.9, three layers), and about 12 ms with 1024 strokes (12.7 to 25.1). At these settings
the card is quicker than the processor for this effect.
