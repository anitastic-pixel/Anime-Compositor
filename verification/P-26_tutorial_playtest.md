# P-26: playing five After Effects tutorials through the app

Asked on 2026-10-03: follow five After Effects tutorials step by step in the app, find what does not work the way After Effects does, and fix it.

The five tutorials:

1. Video Copilot, Shockwave (Fractal Noise band bent into a ring with Polar Coordinates)
2. Video Copilot, Advanced Electric (a lightning strike on a street)
3. Video Copilot, Colorful Glitch (a displacement-map title wipe)
4. Chris Connor, Ultimate Lightsaber
5. Adobe / Film Riot, Lightsaber (a glow on a practical prop)

Every step was tried in a test copy of the app, never the owner's own. The step-by-step notes and stills stay out of the repository, because they hold frames from the tutorial videos.

## Before: how each tutorial went

A step counts as "not passing" when the app could not do it, or could only get part of the way.

| Tutorial | Steps | Passed | Not passing | Now fixed | Now partly fixed | Still open |
|---|---|---|---|---|---|---|
| 1 Shockwave | 24 (+2 left out by the charter) | 14 | 10 | 5 | 1 | 4 |
| 2 Advanced Electric | 18 | 5 | 13 | 7 | 2 | 4 |
| 3 Colorful Glitch | 29 | 12 | 17 | 8 | 3 | 6 |
| 4 Ultimate Lightsaber | 22 | 15 | 7 | 6 | 0 | 1 |
| 5 Film Riot Lightsaber | 12 | 7 | 5 | 5 | 0 | 0 |
| **All** | **105** | **53** | **52** | **31** | **6** | **15** |

The tutorials were **not** played through again from start to end after the fixes. Each fix was checked on its own, by its tests and its pictures (below). "Now fixed" means the step's missing piece was built and checked; the owner's playtest is still the judge.

The 15 still open, and what holds each:

- Polar Coordinates makes an ellipse, not After Effects' circle (tutorial 1): D-308, changes existing pictures.
- Fractal Noise Complexity 10 is refused, the cap is 8 (1): D-308.
- Time Remapping (1): D-308.
- Bulge with separate across and down radii (1): D-309.
- Advanced Lightning's Alpha Obstacle (2): D-308.
- Add without a ceiling, and Exposure past 20 (2, two steps): D-308.
- Glow at the tutorial's values is much weaker (2 and 3): D-308.
- The final look of tutorials 2, 3 and 4: held by the items above, mainly Add without a ceiling and Glow strength.
- Displacement Map's Expand Output (3): D-309.
- 32-bit values past white (3): D-308.
- Colorama (3): D-309.
- The two-node camera and 3D layers (3): left out by the charter.

Partly fixed: Fractal Noise's Dynamic Progressive type (Invert and Offset are built), Fractal Noise Brightness -154 (Scale Height is built, the range stops at -100), Shift Channels (built) with Solid Composite (not yet), Fast Box Blur with Exposure (the blur is built, Add's ceiling is not), Freeze Frame with a luma matte (the matte is built, Freeze Frame is not), and the two Glow-and-Fractal steps of tutorial 3.

## What was built

Every one is **built and awaiting the owner's playtest**. Each starts where the app was before, so older projects and every fixture draw as they did. Each has its own page in `verification/`, with what to look for and how to try it.

| Decision | What it does | Fixes | Page | Pictures |
|---|---|---|---|---|
| D-290 | A frame rate of 23.976 can be typed | every tutorial | `D-290_frame_rate_23976.md` | none, checked by tests |
| D-291 | Expressions on effect settings: `time*150`, `wiggle(5, 20)` | 1, 2, 4, 5 | `D-291_effect_expressions.md` | `D-291 pictures/` |
| D-292 | A footage layer shows its pictures at once, without filling its sheet by hand | 4, 5 | `D-292_sequence_exposed_on_ones.md` | none |
| D-293 | Track mattes: alpha inverted, luma and luma inverted | 2, 3 | `D-293_matte_modes.md` | none |
| D-294 | A mask key whose outline crosses itself is refused, as Draw already did | 4 | `D-294_mask_key_crossing.md` | none |
| D-295 | Color Balance's Preserve Luminosity | 5 | `D-295_color_balance_preserve_luminosity.md` | `D-295 pictures/` |
| D-296 | New solid asks for its name, size and colour first | every tutorial | `D-296_new_solid_settings.md` | `D-296 pictures/` |
| D-297 | An adjustment layer takes a blend mode | 3 | `D-297_adjustment_layer_blend_mode.md` | none |
| D-298 | A mask's feather, opacity and expansion take keys | 1 | `D-298_mask_number_keys.md` | `D-298 pictures/` |
| D-299 | Fractal Noise: Turbulent and Block, Invert, Offset, Scale Width and Height, Cycle Evolution | 1, 3 | `D-299_fractal_noise_look.md` | `D-299 pictures/` |
| D-300 | Lightning's Composite on Original, off by default as in After Effects | 2 | `D-300_lightning_composite.md` | `D-300 pictures/` |
| D-301 | Overlay, Soft Light, Stencil Alpha and Stencil Luma blend modes | 2, 3 | `D-301_blend_modes.md` | `D-301 pictures/` |
| D-302 | Curves on the alpha channel | 3 | `D-302_curves_alpha.md` | `D-302 pictures/` |
| D-303 | Gaussian Blur's Blur Dimensions: blur across or down only | 2, 3 | `D-303_blur_dimensions.md` | `D-303 pictures/` |
| D-304 | Motion Tile's Tile Center, Width and Height | 3 | `D-304_motion_tile_size.md` | `D-304 pictures/` |
| D-305 | Shift Channels | 3 | `D-305_shift_channels.md` | `D-305 pictures/` |
| D-306 | Turbulent Displace's Displacement types and Pinning | 2 | `D-306_turbulent_displace_ways.md` | `D-306 pictures/` |
| D-307 | Hue/Saturation's colour ranges (Reds, Yellows ... Magentas) | 3 | `D-307_hue_saturation_ranges.md` | `D-307 pictures/` |

Commits: cbbaec2 (D-290 to D-297), 79e0eae (D-298), d37a364 (D-299 to D-303), f28574e (D-304), c6cd69e (D-305, D-306), and the D-307 commit.

## Checks

After each batch: the whole core test suite and the app test suite. The only failures are two scratch measurements that failed before P-26 began and are not part of the project's checks (`zz_scratch_g2`, `zz_scratch_p25`). The app suite: 89 pass, 5 set aside as before.

No fixture's expected values were changed.

## Waiting on the owner

1. **Playtests** of D-290 to D-307, each page's "For the owner to try".
2. **D-308** (document 14): After Effects differences that would change existing pictures, so each is your call. Polar's circle, Add without a ceiling, Gaussian and Glow strength, Fractal Noise's wider ranges, Time Remapping, Advanced Lightning's extras, Exposure past 20.
3. **D-309** (document 14): additions that would not change any existing picture, waiting only for a go-ahead. Displacement Map's Expand Output, Colorama, Bulge's two radii, a composition background colour, Solid Composite and Channel Blur, Freeze Frame and Time-Reverse Keyframes, CC Glass and Unsharp Mask.

Left out by the charter, not proposed: video import, the tracker, particles (CC Pixel Polly), 3D layers and lights, the two-node camera.
