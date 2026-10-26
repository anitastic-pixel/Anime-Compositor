# B-776: the precision rule (D-896) on the processor's effects

Written by `tests/b776_precision.rs`. The rule, D-896: single precision by default; double
precision only where it is proven needed; never half precision for the maths. An effect's maths
moves to single precision only when every fixture still passes (2e-5), the floats stay within
1e-4 of before, the processor's 8-bit picture on the reference shot is identical, and it is
measurably quicker.

## Converted

**Bilateral Blur** (`src/median.rs`, `bilateral_blur`), and so **Cartoon**, whose smoothing it is
(and Bilateral Blur's other caller in `src/effects.rs`): each tap's weight (the distance falloff
and the level falloff, an `exp` per tap and channel) is worked in single precision; the colours
and their running sums stay double.

Time of one frame at 1920 by 1080, the processor at Full (what an export takes), the reference
shot with the effect on layer 2, median ms of frames 0 to 210 by 30, from
`perf_audit::perf_audit_timing`. Before is the base build (ec3f3c04), after is this build; five
rounds alternating before and after, the median of the rounds' medians. Two separate sittings,
both on a machine shared with other lanes' builds (AMD64 Family 26 Model 68, 24 threads; RTX
4070 Ti SUPER; release build):

| Effect | Setting | Sitting 1 before | Sitting 1 after | Sitting 2 before | Sitting 2 after |
|---|---|---:|---:|---:|---:|
| (none, for comparison) | | 16.6 | 16.9 | 18.1 | 22.0 |
| Bilateral Blur | as added (Radius 5) | 41.3 | 37.8 | 53.8 | 48.6 |
| Bilateral Blur | Radius 40 | 276.4 | 234.7 | 307.5 | 273.3 |
| Cartoon | as added | 65.4 | 60.4 | 70.1 | 71.1 |
| Cartoon | Detail Radius 40, Threshold 20, 16 steps | 288.0 | 255.6 | 354.2 | 295.6 |

The processor's Draft rows (5 to 10 ms) moved by at most 1 ms in sitting 2. The card draws these
effects in the viewer and is untouched. Cartoon as added gained in sitting 1 and not in sitting 2.

The processor's picture of frame 30 at Full (`perf_audit::perf_audit_pixels`, this build
against the base build's): 0 pixels changed in 8 bits for all four rows; the largest float
change 1.2e-7, on flat areas 6.0e-8.

## Tried and not converted

**Deband** (`src/grade.rs`, its edge-stopping blur). (a) The blur's weights, values and sums in
single precision: Full 112.7 to 71.2 ms as added and 136.0 to 96.7 ms at Edge Softness 40, Blur
40 (five alternating rounds), every fixture within 2.5e-7, but 3 and 4 pixels of the 2,073,600
on the reference shot came out 1 level of 255 apart, so the processor's 8-bit picture is not
identical: not built (the owner's call, see D-896). (b) Only the weights in single precision,
the values and sums double: identical in 8 bits, but 88.7 to 85.2 and 142.9 to 126.5 ms, within
the spread between rounds (the no-effect row moved 20.3 to 17.6): not built.

## Kept in double precision, and why

| Effect | Full ms (base) | Why it stays double |
|---|---:|---|
| Refine Soft Matte, Refine Hard Matte, Key Cleaner (`src/matte_refine.rs`) | 216 to 430 | The guided filter's running totals over whole rows and columns (up to about 157,000 at radius 40) and then E[I*I] - E[I]*E[I] against an epsilon of 1e-4: in single precision the means are off by about 2e-6, a percent-sized error in the filter's slope. |
| Matte Choker (`choker_stage`) | 62 to 213 | Running totals, then a division by a covering that can be very small (D-228's reason for Simple Choker). |
| Contour Paint (`src/layer_fx.rs`, smear and way) | 106 to 205 | Positions in picture coordinates walked step by step, up to 1920 px, with a stop test at 1e-6: in single precision a position is only good to about 1e-4 px and drifts each step (D-228's position rule). |
| Caustics (`src/layer_fx.rs`) | 90 to 127 | The wave heights' blur feeds slopes, then offsets times depth and a division by a determinant: positions again. The bottom picture's blur is a candidate (about 37 ms of the 127 at Bottom Blur 40) but shares its generic FFT with the heights; not tried. |
| Halation, Soft Glow (`src/soft_glow.rs`) | 119 to 134 | Already single precision; only a cheap averaging step is double. |
| Fast Zoom Blur, Radial Blur, Light Burst | 88 to 106 | Samples read at worked-out positions along rays (position rule); not tried in this lane. |
| Everything else | under 90 | Not tried in this lane. |

## Bilateral Blur's fixtures (FX-BILAT), its weights in single precision

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BILAT-001 frame 0: Bilateral Blur as it starts, Radius 5, Threshold 20, Colorize on: the grain, 8 levels either side of the skin, is smoothed toward the skin, while the line and the dark specks, far more than 20 from the skin, stay as they are to well within a level; the white speck, 41 and 65 from the skin in green and blue, stays far lighter than the skin. The hole stays a hole and the half-covered column keeps its half covering. | largest difference 2.1e-7 | yes |
| FX-BILAT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BILAT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-003 frame 0: Threshold 0: each pixel counts only itself, so the drawing is unchanged. | largest difference 1.9e-7 | yes |
| FX-BILAT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-004 frame 0: Colorize off: the same smoothing worked on luminance alone, and every pixel that shows is grey, red, green and blue equal; the line stays dark and the outline is kept. | largest difference 1.4e-7 | yes |
| FX-BILAT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-005 frame 0: Colorize off with Radius 0: no smoothing, each pixel only its own luminance, grey. | largest difference 1.1e-7 | yes |
| FX-BILAT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-006 frame 0: Threshold 255: likeness hardly matters, a soft blur that stays inside the drawing: the line and the specks are blurred into the skin, and the hole, the empty column and the empty row stay empty. | largest difference 2.1e-7 | yes |
| FX-BILAT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-007 frame 0: Radius 1.5, a disc of nine: a lighter smoothing of the grain than FX-BILAT-001. | largest difference 2.0e-7 | yes |
| FX-BILAT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-008 frame 0: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 1.9e-7 | yes |
| FX-BILAT-008 frame 2: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-008 frame 4: Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-009 frame 0: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 1.9e-7 | yes |
| FX-BILAT-009 frame 2: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-009 frame 4: Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-BILAT-001. | largest difference 2.1e-7 | yes |
| FX-BILAT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-010 frame 0: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-BILAT-010 frame 2: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-BILAT-010 frame 4: Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end (about 64 at frame 2): frame 2 is held at 50, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-BILAT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-011 frame 0: Bilateral Blur as it starts, the layer moved three pixels right: FX-BILAT-001 moved with it. | largest difference 2.1e-7 | yes |
| FX-BILAT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-012 frame 0: Bilateral Blur, Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-012 frame 4: Bilateral Blur, Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-013 frame 0: Bilateral Blur, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-013 frame 4: Bilateral Blur, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-014 frame 0: Bilateral Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-014 frame 4: Bilateral Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-015 frame 0: Bilateral Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-015 frame 4: Bilateral Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-016 frame 0: Bilateral Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-016 frame 4: Bilateral Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BILAT-017 frame 0: Bilateral Blur, Colorize "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-017 frame 4: Bilateral Blur, Colorize "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Cartoon's fixtures (FX-CARTOON), its smoothing is Bilateral Blur's

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CARTOON-001 frame 0: The settings as they start: Fill & Edges, Detail Radius 8, Threshold 10, 8 shading steps at smoothness 40, edges at threshold 1.6, width 1.5, softness 60: the grain smoothed into the skin, the skin's shading cut into steps, and dark lines along the line, the specks, the hole and the drawing's outline. | largest difference 6.7e-8 | yes |
| FX-CARTOON-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-002 frame 0: Render Fill: the smoothed, stepped colours, no lines. | largest difference 1.6e-7 | yes |
| FX-CARTOON-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-003 frame 0: Render Edges: the lines alone, black on white, at the drawing's own covering. | largest difference 6.0e-8 | yes |
| FX-CARTOON-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-004 frame 0: Detail Radius 0: nothing smoothed, so the grain keeps its edges. | largest difference 4.6e-8 | yes |
| FX-CARTOON-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-005 frame 0: Detail Threshold 255: the smoothing hardly minds the colours, a soft blur that softens the lines. | largest difference 3.6e-7 | yes |
| FX-CARTOON-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-006 frame 0: Render Fill, 2 shading steps, smoothness 0: every pixel's shade pushed to black or white. | largest difference 1.6e-7 | yes |
| FX-CARTOON-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-007 frame 0: Render Fill, Detail Radius 0, smoothness 100: the drawing as it was. | largest difference 1.9e-7 | yes |
| FX-CARTOON-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-008 frame 0: Edge Threshold 10: only the strongest edges are drawn. | largest difference 1.3e-7 | yes |
| FX-CARTOON-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-009 frame 0: Edge Width 0, softness 0: lines one pixel wide, on the edges alone. | largest difference 1.6e-7 | yes |
| FX-CARTOON-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-010 frame 0: Edge Width 4: thick lines. | largest difference 3.0e-8 | yes |
| FX-CARTOON-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-011 frame 0: Edge Softness 0: the lines' sides hard. | largest difference 9.8e-8 | yes |
| FX-CARTOON-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-012 frame 0: Edge Opacity 0: no lines, the same as Render Fill. | largest difference 1.6e-7 | yes |
| FX-CARTOON-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-013 frame 0: Edge Opacity 50: the lines at half strength. | largest difference 8.7e-8 | yes |
| FX-CARTOON-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-014 frame 0: Render Fill, Edge Enhancement 100: the colours either side of an edge pushed apart. | largest difference 1.9e-7 | yes |
| FX-CARTOON-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-015 frame 0: Render Fill, Edge Enhancement -100: the colours either side of an edge drawn together. | largest difference 1.6e-7 | yes |
| FX-CARTOON-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-016 frame 0: Edge Black Level 0.5: grey lines. | largest difference 6.3e-8 | yes |
| FX-CARTOON-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-017 frame 0: Render Edges, Edge Black Level 1: white lines on black. | largest difference 2.4e-7 | yes |
| FX-CARTOON-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-018 frame 0: Edge Contrast 1: every edge past the threshold drawn fully. | largest difference 6.7e-8 | yes |
| FX-CARTOON-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-019 frame 0: Edge Contrast 0: the lines fade in slowly with the edge. | largest difference 6.7e-8 | yes |
| FX-CARTOON-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-020 frame 0: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 1.8e-7 | yes |
| FX-CARTOON-020 frame 2: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 3.0e-8 | yes |
| FX-CARTOON-020 frame 4: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 3.0e-8 | yes |
| FX-CARTOON-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-021 frame 0: FX-CARTOON-001 moved three pixels right. | largest difference 6.7e-8 | yes |
| FX-CARTOON-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-022 frame 0: After a Motion Tile that grows the layer, unmirrored: the edges are the grown buffer's, so the tile's empty row and column now meet the top row and the left column, and lines run along the top and the left. | largest difference 4.9e-8 | yes |
| FX-CARTOON-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-023 frame 0: Detail Radius 3, Threshold 30, 5 steps at smoothness 20, edges at threshold 2.5, width 2.5, softness 30, opacity 80, enhancement 40, black level 0.2, contrast 0.7: the controls together. | largest difference 2.5e-7 | yes |
| FX-CARTOON-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-024 frame 0: Render "outline", not one of fill, edges or fill_and_edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-024 frame 4: Render "outline", not one of fill, edges or fill_and_edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-025 frame 0: Detail Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-025 frame 4: Detail Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-026 frame 0: Detail Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-026 frame 4: Detail Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-027 frame 0: Shading Steps 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-027 frame 4: Shading Steps 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-028 frame 0: Shading Steps 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-028 frame 4: Shading Steps 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-029 frame 0: Edge Width 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-029 frame 4: Edge Width 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-030 frame 0: Edge Enhancement -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-030 frame 4: Edge Enhancement -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-031 frame 0: Edge Black Level 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-031 frame 4: Edge Black Level 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-032 frame 0: Edge Contrast keyed to 2 at frame 4, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-032 frame 4: Edge Contrast keyed to 2 at frame 4, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Result

121 of 121 checks pass.
