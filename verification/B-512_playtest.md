# B-512: Card Dance

Built on 2026-10-12 as D-632, the third of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request: Card Dance, under
**Generate** next to Caustics, our version of After Effects' effect of that name. It cuts the
layer into a grid of cards, like a stadium crowd holding up cards, and moves, turns and scales
each card by the colour of a gradient layer under its middle, seen in 3D by a camera and lit.

- **Rows & Columns** (Independent), **Rows** (8), **Columns** (8): how many cards; Columns
  Follow Rows keeps the two equal. Up to 1000 each.
- **Back Layer** (none: clear backs): the picture on the cards' backs.
- **Gradient Layer 1** and **Gradient Layer 2** (none: the layer's own picture): the pictures
  whose colours move the cards.
- **Rotation Order** (XYZ) and **Transformation Order** (Scale, Rotate, Position): the order the
  turns and the moves are done in.
- For each of **X, Y, Z Position**, **X, Y, Z Rotation** and **X, Y Scale**: a **Source** (none,
  or red, green, blue, intensity or alpha of either gradient layer), a **Multiplier** and an
  **Offset**. White reads +1, black -1, middle grey 0, times the multiplier, plus the offset. As
  added only Z Position reads Gradient Layer 1's brightness: bright cards come toward you.
- **Camera System** (Camera Position, the effect's own camera with its turns, place, distance
  and Focal Length; or Comp Camera, the composition's camera).
- **Light Type**, **Light Intensity**, **Light Color**, **Light Position**, **Light Depth**,
  **Ambient Light**, **Diffuse Reflection**, **Specular Reflection**, **Highlight Sharpness**:
  Card Wipe's light, shining on the cards.

Adobe gives no formula and few settings when added, so the rule, the size of a Position step
(one card) and the settings when added are ours. Not built yet: the Corner Pins camera (next,
D-634), and lighting from the composition's first light (there are no light layers yet).

The check, `verification/D-632_card_dance_table.md` (243 of 243), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-632 pictures/`, drawn at the reference shot's size from it.

## What to check

1. **As added.** `1_as_added.png`: the picture almost unchanged; the brighter cards have come a
   little toward you and the darker gone back, so thin seams show between cards.
2. **Tilted cards with backs.** `2_tilted_cards_backs.png`: every card turned the same way, like
   slats, with a back picture showing where a card's back faces you.
3. **Turned by a ramp.** `3_turned_by_the_ramp.png`: cards turned about their upright by a
   left-to-right ramp, more toward one side, like a fan of cards.
4. **Scattered by colours.** `4_scattered_by_colours.png`: each card pushed sideways, up and down
   and in depth by its red, green and blue, so the grid breaks apart.
5. **Seen from the side.** `5_seen_from_the_side.png`: the whole grid turned away from you by the
   camera, shrinking into the distance.
6. **The composition camera.** `6_composition_camera.png`: the cards seen through the
   composition's camera instead of the effect's own.
7. **A point light.** `7_point_light_shine.png`: tilted cards lit by a near light, a bright
   shine toward the upper left.
8. **In the app.** Add Card Dance (Generate) to a layer and play: the cards should stand out from
   the picture by its brightness. Set Y Rotation's Source to Intensity 1 and raise its
   Multiplier: the cards should turn by the picture's light and dark.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how far a card moves for one step of Position (one card
size), and that a deleted gradient or back layer reads as clear rather than as the layer.

## Speed

Measured on a quiet machine (`verification/B-512_card_dance_timing_table.md`). On the processor at the reference shot's size (1920 by 1080), a frame took about 49 to 51 ms with Card Dance as added, 52 to 53 ms with 100 by 100 tilted cards, and 234 to 249 ms with 1000 by 1000 cards, the most allowed, against 34 to 37 ms with no effect. Card Dance has no card path yet; up to a few thousand cards it should play near real time at full size, and Draft quality or a smaller preview helps past that.
