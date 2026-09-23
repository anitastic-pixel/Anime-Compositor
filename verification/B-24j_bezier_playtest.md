# B-24j: curving a mask's path by hand

Built on 2026-09-23 from the owner's word at the B-24f playtest ("I would like to be able to do
bezier curves with these when I click and drag to modify curve amount, etc. make it 1 to 1 with
AE's mask ecosystem"). D-80 in document 14 is the rule as built.

It changes no file and no expected value in `Fixtures/`. A point's two handles have been in D-77's
record since the first day — `in` and `out`, offsets in pixels — and every gesture below writes
points and handles through `mask.set_path` and `shape.set_path`, which have existed since B-24c.
Whether a point is **smooth** is not written down anywhere: a point is smooth when both its handles
are out and point opposite ways along one line, which is exactly what makes the curve run through
it without a corner, and the window reads that off the handles each time it needs to know.

There is no generated half. This is the page's own arithmetic and **no test in this repository can
run the page's code**; this sheet is its only check.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the copy.
Nothing below changes the fixtures.

## What to check

1. **The pen draws curves now.** Press **G** for the pen. Place a point with a click. For the next
   one, press **and keep holding** the button, and drag: a pair of handles comes out of the point
   you are placing, one either side, and the path curves as you pull. Let go, place a third point
   the same way, then click the first point to close the mask. What you get is a curved shape, not
   a polygon.
2. **A corner curved by hand.** Press **Q** and drag a rectangle; press **V**. Hold **Alt**, press
   on one of its corners and drag: a pair of handles comes out of that corner, mirrored, and the
   two edges either side of it bend. Let go. That is After Effects' Convert Vertex tool.
3. **Smooth stays smooth.** Drag one of the two handles you just made. The **other handle turns
   with it**, staying opposite, and keeps its own length. The curve runs through the point with no
   kink in it, however you swing the handle.
4. **Breaking a pair.** Hold **Alt** and drag one of those two handles. Only that one moves; the
   other stays where it was, so the two sides of the point now point different ways and the path
   has a kink at it. Let go and drag that handle again **without** Alt: it still moves alone,
   because the point is no longer smooth. That is what After Effects does after a break.
5. **Straightening.** Hold **Alt** and click a point that has handles: they go, and the two
   segments either side become the straight lines they are with no handles at all. Alt and a drag
   out of it curves it again.
6. **Alt no longer adds or removes points.** On the selection tool, Alt and a click on a point
   converts it, and adding and deleting points are the pen's — over an edge it adds, over a point
   it takes off (`verification/B-24h_path_point_playtest.md`, steps 1 and 2). Check that both
   still work, because this unit moved them.
7. **The history says what happened.** Open **Edit** after each of the above: a handle pulled says
   "Move a point of Mask 1" — a handle belongs to its point — and a point converted says the same.
   Each gesture is one entry; one Ctrl+Z puts the path back as it was.
8. **On a path that moves.** Give a mask a stopwatch, make a second key, then scrub between the two
   and pull a handle. It writes to the key under the playhead, as moving a point does, and the
   shape between the keys bends the way you pulled.
9. **Without the mouse.** Press **Tab** until a point has the focus ring. Press **]** a few times:
   a pair of handles grows out of that point along the line between its two neighbours, and the
   path curves more with each press. Press **[** to shorten them again; shortened to nothing the
   point is the corner it was. This is the only way to curve a path without a hand, and it is why
   `verification/B-12c_keyboard_table.md` can still say every gesture has a keyboard twin.
10. **On a shape layer.** All of the above on a shape layer's shape, including an open path drawn
    with the pen and left open with Enter.
11. **Saved and reopened.** Save a project with curved points in it and **Open** it again. The
    curves are where you left them, smooth points are still smooth — drag a handle and its partner
    still follows — and nothing is reported in the diagnostics.

## Known limits

- Smooth is read from the handles, not stored. A point whose two handles happen to line up
  opposite each other is treated as smooth even if it was made by breaking a pair and swinging
  them back into line by hand. In practice that is the same point, but it is a guess the window
  makes and not a fact the file holds.
- Handles are pulled with a hand. **]** and **[** are the keyboard's way in, and they only ever
  make a **mirrored** pair along the line between the point's neighbours: there is no keyboard way
  to break a pair, to swing one handle alone, or to aim a handle anywhere else.
- A handle can be dragged only once it exists. On a point with no handles there is nothing on the
  picture to grab, which is why curving a corner is Alt and a drag on the **point**.
- The pen's press-and-drag pulls a **mirrored** pair, both handles the same length. After Effects'
  pen can break the pair while drawing, with Alt before letting go; this one cannot. Convert the
  point afterwards instead.
- There is no RotoBezier, no Mask Feather tool, no Mask Interpolation and no free transform box
  around a mask. Each is named in D-80 as deferred, and the box is B-24k in document 15.

## What to answer

"works", or which step number did something else and what it did.

## Result

**Passed on 2026-09-23.** The owner played all eleven steps on the build of 2026-09-23 and
answered "1 thru 5 pass" for this sheet and the four walked with it (B-24g, P-15, B-24h, B-24i),
with nothing reported beside the steps.
