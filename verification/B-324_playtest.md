# B-324: Vegas

Built on 2026-10-10 as D-444, under your /loop request: **Vegas**, in **Generate**. It runs
dashes along the layer's masks (or a shape layer's own paths): each dash is whole at its start and
fades towards its end, and animating **Rotation** makes them chase round the path, like the lights
round a theatre sign.

By your decision of 2026-10-10, "Masks only for now", After Effects' other stroke, **Image
Contours** (dashes round the edges of whatever is in the picture), is not built. A project that
asks for it keeps the setting, draws the layer without the dashes, and says why in a sentence.
Logged as a gap; the row in EFFECTS.md is marked partial.

- **Stroke**: **Mask/Path** (the layer's masks) or **Shape Paths** (a shape layer's paths).
- **Mask** (1) picks which mask; **All Masks** on uses every mask.
- **Segments** (32): how many dashes on each path. **Length** (1): how much of each dash's share
  of the path it fills; 1 is end to end, 0.5 half dash, half gap.
- **Segment Distribution**: **Bunched** (dashes in a train from where Rotation starts them) or
  **Even** (spread evenly round the path).
- **Rotation**: turns the dashes round the path; one full turn brings them back where they began.
- **Random Phase** with **Random Seed**: each path starts its dashes at its own random place.
- **Blend Mode**: **Over** (on top of the layer), **Under** (behind it), **Transparent** (the
  dashes alone) or **Stencil** (the layer only shows inside the dashes).
- **Color**, **Width** (2 pixels) and **Hardness** (0 soft, 1 a hard edge).
- **Start Opacity** (1), **Mid-point Opacity** (0), **Mid-point Position** (0.5) and **End
  Opacity** (0): how each dash fades along its length.

The controls are After Effects' own; how they draw is our own rule (Adobe publishes none), not
matched to After Effects pixel for pixel. Masks are always closed in this program.

The check, `verification/D-444_vegas_table.md` (314 of 314), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-444 pictures/`: the street
(`town.png`) with a circle mask.

## What to check

1. **The chase.** `chase_strip.png` (frames 0, 6, 12, 18 and 24 side by side, each also alone as
   `chase_f00.png` and so on): five orange dashes spread round the circle, Rotation keyed from 0
   to 360 over 24 frames. They move round the circle and frame 24 matches frame 0. Nothing
   changes away from the circle.
2. **Tails.** `tails.png`: four blue dashes end to end round the circle, each bright at its start
   and fading to nothing at its end, like comet tails.
3. **Stencil.** `stencil.png`: six wide dashes; the street shows only inside them, the rest
   empty.
4. **In the app.** Draw a mask on a picture, add Vegas (Generate), key Rotation from 0 to 360 and
   play: the dashes run round the mask. Try Even, a wider Width, a colour, and the four Blend
   Modes.
5. **No mask.** Add Vegas to a layer with no mask: nothing is drawn and a sentence says there is
   no mask to draw along.
6. **Image Contours.** Not in the Stroke list. A project that asks for it draws the layer without
   the dashes and says "Vegas's Image Contours stroke is not built yet".
7. **Out of range.** Type 1001 in Segments: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-324_vegas_timing_table.md` (PROVISIONAL): the reference shot with a moving
Noise and a Vegas on three layers, played again, 24.9 ms a frame on the card as added
against 13.0 for the Noise alone; on the processor 48.2 against 45.9:
the card is not quicker than the processor at every setting.
