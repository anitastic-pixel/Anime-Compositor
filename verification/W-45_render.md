# W-45: the Render workspace (D-248)

The seventh screen of the redesign. There is now a third workspace, **Render**, beside Compose and Animate. It follows the Sandbox's Render board, but only as far as the engine already goes: one item, written as one run.

- **How to get there.**
  - The top bar has **Render** after Animate.
  - Alt+3 opens it.
  - Window › Render workspace opens it.
  - Ctrl+Shift+P finds it.
  - Clicking the export task in the top bar, the chip that shows while an export runs, opens it from any workspace.
- **The layout.**
  - The Project panel is on the left, 340 wide.
  - The Render queue fills the rest.
  - The viewer, timeline, Effect controls, Effects and Sheet are put away while you are in Render, and they come back as they were when you return to Compose or Animate.
- **The Render queue panel, top to bottom:**
  - **The head line:** "Not rendering", or "Rendering 1 of 1 · *composition* · *done* of *total*" with a blue bar under it. On the right are **Stop**, shown only while rendering, and the blue **▷ Render** button, greyed while rendering.
  - **The queue table:** Render #, Composition, Status, Output, Time, with one row for the open composition.
    - Status is **Ready**, **Rendering · N%** (blue), **Done** (green), **Stopped** or **Stopped on a problem** (yellow), or **Nothing written**.
    - Output names the format, adding ", missing drawings written" when that box is ticked.
    - Time counts the run as m:ss.
  - **Output:** the same format list and quality list as the File menu's export, and "Write frames whose drawing is missing". While the Render queue is open they live here. When it is put away they go back to the menu.
  - **How it went:** the export's report, the same sentences as before, and below it the frames left and the time left while it runs.
- **▷ Render does exactly what Export did.** It asks which folder to write into, then writes there. **Stop** is the same as the export's Cancel.
- **The panel can be moved like any other.** Drag its tab, or use Window › Move panel, to put it beside the timeline in Compose, for example. Reset workspace puts it away again.

No new command was added. Render and Stop press the existing Export and Cancel. One key was added (Alt+3) and two buttons (`renderstart`, `renderstop`).

## Pictures

| | Picture |
|---|---|
| The Sandbox's Render board above, the app below (staged at 118 of 240) | `W-45 pictures/sandbox_vs_app.png` |
| The whole window, Ready | `W-45 pictures/whole_window.png` |
| The whole window while rendering (staged, see below) | `W-45 pictures/running_staged.png` |
| The real window at 100% / 150% / 200% | `W-45 pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |

**The running pictures are staged.** The page was told "118 of 240 done, 48 s in" without a real export, because a real render first opens the Windows folder dialog, which a script cannot answer. A real render is row 4 of the playtest.

## What it does, checked in the running app (`w45_check.js`)

The check used the reference shot. Nothing was exported or saved, and the window's own layout and settings were put back afterwards.

| Step | What the page said |
|---|---|
| Compose | Render queue not shown; the format list in the File menu |
| Alt+3 | workspace Render; the top bar's Render pressed; only Project (340 wide) and Render queue shown; the format list inside the panel |
| Chose MP4 | Output "MP4 (H.264)…"; the quality list shown |
| Ticked "Write frames whose drawing is missing" | Output gains ", missing drawings written" |
| Pressed ▷ Render | sent `/export?format=mp4&quality=standard&missing=write`, exactly what Export sends (the request was caught, not run) |
| Ready | "Not rendering"; row "1 reference shot Ready"; "Nothing has been rendered yet in this sitting."; Stop hidden; Render not greyed |
| Started, 0 of 240 | "Checking the drawings"; Stop shown; Render greyed |
| 118 of 240, 48 s | "Rendering 1 of 1 · reference shot · 118 of 240"; "Rendering · 49%"; time 0:48 |
| Report "Exported 240 frames into …" | Done, green |
| Report "The 96 frames that finished are in …" | Stopped, yellow |
| Report "The export stopped on a problem after …" | Stopped on a problem, yellow; both report lines under How it went |
| Report "Nothing was exported." | Nothing written |
| Clicked the export task in the top bar from Animate | Render opens |
| Back to Compose | viewer 1970 wide, timeline shown, format list back in the menu, Render queue hidden |
| Moved the Render queue to the bottom in Compose | shown as a tab "Render queue" beside "Timeline"; the format list inside it |
| Reset Compose | Render queue put away again |
| Errors on the page | none |

## Limits, stated

The board shows more than the engine can do yet. These parts are **not built**. Each would need the engine to change, so each will come to you as its own proposal:

- **A queue of several items.** The board has Cut 012, 013 and 011, each with a tick and its own output. Today there is only ever one row, the open composition.
- **Pause and resume.** "Stopped at 96, resumes there" is not possible yet. Stop keeps the frames already written, as it always has.
- **Watching it write.** The newest frame shown large, the strip of frames around it, and the bar with the missing frames in orange are not built.
- **"Keep animating while it renders", and the popup or own-window watcher.**
- **The output folder in the Output column.** The folder is asked when you press Render, so it can't be shown before.

Two smaller ones:

- **Time is counted on the page.** It restarts if the window is reloaded during a render.
- **The panel is mostly empty below the row on a tall window.** That space is where the watcher would go.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| Render (top right), Alt+3, Window › Render workspace | nothing is sent | Project and Render queue only |
| the export task in the top bar | nothing is sent | the Render workspace |
| the format list, the quality list, the missing box | nothing is sent | the Output column follows |
| ▷ Render | the same export as File › Export (folder dialog first) | the row goes blue and counts up; then Done, Stopped or a problem |
| Stop | the same as the export's Cancel | the row turns yellow: Stopped; the frames already written stay |
| Compose / Animate | nothing is sent | your panels as they were |

## Keyboard reach

- **Alt+3 opens Render**, as Alt+1 and Alt+2 open the others. The hint line at the bottom now says so.
- **Render, Stop, the lists and the box are ordinary buttons, lists and a tick box.** Tab reaches them, and Space or Enter presses them.
- These were not checked with a real keyboard.

## Checks

- **All 81 of the app's tests pass.**
  - The wiring table gained `renderstart` and `renderstop`, giving 101 entries.
  - The key table gained "3", giving 49.
  - `verification/B-12c_keyboard_table.md` is rewritten by those tests and is committed with this screen.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the reference shot.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click Render at the top right | Project on the left, Render queue filling the rest; "Not rendering"; one row "1 reference shot Ready" | |
| 2 | Choose MP4 under Output | the row's Output says MP4; a quality list appears | |
| 3 | Tick "Write frames whose drawing is missing" | the row's Output adds ", missing drawings written" | |
| 4 | Press ▷ Render and choose an empty folder | the row goes blue and counts up; the bar moves; Stop appears; then the row says Done in green and How it went says where it wrote | |
| 5 | Open that folder | the film is there | |
| 6 | Press ▷ Render again, choose a folder, and press Stop partway | the row says Stopped in yellow; the frames written so far stay | |
| 7 | Start a render, then click Compose while it runs | your Compose panels as before; the export task shows in the top bar | |
| 8 | Click that task | back in Render | |
| 9 | Press Alt+1, then Alt+3 | Compose, then Render | |

Anything marked ✗, tell me the row number.
