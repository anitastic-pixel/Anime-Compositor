# B-105: Camera Shake, by hand

Built on 2026-09-26 against D-162, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-ninth of the thirty.

The generated halves are `verification/B-105_camera_shake_table.md`, which renders every
FX-SHAKE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Camera Shake card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-105a proposal/` shows what to expect.

## Before you start

Open a project with a background and, above it, a character drawing that does not fill the
frame. Select the drawing and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Camera Shake** in **Add effect…** on the drawing. It goes to the end of
   the stack, and the card shows **Amount** 10, **Rotation** 0, **Hold** 1 and **Seed** 0. Step
   through the frames: each frame the drawing sits in a slightly different place, up to 10
   pixels each way, never cut off at its edges.
2. **Play.** Play: the drawing jitters as if the camera were bumped. The background below stays
   still, because the shake is on the drawing's layer only.
3. **On twos.** Hold 2: each jolt lasts two frames, a coarser anime shake. Hold 100: it jumps once
   and stays.
4. **Seed.** Seed 5: a different shake. Seed 5.7 gives the same as 5.
5. **Rotation.** Amount 0, Rotation 10: the drawing no longer moves but tips a few degrees one way
   or the other each frame, about its own middle, corners not cut off.
6. **Amount 0 and Rotation 0.** The drawing sits exactly where it was, as if the effect were off.
7. **Out of range.** Type 50 in Rotation: it is refused with a sentence saying it runs from 0 to
   45, and the card keeps its old number.
8. **Keyed.** Key Amount at 0 and at 30 a quarter of a second later, then back to 0 a second
   after that. Play: a still drawing, a hard hit, and the shake dying away, the impact look.
9. **Draft preview.** Switch to a half-size draft: the jolts move the drawing the same distance
   on screen as at full size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there, and the same frame shows the drawing in the same place.

## Known limits, on purpose

- **Jumps, no sway.** Each jolt jumps straight to its place with no easing or motion blur
  between jolts, so a slow drifting hand-held sway is out of reach.
- **The layer only.** It shakes the layer it is on, not the whole composition. To shake
  everything, put it on each layer with the same seed, amount and hold.
- **Turns about the drawing's middle only**, with no setting for another pivot.
- **Large on small drawings.** The starting amount of 10 pixels can carry a small drawing out
  of the frame on some frames; on a full-frame drawing it is the right size.
- It is modelled on After Effects' wiggle-driven camera shakes in spirit, not on any one After
  Effects effect.

## What to answer

"works", or which step number did something else and what it did.
