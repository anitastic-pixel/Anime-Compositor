# B-388: Energy Stroke

Built on 2026-10-11 as D-508, under your /loop request: Energy Stroke, in **Generate**, our own
take on what Video Copilot's Saber does (PLUGINS.md pick #11). It draws a glowing beam: a
white-hot core inside a coloured glow.

- **Path From** (Points): Points draws a straight beam from Core Start to Core End; Masks,
  Shapes and Text Outlines draw it along every mask, shape or letter outline of the layer.
- **Core Start** (25, 50) and **Core End** (75, 50): the beam's two ends, in per cent of the
  layer's width and height, for Points.
- **Evolution** (0): where on a closed path (a mask, a letter) the beam begins; a whole turn
  is 360.
- **Start Offset** (0) and **End Offset** (100): the part of the path drawn. Keyframe End
  Offset from 0 to 100 and the beam draws itself on.
- **Start Size** and **End Size** (100): how thick the beam is at each end; 0 at one end
  tapers it to a point.
- **Core Size** (8) and **Core Softness** (30): how wide the white core is and how soft its edge.
- **Glow Color** (blue), **Glow Intensity** (100) and **Glow Spread** (40): the glow's colour,
  brightness and reach.
- **Flicker Amount** (0) and **Flicker Speed** (20): the whole light wavering.
- **Distortion Amount** (0), **Distortion Scale** (60) and **Distortion Speed** (20): the beam
  wobbling like plasma, how far, how long the wobbles are, and how fast they move.
- **Random Seed** (0): a different flicker and wobble.
- **Composite** (Add): Add puts the light over the layer; Transparent shows the light alone;
  Black shows it over black.

Not built: Saber's presets, Glow Bias, Halo, tendrils and the other finer controls, and choosing
another text layer as the path. Where two paths cross, the brighter glow wins rather than the two
adding up; a second copy of the effect does add. The beam is drawn inside the layer: near the
layer's edge it is cut off there.

Saber publishes no formula and no settings when added, so the rule and those settings are ours,
written down in document 21.

The check, `verification/D-508_energy_stroke_table.md` (296 of 296), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-508 pictures/`.

## What to check

1. **As added.** `1_as_added.png`: a white beam across the middle of the street with a blue glow
   round it, fading over the street.
2. **An orange saber.** `2_orange_saber.png`: a thinner diagonal beam with a strong orange glow.
3. **Round a mask.** `3_round_a_mask.png`: a green beam running seven tenths of the way round a
   box mask, from its top-left corner.
4. **Wobble.** `4_wobble.png`: a purple beam round the whole box, wobbling but still joined up.
5. **Over black.** `5_over_black.png`: the blue beam alone on black, the street gone.
6. **In the app.** Add Energy Stroke (Generate) to a layer. Drag Core Start and Core End.
   Keyframe End Offset from 0 to 100 and play: the beam draws on. Turn up Flicker Amount and
   play: the light wavers. Turn up Distortion Amount and play: the beam wobbles and moves.
7. **On a mask and on text.** Draw a mask, set Path From to Masks; on a text layer, set Path From
   to Text Outlines: every letter glows round its edge. With Masks and no mask, the layer is
   drawn as it is and a warning says there is nothing to draw along.
8. **Out of range.** Type 201 in Core Size: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how soft the glow is; how the
wobble and flicker move.

## Speed

`verification/B-388_energy_stroke_timing_table.md`. Three Energy Strokes on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL: measured on 2026-10-11 while another lane's GPU test shared the card, so the figures (card 157 to 283 ms a frame, Noise alone 262) measure the busy machine, not the effect; a quiet re-measure is owed.
