# B-321: Write-on

Built on 2026-10-10 as D-441, under your /loop request: After Effects' Write-on, in
**Generate**. It is a brush that leaves paint wherever it has been: animate **Brush Position**
and, as the shot plays, a line is written on along the path it took.

- **Brush Position** (the middle): where the brush is, as a per cent of the layer's width and
  height. Key it to move the brush.
- **Color** (white), **Brush Size** (6 pixels across), **Brush Hardness** (75: how sharp the edge
  is) and **Brush Opacity** (100).
- **Stroke Length (secs)** (0): how long each bit of paint stays. 0 keeps it for ever; 0.5 leaves
  only the last half second of the line, a tail chasing the brush.
- **Brush Spacing (secs)** (0.01): how often a dab of paint is left. Larger values leave a row of
  separate dots.
- **Brush Time Properties** (None): None makes the whole line take today's size and hardness;
  Size (or Hardness, or both) makes each dab keep the size it had when it was laid, so a growing
  brush leaves a line that swells.
- **Paint Time Properties** (None): the same for Opacity. Color is offered, as in After Effects,
  but colours cannot be animated in this program yet, so it does nothing for now.
- **Paint Style** (On Original Image): *On Original Image* paints over the layer; *On
  Transparent* shows the line alone; *Reveal Original Image* shows the layer only where the line
  is.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. The brush is the same one as Path
Stroke's. A draft draws at half size and halves the brush.

The check, `verification/D-441_write_on_table.md` (260 of 260), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-441 pictures/`
(`1_before.png` is the street with nothing on it). In pictures 3 to 6 the brush is keyed through
a V over two seconds: from the upper left, down to the middle at one second, up to the upper right.

## What to check

1. **As added.** `2_as_added.png`: one small white dot in the middle of the street; nothing else
   changed.
2. **Halfway.** `3_vee_half.png` (one second in): a thick white line from the upper left down to
   the bottom of the V in the middle; the right-hand arm not there yet.
3. **The whole V.** `4_vee_whole.png` (the end): both arms of the V in white, smooth and even.
4. **A tail on transparent.** `5_vee_transparent_tail.png`: the street gone; only a short red
   stretch of the right-hand arm, the last half second of the brush's path.
5. **Revealing, swelling.** `6_vee_reveal_swelling.png`: everything gone except the street seen
   through the V; the line thin at the upper left and growing steadily thicker to a fat round end
   at the upper right.
6. **In the app.** Add Write-on (Generate) to a layer: a white dot in the middle. Key Brush
   Position at two or three places a second apart and play: a line is written on behind the
   brush. Set Stroke Length to 0.5: only a tail follows the brush. Raise Brush Spacing to 0.2: a
   row of dots. Key Brush Size from small to large and set Brush Time Properties to Size: the line
   swells; set it back to None: the whole line takes the size now. Try On Transparent and Reveal
   Original Image. Switch to Draft: the same picture.
7. **Out of range.** Type 201 in Brush Size: it is refused with a sentence saying it runs from 0
   to 200.
8. **Saved and opened again.** Save, close and open the project: the same settings, keys and
   picture.

## Speed

Measured only provisionally: other lanes' builds and tests were running at the time
(`verification/B-321_write_on_timing_table.md`). Three Write-ons on a 1920 by 1080 shot, card / processor, milliseconds a
frame played again: Noise alone 36.4 / 78.6 ms a frame, as added 48.6 / 86.8, keyed across, size 40 42.8 / 99.8, size kept per mark, stroke length 2 40.4 / 99.8; to be measured again.
