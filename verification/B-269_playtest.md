# B-269: Ripple Pulse

Built on 2026-10-09 under your effects loop request, decided as D-390. **Ripple Pulse** (in
**Distort**) is our version of CycoreFX's CC Ripple Pulse: rings that run out from a point, like
a pebble dropped in a pool. Each change in **Pulse Level** sends a ring out, so you key Pulse
Level (or link it to a sound's loudness) and every beat makes the picture throb. The formulas
are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Center** (50, 50): per cent of the layer's width and height, where the rings start.
- **Pulse Level** (0): -1000 to 1000. Nothing happens while it stays still; a rise pushes the
  picture outward in a ring, a fall draws it in.
- **Time Span (sec)** (1): how long a ring takes to reach the corners; 0 makes no rings.
- **Amplitude** (10): how far a ring pushes the picture.
- **Render Bump Map** (Off): On draws the rings as grey heights instead of the picture, for
  Glass or Displacement Map.

The layer does not grow.

The check, `verification/D-390_ripple_pulse_table.md` (143 of 143), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-390 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: Pulse Level has never moved, so the street is unchanged.
2. **One ring.** `3_rings.png`, Pulse Level jumped from 0 to 10 five frames earlier, Time Span
   0.5, Amplitude 10: one ring part way out from the middle, bending the buildings and the road
   markings outward where it passes.
3. **As a bump map.** `4_bump_map.png`: the same ring as a soft grey circle on grey.
4. **In the app.** Add Ripple Pulse and key Pulse Level as steps (0, then 10 a few frames later,
   then 20...). Play: each step sends a ring out that reaches the corners after Time Span
   seconds. Key it down instead and the rings draw the picture in.
5. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Ripple Pulse costs about 2.0 ms a 1080p layer with a moving level and 1.5 ms as a bump map, and about 9.5 and 7.1 ms on the processor
(`verification/B-267_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The rings' shape and the amplitude's scale are our own reading of CycoreFX's one-sentence
  descriptions; no tutorial with numbers was matched against it.
- The level is read once a frame, so a change keyed between frames moves in whole-frame steps.
- There is no built-in link to sound yet; key Pulse Level, or drive it with an expression.
