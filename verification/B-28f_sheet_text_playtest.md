# B-28f: dialogue and camera columns, by hand

Built on 2026-09-24 against D-84c, dialogue and camera columns, which you accepted the same day
("proceed").

The generated halves are `verification/B-28f_sheet_text_table.md` and the new rows in
`verification/B-28b_timesheet_table.md`. The first reads the sample cut's text cells against
document 25, frame by frame. The second checks every fixture's text columns against the second
reader, and that they are saved and reopened unchanged. This sheet covers what the tables cannot:
where the columns sit and how they look.

## Before you start

Import the sample cut as before: **Import cut...** and the folder `Fixtures/xdts/fx_xdts_040`.
Choose **Workspace: Timing**, so the Sheet is the tall column on the left.

## What to check

1. **The notes.** The import message says **2 notes are below**, not 4. The notes are C's tick
   mark and the unused background. Nothing says dialogue or camerawork was not read.
2. **The order.** The Sheet's headings, left to right: **frame, Dialogue, A, B, C, Camera**.
3. **The dialogue.** Frame 0 of Dialogue reads **MIKA Over here!**, in a pale yellow. A line runs
   down from it through frame 15. Frames 16 to 47 of Dialogue are empty.
4. **The camera.** Camera is empty down to frame 19. Frame 20 reads **FOLLOW**, and a line runs
   down from it through frame 47.
5. **Long words.** If the dialogue is cut off with "...", hold the mouse over it. A tooltip shows
   the whole line.
6. **Clicking a text cell.** Click Camera's frame 30. The playhead goes to frame 30, and no cell
   gets a blue outline.
7. **The keys stay on the drawings.** Click A's frame 2, then press **Left**. The outline does not
   move into Dialogue. Press **Right** twice to reach C, then Right again. It does not move into
   Camera.
8. **Writing leaves the words alone.** On A's frame 3, type **5** and Enter. Dialogue and Camera
   are unchanged. Ctrl+Z.
9. **Saved with the project.** Save the project (Ctrl+S), close the window, open it again and
   open the project. The Sheet shows the same Dialogue and Camera columns.
10. **Undo takes them with the cut.** Press Ctrl+Z until the import is undone. The Sheet goes
    back to the composition before, with no Dialogue or Camera.
11. **A project without them.** Open any project made before today. Its Sheet looks as it did
    in B-28e, with no text columns.

## Known limits

- The words can be read and not changed. Typing dialogue, adding a column by hand and moving an
  entry are not in this step.
- Dialogue shows only in the Sheet. The viewer and the timeline do not show it.
- The Sheet does not print yet. That is the next step you asked for.

## What to answer

"works", or which step number did something else and what it did.
