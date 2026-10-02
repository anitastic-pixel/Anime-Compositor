# W-40b: the Sheet column (D-248)

The second screen of the redesign, second part. The Sheet now stands as a tall column at the right of the window, the full height from under the top bar down to the status line, beside the viewer, the panels and the timeline. That is where the Sandbox's Animate board has it. A **Sheet** button at the right of the timeline's header brings the column out or puts it away. Notes on the ruler now show in grey in the Sheet's Action column. Nothing about the picture, the project file or an export changed: this is where the Sheet is, not what it does.

## Before, after, and the Sandbox

| | Picture |
|---|---|
| Before W-40b: the Sheet behind the timeline, with the Timeline / Sheet strip | `W-40 pictures/compose.png` |
| The Sandbox board it copies | `W-38 pictures/sandbox_animate.png` |
| Animate (Alt+2), the real window | `W-40b pictures/animate_window.png` |
| Animate, with two ruler notes showing in the Action column | `W-40b pictures/animate.png` |
| Compose, fresh: the column put away, the Sheet button dark | `W-40b pictures/compose.png` |
| Your own saved layout after pressing Sheet | `W-40b pictures/own_layout_sheet_on.png` |

Three of these pictures show a white viewer: `animate.png`, `compose.png` and `own_layout_sheet_on.png`. They are screenshots of the page taken through the app's debugging port, which does not capture the picture the graphics card draws. `animate_window.png` and the three `scale_` pictures are of the real window, and they show the reference shot as normal. The white is in the screenshots only, not in the app.

What the pictures show:

- **the side column**, at the right, the full height of the window. Its left edge is a border you drag, like the other borders.
- **the Sheet button**, at the right end of the timeline's second row, after "reference shot · 4 layers · 10s 0f". It is lit orange while the Sheet is out, and dark while it is put away.
- **no Timeline / Sheet strip** above the timeline any more, in either built-in workspace. The Sheet is no longer stacked behind the timeline.
- **Animate**: the Sheet column is out, and the Effect controls move to the left, above the Effects list, to make the room. The Sandbox has them there too.
- **Compose**: the column starts put away, so Compose looks as it did, only without the strip.
- **grey notes** in `animate.png`: "Key pose" at frame 6 and "note" at frame 15. These are two notes put on the ruler (the orange flags, or the * key). One has a name, so the Sheet shows it. The other has none, so the Sheet says "note". They are grey and slanted because they belong to the ruler, not the Sheet: they are not printed, and you rename or move them on the ruler. If you type your own Action note on that frame, yours shows instead.

## Something fixed on the way

While building this I found a slip from W-31b (28 September). A style meant for the timeline's time readout ("0s 0f · 24 fps") also caught the Sheet's highlighted row, because both use the same name. That row was pushed out of the table, which made the Sheet's frame column very wide and pushed the drawing columns off to the right. It happened wherever the Sheet was shown. The style now applies only to the timeline. In the pictures, the frame column is narrow again and every layer's column is in view.

## Your own layout

Your stored Compose layout predates this change. It still has the Sheet behind the timeline, with the strip, until you either:

- press the **Sheet** button, which moves it into the column, as in `own_layout_sheet_on.png`; or
- choose **Window → Reset workspace**, which gives you the new Compose.

Nothing else in your layout moves. The rest of Animate's layout, the Sandbox's full Animate board, is W-44.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| The Sheet button, while the Sheet is not in the column | the same "move a panel" the right-click menu uses, to the side | the column comes out with the Sheet in it; the button lights |
| The Sheet button, while the Sheet is in the column | nothing in the project; the column is put away | the column goes; the button goes dark |
| Window → Show or hide the Sheet | the same as the button | the same |
| Dragging the column's left edge | the same as every other border | the column wider or narrower; the width is kept, like other panel sizes |
| A grey note in the Action column | moves the playhead there and chooses that cell, as any Action cell does | the cell outlined; typing writes your own Action note there |

None of these is a project edit, so nothing goes into undo, as with every workspace change (D-85).

## Keyboard reach

- **Tab** goes from the zoom slider to the **Sheet** button, then on to the layer rows.
- **Ctrl+Shift+P**, type "sheet", Enter: the palette line "Show or hide the Sheet". The same line is in the **Window** menu, after "Animate workspace".
- **Alt+2** opens Animate with the column out. **Alt+1** opens Compose with it put away.
- **The column's border** takes the arrow keys once Tab reaches it. Left widens the column, right narrows it. This was checked in the running app: one press of Left took it from 380 to 396.
- No new key is bound.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window in Animate at 100%, 150% and 200%.

- At 100% and 150% everything fits.
- At 200% the column keeps its width, so it takes about two fifths of a small window and the timeline's rows go below the fold. Drag its edge narrower, or press Sheet to put it away. I left this alone: the Sheet needs about that width to show four layers at once.

## Checks

- The page's wiring tests pass, with the control list updated in the same change. The Sheet button is the one new control, so there are 87 now. No new key. The table is `verification/B-12c_keyboard_table.md`.
- All 80 of the app's tests pass.
- The export tests pass: `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. So the exported pictures are unchanged.
- Checked in the running app:
  - the button lights, and the Sheet updates while in the column;
  - pressing it again puts the column away;
  - the border's arrow keys work;
  - the Sheet's table measures 341 wide with a 57-wide frame column (it was 658 and 374 before the fix);
  - your stored layout was put back after each check.
- The engine code was not touched. Printing the Sheet is drawn by the engine, so it is unchanged: the grey notes are not printed.

## Playtest

Open the app as you normally do, on a project with drawings.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Press the **Sheet** button at the right end of the timeline's header | the Sheet as a tall column at the right of the window; the button lights orange; the Timeline / Sheet strip is gone | |
| 2 | Look down the Sheet's frame column | narrow, with every layer's column in view, and no sideways scrolling needed for four layers | |
| 3 | Press Play | the blue row runs down the Sheet with the playhead | |
| 4 | Press Sheet again | the column goes away; the viewer and timeline take the room | |
| 5 | Drag the column's left edge | it gets wider or narrower | |
| 6 | Put a note on the ruler with *, double-click it and name it | the name, grey and slanted, in the Action column on that frame | |
| 7 | Put a note with no name on another frame | "note", grey, in the Action column there | |
| 8 | Click that grey cell and type a word, then Enter | your word replaces the grey "note" on the Sheet; the flag stays on the ruler | |
| 9 | Press Alt+2 | Animate: the Sheet column out, Effect controls on the left | |
| 10 | Press Alt+1, then Window → Reset workspace | Compose: no column, Sheet button dark | |
| 11 | Ctrl+Shift+P, type "sheet", Enter | the column comes out | |

Anything marked ✗, tell me the row number.
