# B-959 playtest: Update Markers From Source (D-1079)

After Effects' Layer > Markers > Update Markers From Source (Replaces All) copies a precomposition's markers onto the composition layer that shows it, so they can be seen on that layer's bar in the outer composition. The workflow tutorial nTpT2PELCbA uses it at about 2:20.

The checked answers are in `verification/B-959_markers_from_source_table.md`.

## In the app

1. Open a composition and add a few markers to it with `*` (Numpad *), for example on frames 0 and 12. Name one "hit".
2. Put that composition into another composition as a layer (drag it from the Project panel, or Pre-compose). Move the layer so it starts at frame 10.
3. Right-click the layer and choose **Update markers from source (replaces all)**.
   - Expected: the layer's bar now shows the same markers, 10 frames later than inside: on 10 and 22, with "hit" named.
4. Press Ctrl+Z.
   - Expected: the markers on the layer go back to what they were before, in one undo.
5. Set the layer's Time Stretch to 200% and choose it again.
   - Expected: the markers are twice as far apart: on 10 and 34.
6. Turn Time Remap on for the layer (Ctrl+Alt+T) and choose it again.
   - Expected: nothing changes, and the status line says Time Remap is on and to turn it off first.
7. Right-click a drawn or solid layer.
   - Expected: the item is greyed out there.
8. Find it in the command finder by typing "markers from source".
   - Expected: it is listed and does the same as the menu item.

Judge it correct if steps 3 to 8 match. Tell me which step looked wrong if any did.
