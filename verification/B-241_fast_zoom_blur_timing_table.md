# B-241: frame times with Fast Zoom Blur

Measured on 2026-10-08 on the code of the code commit (9954cf9), after it was committed and
pushed, the working copy holding exactly what was committed in the files the build reads. No
other cargo or rustc process was running and the owner's app was not open: checked before the
first round and after each one, none. The processor's load from background programs was not
recorded. The timing test is `b241_fast_zoom_blur_timing` in `tests/b241_fast_zoom_blur.rs`, built with
`cargo test --release --test b241_fast_zoom_blur` and run with `--ignored b241_fast_zoom_blur_timing`: one round drawn by
the card, then one round, with `B241_CPU` set, by the processor. One round each, so these are
single figures, not medians of rounds.

Fast Zoom Blur is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
Fast Zoom Blur. So the effect has new input every frame and is worked every time. Every eighth frame is
asked for whole, as the viewer asks for it, at Full: with Draw on: GPU (`preview_frame_srgb8`), or
by the processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30
frames, starting with empty caches. **Again** is the median of the loops after it: seven on the
card (210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.8 | 12.8 | 40.5 | 40.4 |
| Noise, then Fast Zoom Blur as added (amount 50, standard) | 32.4 | 29.5 | 782.4 | 807.9 |
| Noise, then Fast Zoom Blur, amount 50, brightest | 32.3 | 29.3 | 824.1 | 819.8 |
| Noise, then Fast Zoom Blur, amount 100, darkest, about 20, 30 | 32.6 | 29.1 | 987.4 | 958.2 |

**Reading it.** On the card Fast Zoom Blur costs about 16 to 17 ms a frame over Noise alone, about 5.5 ms a 1080p layer, whatever the zoom. Amount 100 measured the same as amount 50; the cap of 256 samples a pixel covers much of a 1080p frame at both, a likely reason, not checked. On the processor it is about 255 to 305 ms a layer, which is why the card draws it.
