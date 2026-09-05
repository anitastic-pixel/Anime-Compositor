# T-08 — exporting a frame range to a PNG sequence

**47 of 47 checks passed.**

Produced by `tests/t08_export.rs`. Covers R-09: a declared inclusive frame range, written as PNGs with a chosen bit depth, naming and alpha policy, with failure reported and cancellation supported between frames.

## What this is

B-08a made a picture in memory. This writes the picture to files, and it is the first output this build produces that is meant to leave the application. The six frames in `verification/T-08_frames/` were exported by the code these checks describe.

## What to look at

- **`T-08_frames/shot_0014.png`** — read this one first, because it should look empty and that is correct. Frame 14 asks for drawing 7 of layer 3, which your fixture deliberately does not contain. The file was written, it is the right size, and it has nothing in it. Nothing was substituted.
- **`T-08_frames/shot_0012.png`** and **`shot_0016.png`** — the bar moves right by one bar-width between them, because the layer runs on twos and these are two drawings apart. The bar is half-transparent: the layer is exported at 50% opacity on purpose, because that is what makes the alpha rules in document 21 visible in a file.
- **`T-08_export_table.md`**, this table.

## The rule that will matter most to you

By default, **an export whose range contains a frame with no drawing is refused before a single file is written**, and it tells you exactly which frames are missing. That is document 07's rule and this build obeys it. Writing those frames anyway, with the layer left out, is possible but has to be asked for, and it still warns.

Document 28 says the opposite for the same situation. That conflict is registered as **D-28** and is yours to settle; nothing here depends on which way it goes except one default.

## What is not here

**There is no video file.** R-09 asks for an image sequence and that is what was built. A video needs an encoder, and an encoder is a dependency and a licence decision, which is yours rather than the code's. That question is registered rather than answered.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| frames 0 to 239 is 240 files, because document 07 includes both ends | `240` | `240` | pass |
| a naming pattern with no frame number is refused before anything is written | `COMMAND_INVALID_VALUE, 0 files` | `COMMAND_INVALID_VALUE, 0 files` | pass |
| and the refusal says why, in the owner's words | `The output naming everyframe.png contains no frame number, so every frame would be written to one file.` | `The output naming everyframe.png contains no frame number, so every frame would be written to one file.` | pass |
| a range that ends before it starts is refused, not silently swapped | `Failed, COMMAND_INVALID_VALUE, 0 files` | `Failed, COMMAND_INVALID_VALUE, 0 files` | pass |
| a refused export never reports success | `false` | `false` | pass |
| an export whose range contains a missing drawing is blocked, and writes nothing | `Blocked, 0 files` | `Blocked, 0 files` | pass |
| a blocked export does not report success | `false` | `false` | pass |
| it counts the frames it could not draw against the frames asked for | `4 of the 48 frames asked for have a drawing that is missing, so nothing was exported.` | `4 of the 48 frames asked for have a drawing that is missing, so nothing was exported.` | pass |
| and names them, as ranges rather than as a list of forty-eight | `Frames 14 to 15, 38 to 39.` | `Frames 14 to 15, 38 to 39.` | pass |
| under an identifier that says what happened | `EXPORT_BLOCKED_MISSING_MEDIA` | `EXPORT_BLOCKED_MISSING_MEDIA` | pass |
| the same six frames export when the missing drawing is chosen to be transparent | `Completed, 6 files` | `Completed, 6 files` | pass |
| and that is the only status that reports success | `true` | `true` | pass |
| the files are named from the frame number, padded to the pattern's width | `shot_0012.png, shot_0013.png, shot_0014.png, shot_0015.png, shot_0016.png, shot_0017.png` | `shot_0012.png, shot_0013.png, shot_0014.png, shot_0015.png, shot_0016.png, shot_0017.png` | pass |
| the two frames with no drawing are warned about, once each | `MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP` | `MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP` | pass |
| and each warning names its own frame and the drawing that is missing | `14 asks for 7, 15 asks for 7` | `14 asks for 7, 15 asks for 7` | pass |
| a missing drawing is not a bypassed feature, so fidelity is not marked incomplete | `false` | `false` | pass |
| an exported frame is the composition's own size | `1920x1080` | `1920x1080` | pass |
| frame 12 shows drawing 6, whose bar covers x 960 to 1119 | `R 255, B 0, A 128` | `R 255, B 0, A 128` | pass |
| and nothing is there two bar-widths to the left, where drawing 4's bar would be | `R 0, B 0, A 0` | `R 0, B 0, A 0` | pass |
| frame 14 has no drawing, and nothing was substituted for the one that is missing | `R 0, B 0, A 0` | `R 0, B 0, A 0` | pass |
| an exported file says what wrote it | `anime_compositor export (R-09)` | `anime_compositor export (R-09)` | pass |
| and what its numbers mean, so a person opening it later is not guessing | `sRGB IEC 61966-2-1, 8 bits per channel / Straight / 12` | `sRGB IEC 61966-2-1, 8 bits per channel / Straight / 12` | pass |
| asking for one frame, 12 to 12, writes one file | `shot_0012.png` | `shot_0012.png` | pass |
| written premultiplied, the same pixel keeps alpha folded into the colour: linear 0.5 through the sRGB curve is 188, not 255 | `R 188, B 0, A 128` | `R 188, B 0, A 128` | pass |
| asked for sixteen bits, the file declares sixteen bits | `16` | `16` | pass |
| and the same straight pixel is the same colour at the deeper precision | `R 65535, B 0, A 32768` | `R 65535, B 0, A 32768` | pass |
| the eight-bit file declares eight, so the two are not the same file with a label | `8` | `8` | pass |
| the samples in the exported file are exactly what the renderer produced, byte for byte | `identical` | `identical` | pass |
| exporting the same frame twice produces the same file, byte for byte | `identical` | `identical` | pass |
| a composition starting at -12 and lasting 24 frames exports 24 files (FX-TIME-004) | `Completed, 24 files` | `Completed, 24 files` | pass |
| a negative frame number keeps its sign in front of the padded digits (D-29) | `neg_-0012.png` | `neg_-0012.png` | pass |
| and the last file is the last frame of the range, included | `neg_0011.png` | `neg_0011.png` | pass |
| an export that cannot write a file stops, and keeps the frames it finished | `Failed, 2 written` | `Failed, 2 written` | pass |
| it never reports success | `false` | `false` | pass |
| it names the frame and the path that failed | `EXPORT_WRITE_FAILED: Frame 14 could not be written.` | `EXPORT_WRITE_FAILED: Frame 14 could not be written.` | pass |
| and says how many frames had been written when it happened | `true` | `true` | pass |
| the two frames it did write are still on disk and still readable | `shot_0012.png 1920x1080, shot_0013.png 1920x1080` | `shot_0012.png 1920x1080, shot_0013.png 1920x1080` | pass |
| an export cancelled before it starts writes nothing and claims nothing | `Cancelled, 0 files, EXPORT_CANCELLED` | `Cancelled, 0 files, EXPORT_CANCELLED` | pass |
| a cancelled export is not a successful one | `false` | `false` | pass |
| an export cancelled while it is running stops and reports it | `Cancelled, EXPORT_CANCELLED` | `Cancelled, EXPORT_CANCELLED` | pass |
| it stopped short of the hundred frames asked for | `true` | `true` | pass |
| and every frame it had finished is a whole file, because the check is between frames | `all 320x180` | `all 320x180` | pass |
| the frames it wrote are still on disk after the stop | `2` | `2` | pass |
| a frame drawn without a parked feature still exports | `Completed, 1 written` | `Completed, 1 written` | pass |
| but the report says the fidelity is incomplete | `true` | `true` | pass |
| and so does the file itself, where it cannot be separated from the picture | `incomplete: a layer carrying a parked feature was drawn without it` | `incomplete: a layer carrying a parked feature was drawn without it` | pass |
| a frame with nothing bypassed carries no such tag | `there is no such tag` | `there is no such tag` | pass |
