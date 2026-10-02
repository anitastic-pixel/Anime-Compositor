# D-253 and D-254: a cut status, and label colours on project items

**D-253, cut status.** Every composition (cut) now has one of five statuses: Not started, In progress, Check, Retake or Done. The status is a coloured chip. It appears in two places in the Project panel:
- beside each card in **By cut**;
- on the line about the chosen composition, at the top of the panel.

Click the chip to choose a status. The change can be undone, and the status is saved with the project.

**D-254, labels.** Right-click a composition or a footage item anywhere in the Project panel to see the same row of eight label colours that layers already have, plus "No label". A labelled item gets a small coloured square before its name. The square shows in all of these places:
- List;
- Pictures;
- Tree;
- By cut;
- the line at the top of the panel.

## Pictures

- `D-253 pictures/1_list_with_label_dots.png`: the List view. It shows the following:
  - the composition labelled lavender;
  - the footage item layer1 labelled yellow;
  - the composition's status chip reading "Retake" on the top line.
- `D-253 pictures/2_by_cut_status_menu.png`: the By cut view with the status menu open. "Retake" has a ✓ beside it, and the card shows the lavender square and the Retake chip.

## The check, written first (`tests/d253_d254_roundtrip.rs`, committed in ecae666)

**On the build before the change** it did not build. The build had no status and no item labels (10 errors).

**On the new build** it writes `D-253_D-254_roundtrip_table.md`. All 10 rows pass:

| Check | Result |
|---|---|
| Label colour 9 is refused | pass |
| A status, a composition label and a footage label set by command read back after save and open | pass (retake, 5, 2) |
| A project without them reads as Not started, with no labels | pass |
| All 2297 fixture projects open and save **byte for byte as before** (one SHA-256 over all of them, taken on e395d8b) | pass |
| A status and labels written by a later build are kept through open and save | pass |

A status or a label is only written to the file when one has been set. This is why no existing project changes.

## In the running app (`d253_check.js`)

| Step | What the page said |
|---|---|
| By cut opened | 1 status chip, reading "Not started"; the top line also reads "Not started" |
| Chip clicked | menu: Not started ✓ / In progress / Check / Retake / Done |
| Retake chosen | project says `status: "retake"`; chip reads "Retake" |
| Right-click on the cut | 9 choices: No label and the 8 colours |
| Lavender chosen | composition label 5; the square shows on the card |
| Right-click on layer1 in List, yellow chosen | footage label 2; squares on both list rows |
| Label 9 sent | "There is no label colour 9; the colours are 1 to 8." |
| Status "finished" sent | "\"finished\" is not a cut status. Send not_started, in_progress, check, retake or done." |
| Three undos | status gone, both labels gone |
| Errors on the page | none |

The window's own settings were put back afterwards.

## Checks

- All 81 of the app's tests pass. The command map lists the new `item.set_label`, and the keyboard table says it can be reached without a mouse (through the command finder).
- `composition.set_settings` can now name any composition. Without a name it changes the one on screen, as before.

## Limits, stated

- **There is no Status box in Composition settings (Ctrl+K).** The status is set from the chip in the Project panel only.
- **An item with no label shows no square.** This keeps unlabelled lists as they were.
- **Labels on project items are not the layer labels.** A layer made from a labelled footage item does not take that item's colour.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Project panel → By cut. Click the grey "Not started" chip on a cut, then choose Retake | The chip turns red and says Retake | |
| 2 | Ctrl+Z | It goes back to Not started | |
| 3 | Right-click a composition or a drawing in List, then pick a colour | A small square of that colour before its name, in every Project view | |
| 4 | Set a status and a label, save, close and reopen the project | Both are still there | |

Anything marked ✗, tell me the row number.
