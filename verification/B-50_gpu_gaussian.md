# B-50: Gaussian Blur on the graphics card

Built on 2026-09-26 at the owner's "proceed with gaussian blur and glow", against the B-50 entry in document 15. D-107, proposed, is its tolerance. It is the fourth effect to move to the card, in D-100's order: Radial Blur, Bloom, Directional Blur, then Gaussian Blur and Glow, each a unit of its own.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is a Gaussian Blur has that blur done by the card rather than the CPU.

- The card blurs across and then down, with the same weights as the CPU and each pixel adding them in the same order. Bloom already did its four blurs this way, so the two effects share that piece of the card's program.
- Any effects before the blur still run on the CPU first, as before.
- The card keeps each blur it makes, and reuses it while the drawing and the sigma stay the same.
- A sigma too small to reach a neighbouring pixel changes nothing, so it is not sent to the card at all.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave the blur to the card.
- A blur with a sigma out of range is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it, blur included, and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.

**Left on the CPU for now:**

- a Gaussian Blur followed by another effect
- a Gaussian Blur on a composition, shape, solid or adjustment layer, or on a matte

**Precision.** As with Directional Blur, the drawing goes to the card in half precision. A blur averages what it reads and has no threshold for a rounding to cross. The blur's own sums need no double precision, but its part of the card's program is the one Bloom's streaks share, so a card without double precision draws these frames on the CPU, with a message saying why. That path has not been seen here, because this card has double precision.

## The check

`verification/B-50_gpu_gaussian_table.md`, written by `tests/b50_gpu_gaussian.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of FX-FXK-006, whose Gaussian Blur on a drawing rises from sigma 0 to 2
- every frame of FX-PRE-002, a Gaussian Blur on a composition layer, which stays on the CPU
- frames 0, 100 and 239 of the reference shot with three Gaussian Blurs added:
  - sigma 10 on the background
  - a Directional Blur and then sigma 4 on the second layer, so the blur starts from a drawing the Directional Blur has already grown
  - sigma 1.5 on the third layer

The other fixtures with a Gaussian Blur have it on an adjustment layer, which the card does not draw at all, so they would compare the CPU with itself.

**24 of 24 checks pass.**

- FX-FXK-006 is the same bytes on the card as on the CPU, at every frame and both qualities. Frame 0 has sigma 0: nothing goes to the card. The other frames animate the sigma and pass on every frame, so a kept blur is never shown after its sigma moves.
- FX-PRE-002 sends nothing to the card. Its 2 pixels that differ by 1 level come from the card's layering of the frame, not from the blur, and are within the same 1 level that D-100 already holds layering to.
- On the reference shot, no channel of any pixel is more than **1 level of 255** from the CPU's. That is D-100's tolerance for layering, so D-107 proposes it for Gaussian Blur unchanged.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft.

## The pictures

In `verification/B-50 pictures/`, for the worst comparison, frame 0 of the blurred reference shot at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the background softly blurred, the red disc slightly soft.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 20,915 of 2,073,600 pixels (1.0%) differ, each by 1 level of 255. They are scattered over the blurred areas, where half precision rounds a sample differently from time to time. The squares make them look far denser than they are.

## Speed

`verification/B-50_gpu_gaussian_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b50_gpu_gaussian -- --ignored`):

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; Windows 11; release build.
- **What was timed:** every frame of the blurred reference shot, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the shot twice.
- **Memory:** D-40's 1 GiB, as in the B-46, B-47 and B-49 tables, so they compare.
- **Figures:** medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.1 | 16.2 | 17.2 | 16.7 |
| Full | 57.6 | 58.8 | 28.0 | 28.4 |

**At Full, the card is about twice as fast:** 28 ms a frame against 59, inside a 24 fps frame's 41.7 ms.

**At Draft the two are about even**, the card half a millisecond slower, as with the earlier effects: D-99 already shrinks each drawing before the blur, and most of a Draft frame is reading the drawings.

## Limits

- A Gaussian Blur followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- The kept blur counts toward the card's memory budget.
- A card without double precision draws every frame with a Gaussian Blur on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
