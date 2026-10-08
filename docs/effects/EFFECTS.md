# Effects audit against After Effects

Audit date 2026-10-08. Source list: https://helpx.adobe.com/after-effects/desktop/apply-effects-and-animation-presets/effects-and-animation-presets/effect-list.html (the page refused automated reading, HTTP 403, so the list was compiled from Adobe's per-category effect pages and web searches; newer additions were checked against release notes, see Sources at the bottom).
Status counts over the rows taken from Adobe's list (including Obsolete and the recent additions): 240 rows: 64 done, 13 partial, 163 missing.
Plus 26 effects (24 done, 2 partial) of ours that are not on Adobe's list (last table).
"done" = in the effect list with fixtures and a verification table. "partial" = it exists but is limited, approximate, reduced, or has no fixture folder. "missing" = we have nothing for it.
Effect descriptions are in our own words; nothing here copies Adobe's text, and no Adobe or third-party code or presets are used.

## How to read this file

- **Type id**: our saved-file name for an effect, for example `core.simple_choker`. Shown wherever we have one.
- **Reference / test** shorthand: `FX-XXX-001..NNN` are fixture cases under `Fixtures/`; `bNNN` is `tests/bNNN_*.rs`; `B-NN table` is `verification/B-NN_*_table.md`; `D-NNN` is the decision in `Markdown/14_Decisions_Risks.md`.
- **Depends on** names the Priority 0 items below (P0-1 to P0-20) or another effect.
- **GPU plan**: the card (the graphics card) draws the viewer and preview. Exports and fixtures are always drawn on the CPU, and the card must agree with the CPU to within 1 level in 255 (ADR-006 as amended by D-100). "On card (done)" means the preview already uses the card. An effect whose Mix is below 100 (D-202) is always drawn on the CPU today.
- **Perf target**: every target is a *target*, not a measurement. Classes, all for one 1920x1080 layer on the reference machine below, preview on the card:
  - **Target P1** (one-pixel colour change): adds 1 ms or less, fused with its neighbours into one pass (B-172).
  - **Target P2** (neighbourhood, blur, distort): adds 4 ms or less.
  - **Target P3** (generators, multi-pass, heavy blurs): adds 8 ms or less.
  - **Target P4** (CPU-only work such as multi-frame time effects): 100 ms or less per frame on the CPU.
  - **Target P5** (simulations): 33 ms or less per frame (30 frames a second).
- **Measured numbers** are quoted only from repo files and are *whole-frame* times, not the cost of the effect alone: the 1920x1080 reference shot with the effect on 3 layers, Full quality, release build, on an RTX 4070 Ti SUPER, Ryzen 9 9900X (24 threads), Windows 11. "CPU a / GPU b" is the "again" (second draw) column of the named file. These were taken on older builds; re-measure before comparing.
  - B-65 / B-76 / B-107 = `verification/B-65_gpu_fx_timing_table.md` and the B-76 and B-107 files of the same name. Their no-effect frame is not in those files.
  - B-151 = `verification/B-151_gpu_fx_timing_table.md`, the 2026-09-30 quiet re-measure, every eighth frame; its no-effect frame is CPU 22.0 / GPU 9.6 ms.

## Priority 0: shared infrastructure

| Id | Item | Status | Evidence | What depends on it |
|---|---|---|---|---|
| P0-1 | Parameters, keyframes, ease, expressions on effect settings | done | D-68 effect keys, D-52 ease, D-332/B-213 keys survive a save, D-291/B-176 expressions on effect numbers, D-59/B-14b, `src/expr.rs`, `src/keykind.rs`; FX-FXK-001..009, FX-KIND | every effect |
| P0-2 | GPU effect pipeline (the card) | partial | `src/gpu.rs`, ADR-006/D-100, B-44..B-172; fusing of one-pixel effects B-172, adjustment layers on the card B-156. About 30 effects have no card path yet, and Mix below 100 falls back to the CPU | every "Add card pass" row |
| P0-3 | Reading other layers (layer as a setting, track mattes) | done | Layer maps D-189/B-125, `src/layer_map.rs`, FX-LMAP-001..042; track matte modes D-293/B-178; Light Wrap reads the frame beneath (D-132). Not built: a general "Set Matte"-style read of any layer's channel | Displacement Map, Compound Blur, Gradient Wipe, Set Matte, Difference Matte, Texturize, CC Glass Wipe, Time Displacement |
| P0-4 | Time sampling (other frames of a layer) | partial | Echo D-195/B-130, Posterize Time D-196/B-131, frame blending D-216/B-150, Time Remap D-323/B-202, Freeze/Reverse D-314, motion blur B-124 (card D-226/B-156b). Not built: a per-pixel time offset, or motion estimated between frames | Echo, Posterize Time, Time Displacement, Time Difference, CC Wide Time, CC Force Motion Blur |
| P0-5 | Depth and extra channels (Z, object id, normals) | missing | `src/exr_io.rs` reads colour and alpha only; the only depth is each layer's plane (D-58) | all of 3D Channel, Camera Lens Blur's depth map, Fog 3D |
| P0-6 | Particle system | missing | L-01 in `Markdown/15`, parked by D-179; the charter leaves particles out (D-309) | Particle Playground, CC Particle World / Systems II, CC Pixel Polly, Foam, CC Bubbles, CC Star Burst, CC Drizzle |
| P0-7 | Text engine | partial | D-263..D-266, `src/text.rs`, tests d263_text, d264_text_styles: font outlines, tracking, kerning, styles, stroke, box, shadow. No text animators, no complex-script shaping | Numbers, Timecode, Basic/Path Text |
| P0-8 | Masks and paths | done | `src/mask.rs` (B-06, B-24b, D-77, mask keys D-298), FX-MSK-001..035; shapes `src/shape.rs` (D-78, D-168..D-170), FX-SHP-001..127 | Stroke, Scribble, Vegas, Fill, Write-on, Matte refinement |
| P0-9 | Float / HDR working depth | done | D-319/B-200 float depth, D-330/B-210 8 bpc rounding, D-333/B-214 AE 32 bpc; FX-8BPC-001..005, FX-AE32-001..006 | Exposure, Glow, every grade |
| P0-10 | Effect Mix | done | D-202/B-137, FX-MIX-001..017 | every effect |
| P0-11 | Adjustment layers | done | B-17b (FX-ADJ-001..013), blend mode on adjustment layers D-297/B-182, card B-156 | every effect used as a grade |
| P0-12 | Camera and 3D layers | partial | D-58/B-13c camera with each layer on a depth plane, D-171/B-111 camera rig. No true 3D layers or lights (left out by the charter, D-309) | CC Sphere, CC Cylinder, CC Environment, 3D Glasses, Card Wipe, Card Dance |
| P0-13 | Motion tracking / optical flow | missing | L-04 in `Markdown/15`; the charter leaves the tracker out (D-309) | Warp Stabilizer, Timewarp, Pixel Motion Blur, 3D Camera Tracker, Rolling Shutter Repair |
| P0-14 | Reading audio | done | `src/audio.rs`, D-71/ADR-018, B-20b, FX-AUD-001..010 | Audio Spectrum, Audio Waveform |
| P0-15 | Whole-frame statistics (histogram, average colour) | missing | no reduction pass exists in `src/gpu.rs` or the CPU path | Auto Color, Auto Contrast, Auto Levels, Equalize, Color Stabilizer, Color Link |
| P0-16 | Keeping unknown effects | done | `Effect::Unsupported { type_id }` in `src/effects.rs`, document 28 | every missing row: a project naming one is kept and diagnosed, not dropped |
| P0-17 | 3D surface simulation | missing | L-02 in `Markdown/15` | Shatter, Card Dance, Caustics, Wave World |
| P0-18 | Mesh and brush warping | missing | L-05 in `Markdown/15` | Liquify, Mesh Warp, Reshape, Bezier Warp, Warp |
| P0-19 | Shared noise basis | done | `core.fractal_noise` (D-128, D-299, D-318, D-326), FX-FRACTAL-001..038; reused by Turbulent Displace | Turbulent Noise, Curl Noise, Noise HLS, Add Grain, Fractal |
| P0-20 | Matte refinement kit (choke, feather, edge-aware smoothing) | partial | `core.simple_choker` (D-159, FX-CHOKE-001..017), mask feather; no guided filter, no edge-aware smoothing | Matte Choker, Refine Soft/Hard Matte, Key Cleaner, our keyer |

## Matte

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Matte | Simple Choker — grows or shrinks the alpha edge by a set amount (`core.simple_choker`) | done | P0-20 | On card (done) | Target P2; measured CPU 37.0 / GPU 24.8 ms (B-107) | FX-CHOKE-001..017, b102, B-102 table, D-159 |
| Matte | Matte Choker — two-stage choke that first spreads then chokes alpha, with geometric softness, to close holes | missing | P0-20, Simple Choker | Add card pass (two dilate/erode passes) | Target P2 | none yet |
| Matte | Refine Soft Matte — rebuilds a soft edge (hair, motion blur) along a matte's boundary, with decontamination | missing | P0-20, P0-4 (motion blur option) | Add card pass (edge-aware filter) | Target P3 | none yet |
| Matte | Refine Hard Matte — smooths and stabilises a hard matte edge, with optional decontamination | missing | P0-20 | Add card pass | Target P3 | none yet |
| Matte | mocha shape — reads tracked shapes from the bundled Mocha plug-in as a matte | missing | P0-13, P0-8 | not planned (third-party tracker data) | n/a | none |

## Blur & Sharpen

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Blur & Sharpen | Bilateral Blur — blurs flat areas while keeping strong edges, by weighting neighbours by colour likeness | missing | P0-2 | Add card pass | Target P3 | none yet |
| Blur & Sharpen | Camera Lens Blur — blur shaped like a camera iris, with bright-spot highlights and an optional depth map (`core.lens_blur`) | partial | P0-5 for the blur map | On card (done) | Target P3; measured CPU 39.0 / GPU 26.2 ms (B-65) | FX-LENS-001..044, b59, b64, B-59 and B-64 tables, D-116, D-121. Limit: iris and highlights built; no blur map or depth input |
| Blur & Sharpen | Camera-Shake Deblur — finds blurry frames from shaky footage and swaps in sharper neighbours | missing | P0-13, P0-4 | not planned | n/a | none |
| Blur & Sharpen | CC Cross Blur — separate horizontal and vertical box blurs with a transfer mode | missing | Fast Box Blur | Add card pass | Target P2 | none yet |
| Blur & Sharpen | CC Radial Blur — spin or zoom blur around a point, with a fading or brightening option | missing | Radial Blur | Reuse Radial Blur card pass | Target P3 | none yet |
| Blur & Sharpen | CC Radial Fast Blur — quick zoom-style streaks from a centre, with brighten or darken modes | missing | Radial Blur | Reuse Radial Blur card pass | Target P2 | none yet |
| Blur & Sharpen | CC Vector Blur — blurs each pixel along a direction taken from the image or a map layer (`core.vector_blur`) | partial | P0-3 | Add card pass | Target P3 | built, fixtures FX-VBLUR-001..027, awaiting playtest; b220, D-336 table |
| Blur & Sharpen | Channel Blur — blurs red, green, blue and alpha by separate amounts (`core.channel_blur`) | partial | none | Add card pass (reuse Gaussian) | Target P2 | b195, D-312_D-313 file, D-313. No fixture folder |
| Blur & Sharpen | Compound Blur — blur amount taken per pixel from a second layer's brightness (`core.compound_blur`) | done | P0-3 | Add card pass | Target P3 | FX-CBLUR-001..026, b127, B-127 table, D-191 |
| Blur & Sharpen | Directional Blur — streaks the image along one angle (`core.directional_blur`) | done | none | On card (done) | Target P2; measured B-49 full quality CPU 56.2 / GPU 26.7 ms | FX-DIRBLUR-001..015, b36, B-36 table, D-92 |
| Blur & Sharpen | Fast Box Blur — repeated box passes that approach a Gaussian, with per-axis and edge options (`core.fast_box_blur`) | done | none | Add card pass | Target P2 | FX-FASTBOX-001..011, b208, D-327 table |
| Blur & Sharpen | Gaussian Blur — smooth bell-shaped blur, with horizontal/vertical choice and edge repeat (`core.gaussian_blur`) | done | none | On card (done); one-way blur on CPU | Target P2; measured B-50 three blurs CPU 58.4 / GPU 29.3 ms | FX-BLURRY-001..009, b07, b188, b205, B-07 table, D-109, D-303, D-321 |
| Blur & Sharpen | Radial Blur — spin or zoom blur about a centre (`core.radial_blur`) | done | none | On card (done) | Target P3; measured B-46 CPU 160.5 / GPU 24.8 ms | FX-RADIAL-001..018, b39, B-39 table, D-95, D-110 |
| Blur & Sharpen | Sharpen — boosts contrast between neighbouring pixels (`core.sharpen`) | done | none | On card (done) | Target P2; measured CPU 37.5 / GPU 24.4 ms (B-107) | FX-SHARPEN-001..018, b90, B-90 table, D-147 |
| Blur & Sharpen | Smart Blur — blurs only where neighbours are within a threshold, keeping edges (`core.smart_blur`) | done | none | On card (done) | Target P2; measured CPU 22.0 / GPU 9.7 ms (B-151) | FX-SMART cases in median_smart_blur, b138, B-138 table, D-203 |
| Blur & Sharpen | Unsharp Mask — sharpens by adding back the difference from a blurred copy, above a threshold (`core.sharpen` with threshold) | partial | Sharpen | On card without threshold; threshold runs on CPU, add to card | Target P2 | b198, D-317_unsharp_glass.md, D-317. Threshold has hand-worked tests only, no fixture |

## Color Correction

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Color Correction | Auto Color — neutralises casts by stretching shadows, mids and highlights from the frame's own statistics | missing | P0-15 | Add card reduction pass, then a P1 pass | Target P2 | none yet |
| Color Correction | Auto Contrast — stretches overall contrast from the frame's darkest and brightest values | missing | P0-15 | Add card reduction pass | Target P2 | none yet |
| Color Correction | Auto Levels — stretches each colour channel separately to the frame's range | missing | P0-15 | Add card reduction pass | Target P2 | none yet |
| Color Correction | Black & White — turns colour to grey with a slider per hue family, plus optional tint (`core.black_white`) | done | none | On card (done), fused | Target P1; measured CPU 37.4 / GPU 25.9 ms (B-107) | FX-BW-001..022, b79, B-79 table, D-136 |
| Color Correction | Brightness & Contrast — simple brightness offset and contrast around the middle (`core.brightness_contrast`) | done | none | On card (done), fused | Target P1; measured CPU 38.3 / GPU 25.8 ms (B-107) | FX-BRICON-001..022, b78, B-78 table, D-135 |
| Color Correction | Broadcast Colors — limits brightness or saturation to safe broadcast levels | missing | none | Add card pass (P1) | Target P1 | none yet |
| Color Correction | CC Color Neutralizer — balances shadows, mids and highlights toward neutral grey | missing | none | Add card pass (P1) | Target P1 | none yet |
| Color Correction | CC Color Offset — rotates each colour channel's phase for psychedelic shifts | missing | none | Add card pass (P1) | Target P1 | none yet |
| Color Correction | CC Kernel — applies a small 3x3 convolution grid the user types in | missing | none | Add card pass | Target P2 | none yet |
| Color Correction | CC Toner — maps brightness onto two, three or five chosen tones | missing | Gradient Map | Reuse Gradient Map card pass | Target P1 | none yet |
| Color Correction | Change Color — shifts hue, lightness and saturation of one picked colour range | missing | Change to Color | Add card pass (P1) | Target P1 | none yet |
| Color Correction | Change to Color — replaces one picked colour range with another colour (`core.change_to_color`) | done | none | Add card pass (P1) | Target P1 | FX-CTC-001..027, b132, B-132 table, D-197 |
| Color Correction | Channel Mixer — builds each output channel from a mix of the input channels (`core.channel_mixer`) | done | none | On card (done), fused | Target P1; measured CPU 37.5 / GPU 27.4 ms (B-107) | FX-MIXER-001..022, b82, B-82 table, D-139 |
| Color Correction | Color Balance — shifts red, green and blue separately in shadows, mids and highlights (`core.color_balance`) | done | none | On card (done), fused | Target P1; measured CPU 39.6 / GPU 25.6 ms (B-76) | FX-BALANCE-001..019, b73, b180, B-73 table, D-130, D-295 |
| Color Correction | Color Balance (HLS) — offsets hue, lightness and saturation for the whole image | missing | Hue/Saturation | Add card pass (P1) | Target P1 | none yet |
| Color Correction | Color Link — tints a layer toward another layer's average or chosen statistic | missing | P0-15, P0-3 | Add card reduction pass | Target P2 | none yet |
| Color Correction | Color Stabilizer — locks brightness or colour to a sampled reference frame to remove flicker | missing | P0-15, P0-4 | CPU first | Target P4 | none yet |
| Color Correction | Colorama — cycles colours through a palette driven by a chosen input (hue, brightness, etc.) (`core.colorama`) | partial | none | Add card pass (P1) | Target P1 | b197, D-316_colorama.md, D-316. Reduced control set, no fixture folder |
| Color Correction | Curves — reshapes tone with a curve for master, each colour and alpha (`core.curves`) | done | none | On card (done); alpha curve on CPU | Target P1; measured CPU 41.5 / GPU 25.7 ms (B-65) | FX-CURVES-001..020, b54, b187, B-54 table, D-111, D-302 |
| Color Correction | Equalize — evens out brightness across the histogram | missing | P0-15 | Add card reduction pass | Target P2 | none yet |
| Color Correction | Exposure — photographic exposure in stops, with offset and gamma, in linear light (`core.exposure`) | done | P0-9 | Add card pass (P1) | Target P1 | FX-EXPAE-001..016, b07, b217, D-335 table |
| Color Correction | Gamma/Pedestal/Gain — per-channel gamma, black lift and gain | missing | Levels | Add card pass (P1) | Target P1 | none yet |
| Color Correction | Hue/Saturation — rotates hue and scales saturation and lightness, overall or per colour range (`core.hue_saturation`) | done | none | On card (done); colour ranges on CPU | Target P1; measured CPU 39.0 / GPU 27.1 ms (B-65) | FX-HUESAT-001..023, b56, b192, B-56 table, D-113, D-307 |
| Color Correction | Leave Color — removes colour from everything except one picked hue (`core.leave_color`) | done | none | On card (done), fused | Target P1; measured CPU 38.5 / GPU 25.2 ms (B-107) | FX-LEAVE-001..022, b84, B-84 table, D-141 |
| Color Correction | Levels — remaps input black/white and gamma to output black/white (`core.levels`) | partial | none | On card (done) | Target P1; measured CPU 38.7 / GPU 26.2 ms (B-65) | FX-LEVELS-001..024, b55, B-55 table, D-112. Limit: one setting for all of RGB; no per-channel or alpha choice |
| Color Correction | Levels (Individual Controls) — the same remap with separate controls for each channel and alpha | missing | Levels | Reuse Levels card pass | Target P1 | none yet |
| Color Correction | Lumetri Color — full colour-grading panel: basic correction, creative look, curves, wheels, HSL secondary, vignette | missing | Curves, Color Balance, Hue/Saturation, Vignette, Color Lookup | Combine existing card passes | Target P2 | none yet |
| Color Correction | Photo Filter — mimics a coloured lens filter while optionally keeping brightness | missing | none | Add card pass (P1) | Target P1 | none yet |
| Color Correction | PS Arbitrary Map — applies a Photoshop arbitrary-map curve file | missing | Curves | Reuse Curves card pass | Target P1 | none yet |
| Color Correction | Selective Color — adjusts cyan, magenta, yellow and black amounts within chosen colour families | missing | none | Add card pass (P1) | Target P1 | none yet |
| Color Correction | Shadow/Highlight — lifts shadows and recovers highlights using local brightness | missing | Gaussian Blur | Add card pass (blur plus tone) | Target P2 | none yet |
| Color Correction | Tint — maps dark pixels to one colour and light pixels to another, by an amount (`core.tint`) | partial | none | Add card pass (P1) | Target P1 | FX-E/FX-T cases in b07, B-07 table, document 21. Limit: ours is one colour plus amount, not AE's black-to-white two-colour map |
| Color Correction | Tritone — maps shadows, midtones and highlights to three chosen colours (via `core.gradient_map`) | partial | none | On card (done) | Target P1; measured CPU 41.3 / GPU 25.0 ms (B-76) | FX-GRADMAP-001..023, b72, B-72 table, D-129. Covered by a three-stop Gradient Map, not a Tritone with its own controls |
| Color Correction | Vibrance — raises saturation of muted colours more than strong ones (`core.vibrance`) | done | none | On card (done), fused | Target P1; measured CPU 40.3 / GPU 27.9 ms (B-107) | FX-VIBRANCE-001..018, b83, B-83 table, D-140 |

## Distort

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Distort | Bezier Warp — bends a layer's outline with curved edges and corner handles | missing | P0-18 | Add card pass (mesh) | Target P2 | none yet |
| Distort | Bulge — swells or pinches an area like a lens (`core.bulge`) | done | none | On card (done); separate or tapered radii on CPU | Target P2; measured CPU 37.8 / GPU 28.4 ms (B-107) | FX-BULGE-001..023, b95, b193, B-95 table, D-152, D-310 |
| Distort | CC Bend It — bends a layer between two points like a strip | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Bender — bends a layer from a base point toward a top point | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Blobbylize — melts the image into soft blobs driven by a layer's brightness | missing | P0-3 | Add card pass | Target P3 | none yet |
| Distort | CC Flo Motion — pulls pixels toward or away from two knot points | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Griddler — slices the image into a grid of rotated, offset tiles | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Lens — strong fish-eye style lens distortion with size and convergence | missing | Bulge | Add card pass | Target P2 | none yet |
| Distort | CC Page Turn — curls a corner of the layer like a turning page, with a back side | missing | P0-3 (back page) | Add card pass | Target P2 | none yet |
| Distort | CC Power Pin — four-corner pin with perspective and expansion controls | missing | Corner Pin | Reuse Corner Pin card pass | Target P2 | none yet |
| Distort | CC Ripple Pulse — a single pulse of ripples timed by keyframes | missing | Ripple | Reuse Ripple card pass | Target P2 | none yet |
| Distort | CC Slant — skews the layer with optional stretching and floor | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Smear — drags pixels from one point toward another | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Split — tears the image open along a line between two points | missing | none | Add card pass | Target P2 | none yet |
| Distort | CC Split 2 — like CC Split with separate control of each side | missing | CC Split | Add card pass | Target P2 | none yet |
| Distort | CC Tiler — repeats the layer as scaled tiles | missing | Motion Tile | Reuse Motion Tile card pass | Target P2 | none yet |
| Distort | Corner Pin — moves the four corners to fit the layer into a quad (`core.corner_pin`) | done | none | On card (done) | Target P2; measured CPU 22.1 / GPU 9.6 ms (B-151) | FX-PIN-001..018, b133, B-133 table, D-198 |
| Distort | Detail-preserving Upscale — enlarges footage while keeping edges sharp | missing | none | Add card pass | Target P3 | none yet |
| Distort | Displacement Map — moves pixels by amounts read from another layer's channels (`core.displacement_map`) | done | P0-3 | Add card pass | Target P2 | FX-DMAP-001..031, b128, b196, B-128 table, D-193, D-315 |
| Distort | Liquify — brush-painted push, twirl, pucker and bloat distortions | missing | P0-18 | Add card pass (mesh) | Target P2 | none yet; L-05 |
| Distort | Magnify — a circular or square magnifier over part of the image | missing | none | Add card pass | Target P2 | none yet |
| Distort | Mesh Warp — distorts the layer by dragging a grid of points | missing | P0-18 | Add card pass (mesh) | Target P2 | none yet; L-05 |
| Distort | Mirror — reflects the image across a line at any angle (`core.mirror`) | done | none | On card (done) | Target P2; measured CPU 37.9 / GPU 24.8 ms (B-107) | FX-MIRROR-001..024, b96, B-96 table, D-153 |
| Distort | Offset — shifts the image with wrap-around (`core.offset`) | done | none | On card (done) | Target P2; measured CPU 41.3 / GPU 26.5 ms (B-76) | FX-OFFSET-001..023, b74, B-74 table, D-131 |
| Distort | Optics Compensation — adds or removes barrel lens distortion by field of view (`core.optics_compensation`) | done | none | On card (done) | Target P2; measured CPU 22.1 / GPU 10.9 ms (B-151) | FX-OPTICS-001..020, b145, B-145 table, D-210 |
| Distort | Polar Coordinates — converts between rectangular and polar layouts (`core.polar_coordinates`) | done | none | On card (done) | Target P2; measured CPU 21.6 / GPU 9.6 ms (B-151) | FX-POLAR-001..022, b136, b201, B-136 table, D-201, D-320 table |
| Distort | Reshape — morphs the area inside one mask into the shape of another | missing | P0-8, P0-18 | Add card pass (mesh) | Target P2 | none yet |
| Distort | Ripple — circular waves spreading from a centre (`core.ripple`) | done | none | On card (done) | Target P2; measured CPU 105.1 / GPU 34.3 ms (B-107) | FX-RIPPLE-001..026, b93, B-93 table, D-150 |
| Distort | Rolling Shutter Repair — straightens skew from rolling-shutter cameras | missing | P0-13 | not planned | n/a | none |
| Distort | Smear — moves a masked region along a path, stretching the surroundings | missing | P0-8, P0-18 | Add card pass | Target P2 | none yet |
| Distort | Spherize — wraps the image around a sphere-like bulge | missing | Bulge | Reuse Bulge card pass | Target P2 | none yet |
| Distort | Transform — the layer's position, scale, rotation, skew and opacity as an effect, with its own motion blur | missing | P0-1 (layer transform exists) | Add card pass | Target P2 | none yet |
| Distort | Turbulent Displace — warps the image with fractal noise in several styles (`core.turbulent_displace`) | done | P0-19 | On card (done); one-way and pinned modes on CPU | Target P3; measured CPU 225.9 / GPU 53.7 ms (B-76) | FX-TURB-001..026, b70, b191, b212, B-70 table, D-127, D-306, D-328 table |
| Distort | Twirl — rotates pixels more toward the centre than the edge (`core.twirl`) | done | none | On card (done) | Target P2; measured CPU 41.4 / GPU 27.7 ms (B-107) | FX-TWIRL-001..021, b94, B-94 table, D-151 |
| Distort | Warp — preset bends (arc, flag, bulge, fish and more) like Illustrator's | missing | P0-18 | Add card pass | Target P2 | none yet |
| Distort | Warp Stabilizer — analyses shake and smooths or locks camera motion | missing | P0-13 | not planned | n/a | none yet; L-04 |
| Distort | Wave Warp — travelling waves of chosen shape across the image (`core.wave_warp`) | done | none | On card (done) | Target P2; measured CPU 94.6 / GPU 35.4 ms (B-107) | FX-WAVE-001..024, b92, B-92 table, D-149 |

## Generate

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Generate | 4-Color Gradient — smooth blend between four coloured points (`core.four_color_gradient`) | done | none | Add card pass (P1) | Target P1 | FX-4CG-001..026, b143, B-143 table, D-208 |
| Generate | Advanced Lightning — branching electric bolts with glow, forks and obstacle options (`core.lightning_bolt`) | done | none | Add card pass | Target P3 | FX-BOLT-001..032 plus FX-LIGHTX, FX-LIGHTA, FX-LCORE, FX-LFORK, FX-LFULL; b126, b185, b203, b211, b215, b218, b219; B-126 table, D-190, D-300, D-324, D-329, D-334, D-338, D-339 tables |
| Generate | Audio Spectrum — draws frequency bars or lines from a sound layer | missing | P0-14 | Add card pass | Target P2 | none yet |
| Generate | Audio Waveform — draws the sound wave of an audio layer along a path | missing | P0-14, P0-8 | Add card pass | Target P2 | none yet |
| Generate | Beam — a glowing laser between two points with length and timing (`core.beam`) | done | none | Add card pass | Target P1 | FX-BEAM-001..029, b142, B-142 table, D-207 |
| Generate | CC Glue Gun — paints a blobby glossy stroke along a path | missing | P0-8 | Add card pass | Target P2 | none yet |
| Generate | CC Light Burst 2.5 — bright rays bursting outward from a point, taken from the image | missing | Light Rays | Reuse Light Rays card pass | Target P3 | none yet |
| Generate | CC Light Rays — radial light shafts streaming from a point through the image (`core.light_rays`) | partial | none | On card (done) | Target P3; measured CPU 39.8 / GPU 26.2 ms (B-76) | FX-RAYS-001..024, b67, B-67 table, D-124. Built in the spirit of CC Light Rays, not matched control for control |
| Generate | CC Light Sweep — a moving band of light across the layer (`core.light_sweep`) | done | none | Add card pass (P1) | Target P1 | FX-SWEEP-001..026, b134, B-134 table, D-199 |
| Generate | CC Threads — woven thread pattern made from the image's colours | missing | none | Add card pass | Target P2 | none yet |
| Generate | Cell Pattern — cellular noise patterns (bubbles, crystals, plates) (`core.cell_pattern`) | done | none | On card (done) | Target P2; measured CPU 22.0 / GPU 9.5 ms (B-151) | FX-CELL-001..033, b144, B-144 table, D-209 |
| Generate | Checkerboard — draws a checker grid | missing | none | Add card pass (P1) | Target P1 | none yet |
| Generate | Circle — draws a filled circle or ring | missing | none | Add card pass (P1) | Target P1 | none yet |
| Generate | Ellipse — draws a soft-edged ellipse outline | missing | none | Add card pass (P1) | Target P1 | none yet |
| Generate | Eyedropper Fill — fills the layer with a colour sampled from an area | missing | P0-15 | Add card reduction pass | Target P1 | none yet |
| Generate | Fill — fills the layer or masks with one colour | missing | P0-8 | Add card pass (P1) | Target P1 | none yet |
| Generate | Fractal — draws Mandelbrot or Julia set images | missing | none | Add card pass | Target P2 | none yet |
| Generate | Gradient Ramp — linear or radial blend between two colours (`core.gradient`) | done | none | On card (done) | Target P1; measured CPU 38.9 / GPU 26.0 ms (B-65) | FX-GRAD-001..022, b57, B-57 table, D-114 |
| Generate | Grid — draws a grid of lines | missing | none | Add card pass (P1) | Target P1 | none yet |
| Generate | Lens Flare — simulated camera flare from a bright point | missing | none | Add card pass | Target P2 | none yet |
| Generate | Paint Bucket — flood-fills an area of similar colour | missing | none | CPU first (flood fill is serial) | Target P4 | none yet |
| Generate | Radio Waves — rings that spread outward from a point over time (`core.radio_waves`) | done | none | Add card pass | Target P2 | FX-RWAVE-001..027, b135, B-135 table, D-200 |
| Generate | Scribble — fills a mask with animated scribbled strokes | missing | P0-8 | Add card pass | Target P2 | none yet |
| Generate | Stroke — draws along a mask path, with write-on start and end | missing | P0-8 | Add card pass | Target P2 | none yet |
| Generate | Vegas — runs moving dashes along edges or a mask path | missing | P0-8 | Add card pass | Target P2 | none yet |
| Generate | Write-on — paints a brush stroke along animated positions | missing | P0-1 | Add card pass | Target P2 | none yet |

## Noise & Grain

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Noise & Grain | Add Grain — film-like grain with presets for size, softness and colour | missing | P0-19 | Add card pass | Target P2 | none yet |
| Noise & Grain | Curl Noise (new in AE 26.3, June 2026) — flowing, swirling 2D noise that moves like smoke, ink or water rather than flickering | missing | P0-19 | Add card pass (curl of a noise field) | Target P3 | none yet |
| Noise & Grain | Dust & Scratches — removes small specks by replacing pixels unlike their neighbours | missing | Median | Reuse Median card pass with threshold | Target P2 | none yet |
| Noise & Grain | Fractal Noise — layered cloud-like noise with evolution, many types and blend modes (`core.fractal_noise`) | done | P0-19 | On card (done) | Target P3; measured CPU 122.7 / GPU 38.7 ms (B-76) | FX-FRACTAL-001..038, b71, b184, B-71 table, D-128, D-299, D-318, D-326 |
| Noise & Grain | Match Grain — measures grain in one layer and adds matching grain to another | missing | P0-3, P0-15 | CPU first | Target P4 | none yet |
| Noise & Grain | Median — replaces each pixel with the middle value of its neighbours (`core.median`) | done | none | On card (done) | Target P2; measured CPU 22.1 / GPU 9.6 ms (B-151) | FX-MEDIAN cases in median_smart_blur, b138, B-138 table, D-203 |
| Noise & Grain | Noise — adds random speckle, optionally coloured (`core.noise`) | done | none | On card (done) | Target P1; measured CPU 82.7 / GPU 37.6 ms (B-65) | FX-NOISE-001..018, b62, B-62 table, D-119 |
| Noise & Grain | Noise Alpha — adds random noise to the alpha channel | missing | Noise | Add card pass (P1) | Target P1 | none yet |
| Noise & Grain | Noise HLS — adds noise to hue, lightness and saturation | missing | Noise | Add card pass (P1) | Target P1 | none yet |
| Noise & Grain | Noise HLS Auto — Noise HLS that animates by itself | missing | Noise HLS | Add card pass (P1) | Target P1 | none yet |
| Noise & Grain | Remove Grain — reduces grain or noise while keeping detail | missing | P0-15 | Add card pass | Target P3 | none yet |
| Noise & Grain | Turbulent Noise — a faster variant of fractal noise with fewer controls | missing | P0-19, Fractal Noise | Reuse Fractal Noise card pass | Target P2 | none yet |

## Stylize

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Stylize | Brush Strokes — repaints the image as painterly strokes | missing | none | Add card pass | Target P3 | none yet |
| Stylize | Cartoon — flattens colours and adds dark edges for a cel look | missing | Posterize, Find Edges, Smart Blur | Combine card passes | Target P3 | none yet |
| Stylize | CC Block Load — reveals the image in progressive coarse-to-fine blocks | missing | Mosaic | Reuse Mosaic card pass | Target P2 | none yet |
| Stylize | CC Burn Film — burning-film holes that spread over time | missing | P0-19 | Add card pass | Target P2 | none yet |
| Stylize | CC Glass — glassy relief lit from a direction, using a layer as the bump (`core.glass`) | partial | P0-3 | Add card pass | Target P2 | b198, D-317_unsharp_glass.md, D-317. Reduced control set, no fixture folder |
| Stylize | CC HexTile — repeats the image in hexagonal tiles | missing | none | Add card pass | Target P2 | none yet |
| Stylize | CC Kaleida — kaleidoscope mirroring around a centre (`core.kaleidoscope`) | done | none | Add card pass | Target P2 | FX-KALEIDO-001..020, b140, B-140 table, D-205 |
| Stylize | CC Mr. Smoothie — smears colours along flow lines taken from a layer | missing | P0-3 | Add card pass | Target P3 | none yet |
| Stylize | CC Plastic — shiny plastic relief from a bump layer | missing | P0-3, Bevel Alpha | Add card pass | Target P2 | none yet |
| Stylize | CC RepeTile — repeats the layer outward with mirror or tile modes | missing | Motion Tile | Reuse Motion Tile card pass | Target P2 | none yet |
| Stylize | CC Threshold — two-level threshold with blending and softness | missing | Threshold | Reuse Threshold card pass | Target P1 | none yet |
| Stylize | CC Threshold RGB — threshold applied to each colour channel separately | missing | Threshold | Reuse Threshold card pass | Target P1 | none yet |
| Stylize | CC Vignette — darkens toward the edges around a centre (`core.vignette`) | partial | none | On card (done), fused | Target P1; measured CPU 40.7 / GPU 26.0 ms (B-76) | FX-VIGNETTE-001..025, b69, B-69 table, D-126. Ours follows a Lumetri-style vignette, not CC Vignette's controls |
| Stylize | Color Emboss — embossed relief that keeps the original colours (`core.emboss` in colour mode) | done | Emboss | On card (done) | Target P2; measured CPU 37.1 / GPU 24.3 ms (B-107) | FX-EMBOSS-001..026, b88, B-88 table, D-145 |
| Stylize | Contour Paint (in AE beta; recent, version unconfirmed) — follows the shapes in the frame and smears and posterizes pixels along them for a painted look | missing | Posterize, Find Edges | Add card pass | Target P3 | none yet |
| Stylize | Emboss — grey relief that makes edges look raised (`core.emboss`) | done | none | On card (done) | Target P2; measured CPU 37.1 / GPU 24.3 ms (B-107) | FX-EMBOSS-001..026, b88, B-88 table, D-145 |
| Stylize | Find Edges — outlines strong edges like a line drawing (`core.find_edges`) | done | none | On card (done) | Target P2; measured CPU 37.4 / GPU 25.4 ms (B-107) | FX-FINDEDGES-001..019, b89, B-89 table, D-146 |
| Stylize | Glow — finds bright areas and blooms light around them (`core.glow`) | done | P0-9 | On card (done) | Target P3; measured B-51 CPU 50.9 / GPU 17.8 ms | FX-GLOW-001..033, FX-GLDISP-001..008, b33, b206, b216, B-33b table, D-89, D-322, D-331, D-337 tables |
| Stylize | Mosaic — turns the image into flat-coloured blocks (`core.mosaic`) | done | none | On card (done) | Target P2; measured CPU 37.9 / GPU 24.3 ms (B-107) | FX-MOSAIC-001..020, b87, B-87 table, D-144 |
| Stylize | Motion Tile — repeats the layer as tiles with mirrored edges and phase (`core.motion_tile`) | done | none | On card (done); a sized output on CPU | Target P2 | FX-TILE-001..023, b97, b189, B-97 table, D-154, D-304 |
| Stylize | Posterize — reduces each channel to a few levels (`core.posterize`) | done | none | On card (done), fused | Target P1; measured CPU 38.4 / GPU 25.5 ms (B-107) | FX-POSTER-001..018, b80, B-80 table, D-137 |
| Stylize | Roughen Edges — eats into alpha edges with fractal noise, rust or spiky styles (`core.roughen_edges`) | done | P0-19 | On card (done) | Target P3; measured CPU 98.7 / GPU 17.9 ms (B-151) | FX-ROUGH-001..024, b141, B-141 table, D-206 |
| Stylize | Scatter — jumbles pixels randomly within a distance | missing | none | Add card pass | Target P2 | none yet |
| Stylize | Strobe Light — flashes colour or transparency on a beat | missing | none | Add card pass (P1) | Target P1 | none yet |
| Stylize | Texturize — embosses a texture layer onto the image | missing | P0-3, Emboss | Add card pass | Target P2 | none yet |
| Stylize | Threshold — turns the image pure black and white at a brightness level (`core.threshold`) | done | none | On card (done), fused | Target P1; measured CPU 37.8 / GPU 25.0 ms (B-107) | FX-THRESH-001..019, b81, B-81 table, D-138 |

## Transition

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Transition | Block Dissolve — layer vanishes in random blocks (`core.block_dissolve`) | done | none | Add card pass (P1) | Target P1 | FX-BDISSOLVE-001..019, b149, B-149 table, D-214 |
| Transition | Card Wipe — flips the layer as a grid of cards to reveal another | missing | P0-12, P0-3 | Add card pass | Target P3 | none yet |
| Transition | CC Glass Wipe — melts away like glass using a layer's brightness | missing | P0-3 | Add card pass | Target P2 | none yet |
| Transition | CC Grid Wipe — reveals through a grid of growing shapes | missing | none | Add card pass (P1) | Target P1 | none yet |
| Transition | CC Image Wipe — wipe timed by another layer's brightness | missing | Gradient Wipe | Reuse Gradient Wipe | Target P1 | none yet |
| Transition | CC Jaws — wipe with jagged teeth edges | missing | none | Add card pass (P1) | Target P1 | none yet |
| Transition | CC Light Wipe — wipe with a glowing edge in a chosen shape | missing | Iris Wipe, Glow | Add card pass | Target P2 | none yet |
| Transition | CC Line Sweep — reveals in sweeping stepped lines | missing | none | Add card pass (P1) | Target P1 | none yet |
| Transition | CC Radial ScaleWipe — circular wipe that magnifies at its edge | missing | Iris Wipe | Add card pass | Target P2 | none yet |
| Transition | CC Scale Wipe — stretches pixels in one direction as it wipes | missing | none | Add card pass | Target P2 | none yet |
| Transition | CC Twister — twists the layer around an axis to reveal its back | missing | P0-3 | Add card pass | Target P2 | none yet |
| Transition | CC WarpoMatic — morph-like warp between layers by brightness | missing | P0-3 | Add card pass | Target P3 | none yet |
| Transition | Gradient Wipe — reveals in the order of another layer's brightness (`core.gradient_wipe`) | done | P0-3 | Add card pass (P1) | Target P1 | FX-GWIPE-001..028, b129, B-129 table, D-194 |
| Transition | Iris Wipe — reveals through a growing polygon or star (`core.iris_wipe`) | done | none | On card (done) | Target P1; measured CPU 36.8 / GPU 24.6 ms (B-107) | FX-IRIS-001..027, b101, B-101 table, D-158 |
| Transition | Linear Wipe — straight-edged wipe at any angle (`core.linear_wipe`) | done | none | On card (done) | Target P1; measured CPU 37.9 / GPU 28.7 ms (B-107) | FX-LWIPE-001..032, b98, B-98 table, D-155 |
| Transition | Radial Wipe — clock-hand sweep around a centre (`core.radial_wipe`) | done | none | On card (done) | Target P1; measured CPU 42.0 / GPU 25.6 ms (B-107) | FX-RWIPE-001..029, b99, B-99 table, D-156 |
| Transition | Venetian Blinds — reveals through stripes that widen (`core.venetian_blinds`) | done | none | On card (done) | Target P1; measured CPU 36.8 / GPU 24.7 ms (B-107) | FX-BLINDS-001..026, b100, B-100 table, D-157 |

## Time

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Time | CC Force Motion Blur — motion blur made by sampling between frames, for layers with no keyframed movement | partial | P0-4 | Layer motion blur on card exists (D-226/B-156b); not as an effect | Target P4 | FX-MB-001..050, b124, B-124b table (layer motion blur, D-188). No effect form |
| Time | CC Wide Time — blends a set number of frames before and after | missing | P0-4, Echo | CPU first, card later | Target P4 | none yet |
| Time | Echo — overlays earlier or later frames as fading copies (`core.echo`) | done | P0-4 | CPU only (multi-frame); card later | Target P4 | FX-ECHO-001..030, b130, B-130 table, D-195 |
| Time | Pixel Motion Blur — motion blur from estimated per-pixel movement | missing | P0-13 | not planned | n/a | none yet |
| Time | Posterize Time — holds frames to a lower frame rate (`core.posterize_time`) | done | P0-4 | CPU only (picks a time; no pixel work) | Target P4 | FX-PTIME-001..020, b131, B-131 table, D-196 |
| Time | Time Difference — shows the difference between this layer and another at an offset time | missing | P0-4, P0-3 | CPU first | Target P4 | none yet |
| Time | Time Displacement — each pixel shows a different moment, chosen by a map layer's brightness | missing | P0-4, P0-3 | CPU first | Target P4 | none yet |
| Time | Timewarp — speed changes with optical-flow in-between frames | missing | P0-13 | not planned | n/a | none yet; L-04 |

## Keying

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Keying | Advanced Spill Suppressor — removes green or blue screen colour from foreground edges | missing | none | Add card pass (P1) | Target P1 | none yet; our keyer's despill below |
| Keying | CC Simple Wire Removal — paints out a thin wire between two points by stretching neighbours | missing | none | Add card pass | Target P2 | none yet |
| Keying | Color Difference Key — builds a matte from the difference between colour channels | missing | none | Add card pass | Target P2 | none yet; the core of our keyer below |
| Keying | Color Range — keys out a range of colours chosen in Lab, YUV or RGB | missing | Color Key | Add card pass (P1) | Target P1 | none yet |
| Keying | Difference Matte — keys out what matches a clean background plate | missing | P0-3 | Add card pass (P1) | Target P1 | none yet |
| Keying | Extract — keys out by brightness of a chosen channel, with softness (`core.extract`) | done | none | Add card pass (P1) | Target P1 | FX-EXTRACT-001..027, b147, B-147 table, D-212 |
| Keying | Inner/Outer Key — pulls a matte for fine edges from an inside and an outside mask | missing | P0-8, P0-20 | Add card pass | Target P3 | none yet |
| Keying | Key Cleaner — recovers edge detail lost by a key and reduces chatter | missing | P0-20 | Add card pass | Target P3 | none yet |
| Keying | Keylight — licensed third-party screen keyer. We will build our own keyer with similar controls instead (plan below) | missing | P0-20, Simple Choker, Light Wrap | Add card passes (see plan) | Target P2 | none yet; L-03 |
| Keying | Linear Color Key — keys by distance from a picked colour with tolerance and softness | missing | Color Key | Add card pass (P1) | Target P1 | none yet |
| Keying | Unmult (new in AE 26.0, January 2026) — turns a black or white background into transparency so fire or smoke footage composites cleanly | missing | P0-9 | Add card pass (P1) | Target P1 | none yet |

### Our own keyer (planned, not a Keylight clone)

Keylight is a licensed product, so we will not copy it. We will build our own screen keyer from openly published methods, with controls a Keylight user will recognise:

- **Screen colour**: pick the green or blue screen with the eyedropper.
- **Colour-difference matte**: the matte comes from how much the screen channel stands out above the other two (screen channel minus a weighted mix of the others). **Screen balance** sets that mix; **screen gain** scales the result.
- **Clip black / clip white**: levels on the matte, so near-solid areas become fully solid or fully clear.
- **Despill**: limit the screen channel to a blend of the other two channels, with a bias control, so green fringes turn neutral.
- **Matte refinement**: shrink/grow and soften (reusing `core.simple_choker`), plus an edge-aware guided filter (He, Sun and Tang, 2010) so hair and soft edges follow the picture.
- **View modes**: screen matte, status (solid, clear, in-between shown as flat colours), and final result.
- **Edge colour**: optional light wrap from the background, reusing `core.light_wrap`.

It builds on what `core.color_key`, `core.hsv_key` and `core.extract` already do. Its fixtures would come, as ours always do, from a second, independent implementation in `tools/*_reference.py`. It needs P0-20 (guided filter) first. This is roadmap item L-03.

## Perspective

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Perspective | 3D Camera Tracker — solves the camera from footage and places 3D markers | missing | P0-13, P0-12 | not planned | n/a | none; left out by the charter (D-309) |
| Perspective | 3D Glasses — combines left and right views into an anaglyph or side-by-side image | missing | P0-3 | Add card pass (P1) | Target P1 | none yet |
| Perspective | Bevel Alpha — raised edge along the alpha outline, lit from a direction (`core.bevel_alpha`) | done | none | On card (done) | Target P2; measured CPU 22.2 / GPU 11.0 ms (B-151) | FX-BEVEL cases, b148, B-148 table, D-213 |
| Perspective | Bevel Edges — chiselled rectangular bevel at the layer's edges (`core.bevel_edges`) | done | none | Add card pass | Target P1 | FX-BEVEL cases, b148, B-148 table, D-213 |
| Perspective | CC Cylinder — wraps the layer around a lit cylinder | missing | P0-12 | Add card pass | Target P2 | none yet |
| Perspective | CC Environment — maps an environment image onto the layer as reflection | missing | P0-12, P0-3 | Add card pass | Target P2 | none yet |
| Perspective | CC Sphere — wraps the layer around a lit sphere | missing | P0-12 | Add card pass | Target P2 | none yet |
| Perspective | CC Spotlight — a spotlight cone lighting the layer | missing | none | Add card pass (P1) | Target P1 | none yet |
| Perspective | Drop Shadow — offset, softened shadow from the alpha (`core.drop_shadow`) | done | none | On card (done) | Target P2; measured CPU 38.9 / GPU 25.0 ms (B-65) | FX-SHADOW-001..023, b58, B-58 table, D-115 |
| Perspective | Radial Shadow — shadow cast from a point light, with distance scaling (`core.radial_shadow`) | done | none | On card (done) | Target P2; measured CPU 21.8 / GPU 9.5 ms (B-151) | FX-RSHADOW-001..025, b146, B-146 table, D-211 |

## Simulation

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Simulation | Card Dance — the layer as a grid of 3D cards moved by maps | missing | P0-17, P0-12 | Add card pass | Target P5 | none yet; L-02 |
| Simulation | Caustics — light patterns of a rippling water surface | missing | P0-17 | Add card pass | Target P5 | none yet; L-02 |
| Simulation | CC Ball Action — rebuilds the image as a grid of 3D balls | missing | P0-12 | Add card pass | Target P5 | none yet |
| Simulation | CC Bubbles — floating bubbles that pick up the image | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | CC Drizzle — raindrop ripples on a surface | missing | P0-6, Ripple | Add card pass | Target P5 | none yet |
| Simulation | CC Hair — grows hair or fur from the layer | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | CC Mr. Mercury — blobby liquid metal particles | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | CC Particle Systems II — a 2D particle emitter | missing | P0-6 | Add card pass | Target P5 | none yet; L-01 |
| Simulation | CC Particle World — a 3D particle emitter with camera | missing | P0-6, P0-12 | Add card pass | Target P5 | none yet; L-01 |
| Simulation | CC Pixel Polly — shatters the layer into flying polygons | missing | P0-6, P0-12 | Add card pass | Target P5 | none yet; charter leaves it out (D-309) |
| Simulation | CC Rainfall — falling rain streaks with wind and depth (`core.rain`) | partial | none | On card (done) | Target P2; measured CPU 79.8 / GPU 33.8 ms (B-107) | FX-RAIN-001..027, b106, B-106 table, D-163. Built in the spirit of CC Rainfall, not matched control for control |
| Simulation | CC Scatterize — blows the layer apart into scattered dots | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | CC Snowfall — falling snowflakes with wind and depth (`core.snowfall`) | done | none | On card (done) | Target P2; measured CPU 68.4 / GPU 18.1 ms (B-151) | FX-SNOW-001..029, b139, B-139 table, D-204 |
| Simulation | CC Star Burst — a field of stars rushing toward the viewer | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | Foam — simulated bubbles that flow and pop | missing | P0-6 | Add card pass | Target P5 | none yet |
| Simulation | Particle Playground — particle cannon, grid and layer-exploding particles with forces | missing | P0-6 | Add card pass | Target P5 | none yet; L-01 |
| Simulation | Shatter — breaks the layer into 3D pieces that fly apart | missing | P0-17, P0-12 | Add card pass | Target P5 | none yet; L-02 |
| Simulation | Wave World — simulated water surface used as a displacement map | missing | P0-17 | Add card pass | Target P5 | none yet; L-02 |

## 3D Channel

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| 3D Channel | 3D Channel Extract — shows depth, normals or other 3D passes as a visible picture | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |
| 3D Channel | Cryptomatte — picks objects by id from rendered Cryptomatte passes | missing | P0-5 | Add card pass | Target P2 | none yet |
| 3D Channel | Depth Matte — keeps pixels within a depth range | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |
| 3D Channel | Depth of Field — blurs by distance from a focal plane using a depth pass | missing | P0-5, Camera Lens Blur | Reuse Lens Blur card pass | Target P3 | none yet |
| 3D Channel | EXtractoR — pulls any named channel out of a multichannel EXR | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |
| 3D Channel | Fog 3D — adds fog that thickens with depth | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |
| 3D Channel | ID Matte — keeps pixels with a chosen object or material id | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |
| 3D Channel | IDentifier — reads object and material ids from a multichannel EXR | missing | P0-5 | Add card pass (P1) | Target P1 | none yet |

## Text

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Text | Numbers — draws numbers, dates or times, randomised or counting | missing | P0-7 | CPU draw (outlines), card composite | Target P2 | none yet |
| Text | Timecode — burns the layer's or composition's time onto the picture | missing | P0-7 | CPU draw (outlines), card composite | Target P2 | none yet |

## Obsolete

Every row here: skip unless needed.

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Obsolete | Basic 3D — swivels and tilts a layer in fake 3D; skip unless needed | missing | P0-12 | skip unless needed | n/a | none |
| Obsolete | Basic Text — old single-line text generator; skip unless needed | missing | P0-7 | skip unless needed | n/a | none |
| Obsolete | Color Key — keys out one colour with tolerance and edge feather (`core.color_key`); skip unless needed | done | none | no card path today; skip unless needed | n/a | FX-KEY-001..023, b41, B-41 table, D-97 |
| Obsolete | Fast Blur (Legacy) — old quick blur; skip unless needed (Fast Box Blur replaces it) | missing | Fast Box Blur | skip unless needed | n/a | none |
| Obsolete | Gaussian Blur (Legacy) — old Gaussian blur; skip unless needed (Gaussian Blur replaces it) | missing | Gaussian Blur | skip unless needed | n/a | none |
| Obsolete | Lightning — old simple bolt generator; skip unless needed (covered by `core.lightning_bolt`) | missing | Advanced Lightning | skip unless needed | n/a | none |
| Obsolete | Luma Key — keys out by brightness; skip unless needed (Extract covers most of it) | missing | Extract | skip unless needed | n/a | none |
| Obsolete | Path Text — old text along a path; skip unless needed | missing | P0-7, P0-8 | skip unless needed | n/a | none |
| Obsolete | Reduce Interlace Flicker — softens thin horizontal lines for interlaced video; skip unless needed | missing | none | skip unless needed | n/a | none |
| Obsolete | Spill Suppressor — old screen-colour despill; skip unless needed (our keyer's despill covers it) | missing | none | skip unless needed | n/a | none |

## Recent Adobe additions outside the categories above

These came up while checking 2023-2026 releases. They are not image effects in the audited categories, so they are not counted.

| Category | Effect (AE behaviour, in our words) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Audio | Compressor (new in AE 26.0, January 2026) — evens out loud and quiet sound | missing | P0-14 | not applicable (sound, not picture) | n/a | none; we do not play or process sound (`src/audio.rs`) |
| Audio | Distortion (new in AE 26.0, January 2026) — deliberately overdrives sound | missing | P0-14 | not applicable | n/a | none |
| Audio | Noise Gate (new in AE 26.0, January 2026) — silences sound below a level | missing | P0-14 | not applicable | n/a | none |
| Tool | Object Matte (new in AE 26.2, April 2026) — AI tool: click an object once and it is cut out and followed through the shot. A tool, not an Effects-menu entry | missing | P0-13 | not planned (AI segmentation) | n/a | none |

## Ours, not in Adobe's list

Effects we have that Adobe's audited categories do not contain. Some exist in AE categories this audit does not cover; that is noted.

| Category | Effect (what it does) | Status | Depends on | GPU plan | Perf target | Reference / test |
|---|---|---|---|---|---|---|
| Ours | Bloom — soft glow spread from bright areas, multi-scale (`core.bloom`) | done | P0-9 | On card (done) | Target P3; measured B-47 CPU 178.6 / GPU 27.8 ms | FX-BLOOM-001..028, b40, B-40 table, D-96 |
| Ours | Camera Shake — handheld-style random position and rotation (`core.camera_shake`) | done | none | On card (done) | Target P2; measured CPU 90.0 / GPU 34.3 ms (B-107) | FX-SHAKE-001..021, b105, B-105 table, D-162 |
| Ours | Chromatic Aberration — splits colour channels outward like a cheap lens (`core.chromatic_aberration`) | done | none | On card (done) | Target P2; measured CPU 38.8 / GPU 26.5 ms (B-65) | FX-CHROMA-001..016, b63, B-63 table, D-120 |
| Ours | Color Lookup — applies a .cube LUT (AE's Apply Color LUT, Utility category, not audited) (`core.color_lookup`) | done | none | On card (done) | Target P1 | FX-LUT-001..013, b118, B-118 table, D-182 |
| Ours | Cross Glare — star-shaped streaks from highlights (`core.cross_glare`) | done | none | On card (done) | Target P3; measured CPU 120.6 / GPU 24.6 ms (B-107) | FX-GLARE-001..029, b104, B-104 table, D-161 |
| Ours | Diffusion — soft dreamy blur blended over the original (`core.diffusion`) | done | none | On card (done) | Target P2; measured CPU 36.7 / GPU 24.3 ms (B-107) | FX-DIFFUSE-001..018, b91, B-91 table, D-148 |
| Ours | Distance Gradation — colour fades by distance from the line art (`core.distance_gradation`) | done | none | On card (done) | Target P2; measured CPU 38.3 / GPU 25.1 ms (B-76) | FX-DISTGRAD-001..024, b66, B-66 table, D-123 |
| Ours | Exposure Flicker — random brightness flicker like old film (`core.exposure_flicker`) | done | none | On card (done) | Target P1; measured CPU 80.8 / GPU 34.2 ms (B-76) | FX-FLICKER-001..022, b68, B-68 table, D-125 |
| Ours | Halftone — printed dot screen, screentone style (`core.halftone`) | done | none | On card (done) | Target P2; measured CPU 37.2 / GPU 25.0 ms (B-107) | FX-HALFTONE-001..025, b86, B-86 table, D-143 |
| Ours | HSV Key — keys by hue, saturation and value ranges (`core.hsv_key`) | done | none | On card (done) | Target P1 | FX-HSV-001..016, b120, B-120 table, D-184 |
| Ours | Invert — inverts colour or a channel (AE Channel category, not audited) (`core.invert`) | done | none | On card (done), fused | Target P1; measured CPU 39.0 / GPU 26.4 ms (B-107) | FX-INVERT-001..020, b77, B-77 table, D-134 |
| Ours | Kira-kira — sparkle stars on highlights (`core.kira_kira`) | done | none | On card (done) | Target P3 | FX-KIRA-001..029, b122, B-122 table, D-186 |
| Ours | Light Wrap — background light bleeding onto the layer's edges (`core.light_wrap`) | done | P0-3 | On card (done, own path) | Target P2; measured CPU 150.1 / GPU 53.6 ms (B-76) | FX-WRAP-001..024, b75, B-75 table, D-132 |
| Ours | Line Blur — blurs along drawn lines (`core.line_blur`) | done | none | On card (done) | Target P2 | FX-LBLUR-001..016, b119, B-119 table, D-183 |
| Ours | Line Recolor — recolours drawn lines (`core.line_recolor`) | done | none | Add card pass (P1) | Target P1 | FX-RECOLOR-001..018, b35, B-35 table, D-91 |
| Ours | Line Smooth — smooths jagged drawn lines (`core.line_smooth`) | done | none | Add card pass | Target P2 | FX-SMOOTH-001..022, b30, B-30b table, D-86 |
| Ours | Line Width — thickens or thins drawn lines (`core.line_width`) | done | none | Add card pass | Target P2 | FX-WIDTH-001..019, b38, B-38 table, D-94 |
| Ours | Outline — stroke outside the alpha edge, like a Photoshop layer stroke (`core.outline`) | done | none | On card (done) | Target P2; measured CPU 38.2 / GPU 25.1 ms (B-65) | FX-OUTLINE-001..020, b61, B-61 table, D-118 |
| Ours | Paraffin — soft light gradient wash over the frame (`core.paraffin`) | done | none | On card (done) | Target P1 | FX-PARA-001..023, b121, B-121 table, D-185 |
| Ours | Rim Light — coloured light along one side of the alpha edge (`core.rim_light`) | done | none | On card (done) | Target P2; measured CPU 39.0 / GPU 24.7 ms (B-65) | FX-RIM-001..028, b60, B-60 table, D-117 |
| Ours | Select Color — keeps only chosen colours (`core.select_color`) | done | none | Add card pass (P1) | Target P1 | FX-SELECT-001..016, b37, B-37 table, D-93 |
| Ours | Selective Color Blur — blurs only chosen colours (`core.selective_color_blur`) | done | none | Add card pass | Target P2 | FX-SELBLUR-001..033, b31, B-31b table, D-87, D-88 |
| Ours | Shift Channels — takes each output channel from a chosen input channel (AE Channel category, not audited) (`core.shift_channels`) | partial | none | Add card pass (P1) | Target P1 | b190, D-305_shift_channels.md. No fixture folder |
| Ours | Solarize — inverts tones above a level, like film solarisation (`core.solarize`) | done | none | On card (done), fused | Target P1; measured CPU 36.6 / GPU 26.5 ms (B-107) | FX-SOLAR-001..016, b85, B-85 table, D-142 |
| Ours | Solid Composite — composites the layer over a solid colour (AE Channel category, not audited) (`core.solid_composite`) | partial | none | Add card pass (P1) | Target P1 | b195, D-312_D-313 file, D-312. No fixture folder |
| Ours | Speed Lines — manga radial or parallel speed lines (`core.speed_lines`) | done | none | On card (done) | Target P2; measured CPU 80.8 / GPU 34.8 ms (B-107) | FX-SPEED-001..029, b103, B-103 table, D-160 |

## Sources

- Adobe, After Effects effect list (the page this audit follows): https://helpx.adobe.com/after-effects/desktop/apply-effects-and-animation-presets/effects-and-animation-presets/effect-list.html (refused automated fetch, HTTP 403; contents cross-checked by web search).
- Adobe, What's new in After Effects: https://helpx.adobe.com/after-effects/desktop/what-s-new/whats-new.html (lists the June 2026 release, 26.3, with the Curl Noise effect, and the September 2026 release, 26.5).
- CG Channel, "Adobe releases After Effects 26.0 with native Substance support" (Unmult; Gate, Compressor and Distortion audio effects): https://www.cgchannel.com/2026/01/adobe-releases-after-effects-26-0-with-native-substance-support/
- CG Channel, "Adobe releases After Effects 26.2" (Object Matte): https://www.cgchannel.com/2026/04/adobe-releases-after-effects-26-2/
- CG Channel, "Adobe releases After Effects 26.3" (Curl Noise, in the Noise & Grain category): https://www.cgchannel.com/2026/06/adobe-releases-after-effects-26-3/
- CG Channel, "Adobe releases After Effects 26.5, plus experimental new AI Assistant": https://www.cgchannel.com/2026/09/adobe-releases-after-effects-26-5-and-new-ai-assistant-in-beta/
- Adobe community, "New in Beta: Unmult effect" (Unmult in the Keying category): https://community.adobe.com/announcements-532/new-in-beta-unmult-effect-about-time-right-314503
- Adobe, After Effects Beta page (Contour Paint described as a beta effect): https://helpx.adobe.com/after-effects/desktop/what-s-new/after-effects-beta.html
- Adobe feature summaries for 2023, 2024 (24.2 to 24.4) and October 2024 (25.0), checked for new effects; none found beyond changes to existing effects: https://helpx.adobe.com/after-effects/desktop/using/whats-new/2023.html, https://helpx.adobe.com/after-effects/desktop/using/whats-new/2024-2.html, https://helpx.adobe.com/after-effects/using/whats-new/2025.html
- Guided filter for matte refinement: K. He, J. Sun, X. Tang, "Guided Image Filtering", ECCV 2010.
