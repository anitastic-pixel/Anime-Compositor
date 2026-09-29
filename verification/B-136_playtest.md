# B-136: Polar Coordinates, by hand

Built on 2026-09-29 against D-201, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Polar Coordinates does, by a rule of our own:
Rect to Polar bends the drawing round its middle, its rows into rings and its columns into
spokes, so upright speed lines become focus lines closing in on the middle; Polar to Rect
unrolls it, rings into rows and spokes into columns. Interpolation says how far to go. The layer
never grows. It sits in the **Distort** group after Corner Pin, and its settings carry After
Effects' own names.

The generated halves are `verification/B-136_polar_coordinates_table.md`, 72 of 72 checks
passing, which renders every FX-POLAR case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Polar
Coordinates card sends every setting the command reads. This sheet covers what the tables cannot:
how it looks and feels in the window.

## The pictures

`verification/B-136 pictures/`, three times enlarged, over a grey check where nothing is drawn:

- `before_lines.png`: speed lines, dark streaks rising from the foot, none in the top third.
- `focus_lines.png`: Rect to Polar at 100: the streaks turned into spokes closing in on the
  middle from an oval ring; the middle and the corners empty.
- `half_way.png`: Rect to Polar at 50: the streaks bending in, half way there.
- `back_again.png`: Rect to Polar, then a second Polar Coordinates, Polar to Rect: the speed
  lines back, a little softer.
- `before_target.png`: a target, five oval bands, yellow in the middle, dark outside.
- `target_unrolled.png`: Polar to Rect at 100: the target unrolled into five stripes, yellow at
  the top, dark at the foot.

## Before you start

Make the composition 160 by 100 and import `speed_lines.png` and `target.png` from
`verification/B-136 pictures/`. Press **Full resolution**.

## What to check

1. **Adding it.** On the speed lines, pick **Polar Coordinates** in **Add effect…**, under
   **Distort**, after Corner Pin; typing "polar", "focus lines", "tiny planet" or "unroll" in the
   search finds it too. The card shows **Interpolation** 100 and **Type of Conversion** Rect to
   Polar, and the picture is as `focus_lines.png` at once.
2. **Half way.** Set Interpolation to 50: as `half_way.png`, the streaks bending in. Drag it
   slowly down to 0: the picture straightens until, at 0, it is the speed lines untouched.
3. **Keyed.** Key Interpolation from 0 at the first frame to 100 at a later one and play: the
   speed lines curl into focus lines.
4. **Unrolled.** On the target, add Polar Coordinates and set Type of Conversion to Polar to
   Rect: as `target_unrolled.png`, five stripes, yellow at the top.
5. **Back again.** On the speed lines, with Rect to Polar at 100, add a second Polar Coordinates
   below it set to Polar to Rect: as `back_again.png`, the speed lines back.
6. **Moved.** Move the layer to the right: the focus lines move with it; nothing is left behind.
7. **Out of range.** Type 101 or -1 in Interpolation: it is refused with a sentence saying it
   runs from 0 to 100, and the card keeps its old number.
8. **Draft.** Press **Draft**: the picture is smaller and bent the same way.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the
   conversion, the number and its keys are still there.

## Known limits, on purpose

- On a drawing that is not square the rings are ovals, fitted to the drawing, not circles.
- In Rect to Polar the corners are empty, as the drawing's edge is its outer ring, unless an
  effect above grew the layer, as Motion Tile does.
- Between 0 and 100 there is a tear straight up from the middle, where the left and right edges
  meet but have not yet joined.
- Detail is read at a few points, not averaged, so fine stripes near the middle can look rough,
  as the target's edges do in `target_unrolled.png`.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Polar Coordinates and is not claimed to match it. After
  Effects starts Interpolation at 0, which shows nothing; ours starts at 100.
- It costs little: the reference shot's frames 100 and 101 at full size take 49 to 54 ms
  without it, 76 to 79 ms with Rect to Polar at 100 on all four layers, and 76 to 81 ms with
  Polar to Rect.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
