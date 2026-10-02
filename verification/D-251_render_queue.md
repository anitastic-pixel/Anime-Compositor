# D-251: the render queue

Until now, the Render panel held a single row: the open composition. **Render** asked for a folder and wrote it straight away, the same as Composition > Export.

Now the panel is a queue, the way After Effects' Render Queue works:

- **Add to queue** (in the Render panel), or **Composition > Add to Render queue…**, asks which folder to write into. It then keeps a row with the composition **as it is at that moment**, written as the Output choices say (PNG, GIF, MP4 and so on).
- Each row has a tick box, its number, the composition, a status, the format and frame count, the folder, when it was added, how long it took, and a **×** to remove it.
- **Render** writes the ticked rows one after another, each into its own folder. A row that finishes is marked Done and unticked, so pressing Render again does not write it twice.
- **Stop** stops the row being written. Rows already done stay whole, and later rows are marked Not started.
- **Drag** a row onto another to change the order.
- If you edit the project after adding a row, the row says "Added at 01:14 PM, before your last edits." in amber, with a **Refresh** button. Refresh takes the project as it is now for that row. Without Refresh, the row writes what it was given.
- With no rows in the queue, Render works as before: it asks for a folder and writes the open composition.

## Pictures

- `D-251 pictures/queued.png`: three rows of the reference shot (frames 0 to 11) added in turn: PNG sequence, GIF, and MP4 Standard. Each one names its folder and says when it was added.
- `D-251 pictures/stale.png`: the same three rows after layer2 was renamed. Each one now reads "Added at 01:14 PM, before your last edits." with Refresh.
- `D-251 pictures/rendering.png`: Render pressed after Refresh on row 1 only. Row 1 is Done and unticked, row 2 is "Rendering · 0%", and row 3 is Queued. The top line reads "Rendering row 2 of 3, reference shot as one GIF…", with Stop beside it. Rows 2 and 3 keep their amber note.
- `D-251 pictures/done.png`: all three are Done, unticked, with their times. How it went reads "Rendered 3 of 3 rows." and then each row's own sentence.

## The check, written first (`d251_the_queue_writes_what_one_export_writes` in `app/src/main.rs`, committed in 42807bc)

**On the build before the change**, it did not build, because there was no render queue.

The test adds rows straight from code (no folder dialog) into scratch folders, and compares them with an ordinary Export. All 19 checks pass in `D-251_render_queue_table.md`:

| Check | Result |
|---|---|
| Four rows added; three ticked, one not | pass |
| Render writes the ticked rows in turn: "Rendered 3 of 3 rows." | pass |
| Row 1, PNG: the same 12 files as one Export, every byte | pass |
| Row 2, GIF: the same file as one Export, every byte | pass |
| Row 3, MP4: the same file as one Export, every byte except the time it was written (see below) | pass |
| The unticked row writes nothing | pass |
| After: the three are Done and unticked; the fourth still waits | pass |
| Stop during row 2: "Stopped during row 2 of 3." Row 1 is whole, row 2 keeps fewer than 12 frames, row 3 has no files | pass |
| A row dragged to the top is first; a row removed with × is gone | pass |
| A row is marked after an edit, and not before; Refresh clears the mark | pass |

**About the MP4.** An MP4 written by Windows' encoder stores the second it was written, in three places at the top of the file. Two films of the same frames written a second apart differ in those bytes alone. The check blanks those bytes, and only those, before comparing. Every picture byte is compared.

## In the running app

| Step | What happened |
|---|---|
| Render workspace, work area frames 0 to 11, "Write frames whose drawing is missing" ticked | |
| Add to queue → PNG folder; Composition > Add to Render queue… → GIF folder; Add to queue → MP4 folder | Three rows, as in `queued.png` |
| Rename layer2 | All three rows marked "before your last edits" |
| Refresh on row 1 | Row 1's mark goes; rows 2 and 3 keep theirs |
| Render | Rows written in turn (PNG 0.7 s, GIF 2.3 s, MP4 1.0 s on this machine); "Rendered 3 of 3 rows." |
| The folders afterwards | 12 PNG files, 1 GIF, 1 MP4 |
| Errors on the page | none |

The folder dialogs were answered by a helper script on this machine. The window's own settings were put back afterwards, and nothing was left in the app's own folder.

One fault was found and fixed during this run: after an edit, the rows did not show the amber mark until something else changed in the queue. The page now asks again after any edit.

## Checks

- All 86 of the app's tests pass.
- No fixture, command ID, saved project byte or exported picture changes. The page and route lists (B-12b, B-12c) gain the two new buttons and the `/queue` route.

## Limits, stated

- **Ctrl+M is still Export** (asks for a folder, writes now). In After Effects it adds to the queue instead; say if you want it changed.
- **One Stop for everything.** Stop (or Cancel export at the top) stops the whole queue at the row being written. Pausing, and watching frames appear as they are written, is D-252, next.
- **The queue lasts for this sitting only.** It is not saved with the project, and it is empty when the app starts again.
- **Only the open composition can be added.** To queue another composition, open it first, then Add to queue.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Render workspace. Pick PNG in Output, press **Add to queue**, choose an empty folder | A row appears, ticked, Queued, naming that folder | |
| 2 | Pick GIF, then **Composition > Add to Render queue…**, choose another empty folder | A second row, GIF | |
| 3 | Rename a layer | Both rows say "before your last edits" in amber, with Refresh | |
| 4 | Press Refresh on row 1 | Row 1's amber note goes; row 2 keeps it | |
| 5 | Drag row 2 onto row 1 | They swap places | |
| 6 | Press **Render** | Each row turns Rendering, then Done; How it went lists both | |
| 7 | Open the two folders | The PNG frames in one, the GIF in the other | |
| 8 | Tick a row again, press Render, then Stop at once | That row says Stopped; the top line says where it stopped | |
| 9 | Press × on a row | It is gone | |

Anything marked ✗, tell me the row number.
