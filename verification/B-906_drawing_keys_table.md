# B-906: Shift and Ctrl while drawing a rectangle or an ellipse (D-1027)

Written by `b906_drawing_keys` in `app/src/tests/w12v.rs`.

After Effects: Shift while dragging a Rectangle or Ellipse tool keeps it a square or a circle; Ctrl draws it out from its centre. The rows below run the page's own drag arithmetic; the numbers are the two corners drawn between, x0, y0, x1, y1, in the layer's pixels.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| No key: corner to corner, 60 by 30 | [100,100,160,130] | [100,100,160,130] | pass |
| Shift: a square, the longer side (60) winning | [100,100,160,160] | [100,100,160,160] | pass |
| Shift dragged up-left and down: still a square, 80 a side, the way the pointer went | [100,100,20,180] | [100,100,20,180] | pass |
| Ctrl: out from the press, 120 by 60 round (100, 100) | [40,70,160,130] | [40,70,160,130] | pass |
| Ctrl and Shift: a square 120 a side round (100, 100) | [40,40,160,160] | [40,40,160,160] | pass |
| Ctrl and Shift dragged up-left: the same square size, 120 a side round (100, 100) | [160,160,40,40] | [160,160,40,40] | pass |
| The page holds: the rectangle, rounded rectangle and ellipse drag read the keys | true | true | pass |
| The page holds: the shape's settings come from those corners | true | true | pass |
| The page holds: the mask's outline comes from those corners | true | true | pass |
