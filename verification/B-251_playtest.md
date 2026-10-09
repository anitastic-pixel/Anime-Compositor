# B-251: letters placed as FreeType and Windows place them (the side-bearing slide)

Built on 2026-10-09 by your choice on D-372, option (a): "go with your recommendation on D-372".

A TrueType font stores each letter's outline and, separately, how far the letter's left edge sits
from the pen (its "left side bearing"). In about half the letters of the bundled font the two
disagree by a few font units. FreeType, HarfBuzz, Windows and fontTools trust the side bearing and
slide the outline to meet it; we used to draw the outline where it was stored. Now we slide it
too. Fonts of the other kind (CFF, usually `.otf`) have no such table and are unchanged.

What moves: in plain Latin at size 100, A, V, W, Y, x, y, v, w and a few others, by up to 0.7
pixel to the right; a few Japanese brackets by up to about 3 pixels at size 100. Everything else is
byte for byte as before. Spacing between letters does not change: only where each letter's shape
sits inside its own space.

**The picture:** `verification/B-251 pictures/before_after.png`. "AVWY" at size 100, before and
after, three times bigger; then both laid over each other six times bigger: a red edge is where
only the old letter was, a cyan edge where only the new one is. The A and V move right by less
than half a pixel; it shows only as a coloured fringe.

## Checks

- `verification/D-371_text_shaping_table.md`: **16 of 16**, and FX-SHAPE-020 ("office fly fit
  staff"), which was "in dispute" because its "y" was 0.4 pixel from HarfBuzz's reference, now
  passes outright.
- `verification/D-350_text_animator_table.md`: 61 of 61, against D-350's expected boxes re-recorded
  with the slide (your choice is what allowed changing them; 16 of 162 boxes moved, at most 0.6
  pixel).
- `verification/D-264_text_styles_table.md`: 57 of 57; two pinned D-263 pictures ("Text あ / Cut
  012" and "AV To / WAVE") are re-pinned because their x, A, V and W moved.
- Timing: `verification/B-251_side_bearing_timing_table.md`, no change that can be measured
  (0.249 ms before, 0.254 after, laying out a 30-letter line; quiet machine).

## What to check

1. Make a text layer in the bundled font, size 200, and type `AVWY`. It should look exactly as
   before to the eye. Zoom the viewer to 400%: the A and V sit a hair further right than in
   yesterday's build, as in the bottom of `before_after.png`.
2. If you have a copy of the same font open in another program (a word processor, Photoshop),
   the letters now line up with it to the pixel.
