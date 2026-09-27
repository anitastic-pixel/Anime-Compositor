# P-21: the performance audit of the ten new effects

**The ten effects of B-54 to B-64 were timed alone on 2026-09-26 and four slow spots were made faster without moving a single pixel. Curves, Levels and Hue/Saturation are 3 to 15 times faster (13.3 ms to 1.0 ms for Curves on an opaque plate). Noise on a plate is 11.0 to 6.8 ms. Outline is about 30% faster. Lens Blur at radius 40 is 63.2 to 42.3 ms on a character. All 82 of 82 effect results, the older effects' included, have the same fingerprint before and after.**

Asked for by the owner on 2026-09-26: "now proceed to gpu-accelerate effects if not implemented along with a performance audit all around these new effects". This page is the audit on the processor. Moving the ten effects onto the graphics card is D-122 and B-65, which follow it.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads; 61.5 GB of memory
- Graphics card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 32.0.16.1074 (NVIDIA 610.74), through Vulkan. It is not used by these numbers.
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- **The owner's copy of the app was open throughout.**
- `tests/p16_effect_cost.rs`: 7 runs a case, median reported, run with `cargo test --release --test p16_effect_cost -- --ignored --nocapture`. This pass adds two Lens Blur cases, radius 40 and D-121's hexagon with highlights. **Before**: commit dfc44c2 (B-64) with the two cases added. **After**: the commit that carries this page. Both on the same day, a few minutes apart.

## What was changed

- **Curves, Levels, Hue/Saturation and Noise** (`src/grade.rs`). A cel is painted in flat colours, so most pixels on a row are exactly the pixel before them. Each row now remembers its last pixel: an identical one takes the same answer without working it out again. Noise still draws its own grain for every pixel, and reuses only the colour conversion.
- **Outline** (`src/layer_fx.rs`): the two passes that build the ring are shared between the threads.
- **Drop Shadow and Rim Light**: their first step, taking the drawing's shape, is shared between the threads. It was a small part of their cost, so their times barely moved.
- **Lens Blur**: the highlight step is shared between the threads, empty rows of the drawing are skipped, and away from the frame's edge a row's pixels are added without testing, for each pixel, whether it is past the edge. At radius 10 on an opaque plate it is no faster: there the cost is the adding itself, and that is the same work as before.
- Gradient and Chromatic Aberration were already quick and are unchanged.

## The ten effects, one at a time

As in P-20: **character** is a 1920x1080 figure in flat colours on nothing, **background** is an opaque plate, and "Picture" is the first 16 hex digits of the SHA-256 of the effect's result, every float by its bits. Bold is a case at least a fifth faster.

| Effect | Cel | Before, median ms | After, median ms | Picture (the same before and after) |
|---|---|---:|---:|---|
| **Curves, all four** | character | **3.5** | **1.1** | `49f9ea8340ade109` |
| **Levels, all five** | character | **3.4** | **1.0** | `4452e7ca6f9ca7c2` |
| **Hue/Saturation, all three** | character | **3.5** | **0.9** | `4ae241d61e7f5088` |
| Gradient, radial, screen | character | 2.3 | 2.0 | `9c720a32db3ea08f` |
| Drop Shadow, softness 6 | character | 21.3 | 21.5 | `af5aa1d6f96d453d` |
| Lens Blur, radius 10 | character | 27.2 | 23.4 | `fd4dba0827c795ce` |
| **Lens Blur, radius 40** | character | **63.2** | **42.3** | `5b78a58e670f402f` |
| Lens Blur 10, hexagon, highlights | character | 28.8 | 26.3 | `1702a2254c6ad7c7` |
| Rim Light, the defaults | character | 15.4 | 15.2 | `c7204affb817278f` |
| **Outline, width 3, softness 2** | character | **27.9** | **19.8** | `f23109e191358043` |
| **Noise, amount 10, colour** | character | **2.9** | **2.0** | `cedd8a1d39531259` |
| Chromatic Aberration, amount 3 | character | 8.7 | 8.1 | `039fb42d05f5fe77` |
| **Curves, all four** | background | **13.3** | **1.0** | `fd5828ba8b35eea8` |
| **Levels, all five** | background | **14.7** | **1.0** | `1596f82e99cc0fa0` |
| **Hue/Saturation, all three** | background | **12.8** | **1.0** | `df9018a7c13f28e3` |
| Gradient, radial, screen | background | 7.3 | 7.8 | `4067848b2949de19` |
| Drop Shadow, softness 6 | background | 19.4 | 18.7 | `9bb64e8144185cdd` |
| Lens Blur, radius 10 | background | 25.3 | 25.4 | `7eb0f2e777fb519f` |
| Lens Blur, radius 40 | background | 61.5 | 52.2 | `e37b918c5501e679` |
| Lens Blur 10, hexagon, highlights | background | 27.4 | 26.7 | `5c15a974d735f870` |
| Rim Light, the defaults | background | 17.2 | 16.9 | `e83dca4aa6729a73` |
| **Outline, width 3, softness 2** | background | **27.1** | **18.8** | `c6f453268db51fcb` |
| **Noise, amount 10, colour** | background | **11.0** | **6.8** | `1480877010898993` |
| Chromatic Aberration, amount 3 | background | 9.1 | 8.4 | `facf68197c79c38c` |

The other 58 cases, the thirteen older effects, were run too. Their pictures are also unchanged. None of their code was touched, and their times moved only by the usual spread from one run to the next, at most 9% (Bloom with streaks on a plate, 153.1 then 166.6 ms).

## What is left, and not done here

- **The blurs inside Drop Shadow, Rim Light and Outline** still soften all four channels when only the shape is used. Softening the shape alone would take roughly a third off each. It was not done here because it touches the shared blur.
- **Lens Blur is still the costliest of the ten**, at 23 to 52 ms. On the processor its cost grows with the radius. The card is the way past that, and it is B-65.

## How to check this

1. Open the app on a project with a character layer and add **Curves**. Drag a point: the picture should follow the mouse at once, where before a large frame lagged a little. Do the same with **Levels** and **Hue/Saturation**.
2. Add **Outline** of 3 and drag its width, then **Lens Blur** of 40.
3. In every case the picture should look exactly as it did before this change. Nothing about how the effects look was touched.
