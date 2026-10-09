# B-253..B-255: frame times with Color Balance (HLS), Color Link and Color Stabilizer

Measured on 2026-10-09 on the working copy just before the code commit (42e982c), holding exactly
what was then committed in the files the build reads. No other cargo or rustc process was running
and the owner's app was not open: checked before each round and after the last, none. The
processor's load was not recorded. The timing test is `b253_colour_timing` in
`tests/b253_color_balance_link_stabilizer.rs`, built with
`cargo test --release --test b253_color_balance_link_stabilizer` and run with
`--ignored b253_colour_timing`: one round drawn by the card, then one round, with `B253_CPU` set,
by the processor, then a second round by the card. These are single figures per round, not
medians of rounds. The raw output of each round is in `verification/B-253_timing_raw.md`,
`B-253_timing_raw_cpu.md` and `B-253_timing_raw_card2.md`.

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
(`preview_frame_srgb8`; this timing test does not itself check where each frame was drawn), or
by the processor alone (`preview_frame_cached`). **First** is the median of the first loop of
30 frames, starting with empty caches. **Again** is the median of the loops after it: seven on the
card (210 frames), two on the processor (60 frames). Milliseconds a frame.

| Shot | Card first | Card again | Processor first | Processor again | Card first, round 2 | Card again, round 2 |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 16.5 | 11.5 | 40.8 | 41.3 | 14.9 | 11.4 |
| Noise, then Color Balance (HLS), hue 120, saturation 30 | 19.3 | 15.1 | 61.7 | 62.1 | 18.8 | 14.3 |
| Noise, then Color Link, layer 4's average, Overlay at 50 | 35.1 | 38.0 | 85.9 | 86.0 | 34.3 | 36.9 |
| Noise, then Color Link, its own average, Overlay at 50 | 78.6 | 83.4 | 80.3 | 80.4 | 75.8 | 86.0 |
| Noise, then Color Stabilizer, Levels to frame 0 | 73.7 | 58.5 | 52.0 | 52.5 | 52.7 | 58.6 |

**Reading it.**

- **Color Balance (HLS)** costs about 1.0 to 1.2 ms a 1080p layer on the card (14.3 to 15.1
  against 11.4 to 11.5 ms a frame over three layers), one fused colour pass, within Target P1; on
  the processor about 7 ms a layer (62.1 against 41.3).
- **Color Link with another layer** costs about 8.5 ms a layer with the card (36.9 to 38.0
  against 11.4 to 11.5), over Target P2: the card lays the colour on, but the source layer's
  statistics are read by the processor at every frame. On the processor alone about 15 ms a
  layer (86.0 against 41.3).
- **Color Link on the layer's own picture** is drawn by the processor from Color Link on, so with
  the card the frame takes about what the processor alone takes (83.4 and 86.0 against 80.4),
  about 13 ms a layer over the processor's Noise alone.
- **Color Stabilizer** is drawn by the processor (it reads another frame of the layer): about
  3.7 ms a layer there (52.5 against 41.3), within Target P4. With the card the frame is slower
  than the processor alone (58.5 and 58.6 against 52.5), because each layer leaves the card at
  the stabilizer; logged as a gap.
- First loops vary from round to round (the stabilizer's 73.7 in round 1, 52.7 in round 2);
  Again is the steady figure.
