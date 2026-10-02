# D-260: real pictures in the Project panel

The Project panel now shows small pictures instead of symbols:
- each composition's tile shows its frame;
- each drawings tile shows its first drawing.

This applies to the Pictures view's cards and to the top line about the chosen item. The composition on screen shows the frame at the playhead. When the playhead stops moving, its picture follows a moment later. Other compositions show their frame 0. Transparent parts show as a dark checkerboard.

Sound items and lookup files keep their symbols, ♪ and ◑. A symbol also shows until the picture arrives, and stays if no picture can be made.

## Pictures

- `D-260 pictures/composition_picture.png`: the top line and the composition card, each showing the reference shot's frame.
- `D-260 pictures/drawings_pictures.png`: the four drawings cards (layer1 to layer4), each showing its first drawing, with the counts on top as before.

## The check, written first (`d260_a_small_picture_is_its_full_picture_shrunk` in `app/src/main.rs`, committed in b6df33c)

**On the build before the change** it did not build, because there was no `thumbnail`.

To make the expected pictures, the check shrinks each full picture itself with its own loop. Each 12 by 12 block of pixels is averaged. It then compares every byte. All 5 checks pass in `D-260_thumbnails_table.md`:

| Check | Result |
|---|---|
| layer2's small picture is its first drawing, layer2_000.png, shrunk: 160 by 90, every byte the same | pass |
| The reference shot's small picture at frame 10 is its full frame 10, shrunk: 160 by 90, every byte the same | pass |
| That picture is not empty | pass |
| A name that is no item is refused | pass |
| Asking changes nothing: the same composition on screen, no undo step, the same frames ready | pass |

## In the running app (`d260_check.js`)

| Step | What the page said |
|---|---|
| Pictures view opened | all 5 tiles are pictures, 160 by 90: reference shot, layer1, layer2, layer3, layer4 |
| Panel drawn again with nothing changed | no new picture asked for (kept) |
| Playhead moved to frame 20 | one new picture asked for: the reference shot at frame 20 |
| Errors on the page | none |

The window's own settings were put back afterwards.

## Checks

- All 83 of the app's tests pass. `/thumb` is listed among the window's own routes and in the not-found sentence.
- No fixture, command ID, saved project byte or exported picture changes.

## Limits, stated

- **A picture is renewed when its own item changes.** For example, an edit to the composition or to the drawings' settings renews it. A composition's picture is *not* renewed when a composition nested inside it changes, or when a drawing file is replaced on the disk under the same name. Switching the panel's view, or reopening the project, brings those up to date.
- **While playing, the picture of the composition on screen stays still.** It catches up when playback stops.
- **Only the first drawing is shown for drawings.** It is not the drawing at the playhead.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | In the Project panel, click **Pictures** | Each composition and each drawings card shows a small picture, not a symbol | |
| 2 | Move the playhead to another frame and let go | Within a second, the on-screen composition's picture shows that frame (look at the top line too) | |
| 3 | Import a sound file | Its card keeps the ♪ symbol | |

Anything marked ✗, tell me the row number.
