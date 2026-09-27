# B-76: the second batch of ten on the graphics card

Built on 2026-09-26 against the B-76 entry in document 15, as the last step of the owner's "start the second effects batch". D-133, proposed, is its tolerance: the same 1 level of 255 as D-122.

## What it does

When the viewer draws on the card (**Draw on: Auto** or **GPU**), a drawn layer whose last effect is one of these nine has that effect done by the card rather than the CPU:

Distance Gradation, Light Rays, Exposure Flicker, Vignette, Turbulent Displace, Fractal Noise, Gradient Map, Color Balance and Offset.

The tenth, **Light Wrap**, is different. It is not part of the layer's own effects: it reads everything beneath the layer (D-132), so it runs as the layer is laid onto the picture. It now runs on the card too, wherever it sits among the layer's effects. Until this step, any frame with a Light Wrap was drawn by the CPU.

- The card follows the CPU's rule pixel by pixel, on the drawing sent in full precision.
- Exposure Flicker, Turbulent Displace and Fractal Noise make the CPU's own random numbers on the card, so the flicker, the wobble and the clouds are the same.
- Light Wrap softens what is beneath with the same blur Gaussian Blur already uses on the card.
- Any effects before it still run on the CPU first, as before.
- An effect that changes nothing (a Vignette of amount 0, an Offset of no shift, and so on) is not sent to the card at all.

**A finding on this card.** Offset first came back from the card as an empty picture. The cause is in the card's driver: a small shader function that hands back four double-precision numbers hands back zeros instead. The shaders now hand back ordinary precision from such functions and widen it where it is used. Nothing else changed, and the check below shows every effect agreeing with the CPU.

**What it does not change:**

- Exports, fixtures and **Draw on: CPU** are untouched. They never leave these effects to the card.
- An effect with invalid settings is left out with the warning `EFFECT_PARAMETER_INVALID` on both paths, as before.
- If the card cannot draw a frame, the CPU draws it and says so (`GPU_PREVIEW_ON_CPU`). The CPU's picture is then exactly the one it has always drawn.

**Left on the CPU:**

- one of the nine followed by another effect (a Light Wrap after it does not count; it is not in the layer's own effects)
- one on a composition, shape, solid or adjustment layer, or on a matte
- any frame with an adjustment layer in it, whatever its effects, by B-44's rule. The check makes sure the CPU's picture of such a frame is exact, byte for byte.

**Precision.** The card needs double precision (`SHADER_F64`), as for B-65. A card without it draws these frames on the CPU, with a message saying why. Not seen here: this card has it.

## The check

`verification/B-76_gpu_fx_table.md`, written by `tests/b76_gpu_fx.rs`, compares the eight-bit picture the viewer receives, drawn by the CPU and by the card, at Full and at Draft. It covers:

- every frame of every fixture of the ten, the ones with invalid settings included
- frames 0, 100 and 239 of the reference shot with each effect added to its first three layers. The second layer gets a Drop Shadow first, so the effect also runs on a drawing already grown by another. The settings are chosen to do something visible:

| Effect | Settings in the reference shot |
|---|---|
| Distance Gradation | dark blue, width 20, 70%, Multiply |
| Light Rays | from 50%, 40%, length 40, above 60, intensity 1.5, warm white |
| Exposure Flicker | 0.5 stops, held 3 frames, seed 4 |
| Vignette | 60, dark purple, size 90, roundness 50, softness 60, centre at 45%, 55% |
| Turbulent Displace | 12, size 50, complexity 3, evolution 30, speed 10, seed 5, edges transparent |
| Fractal Noise | size 80, complexity 5, contrast 130, brightness 5, speed 15, seed 3, dark blue to cream, 60%, Screen |
| Gradient Map | deep purple, rose, cream, midpoint 40, amount 80 |
| Color Balance | shadows bluer, midtones warmer, highlights yellower |
| Offset | shift 37.5 across, −21.25 up |
| Light Wrap | width 15, intensity 150, Screen |

**2460 of 2460 checks pass.**

- Every one of the ten, on every fixture frame and every reference frame, at both qualities, is within **1 level of 255** of the CPU's picture, with the same warnings on both paths.
- On each effect's fixtures, the effect went to the card on every frame where it does something with valid settings, except the frames with an adjustment layer. The table's **Frames with it on the card** column counts these, from 130 frames for Color Balance to 204 for Fractal Noise.
- On the reference shot, all three copies of the effect went to the card on every frame.
- Every frame with an adjustment layer was drawn by the CPU and was the CPU's picture exactly.

## The pictures

In `verification/B-76 pictures/`, for the worst comparison, frame 0 of the reference shot with three Vignettes at Full:

- `cpu.png`: what the CPU draws.
- `gpu.png`: what the card draws. It should look the same as `cpu.png`: the reference shot, its first three layers darkened a little toward dark purple at their edges. The darkening is gentle, so the picture looks much like the plain shot.
- `difference.png`: black where the two agree, and a white 7 by 7 square around every pixel where they do not. 3,937 of 2,073,600 pixels (0.2%) differ, each by 1 level of 255. They are scattered specks. The squares make them look far denser than they are.

## Speed

`verification/B-76_gpu_fx_timing_table.md`, written by the same test on 2026-09-26 (`cargo test --release --test b76_gpu_fx -- --ignored`), run on its own:

- **Machine:** RTX 4070 Ti SUPER, driver 610.74, Vulkan; AMD Ryzen 9 9900X with 24 threads; Windows 11; release build.
- **What was timed:** every fourth frame, 60 in all, of the reference shot with three copies of one effect, asked for as the viewer asks, whole: reading the drawings, the effects, drawing, and the eight-bit picture. Each path started with empty caches and played the frames twice.
- **Figures:** medians in ms, the second loop.

| Effect | Draft, CPU | Draft, GPU | Full, CPU | Full, GPU |
|---|---:|---:|---:|---:|
| Distance Gradation | 26.3 | 25.6 | 38.3 | 25.1 |
| Light Rays | 26.0 | 26.2 | 39.8 | 26.2 |
| Exposure Flicker | 28.2 | 28.7 | 80.8 | 34.2 |
| Vignette | 26.8 | 26.8 | 40.7 | 26.0 |
| Turbulent Displace | 27.1 | 29.1 | 225.9 | 53.7 |
| Fractal Noise | 26.2 | 28.3 | 122.7 | 38.7 |
| Gradient Map | 26.7 | 26.1 | 41.3 | 25.0 |
| Color Balance | 26.1 | 26.5 | 39.6 | 25.6 |
| Offset | 27.0 | 26.7 | 41.3 | 26.5 |
| Light Wrap | 40.3 | 26.6 | 150.1 | 53.6 |

**At Full, the card is faster for every one of the ten.** The seven light ones take 25 to 27 ms a frame against 38 to 42 on the CPU, about a third faster. The heavy ones gain most:

- Turbulent Displace: 54 ms against 226, four times as fast.
- Light Wrap: 54 ms against 150.
- Fractal Noise: 39 ms against 123.
- Exposure Flicker: 34 ms against 81.

Eight of the ten are now inside a 24 fps frame's 41.7 ms. Turbulent Displace and Light Wrap, at 54 ms, are not, but they were three to four times further from it.

**At Draft the two are about even**, within 2 ms either way, except Light Wrap: 27 ms on the card against 40. Draft already shrinks each drawing before its effects, so most of a Draft frame is reading the drawings and laying them together, which the card cannot speed up. Light Wrap is the exception because on the CPU it softens everything beneath each layer at the frame's full size.

## Limits

- One of the nine followed by another effect still runs on the CPU. So does one on anything but a drawn layer.
- A frame with an adjustment layer is still drawn by the CPU, Light Wrap or not.
- Light Wrap, Turbulent Displace, Light Rays and Distance Gradation need working memory on the card as well as the picture. It counts toward the card's memory budget; a frame over it is drawn by the CPU with a message.
- A card without double precision draws every frame with one of these on the CPU. Not seen on this machine.
- Checked on this machine's card only, as D-100 says.
