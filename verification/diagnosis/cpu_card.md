# Diagnosis lane cpu_card: processor-versus-card mismatches from the real-app effect check

Read-only diagnosis of 2026-10-13 on main at 90a33e9a. Nothing was built, run or changed: no cargo,
no app, no `tools/app_snapshot.py` (a speed measurement was running). The evidence is the pictures
and `summary.json` files the three lanes already wrote under `verification/app_snapshots/`, the
tool's own source, and the page's source.

## Plain summary

**None of these mismatches is the graphics card drawing a different picture from the processor.
They are all one fault in the checking tool, not in the app.**

The tool takes the processor's picture straight from the app, but it takes the card's picture back
out of the viewer's web canvas. A web canvas keeps every pixel "pre-multiplied" by its
see-through amount, in 8 bits. For a solid pixel that loses nothing. For an almost invisible pixel
(1 to 12 out of 255 solid) it loses almost all the colour: a pixel that is 1/255 solid can only come
back as colour 0 or 255, one that is 2/255 solid only as 0, 128 or 255, and so on. The tool then
compares the colours as if the pixel were solid, so a pixel nobody can see counts as "255 levels
apart".

Every large number in the list sits on such a nearly invisible pixel. Wherever the picture is solid,
the card and the processor agree within 1 level in every sampled pixel, which is the project's
standard. Compared the way the canvas actually stores them (colour times see-through amount), every
sampled pixel of every listed effect agrees within 1 level, Blobbylize included.

What the owner would see: nothing. In the window the card paints its own picture directly (B-45);
the canvas is only used for the tool's read-back, the Snapshot button and the processor path.

The fix is to the checking tool: compare the two pictures the way the canvas stores them. Small.
No engine code changes, no `Fixtures/` value changes.

## The project's tolerance

The card must match the processor within **1 level of 255** in the 8-bit picture the viewer
receives (straight sRGB). This is the rule every card batch was judged by: B-50, B-65
(`verification/B-65_gpu_fx.md`, "within 1 level of 255"), B-76, D-228's single-precision audit
(document 00 line 61, "every picture within 1 level"), and the per-effect tests such as
`tests/b256_bend_it_bender_blobbylize.rs:194` and `:226` (`largest <= 1`). Those tests compare
`Gpu`-drawn bytes with `to_srgb8_straight` bytes in process (`tests/b256_bend_it_bender_blobbylize.rs:131-142`),
never through a canvas. The "limit of 2" in LANE2_RESULTS.md is the lane's own threshold, not a
project number. Document 21 line 839 asks for per-effect declared tolerances for filters; the
1-level table is how this project declared them.

## The one root cause (all five bugs)

1. **Processor side**: `cpu_js` (`tools/app_snapshot.py:190-207`) fetches `/frame/N?q=full` and
   keeps the bytes exactly as the app encoded them (`Image::to_srgb8_straight`, `src/lib.rs:300`,
   which unpremultiplies in float and only then quantises).
2. **Card side**: `card_js` (`tools/app_snapshot.py:209-239`) has the page `show()` the frame, which
   puts it into the 2D canvas with `paint.putImageData(...)` (`app/ui/index.html:2723-2724`), and
   reads it back with `paint.getImageData(...)` (`tools/app_snapshot.py:236`). A 2D canvas stores
   premultiplied 8-bit colour, so put-then-get rounds each channel to a multiple of about 255/alpha.
   The tool's own docstring says so (`tools/app_snapshot.py:14-16`: "within rounding where it is
   partly see-through"), but `difference()` (`tools/app_snapshot.py:170-180`) ignores it and
   compares straight colours at every alpha, including alpha 0 and 1.

The tell-tale in the card pictures: at the worst pixels the card's colours are exactly the
canvas's steps. Examples from the frame-12 previews:

| Effect | Pixel | Processor RGBA | Card RGBA | Note |
|---|---|---|---|---|
| Corner Pin | (478,223) | 16,171,3,**1** | 0,**255**,0,1 | alpha 1: colour can only be 0 or 255 |
| Corner Pin | (178,255) | 24,182,7,**2** | 0,**128**,0,2 | alpha 2: 0, 128 or 255 |
| Corner Pin | (415,235) | 37,157,18,**4** | **64,128**,0,4 | alpha 4: steps of 64 |
| Radial Blur | (0,0) corner | 46,144,255,**1** | 0,255,255,1 | the "R50 G111" patch is the transparent corners, not the centre |
| CC Cylinder / Select Color | (146,140) | 11,2,4,**12** | 21,0,0,12 | alpha 12: steps of 21 |
| CC Blobbylize | (0,236) | 17,121,119,**0** | 0,0,0,0 | fully transparent; colour meaningless |

Method used (light, read-only Python over the existing 480x270 preview PNGs, every 4th pixel of the
full frame, frame 12, all effects named in the brief): for every sampled pixel, (a) the largest
straight difference where both pixels are fully solid, and (b) the largest difference after
premultiplying each channel, `round(c * a / 255)`.

| Effect | (a) solid pixels | (b) premultiplied | Verdict |
|---|---|---|---|
| CC Blobbylize | 1 | 1 | inside tolerance |
| Radial Blur (spin) | 1 | 1 | inside |
| Gaussian Blur | 1 | 1 | inside |
| CC Cylinder, CC Sphere | 1 | 1 | inside |
| CC Bend It, CC Bender, CC Flo Motion, CC Griddler, CC Lens, CC Page Turn, CC Power Pin, CC Slant, CC Split, CC Split 2, Corner Pin, Displacement Map, Optics Compensation, Polar Coordinates, Ripple, Transform | 1 | 1 | inside (all 16) |
| Bubbles, Foam, Particle Emitter/Grid/Plane/Playground/Space, Pixel Polly (polygon_shards), Scatterize, Star Burst, CC Ball Action, Mr. Mercury | 0-1 | 0-1 | inside |
| Block Load, Burn Film, Card Dance, Shatter, Glass, Threads | 1 | 1 | inside |
| Camera Shake, Select Color (not in the brief, same signature) | 1 | 1 | inside |

The previews hold 1 pixel in 16, so this is strong evidence, not proof, for the other 15. The
single check that settles it is the `--full` run below.

## Per bug

### 1. CC Blobbylize (`core.blobbylize`): max 198, mean about 0.7

- **Symptom**: the tool reports the biggest, most widespread mismatch in lane 2.
- **Root cause**: the canvas round trip above. Blobbylize with Cut Away 20 leaves the background
  partly see-through almost everywhere (in the processor preview, 122,922 of 129,600 sampled pixels
  have alpha below 255, many at 0-12), so the canvas rounding touches most of the frame, which is
  why the mean is high. Worst pixels have alpha 0 or 1. The engine's own card check for this effect
  (`tests/b256_bend_it_bender_blobbylize.rs`, "within 1 level of 255", shader at `src/gpu.rs:7386`,
  `:9142`, `:9172`, `:16364`) compares in process and is not affected.
- **Side note, harmless**: the processor writes colour into pixels whose 8-bit alpha is 0
  (17,121,119,0): `to_srgb8_straight` unpremultiplies a float alpha that is above 0 but rounds to 0.
  Invisible, and standard for straight output; not a bug.
- **Confidence**: likely, strongly supported (all 129,600 sampled pixels within 1 premultiplied
  level, all solid pixels within 1). Confirm with the `--full` run.
- **Fix**: the tool fix below. **Size**: small.

### 2. Radial Blur, spin: max R50 G111 B5

- **Symptom**: a "small patch" differs; the lane guessed the centre.
- **Root cause**: same. The patch is the near-transparent corners and edges of the frame that the
  spin with transparent edges leaves (worst at (0,0), (479,0), (0,269), alpha 1 to 11), not the
  centre.
- **Confidence**: likely, strongly supported. **Fix**: tool fix. **Size**: small.

### 3. Sixteen distorts differ on a handful of pixels (max 16-255, mean under 0.01)

- **Shared cause, named**: not the edge sampling, the sample-coordinate rounding or clamp-versus-
  transparent in the engine. It is the canvas's premultiplied 8-bit storage of the warped layer's
  anti-aliased outline (alpha 1 to 12), compared as straight colour. Every one of the 16 has its
  worst pixel on that outline, and all 16 agree within 1 level on solid pixels and within 1
  premultiplied level everywhere sampled. Distorts that leave no see-through outline (Twirl, Wave
  Warp with repeat edges, Offset, Mirror, ...) show max 1, which fits.
- **Confidence**: likely, strongly supported. **Fix**: tool fix. **Size**: small.

### 4. CC Cylinder, CC Sphere (max 11) and Gaussian Blur (max 3)

- **Cylinder/Sphere**: the numbers are R11 G4 B7, mean 0.028, and **exactly the same numbers**
  appear for Select Color, Card Dance, Block Load frame 0, and frame 0 of Foam, Mr. Mercury and
  every Particle effect (before any particle is born). So it is not the lighting. It is the soft
  red circle (layer 2) whose feathered rim lies over a background these effects made transparent;
  that rim reaches alpha 11-12, where the canvas steps are 21 levels. The lit surface itself agrees
  within 1.
- **Gaussian Blur**: max 3, at the blurred, partly see-through frame edge; at alpha 40-85 the
  canvas step is 3 to 6 levels, so 3 is the round trip. Solid pixels agree within 1.
- **Confidence**: likely, strongly supported. **Fix**: tool fix. **Size**: small.

### 5. Lane 1 particle/simulation effects, and the soft-edge group

- **Particles** (Bubbles, Foam, Particle*, Pixel Polly, Scatterize, Star Burst, Ball Action,
  Mr. Mercury), single pixels up to 255: not random-number or float-order differences. The project
  makes the card use the processor's own random numbers (B-65 note, `verification/B-65_gpu_fx.md`:
  "Noise makes the CPU's own random numbers"), and no ADR or decision accepts more than 1 level for
  these. The large values are on the faint, nearly transparent particle edges over a background the
  effect cleared; premultiplied, every sampled pixel agrees within 1 (solid pixels within 0-1).
  Particle Plane/Space's high means (0.4-1.4) are many such faint pixels, not a wrong particle.
- **Block Load, Burn Film, Card Dance, Shatter, Glass, Threads** (4-22 at the red circle's soft
  edge): the same red-circle-rim signature as item 4; within 1 premultiplied level.
- **Against D-228**: D-228 moved some heavy card sums to single precision and still held every
  picture within 1 level; nothing here needs D-228's allowance or any wider one. All are **inside**
  the project tolerance once compared correctly; **none breaks it** on the evidence available.
- **Confidence**: likely, strongly supported. **Fix**: tool fix. **Size**: small.

## Fix plan (one change, in the checking tool only)

`tools/app_snapshot.py`, function `difference()` (lines 170-180): compare what the canvas can
actually hold. Premultiply both sides per channel, `round(c * a / 255)`, then take max and mean, and
keep alpha as it is. Also report the plain straight max over pixels where both alphas are 255, so a
real colour fault on solid pixels still shows by its full size. Keep the docstring note
(lines 14-16) and say there that the comparison is premultiplied for that reason. Nothing under
`src/`, `app/`, `tests/` or `Fixtures/` changes; no fixture expected value changes, so no
specification decision is needed.

(The alternative, reading the card's bytes before they reach the canvas, needs a page hook in
`app/ui/index.html`; not worth it while the engine tests already compare the card's exact bytes.)

The lane results files' verdicts for these 40-odd rows ("cpu/card differ") should then be re-run
and rewritten from the corrected numbers.

## Proof plan

- **Confirming check first, with today's tool** (when the machine is free), one effect per group,
  full-size pictures kept:

  ```
  python tools/app_snapshot.py --project verification/app_snapshots/cc_blobbylize/cc_blobbylize.json --frames 0,12,24 --on both --full --out verification/diagnosis/cpu_card_full/cc_blobbylize
  python tools/app_snapshot.py --project verification/app_snapshots/radial_blur/radial_blur.json --frames 12 --on both --full --out verification/diagnosis/cpu_card_full/radial_blur
  python tools/app_snapshot.py --project verification/app_snapshots/corner_pin/corner_pin.json --frames 12 --on both --full --out verification/diagnosis/cpu_card_full/corner_pin
  python tools/app_snapshot.py --project verification/app_snapshots/cc_cylinder/cc_cylinder.json --frames 12 --on both --full --out verification/diagnosis/cpu_card_full/cc_cylinder
  python tools/app_snapshot.py --project verification/app_snapshots/particle_plane/particle_plane.json --frames 12 --on both --full --out verification/diagnosis/cpu_card_full/particle_plane
  ```

  Then, on each `frame_0012_cpu_full.png` / `frame_0012_card_full.png` pair, list the differing
  pixels with their alpha. Pass: every pixel more than 1 level apart has alpha at most about 85,
  and the premultiplied difference is at most 1 everywhere. Any solid (alpha 255) pixel more than
  1 apart would disprove this diagnosis and point back at that effect's shader.
- **The test for the fix**: a small `__main__` self-check in the tool: a solid pixel off by 2 is
  reported as 2; a pixel (16,171,3,1) against (0,255,0,1) is reported as 0 premultiplied; a pixel
  (11,2,4,12) against (21,0,0,12) as at most 1.
- **Before and after for the owner**: the same `--full` Blobbylize run as above before the fix
  (summary says max 198) and after (summary says premultiplied max 1, solid-pixel max 1), with the
  two frame-12 pictures side by side, which look the same.

## Sizes

| Bug | Root cause in one line | Confidence | Size |
|---|---|---|---|
| 1 CC Blobbylize | canvas premultiplied round trip of the card picture in the tool, over a mostly see-through frame | likely, strongly supported | small (tool) |
| 2 Radial Blur spin | same, at the transparent corners, not the centre | likely, strongly supported | small (same fix) |
| 3 sixteen distorts | same, at the warped outline's alpha 1-12 pixels | likely, strongly supported | small (same fix) |
| 4 Cylinder/Sphere, Gaussian | same, at the red circle's rim / blurred edge | likely, strongly supported | small (same fix) |
| 5 particles, soft-edge group | same; no RNG or float-order cause, inside the 1-level tolerance | likely, strongly supported | small (same fix) |
