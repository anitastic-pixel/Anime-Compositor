# D-323 / B-202: Time Remapping

From D-308, approved by the owner on 2026-10-04, after P-26, with ADR-021. In tutorial 1 (the anime ring), the shot is sped through a burst and then plays at real speed by After Effects' **Time Remap**, and a composition layer is frozen by one hold key on it. Neither existed here: a layer could only be stretched evenly, and D-314's Freeze Frame held only a drawn layer.

## What changed

- A drawn or composition layer may have a **Time Remap**: keys, in source frames, of which frame of its source the layer shows. It has its own row under the layer on the timeline, above Blending and Transform, and is keyed like any other setting (linear, hold or eased keys).
- **Enable Time Remapping** is in the Layer menu, the layer's right-click menu and the inspector (a **Time remap** checkbox), and on **Ctrl+Alt+T**:
  - On, it writes two keys that change no frame: the source frame at the layer's start and at its last frame.
  - Off, it takes the remap and its keys away.
  - Each is one step to undo.
- **Freeze Frame** now works on a composition layer, or any layer with a Time Remap: the keys are replaced by one hold key holding the frame under the playhead.
- Moving a layer in time moves its remap keys with it. Trimming its start does not change what a frame shows.
- A file from before has no Time Remap. It draws exactly as before and is saved as it was; a remap is saved as `time_remap`.
- A Time Remap on a layer with no frames of its own (solid, null, adjustment), a key that is not one number, or an expression on it, is refused when the file is read. A key while it is off, an expression, and turning it on for a solid are refused with a sentence, changing nothing.

## Checks (cargo test)

All in `verification/D-323_time_remap_table.md`, **180 of 180 pass**:

| Check | Expected | Got |
|---|---|---|
| FX-TREMAP-001 to 009 and 030: which source frame each frame shows (straight, burst, one key, backwards, hold, eased, stretched) | every time as `Fixtures/time_remap/expected_time_remap.json`, within 1e-12 | pass |
| FX-TREMAP-010 to 019: the frames, drawn and composition layers, with frame blending | every pixel as the expected file, within 1e-6 | pass, largest difference 0 |
| The ten files opened and saved | the same file back, Time Remap and keys included | pass |
| An old file with no Time Remap | saved with no `time_remap` | pass |
| FX-TREMAP-050 to 055: remap on a solid, null or adjustment layer; a pair of numbers; an expression; a bare number | refused when read, `PROJECT_SCHEMA_INVALID` | pass |
| FX-TREMAP-040 to 044: Enable Time Remapping | the two keys expected; every frame byte for byte as before; one undo takes it off | pass |
| Off again | the remap gone, the file written without `time_remap` | pass |
| FX-TREMAP-045 to 047: Freeze Frame | one hold key as expected; every frame the playhead's; one undo puts it back | pass |
| FX-TREMAP-048: the layer moved 2 frames | keys move to 2 and 8, values kept | pass |
| Start trimmed from 0 to 2 | frame 4 byte for byte as before | pass |
| A key or value while off, a freeze past the layer's end, an expression, Enable on a solid | refused with a sentence, nothing changes | pass |
| Frames 012@3, 014@3, 017@5 in tiles of 1 and of 64 | byte for byte the same | pass |

## Pictures

In `verification/D-323 pictures/`.

Pictures 1 to 4 are drawn by the test (B-202): a made-up ball in seventeen drawings (`ball_0001.png` to `ball_0017.png`, each one step further right), placed as a drawn layer, and twelve frames of it laid side by side, frame 0 on the left.

Pictures 5 to 9 are the app's test copy, not your app (`target/p26/d323_steps.js`): a new 640 by 360 composition "Retime", a dark Ground, and a 40 by 40 orange Dot keyed from x 40 at frame 0 to x 600 at frame 23, precomposed as "Shot". Your Unsaved work and window were put back after the run.

| Picture | What it shows | Look for | Pass? |
|---|---|---|---|
| `as_drawn.png` | No remap | The ball one step further each frame, drawings 0 to 11 | |
| `burst_then_real_speed.png` | Tutorial 1's keys: 0 at 0, 8 at 4, 16 at 12 | The ball jumps two steps a frame for the first four frames, then one step a frame | |
| `backwards.png` | Keys 11 at 0 and 0 at 11 | The ball going right to left | |
| `frozen_at_5.png` | Freeze Frame at 5 | The same ball, drawing 5, in every frame | |
| `app_1_before_frame6.png` | The app at frame 6, no remap | The dot about a third of the way across; no Time Remap row under Shot | |
| `app_2_remap_on_frame6_same.png` | Ctrl+Alt+T's command: Time Remapping on | The dot in exactly the same place; a **Time Remap** row under Shot with keys at frames 0 and 23; the inspector's Time remap box ticked | |
| `app_3_burst_frame6_shows_16.png` | A Time Remap key of 16 at frame 6 | At frame 6 the dot is two thirds across (source frame 16); a third key at frame 6; the row reads 16 f | |
| `app_4_frozen_at_3_frame20.png` | Freeze Frame at frame 3, then the playhead at 20 | At frame 20 the dot still a third across (frozen on source frame 8); one key only, at frame 3 | |
| `app_5_undone_frame6.png` | Ctrl+Z three times | Back to picture 5: the dot a third across at frame 6, no Time Remap row, the box unticked | |

The app read back keys 0 at 0 and 23 at 23 after turning it on; 0 at 0, 16 at 6 and 23 at 23 after the key; one hold key, 8 at 3, after the freeze; and no remap after the three undos.
