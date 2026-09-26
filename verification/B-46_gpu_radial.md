# B-46: Radial Blur on the graphics card

Built on 2026-09-26 at the owner's "let's proceed with gpu-acceleration on the effects", against the B-46 entry in document 15. D-103, proposed, is its tolerance. It is the first effect to move to the card, as D-100 ordered: Radial Blur, then Bloom, Directional Blur, Gaussian Blur and Glow, each a unit of its own.

## What it does

With the viewer's switch on **Draw on GPU**, a drawn layer whose last effect is a Radial Blur has that blur done by the card rather than the CPU. Any effects before it still run on the CPU first, as before. The card keeps each blur it makes and reuses it while the drawing and the settings stay the same. That matches what the CPU's effect cache does, and it is where most of the speed comes from.

**What it does not change:**

- Exports, fixtures and **Draw on CPU** are untouched. They never leave the blur to the card.
- A blur with a setting out of range is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it, blur included, and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.

**Left on the CPU for now:**

- a Radial Blur followed by another effect
- a Radial Blur on a composition, shape, solid or adjustment layer, or on a matte

## The check

`verification/B-46_gpu_radial_table.md`, written by `tests/b46_gpu_radial.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of FX-RADIAL-001 to 018
- frames 0, 100 and 239 of the reference shot with three Radial Blurs added:
  - a zoom 30 on the background
  - a Gaussian Blur and then a spin 12 on the second layer, so the blur's centre is found after the Gaussian Blur has grown the drawing
  - a spin 100, the most there is, on the third layer

**188 of 188 checks pass.**

- Wherever the blur went to the card, no channel of any pixel is more than **1 level of 255** from the CPU. That is the same tolerance D-100 set for layering, so D-103 proposes it for Radial Blur unchanged.
- FX-RADIAL-008 and 009 animate the amount and the centre on a drawing that does not change. They pass on every frame, so a kept blur is never shown after its settings move.
- FX-RADIAL-013 to 018 have a setting out of range. Nothing goes to the card, the two pictures are the same bytes, and both paths give the same warning.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft.

## The pictures

In `verification/B-46 pictures/`, for the worst comparison, frame 0 of the blurred reference shot at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 20,539 of 2,073,600 pixels (1%) differ, each by 1 level of 255. They are scattered evenly as rounding noise, not gathered into any shape, and the squares make them look far denser than they are.

## Speed

`verification/B-46_gpu_radial_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b46_gpu_radial -- --ignored`):

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; Windows 11; release build.
- **What was timed:** every frame of the blurred reference shot, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the shot twice.
- **Figures:** medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.5 | 17.0 | 17.5 | 17.2 |
| Full | 160.1 | 152.1 | 25.7 | 26.5 |

**At Full, the card is about 6 times faster:** 26 ms a frame against 152, which is inside a 24 fps frame's 41.7 ms.

**At Draft the two are even.** D-99 already shrinks each drawing to a sixteenth of its pixels before the blur, so the CPU's blur is small. At Draft most of a frame is reading the drawings, which the card cannot speed up.

The first build of B-46 redid the blur every frame and was slower at Draft: 23 ms against the CPU's 17. Two changes brought it level:

- The card now keeps each blur it has made.
- At Draft, on the card's path only, the shrunken drawing is now kept in the effect cache, so it goes to the card once rather than every frame. This took the card's sending time from 1,188 ms to 131 ms over the 480 frames.

The CPU's "again" at Full is no faster than its first loop. B-46 did not look into why.

## Limits

- A Radial Blur followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- The kept blur counts toward the card's memory budget, at twice a drawing's bytes: sixteen bytes a pixel against the drawing's eight.
- Checked on this machine's card only, as D-100 says.
