# D-257: files gone from the disk, in the health chip

The warning chip in the top bar now also counts drawings whose file is no longer on the disk. For example, it reads "⚠ 1 file gone". The window asks every 5 seconds and again whenever it gets focus back. You don't need to reopen the project.

Click the chip to see each gone file. The line reads "layer2: layer2_005.png is gone", with **Relink…** on the right. Relink opens the same file dialog as the Project panel's Relink. When there are also warnings, the list ends with **Error details**. With no gone files, the chip opens Error details as before.

## Picture

- `D-257 pictures/file_gone_chip_and_menu.png`: the top bar reading "⚠ 1 file gone", with the list open beneath it.

## The check, written first (`d257_a_deleted_drawing_is_named_and_a_restored_one_is_not` in `app/src/main.rs`, committed in 50ab823)

**On the build before the change** it did not build, because there was no `files_gone`.

While running it on the new build I found a fault in the check itself: it locked the project twice in one line, so it waited forever. I fixed that line. The six checks it makes are the same.

All 6 checks pass in `D-257_files_gone_table.md`, on a copy of the reference shot:

| Check | Result |
|---|---|
| Copied reference shot: nothing gone | pass |
| Drawing 7 of layer3, which never had a file, is a gap and not a gone file | pass |
| layer2_005.png deleted: named, with its footage item | pass |
| Asking changes nothing (no undo step, no new note) | pass |
| The answer names the footage item to relink (asset-layer2) | pass |
| Put back: no longer named | pass |

## In the running app (`d257_check.js`)

A scratch copy of a drawing was imported. The file was then deleted, and later put back, by a separate program, with the window left alone. No fixture was touched.

| Step | What the page said |
|---|---|
| Imported, file there | chip hidden, 0 gone |
| File deleted outside the app | chip "⚠ 1 file gone" within the next 5-second check |
| Chip clicked | "visitor_000.png: visitor_000.png is gone", with Relink… |
| File put back | chip hidden again, 0 gone |
| Import undone | back to 4 footage items |
| Errors on the page | none |

The window's own settings were put back afterwards.

## Checks

- All 82 of the app's tests pass. `/files-gone` is listed among the window's own routes and in the not-found sentence.

## Limits, stated

- **A file replaced by a different picture under the same name is not "gone".** The viewer already redraws it.
- **A file that was already missing when the project opened is counted too.** The chip counts what is not on the disk now. The open-time warning about it also stays in Error details.
- **Relink is per footage item.** Two gone drawings of one sequence give two lines that open the same relink.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Import a drawing from a spare folder, then delete that file in Explorer and click back on the window | The chip reads "⚠ 1 file gone" | |
| 2 | Click the chip | The file's name with Relink… | |
| 3 | Put the file back (Recycle Bin → Restore) and click back on the window | The chip goes away | |

Anything marked ✗, tell me the row number.
