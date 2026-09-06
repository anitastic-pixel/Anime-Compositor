# B-10, exporting from the window

The renderer could export a shot and the window could not ask it to. It can now, and this is what the asking does. Produced by `cargo test -p anime_compositor_app`, from `app/src/main.rs`.

What the frames themselves look like is not this table's question — `T-08_export_table.md` checks the pixels and the naming to the frame, and `B-10_full_shot_table.md` exports the whole shot twice and requires the two to be identical byte for byte. What is new here is everything between the button and that: **which frames the window asks for, what it names them, that what is written is the project as it was when the person asked rather than as it is when the job finishes, and that a refusal, a cancellation and a failure each arrive in words instead of in silence.**

The project is the reference shot, chosen because of its deliberate gap: layer 3 has no drawing 7, so frames 14 and 15 ask for a drawing that is not there. Document 07 blocks an export on that by default, and the checkbox beside the Export button is the person overriding it in front of a sentence that says what the override does.

Rows about a sentence quote it. The Expected column is the words that had to reach the person; the Actual column is those words if they did, and everything the window said instead if they did not.

| Check | Expected | Actual | Result |
|---|---|---|---|
| the range offered is the work area, first to last frame inclusive | 0 to 239 | 0 to 239 | pass |
| the files are named for the project and carry the frame number, four digits wide | the reference shot_%04d.png | the reference shot_%04d.png | pass |
| at eight bits with straight alpha, which is what the export fixtures were written with | Eight, Straight | Eight, Straight | pass |
| the open project has been changed since the export was asked for | Renamed while the export was running | Renamed while the export was running | pass |
| and the export is still writing the project as it was when it was asked for | layer1 | layer1 | pass |
| by default a frame whose drawing is missing stops the export before anything is written | Nothing was exported. | Nothing was exported. | pass |
| and the person is told how many frames it is | 2 of the 2 frames asked for have a drawing that is missing | 2 of the 2 frames asked for have a drawing that is missing | pass |
| and what to do about it | Relink or restore the missing drawings | Relink or restore the missing drawings | pass |
| nothing at all is on the disk | [] | [] | pass |
| asked to write them anyway, the window writes them and says where | Exported 2 frames into <a temporary directory>. | Exported 2 frames into <a temporary directory>. | pass |
| the two files are named for the frames that were asked for | ["the reference shot_0014.png", "the reference shot_0015.png"] | ["the reference shot_0014.png", "the reference shot_0015.png"] | pass |
| an exported frame is full size, whatever resolution the preview was showing | 1920x1080 | 1920x1080 | pass |
| and the drawing that is missing is still reported rather than passed over in silence | Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing. | Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing. | pass |
| once for each frame it was missing on, not once for the export | Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing. | Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing. | pass |
| a cancelled export says how far it got, and does not claim to have succeeded | Export stopped at your request after 0 of 2 frames | Export stopped at your request after 0 of 2 frames | pass |
| and the window's own first sentence says what is there, not that it exported them | The 0 frames that finished are in C:\Users\Andrew\AppData\Local\Temp\anime_compositor_b10_cancel. | The 0 frames that finished are in C:\Users\Andrew\AppData\Local\Temp\anime_compositor_b10_cancel. | pass |
| and left no half-written file behind | [] | [] | pass |
| asking a window with nothing running to cancel is not an error either | No export is running. | No export is running. | pass |
| an export into a folder that is not there says how far it got before it stopped | The export stopped on a problem after 0 of 2 frames | The export stopped on a problem after 0 of 2 frames | pass |
| and names the file it could not write, rather than only that something went wrong | Frame 14 could not be written to <a temporary directory>\no_such_directory\the reference shot_0014.png. Check that the folder exists, is writable and has room | Frame 14 could not be written to <a temporary directory>\no_such_directory\the reference shot_0014.png. Check that the folder exists, is writable and has room | pass |
| and the folder is still not there: nothing was created to hold a failure | false | false | pass |

**21 of 21 checks pass.**

## What this does not cover

The folder dialog, which belongs to the operating system and which a test has no hands to answer, so what is checked here begins at the folder the person chose.

**Two frames, not two hundred and forty.** Which frames the window asks for is a row above; what a whole shot does is `B-10_full_shot_table.md`, which runs for four minutes and is not part of an ordinary build.

**There is no progress bar.** R-09 asks for cancellation between frames and for failure to be reported, and both are here; it does not ask for a count of frames as they are written, and the core has no hook to report one without a change to its signature that only a window wants. What the window shows while a job runs is what it is doing and a Cancel button, and what it shows afterwards is the core's report.

**A second export while one is running** is refused by `start_export`, which needs a running application to reach — the refusal is one branch above the part this table can call.

Where a row says *a temporary directory*, the real value was this machine's scratch directory, which differs on every machine and every run.
