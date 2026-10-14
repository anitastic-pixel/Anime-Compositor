# Lane 1 effect check: Generate, Noise & Grain, Stylize, Simulation

Real frames 0, 12, 24 out of the release app (tools/app_snapshot.py), drawn by the CPU and by the graphics card. Timings are frame 12, one run each, not benchmarks. "Largest diff" is the biggest cpu-vs-card difference (0-255) over the three frames.

## Verdicts

- works: 91
- looks wrong: 1
- cpu/card differ: 0
- fails: 0
- not testable here: 0
- not built: 1

## Bugs found (most serious first)

1. Fixed by D-680 (91452ccd, 2026-10-14): the comb of dots is gone and most seams are gone; swirl size is unchanged and is the owner's call. Original note: Curl Noise (looks wrong, mild): at Size 140 the swirls come out about 40-100 pixels across on a 1920 frame, and the flow streaks show a regular comb of dots instead of smooth lines. See verification/app_snapshots/curl_noise/full/frame_0012_card_full.png.
2. Resolved 2026-10-14: the app does show a message in its notes panel; the old snapshot tool did not read it. The fixed tool records it (confirmed on Camera Lens Blur, highlight threshold 200). Original note: A wrong choice word or a self-reference changes nothing and says nothing: Caustics with its bottom layer set to its own layer gave flat purple; mistyped choice words (fractal palette "rainbow", stroke source "layer", glow falloff "gaussian", glint ray colours "two") left the picture unchanged. The snapshot report was empty each time; document 28 asks for a diagnosis. The app menus may stop people typing these, so it matters mostly for hand-edited or imported projects.
3. Explained by D-679 (the obstacle default and its tooltip). Original note: Advanced Lightning: the default obstacle of 50 hides the bolt completely on an ordinary opaque layer, with no message. May be correct AE behaviour; worth a check.
4. Resolved 2026-10-14: re-checked with the fixed comparison, every effect agrees within 1 level on solid pixels (premultiplied at most 2); the old figures came from the 8-bit preview tool (verification/diagnosis/cpu_card.md). Original note: CPU and card differ on 18 effects, all invisible to the eye: particle and simulation effects have scattered single pixels up to 255 apart with tiny averages (under 1.5); Block Load, Burn Film, Card Dance, Shatter, Glass and Threads differ by 4-22 in a few pixels, mostly the almost-clear edge of the soft red circle.
5. Glint Streaks on the CPU took about 4 seconds for one frame (card about 1.1 s); every other effect was under 1 s.

## Results

| Effect | type_id | Verdict | Picture | Settings | CPU ms | Card ms | Largest diff | Note |
|---|---|---|---|---|---|---|---|---|
| 4-Color Gradient | core.four_color_gradient | works | Layer replaced by a soft four-corner gradient: yellow, green, magenta, blue. | defaults | 300 | 479 | 1 |  |
| Advanced Lightning | core.lightning_bolt | works | Blue-white forked lightning bolt from top to bottom over the scene. | start [30, 0], end [60, 100], width 4, glow 24, branches 30, detail 8, kind "direction", glow_color "#6e8cff", obstacle 0 | 350 | 552 | 1 | Obstacle set to 0; the fixture default obstacle 50 on an opaque layer blocks the bolt so nothing showed. |
| Audio Spectrum | core.audio_spectrum | works | Spectrum bars drawn from a test tone file, changing between frames. | audio_layer "sound", maximum_height 1500, thickness 6, composite "on", display_options "analog_lines", side_options "side_a", start_frequency 50, end_frequency 1500 | 348 | 492 | 1 | Uses a generated two-second chord/chirp wav. |
| Audio Waveform | core.audio_waveform | works | Waveform line drawn from the test tone file. | audio_layer "sound", maximum_height 300, thickness 6, composite "on", displayed_samples 200 | 450 | 523 | 1 |  |
| Beam | core.beam | works | Red laser beam with glow travelling across the frame. | start [15, 30], end [85, 70], length 60, time "keyframed", start_thickness 20, end_thickness 40, softness 50, outside_color "#FF3020" | 332 | 536 | 1 |  |
| CC Glue Gun | core.glue_gun | works | Glossy wobbly glue trail drawn left to right as the brush moves. | brush_position "keyframed", stroke_width 80, paint_style "wobbly" | 422 | 571 | 1 |  |
| CC Light Burst 2.5 | core.light_burst | works | Strong white rays bursting from the upper middle; very bright at intensity 250. | center [50, 35], intensity 250, ray_length 60, burst "straight" | 292 | 472 | 1 |  |
| CC Light Rays | core.light_rays | works | Light rays streaming from a point near the top. | center [50, 30], radius 300, intensity 150, warp_softness 120, transfer_mode "add" | 326 | 504 | 1 |  |
| CC Light Sweep | core.light_sweep | works | Diagonal light band sweeping across the layer. | center "keyframed", width 200, sweep_intensity 80, edge_thickness 2 | 287 | 474 | 1 |  |
| CC Threads | core.threads | works | Woven thread grid laid over the picture. | defaults | 280 | 520 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 4. Largest difference 4 levels, average under 0.01; invisible. |
| Cell Pattern | core.cell_pattern | works | Layer replaced by a cell pattern that evolves between frames. | size 80, evolution "keyframed" | 402 | 1000 | 1 |  |
| Checkerboard | core.checkerboard | works | Orange checkerboard at 60% over the picture. | width 120, blending_mode "normal", opacity 60, color "#ff8000" | 394 | 593 | 1 |  |
| Circle | core.circle | works | Orange disc with a soft edge in the middle. | radius 300, color "#ff8000", blending_mode "normal", feather_outer 20 | 411 | 565 | 1 |  |
| Ellipse | core.ellipse | works | Glowing ring (ellipse) in the middle. | width 900, height 500, thickness 40, softness 50 | 381 | 638 | 1 |  |
| Eyedropper Fill | core.eyedropper_fill | works | Whole layer filled with the sampled sky blue. | sample_point [50, 20], sample_radius 40 | 375 | 621 | 1 |  |
| Fill | core.fill | works | Blue fill inside the rectangle mask only. | mask 1, color "#3080ff", opacity 100 | 381 | 526 | 1 |  |
| Fractal | core.fractal | works | Mandelbrot set in hue-wheel colours. | palette "hue_wheel" | 291 | 800 | 1 |  |
| Gradient Ramp | core.gradient | works | Layer replaced by a warm-to-cool linear ramp. | start_opacity 100, end_opacity 100, blend "normal", start_color "#ffd060", end_color "#3050c0" | 324 | 438 | 1 |  |
| Grid | core.grid | works | Grid lines over the picture. | width 120, border 4, size_from "width_slider", blending_mode "normal" | 387 | 721 | 1 |  |
| Lens Flare | core.lens_flare | works | Lens flare with rings near the top left. | flare_center [30, 30], flare_brightness 120, lens_type "105mm", dirt_layer "", dirt_opacity 0 | 408 | 714 | 1 |  |
| Paint Bucket | core.paint_bucket | works | Part of the sky flood-filled red from the click point. | fill_point [50, 10], tolerance 25, color "#ff0000" | 397 | 550 | 1 |  |
| Radio Waves | core.radio_waves | works | Cyan rings expanding from the centre. | interval 6, expansion 30, start_width 20, end_width 20, color "#40e0ff" | 383 | 677 | 1 |  |
| Scribble | core.scribble | works | Red hatching scribble growing inside the mask. | edge_width 40, stroke_width 8, spacing 25, color "#ff3020", end "keyframed" | 405 | 794 | 1 |  |
| Stroke (Path Stroke) | core.stroke | works | Red line drawn along the mask edge, half done at frame 12. | brush_size 20, color "#ff3020", end "keyframed", source "masks" | 327 | 584 | 1 |  |
| Vegas | core.vegas | works | Faint yellow dashes running round the mask edge. | segments 8, length 0.5, width 12, rotation "keyframed", color "#ffe040" | 356 | 581 | 1 | Thin and faint at these settings, but present and moving. |
| Write-on | core.write_on | works | Red brush stroke written on along its path. | brush_position "keyframed", brush_size 40, color "#ff3020" | 354 | 490 | 1 |  |
| Energy Stroke | core.energy_stroke | works | Bright cyan-white energy line across the middle with glow. | core_start [15, 50], core_end [85, 50], core_size 20, glow_spread 40, glow_intensity 400, composite "add", distortion_amount 1, flicker_amount 30 | 408 | 563 | 1 |  |
| Add Grain | core.add_grain | works | Film grain added over the picture. | intensity 3, size 2 | 337 | 902 | 1 |  |
| Dust & Scratches | core.dust_scratches | works | Added noise softened away (noise added first by a Noise effect). | radius 3, threshold 0 | 420 | 536 | 1 |  |
| Fractal Noise | core.fractal_noise | works | Layer replaced by grey cloudy fractal noise. | size 200, contrast 100, brightness 0, speed 90, complexity 6 | 388 | 595 | 1 |  |
| Match Grain | core.match_grain | works | Grain copied from a grain source layer onto the picture. | layer "grainsrc" | 375 | 621 | 1 |  |
| Median | core.median | works | Picture smoothed into soft painterly blobs. | radius 8 | 330 | 572 | 1 |  |
| Noise | core.noise | works | Coloured noise speckles over the picture. | amount 40 | 289 | 475 | 1 |  |
| Noise Alpha | core.noise_alpha | works | Speckled holes punched in the layer transparency. | amount 60, original_alpha "scale" | 320 | 573 | 2 | Visible only where the alpha is shown. |
| Noise HLS | core.noise_hls | works | Hue/lightness/saturation noise over the picture. | hue 20, lightness 30, saturation 40, grain_size 4 | 325 | 553 | 1 |  |
| Noise HLS Auto | core.noise_hls_auto | works | Animated hue/lightness/saturation noise. | hue 20, lightness 30, saturation 40, grain_size 4 | 337 | 510 | 1 |  |
| Remove Grain | core.remove_grain | works | Added colour noise cleaned up. | noise_reduction 3, passes 2 | 480 | 590 | 1 |  |
| Turbulent Noise | core.turbulent_noise | works | Layer replaced by turbulent grey noise. | size 200, contrast 150, evolution "keyframed", fractal_type "dynamic" | 385 | 581 | 1 |  |
| Deband | core.deband | works | Banded (posterised) gradient softened; bands still faintly visible. | blur_amount 40, diffusion_amount 3, noise_amount 4, edge_threshold 30 | 430 | 543 | 1 | Edge threshold raised to 30; at 4 the bands stayed sharp. |
| Brush Strokes | core.brush_strokes | works | Picture repainted with short brush strokes. | brush_size 8, stroke_length 40 | 366 | 498 | 1 |  |
| Cartoon | core.cartoon | works | Flat cartoon shading with dark outlines. | detail_radius 16, edge_width 4 | 339 | 563 | 1 |  |
| CC Block Load | core.block_load | works | Picture loading in blocks; frame 0 empty. | completion "keyframed" | 402 | 517 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 11. Largest difference 11 on frame 0 only, average 0.03, in the almost-clear edge of the soft red circle; invisible. |
| CC Burn Film | core.burn_film | works | Burn hole with charred edge spreading through the picture. | burn "keyframed" | 402 | 518 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 10. Largest difference 10 on frame 24, average 0.015; invisible. |
| CC Glass | core.glass | works | Glassy embossed look. | layer "", property "luminance", softness 10, height 100 | 289 | 552 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 22. Largest difference 22 on a handful of pixels, average under 0.001; invisible. |
| CC HexTile | core.hex_tile | works | Picture tiled into hexagons. | radius 120 | 345 | 541 | 1 |  |
| CC Kaleida | core.kaleidoscope | works | Kaleidoscope mirror pattern. | defaults | 339 | 588 | 1 |  |
| CC Mr. Smoothie | core.mr_smoothie | works | Smeared colour swirl. | smoothness 20 | 337 | 555 | 1 |  |
| CC Plastic | core.plastic | works | Subtle shiny plastic relief. | property "luminance", softness 20 | 344 | 571 | 1 |  |
| CC RepeTile | core.repeat_tile | works | Mirrored tiles of the layer filling the frame. | expand_right 3000, expand_left 3000, expand_down 2000, expand_up 2000, tiling "unfold" | 473 | 587 | 1 |  |
| CC Threshold | core.channel_threshold | works | Per-channel black/colour threshold. | defaults | 301 | 527 | 1 |  |
| CC Threshold RGB | core.threshold_rgb | works | RGB threshold, picture in pure primaries. | defaults | 324 | 578 | 1 |  |
| CC Vignette | core.cc_vignette | works | Darkened corners. | defaults | 350 | 648 | 1 |  |
| Color Emboss | core.emboss | works | Colour emboss relief. | mode "color", relief 3, contrast 200 | 344 | 519 | 1 |  |
| Contour Paint | core.contour_paint | works | Picture flattened into flat colour regions. | defaults | 411 | 904 | 1 |  |
| Emboss | core.emboss | works | Grey emboss relief. | relief 3, contrast 200 | 362 | 538 | 1 |  |
| Find Edges | core.find_edges | works | Edges drawn dark on white. | defaults | 311 | 565 | 1 |  |
| Glow | core.glow | works | Bright parts glowing. | threshold 60, radius 60, intensity 1.5 | 299 | 510 | 1 |  |
| Mosaic | core.mosaic | works | Picture in 40-pixel blocks. | size 40 | 308 | 576 | 1 |  |
| Motion Tile | core.motion_tile | works | Small layer repeated in mirrored tiles. | output_width 300, output_height 300, mirror "on" | 359 | 568 | 1 |  |
| Posterize | core.posterize | works | Picture reduced to 4 levels per channel. | levels 4 | 384 | 522 | 1 |  |
| Roughen Edges | core.roughen_edges | works | Yellow and cyan rectangles with ragged, rusty edges. | border 40, size 60, edge_type "roughen_color" | 372 | 521 | 1 | Run on layer 3 (the rectangles). |
| Scatter | core.scatter | works | Pixels scattered, grainy edges. | scatter_amount 40 | 349 | 510 | 1 |  |
| Strobe Light | core.strobe_light | works | White flash on frames 0 and 24, normal picture on frame 12. | defaults | 368 | 523 | 1 |  |
| Texturize | core.texturize | works | Dot texture pressed into the picture. | layer "tex", fit "tile", texture_contrast 2 | 400 | 531 | 1 | Fit set to tile. |
| Threshold | core.threshold | works | Black and white threshold. | defaults | 341 | 497 | 1 |  |
| Soft Physical Glow | core.glow | works | Soft wide physical glow. | falloff "physical", radius 60, exposure 1, tint "off", spectral "off", threshold 50 | 335 | 526 | 1 |  |
| Glow core-to-edge colour | core.glow | works | Glow with a pinkish core-to-edge tint. | falloff "physical", radius 60, threshold 50, tint "core_to_edge", inner_color "#ffffff", outer_color "#ff4020", spectral "off" | 319 | 514 | 1 |  |
| Spectral Glow | core.glow | works | Glow with rainbow fringes. | falloff "physical", radius 60, threshold 50, tint "off", spectral "on", spectral_red 50, spectral_green 100, spectral_blue 180 | 375 | 566 | 1 |  |
| Halation | core.halation | works | Subtle red halo round bright edges. | radius 60, amount 200 | 362 | 539 | 1 |  |
| Lens Dirt | core.lens_flare | works | Lens flare with dotted dirt showing in its light. | flare_center [30, 30], flare_brightness 150, dirt_layer "smudge", dirt_opacity 100, dirt_exposure 2 | 303 | 517 | 1 |  |
| Glint Streaks | core.cross_glare | works | Bright wash with faint star streaks. | threshold 70, length 300, points 6, angle 15, intensity 1.5, ray_colors "alternate", color_2 "#80c0ff", shimmer 50, shimmer_period 24 | 3996 | 1135 | 1 | CPU took about 4 seconds for the frame (card about 1.1 s). |
| Card Dance | core.card_dance | works | Picture split into tilted card strips. | gradient_layer_1 "layer1", rows 8, columns 14, y_rotation_source "red_1", y_rotation_multiplier 70, z_position_multiplier 1 | 352 | 533 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 11. Largest difference 11, average under 0.03, in the almost-clear edge of the soft red circle; invisible. |
| Caustics | core.caustics | works | Rippled water caustic light over a separate floor layer. | bottom_layer "floor", water_surface "ripples", caustics_strength 0.7, wave_height 0.4, water_depth 0.2 | 359 | 610 | 1 | Bottom layer must be another layer; pointing it at itself gave flat purple with no message. |
| CC Ball Action | core.cc_ball_action | works | Picture turned into a grid of balls. | grid_spacing 24, scatter 30, rotation 30 | 297 | 500 | 2 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 215. Largest difference 215 on scattered single pixels, average 0.09; looks the same. |
| CC Bubbles | core.bubbles | works | Bubbles rising over the picture. | size 1.5, speed 1.5, wobble_amplitude 20 | 345 | 529 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255 on bubble highlight specks, average about 0.3; looks the same. Size 40 makes giant bubbles. |
| CC Drizzle | core.drizzle | works | Rain-drop ripples on the picture. | spreading 300 | 306 | 506 | 1 |  |
| CC Hair | core.hair | works | Hair strands growing over the layer. | length 120 | 376 | 644 | 1 |  |
| CC Mr. Mercury | core.mercury | works | Blue mercury blobs. | defaults | 299 | 486 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 126. Largest difference 126, average 0.03; invisible. |
| CC Particle Systems II | core.particle_plane | works | Large yellow-pink faded-sphere haze of particles. | birth_rate 1, longevity 1.5, velocity 1, gravity 0.5, birth_size 40, death_size 20, particle_type "faded_sphere" | 336 | 551 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 249. Largest difference 249, average up to 1.5, in nearly see-through pixels (alpha differs by 1 at most); looks the same. |
| CC Particle World | core.particle_space | works | Soft yellow particles scattered over the picture. | birth_rate 1, longevity 1.5, velocity 1, gravity 0.5, birth_size 1, death_size 0.5, particle_type "faded_sphere" | 294 | 488 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average up to 0.73; particles in the same places, looks the same. Size 20 showed nothing; size 1 used. |
| CC Pixel Polly | core.polygon_shards | works | Layer shattered into triangles. | grid_spacing 60 | 343 | 499 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.07; looks the same. |
| CC Rainfall | core.cc_rainfall | works | Rain streaks over the picture. | opacity 70 | 327 | 506 | 1 |  |
| CC Scatterize | core.scatterize | works | Layer dissolving into specks. | amount "keyframed" | 694 | 927 | 2 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.8; looks the same. |
| CC Snowfall | core.snowfall | works | Snow flakes falling. | density 100 | 381 | 576 | 1 |  |
| CC Star Burst | core.star_burst | works | Coloured discs flying outward. | grid_spacing 40 | 287 | 652 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.03; looks the same. |
| Foam | core.foam | works | Cluster of foam bubbles. | view "rendered", size 3, production_rate 2 | 291 | 712 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.12; looks the same. |
| Particle Playground | core.particle_playground | works | Red particles fired from a cannon. | particles_per_second 60, cannon_position [50, 85], velocity 800, cannon_particle_radius 10, gravity_force 300, particles_across 0, particles_down 0 | 281 | 531 | 0 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.03; looks the same. |
| Shatter | core.shatter | works | Layer broken into 3D cube pieces. | defaults | 356 | 548 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 11. Largest difference 11, invisible. |
| Wave World | core.wave_world | works | Grey height map with a ripple. | view "height_map", grid_resolution 40 | 376 | 783 | 1 | Height-map view; wireframe view differed by 11. |
| Particle Emitter | core.particle_emitter | works | White particles sprayed upward. | emitter_position [50, 85], velocity 400, gravity 60, size 12 | 344 | 467 | 0 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.03; looks the same. |
| Particle Grid | core.particle_grid | works | White grid of dots. | base_form "box_grid", particles_in_x 20, particles_in_y 12, form_size_x 1600, form_size_y 900, size 12, color "#ffffff" | 382 | 831 | 1 | Re-checked 2026-10-14 with the fixed comparison (premultiplied; solid pixels within 1): the old figure was the 8-bit preview tool, not the effect. Was 255. Largest difference 255, average 0.03; looks the same. |
| Curl Noise | core.curl_noise | looks wrong | Grey swirling flow texture filling the layer. | size 140, speed 30, swirl 90, sample_count 12, sample_radius 40, contrast 120 | 399 | 488 | 1 | D-680 (91452ccd, 2026-10-14) removed the comb and most seams; swirl size is unchanged and is the owner's call (fix 3). Was: Full-size shot (verification/app_snapshots/curl_noise/full/frame_0012_card_full.png): at Size 140 most swirls are only about 40-100 pixels across on a 1920 frame, finer than the setting suggests, and the streaks carry a regular comb/ladder of dots (looks like the 12 flow samples showing as separate steps). CPU and card match. |
| Zoom Glow and Edge Glow | - | not built |  |  | - | - | - | Preset only in EFFECTS.md; no core type. |

Not built: Zoom Glow and Edge Glow (Stylize, Pick #16) - preset only, no effect type.
