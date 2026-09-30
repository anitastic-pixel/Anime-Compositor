# B-172: runs of colour effects in one card pass, before and after

Measured on 2026-09-30 by `b172_timing` (in `tests/b172_fused.rs`, run with
`cargo test --release --test b172_fused b172_timing -- --ignored`), three runs before the build
(the source of the checks-first commit 4c35018) and three after. No other cargo process was running
at the start of the runs.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the graphics card.

The reference shot with the same stack on each of its first three layers. Each figure is the
median frame of a run, in milliseconds, and the median of the three runs.

- **Noise then eight colour effects, drawn each frame:** a Noise that changes every frame, then
  Levels, Hue/Saturation, Color Balance, Brightness & Contrast, Vibrance, Channel Mixer, Curves and
  Gradient Map. Every eighth frame, four times round, the first round not timed. The whole stack
  is drawn every frame, as in playback.
- **Eight colour effects, the fifth changed each frame:** the same eight without the Noise, frame
  100 drawn 100 times with the fifth (Vibrance) set to 40 and 55 in turn, the first 10 not timed:
  a slider dragged in the middle of a run.

| Case | Quality | Before (ms) | After (ms) | Card passes a frame, before | After |
|---|---|---:|---:|---:|---:|
| Noise then eight colour effects, drawn each frame | Full | 43.1 | **33.6** | 31 | 7 |
| Eight colour effects, the fifth changed each frame | Full | 14.1 | **13.1** | 16 | 7 |
| Noise then eight colour effects, drawn each frame | Draft | 14.5 | **12.9** | 31 | 7 |
| Eight colour effects, the fifth changed each frame | Draft | 5.8 | **4.9** | 16 | 7 |

The three runs: before 42.2, 43.7, 43.1 / 14.1, 14.2, 14.0 / 14.5, 15.2, 13.8 / 6.3, 5.8, 5.3;
after 33.6, 33.5, 33.6 / 13.1, 13.1, 13.1 / 12.9, 12.9, 12.9 / 4.7, 5.0, 4.9.

**A regression found and fixed on the way.** The first build drew a whole run again when one
effect in its middle changed, since it kept no picture inside a run: the second case measured
22.6 ms at Full (runs 22.6, 22.6, 22.6), against 14.1 before. The build now cuts a run where the
setting changed, keeps the part before it, and on the next change draws only the part after it in
one pass, which gives the 13.1 above.
