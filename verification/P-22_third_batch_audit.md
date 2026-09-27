# P-22: the performance audit of the third batch of effects

**The thirty effects of B-77 to B-106 were timed alone on 2026-09-27 and six slow spots were made faster without moving a single pixel. Cross Glare on a character is 262.2 ms to 73.9 ms, and 257.3 to 153.5 ms on an opaque plate. Find Edges is 2.3 to 3 times faster (40.2 to 13.4 ms on a plate). Invert, Leave Color and Solarize on a plate are 10 to 13 times faster (12.4 to 0.9 ms for Invert). Mosaic is 3.4 times faster. All 160 of 160 effect results, the older effects' included, have the same fingerprint before and after.**

Asked for by the owner on 2026-09-27: "now let's do a full performance audit like we did with these new effects, like gpu-acceleration, etc." This page is the audit on the processor. Moving the thirty onto the graphics card is D-165 and B-107, which follow it.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads; 61.5 GB of memory
- Graphics card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 32.0.16.1074 (NVIDIA 610.74), through Vulkan. It is not used by these numbers.
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- **The owner's copy of the app was not open.**
- `tests/p16_effect_cost.rs`: 7 runs a case, median reported, run with `cargo test --release --test p16_effect_cost -- --ignored --nocapture`. **Before**: commit 844aa8a (B-106 and its follow-up). **After**: the commit that carries this page. Both on the same day, a few minutes apart.

## What was changed

- **Cross Glare** (`src/layer_fx.rs`). Each arm of the glare adds up samples of the bright parts along a line. Most samples land where nothing is bright, and add exactly nothing. A count of the bright pixels is now made once, so a sample whose four surrounding pixels are all dark is skipped without being worked out, and a whole output pixel with no bright pixel in reach is skipped too. The additions that do happen are the same ones, in the same order, so the answer is the same to the last bit.
- **Invert, Leave Color and Solarize** (`src/grade.rs`). A cel is painted in flat colours, so most pixels on a row are exactly the pixel before them. As P-21 did for Curves, Levels and Hue/Saturation, each row now remembers its last pixel, and an identical one takes the same answer.
- **Find Edges**: its first step, the brightness of every pixel, is shared between the threads.
- **Mosaic**: the rows of blocks are shared between the threads.
- The other twenty-four were already quick, or their cost is work that cannot be skipped without changing the answer, and they are unchanged.

## The thirty effects, one at a time

As in P-20 and P-21: **character** is a 1920x1080 figure in flat colours on nothing, **background** is an opaque plate, and "Picture" is the first 16 hex digits of the SHA-256 of the effect's result, every float by its bits. Bold is a case at least a fifth faster and at least 1 ms saved. Cases at 0.0 ms took under a twentieth of a millisecond; Motion Tile at its defaults repeats the drawing once, which is a copy.

| Effect | Cel | Before, median ms | After, median ms | Picture (the same before and after) |
|---|---|---:|---:|---|
| **Invert, a negative** | character | **3.0** | **0.9** | `867bc535cd67215b` |
| Brightness & Contrast, both moved | character | 0.8 | 0.9 | `c741319fb50c91cf` |
| Black & White, a red filter | character | 0.8 | 0.9 | `7624f1a4219b83f2` |
| Posterize, six levels | character | 0.8 | 0.9 | `0b6e5cca6ed3e6db` |
| Threshold, at the middle | character | 0.8 | 0.9 | `b8fa70f6ac3cd06b` |
| Channel Mixer, red and blue swapped | character | 0.9 | 0.8 | `110129fd09606cc0` |
| Vibrance, an everyday grade | character | 0.9 | 0.9 | `8abc92100e6ffc8b` |
| **Leave Color, the reds kept** | character | **3.0** | **0.9** | `e8f7ca75f7d43d90` |
| **Solarize, at the middle** | character | **2.4** | **1.0** | `37a5a3d0b7fd94b9` |
| Halftone, as it starts | character | 2.2 | 2.2 | `5a7484e4d2609bc5` |
| **Mosaic, as it starts** | character | **3.8** | **1.1** | `1f54a5518d89da84` |
| Emboss, as it starts | character | 8.2 | 7.6 | `ddaf6d5e7d9d855e` |
| **Find Edges, as it starts** | character | **12.7** | **5.5** | `29732055a4379d4e` |
| Sharpen, as it starts | character | 19.5 | 17.3 | `17bc694a2b21db90` |
| Diffusion, as it starts | character | 15.6 | 13.7 | `b5158871b922a5ee` |
| Wave Warp, as it starts | character | 6.7 | 6.7 | `0b8951e68c24e270` |
| Ripple, as it starts | character | 10.5 | 9.7 | `1826361c40b7c3d3` |
| Twirl, as it starts | character | 5.8 | 5.1 | `2e719e5df6b21dbe` |
| Bulge, as it starts | character | 6.1 | 5.2 | `538aec187959768d` |
| Mirror, as it starts | character | 6.3 | 5.7 | `42b10d7491e19804` |
| Motion Tile, as it starts | character | 0.0 | 0.0 | `72cac54dd11e4165` |
| Linear Wipe, half way | character | 1.2 | 1.2 | `9d1e780329b9866f` |
| Radial Wipe, half way | character | 3.1 | 2.7 | `1bbaffb4e7f808d5` |
| Venetian Blinds, half way | character | 1.3 | 1.3 | `48512de09eef45ad` |
| Iris Wipe, half way | character | 1.9 | 1.6 | `72cac54dd11e4165` |
| Simple Choker, spread 3 | character | 16.1 | 15.0 | `0405155e032240a7` |
| Speed Lines, as they start | character | 2.3 | 2.2 | `e7e638024252bdff` |
| **Cross Glare, as it starts** | character | **262.2** | **73.9** | `76fe35062f44e547` |
| Camera Shake, as it starts | character | 4.5 | 4.3 | `0255fbc0fb505cb3` |
| Rain, as it starts | character | 1.8 | 1.4 | `00663570c3957377` |
| **Invert, a negative** | background | **12.4** | **0.9** | `b3bf29f4c634102a` |
| Brightness & Contrast, both moved | background | 0.9 | 1.0 | `84e7bb8f0d208a4e` |
| Black & White, a red filter | background | 0.9 | 0.9 | `c228405bfd4ac5ae` |
| Posterize, six levels | background | 0.9 | 1.0 | `8aa313c4bc1e77b6` |
| Threshold, at the middle | background | 0.9 | 0.9 | `240ecef2dceb9e6f` |
| Channel Mixer, red and blue swapped | background | 0.9 | 0.9 | `5294246ff9ee5612` |
| Vibrance, an everyday grade | background | 0.9 | 1.0 | `5aa74908dbe7a857` |
| **Leave Color, the reds kept** | background | **11.9** | **0.9** | `712804200b6652dc` |
| **Solarize, at the middle** | background | **9.7** | **0.9** | `da0ac5ce5b920b3b` |
| Halftone, as it starts | background | 6.8 | 7.1 | `4b567ef2ae10fcbe` |
| **Mosaic, as it starts** | background | **3.7** | **1.1** | `4e2af2d1b68e8d1e` |
| Emboss, as it starts | background | 18.0 | 17.7 | `eed3ada980eb8b6f` |
| **Find Edges, as it starts** | background | **40.2** | **13.4** | `1d4690bf517ee184` |
| Sharpen, as it starts | background | 30.9 | 28.9 | `f25f060bcea67999` |
| Diffusion, as it starts | background | 16.0 | 14.6 | `9f9fbe1a5710979a` |
| Wave Warp, as it starts | background | 6.4 | 6.3 | `10957844bee47352` |
| Ripple, as it starts | background | 10.3 | 9.4 | `a54f1770b34cfdba` |
| Twirl, as it starts | background | 6.3 | 5.4 | `4c028ebcd1177d28` |
| Bulge, as it starts | background | 5.9 | 5.2 | `14e924c1ac8a51df` |
| Mirror, as it starts | background | 6.2 | 5.8 | `10f1f39f5bc566f1` |
| Motion Tile, as it starts | background | 0.0 | 0.0 | `5e8c26a9f8d4132a` |
| Linear Wipe, half way | background | 1.3 | 1.1 | `8ee5d77adea38605` |
| Radial Wipe, half way | background | 3.1 | 2.7 | `52ba324b28659612` |
| Venetian Blinds, half way | background | 1.4 | 1.2 | `4696d9be4a61c649` |
| Iris Wipe, half way | background | 1.7 | 1.5 | `ea6edab3f86cacc0` |
| Simple Choker, spread 3 | background | 16.8 | 16.8 | `d5c713ec71dd8182` |
| Speed Lines, as they start | background | 6.7 | 6.7 | `797d147399936197` |
| **Cross Glare, as it starts** | background | **257.3** | **153.5** | `af58bbcaec4821c1` |
| Camera Shake, as it starts | background | 4.3 | 4.5 | `a61c573aac60b613` |
| Rain, as it starts | background | 4.7 | 4.4 | `0b21f2ebee9fb31e` |

The other 100 cases, the fifty older effects, were run too. Their pictures are also unchanged. None of their code was touched, and their times moved only by the usual spread from one run to the next; among cases of 5 ms or more the largest was 13% (Offset, a part-pixel slide, on a plate, 7.6 then 6.6 ms).

## What is left, and not done here

- **Cross Glare on a plate is still 153.5 ms**, the costliest of the thirty. On a plate much more of the picture is bright, and each bright sample must still be added. The card is the way past that, and it is B-107.
- **Emboss, Sharpen and Diffusion** cost 7.6 to 28.9 ms. Their time goes on the conversions between stored light and the picture's brightness scale, and on the blur, for every pixel. There is no saving there that keeps every bit the same.
- **Simple Choker** costs about 16 ms, and **Halftone** 7 ms on a plate. Both are work per pixel that cannot be skipped.
- All thirty except Motion Tile go to the card in B-107.

## How to check this

1. Open the app on a project with a character layer and add **Cross Glare**. Drag its length: the picture should follow much more quickly than before.
2. Add **Find Edges**, **Mosaic**, **Invert**, **Leave Color** and **Solarize** to a full-frame picture, and drag their settings.
3. In every case the picture should look exactly as it did before this change. Nothing about how the effects look was touched.
