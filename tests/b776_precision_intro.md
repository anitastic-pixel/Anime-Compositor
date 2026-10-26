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
