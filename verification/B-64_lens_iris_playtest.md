# B-64: Lens Blur's iris and highlights, by hand

Built on 2026-09-26 against D-121, which is proposed and awaits your acceptance, at your request
to "expand lens blur capability ... like AE CC lens blur does with it's camera blur types".

The generated halves are `verification/B-64_lens_iris_table.md`, 101 of 101, which renders
FX-LENS-019 to 044 against the numbers written before the code (worst difference 1.9e-7 against
a tolerance of 2e-5); `verification/B-59_lens_blur_table.md`, which shows the old round blur,
FX-LENS-001 to 018, did not move; and `verification/B-12b_state_fields_table.md`, which checks
the card sends every setting the command reads. This sheet covers what the tables cannot: how it
looks and feels in the window. The pictures in `verification/B-64a proposal/` show what to
expect, and `verification/B-64b build/lens_blur_cards.png` shows the two cards.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer and press **Full resolution**. A drawing with small bright dots, sparkles or a white
highlight shows the effect best. A Lens Blur you added before this build opens as it was: a
circle, and the new numbers at their starts.

## What to check

1. **The card.** Add **Lens Blur**. Above its numbers is a picture of the iris, the shape one
   bright dot spreads into: a circle. Under **Radius** are **Roundness** 0, **Rotation** 0° with
   a dial, **Aspect** 1, **Brightness** 0 and **Threshold** 100, then **Iris Shape** Circle and
   **Edges**. Hover a label for its full name and range.
2. **Iris shapes.** Radius 8. Go through **Iris Shape** from Triangle to Decagon. The picture
   changes to each shape, and each small dot in the drawing opens into that shape, as in
   `iris_shapes.png`. The triangle stands point up; the hexagon is flat along its top and bottom.
3. **Rotation.** Drag round the dial: the picture and the dots' shapes turn with it, clockwise
   from up; Shift snaps to 15 degrees. Rotation 180 turns the triangle point down.
4. **Roundness.** On Square, Roundness 50 bows the sides out; 100 is the circle again.
5. **Aspect.** Aspect 2 makes the shape twice as wide as tall; 0.5 twice as tall as wide. A
   stretched blur still spreads past the drawing's box and is not cut off.
6. **Highlights.** Brightness 3, Threshold 80: the brightest parts (near white) spread as bright
   shaped blobs instead of fading into the blur, as in `iris_settings.png`. Lowering Threshold
   lets darker colours join in; Brightness 0 turns it off.
7. **Out of range.** Type 101 in Roundness, 3601 in Rotation, 0.05 in Aspect or -1 in
   Brightness: each is refused with a sentence and the card keeps its old number.
8. **Keyed.** Key Rotation at 0 and at 60 at a later frame and scrub: the shapes turn smoothly.
   Key Brightness from 0 to 3: the highlights swell.
9. **Draft.** Press **Draft**: the picture is smaller, with the same shapes.
10. **Undo and saved.** Ctrl+Z steps back each change, the iris choice included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- The whole layer is blurred alike; there is no depth map, which After Effects' Camera Lens Blur
  takes from another layer.
- There is no diffraction fringe, blade curvature or noise, as After Effects has.
- The highlight test is on each pixel's own colour; there is no gamma or range setting.
- It runs on the processor only, so large radii on full-size frames are slow, as B-59 was.

## What to answer

"works", or which step number did something else and what it did. Answering also decides D-121:
"works" accepts it.
