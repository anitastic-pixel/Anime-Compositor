# B-270..B-272: frame times with Slant, Smear and Split

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b270_slant_smear_split --no-run` and run with `--ignored b270_distort_timing`: once on the card, then once with `B270_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 1% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone: card, round 1 23.5, card, round 2 19.4, processor 44.4; Noise, then Slant, leaning 30, stretched: card, round 1 23.0, card, round 2 27.0, processor 94.9 ms.

There is no "before" for the effects themselves, which are new; Noise alone is the shot without them, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-267: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`; the test checks no frame
fell back to the processor) or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop, from empty caches; **Again** the median of the loops after it (8 loops
on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 15.7 | 12.2 | 41.8 | 42.2 |
| Noise, then Slant, leaning 30, stretched | 18.5 | 14.6 | 60.3 | 60.6 |
| Noise, then Smear, reach 300, radius 200 | 18.7 | 15.6 | 60.4 | 60.8 |
| Noise, then Split, a diagonal, split 200 | 19.5 | 16.4 | 67.8 | 68.0 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Slant | about 0.8 ms (14.6 against 12.2) | within Target P2's 4 ms | about 6.1 ms (60.6 against 42.2) |
| Smear | about 1.1 ms (15.6 against 12.2) | within Target P2's 4 ms | about 6.2 ms (60.8 against 42.2) |
| Split | about 1.4 ms (16.4 against 12.2) | within Target P2's 4 ms | about 8.6 ms (68.0 against 42.2) |

This changes the busy run's reading, which could not read the card at all (Smear and Split came out below the Noise alone): on a quiet machine Slant, Smear and Split cost about 0.8, 1.1 and 1.4 ms a 1080p layer on the card, within Target P2, and about 6 to 9 ms on the processor, under the busy run's 11 to 17.
