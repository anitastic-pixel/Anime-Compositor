# B-422: Soft Physical Glow's tint

Built on 2026-10-11 as D-542, your pick #6 (glow core-to-edge colour, 透過光). It is not a new
effect: Soft Physical Glow (Glow with Falloff set to Physical) has four new settings, after the
tint of the glow plugin `docs/effects/PLUGINS.md` reads. The arithmetic is our own.

- **Tint** (Off): Off is the glow as before. One Colour colours the whole glow the Inner Colour.
  Core to Edge colours it the Inner Colour close to the bright parts and the Outer Colour out in
  the tail, shading from one to the other across the glow's sizes.
- **Inner Colour** (white) and **Outer Colour** (blue, #4080ff).
- **Tint Amount** (100): how much of the tint; 0 is the glow's own colours. It can be keyed.

A grey or white light glows the tint colour as strongly as a coloured light of the same
brightness, so a white line can glow orange.

Not built: the plugin's Overlay and Soft Light ways of laying the tint on (ours is one way, like
its Multiply), and its Swap Colors (swap the two colours by hand instead).

The check, `verification/D-542_glow_tint_table.md` (285 of 285), holds every pixel to numbers
worked out by a separate program before the code existed, runs all 45 of Soft Physical Glow's
earlier checks again unchanged (a glow without a tint draws exactly as before), and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-542 pictures/`.

## What to check

1. **No tint.** `untinted.png`: the town with Soft Physical Glow (threshold 60, smooth 50,
   radius 60). The glow takes the town's own warm colours.
2. **One Colour.** `one_colour_orange.png`: the same with Tint One Colour, orange. All of the
   glow is orange, the pale sky's glow too.
3. **Core to Edge.** `core_to_edge.png`: Tint Core to Edge as added: white close to the bright
   parts, blue further out.
4. **Wide.** `core_to_edge_wide.png`: Core to Edge, orange to purple, radius 200: a long purple
   haze round the frame with an orange tinge close in.
5. **In the app.** Add Glow, set Falloff to Physical, then set Tint to Core to Edge. Pick an
   Inner Colour and an Outer Colour and change Radius: the edge colour should stay out in the
   tail. Key Tint Amount from 0 to 100 and play it: the glow fades from its own colours to the
   tint.
6. **Out of range.** Type 101 in Tint Amount: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.
   An older project with Soft Physical Glow opens and looks as it did.

Our own choices you may want to judge: the colours as added (white and blue); how quickly the
colour changes from core to edge (a step at each of the glow's sizes); and that the brightness
used is the light's brightest channel, so pure white light keeps its full strength when tinted.

## Speed

Measured, but PROVISIONAL: other builds were running on this machine at the time, so the numbers are higher and noisier than a quiet machine gives. On the graphics card, the reference shot with colour noise and then the glow on its three layers took 55.6 ms a frame without a tint and 75.6 ms with Core to Edge (radius 60, playing it again); the glow as added (radius 500) with Core to Edge took 94.3 ms; the noise alone took 15.7 ms. The full table is `verification/B-422_glow_tint_timing_table.md`. What to check by eye: whether scrubbing a tinted glow in the viewer still feels smooth enough to you.
