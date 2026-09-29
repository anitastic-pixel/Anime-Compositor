# B-124a: motion blur, proposed (D-188, ADR-019)

**You accepted this on 2026-09-28:** "I approve of motion blur plan". The three answers written under
"Questions for you" stand as written: a held jump shows twice at half strength, expression
motion holds, and Draft uses the same samples as Full. The build is B-124b.

This is item 10 of the list. You accepted items 1 to 9 on 2026-09-28. Item 10 was to be written
up to a decision and **stop before any code**, because the time rules (document 20) say motion
blur needs an ADR and fixtures of its own first. So this is a proposal. **Nothing is built until
you accept it.**

## What it is

A fast-moving layer is drawn several times inside one frame, at moments spread across the time
a film camera's shutter would be open, and the drawings are averaged. The result is the smear
you see on a fast pan or a thrown object. It works the way After Effects' motion blur does:

- The **composition** has a switch and three settings:
  - **Shutter Angle**, 0 to 720 degrees, 180 when added: how long the shutter is open. 180 is
    half a frame, 360 a whole frame.
  - **Shutter Phase**, -360 to 360 degrees, -90 when added: when the shutter opens, relative to
    the frame. -90 centres the smear on where the layer is on the frame.
  - **Samples**, 2 to 64, 16 when added: how many drawings are averaged.
- **Each layer** has its own switch, off when added.

A layer is blurred only when both switches are on.

## What you will see

`motion_blur.png`, in this folder, is a square moving right and turning, drawn by the proposed
rule:

1. with motion blur off;
2. as added: 180, -90, 16;
3. with a whole frame of shutter;
4. with a quarter frame;
5. with phase 0, which puts the smear ahead of the square;
6. with only 4 samples, where the steps show;
7. with a held key that jumps on this frame, where the square is seen twice at half strength.

In the program:

- The composition settings get a **Motion Blur** switch with the three settings under it.
- The timeline gets a motion-blur switch on each layer, next to the others.

## What moves and what holds

**Drawings hold.** Only where a layer is, moves:

- Read at each moment inside the shutter:
  - its position, anchor, scale and rotation;
  - its parents';
  - its depth;
  - the camera.
- Read once, at the frame:
  - the drawing the exposure sheet shows;
  - masks and effects;
  - shape outlines;
  - what a composition layer shows;
  - opacity;
  - in and out points;
  - the drawing order.

So a cel on twos is never smeared into the next drawing. The blur is only the camera's and the
layer's travel.

A layer that does not move in the frame is drawn once. It is the same picture, bit for bit, as
with its switch off.

## How it differs

- **Directional Blur and Radial Blur** blur a layer the same way whatever it does. Motion blur
  follows the actual path, and it is zero when nothing moves.
- **Speed Lines** draws lines. Motion blur draws the layer itself.

## Known limits, on purpose

- **Cost.** Each blurred layer that moves costs about as many times its drawing as there are
  samples. A heavy shot should use fewer samples, or Draft.
- **Effects are not smeared by time.** Effects are run once and the result is moved, so an
  effect that changes over the frame (a wiggle, a twinkle) is not smeared by its own change.
- **Expressions.** A property driven by an expression is read at the frame and holds across the
  shutter. D-59 says an expression never reads between frames.
- **The graphics card.** At first the card does not draw it. A frame with motion blur is drawn
  on the processor in the viewer, which says so (D-101). The card is a later unit.
- **Not part of this proposal:**
  - motion blur from a camera that turns;
  - depth of field;
  - frame blending (skipped).

## Questions for you

1. **Held keys.** A held key that jumps inside the shutter shows the layer twice at half
   strength, as After Effects does (strip 7). Keep that, or should a held jump stay sharp?
2. **Expressions.** Expression motion is held, not blurred. Is that acceptable for now?
3. **Draft.** Draft uses the same number of samples as Full. Should Draft use fewer, for speed?

## How you will check it

The build will draw every case in `Fixtures/motion_blur/` and compare it with the numbers in
`expected_motion_blur.json`, which `tools/motion_blur_reference.py` works out a second way.
Document 25 lists the cases in tables you can read:

- a still layer stays sharp;
- a bar moving 16 pixels a frame becomes the exact staircase of shares its four positions give;
- a camera pan, or a moving parent, gives the same staircase;
- the drawing and the opacity hold;
- a matte is blurred by its own switch;
- files that break the rules are refused.

A playtest sheet will come with the build.
