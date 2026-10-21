# B-609 playtest: parametric shapes (D-738 (a))

## What to try

1. Press **Q** a few times. The shape tool steps through Rectangle, Rounded Rectangle, Ellipse, Polygon and Star, then back to Rectangle. The status bar names each one.
2. With a shape layer selected, drag on the viewer. A rectangle or ellipse is drawn corner to corner; a polygon or star is drawn from its middle outwards (the drag sets its size and which way it points). The tool goes back to the selection tool afterwards.
3. With nothing selected, drag with a shape tool. A new shape layer is made for it.
4. With a solid, picture or other layer selected, drag with a shape tool. It makes a mask of that shape, as in After Effects.
5. Open Effect Controls on the shape layer. Each drawn shape lists its settings (Size, Position, Roundness for a rectangle; Points, Rotation, Inner/Outer Radius and Roundness for a star). Click a stopwatch to key one and change it at another frame; the shape animates between them.
6. On a star or polygon, the **Star / Polygon** switch turns one into the other.
7. **Convert to Bezier Path** turns the shape into ordinary points you can move one by one (Ctrl+Z undoes it). Until then its points can't be dragged one by one; the app says so in words.

## What the check found (private copy, `Fixtures/parametric_shapes/host.json`): 21 of 21 pass

- The five shapes open as rectangle, rounded rectangle, ellipse, star and polygon.
- Effect Controls shows each shape's settings, a Convert to Bezier Path button for each of the five, the Star/Polygon switch on the star and the hex, and no Edit points button.
- The box's size keyed 120 x 80 at frame 0 and 40 x 200 at frame 24 is 80 x 140 at frame 12. The timeline shows the row as "Box size".
- Star to polygon and back keeps the star's inner settings.
- Moving a hex point on its own is refused: "Hex" is a polygon, drawn from its settings, so its points cannot be moved one by one. Key its settings, or Convert to Bezier Path first.
- Convert to Bezier Path gives the hex six plain points; one undo makes it a polygon again.
- Q steps rectangle, rounded, ellipse, polygon, star, rectangle.
- A star dragged 30 px up from (60, 250): middle (60, 250), outer radius 30, inner 15, 5 points.
- A rounded rectangle dragged from (320, 240) to (390, 290): middle (355, 265), 70 x 50, roundness 16.
- An ellipse with no layer chosen made a new shape layer: middle (200, 40), 100 x 40.
- A polygon dragged on a solid masked it with five points, the top one at (200, 60).
- No page errors.

Fixture check `tests/b609_parametric_shapes.rs`: 34 of 34 (see `B-609_parametric_shapes_table.md`). Each outline matches `tools/parametric_shape_reference.py` within 1e-9 and is drawn with the same pixels as a plain shape of that outline.

## Pictures (`B-609 pictures/`)

- `all.png`: every picture below on one sheet.
- `00_fixture_f00.png`, `00_fixture_f12.png`, `00_fixture_f24.png`: the five fixture shapes at frames 0, 12 and 24 (the rounded rectangle is keyed in the fixture).
- `01_box_size_keyed_f12.png`: the box halfway through its keyed size.
- `02_star_as_polygon.png`: the star switched to a polygon.
- `03_hex_as_bezier.png`: the hex after Convert to Bezier Path (looks the same, now plain points).
- `04_drawn_star_and_rounded.png`: the star and rounded rectangle drawn by dragging.
- `05_new_layer_ellipse.png`: the ellipse that made its own layer.
- `06_polygon_mask_on_solid.png`: a blue solid cut to a pentagon by the polygon tool.

## How this differs from After Effects

- The Rectangle and Rounded Rectangle tools both make a rectangle; Rounded just starts with roundness 16.
- A drag can't be held with Shift for a perfect square or circle yet.
- Shape groups, Repeater, Merge Paths and the other shape operators are D-738 (b) to (d), still proposals.
