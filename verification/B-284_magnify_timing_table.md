# B-284: frame times with Magnify

Measured on 2026-10-09 on a quiet machine, on the code commit (5f297f6c, 56d0c4d5 once rebased onto main), the working copy holding
exactly what was committed in the files the build reads. No other cargo or rustc process was
running: checked before the card round and after each round, none; the owner's app was not
touched. The timing test is `b284_magnify_timing` in `tests/b284_magnify.rs`, built with
`cargo test --release --test b284_magnify` and run with `--ignored b284_magnify_timing`
(`B284_CPU` set for the processor). One round each, so these are single figures, not medians of
rounds.

Magnify is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-273: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.2 | 12.3 | 42.3 | 42.5 |
| Noise, then Magnify as added (radius 100) | 18.8 | 15.3 | 55.2 | 55.3 |
| Noise, then Magnify, radius 500, soft, feather 50 | 19.5 | 16.4 | 54.9 | 54.5 |
| Noise, then Magnify, radius 500, scatter, Overlay | 22.9 | 20.2 | 68.4 | 69.1 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers: on the card Magnify
as added costs about 1.0 ms (15.3 against 12.3), radius 500 soft with a feather about 1.4 ms, and
radius 500 scattered in Overlay about 2.6 ms, inside Target P2's 4 ms. On the processor the same three
cost about 4.3, 4.0 and 8.9 ms a layer. Every pixel of the layer is visited whatever the radius,
since the layer under the lens is laid down too; scatter's hash and Overlay's sum are the extra.
