# B-173: the card keeps the picture of each run, before and after

Measured on 2026-09-30 with nothing else running. Each build was tested from its own copy of the
source, and the builds took turns: one run of one build, then one of the other, the order swapped
each round. The tests are the units' own timing tests, run with
`cargo test --release --test <test> <name>_timing -- --ignored`:

- `b173_draft_moving`, B-173's own
- `b151_gpu_fx`
- `b152_card_whole_frame`
- `b155_gpu_chain`
- `b156_gpu_adjust`
- `b157_f64_audit`
- `b164_downsampled_blurs`
- `b172_fused`

Each figure is the median frame of a run in milliseconds. The table gives the median over the
rounds, and the individual runs are listed under each table.

**Machine:** AMD Ryzen 9 9900X (24 threads), 64 GB, NVIDIA GeForce RTX 4070 Ti SUPER, driver
610.88, Vulkan, Windows 11. **Build:** release. **Drawn on:** the graphics card unless a row says
CPU.

The build names used below:

- **Before B-173:** 898e398.
- **Before B-151:** a763, the build before B-151.
- **Before B-155:** 7639, the build before B-155.
- **B-173:** this build.

"First" is the first time through a loop of every eighth frame, when nothing is kept yet. "Again"
is the median of seven more times round, as B-151's timing takes them.

## 1. The rows the fix was for, against the build before the unit that slowed them

These are the rows the quiet re-measure of 2026-09-30 found slower, compared with the build before
the unit that slowed them. B-151 was timed over six rounds and B-155 over nine.

| Row | Before its unit | B-173 |
|---|---:|---:|
| B-155: the first two runs beginning with a moving Noise, Draft | 12.2 | **11.9** |
| B-155: the same, Full | 90.1 | **10.8** |
| B-155: the first two runs ending in a moving Noise, Draft | 13.0 | **11.9** |
| B-155: the same, Full | 15.4 | **10.9** |
| B-155: runs of four, three and three, Draft | 11.6 | **11.3** |
| B-155: the same, Full | 11.4 | **10.8** |
| B-151: Snowfall, Draft, again | 12.2 | **12.0** |
| B-151: Roughen Edges, Draft, again | 12.1 | **12.2** |
| B-151: Optics Compensation, Draft, again | 12.1 | **12.1** |
| B-151: Bevel Alpha, Full, again | 11.3 | **10.9** |
| B-151: Snowfall, Full, again | 62.4 | **17.9** |
| B-151: Roughen Edges, Full, again | 96.6 | **18.2** |

The individual runs, before its unit / B-173:

- B-155 moving Noise first, Draft: 11.7, 11.5, 11.6, 12.2, 11.8, 12.8, 12.7, 12.2, 12.3 / 11.8,
  11.6, 12.0, 11.7, 11.6, 12.7, 12.2, 11.9, 12.0.
- Snowfall Draft again: 11.8, 11.8, 12.0, 12.3, 12.4, 12.3 / 11.7, 11.5, 11.8, 12.3, 12.3, 12.2.
- Roughen Edges Draft again: 11.9, 12.0, 11.7, 12.1, 12.9, 12.4 / 11.7, 12.2, 11.8, 12.2, 12.4,
  12.4.
- Optics Compensation Draft again: 11.6, 11.7, 11.6, 12.4, 12.4, 12.5 / 13.1, 11.7, 11.9, 12.2,
  12.0, 12.1.
- Bevel Alpha Full again: 10.8, 11.0, 11.4, 11.5, 11.2, 11.6 / 10.7, 10.8, 10.5, 11.2, 11.3, 11.1.

Roughen Edges at Draft played again is 0.1 ms above the build before B-151. That is inside the
spread of either build's runs, 11.7 to 12.9.

Every other card row of B-151's table, first time and again, at Draft and Full, is equal (within
0.3 ms) or faster than before B-151. Corner Pin at Draft played again measured 11.9 before and 12.2
now, runs 11.6 to 12.4 and 11.7 to 12.5.

**Not B-173's, noted for later.** In the same rounds, B-151's CPU rows at Full measured 1.0 to 1.6
ms slower than before B-151. That includes the row with no effect at all (24.5 to 26.1 ms played
again). B-173 changes nothing the CPU draws, and against 898e398 those rows are equal. The change
lies somewhere between B-151 and 898e398 and has not been looked for.

## 2. B-173's own shots, before and after B-173

The reference shot's first three layers carry each effect the card draws that moves by itself, and
three keyed ones, all at Draft. Six rounds each.

| Shot, Draft | GPU first, before | B-173 | GPU again, before | B-173 |
|---|---:|---:|---:|---:|
| Noise | 14.1 | 14.9 | 12.3 | **11.8** |
| Exposure Flicker | 14.2 | 14.4 | 12.0 | **11.8** |
| Turbulent Displace | 16.5 | 15.1 | 12.8 | **11.6** |
| Fractal Noise | 16.2 | 15.7 | 13.3 | **11.7** |
| Ripple | 14.8 | 15.2 | 12.2 | **11.8** |
| Wave Warp | 15.2 | 14.8 | 12.1 | **11.6** |
| Speed Lines | 14.4 | 14.9 | 12.2 | **11.6** |
| Camera Shake | 14.6 | 16.0 | 11.9 | **11.8** |
| Rain | 14.9 | 15.6 | 12.1 | **11.8** |
| Snowfall | 15.7 | 16.2 | 13.2 | **12.1** |
| Roughen Edges | 14.6 | 16.1 | 12.9 | **12.1** |
| Kira-kira | 15.6 | 16.6 | 13.7 | **12.1** |
| Gaussian Blur, radius keyed 2 to 40 | 14.4 | 15.7 | 12.1 | **12.0** |
| Bloom, intensity keyed 0.5 to 3 | 23.1 | 23.9 | 18.3 | **12.3** |
| Directional Blur, length keyed 5 to 80 | 19.8 | 19.8 | 15.3 | **11.9** |
| B-155's moving Noise first | 16.1 | 16.4 | 12.9 | **11.9** |
| Median, which does not move (the control) | 13.2 | 13.6 | 11.6 | 11.6 |

**Played again is faster for every shot.** The CPU draws a frame played again in 12.2 to 13.0 ms,
so the card is now quicker than the CPU there too.

**The first time through costs a little more.** The card now keeps each frame's pictures, and
making room for them takes one more texture a frame. Measured across builds, the rise is up to
1.5 ms. That figure is larger than the true cost, because the first time through varies by up to
7 ms between runs of the same build (Ripple 14.0 to 21.7). A direct comparison inside one build,
with keeping switched on and off in turn over 4 to 8 runs, measured the cost itself:

| First time through, Draft | Without keeping | With keeping |
|---|---:|---:|
| Roughen Edges | 15.2, 14.8 | 15.9, 14.9 |
| Noise | 13.3, 13.5, 13.3 | 13.6, 13.4, 13.8 |
| Turbulent Displace | 13.2, 13.2, 13.4, 15.0 | 13.7, 13.7, 13.9, 15.0 |

So the cost is about 0.3 to 0.5 ms a frame, and only the first time through.

## 3. The guard: other rows that must not get slower

The guard covers the heavy rows the card helped with at Draft: B-164's big blurs, B-157's effects,
B-156b's motion blur and B-156's adjustment layers. Also covered are B-152, B-155 and B-172.
Before B-173 (898e398) and B-173, three rounds each unless noted.

| Row | Before B-173 | B-173 |
|---|---:|---:|
| B-152: reference shot with motion blur, Draft | 9.6 | 9.2 |
| B-152: the same, Full | 15.4 | 14.8 |
| B-152: with motion blur and Roughen Edges, Draft | 10.3 | 10.0 |
| B-152: the same, Full | 17.6 | **12.3** |
| B-152: with frame mix and dissolve, Draft | 29.5 | 29.5 |
| B-152: the same, Full | 43.4 | 44.4 |
| B-156: Levels, Gaussian Blur and Hue/Saturation over the whole frame, Draft | 15.1 | 14.7 |
| B-156: the same, Full | 24.6 | 22.6 |
| B-156: a moving Noise, then the same, Draft | 15.0 | 15.2 |
| B-156: the same, Full | 26.9 | 24.7 |
| B-172: Noise then eight colour effects, Draft | 14.1 | **10.9** |
| B-172: eight colour effects, the fifth changed each frame, Full | 16.5 | **2.4** |
| B-172: the same, Draft | 5.1 | **3.3** |

B-164's big blurs, nine rounds each:

| Row | Before B-173 | B-173 |
|---|---:|---:|
| Gaussian Blur 20, Draft | 7.3 | 7.4 |
| Gaussian Blur 50, Draft | 8.6 | 8.8 |
| Gaussian Blur 100, Draft | 8.5 | 8.7 |
| Gaussian Blur 200, Draft | 8.9 | 9.0 |
| Glow 100, Draft | 8.5 | 8.5 |
| Glow 200, Draft | 10.1 | 9.8 |
| Bloom 100, Draft | 9.7 | 9.3 |
| Bloom 200, Draft | 9.7 | 9.4 |
| Gaussian Blur 20, Full | 15.3 | 16.2 |
| Gaussian Blur 200, Full | 17.5 | 16.8 |
| Bloom 100, Full | 35.2 | 37.1 |
| Bloom 200, Full | 39.8 | 39.2 |

**B-157 and B-164's timings do not reach B-173's code.** Both tests clear the card before every
frame, which the program never does. A count added for one run found that no picture was kept
during either test, 0 of 0. There is no picture to keep because no frame comes round again. What
B-173 adds on their path is a search of an empty list, and the byte count of each drawing is the
same number as before, since its texture is always made at the drawing's own size.

Their rows therefore move with the machine, not with B-173. The Draft rows of B-164 above are all
within 0.2 ms or faster. At Full, Bloom 100 reads 1.9 ms slower while Bloom 200 reads 0.6 faster;
Gaussian Blur 20 reads 0.9 slower while Gaussian Blur 200 reads 0.7 faster.

B-157's table of 80 effects, six rounds each, is faster on 55 rows and slower on 8. The slower
ones:

- Color Lookup, Draft: 15.8 to 16.9. Runs 14.8, 15.7, 15.7, 16.0, 17.0, 15.9 / 17.1, 17.5, 15.7,
  16.6, 17.2, 16.0.
- HSV Key, Draft: 15.4 to 16.2.
- Line Blur and Motion Tile: by 0.4 to 0.6 ms once the frame without effects is taken away.

All of these lie within the spread of the runs.

B-155's "runs of four, three and three, Draft" measured 10.4 before B-173 and 11.0 after in these
three rounds. Over nine rounds against the build before B-155 (section 1) it is 11.6 before B-155
and 11.3 now.

B-152 "with frame mix and dissolve, Full" measured 43.4 before and 44.4 after. The runs were 41.7,
43.4 and 44.5 before, 44.8, 43.6 and 44.4 after. That frame is drawn by the processor (D-220).
