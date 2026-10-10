# B-289: Line Boil

Built on 2026-10-10 as D-410, under your /loop request: your plugin pick #4, Line Boil, the
hand-drawn wobble where lines jump to a new shape every few frames, as if each drawing were redrawn.
As PLUGINS.md suggested, it is not a new effect: **Turbulent Displace** (in **Distort**) gains
one setting.

- **New Seed Every** (0): frames, 0 to 100, keyable. At 0 the warp keeps one pattern, as before.
  At 2 the pattern holds for two frames and then jumps to a new one (on twos); 3 is on threes; 1
  jumps every frame. Only the whole number counts (2.9 acts as 2).

A Turbulent Displace in a project saved before this has no New Seed Every and draws exactly as it
did. For the usual boil, use a small Amount (a few pixels), Speed 0 so the warp stands still
between jumps, and New Seed Every 2 or 3.

The check, `verification/D-410_line_boil_table.md` (163 of 163), holds every pixel to numbers
worked out by a separate program before the code existed, checks that all 38 older Turbulent
Displace test files open, save and draw exactly as before, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-410 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **The boil.** `2_boil_frame_0.png` to `6_boil_frame_4.png` are frames 0 to 4 with Amount 6,
   Size 30, Speed 0, New Seed Every 2. Frames 0 and 1 are the same picture, frames 2 and 3 the
   same as each other but a different wobble, and frame 4 another. Flick through them: the edges
   of the houses should jump every second frame and hold still between.
2. **One seed.** `7_one_seed_frame_4.png` is frame 4 with New Seed Every 0: the same as frame 0,
   never jumping.
3. **In the app.** Add Turbulent Displace to a layer and set New Seed Every to 2 and Speed to 0;
   play: the warp jumps every second frame. Set it to 0: it stops jumping.
4. **An older project.** Open a project saved before today with a Turbulent Displace in it: New
   Seed Every shows 0 and the picture is the same as before.
5. **Out of range.** Type 101 in New Seed Every: it is refused with a sentence saying it runs
   from 0 to 100, and the card keeps its old number.
6. **Saved and opened again.** Save, close and open the project: the same setting and picture.

## Speed

In `verification/B-289_line_boil_timing_table.md` (quiet machine): the reference shot with a moving Noise on three layers, played again,
16.3 ms a frame on the card with a new seed every 2 frames against 16.1 with one seed, so the
new seed costs nothing measurable; on the processor 130.8 against 132.8.
