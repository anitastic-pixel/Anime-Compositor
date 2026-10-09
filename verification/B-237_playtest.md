# B-237: Bilateral Blur

Built on 2026-10-08 under your effects loop request, decided as D-358. **Bilateral Blur**, in
**Blur & Sharpen**, is after After Effects' Bilateral Blur. Each pixel is mixed with the pixels
round it, and how much each one counts depends on two things: how near it is, and how alike in
colour. Grain and small blotches on a flat area are smoothed away. A strong edge, such as a dark
line against skin, is left sharp, because the pixels across it are too unlike to be mixed in. It
is a softer cousin of Smart Blur: Smart Blur counts a neighbour fully or not at all, while
Bilateral Blur fades it out gradually as it gets less alike.

Its settings, with the values it starts at:

- **Radius** (5): how far round each pixel it looks, 0 to 50 pixels. 0 does nothing.
- **Threshold** (20): how unlike, in levels of 255, a neighbour can be and still count much, 0 to
  255. 0 does nothing. Low values keep more edges; high values blur more, edges included.
- **Colorize** (On): On smooths red, green and blue each by its own likeness and keeps the
  colours. Off works on brightness alone, and the layer comes out **grey**.

The check, `verification/D-358_bilateral_blur_table.md` (123 of 123), holds every pixel to numbers
worked out by a separate program before the code existed. It also holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-358 pictures/`,
three times enlarged, on a scanned face with grainy skin.

## What to check

Compare the pictures first. **`before.png`** is the face with no effect: grainy skin, specks and
a few pinholes. **`bilateral_default.png`** is the same face with Bilateral Blur as it starts.

1. **Grain smoothed, lines kept.** In `bilateral_default.png` the skin should look smooth, with
   the grain mostly gone. The outline, the eyes, the brow line and the dark specks should be as
   sharp as in `before.png`. The pinholes should still be see-through.
2. **More smoothing.** `bilateral_12_30.png` is Radius 12, Threshold 30. The skin should be
   smoother still, and the lines still sharp.
3. **Too high a threshold.** `bilateral_12_60.png` is Radius 12, Threshold 60. Now some skin
   colour leaks into the thin brow line, which turns lighter. This is expected: the higher the
   threshold, the less the edges are protected.
4. **Colorize off.** `bilateral_colorize_off.png` should be the smoothed face in grey.
5. **In the app.** Put Bilateral Blur on a grainy layer of the reference shot (add Noise first if
   you have no grainy footage). Drag Radius and Threshold and watch the grain go while the edges
   stay. With Draw on: GPU it should play smoothly at the starting values.
6. **Keys.** Key Radius from 0 to 10 over a second and play. The smoothing should grow from
   nothing.
7. **Save and open.** Save, close and reopen. The three settings and the keys should be as you
   left them.

## Not done, on purpose or for later

- **Speed** (`verification/B-237_bilateral_blur_timing_table.md`): on the graphics card it adds
  about 1.5 ms a frame for each 1080p layer as it starts, and about 2.4 ms at Radius 10, within the
  target. On the processor, which draws exports, it is slow: about 65 ms a layer as it starts and
  about 220 ms at Radius 10, and it gets slower with the square of the radius.
- **The starting values and ranges are our own pick.** Adobe's help pages could not be read, so
  Radius 5, Threshold 20, Colorize On and the 0 to 50 radius range are chosen to feel familiar,
  not copied.
- **How alike counts is our own rule.** Adobe does not publish its method. Ours fades a neighbour
  out along a bell curve whose width is Threshold (written down in document 21).
- **Colorize off gives grey.** We read After Effects' Colorize off as working on brightness alone,
  so the result is grey. If you expected Colorize off to keep the colours, say so.

**Tutorial gaps** (steps we could not reproduce, logged rather than worked round):

- The tutorials found for Bilateral Blur are videos, which could not be followed step by step
  here, so no tutorial was replayed exactly.
- A "filmic glow" look in them puts a Bilateral-Blurred copy of a layer over the original with a
  blending mode. There is no single effect for that here. It can be built by hand with a duplicate
  layer, but it has not been checked against a tutorial.

## If something is wrong

Say which step. The most likely faults would be in step 1 (lines softened, or the grain not
smoothed) or step 4 (colour left in with Colorize off).
