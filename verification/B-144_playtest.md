# B-144: Cell Pattern, by hand

Built on 2026-09-29 against D-209, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Cell Pattern does, by a rule of our own: it lays
a pattern of cells over everything the layer shows, bubbles, crystals, cracked plates or flat
grey tiles, in two colours, black and white as it starts, for a texture, a background or a magic
surface on a cel. Evolution sends each cell's point round a little circle, so keying it makes
the pattern churn. It never spills past the drawing and does not grow the layer. It sits in the
**Generate** group after 4-Color Gradient.

The generated halves are `verification/B-144_cell_pattern_table.md`, 157 of 157 checks passing,
which renders every FX-CELL case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks in the
window.

## The pictures

`verification/B-144 pictures/`, three times enlarged, every one at Size 16 (the card starts at
60, so the cells are bigger there):

- `drawing.png`: the drawing at its own size, a face on a grey card with empty space around it;
  `before.png` is the same enlarged.
- `bubbles.png`: Bubbles: grey balls over the whole card, white at their middles and dark where
  they meet, the face painted over, the empty space around the card still empty.
- `crystals.png`: Crystals: the same cells as faceted cones, darker on the whole.
- `plates.png`: Plates: flat white plates, dark only in the seams between them.
- `static_plates.png`: Static Plates: each cell one flat grey, like stained glass in greys.
- `invert.png`: Invert On: the bubbles turned over, dark balls on light seams.
- `contrast_300.png`: Contrast 300: the greys pushed out to black and white.
- `disperse_0.png`: Disperse 0: the balls in a tidy square grid.
- `evolution_90.png`: Evolution 90: the balls moved from where `bubbles.png` has them.
- `navy_gold_multiply.png`: Dark Colour navy, Light Colour gold, laid on by Multiply: the card
  warm in the balls and cool in the seams, the face's lines still dark.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-144 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Cell Pattern** in **Add effect…**, under **Generate**,
   after 4-Color Gradient; typing "cell", "bubbles", "voronoi" or "stained glass" in the search
   finds it too. The card shows Cell Pattern Bubbles, Invert Off, Contrast 100, Disperse 1, Size
   60, Evolution 0, Random Seed 0, Dark Colour black, Light Colour white, Opacity 100 and Blend
   Normal, and the card is covered in big grey balls at once.
2. **As the pictures.** Size 16: as `bubbles.png`. Then Cell Pattern Crystals, Plates and Static
   Plates: as `crystals.png`, `plates.png` and `static_plates.png`. Back to Bubbles.
3. **Invert and contrast.** Invert On: as `invert.png`. Invert Off, Contrast 300: as
   `contrast_300.png`. Contrast back to 100.
4. **Disperse.** Disperse 0: as `disperse_0.png`. Drag it up to 1.5: the grid loosens into
   uneven cells.
5. **Evolution.** Disperse 1, Evolution 90: as `evolution_90.png`. Key Evolution from 0 at the
   first frame to 360 at frame 48 and play: the cells churn, and frame 48 looks like frame 0.
6. **Random Seed.** Seed 1, then 2: a different arrangement each time; Seed 0 brings the first
   back.
7. **Colours and blending.** Dark Colour #203070, Light Colour #ffd060, Blend Multiply: as
   `navy_gold_multiply.png`. Opacity 50: the pattern at half strength. Opacity 0: the drawing as
   it was.
8. **Moved.** Move the layer: the cells move with the drawing.
9. **Out of range.** Type 1001 in Contrast, 2 in Disperse, 0 in Size or 101 in Opacity: it is
   refused with a sentence saying what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller with the same cells in the same places.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- It has four patterns, Bubbles, Crystals, Plates and Static Plates. After Effects' others,
  Pillow, Mixed Crystals, Tubular and the Textured and Static versions of the rest, are left
  out, as are its Tiling Options, Offset and Overflow; say if one is missed.
- It has four blending modes, Normal, Multiply, Screen and Add, the ones Fractal Noise and
  4-Color Gradient have; After Effects offers its full list.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Cell Pattern and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 50 to 60 ms without it; Cell Pattern as it starts, 64 to 76 ms; Size 4, the cells
  smallest, 69 to 75 ms; Static Plates, 62 to 66 ms; Multiply at Opacity 60, 64 to 71 ms.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a throwaway
  test, not kept.

## What to answer

"works", or which step number did something else and what it did.
