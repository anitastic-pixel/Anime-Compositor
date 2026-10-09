# B-239: frame times with Cross Blur

Measured on 2026-10-08 on the code of the code commit (9954cf9), after it was committed and
pushed, the working copy holding exactly what was committed in the files the build reads. No
other cargo or rustc process was running and the owner's app was not open: checked before the
first round and after each one, none. The processor's load from background programs was not
recorded. The timing test is `b239_cross_blur_timing` in `tests/b239_cross_blur.rs`, built with
`cargo test --release --test b239_cross_blur` and run with `--ignored b239_cross_blur_timing`: one round drawn by
the card, then one round, with `B239_CPU` set, by the processor. One round each, so these are
single figures, not medians of rounds.

Cross Blur is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
Cross Blur (edges transparent, so the layer grows by the larger reach). So the effect has new input every frame and is worked every time. Every eighth frame is
asked for whole, as the viewer asks for it, at Full: with Draw on: GPU (`preview_frame_srgb8`), or
by the processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30
frames, starting with empty caches. **Again** is the median of the loops after it: seven on the
card (210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.5 | 12.7 | 40.6 | 40.3 |
| Noise, then Cross Blur as added (Radius X 10, Radius Y 10, blend) | 20.4 | 16.1 | 105.0 | 105.7 |
| Noise, then Cross Blur, Radius X 40, Radius Y 4, add | 23.6 | 20.7 | 138.8 | 139.8 |
| Noise, then Cross Blur, Radius X 100, Radius Y 100, screen | 34.1 | 30.2 | 202.9 | 204.2 |

**Reading it.** On the card Cross Blur as added costs about 3.4 ms a frame over Noise alone, about 1.1 ms a 1080p layer; a long flare (Radius X 40) about 2.7 ms a layer, and Radius 100 both ways in Screen about 5.8 ms a layer, as the layer grows by 100 pixels each side. On the processor the same three cost about 22, 33 and 55 ms a layer. Well inside Target P3 on the card.
