# B-47: Bloom on the graphics card

Built on 2026-09-26 at the owner's "works; proceed to next", against the B-47 entry in document 15. D-104, proposed, is its tolerance. It is the second effect to move to the card, in D-100's order: Radial Blur, then Bloom, Directional Blur, Gaussian Blur and Glow, each a unit of its own.

## What it does

With the viewer's switch on **Draw on GPU**, a drawn layer whose last effect is a Bloom has that bloom done by the card rather than the CPU. The card does every part of it:

- it finds the bright pixels
- it blurs them four times for the halo
- it draws the streaks
- it lays the result on the drawing

Any effects before the Bloom still run on the CPU first, as before. The card keeps each bloom it makes and reuses it while the drawing and the settings stay the same.

A Bloom that lights nothing (intensity 0, or nothing as bright as the threshold) changes nothing, so it is not sent to the card at all.

**What it does not change:**

- Exports, fixtures and **Draw on CPU** are untouched. They never leave the bloom to the card.
- A bloom with a setting out of range is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it, bloom included, and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.

**Left on the CPU for now:**

- a Bloom followed by another effect
- a Bloom on a composition, shape, solid or adjustment layer, or on a matte

## Two things the card needs for Bloom

**Full precision for the drawing.** Since B-44b the card has held drawings in half precision, which is plenty for layering. Bloom is different: it first keeps only the pixels at least as bright as the threshold, and half precision moves some pixels from one side of the threshold to the other. Each pixel that moves adds or loses a whole halo and its streaks. Built with half precision first, the reference shot with three Blooms was up to **9 levels** off at Full and **40** at Draft. So a drawing a Bloom starts from now goes to the card as the CPU holds it. That takes twice the card's memory for those drawings only, sixteen bytes a pixel against eight. With it, every picture is within 1 level.

**Double precision for the streaks.** The streaks keep running totals along long lines, and the CPU keeps them in double precision so they do not drift. The card does the same. This machine's card offers double precision through Vulkan. A card that does not have it draws frames with a Bloom on the CPU instead, with a message saying why. That path has not been seen here, because this card never takes it.

## The check

`verification/B-47_gpu_bloom_table.md`, written by `tests/b47_gpu_bloom.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of FX-BLOOM-001 to 028
- frames 0, 100 and 239 of the reference shot with three Blooms added:
  - the default Bloom (threshold 80, radius 20) on the background
  - a Gaussian Blur and then a Bloom with a star of streaks, length 60, on the second layer, so the bloom starts from a drawing the blur has already grown
  - a low threshold (30) with a slanted cross of streaks at twice the strength on the third layer

**288 of 288 checks pass.**

- On every fixture frame where the bloom went to the card, the card's picture is **the same bytes** as the CPU's.
- On the reference shot, no channel of any pixel is more than **1 level of 255** from the CPU's. That is the tolerance D-100 set for layering, so D-104 proposes it for Bloom unchanged.
- FX-BLOOM-014 to 018 animate the radius, length, angle, threshold and position on a drawing that does not change. They pass on every frame, so a kept bloom is never shown after its settings move.
- FX-BLOOM-002, 004 and frame 0 of 017 light nothing. Nothing goes to the card and the two pictures are the same bytes.
- FX-BLOOM-020 to 028 have a setting out of range. Nothing goes to the card, the two pictures are the same bytes, and both paths give the same warning.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft.

**Found and fixed on the way:** the first run cut a strip off the right and bottom of every bloomed layer. A Bloom grows its drawing, and the card was still placing it at its old size. Now the box a layer is drawn in counts that growth.

## The pictures

In `verification/B-47 pictures/`, for the worst comparison, frame 0 of the bloomed reference shot at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 2,640 of 2,073,600 pixels (0.13%) differ, each by 1 level of 255. Most are inside the see-through square in the middle, with a few specks elsewhere. The squares make them look far denser than they are.

## Speed

`verification/B-47_gpu_bloom_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b47_gpu_bloom -- --ignored`):

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD processor with 24 threads; Windows 11; release build.
- **What was timed:** every frame of the bloomed reference shot, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the shot twice.
- **Figures:** medians over all 240 frames, in ms.

| Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |
|---|---:|---:|---:|---:|
| Draft | 16.7 | 16.8 | 16.8 | 16.7 |
| Full | 182.5 | 163.0 | 26.3 | 26.8 |

**At Full, the card is about 6 times faster:** 27 ms a frame against 163, which is inside a 24 fps frame's 41.7 ms.

**At Draft the two are even**, as with Radial Blur: D-99 already shrinks each drawing before the bloom, and most of a Draft frame is reading the drawings, which the card cannot speed up.

These are medians over the whole shot, where the card reuses a kept bloom on every frame whose drawing has not changed. B-47 did not time a single bloom made from scratch on its own.

## Limits

- A Bloom followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- The kept bloom counts toward the card's memory budget. So does the drawing it starts from, at sixteen bytes a pixel.
- A card without double precision draws every frame with a Bloom on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
