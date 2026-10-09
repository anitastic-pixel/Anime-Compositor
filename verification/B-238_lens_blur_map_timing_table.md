# B-238: frame times with Lens Blur's blur map

Measured on 2026-10-08 on the code of the code commit (cdabf63), before it was committed, the
working copy holding exactly what was committed. No other cargo or rustc process was running and
the owner's app was not open: checked before each set of rounds and after the last, none. The
processor's load from background programs was not recorded. The timing test is
`b238_lens_blur_map_timing` in `tests/b238_lens_blur_map.rs`, built with
`cargo test --release --test b238_lens_blur_map` and run with `--ignored b238_lens_blur_map_timing`:
one round drawn by the card, then one round, with `B238_CPU` set, by the processor. One round
each, so these are single figures, not medians of rounds.

The blur map is new, so the "before" is the same Lens Blur with no map, timed in the same run.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) has a Noise that changes every frame on each of its first three layers, then
Lens Blur (edges transparent). So the effect has new input every frame and is worked every time.
The map is layer 4 of the shot, read by its Luminance, Centre, Blur Focal Distance 0. Every eighth
frame is asked for whole, as the viewer asks for it, at Full: with Draw on: GPU
(`preview_frame_srgb8`), or by the processor alone (`preview_frame_cached`). **First** is the
median of the first loop of 30 frames, starting with empty caches. **Again** is the median of the
loops after it: seven on the card (210 frames), two on the processor (60 frames). Milliseconds a
frame.

| Shot | Card first | Card again | Processor first | Processor again |
|---|---:|---:|---:|---:|
| Noise alone | 16.0 | 11.9 | 40.4 | 41.4 |
| Noise, then Lens Blur radius 10, no map (before) | 38.4 | 36.0 | 89.0 | 80.8 |
| Noise, then Lens Blur radius 10, layer 4 as the blur map | 53.1 | 50.8 | 85.6 | 86.0 |
| Noise, then Lens Blur radius 20, layer 4 as the blur map | 53.2 | 51.6 | 87.1 | 87.7 |

**Reading it.** On the card the map adds about 15 ms a frame over the plain blur at radius 10,
about 5 ms a 1080p layer: each pixel sums two iris sizes instead of one and the map is read in.
At radius 20 it measured about the same; why was not looked into. The processor's time hardly changes with the map. Lens Blur, map or not, is
above Target P3 on the card at these sizes, as B-65 already found for the plain blur.
