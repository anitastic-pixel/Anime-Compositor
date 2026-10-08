# B-231: frame times with Stretch Levels, Stretch Color, Spread Tones and a temporal smoothing

Measured on 2026-10-08, before the code commit, on the code that commit holds. No other cargo or
rustc process was running: checked before and after every round, none. The owner's app was not
open; the processor was about 14 per cent busy with the usual background programs (a browser) before
the first round. The timing test is `b231_auto_tone_timing` in `tests/b231_auto_tone.rs`, built
with `cargo test --release --test b231_auto_tone` and run three rounds in a row with
`--ignored b231_auto_tone_timing --exact`, each round timing the five shots in the order below.

The four effects are new, so there is no "before" build: the row with Noise alone is the
comparison. The statistics and the stretch are worked by the processor; the viewer's frame is then
finished by the graphics card.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with the effect on each of its first three layers, then a Noise that changes
every frame so the finished frame is never kept. Every eighth frame is asked for whole, as the
viewer asks for it (`preview_frame_srgb8`), at Full with Draw on: GPU. **First** is the median of
the first loop of 30 frames, starting with empty caches; **Again** the median of the next seven
loops, 210 frames. Each figure is the median of the three rounds, in milliseconds a frame.

| Shot | First | Again |
|---|---:|---:|
| Noise alone (before) | 16.1 | 12.3 |
| Stretch Levels as added, then Noise | 18.0 | 13.0 |
| Stretch Color, Snap Neutral Midtones, then Noise | 16.9 | 12.4 |
| Stretch Levels, Temporal Smoothing 0.1 s (2 frames each side), Scene Detect, then Noise | 113.4 | 128.0 |
| Spread Tones, RGB, then Noise | 17.0 | 12.4 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 16.1 / 12.3 | 16.4 / 12.6 | 15.7 / 12.2 |
| Stretch Levels | 16.1 / 12.5 | 18.0 / 13.0 | 18.8 / 14.7 |
| Stretch Color, snap | 16.6 / 12.4 | 16.9 / 12.4 | 19.0 / 14.7 |
| Stretch Levels, smoothing 0.1 s, scene detect | 113.4 / 128.0 | 113.3 / 127.6 | 131.6 / 129.2 |
| Spread Tones | 19.2 / 12.7 | 16.3 / 12.2 | 17.0 / 12.4 |

**Against the target.** EFFECTS.md gives all four Target P2. Without smoothing the four cost at most about 2 ms a frame on three 1080p layers
(16.1 to 18.0 the first time), and nearly nothing played again, because the stretched picture is
kept like any other effect's and only the Noise after it is new each frame. All are well inside a
24 frames a second budget (41.7 ms).

**Temporal smoothing is slow.** With 0.1 seconds (two frames each side) on three layers the frame
takes about 128 ms, three times the budget, about 8 frames a second. Each frame reads the layer's
picture at the four frames around it again and counts each one, and does so every time the frame
is asked for, because the counts decide what the stretch is and so must be known before the kept
picture can be looked up. A longer smoothing costs more in proportion. The way to speed it up,
not done here, is to keep each frame's counts once made, so playing again reads them from memory
(`src/compose.rs`, `fill_stats`, marked). For deflickering a shot for export this is usable; for
playing it live it is not.
