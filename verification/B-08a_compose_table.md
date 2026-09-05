# B-08a — a frame assembled and rendered from a project file

**45 of 45 checks passed.**

Produced by `tests/b08a_compose.rs`. Covers document 20's evaluation order at one frame, steps 1 to 8.

## What this is

Until now every picture this build produced was assembled by a test. This one is assembled from a project. The reference shot is built through the application's own commands, written out as `verification/B-08a_project.json`, read back from that text, and rendered. The four frames in `verification/B-08a_frames/` came out of the end of that chain.

## What to look at

- **`B-08a_frames/frame_000.png`** — the shot at frame 0, all four layers. The yellow bar is at the far left and the cyan bar at the bottom right; that is drawing 0 of layer 3.
- **`B-08a_frames/frame_100.png`** — the same shot a hundred frames later. The yellow bar has moved right by two bar-widths and the green square has moved. If these two images were the same, the exposure sheet would not be reaching the renderer.
- **`B-08a_frames/frame_014.png`** — layer 3 is absent here, and that is correct. Frame 14 asks for drawing 7, which the fixture deliberately does not contain. The bars are gone and nothing has been substituted for them. A build that quietly held drawing 6 for two more frames would look better and be wrong.
- **`B-08a_frames/frame_165.png`** — layer 4 re-exposes drawing 11 here, after drawing 14. Cel work does this constantly; an implementation that assumed drawing numbers only go up would show drawing 15 and this frame would be in the wrong place.

## What a wrong result looks like

The table's expected values were worked out from the fixture's own record of how the cels were drawn — where each drawing puts its bar, its blob and its square — before the code was run. A layer resolving to the wrong drawing shows up as a bar of alpha where the table expects nothing, which is why almost every row samples two points: one where the mark should be and one where the previous or next drawing's mark would be.

## What is not here

There is no viewer, no playback and no export. This is the headless half of B-08: it turns a project and a frame number into a picture, and stops there. Masks, effects and track mattes are parked (document 23); a layer carrying a matte renders without it and says so rather than pretending.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the project read back from its own file has four layers | `4` | `4` | pass |
| in composition order, bottom of the stack first | `layer1, layer2, layer3, layer4` | `layer1, layer2, layer3, layer4` | pass |
| layer 3's asset has no drawing 7, because that file is a deliberate defect of the shot | `0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11` | `0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11` | pass |
| frame 0 plans four layers | `4` | `4` | pass |
| and hands the renderer them bottom first, so layer 4 composites last | `layer-1, layer-2, layer-3, layer-4` | `layer-1, layer-2, layer-3, layer-4` | pass |
| at the composition's own extent | `1920x1080` | `1920x1080` | pass |
| frame 0 reports nothing wrong | `` | `` | pass |
| layer 3 at frame 0 is drawing 0: its yellow bar is where drawing 0 puts it | `R 1.000000, B 0.000000, A 1.000000` | `R 1.000000, B 0.000000, A 1.000000` | pass |
| and drawing 2's position is empty, so this is not simply every bar at once | `0.000000` | `0.000000` | pass |
| the same drawing's cyan bar is in the bottom right, at full alpha | `R 0.000000, B 1.000000, A 1.000000` | `R 0.000000, B 1.000000, A 1.000000` | pass |
| layer 4 keeps its exactly-50% interior through the conversion to the working space | `0.501961` | `0.501961` | pass |
| and is transparent outside the square it was painted in | `0.000000` | `0.000000` | pass |
| layer 3 at frame 100 is drawing 2, one hundred frames of exposure later | `R 1.000000, B 0.000000, A 1.000000` | `R 1.000000, B 0.000000, A 1.000000` | pass |
| and drawing 0's position is now the empty one | `0.000000` | `0.000000` | pass |
| frame 165 re-exposes layer 4's drawing 11, which an earlier exposure already used | `0.501961` | `0.501961` | pass |
| and it is not drawing 15, which is what counting exposures forward would have given | `0.000000` | `0.000000` | pass |
| frame 13 opens layer2_桜_013.png, whose name is not ASCII | `1.000000` | `1.000000` | pass |
| frame 14 plans three layers, not four | `3` | `3` | pass |
| and it is layer 3 that is missing, the others untouched | `layer-1, layer-2, layer-4` | `layer-1, layer-2, layer-4` | pass |
| the frame says why, naming the drawing rather than the file | `MEDIA_SEQUENCE_GAP: Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.` | `MEDIA_SEQUENCE_GAP: Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.` | pass |
| no neighbouring drawing is substituted: layer 3 contributes nothing at all | `layer-1, layer-2, layer-4` | `layer-1, layer-2, layer-4` | pass |
| over frames 14 and 15 with a limit of one, the second is summarised, not repeated | `2` | `2` | pass |
| and the summary names both frames and how many were held back | `Frames 14 to 15. 1 further identical warnings were not logged individually.` | `Frames 14 to 15. 1 further identical warnings were not logged individually.` | pass |
| a layer's opacity is read at the frame, not at zero: frame 0 | `0.000000` | `0.000000` | pass |
| halfway between the two keyframes, frame 12 | `0.500000` | `0.500000` | pass |
| and at the second keyframe, frame 24 | `1.000000` | `1.000000` | pass |
| the anchor point lands on the layer's position | `(960.0, 540.0)` | `(960.0, 540.0)` | pass |
| and the layer's origin lands anchor-distance away, scaled | `(760.0, 440.0)` | `(760.0, 440.0)` | pass |
| an asset recorded as already linear is not put through the transfer function again | `0.862745` | `0.862745` | pass |
| and the same drawing under the default sRGB tag is darker, because it is converted | `darker` | `darker` | pass |
| an asset recorded as premultiplied is not multiplied by its alpha a second time | `brighter` | `brighter` | pass |
| a frame past the end of the composition is refused, not clamped to the last one | `COMMAND_INVALID_VALUE` | `COMMAND_INVALID_VALUE` | pass |
| and one before the start likewise | `COMMAND_INVALID_VALUE` | `COMMAND_INVALID_VALUE` | pass |
| frame 239 is inside the shot and renders, so the refusal is not off by one | `4` | `4` | pass |
| a composition the project does not have is refused | `COMMAND_TARGET_MISSING` | `COMMAND_TARGET_MISSING` | pass |
| a layer switched off is left out of the frame | `layer-1, layer-3, layer-4` | `layer-1, layer-3, layer-4` | pass |
| and switching it off is not a fault, so nothing is reported | `` | `` | pass |
| a track matte, which this build does not render, is reported rather than ignored | `PROJECT_FEATURE_UNSUPPORTED: Layer layer4 has a track matte, which this build does not render.` | `PROJECT_FEATURE_UNSUPPORTED: Layer layer4 has a track matte, which this build does not render.` | pass |
| the layer still draws; what it loses is the matte, not itself | `4` | `4` | pass |
| a layer naming an asset the project does not have is reported and left out | `PROJECT_SCHEMA_INVALID: Layer layer3 names asset asset-layer3, which is not in the project.` | `PROJECT_SCHEMA_INVALID: Layer layer3 names asset asset-layer3, which is not in the project.` | pass |
| a file the project points at that is not on disk is a different fault, named differently | `MEDIA_MISSING: layer3/layer3_000_moved_away.png is not where the project says it is.` | `MEDIA_MISSING: layer3/layer3_000_moved_away.png is not where the project says it is.` | pass |
| the rendered frame is the composition's size | `1920x1080` | `1920x1080` | pass |
| where layer 3 is opaque, the composite is layer 3, because it is above layer 1 | `R 1.000000, B 0.000000, A 1.000000` | `R 1.000000, B 0.000000, A 1.000000` | pass |
| tile size is a speed setting: 64 and 128 give the same frame, byte for byte | `identical` | `identical` | pass |
| four frames were written for the owner to look at | `4` | `4` | pass |
