# D-258: onion skin of one layer

The viewer's top row has a new **Onion** button (three rings), next to Compare. Select a layer and turn it on. Over the picture you then see:

- the layer two frames before and one frame before, in faint red;
- the layer one frame after and two frames after, in faint green.

The nearer frames are a little stronger. The ghosts follow you as you move the playhead or pick another layer. They are hidden while playing.

The ghosts are drawn by the page over the picture, the same way the grid and the safe areas are. They are never in the picture's pixels, a snapshot or an export. Nothing is soloed: the timeline's solo switches are left as they were.

## Pictures

- `D-258 pictures/onion_off_and_on.png`: frame 10 of the reference shot with layer2 selected. On the left, Onion is off. On the right, Onion is on: layer2's red dot at frames 8 and 9 in red, and at 11 and 12 in green. The frame was drawn on the CPU for this picture, because a frame drawn by the graphics card is not in a page screenshot.
- `D-258 pictures/onion_button.png`: the viewer's top row with Onion on (highlighted).

## The check, written first (`d258_one_layer_alone_is_that_layer_soloed` in `app/src/main.rs`, committed in b612662)

**On the build before the change** it did not build, because there was no `alone`.

The window draws each ghost with `/alone`, "this layer by itself, for this one picture". The check compares that with the frame the window really sends when the layer is soloed. All 6 checks pass in `D-258_alone_table.md`:

| Check | Result |
|---|---|
| Asking for layer2 alone solos nothing in the window, and adds no undo step | pass |
| The ordinary frame 10 afterwards is the ordinary frame 10 before, every byte | pass |
| layer2 alone at frame 10, Full, is the frame sent with layer2 soloed: 1920 by 1080, every byte the same | pass |
| And at Draft: 480 by 270, every byte the same | pass |
| It is not the whole picture | pass |
| A layer not in the composition is refused | pass |

## In the running app (`d258_check.js`)

| Step | What the page said |
|---|---|
| layer2 selected at frame 10, Onion still off | no ghosts, nothing asked |
| Onion clicked | ghosts shown, asked for layer2 at frames 8, 9, 11, 12 at Draft (the quality on screen); 6297 red and 5024 green pixels |
| Status line | "Onion skin: layer2, two frames each side." |
| Panels drawn again with nothing changed | nothing asked again |
| The window's solo list | empty |
| Playhead to frame 0 | asked only for frames 1 and 2 (there are no frames before 0) |
| Onion clicked again | ghosts hidden |
| Errors on the page | none |

The window's own settings, and its CPU / graphics card choice, were put back afterwards.

## Checks

- All 84 of the app's tests pass. `/alone` is listed among the window's own routes and in the not-found sentence, and `onionbtn` among the controls.
- No fixture, command ID, saved project byte or exported picture changes.

## Limits, stated

- **One layer, two frames each side.** The ghosts are of the selected layer only. With several layers selected, it is the one clicked last.
- **The ghosts are the layer as it renders**, with its effects, moves and matte. An effect that spreads across the whole frame (a glow, for example) makes a larger ghost.
- **Each new frame costs four extra drawings of that one layer**, asked only once the playhead rests. On a heavy layer, the ghosts can arrive a moment after the picture.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Select a moving layer, then click **Onion** (three rings, right of Compare) | Faint red copies of the layer where it was, faint green where it is going | |
| 2 | Step a frame forward with the arrow key | The ghosts move along with it | |
| 3 | Press Play | The ghosts disappear while playing, and come back when you stop | |
| 4 | Export a frame with Onion on | No ghosts in the exported picture | |

Anything marked ✗, tell me the row number.
