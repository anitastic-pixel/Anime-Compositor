# B-10 full shot: 240 frames exported twice

Document 15 asks B-10 for "the exported 240-frame sequence, plus a byte comparison of two consecutive exports proving determinism". This is that comparison. Produced by `tests/b10_full_shot.rs`, which is `#[ignore]`d in normal runs and run by name.

The frames themselves are not committed: one frame of the assembled shot is about two megabytes, so the sequence is 480 megabytes across the two passes. They are written to `target/b10_full/pass1` and left there, so they can be opened or flipped through after a run. What is committed in their place is `B-10_contact_sheet.png`, which is all 240 frames at a twelfth scale in a 16 by 15 grid, reading left to right and top to bottom.

**Ten pairs of cells in the contact sheet are missing their yellow bar. That is correct.** Layer 3 has no drawing 7 — a deliberate defect of the reference shot per `Fixtures/reference_shot/README.md` — and the twenty frames that ask for it are written with that layer left out and warned about, which is D-28's recorded override. Nothing was substituted for the missing drawing.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the whole shot is asked for and the whole shot is written | 240 frames requested, 240 written, completed | 240 frames requested, 240 written, completed | pass |
| the second export asks for and writes the same number of frames | 240 frames requested, 240 written, completed | 240 frames requested, 240 written, completed | pass |
| the first and last file are named for their own frame, four digits wide | shot_0000.png and shot_0239.png | shot_0000.png and shot_0239.png | pass |
| every one of the 240 frames is byte for byte what a second export of the same project produced | all 240 pairs identical | all 240 pairs identical | pass |
| the byte comparison can fail: each frame paired with the next frame's file instead | 0 of the 239 neighbouring pairs match | 0 of the 239 neighbouring pairs match | pass |
| the frames whose drawing the shot deliberately does not contain are the twenty the cadence predicts | 14, 15, 38, 39, 62, 63, 86, 87, 110, 111, 134, 135, 158, 159, 182, 183, 206, 207, 230, 231 | 14, 15, 38, 39, 62, 63, 86, 87, 110, 111, 134, 135, 158, 159, 182, 183, 206, 207, 230, 231 | pass |
| twenty affected frames are reported as three in full and one summary, per D-25 | 4 gap diagnostics | 4 gap diagnostics | pass |
| the summary names every affected frame and counts the ones it did not log | Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134 to 135, 158 to 159, 182 to 183, 206 to 207, 230 to 231. 17 further identical warnings were not logged individually. | Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134 to 135, 158 to 159, 182 to 183, 206 to 207, 230 to 231. 17 further identical warnings were not logged individually. | pass |
| no parked feature was bypassed, so the fidelity flag stays down; the missing drawings are reported as the warnings above instead | fidelity incomplete: false | fidelity incomplete: false | pass |
| a reader of the export report is told about the last affected frames, not just the first ones logged | the report names frames 230 to 231: true | the report names frames 230 to 231: true | pass |
| a frame missing its drawing has none of layer 3's paint anywhere in it | frame 14: no layer 3 paint, frame 15: no layer 3 paint | frame 14: no layer 3 paint, frame 15: no layer 3 paint | pass |
| the frames either side of the gap do contain layer 3's paint | frame 13: layer 3 paint present, frame 16: layer 3 paint present | frame 13: layer 3 paint present, frame 16: layer 3 paint present | pass |
| every exported frame reads back as a 1920 by 1080 PNG | 240 of 240 readable | 240 of 240 readable | pass |
| the contact sheet holds all 240 frames in a 16 by 15 grid | 2560 by 1350, 240 cells | 2560 by 1350, 240 cells | pass |

**14 of 14 checks pass.**
