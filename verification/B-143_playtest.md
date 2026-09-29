# B-143: 4-Color Gradient, by hand

Built on 2026-09-29 against D-208, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' 4-Color Gradient does, by a rule of our own: it
pins four colours to four points of the layer and runs them smoothly into each other over
everything the layer shows, yellow, green, magenta and blue in the four corners as it starts, for
a sky, a glow or a wash of colour across a cel. Blend says how far each colour reaches into the
others. It never spills past the drawing and does not grow the layer. It sits in the
**Generate** group after Beam.

The generated halves are `verification/B-143_four_color_gradient_table.md`, 126 of 126 checks
passing, which renders every FX-4CG case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks in the
window.

## The pictures

`verification/B-143 pictures/`, three times enlarged:

- `drawing.png`: the drawing at its own size, a face on a grey card with empty space around it;
  `before.png` is the same enlarged.
- `as_it_starts.png`: 4-Color Gradient as it starts: yellow in the top-left of the card, green in
  the top-right, magenta in the bottom-left and blue in the bottom-right, running into each
  other, the face painted over, the empty space around the card still empty.
- `blend_1.png`: Blend 1: four flat patches of the four colours meeting at sharp seams.
- `blend_1000.png`: Blend 1000: the four mixed nearly evenly, the card close to one colour.
- `opacity_50.png`: Opacity 50: the colours at half strength, the face showing through.
- `multiply.png`, `screen.png` and `add_30.png`: Blending Mode Multiply (tinted and darker, the
  lines kept dark), Screen (lighter), and Add at Opacity 30 (lighter).
- `point_1_middle.png`: Point 1 moved to the middle: the yellow sits on the face.
- `sunset_multiply.png`: gold and orange along the top, purple and navy along the bottom, laid
  on by Multiply: a sunset over the drawing.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-143 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **4-Color Gradient** in **Add effect…**, under
   **Generate**, after Beam; typing "four", "color", "gradient" or "sky" in the search finds it
   too. The card shows Point 1 10, 10, Point 2 90, 10, Point 3 10, 90, Point 4 90, 90, Color 1
   yellow, Color 2 green, Color 3 magenta, Color 4 blue, Blend 100, Opacity 100 and Blending Mode
   Normal, and the picture is as `as_it_starts.png` at once.
2. **Blend.** Drag Blend to 1, then 1000: as `blend_1.png`, then `blend_1000.png`. Between them
   the seams soften smoothly.
3. **Opacity.** Opacity 50: as `opacity_50.png`. Opacity 0: the drawing as it was.
4. **Blending modes.** Blending Mode Multiply, then Screen, then Add with Opacity 30: as
   `multiply.png`, `screen.png` and `add_30.png`.
5. **Moving a point.** Point 1 50, 50: as `point_1_middle.png`, the yellow in the middle.
6. **Sunset.** Point 1 20, 0, Point 2 80, 0, Point 3 20, 100, Point 4 80, 100, Colors #ffd060,
   #ff6040, #6040a0 and #203070, Blending Mode Multiply: as `sunset_multiply.png`.
7. **Keyed.** Key Point 1 from 10, 10 at the first frame to 90, 90 at frame 24 and play: the
   yellow drifts across the card.
8. **Moved.** Move the layer: the colours move with the drawing.
9. **Out of range.** Type 0 or 1001 in Blend, 101 in Opacity or 1001 in Point 1: it is refused
   with a sentence saying what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller with the same colours in the same places.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects' 4-Color Gradient has a Jitter setting, a fine grain that hides banding; ours has
  none. Put Noise after it for the same.
- It has four blending modes, Normal, Multiply, Screen and Add, the ones Gradient and Fractal
  Noise have; After Effects offers its full list.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' 4-Color Gradient and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 49 to 58 ms without it; 4-Color Gradient as it starts, 63 to 67 ms; Blend 1, 60 to 67
  ms; Multiply at Opacity 60, 62 to 66 ms. **Machine:** AMD Ryzen 9 9900X with 24 threads;
  Windows 11; release build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
