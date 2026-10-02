# W-41b: turn, mirror, snapshot and compare (D-248; D-250 proposed)

The third screen of the redesign, second part. The viewer's row gains the Sandbox's Viewer A tools for looking at a drawing differently:

- **turn the view** left or right, 15° a click, as an animator turns the paper;
- **mirror** it, to see a drawing's mistakes fresh;
- a readout saying how it is turned, with **Reset**;
- a **snapshot** of the picture, and **compare**, which shows the snapshot beside the picture now with a split you drag.

All of these are on screen only. Nothing goes into the picture's pixels, the project file or an export.

## Pictures

| | Picture |
|---|---|
| The Sandbox board it copies | `W-38 pictures/sandbox_compose.png` |
| The real window, 100% / 150% / 200% | `W-41b pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |
| Turned and comparing | `W-41b pictures/turned_and_compare.png` |
| Grid and field guide (their boxes fixed, below) | `W-41b pictures/grid_and_field_guide.png` |

In the row, after the field-guide button: turn left ↺, turn right ↻, mirror ⇋, then snapshot 📷 and compare (greyed until there is a snapshot), then the slate and strip buttons from W-41a. When the view is turned, a readout appears, for example "15° left, mirrored · on screen only, Export is not changed", with a Reset button.

## What it does, checked in the running app (`w41b_view.js`)

| Step | What the page said |
|---|---|
| Opened | the card is painting the picture |
| Snapshot | compare became clickable; the card paints again afterwards |
| Moved to frame 120, Compare | the compare shown at the picture's size (1219.56 px), its tag "Snapshot, frame 0" at the left and "Now" at the right |
| Compare off; turn right twice; mirror | readout "30° right, mirrored · on screen only, Export is not changed"; the card stops painting and the page turns the picture |
| A press on the turned picture | "The view is turned or mirrored, so the picture is not edited by hand. Reset turns it back." |
| Reset | the readout hides; the card paints again |
| Errors on the page | none |

## Fixed on the way

- **The field guide's boxes.** W-41a placed the 12F/10F/8F boxes and safe areas with styles written in the page, which the window's security rule throws away, so they all sat on the picture's edge. They are now set another way. Checked: the 8F box is 813 by 457 px on a 1220 px picture, two-thirds of it, as it should be.
- **The guides are easier to see.** Each line has a thin dark edge, so it shows on white paper and on black.

## Limits, stated

- **On screen only.** Turn and mirror never change the picture, the project or an export. The readout says so.
- **No edits by hand while turned.** Dragging a layer on a turned picture would move it the wrong way, so a press says so instead. The middle button still pans. Reset first.
- **The snapshot is kept until the window closes.** It is not saved anywhere.
- **The snapshot's status message is short-lived.** The next frame's own line replaces it within a moment.
- **A turned view is drawn by the page, not the card.** It can be a little slower to follow while playing. The card goes back to painting as soon as the view is reset.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| ↺ / ↻ | nothing is sent to the project | the picture turns 15° each click; the readout shows |
| ⇋ | nothing is sent to the project | the picture flips left to right |
| Reset (in the readout) | nothing is sent to the project | the picture is straight again; the readout hides |
| 📷 | the empty place (D-250), once | compare becomes clickable |
| compare | nothing is sent | the snapshot on the left, the picture now on the right; drag the knob to move the split |

## Keyboard reach

- **Every new button is a button Tab stops at.** Each has a name a screen reader says.
- **All six are in the View menu and in Find any command (Ctrl+Shift+P):**
  - "Turn the view left"
  - "Turn the view right"
  - "Mirror the view"
  - "Turn the view back"
  - "Snapshot of the picture"
  - "Compare with the snapshot"
- **No new key.**

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window at 100%, 150% and 200%. At 200% the row wraps onto a second line and every control stays visible.

## Checks

- **The empty-place check** passes after the change; two rows failed before it (`verification/W-41b_place_table.md`).
- **All 81 of the app's tests pass.** The wiring table lists the six new buttons: `turnleft`, `turnright`, `mirror`, `unturn`, `snapshot`, `comparebtn` (`verification/B-12c_keyboard_table.md`).
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the app as you normally do, on a project with drawings.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click ↻ (turn right) twice | the picture turns a little to the right each time, and stays inside the viewer; a readout says "30° right · on screen only, Export is not changed" | |
| 2 | Click ⇋ (mirror) | the picture flips left to right; the readout adds "mirrored" | |
| 3 | Try to drag a layer on the picture | it does not move; the status line says the view is turned | |
| 4 | Click Reset in the readout | the picture is straight and unflipped; the readout is gone | |
| 5 | Click 📷 (snapshot) | the compare button next to it stops being grey | |
| 6 | Move to another frame, or change something, then click compare | the snapshot on the left (tagged "Snapshot, frame N"), the picture now on the right ("Now") | |
| 7 | Drag the round knob on the split line | the line follows; more of one side shows | |
| 8 | Click compare again | the normal picture is back | |
| 9 | Turn on the field guide (W-41a's button) | the green 12F, 10F and 8F boxes sit one inside the other, not all on the edge | |
| 10 | Export the shot | it is exactly as before; the turn and the snapshot are not in it | |

Anything marked ✗, tell me the row number.
