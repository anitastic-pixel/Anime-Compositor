# B-243: frame times with Diffusion's second pass

Measured on 2026-10-08 on the code of the code commit (4ea3ae2), after it was committed and
pushed, the working copy holding exactly what was committed in the files the build reads. No
other cargo or rustc process was running and the owner's app was not open: checked before the
first round and after each one, none. The processor's load from background programs was not
recorded. The timing test is `b243_diffusion_second_timing` in `tests/b243_diffusion_second.rs`,
built with `cargo test --release --test b243_diffusion_second` and run with
`--ignored b243_diffusion_second_timing`: one round drawn by the card, then one round, with
`B243_CPU` set, by the processor. One round each, so these are single figures, not medians of
rounds.

The "before" is Diffusion with the second pass off, exactly as before D-364, timed in the same
run; Noise alone is the shot without Diffusion.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
Diffusion radius 30, amount 50, lighten. So the effect has new input every frame and is worked
every time. Every eighth frame is asked for whole, as the viewer asks for it, at Full: with Draw
on: GPU (`preview_frame_srgb8`; no frame fell back to the processor, the test checks), or by the
processor alone (`preview_frame_cached`). **First** is the median of the first loop of 30 frames,
starting with empty caches. **Again** is the median of the loops after it: seven on the card
(210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.2 | 12.4 | 40.0 | 40.1 |
| Noise, then Diffusion radius 30, lighten 50 (second pass off, as before D-364) | 20.9 | 16.7 | 80.2 | 80.4 |
| Noise, then Diffusion radius 30, lighten 50, soft light 50 | 22.5 | 19.7 | 94.3 | 94.9 |
| Noise, then Diffusion radius 30, lighten 50, overlay 50 | 22.5 | 19.6 | 93.2 | 94.4 |

**Reading it.** On the card Diffusion without the second pass costs about 1.4 ms a 1080p layer;
the second pass adds about 1 ms a layer (19.7 against 16.7 ms a frame over three layers), soft
light and overlay alike, as it shares the blur and only adds per-pixel work. On the processor the
second pass adds about 5 ms a layer (94.9 against 80.4). Within Target P2 on the card.
