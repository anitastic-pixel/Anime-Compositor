# D-311 / B-194: a composition's background colour

Found by P-26, tutorial 1 (Shockwave), which sets After Effects' composition background colour. Here a composition had none. This also builds the Background row of D-275; that proposal's other three rows (pixel shape, resolution, start frame) stay proposed.

## What changed

- The New composition and Composition settings window (Ctrl+K) has a **Background** row: a Colour box and a colour. Unticked means no colour, as before.
- With the checkerboard off, the viewer shows the colour behind the picture instead of its dark grey. With the checkerboard on, you see the checkerboard, as before.
- An **MP4**, which cannot be see-through, is laid over the colour instead of over black.
- A PNG, an EXR, an animated PNG, a GIF and a composition placed inside another stay see-through, as in After Effects. The frame itself does not change.
- It is saved in the project only when set. Setting it is one step to undo.

## Checks (cargo test)

`tests/b194_comp_background.rs`, 3 of 3 pass:

| Check | Expected | Got |
|---|---|---|
| An old project | has no colour and saves none | so |
| Set to 0.5, 0.25, 1 | written, read back the same, and undo takes it away | so |
| A channel past 1 or below 0, set by the window | refused | refused |
| In a file: two numbers, a 2, or a word | the file is refused with a clear message | refused |
| An MP4 pixel: opaque, half see-through, clear | stays; mixed half and half with the colour; becomes the colour | so |
| An MP4 over a black background | exactly the MP4 as before | so |

Unchanged and still passing: the fixture round trip that saves all 2297 fixture projects byte for byte (`tests/d253_d254_roundtrip.rs`), the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

An orange card on a 640 x 360 composition, its background set to navy.

| Picture | Look for | Pass? |
|---|---|---|
| `D-311 pictures/1_navy_background_checkerboard_off.png` | navy all round the card | pass |
| `D-311 pictures/2_checkerboard_on.png` | the checkerboard instead of the navy | pass |
| `D-311 pictures/3_settings_window.png` | the window's Background row, Colour ticked, the navy swatch | pass |
| `D-311 pictures/4_after_undo_none.png` | after one undo, the dark grey of no colour | pass |

## For the owner to try

1. Press Ctrl+K. In the Background row tick **Colour** and pick a colour. Apply.
2. Turn the checkerboard off (Alt+G): the colour is behind your picture.
3. Export an MP4: its empty parts are that colour. A PNG's stay see-through.
