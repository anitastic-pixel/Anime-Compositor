# B-891 playtest: Color Grade's Hue vs Saturation curve you can draw (D-1011)

What it is: in the Lumetri tutorial (8:20) a colour is made stronger or weaker by clicking on the
Hue vs Saturation curve and dragging a point up or down. Color Grade only took typed points.
Now a graph sits above the **Hue vs Saturation points** box: the colours run along the bottom,
up is more saturation, down is less, and the dashed line is "no change".

## Steps

1. Open a project with a colourful picture and select its layer.
2. Effect > Color Grade. Scroll to **Hue vs Saturation points** and its graph.
3. Click on the flat line above the green part of the colour strip.
4. Drag that square down to the bottom.
5. Start dragging it up again and press Escape before letting go.
6. Double-click the square.
7. Click above red at the far left and drag up; click above blue and drag down.
8. Press Ctrl+Z twice.

## What you should see

- Step 3: a square appears on the line; the picture does not change.
- Step 4: the greens in the picture turn grey; the line dips smoothly around green. The box
  below shows "120 0" or near it.
- Step 5: the square jumps back to the bottom.
- Step 6: the square goes, the line is flat again and the greens come back.
- Step 7: reds get stronger and blues go dull, as in the picture below.
- Step 8: each Ctrl+Z undoes one drag.

## Pictures

- `verification/B-891 pictures/hue_curve_panel.png`: the graph in a private copy of the app
  after the reds were dragged up to 180 and the blues down to 20; the box reads "0 180, 240 20".

## Fixtures

- `verification/B-891_hue_curve_page_table.md`: 30 of 30. HUECURVE-001 to 024 run through the
  page's own code (the line drawn matches the picture's curve within 1e-6), then a press, a drag,
  Escape, the box, a double-click and two drags in a private copy of the app.
