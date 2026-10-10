# B-295: Fractal

Built on 2026-10-10 as D-416, under your /loop request: After Effects' Fractal, in **Generate**.
It draws the Mandelbrot or Julia set in place of the layer: the set itself black, the outside in
bands of colour by how quickly each point runs off to infinity.

- **Set** (Mandelbrot): Mandelbrot, Mandelbrot Inverse, Mandelbrot Over Julia, Mandelbrot Inverse
  Over Julia, Julia, Julia Inverse. The inverse ones turn the picture inside out.
- **Equation** (z = z^2 + c): up to z^6; higher powers give more fold-outs round the set.
- **Mandelbrot Center**, **Magnification**, **Escape Limit** (-0.75, 0; 0; 100): where you look,
  how close (each step of 1 is twice as close) and how much detail is worked out.
- **Julia Center**, **Magnification**, **Escape Limit** (0, 0; 0; 100): the same for the Julia set.
  The Julia set drawn is the one belonging to the Mandelbrot Center's point.
- **Overlay** (Off): On shows the other set as a pale ghost and a white cross at the middle,
  handy for picking a Julia point.
- **Transparency** (Off): On makes the black set see-through.
- **Palette** (Lightness Gradient): Lightness Gradient, Hue Wheel, Black And White, Solid Color.
- **Hue** (0 degrees): the colour the palette starts on.
- **Cycle Steps** (10), **Cycle Offset** (0): how many bands before the colours repeat, and which
  band they start on.
- **Edge Highlight** (Off): white lines where the bands change, only when nothing is oversampled.
- **Oversample Method** (Edge Detect), **Oversample Factor** (2): smooths the band edges by
  working several points a pixel, only along the edges (Edge Detect) or everywhere (Brute Force).

Adobe's page describes the effect in words and gives no formula, so how each pixel is worked out
is our own rule, written down in document 21. A draft turns the smoothing off and draws half size,
so the same view shows. A very heavy Fractal (a large picture with a very high Escape Limit) is
drawn by the processor rather than the graphics card, so Windows never stops the card mid-frame;
it looks the same, only slower.

The check, `verification/D-416_fractal_table.md` (278 of 278), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's (on the test pictures it matched exactly). The pictures are in
`verification/D-416 pictures/` (`town.png` is the layer; Fractal replaces it, so it does not show).
In these pictures a see-through part shows as white or as your viewer's checker.

## What to check

1. **As added.** `2_as_added.png`: the familiar Mandelbrot shape in black, ringed by red bands
   getting lighter toward it, a darker strip on the left.
2. **Julia, Hue Wheel.** `3_julia_hue_wheel.png`: a black shape of round blobs in a row, ringed by
   orange, yellow, green, blue and pink bands.
3. **Seahorse valley.** `4_seahorse.png`: zoomed far in, curling seahorse tails in many colours on
   orange and green.
4. **Solid, clear.** `5_solid_clear.png`: the Mandelbrot shape alone in blue, the rest see-through.
5. **Overlay.** `6_overlay.png`: thin white lines along the band edges, the Julia set as a pale
   grey ghost over the black, and a small white cross with a dark shadow at the middle.
6. **In the app.** Add Fractal (Generate) to a layer: the Mandelbrot set replaces it. Raise
   Mandelbrot Magnification to 3: it zooms in on the middle. Drag Mandelbrot Center: the view
   moves. Key Magnification from 0 to 15 over a few seconds and play: a zoom into the set. Set
   Palette to Hue Wheel and turn Hue: the colours cycle. Set Set to Julia and move Mandelbrot
   Center: the Julia shape changes. Switch to Draft: the same picture, a little rougher.
7. **Out of range.** Type 0 in Mandelbrot Escape Limit: it is refused with a sentence saying it
   runs from 1 to 10000.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

Measured only provisionally so far: other programs were using the processor and the graphics
card at the time, so these numbers are rough and will be measured again on a quiet machine
(`verification/B-295_fractal_timing_table.md`). On the test shot (three 1920 by 1080 layers),
Fractal as added cost roughly 7 to 15 ms a layer a frame; Brute Force 4 by 4 about 85 ms a
layer; and a deep zoom with Escape Limit 1000 over 400 ms a layer, on the card and the
processor alike. High Escape Limits and Brute Force are slow; keep them for final renders.
