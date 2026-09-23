# B-24i: several mask points chosen at once, by hand

Built on 2026-09-23 from the owner's word at the B-24f playtest ("I want to multi-select mask
points, and be able to drag select as well"). D-80 in document 14 is the rule as built. It changes
no file, no schema and no expected value in `Fixtures/`: which points are chosen is the window's
own state, saved nowhere, exactly as a chosen layer is, and every move below is sent as the whole
path through `mask.set_path`, which has existed since B-24c.

The generated half is `verification/B-24h_panel_table.md`, which now walks several points taken off
in one request, the one entry in the history it makes, the Undo that brings all of them back, and
the refusal of a list naming a point the path has not got. Everything else here is the page's own
arithmetic, and **no test in this repository can run the page's code** — this sheet is its only
check.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the copy.
Nothing below changes the fixtures. Draw a mask with the rectangle (**Q**, then drag) and add a few
points to it with the pen (**G**, then click on an edge) so that there are six or so to play with.
Press **V** for the selection tool before each step below.

## What to check

1. **One point still behaves.** Click a point: it lights, and it alone. Drag it: it moves and the
   shape follows, exactly as before this unit.
2. **Shift adds and takes away.** Click one point, then hold **Shift** and click two more. All
   three are lit. Shift and click one of the three again: it goes dark and the other two stay lit.
   Nothing moved while you were choosing.
3. **A box catches what it is drawn around.** Start a drag on a part of the picture with **no layer
   under the pointer** — the dark around the frame always works — and drag a box across two or
   three of the mask's points. On release exactly the points inside the box are lit. Hold
   **Shift** and draw a second box somewhere else: those points are added to the ones already lit.
4. **The box still selects layers when no mask is being worked on.** Click an empty part of the
   picture until nothing is lit, then drag a box across two layers: they are selected, as they
   always were. This is the half of the gesture that must not have broken.
5. **They move together.** With three points lit, drag any one of them. All three travel by the
   same amount and the rest of the shape stays where it is. Hold **Shift** while dragging: the
   move is held to one direction, across or up and down.
6. **One entry in the history.** After that drag, open **Edit**: Undo offers "Move 3 points of
   Mask 1" — one entry for the whole drag, not three. Ctrl+Z puts all three back at once.
7. **The whole path at once.** Click the mask's **outline**, away from any point: every point of it
   lights. Drag from there and the whole mask travels. Undo says "Move Mask 1's path".
8. **The arrow keys move the set.** With three points lit, press **Tab** until one of them has the
   focus ring, then press an arrow key: all three move a pixel, and ten pixels with **Shift**.
   Hold the key down: the steps keep up and none is dropped.
9. **Delete takes them all off.** With three points lit, press **Delete**. All three come off in
   one go; the history says "Remove 3 points from Mask 1" and one Ctrl+Z brings all three back.
   With no points lit, Delete still deletes the **layer**, as it always did — check that too.
10. **Escape lets go.** With points lit, press **Escape**: the points go dark and the layer stays
    selected. Press **Escape** again: the layer is deselected, as it always was.
11. **On a path that moves.** Give a mask a stopwatch, make a second key, scrub between the two and
    drag three points together. It writes to the key under the playhead, as one point always did,
    and one Ctrl+Z takes the whole move back.
12. **On a shape layer.** The same on a shape layer's shape: choosing, boxing, dragging, Delete.
    The history says "Shape 1" instead of "Mask 1".

## Known limits

- The box chooses points only while a mask's points are showing, and it has to **start** where
  there is no layer under the pointer. Over a layer a drag still moves the layer, which is what
  After Effects does; the dark around the picture is always somewhere safe to start.
- A point is caught by the box when the **point itself** is inside it. A handle sticking out of the
  box does not put its point in, and a point just outside a box whose edge crosses the path is not
  caught.
- Points are chosen on **one** mask at a time — the one being worked on. There is no way to choose
  points on two masks at once, as there is none in After Effects either.
- The choice is not saved. Opening a project, changing the frame or choosing another layer starts
  with nothing chosen.
- Delete on a set that would take the mask below three points is refused whole: none of them comes
  off, and the sentence says how many the path has. That is D-77's rule and not new here.

## What to answer

"works", or which step number did something else and what it did.

## Result

**Passed on 2026-09-23.** The owner played all twelve steps on the build of 2026-09-23 and
answered "1 thru 5 pass" for this sheet and the four walked with it (B-24g, P-15, B-24h, B-24j),
with nothing reported beside the steps.
