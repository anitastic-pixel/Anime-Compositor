# B-850 playtest: Alt-drag a project item onto a layer to replace it (D-970)

What changed: dragging a picture or video from the Project panel onto a layer row while holding
Alt swaps that layer's source, as in After Effects. The layer keeps its place, timing, position,
masks and effects.

## Steps

1. Open a composition with two different pictures in the Project panel, one of them on a layer.
2. Hold Alt and drag the other picture from the Project panel over that layer's row in the
   timeline. Let go there.
3. Press Ctrl+Z.
4. Do the same drag without Alt.
5. Press Ctrl+Z.

## What to see

- Step 2: while you hover, the layer row has a blue outline. After the drop the layer shows the
  new picture, with the same name, place and effects. No new layer appears.
- Step 3: the old picture is back in one undo.
- Step 4: a new layer is added, as before; the first layer keeps its picture.
- Step 5: the new layer is gone.

`B-850_private_app_table.md` has these checks run in a private copy of the app: 9 of 9 pass.

If any of these is not so, B-850 is wrong.
