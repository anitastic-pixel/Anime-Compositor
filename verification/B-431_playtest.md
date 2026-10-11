# B-431: CC Ball Action

Built on 2026-10-11 as D-551, under your /loop request: CC Ball Action, in **Generate** (there is
no Simulation group), after CycoreFX's effect of that name. The layer is rebuilt as a grid of
shiny balls, each the colour of the picture under it, which can be turned, twisted and scattered
in 3D. Like CC Cylinder, it has its own eye and its own light: it does not follow a comp camera
or comp lights, which this program does not have.

- **Scatter** (0): how far the balls fly apart, in every direction.
- **Rotation Axis** (X): what the grid turns about: X, Y, Z, two or three of them together, or
  X15Z and XY15Z, which spin 15 times round Z for each turn round the others.
- **Rotation** (0): how far the grid turns.
- **Twist Property** (X Axis): what decides how much each ball is turned on top of the Rotation:
  its place across or down, its distance from the middle (Radius, Diamond, Rectangle), chance
  (Random), or how little red, green, blue or brightness it has. Fast Top spins round X and twice
  as fast round Y.
- **Twist Angle** (0): how much that twist turns the balls.
- **Grid Spacing** (8): the distance between balls; larger makes fewer, bigger balls.
- **Ball Size** (100): each ball's size as a share of the spacing; 100 makes neighbours touch.
- **Instability State** (0): with Scatter above 0, turns each scattered ball round its own place.

Only fully solid pixels become balls. The layer does not grow: balls that leave it are cut off.

CycoreFX publishes no formula, no lighting and no settings when added, so the rule, the light
(from the upper left, with a white highlight) and those settings are ours, written down in
document 21.

The check, `verification/D-551_cc_ball_action_table.md` (235 of 235), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-551 pictures/`, all made from the street picture of D-506.

## What to check

1. **As added.** `1_as_added.png`: the street as small touching balls, lit from the upper left.
2. **Bigger balls.** `2_large_balls.png`: grid spacing 16: fewer, larger balls.
3. **Turned.** `3_turned_about_y.png`: rotation 50 about Y: the grid swung round, the near side
   larger than the far side.
4. **Twisted.** `4_twist_radius.png`: twist Radius 120: the middle turned most, the edges least.
5. **Scattered.** `5_scattered.png`: scatter 40 and instability 90: the balls flung apart, near
   ones in front of far ones.
6. **In the app.** Add CC Ball Action (Generate) to a layer, keyframe Rotation from 0 to 720,
   and play. Try each Rotation Axis and Twist Property, and Scatter with Instability State.
7. **Out of range.** Type 1 in Grid Spacing: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; the fixed light and how shiny
the balls are; that a pixel shows at most its two nearest balls; that the balls have no shadows.

## Speed

`verification/B-431_cc_ball_action_timing_table.md`. Three CC Ball Actions on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet: other lanes' two B-155 runs and their cargo processes were running), card 76.5, 68.5 and 76.9 ms a frame (as added; grid spacing 4, rotation 40 about XY; scatter 60, ball size 300) against 45.0 for the Noise alone, about 8 to 11 ms added a 1080p layer; processor 246.8, 280.7 and 311.5 against 102.1; a quiet re-measure is owed.
