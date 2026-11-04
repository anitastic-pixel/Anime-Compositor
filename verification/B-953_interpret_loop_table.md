# B-953: Interpret Footage Loop (D-1073)

Written by `b953_interpret_loop` in `app/src/tests/w12x.rs`.

After Effects' Interpret Footage > Loop (nTpT2PELCbA, about 1:30). The requests go to the window's own command handler on the reference shot, with its longest sequence; the frames are drawn by the renderer. The picture is `verification/B-953 pictures/loop.png`.

**24 of 24 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the reference shot's longest sequence (24 drawings), before: Loop | 1 | 1 | pass |
| after asset.set_loop times=3: Loop | 3 | 3 | pass |
| entries to undo | 1 | 1 | pass |
| the saved project: the sequence's "loop" | 3 | 3 | pass |
| the saved project: other assets with a "loop" written | 0 | 0 | pass |
| saved and opened again: Loop | Some(3) | Some(3) | pass |
| a file saying "loop": 0 is refused | true | true | pass |
| refused, 0 times: Loop stays 3 and the window answers | 3, answered | 3, answered | pass |
| refused, 100 times: Loop stays 3 and the window answers | 3, answered | 3, answered | pass |
| refused, not a number: Loop stays 3 and the window answers | 3, answered | 3, answered | pass |
| refused, an asset not in the project: Loop stays 3 and the window answers | 3, answered | 3, answered | pass |
| entries to undo after the refusals | 1 | 1 | pass |
| New Comp from Selection: its length in frames, 3 times the drawings | 72 | 72 | pass |
| the new layer's sheet: one frame per drawing, the drawings 3 times over | true | true | pass |
| the sheet's length in frames | 72 | 72 | pass |
| frames 0, 24 and 48: the most any pixel differs, 0 to 1 | 0 | 0 | pass |
| frames 12, 36 and 60: the most any pixel differs, 0 to 1 | 0 | 0 | pass |
| frames 23, 47 and 71: the most any pixel differs, 0 to 1 | 0 | 0 | pass |
| the first and last drawing differ (so the rows above are a real check) | true | true | pass |
| loop.png: three rows, one per pass, each the first, middle and last drawing | drawn | drawn | pass |
| two Ctrl+Z: Loop back to | 1 | 1 | pass |
| the Project panel's line for a sequence has Loop | true | true | pass |
| only for a sequence | true | true | pass |
| a change sends asset.set_loop | true | true | pass |
