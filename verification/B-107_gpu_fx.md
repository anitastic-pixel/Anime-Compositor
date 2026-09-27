# B-107: the third batch on the graphics card

Built on 2026-09-27 against the B-107 entry in document 15, as the card half of the owner's "now let's do a full performance audit like we did with these new effects, like gpu-acceleration, etc." (P-22 was the processor half). D-165, proposed, is its tolerance: the same 1 level of 255 as D-122 and D-133.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is one of these twenty-nine has that effect done by the card rather than the CPU:

Invert, Brightness & Contrast, Black & White, Posterize, Threshold, Channel Mixer, Vibrance, Leave Color, Solarize, Halftone, Mosaic, Emboss, Find Edges, Sharpen, Diffusion, Wave Warp, Ripple, Twirl, Bulge, Mirror, Linear Wipe, Radial Wipe, Venetian Blinds, Iris Wipe, Simple Choker, Speed Lines, Cross Glare, Camera Shake and Rain.

The thirtieth, **Motion Tile**, stays on the CPU, as D-165 proposes. It can grow a drawing by different amounts across and down, which the card's path does not do, and it costs nothing to speak of (P-22: 0.0 ms).

- The card follows the CPU's rule pixel by pixel, on the drawing sent in full precision.
- Speed Lines, Camera Shake and Rain make the CPU's own random numbers on the card, so the lines, the jolts and the streaks are the same.
- Any effects before it still run on the CPU first, as before.
- An effect that changes nothing (an Invert of amount 0, a wipe at 0%, and so on) is not sent to the card.

**Three findings on this card, each fixed and each now checked by the table below.**

1. **Mosaic** first came back from the card as an empty picture. The cause is the driver issue B-76 found with Offset, in a new place: four double-precision numbers kept together in one variable read back as zeros. Mosaic now keeps them as four separate numbers.
2. **Mirror and Venetian Blinds at 45°** each put a handful of pixels on the wrong side of their hard edge. The cause: this card does a multiply and the add after it in one step, rounding once, where the CPU rounds after each, and on a hard edge that last-digit difference moves a pixel across. Two ordinary ways of stopping it did not work on this driver. What works is a form of multiply the card will not merge. Mirror, Venetian Blinds and Linear Wipe now use it where they decide which side a pixel is on.
3. **Rain at Draft.** Draft shrinks every distance to match the smaller drawing. A Rain whose spacing then falls below its least, 2 pixels, is invalid, so the CPU reports it and skips it. The card drew it anyway. The plan now checks an effect's settings at the distances it will really run at before leaving it to the card, so both skip it and both report `EFFECT_PARAMETER_INVALID`.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave these effects to the card.
- An effect with invalid settings is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it and says so (`GPU_PREVIEW_ON_CPU`). The picture is then exactly the one the CPU always drew.

**Left on the CPU:**

- Motion Tile
- one of the twenty-nine followed by another effect
- one on a composition, shape, solid or adjustment layer, or on a matte
- any frame with an adjustment layer in it, by B-44's rule. The check makes sure the CPU's picture of such a frame is exact, byte for byte.

**Precision.** The card needs double precision (`SHADER_F64`), as for B-65 and B-76. A card without it draws these frames on the CPU, with a message saying why. This card has it.

## The check

`verification/B-107_gpu_fx_table.md`, written by `tests/b107_gpu_fx.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of every fixture of the twenty-nine. A fixture that is meant not to open (a deliberately broken setting) has no picture to compare and is skipped.
- frames 0, 100 and 239 of the reference shot, with each effect added to its first three layers. The second layer gets a Drop Shadow first, so the effect also runs on a drawing another effect has already grown. The settings:

| Effect | Settings in the reference shot |
|---|---|
| Invert | RGB, 80% |
| Brightness & Contrast | brightness 30, contrast 40 |
| Black & White | reds 120, yellows 110, greens −10, cyans −50, blues −50, magentas 120 |
| Posterize | 6 levels |
| Threshold | level 128 |
| Channel Mixer | red and blue swapped |
| Vibrance | vibrance 40, saturation 20 |
| Leave Color | keep red, tolerance 15, softness 10, 100% |
| Solarize | threshold 128 |
| Halftone | size 8, 45°, black on white, 100% |
| Mosaic | size 10 |
| Emboss | 135°, relief 1, contrast 100, grey |
| Find Edges | 100% |
| Sharpen | 100, radius 1 |
| Diffusion | radius 10, 50%, Screen |
| Wave Warp | sine, height 10, width 40, 90°, speed 1 |
| Ripple | centre, amplitude 5, wavelength 30, speed 20 |
| Twirl | 90°, radius 50, centre |
| Bulge | centre, radius 50, height 1 |
| Mirror | centre, 0° |
| Linear Wipe | 50%, 90°, no feather |
| Radial Wipe | 50%, from 0°, centre, clockwise, no feather |
| Venetian Blinds | 50%, 0°, width 20, no feather |
| Iris Wipe | 50%, centre, no feather |
| Simple Choker | −3 (spreads) |
| Speed Lines | black, 120 lines, thickness 1.5, inner 150, jitters 40 and 50, held 2 frames |
| Cross Glare | above 80, length 40, 4 points, 45°, intensity 1, white |
| Camera Shake | 10 pixels, no rotation, every frame |
| Rain | pale blue, density 30, spacing 24, length 20, width 1, 170°, speed 30, 60% |

The wipes are checked without feather on purpose: a hard edge is where a card and a CPU are most likely to disagree.

**6802 of 6802 checks pass.**

- Every one of the twenty-nine, on every fixture frame and every reference frame, at both qualities, is within **1 level of 255** of the CPU's picture, with the same warnings on both paths. Threshold agrees exactly.
- The table's **Frames with it on the card** column counts, for each effect, the frames where the card really did it: from 94 for Rain to 216 for Speed Lines. The rest are frames where the effect changes nothing, has invalid settings, or shares the frame with an adjustment layer.
- Every frame with an adjustment layer was drawn by the CPU and was the CPU's picture exactly.

## The pictures

In `verification/B-107 pictures/`, for the worst comparison, frame 0 of the reference shot with three Black & Whites at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the reference shot with its first three layers (sky, mountains and hills) turned grey, and the green square and the white shapes in their own colours.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 33,060 of 2,073,600 pixels (1.6%) differ, each by 1 level of 255, spread thinly over the grey layers. The squares make them look far denser than they are.

## Speed

`verification/B-107_gpu_fx_timing_table.md`, written by the same test on 2026-09-27 (`cargo test --release --test b107_gpu_fx -- --ignored`), run on its own, with nothing else building:

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD Ryzen 9 9900X with 24 threads; Windows 11; release build.
- **What was timed:** every fourth frame, 60 in all, of the reference shot with three copies of one effect, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the frames twice.
- **Figures:** medians in ms, the second loop.

| Effect | Draft, CPU | Draft, GPU | Full, CPU | Full, GPU |
|---|---:|---:|---:|---:|
| Invert | 26.1 | 26.3 | 39.0 | 26.4 |
| Brightness & Contrast | 27.0 | 25.8 | 38.3 | 25.8 |
| Black & White | 26.7 | 26.1 | 37.4 | 25.9 |
| Posterize | 26.4 | 26.1 | 38.4 | 25.5 |
| Threshold | 26.5 | 25.5 | 37.8 | 25.0 |
| Channel Mixer | 26.2 | 26.9 | 37.5 | 27.4 |
| Vibrance | 27.6 | 27.0 | 40.3 | 27.9 |
| Leave Color | 27.4 | 25.6 | 38.5 | 25.2 |
| Solarize | 26.4 | 25.3 | 36.6 | 26.5 |
| Halftone | 26.1 | 25.8 | 37.2 | 25.0 |
| Mosaic | 25.8 | 25.2 | 37.9 | 24.3 |
| Emboss | 25.9 | 25.2 | 37.1 | 24.3 |
| Find Edges | 26.1 | 25.0 | 37.4 | 25.4 |
| Sharpen | 25.7 | 25.2 | 37.5 | 24.4 |
| Diffusion | 25.6 | 25.0 | 36.7 | 24.3 |
| Wave Warp | 25.6 | 26.3 | 94.6 | 35.4 |
| Ripple | 27.2 | 26.6 | 105.1 | 34.3 |
| Twirl | 25.6 | 25.0 | 41.4 | 27.7 |
| Bulge | 28.2 | 26.2 | 37.8 | 28.4 |
| Mirror | 26.0 | 25.5 | 37.9 | 24.8 |
| Linear Wipe | 27.0 | 26.2 | 37.9 | 28.7 |
| Radial Wipe | 27.4 | 27.1 | 42.0 | 25.6 |
| Venetian Blinds | 26.9 | 25.5 | 36.8 | 24.7 |
| Iris Wipe | 26.1 | 25.5 | 36.8 | 24.6 |
| Simple Choker | 26.1 | 25.6 | 37.0 | 24.8 |
| Speed Lines | 25.8 | 28.3 | 80.8 | 34.8 |
| Cross Glare | 26.4 | 25.0 | 120.6 | 24.6 |
| Camera Shake | 26.0 | 26.9 | 90.0 | 34.3 |
| Rain | 26.3 | 27.0 | 79.8 | 33.8 |

**At Full, the card is faster for every one of the twenty-nine, and every one now fits inside a 24 fps frame's 41.7 ms.**

- The twenty-three light ones take 24 to 29 ms a frame against 37 to 42 on the CPU, about a third faster.
- The heavy ones gain most:
  - Cross Glare: 25 ms against 121, about five times as fast.
  - Ripple: 34 against 105, three times.
  - Wave Warp: 35 against 95.
  - Camera Shake: 34 against 90.
  - Speed Lines: 35 against 81.
  - Rain: 34 against 80.

**At Draft the two are about even**, within 2.5 ms either way. Draft already shrinks each drawing before its effects, so most of a Draft frame is reading the drawings and laying them together, which the card cannot speed up.

## Limits

- Motion Tile stays on the CPU.
- One of the twenty-nine followed by another effect still runs on the CPU. So does one on anything but a drawn layer, and any frame with an adjustment layer.
- The multiply the card merges is guarded only where a pixel's side of a hard edge is decided (Mirror, Linear Wipe, Venetian Blinds), and the table shows those now agree. Elsewhere the merge can move a result by the last digit, well inside 1 level.
- Checked on this machine's card only, as D-100 says.
