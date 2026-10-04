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

"Now fixed" means the step's missing piece was built and checked; the owner's playtest is still the judge. The counts above are from before D-309; the table below is after it.

## After: all five played through again (2026-10-04)

After D-310 to D-317 (D-309) were built, every tutorial was played again from an empty project to its last step in the test copy, with the fixes in use: 23.976 fps, expressions on effect settings, the new Fractal Noise looks, luma mattes, Overlay and Soft Light, Colorama, Expand Output, Bulge's two radii, CC Glass, Sharpen's threshold, Solid Composite, black composition backgrounds, and so on. Every step the app can do ran; the only refusals were values past a range that D-308 would widen (Fractal Noise Brightness -204 and 153, Exposure 20.49). The stills are the test copy's own pictures; they stay out of the repository with the tutorial frames.

| Tutorial | Steps | Passed before | Now fixed | Partly | Still open | Still open because of |
|---|---|---|---|---|---|---|
| 1 Shockwave | 24 | 14 | 6 | 1 | 3 | D-308 |
| 2 Advanced Electric | 18 | 5 | 8 | 1 | 4 | D-308 |
| 3 Colorful Glitch | 29 | 12 | 11 | 2 | 4 | D-308, charter (camera) |
| 4 Ultimate Lightsaber | 22 | 15 | 6 | 0 | 1 | D-308 |
| 5 Film Riot Lightsaber | 12 | 7 | 5 | 0 | 0 | — |
| **All** | **105** | **53** | **36** | **4** | **12** | |

D-309 closed five more: Bulge's two radii (1), Freeze Frame with a luma matte (2), Expand Output and Colorama (3), and Shift Channels with Solid Composite (3).

How each one looks now, against the tutorial's own result:

1. **Shockwave.** The burst is centred on a black background, bent by CC Glass and the two-radius Bulge, sharpened with a threshold. At its middle it still fills in white where After Effects shows a ring with a dark hole: Polar Coordinates makes an ellipse, and there is no Time Remapping (both D-308).
2. **Advanced Electric.** The bolt, the ground mask and the hotspot match. The wet-ground reflection now shows the ground's texture through the luma matte, but inside a hard-edged box instead of a soft puddle, and the final glow breaks into fine dust. The box: Gaussian Blur stops at three times its softness, and Exposure +20 (about a million times brighter) turns the faint last edge of the blur into a visible wall. The dust is likely the same story: our glow is narrower than After Effects', so the tutorial's fine Turbulent Displace scatters it (not measured). Both belong with D-308's Gaussian and Glow item, together with Add's ceiling.
3. **Colorful Glitch.** The glitch map, the displaced text, the red wipes, the stage and the reflection all work and read like the tutorial's. The stage is lighter grey than After Effects' dark blue (stock-texture stand-ins), and the strong glow blows to a white slab because values cannot go past white (D-308).
4. **Ultimate Lightsaber.** Both methods (green, and blue through Hue/Saturation) run to the end. The glow is wider and flatter than After Effects' at the same numbers (D-308 Gaussian). The saw-tooth along the blade is the striped prop in the stand-in footage showing beside the drawn blade, not the app.
5. **Film Riot Lightsaber.** Fixed: the frame no longer floods blue, the face and room keep their colours, and the blade is white-hot inside a blue halo, as in the tutorial. The halo ends in a straight edge at the top and left, for the same reason as tutorial 2's box (the feather's edge, brightened by Color Balance): D-308's Gaussian item.

The 12 still open, and what holds each:

- Polar Coordinates makes an ellipse, not After Effects' circle (tutorial 1): D-308, changes existing pictures.
- Fractal Noise Complexity 10 is refused, the cap is 8 (1): D-308.
- Time Remapping (1): D-308.
- Advanced Lightning's Alpha Obstacle (2): D-308.
- Add without a ceiling, and Exposure past 20 (2, two steps): D-308.
- Glow at the tutorial's values is much weaker (2 and 3): D-308.
- The final look of tutorials 2, 3 and 4: held by the items above, mainly Add without a ceiling and Glow and Gaussian strength.
- 32-bit values past white (3): D-308.
- The two-node camera and 3D layers (3): left out by the charter.

Partly fixed: Fractal Noise's Dynamic Progressive type (Invert and Offset are built), Fractal Noise Brightness -154 (Scale Height is built, the range stops at -100), Fast Box Blur with Exposure (the blur is built, Add's ceiling is not), and the two Glow-and-Fractal steps of tutorial 3.

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
| D-310 | Bulge's Vertical Radius and Taper Radius | 1 | `D-310_bulge_radii.md` | `D-310 pictures/` |
| D-311 | A composition's background colour | every tutorial | `D-311_comp_background.md` | `D-311 pictures/` |
| D-312, D-313 | Solid Composite and Channel Blur | 3 | `D-312_D-313_solid_composite_channel_blur.md` | `D-312 D-313 pictures/` |
| D-314 | Time-Reverse Keyframes and Freeze Frame | 1, 2 | `D-314_reverse_keys_freeze_frame.md` | `D-314 pictures/` |
| D-315 | Displacement Map's Expand Output | 3 | `D-315_displacement_expand_output.md` | `D-315 pictures/` |
| D-316 | Colorama (reduced) | 3 | `D-316_colorama.md` | `D-316 pictures/` |
| D-317 | Unsharp Mask's Threshold and CC Glass (reduced) | 1 | `D-317_unsharp_glass.md` | `D-317 pictures/` |

Commits: cbbaec2 (D-290 to D-297), 79e0eae (D-298), d37a364 (D-299 to D-303), f28574e (D-304), c6cd69e (D-305, D-306), bf0978f (D-307), 69980f7 (D-310), e419620 (D-311 to D-313, D-315), 5f866db (D-314), 5ea35a3 (D-316), 15f8f25 (D-317).

## Checks

After each batch: the whole core test suite and the app test suite. The only failures are two scratch measurements that failed before P-26 began and are not part of the project's checks (`zz_scratch_g2`, `zz_scratch_p25`). The app suite: 90 pass, 5 set aside as before.

No fixture's expected values were changed.

## Waiting on the owner

1. **Playtests** of D-290 to D-307 and D-310 to D-317, each page's "For the owner to try".
2. **D-308** (document 14): After Effects differences that would change existing pictures, so each is your call. Polar's circle, Add without a ceiling, Gaussian and Glow strength, Fractal Noise's wider ranges, Time Remapping, Advanced Lightning's extras, Exposure past 20. Every gap left in the five tutorials, apart from the charter's, sits here. The replay adds one thing to the Gaussian item: the blur's hard edge at three times its softness shows as a box or straight edge once a later step brightens it a lot (tutorials 2 and 5).

D-309 was approved on 2026-10-04 and is built (D-310 to D-317).

Left out by the charter, not proposed: video import, the tracker, particles (CC Pixel Polly), 3D layers and lights, the two-node camera.
