# W-40c: the ready frames (D-249)

The second screen of the redesign, third and last part. A green line under the work area now marks the frames already in memory, the ones that play at full speed. Beside the time, "240 of 240 ready" says how many of the work area's frames those are. Hovering the count says how much memory they take of your memory setting. Nothing about the picture, the project file or an export changed.

## Before, after, and the Sandbox

| | Picture |
|---|---|
| Before W-40c | `W-40b pictures/compose.png` |
| The Sandbox board it copies | `W-38 pictures/sandbox_compose.png` |
| The real window, after Play has run through the loop once | `W-40c pictures/scale_1.0.png` |
| The page after play, a note added, and Ctrl+Z | `W-40c pictures/after_play_and_undo.png` |

In `scale_1.0.png`:

- the **green line** runs the whole width under the blue work-area band, above the seconds;
- **"240 of 240 ready"**, in green, sits beside "2s 2f · 24 fps".

`after_play_and_undo.png` shows a white viewer. As in W-40b, that is the debugging screenshot only, not the app.

## What it does, checked in the running app

| Step | The count said |
|---|---|
| The app just opened, one frame shown | 1 of 240 ready |
| After Play ran about 12 seconds, then Pause | 240 of 240 ready |
| After clicking Note (an edit) | 1 of 240 ready |
| After Ctrl+Z | 240 of 240 ready |

The tip on the count after play said: "Green line: frames already in memory, they play at full speed · 240 of the 240 work-area frames · about 124 MB of the 17 GB memory setting · an edit clears the line, undo brings it back".

## Its limits (written into D-249)

1. **Any edit clears the whole line, and undo brings it back.** The memory keeps frames for the whole project, not per change. The Sandbox's tip said "an edit clears the frames it changes"; the app's tip says what it really does.
2. **It does not look at the disk.** A drawing replaced in another program leaves its frames green until each is shown. That frame is then made again, so the picture is never out of date.
3. **It fills ahead of the playhead only while playing.**
4. **Zoomed-in part frames are not kept**, so they never count as ready.

## The check

`verification/W-40c_ready_frames_table.md`: **16 of 16 pass**. Before the build it was 5 of 16 (commit 46a01d2). It covers:

- nothing made, nothing ready;
- Full and Draft kept apart;
- solo;
- an edit and undo;
- the whole 240-frame loop made ahead while playing, and its size;
- the graphics card and the CPU kept apart.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| Play | frames are made and kept, as since B-154; the page asks `/ready` as they arrive | the green line grows; the count climbs |
| Any edit (a note, a key, a hidden layer) | the edit, as before | the line drops to the frame on screen |
| Ctrl+Z | undo, as before | the line comes back |
| Hover the count | nothing | the tip with the memory used |

## Keyboard reach

The line and the count are things to read, not controls, so there is nothing new for Tab to reach. Space plays, which fills the line. No new key.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png`: a 1280 by 800 window at 100%, 150% and 200%, playing. The count stays beside the time at every size.

## Checks

- The ready-frames check, 16 of 16.
- All 81 of the app's tests pass. The wiring tables list `ready` as a question the page asks (`verification/B-12b_page_table.md`, `B-12b_routes_table.md`, `B-12c_keyboard_table.md`).
- The engine's frame-memory tests pass.
- The export tests pass: `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the app as you normally do, on a project with drawings.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look beside the time in the timeline's header | a short green dash and "1 of N ready" (N is your work area's length) | |
| 2 | Press Space and let it play through once | a green line growing under the work area; the count climbing to "N of N ready" | |
| 3 | Press Space again to stop, then again to play | it plays smoothly from the start; the line stays full | |
| 4 | Hover the count | a tip with the MB used of your memory setting | |
| 5 | Move a layer or add a key | the line drops to one frame | |
| 6 | Press Ctrl+Z | the full line comes back | |
| 7 | Switch Full resolution on | the line drops (Full frames are not made yet); play fills it again | |
| 8 | Drag the work area shorter | the count's second number becomes the new length | |

Anything marked ✗, tell me the row number.
