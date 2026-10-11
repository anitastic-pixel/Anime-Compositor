# B-387: CC Spotlight

Built on 2026-10-11 as D-507, under your /loop request: CC Spotlight, in **Camera & Lens** (the
panel has no Perspective group), with CycoreFX's settings. It shines a spotlight cone down onto
the layer from a lamp you place above it.

- **From** (25, 25): where the lamp is, in per cent of the layer's width and height.
- **To** (50, 50): where the lamp points.
- **Height** (50): how far above the layer the lamp is, in per cent of the layer's width. Low
  and to one side stretches the pool of light into a long oval.
- **Cone Angle** (40): how wide the beam opens, 0 to 180 degrees.
- **Edge Softness** (20): how soft the rim of the beam is: 0 is a hard edge, 100 fades all the
  way from the middle.
- **Color** (white): the light's colour.
- **Intensity** (100): how strong the light is; in Light Shadow, how dark outside the cone.
- **Render** (Light Shadow): Light Only (the light alone, the layer gone), Light Add (the light
  added where the layer is), Light Add+ (added everywhere, the transparent parts too) and Light
  Shadow (the layer lit inside the cone, darkened outside, tinted by the colour).

Not built yet: the four Gel renders and Gel Layer (which project another layer through the
light, like a slide). A project asking for a Gel render keeps the effect as written, leaves it
out of the picture and says why.

CycoreFX's manual describes the controls but gives no formula and no settings when added, so
the rule and those settings are ours, written down in document 21. The light does not fade with
distance from the lamp.

The check, `verification/D-507_cc_spotlight_table.md` (167 of 167), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-507 pictures/`.

## What to check

1. **As added.** `1_as_added.png`: the street lit by an oval pool of light round the middle,
   with a soft rim; outside it the street is black.
2. **Light Only.** `2_light_only.png`: only the white pool of light, no street.
3. **Light Add, orange.** `3_light_add_orange.png`: the street as it was, with orange light
   added inside the cone, strong enough to wash the middle out towards white.
4. **Low and wide.** `4_low_and_wide.png`: the lamp low and the cone wide: a long oval of light
   reaching right across the street.
5. **Hard edge.** `5_hard_edge.png`: Edge Softness 0: the pool's rim is a sharp step.
6. **In the app.** Add CC Spotlight (Camera & Lens) to a layer and try each Render, then drag
   From and To and change Height, Cone Angle and Edge Softness. Keyframe From and play it: the
   light should swing smoothly across the layer.
7. **Out of range.** Type 181 in Cone Angle: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how the edge fades; and that
the light does not grow weaker far from the lamp.

## Speed

Measured on a quiet machine (`verification/B-387_cc_spotlight_timing_table.md`). Three CC Spotlights on a 1920 by 1080 shot,
card / processor, milliseconds a frame played again: the shot after a moving Noise 11.8 / 45.9 ms a frame, light shadow as added 14.7 / 56.6, light add orange at intensity 250 14.5 / 56.0, light add plus low and wide 14.5 / 54.0; about 0.9 to 1.0 ms a 1080p layer on the card, at the row's Target P1 (1 ms), and about 2.7 to 3.6 on the processor.
