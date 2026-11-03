# B-847 playtest: U and UU with no layer selected (D-967)

What changed: U and UU now work with nothing selected. They then look at every layer, as in
After Effects. With layers selected they look only at those, as before.

## Steps

1. Open a composition where one layer has keyframes (say on Rotation) and others have none.
2. Click an empty part of the timeline so no layer is selected.
3. Press U.
4. Press U again.
5. Press U twice quickly (UU).
6. Press P.
7. Select a layer with no keyframes and press U.

## What to see

- Step 3: the keyed layer opens and shows its Rotation row; the other layers stay closed.
- Step 4: it closes again.
- Step 5: every layer opens the properties you have changed from a new layer's values.
- Step 6: nothing opens (P still needs a selected layer, as in After Effects).
- Step 7: nothing opens on the keyed layer; only the selected layer is looked at.

`B-847_private_app_table.md` has these checks run in a private copy of the app: 6 of 6 pass.

If any of these is not so, B-847 is wrong.
