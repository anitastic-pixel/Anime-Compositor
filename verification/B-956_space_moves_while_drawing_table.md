# B-956: Space moves a shape or mask while drawing it (D-1076)

Written by `b956_space_moves_while_drawing` in `app/src/tests/w12x.rs`.

After Effects' Space held while dragging out a shape (4GeGLgK2BBk, about 3:00). `spaceMove` is the page's own code run under Node, in layer pixels; the other rows read the page.

**8 of 8 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| dragged from (10, 10) to (50, 40): the shape is sized, its corner stays | 10, 10 to 50, 40 | 10, 10 to 50, 40 | pass |
| Space held, on to (70, 60): the whole shape moves by (20, 20), its size kept | 30, 30 to 70, 60 | 30, 30 to 70, 60 | pass |
| Space let go, on to (80, 80): sized again from where it was moved to | 30, 30 to 80, 80 | 30, 30 to 80, 80 | pass |
| Space already held at the first step: nothing to move by yet | 5, 5 | 5, 5 | pass |
| Space during the drag is the drag's, so it does not play | true | true | pass |
| every step of the drag goes through it | true | true | pass |
| the shape is drawn from where it was moved to | true | true | pass |
| the keys are let go of when the drag ends | true | true | pass |
