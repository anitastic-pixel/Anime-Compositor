# B-267..B-269: frame times with Page Turn, Power Pin and Ripple Pulse

**PROVISIONAL.** Measured on 2026-10-09 while another agent's two cargo processes were running
on the same machine (they were running before, between and after every round). In card round 2,
Power Pin came out at 44.2 ms against 17.6 in round 1, and the bump map's "first" at 53.3, with
nothing changed: the machine, not the effects. Expect these figures to move when measured
again on a quiet machine. The owner's app was not checked. The timing test is
`b267_distort_timing` in `tests/b267_page_turn_power_pin_ripple_pulse.rs`, built with
`cargo test --release --test b267_page_turn_power_pin_ripple_pulse` and run with
`--ignored b267_distort_timing` (`B267_CPU` set for the processor). These are single figures per
round, not medians of rounds.

All rounds are on the code commit (4a7f327). There is no "before" for the effects themselves,
which are new. Noise alone is the shot without them, timed in the same run.

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

| Shot | Card first, round 1 | Card again, round 1 | Card first, round 2 | Card again, round 2 | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 16.7 | 13.2 | 16.8 | 12.5 | 43.5 | 44.0 |
| Noise, then Page Turn as it starts (bottom right to 75, 75, radius 30) | 21.0 | 16.4 | 21.7 | 17.3 | 73.4 | 72.4 |
| Noise, then Power Pin, a keystone (25, 0 and 75, 0) | 20.4 | 17.6 | 48.7 | 44.2 | 62.3 | 63.7 |
| Noise, then Ripple Pulse, a moving level, time span 1 | 22.2 | 19.1 | 20.1 | 18.1 | 73.4 | 72.5 |
| Noise, then Ripple Pulse as a bump map | 22.2 | 17.6 | 53.3 | 17.3 | 64.5 | 65.2 |

**Reading it (provisional).** Per 1080p layer, from the "again" figures over three layers,
round 1 (round 2's Power Pin is unreadable):

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Page Turn | about 1.1 ms (16.4 against 13.2); 1.6 in round 2 | within Target P2's 4 ms | about 9.5 ms (72.4 against 44.0) |
| Power Pin | about 1.5 ms (17.6 against 13.2); round 2 unreadable | within P2 | about 6.6 ms |
| Ripple Pulse, moving level | about 2.0 ms (19.1 against 13.2); 1.9 in round 2 | within P2 | about 9.5 ms |
| Ripple Pulse, bump map | about 1.5 ms (17.6 against 13.2); 1.6 in round 2 | within P2 | about 7.1 ms |

Measure again on a quiet machine before relying on any figure here.
