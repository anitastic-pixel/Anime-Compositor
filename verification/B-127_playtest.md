# B-127: Compound Blur, by hand

Built on 2026-09-29 against D-191, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Compound Blur does, by a rule of our own: one
layer is blurred by another layer of the same composition, the map, more where the map is
brighter. White in the map is the Maximum Blur, black is none, grey in between. Paint a depth
map, or use any layer you have, and the picture goes soft where it says. It is the first effect
that reads another layer (D-189).

The generated halves are `verification/B-127_compound_blur_table.md`, 125 of 125 checks passing,
which renders every FX-CBLUR case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Compound Blur
card sends every setting the command reads. This sheet covers what the tables cannot: how it
looks and feels in the window.

## One question for you: D-192

One fixture case, FX-CBLUR-019, is wrong in its file. It says the blur is on an adjustment layer
**above** the drawing, but its file puts the adjustment layer **beneath** it, where it has nothing
to work on. The build draws the file as written, the drawing unblurred. With the adjustment layer
moved above, the build gives exactly the case's expected picture; the table checks both. The
proposal is to correct the file so it says what the case says. Nothing else changes. Answer
"D-192 yes" or "D-192 no" with the rest.

## The pictures

`verification/B-127 pictures/`, a street at a quarter of 1920 by 1080. A quarter-size picture
takes a quarter of the blur, as a quarter-size draft does, so Maximum Blur 10 here shows what 40
does at full size:

- `town.png`: the street, a sky, a row of houses with lit and dark windows, a road.
- `depth.png`: the depth map, black along the house fronts, whitening above and below.
- `before.png`: the street with no effect.
- `depth_of_field.png`: the depth map, Edges Repeat Edge Pixels. The house fronts are sharp,
  the sky and the top of the houses soft, the road a little soft. The rows the map leaves black
  are exactly as they were.
- `depth_inverted.png`: the same with Invert Blur on. The house fronts are soft and the sky
  sharp.
- `edges_transparent.png`: the depth map with Edges Transparent. The sky fades to clear at the
  frame's rim, as a Gaussian Blur with those edges does.
- `own_brightness.png`: the street naming itself as its map. The bright sky and the lit windows
  are the softest, the dark road the least soft.

## Before you start

Import `verification/B-127 pictures/town.png` and `verification/B-127 pictures/depth.png`, make
a layer from each with `town` on top, make the composition 480 by 270, and press **Full
resolution**. Switch the `depth` layer off with its eye: a map is read whether or not it is
shown.

## What to check

1. **Adding it.** On `town`, pick **Compound Blur** in **Add effect…**, under **Blur &
   Sharpen**, after Lens Blur; typing "compound", "depth" or "focus" in the search finds it too.
   The card shows **Blur Layer** None, **Maximum Blur** 20, **If Sizes Differ** Stretch Map to
   Fit, **Invert Blur** Off and **Edges** Transparent. With no layer named, nothing changes.
2. **The map.** Set Blur Layer to `depth`, Maximum Blur 10 and Edges Repeat Edge Pixels: it
   looks like `depth_of_field.png`. The list names every layer of the composition; `town`
   says "(this layer)".
3. **Invert.** Set Invert Blur On: like `depth_inverted.png`. Back to Off.
4. **Edges.** Set Edges Transparent: like `edges_transparent.png`.
5. **Itself.** Set Blur Layer to `town (this layer)`: like `own_brightness.png`.
6. **Nothing.** Set Maximum Blur 0, or name a layer that is all black: the picture is exactly as
   imported.
7. **Out of range.** Type 501 in Maximum Blur: it is refused with a sentence saying it runs from
   0 to 500, and the card keeps its old number.
8. **Sizes.** Make a small solid, 120 by 60, white, and name it. Stretch Map to Fit: the whole
   picture is blurred the most. Centre: only the middle 120 by 60 is blurred. Tile: all of it,
   since white repeated is white. With a map that is not all one colour, Tile repeats its
   pattern across the layer.
9. **A circle.** Add a Compound Blur to `depth` too, and set its Blur Layer to `town`. It is
   refused with a sentence: the two layers would read each other in a circle. The card keeps
   None. Naming the layer itself is never a circle.
10. **A missing layer.** Delete `depth` while `town` names it: the blur goes, and the warning
    panel says the effect reads a layer that is not in the composition. Undo: `depth` comes back
    and so does the blur. Saving and opening again with a layer missing keeps the name and says
    so on opening.
11. **Keyed.** Key Maximum Blur from 0 at the first frame to 10 a second later: the soft parts
    grow soft, the house fronts stay sharp throughout.
12. **Moving.** Move the `depth` layer, scale it or rotate it: nothing changes, since a map is
    read before its layer is placed. Move `town`: the blur moves with it, since the map lies on
    the layer.
13. **Draft.** Press **Draft**: the picture is smaller and the blur with it, the same parts soft.
14. **Copies.** Duplicate `town` while it names itself: the copy names the copy. Duplicate the
    composition: the new `town` names the new `depth`. Keep the card as a preset and add it to
    another layer: the preset names no layer, and you choose one.
15. **Undo and saved.** Ctrl+Z steps back each change, the layer named included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- A map is read the size of its own drawing and fitted to the layer by If Sizes Differ, not
  where it sits in the frame. On an adjustment layer the map is fitted to the frame.
- The blur is at most a third of Maximum Blur as a Gaussian's sigma, and between the six levels
  (none, a sixteenth, an eighth, a quarter, a half, all) each pixel mixes the two it falls
  between, so a smooth map gives a smooth change.
- It is drawn on the processor; the graphics card learns it in a card unit of its own. On the
  reference shot, 1920 by 1080 at frame 100, with the effect on the bottom layer naming layer 2,
  inverted, three runs each: 51 to 58 ms without it; 199 to 205 ms at Maximum Blur 20 with
  transparent edges; 184 to 190 ms at 40 repeated; 209 to 211 ms at 100 repeated; and 390 to
  403 ms at 500 repeated, the most it goes.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did; and "D-192 yes" or "D-192 no".
