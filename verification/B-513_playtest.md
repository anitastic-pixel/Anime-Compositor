# B-513: Shatter

Built on 2026-10-12 as D-633, the fourth of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request: Shatter, under **Generate**
next to Card Dance, our version of After Effects' effect of that name. It breaks the layer like
a pane of glass: the pieces a force reaches fly out from it as thick slabs, fall under gravity
and tumble, seen in 3D by a camera and lit.

- **View** (Rendered) and **Render** (All; Layer shows only the pieces still in place, Pieces
  only the flying ones).
- **Pattern** (Bricks; also Squares, Hexagons, Triangles 1 and 2, Rhombus, Planks,
  Herringbone 1, Octagons & Squares, Glass, Custom), **Custom Shatter Map**, **White Tiles
  Fixed**, **Repetitions** (10 across), **Direction**, **Origin**, **Extrusion Depth** (0.2 of
  the layer's width): the shape, size and thickness of the pieces.
- **Force 1** and **Force 2**: each a ball with a **Position**, **Depth**, **Radius** and
  **Strength**; pieces inside it are thrown out from its middle, or pulled in with a negative
  strength. Only Force 1 is on as added.
- **Gradient Layer**, **Shatter Threshold**, **Invert Gradient**: a grey layer that holds pieces
  back; white parts break first as the threshold rises.
- **Rotation Speed**, **Tumble Axis**, **Randomness**, **Viscosity**, **Mass Variance**,
  **Gravity**, **Gravity Direction**, **Gravity Inclination**: how the pieces fly.
- **Color**, **Opacity**, **Front**, **Side** and **Back Mode** with their layers: what the
  pieces' faces show.
- **Camera System** and Card Dance's camera, and Card Wipe's light.

Adobe gives no formula and few settings when added, so the rule, the units, the shapes of the
patterns and most settings when added are ours. Each frame throws the pieces again from the
layer's start, rather than carrying them on from the frame before as After Effects does; the
picture is the same either way when you play from the start. Not built yet: the wireframe views
(D-635), After Effects' other ten patterns (D-636) and the Corner Pins camera (D-634), all
refused with a warning rather than drawn as something else; and lighting from the composition's
first light (there are no light layers yet).

The check, `verification/D-633_shatter_table.md` (295 of 295), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-633 pictures/`, drawn at the reference shot's size from it.

## What to check

1. **As added.** `1_as_added_frame_2.png`: at frame 2 the middle bricks have burst outward as
   thick blocks, the bricks further out still in place.
2. **Hexagons.** `2_hexagons_burst.png`: six-sided pieces bursting from the middle.
3. **Glass, two forces.** `3_glass_two_forces.png`: two holes of jagged glass pieces: the left
   force throws its pieces out, the right one (a negative strength) pulls its pieces in.
4. **Custom map.** `4_custom_cracks.png`: a few large pieces cut by a custom map's colours.
5. **A ramp threshold.** `5_ramp_threshold.png`: only part of the layer breaks, held back by a
   gradient, the pieces tumbling down.
6. **Red sides, seen from the side.** `6_red_sides_seen_from_the_side.png`: thick pieces turning
   about their uprights, their sides red, seen at an angle by the camera.
7. **The composition camera.** `7_composition_camera.png`: the burst seen through the
   composition's camera.
8. **A point light.** `8_point_light_shine.png`: the pieces lit by a near light toward the upper
   left.
9. **In the app.** Add Shatter (Generate) to a layer and play from the start: the middle should
   burst into flying bricks that fall away. Change Pattern to Glass and Repetitions to 30.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how hard the pieces fly and fall as added, the shapes of
the patterns, and the staircase edges of pieces seen very close.

## Speed

Measured on a quiet machine (`verification/B-513_shatter_timing_table.md`). On the processor at the reference shot's size (1920 by 1080), a frame took about 88 to 96 ms with Shatter as added, 104 to 141 ms with glass 50 across and every piece freed, and 2.7 to 5.0 seconds with 1000 tiles across, the most allowed, against 34 to 40 ms with no effect. Shatter has no card path yet: expect a little under real time at full size with the usual patterns, and Draft quality or a smaller preview past that.
