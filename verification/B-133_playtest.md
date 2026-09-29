# B-133: Corner Pin, by hand

Built on 2026-09-29 against D-198, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Corner Pin does, by a rule of our own: it pins
the drawing's four corners to four new points and stretches the drawing between them in
perspective, a poster onto a wall that leans away, or a picture onto a screen. The layer grows
so no corner is cut off. It sits in the **Distort** group after Motion Tile, and its settings
carry After Effects' own names.

The generated halves are `verification/B-133_corner_pin_table.md`, 76 of 76 checks passing,
which renders every FX-PIN case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Corner Pin card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks
and feels in the window.

## The pictures

`verification/B-133 pictures/`, a poster (a blue sky, a sun up on the right, a red strip along
the bottom, a dark border), three times enlarged, over a grey check where nothing is drawn:

- `before.png`: no effect.
- `keystone.png`: Upper Left (25, 0) and Upper Right (75, 0): the top drawn in, as a poster
  seen from below.
- `wall.png`: the four corners at (52, 18), (88, 8), (52, 70) and (88, 86): the poster on a
  wall on the right that leans away, its far edge shorter.
- `flipped.png`: the left and right corners swapped: the poster turned over, the sun on the
  left.
- `crossed.png`: the two bottom corners swapped, a bow tie: nothing is drawn.

## Before you start

Make the composition 160 by 100 and import `plate.png` from `verification/B-133 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the plate, pick **Corner Pin** in **Add effect…**, under **Distort**, after
   Motion Tile; typing "corner pin", "perspective", "billboard" or "screen" in the search finds
   it too. The card shows **Upper Left** 0, 0, **Upper Right** 100, 0, **Lower Left** 0, 100 and
   **Lower Right** 100, 100, and above them a box standing for the drawing with a mark at each
   corner and the shape they make shaded blue. The picture is unchanged.
2. **Dragging.** Drag the upper left mark in the box towards the middle: the blue shape and the
   picture follow, the drawing stretched to fit. Typing the numbers does the same.
3. **Keystone.** Set Upper Left to 25, 0 and Upper Right to 75, 0: the picture is as
   `keystone.png`, the top drawn in, the top corners empty.
4. **The wall.** Set the four corners to 52, 18; 88, 8; 52, 70; 88, 86: as `wall.png`, the
   poster on the right half, its right edge taller than its left.
5. **Past the edges.** Set Upper Left to -25, -25: the poster reaches past the composition's
   top left corner and nothing is cut off inside the frame. Move the layer to the right and down
   and the part that was past the edge comes into view.
6. **Turned over.** Swap the corners left to right (100, 0; 0, 0; 100, 100; 0, 100): as
   `flipped.png`, the sun on the left.
7. **Crossed.** Back to the start, then set Lower Left to 100, 100 and Lower Right to 0, 100: as
   `crossed.png`, nothing is drawn. Pull Lower Right in to 30, 30, a corner bent inwards: nothing
   is drawn either. Set them back.
8. **Out of range.** Type -401 or 501 in any corner: it is refused with a sentence saying it runs
   from -400 to 500, and the card keeps its old numbers.
9. **Keyed.** Key Upper Right from 100, 0 at the first frame to 100, 50 at a later one and play:
   the top right corner slides down and the drawing follows.
10. **With Motion Tile.** Add Motion Tile above Corner Pin at 300 by 300 and set the corners to
    25, 25; 75, 25; 25, 75; 75, 75: the tiled drawing is shrunk into the middle of the frame,
    the tiles repeating across it.
11. **Draft.** Press **Draft**: the picture is smaller and pinned the same way.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every corner
    and key is still there.

## Known limits, on purpose

- Crossed corners, a corner bent inwards past its neighbours, or three corners in a line draw
  nothing at all, by D-198's rule. The warning panel says nothing: the settings are valid, the
  shape is empty.
- A corner far out, much further than the others, can make the drawing reach the horizon of the
  perspective; nothing is drawn beyond it, where the drawing would otherwise appear again turned
  over.
- The corners are in per cent of the drawing's own size, as the other effects' points are, not
  in pixels as in After Effects.
- There is no motion blur of its own and no tracking: the corners are keyed by hand.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Corner Pin and is not claimed to match it.
- It costs little: the reference shot's frames 100 and 101 at full size take 48 to 54 ms
  without it, 70 to 77 ms with Corner Pin on all four layers pinned to a wall, and 109 to
  116 ms with the corners pulled far outside, so every layer grows.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
