# B-08 preview: resolution selection and the playback clock

The two decisions the owner made on 2026-09-05, turned into behaviour a table can hold. **D-33**: the preview opens at draft resolution and says so. **D-32**: playback holds real time, drops the frames it cannot deliver, and reports how many. Produced by `tests/b08_preview.rs` from `verification/B-08a_project.json`, which is a real project file.

The row that matters most is the fourth from the top of the second group: **a full-resolution preview of frame 100 is the exported frame, sample for sample**. The preview is composed through the display path and the export is written by the export path, and all 8,294,400 samples agree. The row after it is the control that makes that mean something — the same comparison against a draft preview, which must not agree.

`verification/B-08_preview_draft_100.png` is that draft frame, 480 by 270. Beside `verification/B-08a_frames/frame_100.png`, which is the same frame at full size, it is the same picture and a quarter of the width. There is still no window; that is the rest of B-08.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the preview opens at the resolution D-33 chose | Draft | Draft | pass |
| at draft, R-06a's indication that preview differs from export is on | Draft, differs from export: true | Draft, differs from export: true | pass |
| at full, there is nothing to indicate, because nothing differs | Full, differs from export: false | Full, differs from export: false | pass |
| draft is SP-05's measured extent for this composition | 480x270 | 480x270 | pass |
| full is the composition's own extent, unchanged | 1920x1080 | 1920x1080 | pass |
| a composition that does not divide by four keeps its last column and row | 481x271 | 481x271 | pass |
| at draft, the layers are scaled with the extent rather than cropped by it | 480,270 480,270 480,270 480,270 | 480,270 480,270 480,270 480,270 | pass |
| at full, the layers are left exactly as the export sees them | 1920,1080 1920,1080 1920,1080 1920,1080 | 1920,1080 1920,1080 1920,1080 1920,1080 | pass |
| a full-resolution preview of frame 100 is the exported frame, sample for sample | 0 of 8294400 samples differ | 0 of 8294400 samples differ | pass |
| the frame that comparison ran on is the composition's own extent | 1920x1080 | 1920x1080 | pass |
| a draft preview of the same frame is not the exported frame | different lengths: 518400 and 8294400 | different lengths: 518400 and 8294400 | pass |
| the draft frame is the extent draft claims | 480x270 | 480x270 | pass |
| at draft, a frame with a missing drawing has no layer 3 paint anywhere in it | no layer 3 paint | no layer 3 paint | pass |
| at draft, the frame either side of that gap does paint layer 3 | layer 3 paint present | layer 3 paint present | pass |
| at rest, before playback, the work area's first frame is shown | 10 | 10 | pass |
| a nanosecond before the first frame ends, frame 0 is still on screen | 0 | 0 | pass |
| a nanosecond after it ends, frame 1 is | 1 | 1 | pass |
| just past halfway through frame 0, frame 0 is still on screen, not frame 1 | 0 | 0 | pass |
| at 23.976 the frame boundary is exact, not nearly right | 0 then 1 | 0 then 1 | pass |
| asked once every third frame time, each answer is three frames on and two were passed over undrawn | 0+0 3+2 6+2 9+2 | 0+0 3+2 6+2 9+2 | pass |
| and that run reports what it cost | Played 4 frames in real time and dropped 6 to keep the timing true. Step through the frames to see every drawing, or switch the preview to draft resolution. | Played 4 frames in real time and dropped 6 to keep the timing true. Step through the frames to see every drawing, or switch the preview to draft resolution. | pass |
| the dropped frames are counted, not hidden | shown 4, skipped 6 | shown 4, skipped 6 | pass |
| a machine that does keep up drops nothing, and says so | Played 3 frames in real time. No frames were dropped. | Played 3 frames in real time. No frames were dropped. | pass |
| playback loops inside the work area rather than running past its end | 10 13 12 11 10 | 10 13 12 11 10 | pass |
| if the clock the caller supplies runs backwards, the frame is held and nothing is counted as dropped | frame 6, skipped 0 | frame 6, skipped 0 | pass |

**25 of 25 checks pass.**
