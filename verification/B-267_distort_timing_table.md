# B-267..B-269: frame times with Page Turn, Power Pin and Ripple Pulse

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b267_page_turn_power_pin_ripple_pulse --no-run` and run with `--ignored b267_distort_timing`: once on the card, then once with `B267_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 2.9 GB of its memory held by the desktop and open programs); the same after each. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-09, on a machine that was not quiet), again: Noise alone: card, round 1 13.2, card, round 2 12.5, processor 44.0; Noise, then Page Turn as it starts (bottom right to 75, 75, radius 30): card, round 1 16.4, card, round 2 17.3, processor 72.4 ms.

There is no "before" for the effects themselves, which are new; Noise alone is the shot without them, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-264: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`; the test checks no frame
fell back to the processor) or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, from empty caches; **Again** the median of the loops after
it. Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.0 | 12.2 | 41.0 | 42.0 |
| Noise, then Page Turn as it starts (bottom right to 75, 75, radius 30) | 19.6 | 15.9 | 67.4 | 67.7 |
| Noise, then Power Pin, a keystone (25, 0 and 75, 0) | 18.7 | 15.7 | 65.8 | 66.6 |
| Noise, then Ripple Pulse, a moving level, time span 1 | 19.8 | 17.0 | 78.6 | 76.9 |
| Noise, then Ripple Pulse as a bump map | 18.6 | 15.8 | 68.2 | 66.8 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Page Turn | about 1.2 ms (15.9 against 12.2) | within Target P2's 4 ms | about 8.6 ms (67.7 against 42.0) |
| Power Pin | about 1.2 ms (15.7 against 12.2) | within Target P2's 4 ms | about 8.2 ms (66.6 against 42.0) |
| Ripple Pulse, moving level | about 1.6 ms (17.0 against 12.2) | within Target P2's 4 ms | about 11.6 ms (76.9 against 42.0) |
| Ripple Pulse, bump map | about 1.2 ms (15.8 against 12.2) | within Target P2's 4 ms | about 8.3 ms (66.8 against 42.0) |

The same as the busy run's round 1: all four within Target P2 on the card, at 1.2 to 1.6 ms a layer. Round 2's unreadable Power Pin (44.2) was the machine: on a quiet machine it is 15.7, as round 1 had it (17.6).
