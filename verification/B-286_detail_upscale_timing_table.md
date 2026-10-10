# B-286: frame times with Detail-preserving Upscale

Measured on 2026-10-10 on a quiet machine, on the code commit (368de4c3, 1462b70a once rebased onto main), the working copy holding
exactly what was committed in the files the build reads. No other cargo or rustc process was
running: checked before the first round and after each round, none; the owner's app was not
touched. The timing test is `b286_detail_upscale_timing` in `tests/b286_detail_upscale.rs`, built with
`cargo test --release --test b286_detail_upscale` and run with `--ignored b286_detail_upscale_timing`
(`B286_CPU` set for the processor). Two rounds on the card and one on the processor; each figure
is from one round, not a median of rounds.

Detail-preserving Upscale is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-285: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first (round 1) | Card again (round 1) | Card first (round 2) | Card again (round 2) | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 17.7 | 12.6 | 16.2 | 12.3 | 40.4 | 41.4 |
| Noise, then Upscale as added (scale 100, Detail 20) | 19.8 | 16.5 | 20.5 | 16.8 | 112.7 | 113.4 |
| Noise, then Upscale, scale 200 | 24.8 | 21.6 | 24.6 | 21.7 | 267.4 | 270.8 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers: on the card the
Upscale as added costs about 1.3 to 1.5 ms (16.5 against 12.6, 16.8 against 12.3), and at scale
200, where each layer grows to 3840 by 2160 and the sharpening blur is wider, about 3.0 to 3.1 ms;
both inside Target P3. On the processor about 24 ms as added and 76 ms at scale 200 a layer: the
processor draws it only when the card cannot.
