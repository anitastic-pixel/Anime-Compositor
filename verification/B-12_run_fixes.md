# B-12 run fixes, 2026-09-15

Three changes asked for during the owner's acceptance run. Each can be checked in the window.

## One picture imported on its own is a still

1. Import drawings (Ctrl+I) and choose one PNG only, for example `verification/B-05a_reference_frame.png`.
2. The drawings list shows it with the word **still**, not "1 frames", and under its own file name.
3. Select it and press Add layer. The picture shows on frame 0, frame 100 and the last frame: it holds across the whole bar.
4. Importing a whole sequence still works: choose every file in `Fixtures/reference_shot/layer3`; it shows as 11 frames with drawing 7 missing.
5. A project saved with the still in it reopens with the still.

## Forward and Back move every chosen layer

1. Ctrl-click layer 1 and layer 2 so both rows light up. Press Forward (or Ctrl+]): both go up one row, in the same order.
2. One Ctrl+Z puts both back.
3. With the top layer and one other chosen, Forward moves only the other, until it reaches the row under the top one.
4. One layer chosen: Forward and Back work as before.

## Export progress

1. Export with Write frames whose drawing is missing ticked.
2. Beside Cancel export a bar fills as frames are written, with "N of 240 frames, about M s left". At the very start it says it is checking the drawings first.
3. When it finishes, the bar and the words go away and the usual Exported sentence stays.
4. Cancel export part way: the bar goes away and the sentence says how many frames finished.

## What checks it by machine

`one_file_imported_on_its_own_is_a_still` in the app's tests. The bar and the moves are page behaviour and are checked above.

The owner may cut any of the three.
