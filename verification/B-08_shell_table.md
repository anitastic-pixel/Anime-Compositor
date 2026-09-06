# B-08, what the page is handed when it asks for a frame

Between the renderer and the picture on screen there is a short journey: the page asks for a frame over a local address, and gets back raw pixels with everything the window has to say about them in the headers beside. Nothing else in this project looks at that journey. Produced by `cargo test -p anime_compositor_app`, from `app/src/main.rs`.

The promise is that **the picture and the words about it always came from the same render**. The frame number, the resolution, the project's name, the notes and the unsaved-work flag are attached to the picture they describe, so the number on screen can never belong to a different drawing than the one under it.

| Check | Expected | Actual | Result |
|---|---|---|---|
| asking for frame 0 is answered | 200 | 200 | pass |
| with raw pixels rather than an image file, which is what the page draws | application/octet-stream | application/octet-stream | pass |
| and the answer says which frame it is | 0 | 0 | pass |
| at draft resolution, which is what the preview opens at | Draft | Draft | pass |
| and says so, so nobody mistakes it for what an export would write | true | true | pass |
| 480 wide | 480 | 480 | pass |
| 270 high | 270 | 270 | pass |
| and the picture is exactly that many pixels, four bytes each | 518400 | 518400 | pass |
| the page is allowed to read the answer | * | * | pass |
| and allowed to read the headers beside it, not only receive them | * | * | pass |
| a frame asked for by number did not come from the clock, so there is no playback report beside it |  |  | pass |
| stepping past the end of the shot stops at the last frame | 239 | 239 | pass |
| and stepping before the beginning stops at the first | 0 | 0 | pass |
| the first frame of a playback is the one at the start of the shot | 0 | 0 | pass |
| and this time the playback report is beside it | Played 1 frames in real time. No frames were dropped. | Played 1 frames in real time. No frames were dropped. | pass |
| a second into the shot the clock asks for frame 24, not the next one along | 24 | 24 | pass |
| and says the 23 frames in between were passed over | 23 | 23 | pass |
| time running backwards holds the frame rather than rewinding | 24 | 24 | pass |
| and counts nothing as skipped | 0 | 0 | pass |
| asking for full resolution gets it | Full | Full | pass |
| at the shot's own size | 1920 | 1920 | pass |
| and full resolution is what an export would write, so the warning goes away | false | false | pass |
| a resolution nobody recognises changes nothing rather than guessing | Full | Full | pass |
| and asking for draft again gets draft | Draft | Draft | pass |
| a Japanese project name arrives at the page as the name it started as | 背景_日本語.json | 背景_日本語.json | pass |
| and so does a sentence with Japanese in it | Saved to 背景_日本語.json | Saved to 背景_日本語.json | pass |
| the notes travel whole, one to a line | One note.⏎And another. | One note.⏎And another. | pass |
| nothing is being exported, and the page is told so | false | false | pass |
| there is nothing outstanding in the project yet | false | false | pass |
| and once something is changed the page is told that too | true | true | pass |
| a path the window does not understand is not answered with a guess | nothing | nothing | pass |
| and neither is a time that is not a number | nothing | nothing | pass |
| a missing drawing does not take the rest of the shot down with it: the frame still comes back | 200 | 200 | pass |
| and it is honestly empty rather than filled in with a guess | every pixel transparent | every pixel transparent | pass |
| and the page is carrying the sentence that explains why it is empty | 2 of the files for "Cel" are not where the project expects them. The reference is kept as it is. Relink the asset to point it at the files, or put them back. Frames that cannot be found render as nothing rather than as a guess. | 2 of the files for "Cel" are not where the project expects them. The reference is kept as it is. Relink the asset to point it at the files, or put them back. Frames that cannot be found render as nothing rather than as a guess. | pass |
| which the page can only read because the refusal carries the same permission the picture does | * | * | pass |
| a four by four picture is sixty-four bytes | 64 | 64 | pass |
| and a half-transparent red one comes back as the red it was drawn as, not the darker red premultiplying it would give | 255, 0, 0, 128 | 255, 0, 0, 128 | pass |
| and every pixel of it is that same red | true | true | pass |

**39 of 39 checks pass.**

## What this does not cover

The window itself. These rows call the same function the request handler calls, with the same arguments, but no window is opened and no page is loaded, so a build that never registered the handler at all would pass this table. The photographs in `verification/B-08_window_shell.md` are what show that the window exists and draws; this table is what shows that what it draws is described correctly.

Nor does it cover whether the *picture* is right. Whether frame 24 looks like the shot is H-01's question and B-08a's; the question here is only whether the frame that comes back is the frame that was asked for, whether it is the size it says it is, and whether its colours arrive the way the page draws them. The last of those needs a picture the shot cannot supply, so the four-pixel rows draw their own.

Where a row says *the checkout*, the real value was this machine's copy of the repository, whose path differs on every machine. A line break inside a value is shown as ⏎ so that one row stays one row.
