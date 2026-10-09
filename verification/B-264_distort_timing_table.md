# B-264..B-266: frame times with Flow Motion, Griddler and Fisheye

**PROVISIONAL.** Measured on 2026-10-09 on a machine that was not quiet for long. Each round was
started only when no other cargo, rustc or test binary was running, but another agent's
processes started during or straight after rounds 1, 3 and the processor round (its
`b260_before_timing` test binary, about 1.0 to 1.1 GB on the same card, and a rustc build), and
"Noise alone" moved from 11.8 to 21.7 ms between card rounds with nothing changed. Expect these
figures to move when measured again on a quiet machine. The owner's app was not checked. The
timing test is `b264_distort_timing` in `tests/b264_flow_motion_griddler_fisheye.rs`, built with
`cargo test --release --test b264_flow_motion_griddler_fisheye` and run with
`--ignored b264_distort_timing` (`B264_CPU` set for the processor). These are single figures per
round, not medians of rounds.

Card round 1 is the code commit (b9d1b6d). Rounds 2 and 3 and the processor round are on the
next commit (668d623), which only changes the card's Flow Motion: whole-number taps inside the buffer skip
a slow double-precision division, and the 1 / n steps are multiplied rather than divided (exact,
n being 1, 2 or 4); every picture in the D-385 table is byte-for-byte the same. There is no
"before" for the effects themselves, which are new. Noise alone is the shot without them, timed
in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-256: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`; the test checks no frame
fell back to the processor) or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, from empty caches; **Again** the median of the loops after
it, seven on the card, two on the processor. Milliseconds a frame.

| Shot | Card first, round 1 | Card again, round 1 | Card first, round 2 | Card again, round 2 | Card first, round 3 | Card again, round 3 | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Noise alone | 20.6 | 14.8 | 24.8 | 21.7 | 15.0 | 11.8 | 58.7 | 61.0 |
| Noise, then Flow Motion as it starts (10 and -10, low) | 35.0 | 29.0 | 41.4 | 32.1 | 21.3 | 17.1 | 104.0 | 109.5 |
| Noise, then Flow Motion with high antialiasing | 129.6 | 117.8 | 81.9 | 77.5 | 70.6 | 68.8 | 290.4 | 296.4 |
| Noise, then Griddler as it starts (tiles of 10, 80 per cent) | 19.6 | 16.3 | 28.6 | 15.9 | 25.4 | 20.2 | 109.5 | 111.4 |
| Noise, then Fisheye as it starts (size 50, convergence 50) | 21.0 | 20.3 | 19.1 | 15.9 | 21.0 | 17.0 | 68.2 | 96.3 |

**Reading it (provisional).** Per 1080p layer, from the "again" figures over three layers,
round 3 (the lowest Noise alone):

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Flow Motion, Low | about 1.8 ms (17.1 against 11.8); 4.7 in round 1 before the change, 3.5 in round 2 | within Target P2's 4 ms | about 16 ms (109.5 against 61.0) |
| Flow Motion, High | about 19 ms (68.8 against 11.8); 34 in round 1 before the change | over P2: sixteen points a pixel, an option the user chooses | about 78 ms |
| Griddler | about 2.8 ms (20.2 against 11.8); round 2 below its own Noise alone, unreadable | within P2 | about 17 ms |
| Fisheye | about 1.7 ms (17.0 against 11.8) | within P2 | between 3 and 12 ms (the two processor loops disagree) |

Round 2's Griddler and Fisheye came out below that round's Noise alone, and the processor's
Fisheye "first" and "again" disagree by 28 ms: the machine, not the effects. Measure again on a
quiet machine before relying on any figure here.
