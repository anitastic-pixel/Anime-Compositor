# B-423: Soft Physical Glow's Spectral

Built on 2026-10-11 as D-543, your pick #10 (Spectral Glow). It is not a new effect: Soft
Physical Glow (Glow with Falloff set to Physical) has four new settings, after the Glow
Aberration of the glow plugin `docs/effects/PLUGINS.md` reads. The arithmetic is our own.

- **Spectral** (Off): Off is the glow as before. On makes red, green and blue each glow at their
  own size, so where the three sizes part the glow's edge fringes in colour, as light through a
  lens does.
- **Spectral Red**, **Spectral Green**, **Spectral Blue** (80, 100, 120): each colour's size, as
  a share of Radius, 0 to 200 per cent. As added, red stays closer and blue reaches further. They
  can be keyed.

A white light still glows white at its core, where the three sizes agree.

Nothing of the plugin's Glow Aberration is left out. Spectral on does three times the work of a
plain glow, so it is slower (see Speed).

The check, `verification/D-543_spectral_glow_table.md` (327 of 327), holds every pixel to
numbers worked out by a separate program before the code existed, runs all 45 of Soft Physical
Glow's earlier checks and D-542's 22 tint checks again unchanged (a glow with Spectral off draws
exactly as before), and holds the graphics card's picture to within 1 level of the processor's.
The pictures are in `verification/D-543 pictures/`.

## What to check

1. **Off.** `spectral_off.png`: the town with Soft Physical Glow (threshold 60, smooth 50, radius
   60), Spectral off, for comparison.
2. **On.** `spectral_on.png`: the same with Spectral on as added: the far tail a little bluer, the
   near glow a little warmer.
3. **Strong.** `spectral_strong.png`: red 50, blue 200: a strong fringe, blue far out, red held
   close.
4. **Reversed.** `spectral_reversed.png`: red 200, blue 50: the other way round, the tail redder.
5. **In the app.** Add Glow, set Falloff to Physical, then set Spectral to On. Change Spectral
   Blue to 200 and Spectral Red to 50 and watch the edge fringe. Key Spectral Blue from 100 to 200
   and play it: the blue fringe should spread out smoothly.
6. **Out of range.** Type 201 in Spectral Red: it is refused with a sentence saying the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.
   An older project with Soft Physical Glow opens and looks as it did.

Our own choices you may want to judge: the sizes as added (80, 100, 120, the plugin's), the
range up to 200 per cent, and that Exposure and the other settings act once on all three colours
together.

## Speed

Measured, but PROVISIONAL: other builds and graphics-card tests were running on this machine at the time, so the numbers are much higher and noisier than a quiet machine gives (the noise alone took 51.2 ms a frame, against 15.7 at B-422's run). On the graphics card, the reference shot with colour noise and then the glow on its three layers (radius 60, playing it again) took 147.0 ms a frame with Spectral on and 141.4 with it off; the glow as added (radius 500) with Spectral on took 802.0 ms. The full table is `verification/B-423_spectral_glow_timing_table.md`. A quiet re-measure is owed. What to check by eye: whether scrubbing a wide glow with Spectral on still feels smooth enough to you.
