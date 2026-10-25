# B-718 playtest: Radial Blur's zoom smears both ways about each pixel (D-838)

What changed: Radial Blur set to Zoom used to streak everything outward, away from the centre, a
streak as long as the Amount. After Effects' zoom is different: each point is smeared both ways
along the line from the centre, inward and outward, and the whole smear is half as long. Our zoom
now does the same. Spin is unchanged. Light Rays, Spin & Zoom Blur and Fast Zoom Blur are separate
effects and keep their outward streaks.

Projects saved before this change open with no warning and keep their settings. A Radial Blur zoom
in them now looks like After Effects': shorter, softer, centred streaks.

## What to look at

The pictures in `verification/B-718 pictures/` each show three panels side by side, from seven
tutorials by six creators:

1. After Effects' zoom, from the tutorial.
2. Our app before the change, from the same picture before the blur, at the same Amount.
3. Our app after the change, the same way.

The number on each panel is the error against After Effects (0 is identical; lower is better).
"No blur" is the error of the picture left unblurred. "Best fit" is the closest any centred smear
got in the earlier lanes' search.

| Picture | Amount | No blur | Best fit | Before | After |
|---|---|---|---|---|---|
| radial_mytips_zoom47 | 47 | 0.0395 | 0.0101 | 0.1089 (differs) | 0.0100 (matches) |
| radial_yaque_zoom30 | 30 | 0.0257 | 0.0045 | 0.0397 (differs) | 0.0036 (matches) |
| radial_wtfx_zoom36 | 36 | 0.0445 | 0.0062 | 0.0725 (differs) | 0.0063 (matches) |
| radial_wtfx_zoom85 | 85 | 0.0582 | 0.0096 | 0.1135 (differs) | 0.0095 (matches) |
| radial_justin_zoom21 | 21 | 0.0861 | 0.0126 | 0.0918 (differs) | 0.0127 (matches) |
| radial_esr_zoom10 | 10 | 0.0206 | 0.0048 | 0.0441 (differs) | 0.0052 (matches) |
| radial_ool_zoom13 | 13 | 0.0543 | 0.0120 | 0.0834 (differs) | 0.0144 (matches) |

Before, every case was further from After Effects than no blur at all. After, every case is within
1.2 times the best possible centred fit. The middle panel's long one-way streaks against the right
panel's short two-way ones are the change to look at.

## Try it in the app

1. Put a picture in a composition and add Blur & Sharpen > Radial Blur. Set Type to Zoom and Amount
   to 40.
2. The edges should smear softly both ways along lines through the centre, toward the centre as
   well as away from it. Before this change the streaks only went outward and were twice as long.
3. Set Type to Spin. It should look as it did before.
4. Add Light Rays to another picture. It should look as it did before.
5. Open a project saved before this change that uses a Radial Blur zoom. It opens with no warning,
   its Amount is unchanged, and its zoom now looks shorter and centred.

## Tests

- `tests/b718_radial_zoom.rs` checks Radial Blur's fixtures and FX-EDGES-008, that the zoom now
  reaches inward, that an old project opens with no warning, saves back unchanged and draws the new
  zoom, that Light Rays, Spin & Zoom Blur and Fast Zoom Blur did not move, and that the graphics
  card agrees with the processor. Its results are in `verification/B-718_radial_zoom_table.md`:
  212 of 212 checks pass.
- `tests/b39_radial_blur.rs`, `b46_gpu_radial.rs`, `b52_edges.rs` and `b52_gpu_edges.rs` rewrite
  their tables (and B-46's pictures) with the new zoom. `b240_spin_zoom_blur.rs` rewrites D-361's
  table with FX-SPINZOOM-001's new words; its numbers did not move.
