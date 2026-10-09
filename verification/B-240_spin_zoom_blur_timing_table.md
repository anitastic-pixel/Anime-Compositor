# B-240: frame times with Spin & Zoom Blur

Measured on 2026-10-08 on the code of the code commit (9954cf9), after it was committed and
pushed, the working copy holding exactly what was committed in the files the build reads. No
other cargo or rustc process was running and the owner's app was not open: checked before the
first round and after each one, none. The processor's load from background programs was not
recorded. The timing test is `b240_spin_zoom_blur_timing` in `tests/b240_spin_zoom_blur.rs`, built with
`cargo test --release --test b240_spin_zoom_blur` and run with `--ignored b240_spin_zoom_blur_timing`: one round drawn by
the card, then one round, with `B240_CPU` set, by the processor. One round each, so these are
single figures, not medians of rounds.

Spin & Zoom Blur is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
Spin & Zoom Blur about the middle. So the effect has new input every frame and is worked every time. Every eighth frame is
asked for whole, as the viewer asks for it, at Full: with Draw on: GPU (`preview_frame_srgb8`), or
by the processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30
frames, starting with empty caches. **Again** is the median of the loops after it: seven on the
card (210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 15.7 | 12.2 | 40.4 | 40.8 |
| Noise, then Spin & Zoom Blur as added (Straight Zoom 10, quality 50) | 22.5 | 18.4 | 173.3 | 174.5 |
| Noise, then Spin & Zoom Blur, Rotate 30, quality 50 | 30.9 | 27.7 | 750.7 | 746.0 |
| Noise, then Spin & Zoom Blur, Centered Zoom 60, quality 100 | 33.9 | 30.7 | 709.8 | 713.1 |

**Reading it.** On the card Spin & Zoom Blur as added (Straight Zoom 10) costs about 6 ms a frame over Noise alone, about 2 ms a 1080p layer; Rotate 30 about 5 ms a layer and Centered Zoom 60 at quality 100 about 6 ms a layer, where most pixels take the most samples, 256. On the processor the same three cost about 45, 235 and 224 ms a layer, which is why the card draws it.
