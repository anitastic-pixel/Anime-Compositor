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

## After D-308 (D-318 to D-324), played again (2026-10-04)

The owner approved D-308 on 2026-10-04. Five of its parts are built as D-318 to D-324; D-321 (Gaussian Blur strength) and D-322 (Glow strength) are held back, below. Tutorials 1, 2 and 3 were then played again from an empty project to the last step in the test copy. Tutorials 4 and 5 use nothing that changed, so their rows stand.

| Tutorial | Steps | Passed before | Now fixed | Partly | Still open | Still open because of |
|---|---|---|---|---|---|---|
| 1 Shockwave | 24 | 14 | 9 | 1 | 0 | — |
| 2 Advanced Electric | 18 | 5 | 11 | 1 | 1 | D-321, D-322 |
| 3 Colorful Glitch | 29 | 12 | 13 | 1 | 3 | D-322, charter (camera) |
| 4 Ultimate Lightsaber | 22 | 15 | 6 | 0 | 1 | D-321 |
| 5 Film Riot Lightsaber | 12 | 7 | 5 | 0 | 0 | — |
| **All** | **105** | **53** | **44** | **3** | **5** | |

What D-318 to D-324 closed:

- Tutorial 1: Polar Coordinates is now a circle (D-320), Complexity 10 is taken (D-318), and Time Remapping keys 8 at frame 4 and 16 at frame 12 (D-323).
- Tutorial 2: the bolt stops on the ground with Alpha Obstacle 50 (D-324), Add keeps its light past white in Float (D-319), and Exposure 20.49 is taken (D-318).
- Tutorial 3: the two glitch compositions work past white in Float (D-319), and Brightness 153 and -154 are taken (D-318).

How each one looks now:

1. **Shockwave.** The ring is round, with a dark hole in its middle. It is still brighter in the middle than the tutorial's.
2. **Advanced Electric.** The bolt ends on the ground line. The wet-ground reflection now blazes instead of stopping at white, but keeps its hard box edge (Gaussian Blur, D-321), and the gold final still breaks into dust (Glow, D-322).
3. **Colorful Glitch.** The final frame looks as before. The strong glow is still a white slab, because Glow stays held to 0 to 1 even in Float (document 21; D-322). Brightness -204 is still past the new -200 limit, so -200 was used.

The 5 still open: the final look of tutorial 2 (Gaussian and Glow), tutorial 3's Glow and final look, tutorial 4's Gaussian, and the two-node camera (charter). Partly fixed: Fractal Noise's Dynamic Progressive type (1), the glow step of tutorial 2 and one Glow-and-Fractal step of tutorial 3.

Not built from D-308:

- T1-3's "refuse only the bad value": the panel already sends one changed value at a time, so only that value is refused (D-318's page).
- After Effects' negative Alpha Obstacle, which keeps the bolt inside a shape; and the bolt stops at an obstacle rather than going round it (D-324's page).
- D-324's eight Lightning types are this program's reading of Adobe's one-line descriptions. No After Effects frame was compared: Adobe's help page refused the request.

## After D-321, D-322, D-325 to D-327 and D-331, played again (2026-10-05)

The owner has no After Effects, so the two held items were settled from sources on the internet instead of a test in After Effects:

- **Gaussian Blur (D-321).** Two programs that play After Effects' own files use the same number for Blurriness: [lottie-web](https://github.com/airbnb/lottie-web/blob/master/player/js/elements/svgElements/effects/SVGGaussianBlurEffect.js) ("Empirical value, matching AE's blur appearance", 0.3) and Skia's [Skottie](https://github.com/google/skia/blob/main/modules/skottie/src/SkottiePriv.h) ("Close-enough to AE", 0.3).
- **Glow (D-322, corrected by D-331).** The Creative COW thread [Glow Effect and transparent background mechanics](https://creativecow.net/forums/thread/glow-effect-and-transparent-background-mechanics/) measured After Effects' Glow on white shapes. D-322 read its brightness rule as the glow's colour. Against tutorials 2 and 3 that was far too bright: a white slab and a haze of sparks. D-331 reads it as the colour *read straight* (colour over covering), which fits the thread and both tutorials.
- **Exposure** works in linear light ([After Effects CS3 manual, p.402](https://www.manualsdir.com/manuals/753848/adobe-after-effects-cs3.html?page=402)). **32 bpc** keeps values past white ([Prolost, "Linear color workflow in AE7"](https://prolost.com/blog/2006/2/8/linear-color-workflow-in-ae7-part-2.html)).

Also built: D-325 (a blurred layer keeps its outline in place, and Blur is named Gaussian Blur, from the owner's report), D-326 (Fractal Noise Brightness to ±1000, so tutorial 3's -204 is typed as is) and D-327 (Fast Box Blur, which tutorials 2 and 3 use).

All five tutorials were then played again from an empty project to the last step in the test copy.

| Tutorial | Steps | Passed before | Now fixed | Partly | Still open | Still open because of |
|---|---|---|---|---|---|---|
| 1 Shockwave | 24 | 14 | 9 | 1 | 0 | — |
| 2 Advanced Electric | 18 | 5 | 11 | 1 | 1 | D-330, D-328 (proposed) |
| 3 Colorful Glitch | 29 | 12 | 14 | 2 | 1 | charter (camera) |
| 4 Ultimate Lightsaber | 22 | 15 | 7 | 0 | 0 | — |
| 5 Film Riot Lightsaber | 12 | 7 | 5 | 0 | 0 | — |
| **All** | **105** | **53** | **46** | **4** | **2** | |

How each one looks now:

1. **Shockwave.** As before: a round ring with a dark hole, brighter in the middle than the tutorial's.
2. **Advanced Electric.** The gold final is a thin bolt with sparks round it, as in After Effects (D-331). Two gaps remain, both proposed. The ground reflection is a glowing block where After Effects shows a small glow: the tutorial works in 8 bpc, which rounds the blur's faint edge to nothing (D-330). And the glow turns to dust under Turbulent Displace at Size 2 (D-328).
3. **Colorful Glitch.** The letters stand clear with a soft halo, as in After Effects, instead of a white slab (D-331), and Brightness -204 is typed as is (D-326). Partly: our background is grey where the tutorial's is dark navy (we stand in for its textures), and frame 110's glitch slices differ. The two-node camera is left out by the charter.
4. **Ultimate Lightsaber.** The blade's glow matches the tutorial's width now that Gaussian Blur uses After Effects' Blurriness (D-321).
5. **Film Riot Lightsaber.** The blue spill fades out smoothly; the hard box at the blur's edge is gone (D-321).

Not built, proposed for the owner's decision:

- **D-328** Turbulent Displace at small sizes (tutorial 2's dust). No source says how After Effects scales Amount with Size, and a change would alter existing pictures.
- **D-329** Advanced Lightning that goes round shapes, and a negative Alpha Obstacle (D-308's last item). It needs a different way of growing the bolt.
- **D-330** a composition depth "8 bpc (After Effects)" that rounds to 8 bits after every effect (tutorial 2's reflection block). Page: `D-330_eight_bit_rounding_proposal.md`.

Commits: 14280da, c760ed2 (D-321), 7f37444, 5cd1039 (D-322), 822c7e6 (D-325), 2fc709c, 45c36ac (D-326), 88db5d1, 730b28d (D-327, D-330 proposed), efb2671, 7a0b023 (D-331).

## What was built

Every one is **built and awaiting the owner's playtest**. Each starts where the app was before, so older projects and every fixture draw as they did; D-318 to D-324 (D-308) may change pictures where the owner turns them on, and a newly added Polar Coordinates starts as a circle. Each has its own page in `verification/`, with what to look for and how to try it.

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
| D-318 | Wider ranges: Exposure to ±40, Fractal Noise Complexity to 20 and Brightness to ±200 | 1, 2, 3 | `D-318_ranges.md` | `D-318 pictures/` |
| D-319 | Float working depth: a composition can work past white | 2, 3 | `D-319_float_depth.md` | `D-319 pictures/` |
| D-320 | Polar Coordinates' Shape: Circle (new) or Ellipse (old files) | 1 | `D-320_polar_circle.md` | `D-320 pictures/` |
| D-323 | Time Remapping | 1 | `D-323_time_remap.md` | `D-323 pictures/` |
| D-324 | Advanced Lightning's extras: Lightning Type, Turbulence, Decay, Conductivity State, Alpha Obstacle | 2 | `D-324_lightning_extras.md` | `D-324 pictures/` |
| D-321 | Gaussian Blur in After Effects' Blurriness, with a soft far edge | 2, 4, 5 | `D-321_blurriness.md` | `D-321 pictures/` |
| D-322, D-331 | Glow in After Effects' numbers (Units: After Effects or Classic), corrected | 2, 3 | `D-322_glow_ae.md`, `D-331_glow_ae_corrected.md` | `D-322 pictures/`, `D-331 pictures/` |
| D-325 | A blurred layer keeps its outline in place; Blur is named Gaussian Blur | owner's report | `D-325_blur_outline.md` | `D-325 pictures/` |
| D-326 | Fractal Noise Brightness from -1000 to 1000 | 3 | `D-326_fractal_brightness.md` | `D-326 pictures/` |
| D-327 | Fast Box Blur | 2, 3 | `D-327_fast_box_blur.md` | `D-327 pictures/` |

Commits: cbbaec2 (D-290 to D-297), 79e0eae (D-298), d37a364 (D-299 to D-303), f28574e (D-304), c6cd69e (D-305, D-306), bf0978f (D-307), 69980f7 (D-310), e419620 (D-311 to D-313, D-315), 5f866db (D-314), 5ea35a3 (D-316), 15f8f25 (D-317), 4a17745 (D-318), c4fc502 (D-319), e762c99 (D-320), d3bca9e (D-323), f559354 (D-324).

## Checks

After each batch: the whole core test suite and the app test suite. The only failures are two scratch measurements that failed before P-26 began and are not part of the project's checks (`zz_scratch_g2`, `zz_scratch_p25`). The app suite: 91 pass, 5 set aside as before.

One set of fixture values was changed, as a recorded specification decision: D-331 gave FX-GLOW-AE-001, 004, 006, 007 and 008 new expected values (Glow in After Effects units, built the day before), committed before the code. No other fixture's expected values were changed.

## Waiting on the owner

1. **Playtests** of D-290 to D-307, D-310 to D-317 and D-318 to D-324, each page's "For the owner to try".
2. **Playtests** of D-321, D-322 with D-331, and D-325 to D-327, each page's "For the owner to try". The After Effects test asked for here before is no longer needed: D-321 and D-331 are settled from the sources above.
3. **Decisions** on D-328, D-329 and D-330 (proposed, nothing built).

D-309 was approved on 2026-10-04 and is built (D-310 to D-317). D-308 was approved on 2026-10-04 and is built as D-318 to D-320, D-323 and D-324.

Left out by the charter, not proposed: video import, the tracker, particles (CC Pixel Polly), 3D layers and lights, the two-node camera.
