# D-252: watch it write, and Pause

Before this change, the Render panel showed only a progress bar while it wrote. You saw nothing of the frames until the export had finished.

Now, while an export or a queue row is being written, the panel shows:

- **The frame last written, large**, with "Frame 41, as written" under it. This is the file on the disk, shrunk.
- **The five frames last written, small**, each with its number. A frame whose drawing was missing has an orange border and reads "38 · missing".
- **A bar of the job's frames.** Written frames are blue, frames whose drawing was missing are orange, and a white mark shows the last one written. Under the bar: "Frames 0 to 59 · 4 frames so far with a drawing missing, in orange".
- **Pause**, beside Stop. The export waits before writing its next frame. The top line, the row and the status read "Paused", and the button reads **Go on**. Pressing it again carries on from where it waited.

Stop still works while paused. The frames stay on show after the export ends, until the next one begins.

## Pictures (`D-252 pictures/`)

- `d252_paused.png`: the reference shot, frames 0 to 59 as PNG, paused just after frame 40.
  - The top line reads "Paused · Rendering 1 of 1 · reference shot · 42 of 60".
  - The button reads Go on, and the row reads "Paused · 70%".
  - Frame 41 is shown large, with 37 to 41 in the strip.
  - 38 and 39 are marked missing: their yellow square, layer3's drawing 7, is absent.
  - The bar is orange at 14–15 and 38–39.
- `d252_paused_close.png`: the same, close up.
- `d252_done.png`: after Go on. The row reads Done. Frame 59 is large, 55 to 59 are in the strip, and the bar is full, with "4 frames with a drawing missing".

## The check, written first

The check is `d252_paused_it_waits_then_writes_what_an_unpaused_export_writes` in `app/src/main.rs`, committed in 14448dd.

**On the build before the change**, it did not build, because there was no Pause and no watch.

All 11 checks now pass, in `D-252_watch_and_pause_table.md`:

| Check | Result |
|---|---|
| An export never paused writes 60 frames, kept to compare against | pass |
| Pause with nothing running says "No export is running." | pass |
| Paused once frame 40 is on the disk, it waits with 41 to 45 frames written (here 41) | pass |
| For five seconds after that, the folder does not grow | pass |
| The window says it is paused | pass |
| The strip holds the five frames last written (36 to 40) | pass |
| The picture shown for frame 40 is the written file for frame 40, shrunk the same way: 480 × 270, no byte more than 1 apart | pass |
| Pause again says "Going on." | pass |
| After going on, the folder is the never-paused export's: 60 files, every byte the same | pass |
| The orange frames (14 to 15, 38 to 39) are exactly the ones an export that refuses missing drawings names | pass |

## In the running app

| Step | What happened |
|---|---|
| Render workspace, work area 0 to 59, PNG, "Write frames whose drawing is missing" ticked, Render, folder chosen | The export starts; frames appear in the large picture and the strip as they are written |
| Pause, as soon as frame 40 was written | It waited after frame 41: "42 of 60", Paused, Go on |
| Five seconds later | The same five frames, still paused |
| Go on | It finished: "Exported 60 frames into …", with the four missing-drawing frames listed as before |
| The folder afterwards | 60 PNG files |
| Errors on the page | none |

The folder dialog was answered by a helper script on this machine. The window's own settings were put back afterwards.

One fault was found and fixed during this run. After the export had finished, the line under the bar still said "so far", and the orange border on the strip was too thin to see. Both now read as in the pictures.

## Checks

- All 87 of the app's tests pass.
- No fixture, command ID, saved project byte or exported picture changes; the check above compares the written files byte for byte.
- The page and route lists (B-12b, B-12c) gain the Pause button and the three routes.

## Limits, stated

- **Pause settles within a few frames.** Frames are drawn several at a time, and those already drawn are held, not written, until you press Go on. A pause pressed at frame 40 can therefore stop anywhere up to frame 44. It stopped at 41 in both runs here.
- **The time includes the pause.** "Time" and "about N s left" count the paused seconds as well.
- **The pictures are small copies.** They are at most 480 pixels across, made from the frame as written. They are not a full-size viewer.
- **Not included:** a separate watcher window or popup, and keeping the viewer animating while rendering.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Render workspace. Set a long work area (a few seconds), pick PNG, press **Render**, choose an empty folder | Frames appear as they are written: one large, five small with numbers, a blue bar growing | |
| 2 | If any drawing is missing (tick "Write frames whose drawing is missing") | Those frames are orange on the bar and say "missing" in the strip | |
| 3 | Press **Pause** | Within a moment: "Paused" in the top line and the row; the button reads Go on; the picture stops changing | |
| 4 | Look in the folder while paused | It stops growing | |
| 5 | Press **Go on** | It carries on and finishes; the folder holds every frame | |
| 6 | Render again, Pause, then **Stop** | It stops; the frames already written are kept | |
| 7 | Add two rows to the queue and Render; Pause during row 2 | Row 2 reads "Paused"; Go on finishes it and row 3 follows | |

Anything marked ✗, tell me the row number.
