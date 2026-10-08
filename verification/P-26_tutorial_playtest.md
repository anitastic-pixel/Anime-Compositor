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

## After D-328, D-329 and D-330, played again (2026-10-05)

The owner approved the three proposals on 2026-10-05. All three are built, each with its fixtures committed before its code:

- **D-330** a third working depth, **8 bpc (After Effects)**: every pixel is rounded to 8 bits after each effect, and the four blurs average display colours, as After Effects' 8 bpc does. Tutorial 2 now works in 8 bpc until its step F, as the video does.
- **D-329** Advanced Lightning's **Path**: Stop (as before, the default) or Go Round, and Alpha Obstacle from -100 to 100. Tutorial 2 keeps Stop: its bolt stops on the ground.
- **D-328** Turbulent Displace's **Units**: After Effects (new effects) or Classic (every older file). In After Effects units the push shrinks with a wave under 100 pixels, so tutorial 2's Amount 80 at Size 2 is a shimmer of 1.6 pixels instead of dust.

All five tutorials were then played again from an empty project to the last step in the test copy. Every step ran and none was refused.

| Tutorial | Steps | Passed before | Now fixed | Partly | Still open | Still open because of |
|---|---|---|---|---|---|---|
| 1 Shockwave | 24 | 14 | 9 | 1 | 0 | — |
| 2 Advanced Electric | 18 | 5 | 12 | 1 | 0 | — |
| 3 Colorful Glitch | 29 | 12 | 14 | 2 | 1 | charter (camera) |
| 4 Ultimate Lightsaber | 22 | 15 | 7 | 0 | 0 | — |
| 5 Film Riot Lightsaber | 12 | 7 | 5 | 0 | 0 | — |
| **All** | **105** | **53** | **47** | **4** | **1** | |

Tutorial 2, step by step where it changed:

- **Step E, the reflections (now fixed).** In 8 bpc the blur's faint edge rounds to nothing before Exposure lifts it, so only a small blue spot lights under the bolt, as in the tutorial. Before it was a glowing block.
- **The glow under Turbulent Displace (now fixed).** The bolt keeps its tight glow. The same glow layer drawn with Turbulent Displace on and off lights 29,889 and 29,777 pixels: the shimmer moves light, it does not remove it. Before, at Classic units, it broke into dust.
- **The gold final (partly).** At frame 20 the bolt is white-hot gold with a tight glow, as in the tutorial. Its colour changes from frame to frame because the tutorial puts `wiggle(9,3)` on the glow's Exposure: at frame 12 the glow layer is at its dimmest (brightest red 0.81) and the bolt reads orange; from frame 15 to 24 it is 10 to 40 times brighter and reads gold. Still different: after step F the tutorial works in 32 bpc and the ground reflection grows back to a large glowing block, where the tutorial still shows a small glow. With Exposure at +20.49 stops (1.4 million times), any depth that keeps the blur's faint edge lights the whole blur, and this program's sum of After Effects' 32 bpc arithmetic does too (D-330's proposal page). The video does not show what keeps it small, so it is recorded here rather than guessed.

Tutorials 1, 3, 4 and 5 draw as they did after D-331. Tutorial 5's Turbulent Displace is new, so it starts in After Effects units: Amount 5 at Size 60 now pushes 3 pixels instead of 5, and the blade's edge still wobbles gently.

Commits: f809ea6, c0006a3 (D-330), 3d4ecd6, d8ee516 (D-329), fb1a40c, fb1e1b9 (D-328).

## Tutorial 1 rebuilt from scratch (2026-10-06)

After the owner's "Lightsaber went well!", tutorial 1 (the shockwave) was rebuilt from scratch in the test copy, saved, and drawn outside the app from the saved file.

**One real fault found and fixed: D-332.** The saved shockwave would not open again ("expected an array" at Fractal Noise's Offset Turbulence). Any project with a key on one of 23 effect settings could be saved but not reopened, among them Offset Turbulence and Bulge's Vertical Radius and Taper Radius. Fixed and checked over every effect (`D-332_keyed_settings.md`).

**How close it comes now.** SW (the single ring) and SW 2 (nested, scaled, time-remapped) match the tutorial's look: a smoky ring around a dark hole. In SW 3 our middle stays fuller and brighter than the tutorial's until about frame 28. Switching parts off shows why: the three copies at 64%, 73% and 74% sit inside the big ring's hole. With them off, the middle at frame 24 drops from 0.18 to 0.02 brightness. Those scales are the tutorial's own steps as we played them. The tutorial's still is from an unknown frame, so whether After Effects fills the hole the same way at the same frame cannot be told from it. It is recorded here, not guessed.

**Known stand-ins, unchanged:**

- The small grey dot in SW 3's middle is the 19% white solid. It stands in for CC Pixel Polly's sparks; particles are left out by the charter. In the tutorial the solid shatters into sparks at once. Ours stays a dot, and switching that layer off removes it.
- Gradient Map stands in for VC Color Vibrance, a third-party plug-in.
- Nesting stands in for the tutorial's render and re-import.

The rebuilt project is in the owner's Downloads as `shockwave_tutorial1.json`.

## Tutorial 2 rebuilt from scratch (2026-10-06)

Tutorial 2 (Advanced Electric) was replayed in the test copy from an empty project, saved, reopened outside the app (`persist::load`, no error and no notices), and exported as a 26-frame MP4 (Street_Sidewalk, frames 0 to 25, 1920x1080 at 23.976 fps). Frames 12 and 20 were pulled from the MP4 with ffmpeg and looked at.

**The glowing block is still there, and is now a proposal: D-333.** After step F (32 bpc, Lightning Diff retuned to Fast Box Blur 101.2 / 121.3 and Exposure +20.49), the Diff copy lights a block about 600 pixels wide under the strike. The tutorial shows a soft warm pool. Two leads were tested first and ruled out:

- **The ground texture stand-in.** A darker texture only punches holes in the block, and the plate alone gives an even block (table in `D-333_ae_32bpc_proposal.md`).
- **Solid Composite rounding.** Adobe's effect table lists Solid Composite as 32 bpc, so it does not round.

The likely cause is how After Effects' default 32 bpc works. Its blurs average display values, and Exposure converts to linear light first. Converting through a 2.2 power curve reproduces the pool; the sRGB curve and our linear Float both give the block. No source says which curve After Effects uses, so nothing was built. `D-333_ae_32bpc_proposal.md` and `D-333 pictures/` show the three ways side by side.

**The weak glow at frame 12 is not a fault.** The glow layer's Exposure carries the tutorial's `wiggle(9,3)`, which dips around frame 12. With the wiggle switched off, frame 12 glows as strongly as frame 20.

**Known stand-in, unchanged:** the ground texture. The tutorial lays a dark grunge texture image on the ground; ours is the plate with a Fractal Noise grunge in Overlay and a Soft Light copy of the plate.

The rebuilt project is in the owner's Downloads as `advanced_electric_tutorial2.json`, and the video as `advanced_electric_tutorial2.mp4`.

## D-333 built, tutorial 2 replayed again (2026-10-06)

The owner approved D-333 and it is built as B-214 (`D-333_ae_32bpc.md`): Composition Settings has a fourth working depth, **32 bpc (After Effects)**, this program's best reading of After Effects' default 32 bpc. Tutorial 2 was replayed from an empty project in the test copy with this depth at step F, saved again to Downloads (`advanced_electric_tutorial2.json`), reopened outside the app with no notices, and exported again (`advanced_electric_tutorial2.mp4`). Frames 12 and 20 were pulled from the MP4 and looked at.

**Partly.** The 600-pixel yellow block is gone. Under the strike there is now a smaller patch that fades out, but it is red rather than warm, and the ground texture still shows through it in dark holes. The tutorial at frame 20 shows only a small, dim gold pool of light under the tip of the bolt. So D-333 moves the picture the right way but does not match it. Why it is red: Exposure through the 2.2 curve lifts the orange's red far more than its small green part. What else differs is not known; the four reflection copies all pass through the new Exposure, and the tutorial's texture is a stand-in. Nothing further was changed or guessed.

## Tutorial 2's missing effects built, replayed again (2026-10-07)

The owner asked for every effect tutorial 2 uses that the app lacked, built and applied, aiming for the tutorial's exact look. Four were missing:

- **D-334** (B-215), Advanced Lightning's soft core: built, FX-LCORE-001 to 007.
- **D-335** (B-217), Exposure's Offset, Gamma Correction and Bypass Linear Light Conversion: built, 53 of 53 checks (`D-335_exposure_ae.md`). The tutorial's ground uses Gamma Correction 1.69.
- **D-337** (B-216), Glow and Solid Composite on display values in 32 bpc (After Effects): built (`D-337_glow_display.md`).
- **D-336**, CC Vector Blur: proposed at first, since the tutorial switches it off again by 27:16 and its final look does not use it. Built on 2026-10-08 at the owner's word (`verification/D-336_vector_blur.md`, 111 of 111 checks pass); the replay still leaves it out.

Tutorial 2 was replayed from an empty project in the test copy with all three in use (ground Exposure 2.47 with Gamma Correction 1.69; the plate hidden, as at the tutorial's end; hotspot radius 9, feather 20). It was saved to Downloads, reopened outside the app with no notices, and exported again. Frames 0, 12 and 20 were pulled from the MP4 and put beside the tutorial's frame at 28:20.

**Matches:** the dim gold pool under the tip of the bolt (D-333's red patch is gone), the dark street, the glow's spread and the small hot point at the top.

**Still different:**
1. **Colour.** The tutorial's bolt and halo are orange (hue about 26 degrees), ours pale gold (about 45 degrees). Both use the same core colour (185, 153, 59). The tutorial's bolt is orange even before any Glow is added (19:35), so Advanced Lightning itself shifts it. Neither the video nor Adobe's help shows how, so nothing was guessed.
2. **Path.** The bolt's path differs, because After Effects' random pattern cannot be copied. At frame 0 ours leans further left than the tutorial's.

## Tutorial 2's orange matched (2026-10-07)

The owner asked for the orange to match. The tutorial's frame at 28:20 is its frame 9 (the viewer's clock reads 0:00:00:09), so it was measured against our frame 9. The cause of After Effects' redder colour is still not known: no standard colour conversion explains it. So the colour is matched with **three stand-ins, chosen to fit the tutorial's picture, not read from its panels**. Nothing in the program changed.

1. **The bolt's colour** is (185, 137, 59) in place of the panel's (186, 154, 60). This sets the pool on the ground and the bolt's own colour.
2. **An extra Hue/Saturation on the glow layer** (Hue -10, Saturation +50), placed before the tutorial's own Saturation -29. This turns the halo orange without changing the pool.
3. **The hotspot** is a generated picture, `target/p26/t2/hotspot.png`: a salmon glow about 110 pixels across, fading out from the middle, in Add at the bolt's origin. It replaces the feathered white disc. The tutorial's Hotspot.jpg is stock footage, so we only have its look.

Tutorial 2 was replayed from an empty project in the test copy with these stand-ins. It was saved to Downloads again, reopened outside the app with no notices, and exported again. Frame 9 was pulled from the MP4 and measured. Each colour below is the average of the warm pixels in one area (red, green, blue, out of 255):

| Frame 9 | Tutorial | Before | After |
|---|---|---|---|
| Halo beside the bolt | (57, 34, 16), hue 27 | (60, 52, 28), hue 45 | (56, 34, 17), hue 25 to 26 |
| Pool under the tip | (57, 41, 16), hue 37 | hue 45 | (49, 35, 16), hue 35 |
| Hotspot, 20 pixels out | (137, 86, 62), hue 19 | (137, 133, 124), near white | (125, 77, 56), hue 18 to 20 |
| Hotspot, 80 pixels out | 22 | about 10, yellow | 20 |
| Bolt's core, middle value | (255, 244, 166) | | (255, 239, 114) |

| Before (frame 9) | After (frame 9) | After (frame 20) |
|---|---|---|
| ![before](P-26%20pictures/t2_colour_before_f9.png) | ![after](P-26%20pictures/t2_colour_after_f9.png) | ![after 20](P-26%20pictures/t2_colour_after_f20.png) |

**What to look for:** the bolt and its halo are orange instead of pale gold. The glow at the top is a soft salmon ball instead of a small white dot. The pool on the ground stays gold.

**Still different:**
1. **The bolt's core** is a little more yellow than the tutorial's: blue 114 against 166. This comes from the stand-in Hue/Saturation.
2. **Forks.** In the tutorial's later frames (28:40) the bolt splits into three long strands, and each one reaches the ground and lights its own spot. Ours forks into short twigs, so the pool gets less light. How After Effects lays out its forks is not published; changing ours would be a guess and would change every Strike bolt. It is offered to the owner, not done. (Done later the same day, as D-338: see below.)
3. **Path.** The path is random, as before.

## Tutorial 2's long forks (2026-10-07)

The owner said yes to trying the long forks. In the tutorial the author turns Advanced Lightning's Decay down "for them to touch down on the ground". So the strands are long because Decay is low, and they stop at the ground because of the ground mask (Alpha Obstacle). **D-338** gives Lightning Bolt a new setting, **Forks**: Short (as before, and what every older file gets) or Long. With Long, the forks from the main bolt's first three splits run on down beside it until the ground stops them (`verification/D-338_lightning_forks.md`, 45 of 45 checks pass). Old files and Strike bolts don't change unless Forks is set to Long.

Tutorial 2 was replayed from an empty project with Forks Long on the bolt. That was the only difference from the replay before. It was saved to Downloads again, reopened outside the app with no notices, and exported again.

| Frame 20: Short on the left (before), Long on the right (after) |
|---|
| ![forks f20](P-26%20pictures/t2_forks_f20.png) |

Frame 12, `P-26 pictures/t2_forks_f12.png`, shows the same thing.

**What to look for:** on frames 7, 8, 12 and 20 the bolt now splits into two or three strands, and each one reaches the ground. In the tutorial's later frames (28:40) the strands do the same. The ground under the bolt gets more light from them. Average of the ground strip under the bolt, out of 255:

| Frame | Short | Long |
|---|---|---|
| 9 | (5.3, 3.7, 1.5) | (5.3, 3.7, 1.5), no early fork on this frame |
| 12 | (9.1, 6.5, 2.7) | (12.5, 8.9, 3.6) |
| 20 | (9.3, 6.6, 2.7) | (15.4, 11.0, 4.5) |

**Still different:**
1. **Thickness.** Our strands start at half the main bolt's width, a rule kept from D-190's forks. The tutorial's look nearly as thick as the main bolt.
2. **Where and when.** Where each strand goes, and which frames fork, is random, as in After Effects. Frame 9 (the tutorial's 28:20) happens to draw no early fork with our random numbers. Frames 13 and 24 draw six or seven forks and look busier than the tutorial.
3. **Path.** The path is still random, as before.

## Tutorial 2's thicker strands (2026-10-08)

The owner asked for thicker strands to match the tutorial. **D-339** adds a third Forks choice, **Long, full width** (`verification/D-339_lightning_full_forks.md`, 37 of 37 checks pass). Its strands follow exactly the same paths as Long's, but each one starts as wide as the main bolt where it leaves it, instead of half as wide. Short and Long, and every file saved with them, are unchanged.

Tutorial 2 was replayed from an empty project with Forks set to Long, full width. That was the only difference from the replay before; the two saved projects differ in that one word. It was saved to Downloads again, reopened outside the app with no notices, and exported again.

| Frame 20: Long on the left (before), Long, full width on the right (after) |
|---|
| ![strands f20](P-26%20pictures/t2_full_f20.png) |

Frames 7, 8 and 12 show the same thing: `P-26 pictures/t2_full_f7.png`, `t2_full_f8.png`, `t2_full_f12.png`.

**What to look for:** the strands are in the same places, but now as thick as the main bolt where they split from it, as in the tutorial at 28:40. More white bolt means more glow, so the ground under the bolt gets brighter. Average of the same ground strip as before, out of 255:

| Frame | Long | Long, full width |
|---|---|---|
| 7 | (11.4, 8.0, 3.4) | (19.9, 14.2, 5.9) |
| 9 | (5.3, 3.7, 1.5) | (5.3, 3.7, 1.5), no early fork on this frame |
| 12 | (12.5, 8.9, 3.6) | (18.8, 13.5, 5.5) |
| 20 | (15.4, 11.0, 4.5) | (25.0, 18.0, 7.3) |

**Still different:**
1. **Where strands overlap** (frame 7) they now make a hotter, whiter patch than the tutorial shows.
2. **Where and when** each strand goes is still random, as before.

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
| D-328 | Turbulent Displace in After Effects units | 2, 5 | `D-328_turbulent_ae.md` | `D-328 pictures/` |
| D-329 | Advanced Lightning goes round shapes; a negative Alpha Obstacle | 2 | `D-329_lightning_around.md` | `D-329 pictures/` |
| D-330 | 8 bpc (After Effects) working depth | 2 | `D-330_eight_bpc.md` | `D-330 pictures/` |
| D-332 | Keys on every keyable setting survive a save | 1 | `D-332_keyed_settings.md` | `D-332 pictures/` |

Commits: cbbaec2 (D-290 to D-297), 79e0eae (D-298), d37a364 (D-299 to D-303), f28574e (D-304), c6cd69e (D-305, D-306), bf0978f (D-307), 69980f7 (D-310), e419620 (D-311 to D-313, D-315), 5f866db (D-314), 5ea35a3 (D-316), 15f8f25 (D-317), 4a17745 (D-318), c4fc502 (D-319), e762c99 (D-320), d3bca9e (D-323), f559354 (D-324).

## Checks

After each batch: the whole core test suite and the app test suite. The only failures are two scratch measurements that failed before P-26 began and are not part of the project's checks (`zz_scratch_g2`, `zz_scratch_p25`). The app suite: 91 pass, 5 set aside as before.

One set of fixture values was changed, as a recorded specification decision: D-331 gave FX-GLOW-AE-001, 004, 006, 007 and 008 new expected values (Glow in After Effects units, built the day before), committed before the code. No other fixture's expected values were changed.

## Waiting on the owner

1. **Playtests** of D-290 to D-307, D-310 to D-317 and D-318 to D-324, each page's "For the owner to try".
2. **Playtests** of D-321, D-322 with D-331, and D-325 to D-327, each page's "For the owner to try". The After Effects test asked for here before is no longer needed: D-321 and D-331 are settled from the sources above.
3. **Playtests** of D-328, D-329 and D-330 (approved 2026-10-05 and built), each page's "For the owner to try".
4. **Playtest** of D-332: open `Downloads\shockwave_tutorial1.json`, save it under a new name, and open it again (`D-332_keyed_settings.md`, "How to check it yourself").
5. **Playtest** of D-333 (approved 2026-10-06 and built as B-214): the "32 bpc (After Effects)" depth (`D-333_ae_32bpc.md`, "For the owner to try"). Tutorial 2 after step F is now partly right: a red fading patch instead of a block.
6. **Playtest** of D-334, D-335 and D-337 (built 2026-10-07): open `Downloads\advanced_electric_tutorial2.json` and look at frame 20, a dim gold pool under the bolt. Each fix's page in `verification/` lists what else to try. Since the colour stand-ins (same day), frame 9 should look orange, with a salmon glow at the top, as in the tutorial at 28:20. Since D-338 (same day), frames 7, 12 and 20 should show two or three strands reaching the ground; Effect Controls shows Forks: Long on the bolt. Since D-339 (2026-10-08), those strands are as thick as the main bolt where they leave it.
7. **Playtest** of D-336, CC Vector Blur (built 2026-10-08): Effects, Blur & Sharpen, CC Vector Blur; what to try is in `verification/D-336_vector_blur.md`.

D-328 to D-330 were approved on 2026-10-05 and are built. D-309 was approved on 2026-10-04 and is built (D-310 to D-317). D-308 was approved on 2026-10-04 and is built as D-318 to D-320, D-323 and D-324.

Left out by the charter, not proposed: video import, the tracker, particles (CC Pixel Polly), 3D layers and lights, the two-node camera.
