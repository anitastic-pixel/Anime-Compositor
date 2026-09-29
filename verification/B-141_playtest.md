# B-141: Roughen Edges, by hand

Built on 2026-09-29 against D-206, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Roughen Edges does, by a rule of our own: it eats
into the layer's edges by a rough amount that changes along them, set by noise, so a clean
cut-out looks torn, worn or burnt. It never adds anything: a pixel only ever loses covering.
Roughen Color also colours the band just inside the new edge, rust brown as it starts. It does not
grow the layer. It sits in the **Stylize** group after Kaleidoscope.

The generated halves are `verification/B-141_roughen_edges_table.md`, 121 of 121 checks passing,
which renders every FX-ROUGH case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks in the window.

## The pictures

`verification/B-141 pictures/`, three times enlarged:

- `drawing.png`: the plate at its own size, a red card, a blue disc and a yellow bar on nothing;
  `before.png` is the same enlarged, nothing shown as a grey-and-white check by your viewer or
  as black.
- `as_it_starts.png`: Roughen Edges as it starts, border 8, scale 10, complexity 3: every edge
  bitten, up to 8 pixels deep; the middles untouched.
- `border_16.png`: border 16: deeper bites; the thin yellow bar almost gone.
- `scale_40.png`: scale 40: wider, gentler bites.
- `complexity_6.png`: complexity 6: the same broad bites with finer detail along them.
- `seed_3.png`: seed 3: other bites.
- `roughen_color.png`: Roughen Color: the same bites, with a rust band just inside the new edge,
  like burnt paper.
- `roughen_color_blue_16.png`: Roughen Color in blue at border 16: a wider band.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-141 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Roughen Edges** in **Add effect…**, under **Stylize**,
   after Kaleidoscope; typing "torn", "ragged", "burnt" or "rough" in the search finds it too.
   The card shows Edge Type set to Roughen, an Edge Color swatch in rust, Border 8, Scale 10,
   Complexity 3, Evolution 0, Evolution Speed 0 and Random Seed 0, and the picture is as
   `as_it_starts.png` at once.
2. **Border.** Drag Border to 0: the drawing, untouched. Up to 16: as `border_16.png`. Nothing
   ever grows past the drawing's own edge.
3. **Scale and Complexity.** Scale 40 as `scale_40.png`; Complexity 6 as `complexity_6.png`.
   Complexity 3.9 looks the same as 3.
4. **Seed and Evolution.** Random Seed 3 as `seed_3.png`. Drag Evolution: the bites change
   smoothly, not in jumps.
5. **Evolution Speed.** Set 10 and play: the edges crawl by themselves, frame after frame.
6. **Roughen Color.** Pick Roughen Color in Edge Type: as `roughen_color.png`, the covering the
   same as before and a rust band inside the edge. Pick another Edge Color: the band takes it.
7. **Keyed.** Key Border from 0 at the first frame to 16 at frame 48 and play: the edges are eaten
   away gradually.
8. **Moved.** Move the layer: the bites move with the drawing, not the other way.
9. **Out of range.** Type 501 in Border, 0 in Scale or 11 in Complexity: it is refused with a
   sentence saying what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller with the same bites.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- The bites are measured straight across and straight down, so a slanted edge is bitten a little
  less deep than a straight one.
- A gap narrower than the bites can be jumped: a thin sliver of drawing next to one may be left
  standing, and small specks can break off on their own.
- After Effects' Roughen Edges also has Edge Sharpness, Fractal Influence, Stretch Width or
  Height, Offset, a looping evolution and seven more edge types (Cut, Spiky, Rusty and Photocopy
  and their colour kinds); ours has Roughen and Roughen Color.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Roughen Edges and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 49 to 51 ms without it; Roughen Edges as it starts, 95 to 108 ms; Roughen Color, 102 to
  110 ms; Roughen Color at border 40 and complexity 10, 153 to 168 ms. **Machine:** AMD Ryzen 9
  9900X with 24 threads; Windows 11; release build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
