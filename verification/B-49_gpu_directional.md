# B-49: Directional Blur on the graphics card

Built on 2026-09-26 at the owner's "proceed", against the B-49 entry in document 15. D-106, proposed, is its tolerance. It is the third effect to move to the card, in D-100's order: Radial Blur, Bloom, then Directional Blur, Gaussian Blur and Glow, each a unit of its own.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is a Directional Blur has that blur done by the card rather than the CPU.

- The card reads the drawing along the same lines as the CPU, one pixel apart, and keeps the same running totals, in double precision (D-98).
- Any effects before the blur still run on the CPU first, as before.
- The card keeps each blur it makes, and reuses it while the drawing and the settings stay the same.
- A blur of length 0 changes nothing, so it is not sent to the card at all.

Bloom's streaks are the same kind of blur along lines, so the two now share one piece of the card's program. The Bloom check was run again on it and is unchanged: 288 of 288.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave the blur to the card.
- A blur with a setting out of range is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it, blur included, and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.

**Left on the CPU for now:**

- a Directional Blur followed by another effect
- a Directional Blur on a composition, shape, solid or adjustment layer, or on a matte

**Precision.** Unlike Bloom, the drawing goes to the card in half precision, as every other drawing does since B-44b. A blur averages what it reads and has no threshold for a rounding to cross, so it keeps within 1 level without the extra memory. The running totals are in double precision, as the CPU's are, so a card without double precision draws these frames on the CPU, with a message saying why. That path has not been seen here, because this card has it.

## The check

`verification/B-49_gpu_directional_table.md`, written by `tests/b49_gpu_directional.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of FX-DIRBLUR-001 to 015
- frames 0, 100 and 239 of the reference shot with three Directional Blurs added:
  - straight across, length 20, on the background
  - a Gaussian Blur and then a slant at 45 degrees, length 30, on the second layer, so the blur starts from a drawing the Gaussian has already grown
  - straight up and down, length 8, on the third layer

**158 of 158 checks pass.**

- On the fixtures, the card's picture is the same bytes as the CPU's on most frames. Where it is not, at most 2 pixels differ, by 1 level each.
- On the reference shot, no channel of any pixel is more than **1 level of 255** from the CPU's. That is the tolerance D-100 set for layering, so D-106 proposes it for Directional Blur unchanged.
- FX-DIRBLUR-010 and 011 animate the length and the direction. They pass on every frame, so a kept blur is never shown after its settings move. Frame 0 of 010 has length 0: nothing goes to the card, and the two pictures are the same bytes.
- FX-DIRBLUR-005, length 0, sends nothing to the card, and the two pictures are the same bytes.
- FX-DIRBLUR-012 to 015 have a setting out of range. Nothing goes to the card, the two pictures are the same bytes, and both paths give the same warning.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft.

## The pictures

In `verification/B-49 pictures/`, for the worst comparison, frame 100 of the blurred reference shot at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the background smeared sideways, the second layer on a slant.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 14,674 of 2,073,600 pixels (0.7%) differ, each by 1 level of 255. They are scattered over the blurred background, where half precision rounds a sample differently from time to time. The squares make them look far denser than they are.

## Speed

`verification/B-49_gpu_directional_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b49_gpu_directional -- --ignored`):

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; Windows 11; release build.
- **What was timed:** every frame of the blurred reference shot, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the shot twice.
- **Memory:** D-40's 1 GiB, as in the B-46 and B-47 tables, so the three compare. The app now uses Automatic memory (B-48), which this table did not time.
- **Figures:** medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.6 | 17.1 | 16.3 | 16.3 |
| Full | 59.7 | 61.3 | 27.3 | 27.4 |

**At Full, the card is a little over twice as fast:** 27 ms a frame against 61, which is inside a 24 fps frame's 41.7 ms. The gain is smaller than Bloom's because the CPU's Directional Blur was already fast after D-98.

**At Draft the two are even**, as with Radial Blur and Bloom: D-99 already shrinks each drawing before the blur, and most of a Draft frame is reading the drawings.

## Limits

- A Directional Blur followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- The kept blur counts toward the card's memory budget.
- A card without double precision draws every frame with a Directional Blur on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
