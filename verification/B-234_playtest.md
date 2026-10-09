# B-234: Radio Waves drawn by the graphics card

Built on 2026-10-08 from your decision on D-345 ("go with your recommendations on all three"):
option 2. Radio Waves was drawn by the processor until now, so a layer with it was drawn by the
processor from that effect on. Now the graphics card draws it in the viewer, by the processor's
own rule. The waves (when each is born, how far it has grown, how strong it is) are still worked
out by the processor and handed to the card, so they keep the same timing. Exports never use the
graphics card and are untouched.

What changed in the check: a pixel that is fully see-through (0 of 255) on both the processor's
picture and the card's now counts as equal, whatever colour it would have if it were visible,
since that colour is never seen. Every other pixel is still held to 1 level of 255. Only the card
check for these six generators uses that rule (`tests/b225_gpu_generators.rs`, `distance`).
Without it, exactly two frames would fail: FX-RWAVE-010 at frame 0, at Full (3 pixels) and Draft
(1 pixel), each fully see-through on both sides.

The check, `verification/B-225_gpu_generators_table.md`, draws every Radio Waves fixture and the
reference shot with three Radio Waves settings, on both. The timings are in
`verification/B-234_radio_waves_timing_table.md`.

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**.

1. **Radio Waves.** On one layer add Radio Waves. Play the shot: rings should spread from the
   **Producer Point** at a steady pace, with no flicker or jumps.
2. **Shape.** Change **Sides** to 3, 6 and 64: the rings should become triangles,
   hexagons and near circles, with clean edges.
3. **Profile.** Try each **Profile** (Square, Triangle, Sine): a square ring has a crisp edge, a
   triangle or sine ring a soft one.
4. **Movement.** Raise **Velocity** and **Spin**, and change **Direction**: the rings should drift
   and turn smoothly as they grow.
5. **Width and fade.** Set **Start Width** and **End Width** apart, and raise **Fade-out Time**:
   each ring should thin or thicken and fade as it ages.
6. **Same as an export.** Export a frame (or render it to a still) and put it next to the viewer.
   The two should look the same.
7. **Smooth.** Play the shot. It should play noticeably more smoothly than before.
8. **Draft.** Switch to **Draft** and repeat steps 1 and 2.

## If something is wrong

Say which step. The most likely fault would be a ring whose edge looks different between the
viewer and an export; the check compares exactly that, at every frame of every fixture.
