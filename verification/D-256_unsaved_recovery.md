# D-256: recovery copies of a project that was never saved

Until now, a project with no file of its own got no recovery copies. That covers the reference shot the app opens with, and anything made with File > New. If the window closed before Save As, the work was gone.

Now such a project is copied every two minutes while it has unsaved changes, the same as a saved one. The copies go into the app's own folder:

`%LOCALAPPDATA%\dev.anitastic.anime-compositor\Unsaved`

They are named `Unsaved project.autosave-0.json` to `-4.json`. That means five copies at most; the oldest is replaced first.

At the next start, the bottom line says how many there are, and **File > Recover unsaved work** lists them. Choosing one opens it as an untitled project that is still unsaved, so Save asks where to put it. Once Save As gives the project a file, the copies in the app's folder are removed, and later copies go beside the file as before.

## Pictures

- `D-256 pictures/offered_at_the_start.png`: the app just started, after a run that renamed a layer, waited past two minutes and closed without saving. The File menu shows **Recover unsaved work…**, and the bottom line reads "There is 1 recovery snapshot of a project that was never saved. File > Recover unsaved work opens one."
- `D-256 pictures/recovered.png`: after choosing that copy. The title is "Untitled project (recovered)", the renamed layer ("Renamed for D-256") is back in the timeline, Save has its unsaved dot, and frame 10 is drawn. The bottom line names the copy and says Save As gives it a file.

## The check, written first (`d256_a_never_saved_project_is_copied_and_offered_again` in `app/src/main.rs`, committed in 24474c9)

**On the build before the change**, it did not build, because there were no unsaved copies.

The test uses a scratch folder in place of the app's own. All 16 checks pass in `D-256_unsaved_recovery_table.md`:

| Check | Result |
|---|---|
| With no folder for unsaved copies, nothing is written, as before | pass |
| A new project with two layers (a drawing and a solid): nothing written at the first look, nor 119 seconds later | pass |
| Two minutes after the first change, a copy is written into the app's folder | pass |
| Nothing is written beside the drawings | pass |
| The copy is not a save: the project is still unsaved | pass |
| Seven copies later, five are kept | pass |
| The next start offers the five, newest first, and says so | pass |
| A snapshot from anywhere else is still refused | pass |
| Recovering the newest copy gives an unsaved project with no file of its own | pass |
| Saved into the same folder, it is the original project byte for byte (3297 bytes) | pass |
| Its frame 10 is the original's, every byte | pass |
| Once it has been saved, the copies are removed | pass |

## In the running app

| Step | What happened |
|---|---|
| Start, rename layer2, wait 141 seconds | "Recovery snapshot written to …\Unsaved\Unsaved project.autosave-0.json"; still unsaved |
| Close without saving, start again | The recovery list shows that one copy; the note says so |
| Choose it | layer2 is "Renamed for D-256" again; the project is unsaved |
| Errors on the page | none |

The copy written by this check was deleted from the app's folder afterwards. The window's own settings were put back.

## Checks

- All 85 of the app's tests pass. B-09's rows are unchanged; they run with no app folder set.
- No fixture, command ID, saved project byte or exported picture changes.

## Limits, stated

- **One set of five copies for every never-saved project.** If you start the app, work on a new project without recovering the old copies, and leave it unsaved for two minutes, the new copies start replacing the old ones, oldest first.
- **Opening another project does not remove the copies.** They are offered again at the next start, until a never-saved project is saved with Save As.
- **The Save As removal was checked in the test, not in the window**, because Save As opens a file dialog that the check tool cannot answer.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Start the app, change something (rename a layer), wait three minutes, then close and choose not to save | The window closes | |
| 2 | Start the app again | The bottom line says there is a recovery snapshot of a project that was never saved | |
| 3 | File > Recover unsaved work, choose the copy | Your change is back; the title says "Untitled project (recovered)" | |
| 4 | Save As somewhere, close, start again | No recovery copy is offered this time | |

Anything marked ✗, tell me the row number.
