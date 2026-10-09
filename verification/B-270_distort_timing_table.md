# B-270..B-272: frame times with Slant, Smear and Split

**PROVISIONAL.** Measured on 2026-10-09. No cargo or rustc process was running during any round
and the owner's app was not running, but another agent had been compiling just before, and the
card figures are within their own noise: in both card rounds Smear came out faster than Noise
alone, which is impossible for an effect that adds work, and Noise alone itself moved from 23.5
to 19.4 ms between rounds with nothing changed. The card cost of these three effects is too
small to read from these runs. Expect the figures to move when measured again. The timing test
is `b270_distort_timing` in `tests/b270_slant_smear_split.rs`, built with
`cargo test --release --test b270_slant_smear_split` and run with `--ignored b270_distort_timing`
(`B270_CPU` set for the processor). These are single figures per round, not medians of rounds.

All rounds are on the code commit (c704ad5). There is no "before" for the effects themselves,
which are new. Noise alone is the shot without them, timed in the same run.

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

| Shot | Card first, round 1 | Card again, round 1 | Card first, round 2 | Card again, round 2 | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 30.2 | 23.5 | 23.6 | 19.4 | 45.8 | 44.4 |
| Noise, then Slant, leaning 30, stretched | 28.0 | 23.0 | 30.0 | 27.0 | 89.5 | 94.9 |
| Noise, then Smear, from 30, 40 to 60, 60, reach 300, radius 200 | 19.3 | 16.0 | 31.0 | 17.6 | 105.3 | 90.5 |
| Noise, then Split, a diagonal (10, 10 to 90, 90), split 200 | 20.2 | 18.0 | 26.0 | 20.1 | 75.0 | 77.6 |

**Reading it (provisional).** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Slant | unreadable; at most about 2.5 ms (27.0 against 19.4 in round 2), nothing in round 1 | within Target P2's 4 ms | about 16.8 ms (94.9 against 44.4) |
| Smear | unreadable; below Noise alone in both rounds | within P2 | about 15.4 ms |
| Split | unreadable; about 0.2 ms in round 2, below Noise alone in round 1 | within P2 | about 11.1 ms |

Measure again on a quiet machine before relying on any figure here.
