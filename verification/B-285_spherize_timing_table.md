# B-285: frame times with Spherize

Measured on 2026-10-10 on a quiet machine, on the code commit (f5ca8e0d, 715b8522 once rebased onto main), the working copy holding
exactly what was committed in the files the build reads. No other cargo or rustc process was
running: checked before the first round and after each round, none; the owner's app was not
touched. The timing test is `b285_spherize_timing` in `tests/b285_spherize.rs`, built with
`cargo test --release --test b285_spherize` and run with `--ignored b285_spherize_timing`
(`B285_CPU` set for the processor). Two rounds on the card and one on the processor; each figure
is from one round, not a median of rounds.

Spherize is new, so the "before" is the same shot without it, Noise alone, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-284: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first (round 1) | Card again (round 1) | Card first (round 2) | Card again (round 2) | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 20.4 | 13.9 | 15.3 | 11.7 | 58.8 | 41.3 |
| Noise, then Spherize as added (radius 100) | 21.9 | 16.0 | 17.8 | 14.1 | 56.0 | 55.7 |
| Noise, then Spherize, radius 2500 | 20.6 | 23.2 | 20.9 | 17.3 | 66.7 | 66.8 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers: on the card Spherize
as added costs about 0.7 to 0.8 ms (16.0 against 13.9, 14.1 against 11.7), and at radius 2500,
where every pixel is inside the sphere and works its arcsine, about 1.9 to 3.1 ms, the two rounds
disagreeing by more than the effect's own cost at as-added; both inside Target P2's 4 ms. On the
processor about 4.8 ms as added and 8.5 ms at radius 2500 a layer. The card's first round at
radius 2500 came out faster from empty caches than again, so read its round-1 "again" with care.
