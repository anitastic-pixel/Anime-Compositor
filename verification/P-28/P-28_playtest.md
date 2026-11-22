# P-28: one anime shot, end to end

A 5-second shot (120 frames, 24 fps, 1920x1080) built from scratch by a script and driven in the real app, the release build, on 2026-11-21. Nothing in `app/` or `src/` changed. The measured tables are in `P-28_measured_tables.md`, written by `tools/p28_media.py report`; every number below is copied from there.

## Scorecard

| check | result | why |
|---|---|---|
| Keys and time | **Pass** | All ten frames asked for out of order were byte for byte the frames drawn 0..119 in order, in two fresh windows. |
| Hold frames | **Pass** | Frames 48 and 49 are byte for byte the same, on the processor and on the card. |
| Blend modes and alpha | **Pass** | The subject's soft edge matches a mix in linear light to within 1 step of 255 (no dark fringe: a fringe would miss by up to 35). The Screen glow matches Screen in linear light (mean miss 0.20, largest 6); in sRGB it would miss by 7.18 on average. The Add flash was not checked by number, only by eye. |
| Effects | **Partial** | Blur, tint, fill, mask wipe, overshoot, particles and motion blur all draw. The red/blue split leaves a band along the frame's edge with no blue (10 px at the sides, 5 px top and bottom) on frames 48-54: bug 1. |
| Camera and shake | **Partial** | The shake is seen on the contact sheet (frames 50 and 54 are pushed off centre and settled by 55). Its size was not measured as a number. |
| Sound sync | **Pass** | In playback the click lands on sample 96000, which is frame 48.000 exactly. Exported (D-1140, added 2026-11-21): the WAV beside the PNG frames and the MP4's sound both start the click on sample 96000 = 2.000 s = frame 48.000. |
| Processor against card | **Pass** | Largest difference 2 steps of 255; no pixel off by more than 2, at all ten frames. |
| Draft against Full | **Pass** | Draft is a 480x270 picture drawn in 39.4 ms (median) against 74.6 ms for Full on the processor, and 0.8 ms against 2.5 ms on the card. The Draft pictures were kept but not compared with Full pixel by pixel. |
| Viewer against export | **Pass** | Measured after D-1140 (added 2026-11-21), exported through `into=` by the release build of this unit: the PNG frames 0, 48, 50, 90 and 119 are byte for byte the viewer's Full frames (largest difference 0, alpha included). The MP4 at High is the same picture with the codec's loss: mean difference 1.05 to 2.09, largest 139 (frame 0), on sharp coloured edges; recorded as the codec's, not a failure. |
| Speed | **Partial** | The card plays three loops from memory with 0 frames passed over. The processor passes over 87 frames in three loops, although every frame is already in memory (bug 3). |

## What was built (Phase 1: can the app do each beat?)

| beat | what the shot asks for | how it was built | native or faked |
|---|---|---|---|
| Build-up, frames 0-47 | plate pushes in 100 to 108% | image sequence layer, Scale keys, Gaussian Blur 2 px | native |
| | subject eases in | still image, Position keys with an ease-out curve | native |
| | title wipes on, scale overshoot | text layer, an animated rectangle mask with a 24 px feather, a Scale curve whose handle goes past 1 | native |
| | the reference cels on 2s | the `Fixtures/reference_shot/layer2` cels as a sequence on 2s | native |
| Hit, frames 48-54 | subject holds on 48-49 | hold keys, and the plate shows one drawing on both frames | native |
| | white flash | adjustment layer, Fill white, Add, opacity keys | native |
| | 15 px shake dying over 12 frames | an expression on a null that every picture layer is parented to | native (there is no camera here: the shake moves the null) |
| | red/blue split | Chromatic Aberration on an adjustment layer | native, with bug 1 |
| Aftermath, frames 55-119 | Screen glow copy | a copy of the subject, Tint orange, Gaussian Blur 40, Screen at 80% | native |
| | particles with motion blur | Particle Space on a solid set to Add, layer motion blur on, composition motion blur 180° | native; the sparks are 2-4 px each, small at the size chosen, so they are hard to see on the contact sheet |
| | sound | the click on an audio layer, starting at 2.000 s | native |

Nothing had to be faked. The shake moves a null rather than a camera, which is the usual way to do it in After Effects too.

## Pictures

- `P-28_contact_sheet.png`: frames 0, 24, 47, 48, 49, 50, 54, 55, 90 and 119. On 48, 49, 50 and 54, look at the frame's edge: that is bug 1.
- `P-28_edge_4x.png`: the subject's soft edge at 4x. Left is the app, middle is a correct mix, right is what a dark fringe would look like. Left and middle should look the same.
- `P-28_viewer_vs_export.png` (added 2026-11-21, after D-1140): viewer, PNG export and MP4 export side by side at frames 0, 48, 50, 90 and 119. All three columns should look the same.

## Bugs and friction

| # | what the owner sees | steps | where | cause |
|---|---|---|---|---|
| 1 | A thin yellow-orange line round the whole frame while the red/blue split is on (frames 48-54). | Put Chromatic Aberration (amount 12, centre 50,50) on an adjustment layer over a full-frame picture. | `src/layer_fx.rs:1067` | Blue is read from a point further out than the pixel. Near the frame's edge that point is outside the picture, which reads as nothing, so blue becomes 0 there while the pixel stays opaque. Measured: 40600 pixels, a band of 10 px at the sides and 5 px top and bottom; the card does the same. |
| 2 | Export can only be started through the system's folder dialog, so no test tool can check an export without a person. | `/export?format=png&wav=on`, then fill the dialog "Export the frames into a folder" by UI Automation. | `app/src/main.rs:16377` (`ask_where_to_export`), dialog at 16386-16387 | Try 1: the box with id 1152 is a combo box and does not accept text. Try 2: no edit box with that id is under the dialog. Try 3: the only editable box found was the dialog's Search box. The export never started, so viewer against export and the exported sound are not measured. Whether this is a limit of the test tool or of the dialog was not found. |
| 3 | Played at Full on the processor, the shot stutters even once all 120 frames are in memory. | Play the shot at Full on the processor; let it loop three times. | frames passed over are counted at `app/src/main.rs:1701-1705`; cause not found | The window's own time for a frame from memory is 1.7 ms (median), yet only 274 frames were asked for in 15.04 s (about 18 a second) and 87 were passed over. On the card: 0 passed over. So the time is lost between the window and the page. Sending a whole 8 MB frame to the page and painting it there is the likely place, but this was not measured. The window was off the screens, which may matter. |
| 4 (friction) | The session log's "frames" counts every request, including the card's repaints (2156 rows in 15 s of playback, 30 rows for 10 frames). It is not the number of frames shown. | Read `/session-log` after playing on the card. | `app/src/main.rs:18934` | The log has one row per frame answer, and the card asks again when it paints. Folded into D-1141. |

Proposed fixes, each fixture first, are D-1139, D-1140 and D-1141 in `Markdown/14_Decisions_Risks.md`. D-1140 was accepted and built on 2026-11-21 (B-1003), and the export checks above were then run; D-1139 and D-1141 are PROPOSED.

## Machine and build

AMD Ryzen 9 9900X 12-Core Processor, 12 cores 24 threads, 61.5 GB, Windows 11 Education 10.0.26300; NVIDIA GeForce RTX 4070 Ti SUPER, driver 610.88, Vulkan. Release build of e47d4bd9 at `target/lanes/p28/release/anime_compositor_app.exe` (measured from checkout 89fec3d2, which adds only the P-28 script and project). Every window was a copy with its own web view profile, placed off the screens (`ANIME_COMPOSITOR_BACKGROUND=1`). Other programs the owner had open were not recorded. The two exports (added 2026-11-21) were made by the release build of B-1003 at `target/lanes/d1140/release/anime_compositor_app.exe`, started with `ANIME_COMPOSITOR_TEST_EXPORT=1`, and compared with the viewer frames kept from the first run; B-1003 changes only how the folder is named, not drawing.

## Not measured, and why

- The Add flash by number: no variant without the flash was rendered. It was judged by eye only.
- The size of the shake: judged by eye only.
- Sound heard through the speakers: the check reads the window's own playback mix (`/mix`), which the page plays at the frame it names. Nothing listened at the speaker.
- `tools/app_stability.ps1` was not used: it drives the owner's `target\release` build with the real mouse.

## Re-run

```
python tools/p28_media.py media
python tools/p28_media.py project
python tools/p28_media.py measure <path to a release anime_compositor_app.exe>
python tools/p28_media.py exports <same exe>    (only the export step; through into=, D-1140)
python tools/p28_media.py report
```

Media and raw frames go to `target/p28` (not committed). `measured.json`, the tables and the pictures go to `verification/P-28`.
