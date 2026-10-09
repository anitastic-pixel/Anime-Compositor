# B-267: Page Turn

Built on 2026-10-09 under your effects loop request, decided as D-388. **Page Turn** (in
**Distort**) is our version of CycoreFX's CC Page Turn: a corner of the layer (or, with Classic,
a straight edge at any angle) curls over like the page of a book, rolling round a cylinder, its
back showing as plain paper or as another layer. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Controls** (Bottom Right Corner): which corner turns, or Classic UI for a straight fold.
- **Fold Position** (75, 75): per cent of the layer's width and height. With a corner, where the
  corner is pulled to; with Classic, a point the fold passes through.
- **Fold Direction** (-60): Classic only, which way the page travels, in degrees.
- **Fold Radius** (30): how round the curl is, in pixels. 0 folds the page flat.
- **Light Direction** (-45): where the light comes from, shading the curl.
- **Render** (Front & Back Page): both sides, only the front, or only the back, so another layer
  can go between two copies.
- **Back Page** (none): a layer to show on the back of the page; none shows the paper.
- **Back Opacity** (100): how much the back page or paper covers; 0 shows the page's own picture
  seen through, mirrored.
- **Paper Color** (#f0f0f0, a pale grey).

The layer does not grow; what turns past its edge is cut off.

The check, `verification/D-388_page_turn_table.md` (192 of 192), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-388 pictures/`. They are
saved over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the bottom right corner of the street curls up and over, the
   curl shaded grey to white, the corner behind it white.
2. **Folded flat.** `3_half_flat.png`, Classic down the middle, Fold Radius 0: the left half is
   folded over onto the right, so the left half is white and the right half shows the pale paper
   back.
3. **Blue paper.** `4_corner_blue.png`: the corner turned to near the middle, its back blue.
4. **Seen through.** `5_seen_through.png`, Classic, Back Opacity 0: the turned part shows the
   street through the paper, mirrored and tilted.
5. **In the app.** Add Page Turn to a layer and key Fold Position from (100, 100) to (0, 0) over
   two seconds: the corner peels across the layer. Pick another layer as Back Page: it shows on
   the back. Switch Render to Front Page and Back Page: only that side is drawn.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Page Turn costs about 1.1 ms a 1080p layer, and about 9.5 ms on the processor
(`verification/B-267_distort_timing_table.md`; provisional, the machine was busy).

## Not built

- The cylinder, the shading and where the corner lands are our own reading of CycoreFX's
  one-sentence descriptions; no tutorial with numbers was matched against it.
- The back page is fixed to the paper, so it shows mirrored on a turned page.
- With a corner, while the curl is still rolled up (a big radius on a small layer) the whole page
  can roll away out of the frame, so the frame is clear.
- There is no handle on the viewer to drag the fold; set Fold Position in Effect Controls.
