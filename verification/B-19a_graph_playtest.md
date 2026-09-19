# B-19a: the graph editor's window gaps, by hand

Built on 2026-09-18, after the owner named B-19's scope ("all 10 please; along with adding
animation/keyframing stopwatches to effects as well"). This sheet is six of the ten: dragging a
key's value, the box around keys, typing a key, zoom and Fit of the values, numbered lines, and
snapping. The other four, and keyframed effect settings, need a decision first and come as B-19b
onwards.

The generated half is `verification/B-19a_panel_table.md`, 11 of 11. It sends what the graph
sends when a key is let go of, and checks that the key lands on its frame with its value, keeps
its ease, is one entry to undo, and that the curve the graph is given passes through it. This
sheet covers what a table cannot judge: the dragging, and what is drawn.

## Before you start

Open any project, or use **Save As...** and work on a copy. Choose a layer, put two or three
**Position** keys on it at different places, and two **Opacity** keys. Open the **Graph** tab
and pick Position in the list. Nothing below changes the fixtures.

## What to check

1. **Numbered lines.** Grey lines run across the graph at round values, each with its number in
   the left margin, in place of the two numbers that used to sit at the top and bottom.
2. **Drag a value.** Take hold of a key's dot on the green (Y) line and drag it straight up. The
   dot follows, the status line says the frame and the value, and on release the curve is drawn
   through the new place and the layer has moved in the viewer. Its frame has not changed.
3. **X and Y apart.** Drag the dot of the same key on the blue (X) line. Only X changes.
4. **Both ways at once.** Drag a dot diagonally: it changes frame and value together. Ctrl+Z
   once puts both back.
5. **Shift.** Hold Shift while dragging: the key goes only the way it has gone furthest, along
   or up and down.
6. **Snapping in time.** Zoom the timeline out so frames are narrow. Put the playhead on a frame
   with no key and drag a key towards it: the key jumps to the playhead when it is close. It
   does the same at the frame of an Opacity key. Hold Ctrl and it passes them freely.
7. **Snapping in value.** Drag a key's dot up or down towards the height of another key on the
   same line: it settles level with it. Ctrl lets it past.
8. **An eased key keeps its ease.** Press F9 on a key, then drag its value. The handles are
   still there afterwards.
9. **The box.** Press on an empty part of the graph and drag: a blue box is drawn, and on
   release every dot inside it is chosen. Shift or Ctrl with it adds to what was chosen. Drag
   one of the chosen dots: they all go together, in time and in value.
10. **A click on empty graph** still puts the playhead there, and now also un-chooses the keys.
    To drag the playhead, drag in the strip of frame numbers along the top of the graph.
11. **Typing.** Double-click a dot, or right-click it and choose **Key value and frame...**. A
    small box shows Frame and the value (X and Y for position; opacity as a percentage). Type a
    new value and press Enter: the key has it. Type a new frame: the key moves. Ctrl+Z once
    undoes either, or both if both were typed.
12. **Zooming the values.** Roll the mouse wheel over the graph: the values zoom about the
    pointer and the numbered lines follow. Alt with the wheel still zooms time, and Shift with
    it still slides along time.
13. **Sliding the values.** Press the middle mouse button on the graph and drag up or down.
14. **Fit.** With two keys chosen, press **Fit** above the timeline: the graph closes in on
    those keys, in value and in time. With no key chosen, **Fit** goes back to the whole curve's
    height.
15. **The camera.** Click Camera on the timeline with keys on its Place: steps 2, 9 and 11 work
    on the camera's keys as well.
16. **A locked layer.** Lock the layer: its dots do not move.

## Known limits

- While a dot is dragged the line waits, and is drawn through the key on release. The handles of
  a curve do follow while they are pulled, as before.
- A key cannot be put on a frame where the same property already has a key; the move is refused
  and nothing changes. Snapping in time therefore ignores the same property's keys.
- The box chooses the dots it surrounds, so on Position it takes a key when either its X dot or
  its Y dot is inside.
- The key dialog changes one key: the one double-clicked, or the first chosen.
- The speed graph has no key dots, so it has the lines, the zoom and Fit but nothing to drag
  beyond its handles.
- The numbers on the lines are the file's own, so opacity reads 0 to 1 there, as the dots'
  hover text always has.
- The graph is still one property at a time and 170 pixels tall; several properties at once is
  B-19d.

## What to answer

"works", or which step number did something else and what it did.
