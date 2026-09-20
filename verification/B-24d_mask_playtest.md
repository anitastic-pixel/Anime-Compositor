# B-24d: a mask's path set moving, by hand

Built on 2026-09-20 against D-77, which the owner accepted on 2026-09-19 and which already said
how a path moves: point by point, the point and both handles, between keys that hold the same
points. Nothing new was decided for this; only built.

The generated halves are `verification/B-24d_mask_key_table.md`, 39 of 39, which checks the
shape between two keys pixel by pixel against `Fixtures/masks` — five cases, five frames each,
drawn in `verification/B-24d keys/mask_key_cases.png` — and
`verification/B-24d_panel_table.md`, 9 of 9, which checks what the window does with it. This
sheet covers what neither can judge: whether the stopwatch, the marks and a shape dragged frame
by frame behave the way a person expects.

## Before you start

Open any project with at least one drawing layer, or use **Save As...** first and work on the
copy. Nothing below changes the fixtures.

## What to check

1. **The stopwatch.** Choose a layer, press **Q** and drag a rectangle across it. On the
   timeline the mask's row now has a **hollow diamond** beside its name, where every other
   stopwatch in this window is. Go to frame 0 and press it: the diamond fills and **one mark**
   appears on the mask's row, under the playhead.
2. **A second key.** Go to frame 24. The diamond is hollow again, because this frame has no key.
   Drag one of the mask's points across the picture and let go. A **second mark** appears at
   frame 24 and the diamond fills — a point moved on a path that is already moving sets a key
   here rather than moving the whole shape.
3. **It moves.** Drag the playhead from frame 0 to frame 24 slowly. The mask's outline travels
   between the two shapes, and the picture is re-cut as it goes. Stop half way: the shape is
   half way between them, and the diamond is hollow because that frame has no key of its own.
4. **Every point, and its handles.** Draw an ellipse instead, set a key, go forward, and drag
   one of its **handles** out. Scrub back: the curve grows out of the shape gradually. A handle
   travels with the point it belongs to, so nothing swings across the picture on its way.
5. **Undo.** One Ctrl+Z after a drag takes back that whole drag, not a piece of it; one Ctrl+Z
   after pressing the diamond takes back the key. Ctrl+Shift+Z puts each back.
6. **Going to a key.** Press one of the marks on the mask's row: the playhead goes to that
   frame and the diamond fills.
7. **The stopwatch off.** Press the filled diamond at frame 24: that mark goes and the path is
   left with one key. Press it again at frame 0: the last mark goes, the row has no marks, and
   **the mask stays exactly where it was on screen** — it does not jump back to the shape it had
   before you started keying.
8. **Saved and reopened.** With two keys on a path, save, then **Open** the same file. The marks
   are where they were, the shape still moves between them, and nothing is reported in the
   diagnostics. A file with a keyed path used to open with a warning that the build could not
   animate it; that warning is gone because it is no longer true.
9. **Export.** Export the range as PNG. The exported frames are cut the way the viewer showed
   them, frame by frame, not all cut the same way.
10. **More than one mask.** Draw a second mask and key only the first. The second one stands
    still, has no marks, and its diamond stays hollow. Both cut the picture at once.

## Known limits

- A mask's key cannot be **dragged** along its row, eased with **F9**, or opened in the graph
  editor. Those take a property's keys and a path's key is not one of them. To change when a
  shape arrives, take the key off at one frame and set it at another.
- A key is always **linear**. A file that holds an eased path key is read, kept, saved and drawn
  eased — the core does all three, and `FX-MSK-033` proves it — but the window has no way to set
  one yet.
- Adding or removing a **point** on a path that has keys is refused, because a path's points are
  the path's and every key holds them all. Draw the shape you want first and key it afterwards.
- One Undo entry per change still reads "Set mask of N points", whatever the change was: the
  core takes the whole list of masks as one command.

## What to answer

"works", or which step number did something else and what it did.
