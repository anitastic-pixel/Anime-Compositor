# B-268: Power Pin

Built on 2026-10-09 under your effects loop request, decided as D-389. **Power Pin** (in
**Distort**) is our version of CycoreFX's CC Power Pin: Corner Pin's four pins, with
Perspective that can be eased off, the picture expanded past the pins, and Unstretch to run the
pinning backwards. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Top Left, Top Right, Bottom Left, Bottom Right** (each at its own corner): per cent of the
  layer's width and height, where each corner of the picture goes.
- **Perspective** (100): 100 leans the picture back in true perspective, as Corner Pin does; 0
  squeezes it evenly instead; between, a mix.
- **Unstretch** (Off): On runs it backwards: the shape the four pins mark is stretched out to
  fill the layer, to flatten a screen or sign filmed at an angle.
- **Expansion Top, Left, Right, Bottom** (0): -40 to 100, how much of the picture shows past
  each side of the pins, as a share of the pinned shape.

The layer grows to hold pins pulled past its edges, unless Unstretch is on.

The check, `verification/D-389_power_pin_table.md` (171 of 171), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-389 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: every pin at its own corner, so the street is unchanged.
2. **A keystone.** `3_keystone.png`, the top pins pulled in to a quarter and three quarters: the
   street leans back, its top narrower and the upper buildings drawn smaller, the top corners
   white.
3. **Perspective 0.** `4_keystone_flat.png`: the same shape, but squeezed evenly, the buildings
   their full height.
4. **Expanded.** `5_expanded.png`, pinned small in the middle with Expansion Left and Right 50:
   a full-width band across the middle, white above and below.
5. **Unstretched.** `6_unstretched.png`, the keystone with Unstretch on: the other way round, the
   street's top stretched wider than the layer, so the buildings lean outward.
6. **In the app.** Add Power Pin, drag Top Right down to (100, 50) in Effect Controls, then
   slide Perspective from 100 to 0: the picture changes from leaning back to squeezed flat.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Power Pin costs about 1.5 ms a 1080p layer, and about 6.6 ms on the processor
(`verification/B-267_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The mix between perspective and even squeeze, and how the expansions are read, are our own
  reading of CycoreFX's one-sentence descriptions; no tutorial with numbers was matched.
- There are no pin handles on the viewer yet (Corner Pin has them); set the pins in Effect
  Controls.
- Pins crossed into a bow tie, or a shape expanded past its vanishing line, give a clear frame.
