# B-288: Lens Chromatic Aberration

Built on 2026-10-10 as D-409, under your /loop request: your plugin pick #3, Lens Chromatic
Aberration, with its RGB Separation row as a mode. It is not a new effect: **Chromatic
Aberration** (in **Distort**) gains new settings. A Chromatic Aberration in a project saved before
this keeps its two settings (Amount and Centre) and draws exactly as it did; one added now starts
with the new settings, at the same picture as before until you change them.

The settings, with the values they start at:

- **Mode** (Radial): **Radial (lens)** splits the colours outward from a centre, as a lens does;
  **Offset (RGB Separation)** moves them the same distance everywhere, in a straight line.
- **Amount** (3): pixels a colour at scale 100 moves, at the corner (Radial) or everywhere
  (Offset).
- **Centre** (50, 50): Radial only, the point nothing moves from.
- **Angle** (90): Offset only, the direction red moves, clockwise from up (90 is right).
- **Falloff** (0): Radial only. At 0 the split grows evenly from the centre; at 100 the middle of
  the picture barely splits and the edges split fully, as a real lens does.
- **Red Scale** (100), **Green Scale** (0), **Blue Scale** (-100): how far each colour moves, in
  per cent of the amount; a negative scale moves it inward or the other way.
- **Fringe Blur** (0): smears each colour along its own move, softening the coloured fringes.

All but Mode are keyable.

The check, `verification/D-409_lens_chromatic_aberration_table.md` (193 of 193), holds every pixel
to numbers worked out by a separate program before the code existed, checks that every older
Chromatic Aberration test file still opens and draws exactly as before, and holds the graphics
card's picture to within 1 level of the processor's. The pictures are in
`verification/D-409 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Falloff.** `2_no_falloff.png` and `3_lens_falloff_100.png`, Amount 12: without falloff the
   houses in the middle already show red and blue edges; with Falloff 100 the middle is almost
   clean and the edges of the picture still split.
2. **RGB Separation.** `4_offset_rgb_separation.png`, Offset, Amount 6, Angle 90: a red copy six
   pixels to the right and a blue one six to the left, the same across the whole picture.
3. **Fringe blur.** `5_fringe_blur_100.png`, picture 3 with Fringe Blur 100: the coloured fringes
   smeared outward into soft streaks instead of hard copies.
4. **One colour.** `6_green_only.png`, Red 0, Green 100, Blue 0: only green moves outward; magenta
   and green fringes at the edges.
5. **In the app.** Add Chromatic Aberration to a layer: the card shows Mode, Amount, Centre, Angle,
   Falloff, Red Scale, Green Scale, Blue Scale and Fringe Blur, and the picture is the old one.
   Switch Mode to Offset and turn Angle: the split turns round.
6. **An older project.** Open a project saved before today with a Chromatic Aberration in it: its
   card shows only Amount and Centre, and the picture is the same as before.
7. **Out of range.** Type 101 in Falloff: it is refused with a sentence saying it runs from 0 to
   100, and the card keeps its old number.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

**Timing.** In `verification/B-288_lens_chromatic_aberration_timing_table.md`, the reference shot with a moving Noise on three layers, played again
(PROVISIONAL: another lane's test was running at the end of the second round): 15.7 ms a frame on
the card without the effect, 16.8 radial with falloff and 15.7 offset (under 2 ms a layer), 41.4
with Fringe Blur 100 at Amount 10 (about 9 ms a layer: fringe blur at a large amount is the slow
part). On the processor 42.3, 84.5, 69.0 and 204.8.

## Not built

- PLUGINS.md describes the plugin's look, not its numbers; the falloff's curve (the distance to
  the power 1 + 2 times falloff / 100) and the fringe blur's even samples along each colour's move
  are ours.
