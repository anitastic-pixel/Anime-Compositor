# B-132: Change to Color, by hand

Built on 2026-09-29 against D-197, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Change to Color does, by a rule of our own: it
turns one colour of a drawing into another, a red jacket into a blue one, and keeps the jacket's
shading, its highlight and its shadow, while every other colour stays as it was. It sits in the
**Colour Correction** group after Leave Color, and its settings carry After Effects' own names.

The generated halves are `verification/B-132_change_to_color_table.md`, 105 of 105 checks
passing, which renders every FX-CTC case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Change to
Color card sends every setting the command reads. This sheet covers what the tables cannot: how
it looks and feels in the window.

## The pictures

`verification/B-132 pictures/`, a figure in a red jacket (#c82828) with a light stripe down its
left side, a shadow down its right, a pink ribbon at the collar and a face shaded on its right,
three times enlarged:

- `before.png`: no effect.
- `hue.png`: From the jacket's red to #0080ff, **Hue**: the jacket, its highlight and its shadow
  turn blue, each as light or dark as it was. The pink ribbon, the face and the paper keep their
  colours.
- `transforming.png`: the same with **Transforming To Color**: each red is turned as far round
  the colour wheel as #0080ff is from the red, so the result is almost the same here.
- `flat.png`: **Hue, Lightness & Saturation**: the jacket becomes one flat #0080ff, its shading
  gone.
- `matte.png`: **View Correction Matte** on: white where the colour changes, black elsewhere.

## Before you start

Make the composition 160 by 100 and import `plate.png` from `verification/B-132 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the plate, pick **Change to Color** in **Add effect…**, under **Colour
   Correction**, after Leave Color; typing "change to color", "replace", "recolour" or "swap" in
   the search finds it too. The card shows **From** red (#ff0000), **To** a sky blue (#0080ff),
   **Change** Hue, **Change By** Setting To Color, **Hue Tolerance** 5, **Lightness
   Tolerance** 50, **Saturation Tolerance** 50, **Softness** 50 and **View Correction Matte**
   Off. The jacket already turns blue: its red, highlight and shadow are near enough to pure
   red. The pink ribbon does not: its hue is too far from red.
2. **The jacket.** Set From to #c82828, the jacket's own red: the jacket is blue with its
   highlight and shadow, as in `hue.png`. The pink ribbon and the face stay as they were.
3. **Flat.** Set Change to Hue, Lightness & Saturation: the jacket is one flat blue, as in
   `flat.png`. Hue & Lightness and Hue & Saturation are the two halves of that.
4. **Transforming.** Back to Hue; set Change By to Transforming To Color: much the same, as in
   `transforming.png`. Set To to a green: the jacket turns green, and with Hue Tolerance 30 the
   pink ribbon turns green too, a little yellower, turned as far round as the jacket is.
5. **The matte.** Set View Correction Matte to On: the picture is black and white, white where
   the colour changes, as in `matte.png`, grey where it changes part way. Set it back to Off.
6. **Tolerances.** Change To back to #0080ff and Change By to Setting To Color. Hue
   Tolerance 0: only the jacket's exact hue changes, and its shadow, a slightly different red,
   stays red. Hue Tolerance 30: the pink ribbon changes too. Back to 5, then Lightness
   Tolerance 0: only colours exactly as light as the jacket's red change, and its highlight
   and shadow stay red. Set it back to 50.
7. **Softness.** Softness 0: a colour is either changed whole or not at all, a hard edge in the
   matte. Softness 100: the change fades out gently past each tolerance.
8. **Soft edges and empty space.** Soft edges of the jacket change with it, and the empty parts
   of a drawing stay empty.
9. **Out of range.** Type 101 in Hue Tolerance: it is refused with a sentence saying it runs
   from 0 to 100, and the card keeps its old number. The same for the other two tolerances and
   Softness.
10. **Keyed.** Key Hue Tolerance from 0 at the first frame to 30 at a later one and play: the
    change spreads from the jacket's red to its shadow, then to the ribbon.
11. **Draft.** Press **Draft**: the picture is smaller and changed the same way.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every
    setting and key is still there.

## Known limits, on purpose

- To turn a colour grey, or grey into a colour, set Change to Hue & Saturation or more: Hue
  alone keeps the saturation, so a grey To leaves the colour as strong as it was.
- A line drawn in the From colour changes with the fill; there is no telling them apart.
- The colours themselves cannot be keyed; the three tolerances and softness can.
- It changes the picture alone, not the layer's shape: nothing grows.
- It is a picture effect, like Leave Color: there is no card version yet; that comes later as
  its own unit.
- It is modelled on After Effects' Change to Color and is not claimed to match it.
- It costs little: the reference shot's frames 100 and 101 at full size take 48 to 57 ms
  without it, 54 to 58 ms with Change to Color on all four layers, and 57 to 60 ms with every
  tolerance at 100, so that every pixel changes.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
