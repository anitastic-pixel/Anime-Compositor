# B-237: frame times with Bilateral Blur

Measured on 2026-10-08. The card rounds are on the code of the second code commit (abed3df), the
processor rounds on the first (af8842e). The second commit changes only the card's pass, so the
processor's code is the same in both. No other cargo or rustc process was running and the owner's
app was not open: checked before every set of rounds and after the last, none. The processor's
load from background programs was not recorded. The timing test is `b237_bilateral_blur_timing`
in `tests/b237_bilateral_blur.rs`, built with `cargo test --release --test b237_bilateral_blur`
and run three rounds in a row with `--ignored b237_bilateral_blur_timing --exact`. Each round
times the four shots in the order below, once drawn by the card and once, with `B237_CPU` set, by
the processor.

The effect is new, so there is no "before" build: the row with Noise alone is the comparison.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, and
Bilateral Blur after it. So the effect has new input every frame and is worked every time. Every
eighth frame is asked for whole, as the viewer asks for it, at Full: with Draw on: GPU
(`preview_frame_srgb8`), or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, starting with empty caches. **Again** is the median of the
loops after it: seven on the card (210 frames), two on the processor (60 frames). Each figure is
the median of the three rounds, in milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone (before) | 15.4 | 11.7 | 42.3 | 41.8 |
| Noise, then Bilateral Blur as added (Radius 5, Threshold 20, Colorize on) | 19.7 | 16.1 | 234.1 | 235.3 |
| Noise, then Bilateral Blur, Radius 5, Threshold 20, Colorize off | 18.2 | 15.1 | 149.2 | 150.8 |
| Noise, then Bilateral Blur, Radius 10, Threshold 40, Colorize on | 22.0 | 18.9 | 666.1 | 703.8 |

The rounds, first / again:

| Shot | Card round 1 | Card round 2 | Card round 3 | Processor round 1 | Processor round 2 | Processor round 3 |
|---|---|---|---|---|---|---|
| Noise alone | 15.6 / 11.7 | 15.1 / 11.7 | 15.4 / 11.7 | 41.1 / 41.6 | 42.5 / 41.8 | 42.3 / 42.5 |
| As added | 19.3 / 16.1 | 19.7 / 16.0 | 20.1 / 16.1 | 234.7 / 234.3 | 234.1 / 235.3 | 233.5 / 236.0 |
| Colorize off | 18.1 / 15.1 | 18.2 / 15.0 | 18.5 / 15.2 | 149.2 / 150.8 | 148.9 / 150.7 | 151.8 / 152.3 |
| Radius 10, Threshold 40 | 22.0 / 18.9 | 22.1 / 18.9 | 21.9 / 18.9 | 663.9 / 703.8 | 666.1 / 705.7 | 668.9 / 669.0 |

**A first card pass, before this table.** The card pass as first written (af8842e) added up the
taps in double precision. Three rounds on the same machine, same build settings, gave (median,
first / again) 61.6 / 58.0 as added, 48.6 / 45.1 with Colorize off and 184.9 / 181.5 at Radius 10,
against 15.9 / 11.8 for Noise alone. This card does double precision far more slowly than single.
The second commit (abed3df) adds up in single precision and is about ten times faster in the time
the effect adds. Every check still passes, every picture within 1 level of the processor
(`verification/D-358_bilateral_blur_table.md`, 123 of 123).

**Against the target.** EFFECTS.md gives Bilateral Blur Target P3 (8 ms or less added).

- **On the card** it adds about 4.4 ms a frame on three 1080p layers as added (about 1.5 ms a
  layer), about 3.4 ms with Colorize off and about 7.2 ms at Radius 10 (about 2.4 ms a layer).
  That is within the target.
- **On the processor** it adds about 190 ms as added and about 660 ms at Radius 10. Each pixel
  looks at every pixel of its disc, so the time grows with the square of the radius. The card is
  the way to play it; exports, which use the processor, are slow with a large radius.
