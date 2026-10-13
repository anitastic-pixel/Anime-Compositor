# B-555: Scatterize

Built on 2026-10-13 as D-676 under your /loop request: **Scatterize**, in **Generate** beside
Foam, after what CycoreFX's CC Scatterize is for. It blows the layer apart into dust. Every pixel
becomes a dot thrown away from where it was. The layer can also be twisted like a ribbon about its
middle row.

Nothing moves by itself. A frame is just the settings as they are at that frame, so key Amount or
the twists to animate it.

The settings (the value when added in brackets):

- **Amount** (50): how far the dots fly, in pixels on a picture 1080 high. Below 0 every dot is
  thrown the opposite way.
- **Right Twist** (0) and **Left Twist** (0): how far the right and left edges turn about the
  middle row, in degrees. The columns between turn by a share of each.
- **Transfer Mode** (Composite): how the dots are laid on each other: Composite, Screen, Add or
  Alpha Add.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/scatterize_reference.py`.

The check is `verification/D-676_scatterize_table.md`, and 142 of 142 checks pass. It
compares every pixel with numbers that a separate program worked out before the code existed. It
also checks that the graphics card's picture stays within 1 level of the processor's (41 of
41 comparisons). The pictures are in `verification/D-676 pictures/` and use the street picture
from D-506.

## What to check

1. **As added.** `1_as_added.png`: the street dusted all over with fine white specks. Sky, buildings
   and road are still easy to tell apart, but the windows and road markings are lost in the grain.
2. **Cloud.** `2_cloud_amount_300.png`: the street dissolved into a cloud of dots: blue at the top,
   dark grey at the bottom, with faint patches of the buildings' colours between. The shapes are
   only guessable.
3. **Ribbon.** `3_ribbon_twisted_180.png`: no scatter, the ends turned opposite ways. The street is
   twisted like a ribbon: pinched to a point about a quarter and three quarters of the way across,
   upright in the middle, and upside down (road along the top) at both sides.
4. **Edge-on with Add.** `4_right_edge_on_with_add.png`: the street, lightly dusted, full height at
   the left and narrowing steadily to a point at the right edge, which is turned edge-on.
5. **Blown away.** `5_blown_away_amount_3000.png`: the dots thrown far. The picture is mostly white,
   with a thin even scatter of tiny blue, grey and coloured dots.
6. **Screened cloud.** `6_screened_cloud.png`: a cloud of dots like picture 2, laid by Screen, so
   the whole picture is lighter and paler where the dots pile up.
7. **In the app.** Put Scatterize (Generate) on the street picture and turn Amount up and down.
   The street should dissolve into dust and come back.
8. **Keyed amount.** Key Amount from 0 to 600 and play. The street should blow apart smoothly.
9. **Out of range.** Type 20000 in Amount. It should be refused with a sentence that gives the
   range.
10. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added and the units of Amount;
- that each dot covers one pixel;
- that there is no seed (CC Scatterize has none).

## Speed

`verification/B-555_scatterize_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 16.3 / 51.5, as added 584.3 / 620.9; Scatterize, Amount 300, the right edge edge-on 615.8 / 808.7; Scatterize, Amount keyed 0 to 600, Add 655.4 / 603.4; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
