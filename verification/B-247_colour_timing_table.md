# B-247..B-249: frame times with Kernel, Toner and Change Color

Measured on 2026-10-09 on the code of the code commit (03cd7c1), after it was committed, the
working copy holding exactly what was committed in the files the build reads. No other cargo or
rustc process was running and the owner's app was not open: checked before each round and after
the last, none. The processor's total load was 6 to 7% just before the first round and under 1%
just after the processor round. The timing test is `b247_colour_timing` in
`tests/b247_kernel_toner_change_color.rs`, built with
`cargo test --release --test b247_kernel_toner_change_color` and run with
`--ignored b247_colour_timing`: one round drawn by the card, then one round, with `B247_CPU` set,
by the processor, then a second round by the card to check the Kernel figure. These are single
figures per round, not medians of rounds.

There is no "before" for the effects themselves, which are new; Noise alone is the shot without
them, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
the effect. So the effect has new input every frame and is worked every time. Every eighth frame
is asked for whole, as the viewer asks for it, at Full: with Draw on: GPU
(`preview_frame_srgb8`; no frame fell back to the processor, the test checks), or by the
processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30
frames, starting with empty caches. **Again** is the median of the loops after it: seven on the
card (210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again | Card first, round 2 | Card again, round 2 |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 24.5 | 14.1 | 42.9 | 43.8 | 16.9 | 13.1 |
| Noise, then Kernel, sharpen (0 -1 0, -1 5 -1, 0 -1 0) | 31.3 | 26.9 | 72.9 | 75.4 | 32.8 | 29.6 |
| Noise, then Toner, pentone purple and orange | 18.7 | 16.1 | 53.2 | 52.7 | 20.7 | 16.8 |
| Noise, then Change Color, red by hue turned 120, softness 8 | 20.3 | 16.8 | 50.9 | 51.2 | 20.4 | 15.3 |

**Reading it.** Toner and Change Color each cost about 0.7 to 1.2 ms a 1080p layer on the card
(15.3 to 16.8 against 13.1 to 14.1 ms a frame over three layers), the cost of one fused colour
pass, within Target P1; on the processor about 2.5 to 3 ms a layer. Kernel costs about 4.3 to
5.5 ms a layer on the card (26.9 and 29.6 against 14.1 and 13.1), just above Target P2's 4 ms,
and about 10.5 ms a layer on the processor (75.4 against 43.8). The first loop's Noise alone on
the card (24.5 in round 1) includes the card's start-up; round 2 gives 16.9.
