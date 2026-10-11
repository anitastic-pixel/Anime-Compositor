# B-399: CC Spotlight gels

Built on 2026-10-11 as D-519, under your /loop request: the four Gel renders and the Gel Layer of
**CC Spotlight** (Perspective), which B-387 left out. The light now shines through another layer,
like a slide projector throwing a slide onto a wall.

- **Gel Layer** (none): the layer the light shines through. Its clear parts let white light
  through; its coloured parts colour the light.
- **Render**, four new choices:
  - **Gel Only**: the layer itself goes; only the coloured light is left.
  - **Gel Add**: the coloured light is added onto the layer.
  - **Gel Add+**: the same, and the light also lands on the layer's transparent parts.
  - **Gel Shadow**: dark outside the beam; inside, the layer lit through the gel.

The gel always fills the beam: its longer side reaches the edge of the circle of light, and it
is centred. A light straight down shows it the right way round; a low light stretches it.
Only the Gel renders look at the Gel Layer. A project saved before this opens as it was.

CycoreFX publishes no formula, so the rule is ours, written down in document 21.

The check, `verification/D-519_cc_spotlight_gel_table.md` (148 of 148), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-519 pictures/`.

## What to check

1. **The street and the slide.** `1_street.png` is the shot; `2_gel.png` is the slide: four
   coloured panes (red, green, blue, yellow) on a clear background.
2. **Gel Only.** `3_gel_only.png`: the street gone; a round pool of white light on black with the
   four panes thrown in its middle, red top left, green top right, blue bottom left, yellow
   bottom right.
3. **Gel Add.** `4_gel_add.png`: the street with the pool of light added, the panes' colours
   tinting it in the middle.
4. **Gel Shadow.** `5_gel_shadow.png`: black outside the pool; inside, the street lit through
   the panes, each quarter tinted its pane's colour.
5. **A low light.** `6_low_light.png`: Gel Only with the lamp low at the top left: the pool and
   the panes stretched out into a long slanted shape.
6. **In the app.** Add CC Spotlight to a layer over another, choose that layer as the Gel Layer,
   choose Gel Add, and drag From and To about: the picture moves with the light.
7. **No gel chosen.** With Gel Layer set to none, Gel Add looks exactly like Light Add.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.
   A project with a CC Spotlight saved before today opens with no Gel Layer and looks the same.

Our own choices you may want to judge: the gel always filling the beam, with no way to size,
move or turn it.

## Speed

`verification/B-399_cc_spotlight_gel_timing_table.md`. Three CC Spotlights with a gel on a 1920 by 1080 shot, milliseconds a frame
played again: PROVISIONAL (machine not quiet, other lanes' cargo runs sharing the machine), card 167.0, 141.7 and 99.7 ms a frame (gel add, opening 90; gel shadow; gel only, low and wide; layer 4 as the gel in each) against 51.0 for the Noise alone, processor 140.9, 141.3 and 150.6 against 81.5; a quiet re-measure is owed.
