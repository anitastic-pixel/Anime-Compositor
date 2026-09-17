# B-15c: collecting and checking a shot, by hand

Built on 2026-09-16 against D-61, which the owner accepted the same day ("accept d-61").

The generated half is `verification/B-15c_panel_table.md`, 19 of 19. It calls what the window
calls and checks what comes back, starting from a folder already chosen, because no test can
answer a Windows folder picker. This sheet covers what a table cannot judge: where the tick and
the two buttons are, the folder picker itself, and whether a package really opens somewhere else.

## Before you start

Open `Fixtures/packaging/source/shot.json`: this is D-61's test shot, with eight drawings. At
once use **Save As...** and save it as `verification/B-15c playtest/shot.json` (make the
folder). Everything below is done on that copy, so the fixture itself is never changed. The copy
still points at the drawings in `Fixtures/packaging/source`.

One of its drawings, `gone_0001.png`, is missing on purpose. The window says so when the shot
opens. That is expected.

## What to check

1. **The tick.** In the Project panel, under Drawings, each row has a **Can be passed on** box.
   All are ticked except **Licensed sheet**.
2. **Unticking is an undoable change.** Untick **Cels**. The status line says "Keep asset-cel
   out of packages", the project shows unsaved changes, and the Undo button names the change.
   Press Ctrl+Z: Cels is ticked again. Clicking the box does not select the row or start a drag.
3. **The keyboard.** Tab to a Cels box and press Space. It unticks (and the row is not chosen
   instead). Press Ctrl+Z to put it back.
4. **Collect Files...** is beside Save As at the top. Click it. A folder picker opens. Make a new
   empty folder, for example `B-15c playtest/package`, and choose it. The status line says:
   "Collected into ...: 10 drawings copied. 1 drawing was missing, listed without a copy.
   1 drawing was left out, marked not to be passed on. The open project is unchanged."
5. **Nothing in the window changed.** The title, the unsaved mark and the Undo button are as
   they were before step 4.
6. **The folder.** In File Explorer, the package folder holds `shot.json`,
   `package-manifest.json`, and a `media` folder with one folder per drawing set. There is no
   `sheet.png` (left out) and no `gone_0001.png` (missing). No `package.partial` folder is left
   next to it.
7. **A folder with something in it is refused.** Click **Collect Files...** again and choose the
   same folder. The status line says the folder is not empty, and nothing in it changes.
8. **Moved, it still opens.** Move or copy the package folder somewhere else, ideally a
   different drive or another computer, into a folder whose name has a space in it. Open its
   `shot.json`. The shot draws as before. The notes list two drawings as missing: the licensed
   sheet and `gone_0001.png`. Nothing else is listed.
9. **Check Package.** With the moved package open, click **Check Package**. The status line
   says "Checked 12 files against the package manifest: 10 ok, 1 missing, 1 excluded." The
   notes list the two by their place in the package.
10. **A changed file is caught.** Replace `media/asset-cel/cel_0001.png` in the package with any
    other PNG (keep the name). Click **Check Package** again. The status line now says
    "1 changed", and the notes name that file with PACKAGE_FILE_CHANGED under the error details.
11. **The licensed sheet supplied.** Copy `Fixtures/packaging/source/licensed/sheet.png` to
    `media/asset-licensed/sheet.png` in the package. Click **Check Package**. The excluded one
    is gone from the notes and counts as ok.
12. **Check Package on a project that is not a package.** Open your playtest copy
    `B-15c playtest/shot.json` and click **Check Package**. The status line says the package
    manifest is missing or unreadable.
13. **The tick is saved.** In the playtest copy, untick **Cels**, save, close the window and
    open the copy again. Cels is still unticked. Collect it into a new folder: the message now
    says 3 drawings were left out, and `media/asset-cel` has no files.

## Known limits

- Collecting is not in a real File menu. This window has a row of buttons, and the two new ones
  sit beside Save As.
- No progress bar: a very long sequence collects with the window waiting. The fixture shot
  takes an instant.
- Checking is done only when you ask, not when a shot opens (D-61).
- The tick has no licence details; it stands in for them (D-61, "not decided here").

## What to answer

"works", or which step number did something else and what it did.
