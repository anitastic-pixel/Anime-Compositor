# B-24h: a point added to a path that moves, by hand

Built on 2026-09-22 out of the last two Known limits on `verification/B-24d_mask_playtest.md`,
at the owner's word ("also tackle said limits"). D-79 in document 14 is the rule as built; it
changes no file, no schema and no expected value in `Fixtures/`, because D-77 had already said
that adding a point to a keyed path was a command's problem and not a file's.

The generated halves are `verification/B-24h_path_point_table.md`, 38 of 38, which checks that
the curve does not move when a point is cut into it — against document 19's cubic written out a
second time, at two hundred places along every segment — and `verification/B-24h_panel_table.md`,
which checks what the window does with it. This sheet covers what neither can judge: whether a
click lands where the hand meant, and whether the shape looks unmoved afterwards.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the
copy. Nothing below changes the fixtures.

## What to check

1. **A point added.** Choose a layer, press **Q** and drag a rectangle across it. Press **G** for
   the pen and click on the middle of one of its four edges: a **fifth point** appears there, and
   the rectangle's shape does not change at all — the edge is still the same straight edge, just
   with a point on it now. The new point is lit, as a pressed point is. (The pen is where After
   Effects keeps Add Vertex and Delete Vertex, which is why it is the pen and not Alt and a click;
   D-80 on 2026-09-23 gave Alt its After Effects job, converting a point, and moved these two
   here.)
2. **A point taken off.** With the pen still chosen, click on that new point. It goes, and the
   shape is the rectangle again.
3. **On a curve.** Press **V**, then **Q** twice — or use the ellipse — and draw an ellipse. Press
   **G** and click somewhere on its curve. A point appears on the curve **with handles**, and the
   ellipse does not change shape. This is the thing worth looking hardest at: the outline before
   and after should be the same outline. Press **V** and drag the new point afterwards; the curve
   bends around it.
4. **On a path that moves.** Draw a mask, go to frame 0 and press its stopwatch. Go to frame 24
   and drag a point across the picture, which makes a second key. Now scrub to **frame 12**,
   between the two, and click an edge with the pen. Three things to look for:
   - the shape at frame 12 does not move when the point appears;
   - scrub to frame 0 and to frame 24: **neither key moved either**, and both now have the extra
     point;
   - scrub across the whole range: the movement between the keys is the movement it was.
5. **Undo.** One Ctrl+Z takes the point back off — off the base and off both keys together.
   Ctrl+Shift+Z puts it back.
6. **The history says what happened.** Open **Edit** and look at what Undo offers after each
   thing you do. It should now name the change rather than saying "Set mask of N points" every
   time: "Draw Mask 1" after drawing, "Move a point of Mask 1" after a drag, "Add a point to
   Mask 1" after step 1, "Start Mask 1's path moving" after the stopwatch, "Key Mask 1's path at
   frame 24" after a key, "Set Mask 1's feather to 6 px" after typing a feather, "Rename Mask 1
   to Face" after renaming it. Rename the mask and check the later entries use the new name.
7. **A shape layer too.** Make a shape layer, draw a shape on it, and click its path with the pen.
   The same thing happens, and the history says "Add a point to Shape 1".
8. **Refusals.** With the pen, click the empty picture away from the path: nothing is added to the
   mask — the pen starts a new path there instead, which is what it has always done, and Escape
   lets that go. Take a mask down to three points and click one of them with the pen: it is
   refused, because a mask of fewer than three points encloses nothing — the mask is kept and the
   diagnostics say so, which is what this build has always done.
9. **Without the mouse.** Draw a mask and press **Tab** until one of its points has the focus
   ring — the arrow keys move that point a pixel a press, ten with Shift, which is how you can
   tell you are on one. Press **+** (Shift and the `=` key): a point appears in the middle of the
   edge that leaves it, exactly as the pen on that edge would. Press **-** while a point has
   the ring: that point goes. Everything in steps 4 to 6 holds the same way — keys, undo and the
   name in the Edit menu.
10. **Saved and reopened.** Save a project with an added point on a keyed path, then **Open** it
   again. The point is there, on the base and on every key, the path still moves, and nothing is
   reported about it in the diagnostics.

## Known limits

- The place along the segment is chosen by the **nearest point on the drawn curve** to where you
  clicked. On a nearly straight segment that can land a pixel or so from where the hand meant;
  drag it afterwards if it matters.
- Adding a point does not move the shape. **Taking one off does**, and is meant to: the segment
  it sat on becomes one segment between its neighbours.
- A curved segment cut into two is drawn in slightly more, shorter pieces than it was before, so
  a few pixels along that edge can differ by one of the sixteen samples a pixel is measured with
  — measured at 5 pixels of 3072 in the table. You are not expected to be able to see it; it is
  written down because it is the one thing that does change.
- There is no way to add a point at a typed fraction along a segment, and no way to add several
  at once. ~~Several cannot be taken off at once either~~ — B-24i does that: choose them and press
  Delete.
- The gestures in steps 1, 2, 3, 4, 7 and 8 were **Alt and a click** when this sheet was first
  written, on 2026-09-22. D-80 moved them to the pen on 2026-09-23, before the sheet was played,
  so that Alt could mean what it means in After Effects. The keyboard route in step 9 never
  changed.

## What to answer

"works", or which step number did something else and what it did.

## Result

**Passed on 2026-09-23.** The owner played all ten steps on the build of 2026-09-23 and answered
"1 thru 5 pass" for this sheet and the four walked with it (B-24g, P-15, B-24i, B-24j), with
nothing reported beside the steps.
