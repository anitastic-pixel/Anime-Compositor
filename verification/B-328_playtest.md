# B-328: Cartoon

Built on 2026-10-10 as D-448, under your /loop request: **Cartoon**, in **Stylize**. It makes
the picture look drawn: flat areas of colour, the shading cut into a few steps, and dark lines
along the edges.

- **Render**: Fill (the flat colours alone), Edges (the lines alone, on paper) or Fill & Edges
  (both, when added).
- **Detail Radius** (8 pixels, 0 to 50) and **Detail Threshold** (10, 0 to 255): how far and how
  much the picture is smoothed into flat areas first; 0 leaves it as it is.
- **Shading Steps** (8, 2 to 64): how many shades of light and dark the fill keeps.
- **Shading Smoothness** (40, 0 to 100): 0 hard steps, 100 the shading as it was.
- **Edge Threshold** (1.6, 0 to 10): how strong an edge must be to get a line.
- **Edge Width** (1.5 pixels, 0 to 10) and **Edge Softness** (60, 0 to 100): how thick the lines
  are and how soft their sides.
- **Edge Opacity** (100%): 0 hides the lines.
- **Edge Enhancement** (0, -100 to 100): above 0 the colours either side of an edge are pushed
  apart, sharper; below 0 drawn together, softer.
- **Edge Black Level** (0, 0 to 1): the lines' shade, 0 black, 1 white.
- **Edge Contrast** (0.5, 0 to 1): how suddenly lines appear as edges get stronger.

The controls are After Effects' own; how they draw is our own rule (Adobe publishes none), not
matched to After Effects pixel for pixel.

The check, `verification/D-448_cartoon_table.md` (191 of 191), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to within
1 level of the processor's. The pictures are in `verification/D-448 pictures/`, over the street
(`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street in flatter colours with dark lines round the
   buildings, windows and edges.
2. **Fill.** `3_fill.png`: the same flat colours with no lines.
3. **Edges.** `4_edges.png`: dark lines on white paper, like an inked drawing.
4. **Thick lines.** `5_thick_edges.png` (width 5, softness 0): heavier, hard-edged lines.
5. **Smoother.** `6_smoother.png` (Fill, detail radius 20, threshold 60): larger flat areas.
6. **White lines.** `7_black_level_1.png` (Edges, black level 1): white lines on black.
7. **In the app.** Add Cartoon (Stylize) to a picture. Switch Render between Fill, Edges and Fill
   & Edges; drag Shading Steps down to 2 or 3 for a poster look; widen Edge Width; try Edge
   Enhancement both ways.
8. **Out of range.** Type 1 in Shading Steps: it is refused with a sentence giving the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-328_cartoon_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Cartoon on three layers, played again, 48.1 ms a frame on the card as added
against 12.6 for the Noise alone; on the processor 665.1 against 44.8:
a busy machine cannot say whether the card is quicker than the processor for this effect.
