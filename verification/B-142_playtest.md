# B-142: Beam, by hand

Built on 2026-09-29 against D-207, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Beam does, by a rule of our own: it draws a
straight beam of light along a stretch of the line from a start point to an end point, white in
the middle and blue at the edges as it starts, like a laser or an energy shot. Length says how much
of the line is lit, and Time moves that stretch from the start to the end, so keying Time fires the
shot across. It does not grow the layer. It sits in the **Generate** group after Radio Waves.

The generated halves are `verification/B-142_beam_table.md`, 137 of 137 checks passing, which
renders every FX-BEAM case against the numbers written before the code and draws the pictures
below, and `verification/B-12b_state_fields_table.md`, which checks the card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks in the window.

## The pictures

`verification/B-142 pictures/`, three times enlarged:

- `drawing.png`: the plate at its own size, a night sky over dark rooftops; `before.png` is the
  same enlarged.
- `as_it_starts.png`: Beam as it starts: a short white shot with a blue edge at the left of a line
  across the middle.
- `time_50.png` and `time_100.png`: Time 50 and 100: the same shot in the middle, then at the end.
- `length_100.png`: Length 100: the whole line lit.
- `thickness_2_to_20.png`: from 2 pixels thick at the start to 20 at the end: a cone.
- `softness_0.png` and `softness_100.png`: a hard edge, then a soft one reaching twice as far.
- `yellow_in_red.png`: Inside Color yellow and Outside Color red, slanted from low left to high
  right, 12 thick.
- `composite_off.png`: Composite On Original off: the beam alone, the night gone.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-142 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Beam** in **Add effect…**, under **Generate**, after
   Radio Waves; typing "laser", "ray", "shot" or "beam" in the search finds it too. The card shows
   Start Point 10, 50, End Point 90, 50, Length 25, Time 0, Starting and Ending Thickness 8,
   Softness 50, a white Inside Color, a blue Outside Color and Composite On Original on, and the
   picture is as `as_it_starts.png` at once.
2. **Time.** Drag Time to 50, then 100: as `time_50.png`, then `time_100.png`. The shot slides
   smoothly along the line.
3. **Firing it.** Key Time from 0 at the first frame to 100 at frame 24 and play: the shot flies
   from left to right.
4. **Length.** Length 100: as `length_100.png`. Length 0: a round dot the beam's thickness.
5. **Thickness.** Starting Thickness 2 and Ending Thickness 20 with Length 100: as
   `thickness_2_to_20.png`.
6. **Softness.** 0, then 100: as `softness_0.png`, then `softness_100.png`.
7. **Points and colours.** Start Point 10, 90, End Point 90, 10, thickness 12, Inside Color
   #ffe080 and Outside Color #ff3020, Length 100: as `yellow_in_red.png`.
8. **Composite.** Turn Composite On Original off: as `composite_off.png`, only the beam left.
9. **Moved.** Move the layer: the beam moves with the drawing.
10. **Out of range.** Type 101 in Length, 501 in Starting Thickness or 1001 in Start Point: it is
    refused with a sentence saying what it runs to, and the card keeps its old number.
11. **Draft.** Press **Draft**: the picture is smaller with the same beam.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects' Beam has a 3D Perspective switch, which thickens the near end as if the beam
  came towards you; ours is flat, the thickness changing evenly along the whole line.
- It does not glow past its softness; put Glow after it for a halo.
- A Length of 0 leaves a round dot, not nothing; the ends are always round.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Beam and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 48 to 52 ms without it; Beam as it starts, 58 to 64 ms; the whole line 200 to 500
  pixels thick and fully soft, 59 to 63 ms; composite off, 57 to 63 ms. **Machine:** AMD Ryzen
  9 9900X with 24 threads; Windows 11; release build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
