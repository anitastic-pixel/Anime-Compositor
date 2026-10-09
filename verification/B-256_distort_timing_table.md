# B-256..B-258: frame times with Bend It, Bender and Blobbylize

**PROVISIONAL.** Measured on 2026-10-09 on the code of the code commit (053957e), on a machine
that was not quiet. Another agent's cargo processes (PIDs 67100 and 31368) were running the whole
time, with its test binary `b221_gpu_mix` (about 0.8 to 1.1 GB, using the same card) running
before, during and after every round. Expect these figures to move when measured again on a quiet
machine. The owner's app was not open. The timing test is `b256_distort_timing` in
`tests/b256_bend_it_bender_blobbylize.rs`, built with
`cargo test --release --test b256_bend_it_bender_blobbylize` and run with
`--ignored b256_distort_timing`. Rounds: one by the card, then one by the processor (with
`B256_CPU` set), then a second by the card. These are single figures per round, not medians of
rounds.

There is no "before" for the effects themselves, which are new. Noise alone is the shot without
them, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot is `verification/B-08a_project.json`: 1920 by 1080, 24
frames a second, 240 frames. Each of its first three layers has a Noise that changes every frame,
then the effect with its starting settings. So the effect gets new input every frame and is
worked every time. Every eighth frame is asked for whole at Full, as the viewer asks for it:
- with Draw on: GPU (`preview_frame_srgb8`; no frame fell back to the processor, the test
  checks), or
- by the processor alone (`preview_frame_cached`).

**First** is the median of the first loop of 30 frames, which starts with empty caches. **Again**
is the median of the loops after it: seven on the card (210 frames), two on the processor (60
frames). All figures are milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again | Card first, round 2 | Card again, round 2 |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 18.2 | 12.9 | 41.1 | 42.2 | 45.0 | 13.2 |
| Noise, then Bend It as it starts (45, up the middle) | 37.0 | 21.0 | 72.6 | 73.2 | 25.8 | 22.6 |
| Noise, then Bender as it starts (bend 20, up the middle) | 18.9 | 15.5 | 84.1 | 61.9 | 19.2 | 15.3 |
| Noise, then Blobbylize as it starts (its own alpha, softness 10) | 23.8 | 19.5 | 133.9 | 141.0 | 45.8 | 19.1 |

**Reading it (provisional).** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Bend It | about 2.7 to 3.1 ms (21.0 and 22.6 against 12.9 and 13.2) | within Target P2's 4 ms | about 10.3 ms (73.2 against 42.2) |
| Bender | about 0.7 to 0.9 ms | within P2 | about 6.6 ms |
| Blobbylize | about 2.0 to 2.2 ms | within P3 | about 33 ms (its two blurs and lighting) |

The "first" figures swing (45.0 for Noise alone in round 2), most likely because of the other
agent's test sharing the card. They are not a reading of these effects.
