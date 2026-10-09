# B-244..B-246: frame times with Broadcast Safe, Color Neutralizer and Color Offset

Measured on 2026-10-09 on the code of the code commit (ff3cba2), after it was committed, the
working copy holding exactly what was committed in the files the build reads. No other cargo or
rustc process was running and the owner's app was not open: checked before the first round and
after the last, none. The processor's load from background programs was not recorded. The timing
test is `b244_colour_timing` in `tests/b244_broadcast_neutralizer_offset.rs`, built with
`cargo test --release --test b244_broadcast_neutralizer_offset` and run with
`--ignored b244_colour_timing`: one round drawn by the card, then one round, with `B244_CPU` set,
by the processor. One round each, so these are single figures, not medians of rounds.

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

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 15.5 | 12.0 | 40.4 | 40.8 |
| Noise, then Broadcast Safe, NTSC, reduce luminance, 90 IRE | 18.7 | 14.2 | 51.7 | 52.0 |
| Noise, then Color Neutralizer with the three warm colours, pinning 30 | 18.4 | 14.2 | 52.2 | 52.8 |
| Noise, then Color Offset, red 45, green -135, blue 600, polarize | 18.8 | 14.5 | 51.8 | 52.0 |

**Reading it.** On the card each of the three costs about 0.7 to 0.8 ms a 1080p layer (14.2 to
14.5 against 12.0 ms a frame over three layers), the cost of one fused colour pass; on the
processor about 4 ms a layer (52.0 to 52.8 against 40.8). Within Target P1 on the card.
