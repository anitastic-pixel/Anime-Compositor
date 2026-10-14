# B-256..B-258: frame times with Bend It, Bender and Blobbylize

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b256_bend_it_bender_blobbylize --no-run` and run with `--ignored b256_distort_timing`: once on the card, then once with `B256_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 4% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone: card 12.9, processor 42.2, card, round 2 13.2; Noise, then Bend It as it starts (45, up the middle): card 21.0, processor 73.2, card, round 2 22.6 ms.

There is no "before" for the effects themselves, which are new; Noise alone is the shot without them, timed in the same run.

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

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.0 | 12.5 | 41.3 | 41.7 |
| Noise, then Bend It as it starts (45, up the middle) | 24.6 | 20.2 | 71.5 | 71.3 |
| Noise, then Bender as it starts (bend 20, up the middle) | 18.7 | 15.2 | 60.6 | 61.1 |
| Noise, then Blobbylize as it starts (its own alpha, softness 10) | 22.4 | 19.3 | 131.5 | 131.9 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Bend It | about 2.6 ms (20.2 against 12.5) | within Target P2's 4 ms | about 9.9 ms (71.3 against 41.7) |
| Bender | about 0.9 ms (15.2 against 12.5) | within Target P2's 4 ms | about 6.5 ms (61.1 against 41.7) |
| Blobbylize | about 2.3 ms (19.3 against 12.5) | within Target P3's 8 ms | about 30.1 ms (131.9 against 41.7) |

The same as the busy run's reading: each effect within its target on the card, and the card several times quicker than the processor. The busy run's swinging "first" figures (45.0 for Noise alone in its round 2) are gone: on a quiet machine the first loop is 3 to 4 ms over the loops after it, as for other effects.
