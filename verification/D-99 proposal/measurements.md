# D-99 proposal: a drawing's effects run at draft size in Draft preview

**Built and switched on in this build at the owner's "proceed with option 1" (2026-09-25), pending the owner's look at these pictures and a try in the app.** Nothing changes at Full quality or in export. Only Draft preview's pixels move, and only on drawn layers that have an effect.

## What changes

Draft preview shows the frame at a quarter of its width and height. Until now it still ran every effect on the full-size drawing, then shrank the result. D-99 shrinks the drawing first and runs the effect on the small one, with every distance in pixels divided by four: blur sizes, glow and bloom radii, streak lengths, line widths. The effect covers the same part of the picture either way. Only the fine detail inside it differs, because it was worked out on fewer pixels.

## How to judge it

Look at the twelve pictures in this folder. Each shows Draft preview before D-99, Draft preview with D-99, and a black panel that is white wherever the two differ, made four times brighter than the real difference so it can be seen. Each panel is doubled in size so single pixels show. If you cannot tell the first two panels apart, the change is safe to accept on looks. Remember that this is only the rough preview. Full quality and export are exactly what they were.

Each effect was put on two layers of frame 100 of the reference shot: the whole-frame background, and the small pink ball.

- `background_gaussian_blur.png`, `ball_gaussian_blur.png`: Gaussian Blur, sigma 8
- `background_glow.png`, `ball_glow.png`: Glow on bright parts over 40, radius 40, intensity 2
- `background_directional_blur.png`, `ball_directional_blur.png`: Directional Blur, direction 30, length 40
- `background_bloom_star.png`, `ball_bloom_star.png`: Bloom, radius 30, a star of streaks 60 long at angle 20
- `background_radial_blur.png`, `ball_radial_blur.png`: Radial Blur, zoom 20 from the centre
- `background_line_width.png`, `ball_line_width.png`: Line Width, 4 thicker, on black lines

## How far pixels move, and what it saves

Written by `tests/d99_draft_effects.rs` under `cargo test --release --test d99_draft_effects -- --ignored --nocapture`, to `measurements_table.md`. Pixel differences are in screen levels out of 255, over the viewer's grey. "Largest" is the single pixel that moves most. The next column is the share of all pixels that move by more than 3 levels.

- Machine: AMD Ryzen 9 9900X, 12 cores and 24 threads, Windows 11 Education 10.0.26200
- Build: rustc 1.89.0, release profile (`opt-level=3`), all threads
- Each time is the median of 5 renders of the whole frame with no cache, so each includes reading the reference shot's four drawings from disk. The same frame with no effect at all takes **47.8 ms**.

| Layer | Effect | Largest | Pixels moving more than 3 | Before, ms | After, ms | Times faster |
|---|---|---|---|---|---|---|
| background | Gaussian Blur, sigma 8 | 8 | 0.3% | 55.3 | 45.5 | 1.2x |
| background | Glow, bright parts over 40, radius 40, intensity 2 | 7 | 0.2% | 66.9 | 43.8 | 1.5x |
| background | Directional Blur, direction 30, length 40 | 34 | 14.8% | 55.0 | 44.1 | 1.2x |
| background | Bloom, radius 30, star streaks of 60 at angle 20 | 32 | 6.0% | 188.5 | 62.4 | 3.0x |
| background | Radial Blur, zoom 20 from the centre | 51 | 9.7% | 231.3 | 47.4 | 4.9x |
| background | Line Width, 4 thicker, black lines | 0 | 0.0% | 57.8 | 46.0 | 1.3x |
| ball | Gaussian Blur, sigma 8 | 1 | 0.0% | 50.9 | 45.8 | 1.1x |
| ball | Glow, bright parts over 40, radius 40, intensity 2 | 18 | 0.0% | 58.7 | 45.8 | 1.3x |
| ball | Directional Blur, direction 30, length 40 | 9 | 0.5% | 51.5 | 46.7 | 1.1x |
| ball | Bloom, radius 30, star streaks of 60 at angle 20 | 23 | 0.1% | 163.2 | 60.5 | 2.7x |
| ball | Radial Blur, zoom 20 from the centre | 6 | 0.3% | 56.3 | 44.3 | 1.3x |
| ball | Line Width, 4 thicker, black lines | 16 | 0.1% | 52.9 | 44.4 | 1.2x |

**Read the times this way.** Most of every row is the 47.8 ms of reading the drawings, which D-99 does not touch. Take that away and what is left is the effect's own cost. Before D-99 that was 7 to 19 ms for the lighter effects, 140 ms for Bloom's star and 184 ms for Radial Blur on the background. With D-99, every effect except Bloom is too small to see under the noise of reading the files, and Bloom is about 15 ms. The effect's own cost is what you wait for when you change a setting on a drawing already on screen, because the drawing is not read again.

**Where pixels move:** on fine brush strokes inside a blurred area, most with the blurs that smear along a line (Directional and Radial Blur on the textured background). The blurred shapes, their sizes and their edges are the same. The 14.8% for Directional Blur is many pixels each moving a little. They are the stroke texture in the water and grass, which the draft-size blur smooths a little more. The ball, a flat colour, barely moves at all. Line Width on the background moves nothing because the background has no black lines for it to thicken; on the ball's dark outline it does.

## What it does not cover

- Only drawn layers are shrunk before their effects. A shape, solid or adjustment layer with an effect still runs it at full size and then shrinks, as before. None of those is slower than it was.
- The shrunk drawing is not kept between frames; shrinking it again costs about 2 to 3 ms. The effect's result is kept, as P-11 keeps it, under a key that says it was made at draft size, so a draft result is never shown at Full.

## How to try it in the app

Put Bloom (star streaks) or Radial Blur on the background drawing, at Draft quality. Drag a setting and watch how quickly the picture follows, then switch to Full and compare the look.
