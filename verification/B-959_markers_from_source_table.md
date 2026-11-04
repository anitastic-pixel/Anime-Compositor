# B-959: Update Markers From Source (D-1079)

Written by `b959_markers_from_source` in `app/src/tests/w12x.rs`.

After Effects' Layer > Markers > Update Markers From Source (Replaces All), from the workflow tutorial nTpT2PELCbA at about 2:20. Five composition layers, each in at 10 with one marker of its own on 30, show an inner composition whose markers are "start" on 0 and "hit" on 12 lasting 6 (one shows an inner composition with none). The expected answers are D-1079's worked examples.

**14 of 14 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| in at 10, starting on inner frame 0, 100%: the layer's markers (its own "30 mine" replaced) | 10 start, 22 hit (6 long) | 10 start, 22 hit (6 long) | pass |
| in at 10, starting on inner frame 0, 100%: entries to undo | 1 | 1 | pass |
| the same at 200% stretch: the layer's markers (its own "30 mine" replaced) | 10 start, 34 hit (12 long) | 10 start, 34 hit (12 long) | pass |
| the same at 200% stretch: entries to undo | 1 | 1 | pass |
| in at 10, starting on inner frame 4: the layer's markers (its own "30 mine" replaced) | 6 start, 18 hit (6 long) | 6 start, 18 hit (6 long) | pass |
| in at 10, starting on inner frame 4: entries to undo | 1 | 1 | pass |
| a source with no markers clears the layer's: the layer's markers (its own "30 mine" replaced) |  |  | pass |
| a source with no markers clears the layer's: entries to undo | 1 | 1 | pass |
| undo after the last puts the layer's own marker back | 30 mine | 30 mine | pass |
| refused, Time Remap on: its markers stay | 30 mine, answered, 0 | 30 mine, answered, 0 | pass |
| refused, a layer that is not a composition layer: nothing changes and the window answers | answered, 0 | answered, 0 | pass |
| the page wires it in: the right-click menu, on a composition layer | true | true | pass |
| the page wires it in: the command finder | true | true | pass |
| the page wires it in: what it sends | true | true | pass |
