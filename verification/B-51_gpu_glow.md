# B-51: Glow on the graphics card

Built on 2026-09-26 at the owner's "proceed with gaussian blur and glow", against the B-51 entry in document 15. D-108, proposed, is its tolerance. It is the fifth and last effect D-100 names for the card, in its order: Radial Blur, Bloom, Directional Blur, Gaussian Blur, then Glow, each a unit of its own.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is a Glow has that Glow done by the card rather than the CPU.

- The card finds what glows the way the CPU does: the bright parts, by the same test as Bloom, or the chosen colours within the tolerance, each on the pixel's 8-bit colour. With a single **Glow Colours** colour, the light is that colour.
- It blurs that light across and then down with the CPU's weights and order, in the pass Bloom and Gaussian Blur already use, and lays it on the drawing by **Add** or **Screen**, times **Glow Intensity**.
- Any effects before the Glow still run on the CPU first, as before.
- The card keeps each Glow it makes, and reuses it while the drawing and the settings stay the same.
- A Glow at intensity 0, or with nothing in the drawing that glows, changes nothing, so it is not sent to the card at all.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave the Glow to the card.
- A Glow with invalid settings is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it, Glow included, and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.
- The CPU's Glow was reorganised so that the CPU and the card read the settings in one place. Its pictures are unchanged: B-33's Glow table and B-47's Bloom table come out the same as before.

**Left on the CPU for now:**

- a Glow followed by another effect
- a Glow on a composition, shape, solid or adjustment layer, or on a matte

**Precision.** As with Bloom, the drawing goes to the card in full precision, because a Glow picks what glows by a threshold, and half precision could push a pixel across it. The card needs double precision for the part of its program that Bloom's streaks share. A card without it draws these frames on the CPU, with a message saying why. That path has not been seen here, because this card has double precision.

## The check

`verification/B-51_gpu_glow_table.md`, written by `tests/b51_gpu_glow.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of all 33 Glow fixtures, FX-GLOW-001 to 033:
  - bright parts and chosen colours, with and without tolerance
  - radius 0 to 10
  - Add and Screen
  - a tint, and intensity up to 2.5
  - keyed radius, threshold and intensity
  - the 12 fixtures whose settings are invalid
- frames 0, 100 and 239 of the reference shot with three Glows added:
  - the background's bright parts: threshold 60, radius 30, Add
  - the red disc's own colour, tolerance 10, radius 12, intensity 1.5, Screen, in orange
  - the third layer's bright parts with radius 0, so the light is not blurred at all, intensity 0.5

**338 of 338 checks pass.**

- **All 33 fixtures are the same bytes on the card as on the CPU**, at every frame and both qualities.
  - On the 21 with valid settings, the Glow goes to the card whenever something glows.
  - Where nothing glows or the intensity is 0, it does not: FX-GLOW-002, 004, 008 and 010, and frame 0 of 017 and 018.
  - On the 12 with invalid settings, both paths leave the Glow out with the same warning.
- On the reference shot, all three Glows go to the card on every frame, and no channel of any pixel is more than **1 level of 255** from the CPU's. That is D-100's tolerance for layering, so D-108 proposes it for Glow unchanged.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft.

## The pictures

In `verification/B-51 pictures/`, for the worst comparison, frame 0 of the reference shot with three Glows at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the sky, clouds and water brightened with a soft haze, and the red disc lit orange with a soft glow round its edge.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 1,981 of 2,073,600 pixels (0.1%) differ, each by 1 level of 255. Most are in the see-through square in the middle, where the card lays the layers with its own rounding, as D-100 already allows. A few are scattered elsewhere. The squares make them look far denser than they are.

## Speed

`verification/B-51_gpu_glow_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b51_gpu_glow -- --ignored`), run on its own:

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; Windows 11; release build.
- **What was timed:** every frame of the reference shot with three Glows, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the shot twice.
- **Memory:** D-40's 1 GiB, as in the B-46 to B-50 tables, so they compare.
- **Figures:** medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.0 | 16.0 | 17.1 | 17.2 |
| Full | 53.2 | 52.4 | 16.2 | 15.9 |

**At Full, the card is more than three times as fast:** 16 ms a frame against 52 to 53, well inside a 24 fps frame's 41.7 ms.

**At Draft the two are about even**, the card about 1 ms slower, as with the earlier effects: D-99 already shrinks each drawing before the Glow, and most of a Draft frame is reading the drawings.

## Limits

- A Glow followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- Before sending a Glow to the card, the CPU looks through the drawing once for anything that glows. That look is part of the timings above.
- The kept Glow counts toward the card's memory budget.
- A card without double precision draws every frame with a Glow on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
