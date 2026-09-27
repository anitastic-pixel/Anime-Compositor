# B-65: the batch of ten on the graphics card

Built on 2026-09-26 at the owner's "now proceed to gpu-accelerate effects if not implemented along with a performance audit all around these new effects", against the B-65 entry in document 15. D-122, proposed, is its tolerance. The performance audit half of that request is P-21, `verification/P-21_new_effects_audit.md`.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is one of these ten has that effect done by the card rather than the CPU:

Curves, Levels, Hue/Saturation, Gradient, Drop Shadow, Lens Blur (with B-64's iris and highlights), Rim Light, Outline, Noise and Chromatic Aberration.

- The card follows the CPU's rule pixel by pixel, on the drawing sent in full precision.
- Curves' curves are worked out on the CPU and only read off on the card.
- Noise makes the CPU's own random numbers, so the grain is the same grain.
- Lens Blur adds up each row of the iris in the CPU's order, in double precision.
- Drop Shadow, Rim Light and Outline soften with the same blur Gaussian Blur already uses on the card.
- Any effects before it still run on the CPU first, as before.
- An effect that changes nothing (a straight Curves, a Levels at its defaults, an Outline of width 0, and so on) is not sent to the card at all.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave these effects to the card.
- An effect with invalid settings is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn: the table checks this byte for byte.

**Left on the CPU:**

- a Levels whose input white equals its input black. That makes it a threshold, and a rounding either side would turn a pixel from black to white (D-122).
- one of the ten followed by another effect
- one on a composition, shape, solid or adjustment layer, or on a matte

**Precision.** The card needs double precision (`SHADER_F64`), as Bloom does. A card without it draws these frames on the CPU, with a message saying why. That path has not been seen here, because this card has double precision.

## The check

`verification/B-65_gpu_fx_table.md`, written by `tests/b65_gpu_fx.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of all 238 fixtures of the ten, the ones with invalid settings included
- frames 0, 100 and 239 of the reference shot with each effect added to its first three layers. The second layer gets a Drop Shadow first, so the effect also runs on a drawing already grown by another. The settings are chosen to do something visible:

| Effect | Settings in the reference shot |
|---|---|
| Curves | master lifted in the middle, red an S-curve, blue squeezed |
| Levels | input 20 to 230, gamma 1.6, output 10 to 245 |
| Hue/Saturation | hue +40, saturation +30, lightness −10 |
| Gradient | radial, yellow to blue, 80% to 40%, Multiply |
| Drop Shadow | dark blue, 70%, 135°, distance 12, softness 9 |
| Lens Blur | radius 12, hexagon, roundness 20, rotation 15, aspect 1.3, highlight gain 2 above 80 |
| Rim Light | warm white, 45°, width 4, softness 3, intensity 80, Screen |
| Outline | white, width 3.5, softness 2, opacity 90 |
| Noise | 12, colour, seed 7, animated |
| Chromatic Aberration | 5, centre at 40%, 60% |

**2460 of 2460 checks pass.**

- Every one of the ten, on every fixture frame and every reference frame, at both qualities, is within **1 level of 255** of the CPU's picture, with the same warnings on both paths.
- On each effect's fixtures, the effect went to the card on every frame where it does something with valid settings. The table's **Frames with it on the card** column counts these, from 104 frames for Chromatic Aberration to 316 for Lens Blur.
- On the reference shot, all three copies of the effect went to the card on every frame.
- The CPU drawing a frame planned for the card gives the same bytes as a frame planned for the CPU, at Full and at Draft, for all ten effects.

## The pictures

In `verification/B-65 pictures/`, for the worst comparison, frame 100 of the reference shot with three Hue/Saturations at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the reference shot with its colours turned, the sky purple-blue, the grass bright green and the red disc orange.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 79,669 of 2,073,600 pixels (3.8%) differ, each by 1 level of 255.
  - The two flat rectangles, green upper left and blue lower right, are off by 1 across their whole area: a flat colour that falls on a rounding edge differs everywhere or nowhere.
  - The rest are scattered edges and specks. The squares make them look far denser than they are.

Every other effect's worst frame differs in far fewer pixels, at most 3,782.

## Speed

`verification/B-65_gpu_fx_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b65_gpu_fx -- --ignored`), run on its own:

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD Ryzen 9 9900X with 24 threads; Windows 11; release build. The owner's app was open at the time.
- **What was timed:** every fourth frame, 60 in all, of the reference shot with three copies of one effect, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the frames twice.
- **Figures:** medians in ms, the second loop.

| Effect | Draft, CPU | Draft, GPU | Full, CPU | Full, GPU |
|---|---:|---:|---:|---:|
| Curves | 25.9 | 25.5 | 41.5 | 25.7 |
| Levels | 27.0 | 25.3 | 38.7 | 26.2 |
| Hue/Saturation | 26.2 | 27.5 | 39.0 | 27.1 |
| Gradient | 26.8 | 25.5 | 38.9 | 26.0 |
| Drop Shadow | 26.2 | 27.0 | 38.9 | 25.0 |
| Lens Blur | 26.4 | 28.2 | 39.0 | 26.2 |
| Rim Light | 26.3 | 26.3 | 39.0 | 24.7 |
| Outline | 26.3 | 25.7 | 38.2 | 25.1 |
| Noise | 26.2 | 30.3 | 82.7 | 37.6 |
| Chromatic Aberration | 26.1 | 25.4 | 38.8 | 26.5 |

**At Full, the card is about a third faster for every one of the ten:** 25 to 27 ms a frame against 38 to 42. With three Noises, the slowest on the CPU, it is more than twice as fast: 38 ms against 83. Every one is now inside a 24 fps frame's 41.7 ms.

**At Draft the two are about even**, within 2 ms either way, and Noise is 4 ms slower on the card. Draft already shrinks each drawing before its effects, so the effects are a small part of a Draft frame. Most of the roughly 25 ms is the rest of the frame: reading the drawings and laying them together. Moving these ten to the card cannot speed that part up.

## Limits

- One of the ten followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- A threshold Levels always runs on the CPU.
- Outline and Lens Blur need working memory on the card as well as the picture. It counts toward the card's memory budget; a frame over it is drawn by the CPU with a message.
- A card without double precision draws every frame with one of these on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
