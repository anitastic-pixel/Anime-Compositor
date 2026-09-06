# Hardening: deliberately breaking the code to see whether the tests notice


A passing test proves nothing on its own. It might be checking something that cannot fail.

This is the check on the checks. Each unit already merged is broken on purpose, one small
change at a time, in ways a real mistake would look like: an off-by-one, a comparison the wrong
way round, a step skipped. The build is then run. If the tests still pass, the mistake would
have reached the owner unnoticed, and the fixture is too weak. Every break is undone
immediately afterwards; nothing here is left in the code.

The rule this pass follows, from `NIGHT_RUN.md`: **if a break survives, the fixture is fixed,
never the assertion.** No expected value was changed, no tolerance was loosened, and nothing in
`Fixtures/` was touched.

**172 breaks were made across seventeen units. All 172 were caught.**

That number is the total of three passes. The first covered five units and made 55 breaks, six of
which got through before the import fixture was strengthened. The second covered the remaining
four units - the document model, transform, the trace and persistence - and made 52 breaks,
twelve of which got through before their fixtures were strengthened. Both sets of survivors are
listed at the end, with what was added for each. The third is B-08a, the unit that assembles a
frame from a saved project: 10 breaks, all caught first time, and no survivors. Three of its ten
were aimed at the asset record's colour and alpha interpretation, which the first draft of its
table could not have caught; three rows were added before the pass rather than after it, and they
are named in the table below. The fourth is T-08, the export writer: 12 breaks, all caught, and
no survivors - but two of them were caught by a crash rather than by a named row, because a
build that fails to write a file leaves the test's own PNG reader with nothing to open. Both of
those places now report a sentence, so the owner sees a table with one line marked wrong instead
of a stack trace. The fifth is the export half of T-07, which joins the file format to the
exported picture: 8 breaks, all aimed at ways a save or an open could lose something the
renderer reads, all caught, no survivors. One of the eight was caught by a crash - a project
that came back with no exposure sheet at all left the test indexing an empty list - and that
place now reports a sentence too. The sixth is H-01, which is not a unit of the build at all
but a second compositor written from document 21 to argue with the first: 6 breaks, all caught,
no survivors, and every one of them a fault that no other table in this project could see - a
frame drawn one pixel to the right, an sRGB exponent off by two thousandths, red and blue
swapped, the top layer left out, the last row of every tile never drawn, and cels composited
with straight colour instead of premultiplied. The seventh is H-02, which does the same for a
frame whose layers have been moved, scaled and faded: 6 breaks, two of which got through the
first time and are listed below with what was added, and all six caught after the fixture was
strengthened. The eighth is B-10, the whole 240-frame shot exported twice: 6 breaks, one of
which survived two attempts and is the most instructive result of the pass, described below. The
ninth is B-11, and it is the only unit here whose breaks are not made in code at all - the thing
being checked is a document, `docs/DEPENDENCIES.md`, and the failure worth preventing is that
record quietly ceasing to describe the build, so the record itself is what gets broken: a crate
dropped, a stale version, a crate the build does not use, a dependency chosen on purpose recorded
as one that arrived underneath another, a blank licence, and only one of the two versions listed
for the crate the build resolves twice - which is the exact defect the check itself shipped with
for an hour, a comparison keyed on crate name alone, found by this record failing against the
build and fixed in the check rather than in the record. A seventh break is made differently from
every other break in this report: it deletes no line, but moves a crate's archived licence
directory out of the way, because the row it aims at asks whether a directory exists and nothing
that can be written into a file can make that false. The tenth is B-08, the preview path: 10
breaks, all caught, no survivors, though one break was withdrawn as unobservable and the code
comment it disproved was corrected rather than the fixture strengthened - that story is told in
full in its own section below.


## B-02 colour and alpha

`src/color.rs, src/composite.rs, src/lib.rs`, checked by `tests/b02_color_alpha.rs`, whose table is `verification/B-02_fixture_table.md`.

**14 of 14 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| C1 | the sRGB curve's linear-segment threshold is moved | yes | `T09-S2L-d: expected 3.9359395040889670e-3 got 3.86996916e-3` |
| C2 | the sRGB decoding exponent is the 2.2 of the common approximation, not 2.4 | yes | `T09-S2L-d: expected 3.9359395040889670e-3 got 6.24397444e-3` |
| C3 | the sRGB encoding drops its offset, so the curve no longer inverts the decode | yes | `T09-L2S-c: expected 4.6135612950044164e-1 got 5.16356111e-1` |
| C4 | 8-bit quantisation truncates instead of rounding to nearest | yes | `T09-RT8: expected [255, 0, 0, 128] got [254, 0, 0, 128]` |
| C5 | 8-bit quantisation scales by 256 instead of 255 | yes | `T09-RT8: expected [255, 0, 0, 128] got [255, 0, 0, 129]` |
| C6 | dequantisation scales by 256 instead of 255, so a round trip drifts | yes | `T09-PREMUL: expected 5.0196078431372548e-1, 0.0000000000000000e0, 0.0000000000000000e0, 5.0196078431372548e-1 got 4.95568424e-1, 0.00000000e0, 0.00...` |
| C7 | unpremultiplying a zero-alpha pixel divides rather than returning zero | yes | `FX-A-004: expected 0.00000000e0, 0.00000000e0, 0.00000000e0, 0.00000000e0 finite=true got NaN, NaN, NaN, 0.00000000e0 finite=false` |
| C8 | normal-over weights the destination by the destination's own alpha | yes | `FX-A-001: expected 2.0000000000000001e-1, 4.0000000000000002e-1, 5.9999999999999998e-1, 1.0000000000000000e0 got 0.00000000e0, 0.00000000e0, 0.0000...` |
| C9 | normal-over adds the two alphas instead of compositing them | yes | `FX-A-002: expected 8.0000000000000004e-1, 1.0000000000000001e-1, 2.0000000000000001e-1, 1.0000000000000000e0 got 8.00000012e-1, 1.00000001e-1, 2.00...` |
| C10 | premultiply scales alpha by itself as well as the colour | yes | `FX-A-003: expected 5.0000000000000000e-1, 0.0000000000000000e0, 5.0000000000000000e-1, 1.0000000000000000e0 got 5.00000000e-1, 0.00000000e0, 7.5000...` |
| C11 | the working conversion premultiplies before decoding sRGB, not after | yes | `T09-PREMUL: expected 5.0196078431372548e-1, 0.0000000000000000e0, 0.0000000000000000e0, 5.0196078431372548e-1 got 2.15860531e-1, 0.00000000e0, 0.00...` |
| C12 | the working conversion never premultiplies | yes | `T09-PREMUL: expected 5.0196078431372548e-1, 0.0000000000000000e0, 0.0000000000000000e0, 5.0196078431372548e-1 got 1.00000000e0, 0.00000000e0, 0.000...` |
| C13 | output encoding writes premultiplied colour instead of straight | yes | `T09-RT8: expected [255, 0, 0, 128] got [188, 0, 0, 128]` |
| C14 | 8-bit PNG input is tagged as already linear, so it is never decoded | yes | `T09-TAG: expected Srgb Straight got LinearLight Straight` |


## B-03 sequence import

`src/media.rs`, checked by `tests/b03_import.rs`, whose table is `verification/B-03_import_table.md`.

**10 of 10 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| I1 | the drawing number is taken from the first digit run in the name, not the last | yes | `layer1: inferred pattern expected "layer1_%03d.png" got "layer%01d_000.png"` |
| I2 | when two files claim one number the later one wins, so the result depends on order | yes | `two files claiming drawing 7: the first in sorted order is the one imported expected "cel_007.png" got "cel_07.png"` |
| I3 | a rejected duplicate still votes on the naming pattern | yes | `two files claiming drawing 7: only the imported file describes the pattern expected "cel_%03d.png" got "cel_%02d.png"` |
| I4 | a sequence that starts at 100 is reported as missing its first hundred drawings | yes | `a sequence starting at 100: the drawings below it are not missing expected "101,102,103,106,107,108" got "0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,...` |
| I5 | a file whose name the pattern does not generate is never reported as a variant | yes | `layer1: names not matching the pattern expected "0" got "1"` |
| I6 | the selection is grouped in whatever order it arrived, not in sorted order | yes | `the same two files selected in the opposite order import identically expected "cel_007.png / cel_%03d.png" got "cel_07.png / cel_%02d.png"` |
| I7 | a run of drawing numbers swallows the gap after it | yes | `the gap warning names runs and keeps the hole between them expected "6 drawings are missing from cel_%03d.png: 101-103, 106-108." got "6 drawings a...` |
| I8 | the inferred pattern is the first naming seen rather than the commonest | yes | `two names of one shape and one of another: the pattern is the commonest expected "cel_%03d.png" got "cel_%02d.png"` |
| I9 | a missing drawing decodes as a neighbouring one instead of raising a warning | yes | `layer3: drawing 7 refuses to decode expected "MEDIA_SEQUENCE_GAP" got "decoded something"` |
| I10 | files with no number in their name are dropped without saying so | yes | `file with no number: diagnostic raised expected "true" got "false"` |


## B-04 time and exposure

`src/time.rs`, checked by `tests/b04_exposure.rs`, whose table is `verification/B-04_exposure_table.md`.

**10 of 10 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| T1 | a layer is active one frame past its out point | yes | `layer-local: frame 110 is outside the half-open interval expected "None" got "Some(15)"` |
| T2 | the source offset is subtracted instead of added, reversing a slipped layer | yes | `layer-local: frame 100 maps to local 5 expected "Some(5)" got "Some(-5)"` |
| T3 | the layer-local frame is the composition frame, so the in point is ignored | yes | `layer-local: frame 100 maps to local 5 expected "Some(5)" got "Some(105)"` |
| T4 | a span's end frame is inclusive, so every hold runs one frame long | yes | `layer2: all 240 drawing numbers match the exposure sheet expected "match" got "frame 1: sheet 1 evaluator 0"` |
| T5 | a frame in a hole between two spans returns the next span's drawing | yes | `a hole between spans is transparent, not an error expected "None" got "Some(2)"` |
| T6 | consecutive spans are laid out one frame apart regardless of their length | yes | the check inside `drawing_numbers_need_not_increase` |
| T7 | seconds are frame times numerator over denominator, the rate inverted | yes | `FX-TIME-003: frame 1 is exactly 1001/24000 seconds expected "1001/24000" got "24000/1001"` |
| T8 | a composition's last frame is one past its end | yes | `composition: frame count expected "240" got "241"` |
| T9 | an exposed but absent drawing renders transparent instead of raising a warning | yes | `layer3: composition frames whose exposed drawing is missing expected "14,15,38,39,62,63,86,87,110,111,134,135,158,159,182,183,206,207,230,231" got ""` |
| T10 | a missing drawing substitutes the nearest one that exists | yes | `layer3: composition frames whose exposed drawing is missing expected "14,15,38,39,62,63,86,87,110,111,134,135,158,159,182,183,206,207,230,231" got ""` |


## B-04b frame-level diagnostic rate limiting

`src/diagnostics.rs`, checked by `tests/b04b_frame_log.rs`, whose table is `verification/B-04b_frame_log_table.md`.

**11 of 11 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| M1 | the limit is off by one: one more occurrence is logged in full than the limit allows | yes | `reference shot: records in the rate-limited log: expected 4 got 5` |
| M2 | a suppressed occurrence is not counted, so the summary under-reports the problem | yes | `reference shot: records in the rate-limited log: expected 4 got 3` |
| M3 | a group with nothing suppressed is summarised anyway | yes | `a group at the limit is logged in full and summarised not at all: expected 3 got 4` |
| M4 | grouping ignores the subject, so two different problems collapse into one summary | yes | `two subjects under one identifier stay apart: two logged, two summaries: expected 4 got 2` |
| M5 | grouping ignores the identifier, so two different diagnostics collapse into one | yes | `two identifiers under one subject stay apart: two logged, two summaries: expected 4 got 2` |
| M6 | the ranges are built without sorting first | yes | `ranges: out of order and repeated input still reads as one honest set: expected 3 to 5, 9 got 9, 3 to 4, 3, 5` |
| M7 | the ranges are built without removing repeated frames | yes | `ranges: out of order and repeated input still reads as one honest set: expected 3 to 5, 9 got 3, 3 to 5, 9` |
| M8 | a run swallows the gap after it: non-consecutive frames are reported as one range | yes | `reference shot: the summary states the ranges and how many were suppressed: expected Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134...` |
| M9 | the summary reports the total as the number suppressed | yes | `reference shot: the summary states the ranges and how many were suppressed: expected Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134...` |
| M10 | the summary drops the remediation, so the one actionable line is the one lost | yes | `reference shot: the summary keeps the remediation of the first occurrence: expected Add the missing file and relink the sequence, or change the exp...` |
| M11 | the summary is given a fresh severity instead of the one it stands for | yes | `reference shot: the summary keeps the severity: expected WARNING got INFO` |


## B-05 the document model, keyframes, commands and undo

`src/model.rs, src/command.rs`, checked by `tests/b05_model.rs`, whose table is `verification/B-05_model_table.md`.

**14 of 14 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| K1 | before the first keyframe the property reads its base value, not the first keyframe's | yes | `rule, before the first keyframe (frame -5) expected "(20, 0)" got "(0, 0)"` |
| K2 | after the last keyframe the property reads its base value, not the last keyframe's | yes | `rule, on the last keyframe expected "(100, 60)" got "(0, 0)"` |
| K3 | a hold keyframe interpolates like a linear one | yes | `rule, inside a hold segment (frame 6) expected "(20, 0)" got "(60, 0)"` |
| K4 | linear interpolation runs backwards between the two keyframes | yes | the check inside `interpolation_mode_belongs_to_the_segment_that_starts_at_it` |
| K5 | the interpolation fraction is divided by one frame too many | yes | `rule, halfway along a linear segment (frame 18) expected "(100, 30)" got "(100, 27.692307692307693)"` |
| K6 | setting a keyframe where one exists adds a second at the same frame | yes | `setting a keyframe where one exists replaces it rather than duplicating expected "3" got "4"` |
| K7 | a Vec2 property interpolates only its x component | yes | `rule, halfway along a linear segment (frame 18) expected "(100, 30)" got "(100, 0)"` |
| L1 | a matte cycle is never detected | yes | `matte: a cycle is rejected expected "MATTE_CYCLE" got "applied"` |
| L2 | moving a layer lands it one place below where it was asked to go | yes | `reorder: an index in the middle of the stack is the place the layer lands expected "sakura, layer1, layer3, layer4" got "sakura, layer3, layer1, la...` |
| L3 | deleting a layer drops it from the order but leaves its record in the map | yes | `the deleted layer's record is gone, not merely unlisted expected "absent, 4 layers" got "still present, 4 layers"` |
| U1 | a layer's matte dependents are read as the layers its own matte points at | yes | `matte: layer 3 now has a dependent expected "layer-fx" got "layer-3"` |
| U2 | a transaction whose later command fails still commits the earlier ones | yes | the check inside `b05_model_and_undo` |
| U3 | redo stores the state before the original edit instead of before the redo | yes | `undo after redo still undoes expected "0" got "12.5"` |
| U4 | a drag that ends where it started still becomes an undo item | yes | `a drag that ends where it started leaves no history item expected "14, (100, 0)" got "15, (100, 0)"` |


## B-05a transform, sampling and opacity

`src/render.rs`, checked by `tests/b05a_transform.rs`, whose table is `verification/B-05a_transform_table.md`.

**12 of 12 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| R1 | positive rotation turns anticlockwise on screen instead of clockwise | yes | `FX-XF-004: a 90 degree rotation about the anchor sends left-of-centre to above-centre expected "[0.250000, 0.500000, 0.750000, 1.000000]" got "[0.0...` |
| R2 | the transform applies rotation before scale, not scale before rotation | yes | `a non-uniform scale is applied in the layer's own axes, before the rotation turns them expected "[0.250000, 0.500000, 0.750000, 1.000000]" got "[0....` |
| R3 | the anchor is translated towards the origin instead of away from it | yes | `FX-XF-004: a 90 degree rotation about the anchor sends left-of-centre to above-centre expected "[0.250000, 0.500000, 0.750000, 1.000000]" got "[0.0...` |
| R4 | a destination pixel is sampled at its top-left corner, not its centre | yes | `FX-XF-001: identity reproduces every source pixel exactly, bit for bit expected "identical" got "first difference at float 3"` |
| R5 | bilinear weights are taken from source pixel corners, not centres | yes | `FX-XF-001: identity reproduces every source pixel exactly, bit for bit expected "identical" got "first difference at float 0"` |
| R6 | a sample off the top or bottom edge repeats the edge row instead of reading as transparent | yes | `an opaque edge shifted half a pixel fades to 0.25 at its corner, it does not clamp expected "0.25" got "0.5"` |
| R7 | the vertical bilinear weight is the horizontal one | yes | `a non-uniform scale is applied in the layer's own axes, before the rotation turns them expected "[0.250000, 0.500000, 0.750000, 1.000000]" got "[0....` |
| R8 | layer opacity fades the colour but not the coverage | yes | `layer opacity 0.5 halves the premultiplied sample, RGB and alpha together expected "[0.125, 0.25, 0.375, 0.5]" got "[0.125, 0.25, 0.375, 1]"` |
| R9 | a layer scaled to zero is drawn untransformed instead of not at all | yes | `a scale of zero renders nothing rather than dividing by zero expected "0" got "1"` |
| R10 | the inverse transform's translation has the wrong sign | yes | `FX-XF-002: the impulse lands whole on the pixel one to the right expected "[0.25, 0.5, 0.75, 1]" got "[0, 0, 0, 0]"` |
| R11 | composition applies the outer transform first | yes | `FX-XF-004: a 90 degree rotation about the anchor sends left-of-centre to above-centre expected "[0.250000, 0.500000, 0.750000, 1.000000]" got "[0.0...` |
| R12 | a layer at zero opacity is drawn at full strength | yes | `a layer at zero opacity contributes nothing at all, not a faint one expected "0" got "1"` |


## B-05b the render trace

`src/trace.rs`, checked by `tests/b05b_trace.rs`, whose table is `verification/B-05b_trace_table.md`.

**12 of 12 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| M1 | the transform and opacity stage names are swapped | yes | `each image names its pipeline stage` |
| M2 | the transform stage image already has the layer's opacity applied | yes | `the transform image is written before opacity, so fg is still fully opaque there` |
| M3 | the composite stage shows the stack below the layer, not including it | yes | `the top layer's composite image is byte-identical to frame.png` |
| M4 | the decode stage writes a blank frame-sized image instead of the source | yes | `the fg decode image is at the layer's own 2x2 extent, not the composition's` |
| M5 | layer IDs go into file names with only the slash replaced | yes | `an ID that reduces to separators keeps what is left of it, not the separators` |
| M6 | PNG tags are written as Latin-1 tEXt instead of UTF-8 iTXt | yes | the check inside `b05b_a_unicode_layer_id_survives_the_round_trip` |
| M7 | the opacity stage claims document 21 step 4 instead of step 6 | yes | `each image names the document 21 step it belongs to` |
| M8 | the manifest lists only the first of the stages this build does not implement | yes | `the manifest names every stage of document 21's order this build does not implement` |
| M9 | stage file names do not pad the layer index, so layer 10 sorts before layer 2 | yes | `the trace directory holds exactly the eight stage images, the frame and the manifest` |
| M10 | a name that cleans to leading or trailing dashes keeps them | yes | `an ID that reduces to separators keeps what is left of it, not the separators` |
| M11 | a very long layer ID is not shortened before it becomes a file name | yes | `an ID of two hundred characters is shortened to a file name a filesystem accepts` |
| M12 | the per-frame trace folder is not zero-padded, so frame 10 sorts before frame 2 | yes | the check inside `b05b_a_unicode_layer_id_survives_the_round_trip` |


## B-05c blend modes

`src/composite.rs, src/render.rs`, checked by `tests/b05c_blend.rs`, whose table is `verification/B-05c_blend_table.md`.

**10 of 10 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| M1 | multiply uses the premultiplied colours instead of the straight ones | yes | `multiply, half-alpha red over opaque 50% grey` |
| M2 | screen drops the -cs*cd term and becomes a plain sum | yes | `FX-B-002 screen, 50% grey over 50% grey` |
| M3 | add is not clamped to 1 | yes | `FX-B-003 add, 70% grey over 60% grey, clamped` |
| M4 | output alpha is the sum of the two coverages instead of their union | yes | `FX-B-001 multiply, opaque red over 50% grey` |
| M5 | the blend term is weighted by the source alpha alone | yes | `screen, half-alpha 80% grey over half-alpha 40% grey` |
| M6 | the source term the destination does not cover is dropped | yes | `screen, half-alpha 80% grey over half-alpha 40% grey` |
| M7 | multiply and screen are swapped | yes | `FX-B-001 multiply, opaque red over 50% grey` |
| M8 | a zero-alpha destination divides rather than reading as zero | yes | `over a transparent background, every mode leaves the source untouched` |
| M9 | the renderer ignores the layer's mode and always composites normally | yes | `the renderer applies the layer's mode: all four pixels of a multiply frame` |
| M10 | layer opacity is applied after the blend instead of before it (step 6 skipped) | yes | `the renderer applies the layer's mode: all four pixels of a multiply frame` |


## B-09 project save, load and recovery

`src/persist.rs`, checked by `tests/b09_persistence.rs`, whose table is `verification/B-09_persistence_table.md`.

**14 of 14 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| P1 | saving drops the keys the file already had and this build does not understand | yes | `minimal_project.json: opening it and saving it reproduces the file — expected "identical", got "line 22: \"      \\\"work_area\\\": {\" became \"  ...` |
| P2 | numeric keys are sorted as text, so drawing 10 comes before drawing 2 | yes | `drawing numbers are written in numeric order, so drawing 2 comes before drawing 10 — expected "\"1\", \"2\", \"10\"", got "\"1\", \"10\", \"2\""` |
| P3 | keys are written in alphabetical order rather than the schema's order | yes | `minimal_project.json: opening it and saving it reproduces the file — expected "identical", got "line 2: \"  \\\"schema_version\\\": 0,\" became \" ...` |
| P4 | a project saved by a newer build is opened rather than refused | yes | `a project saved by a newer version is refused by name (document 07) — expected "PROJECT_SCHEMA_NEWER", got "PROJECT_SCHEMA_INVALID"` |
| P5 | two assets may share one ID | yes | `two asset records claiming the same ID are refused, not silently deduplicated — expected "PROJECT_SCHEMA_INVALID", got "opens with none"` |
| P6 | a layer's asset reference is satisfied by the project having any asset at all | yes | `a layer naming an asset the project does not have is refused — expected "PROJECT_SCHEMA_INVALID", got "opens with none"` |
| P7 | the missing-media check is inverted: the files that are there are reported gone | yes | `the warning names the file that is missing, not the one that is there — expected "true", got "false"` |
| P8 | preserved unknown keys are matched to the first entry, not the one with that ID | yes | `each asset keeps its own preserved field, not the first asset's — expected "belongs-to-asset-other, belongs-to-asset-cel", got "belongs-to-asset-ot...` |
| P9 | scale is written to the file as a factor instead of a percentage | yes | `cel_holds_project.json: opening it and saving it reproduces the file — expected "identical", got "line 72: \"                100,\" became \"      ...` |
| P10 | the save writes over the project directly instead of a temporary sibling | yes | `FX-IO-001: and the project on disk is still the last one that saved, byte for byte — expected "identical", got "line 1: \"{\" became \"the project ...` |
| P11 | the save skips document 07's validate-before-writing step | yes | `a project this build could not reopen is refused instead of written — expected "PROJECT_SAVE_FAILED", got "the save reported success"` |
| P12 | the autosave writes over the manual save instead of a recovery slot | yes | `the first five autosaves use five different files — expected "5", got "1"` |
| P13 | the autosave overwrites the newest recovery slot rather than the oldest | yes | `the sixth autosave reuses the oldest of the five slots rather than making a sixth — expected "shot.autosave-0.json", got "shot.autosave-4.json"` |
| P14 | a write that stops short of the whole file is reported as a success | yes | `FX-IO-002: a write that runs out of room reports the right thing — expected "PROJECT_SAVE_FAILED", got "the save reported success"` |


## B-08a assembling and rendering a frame from a saved project

`src/compose.rs`, checked by `tests/b08a_compose.rs`, whose table is `verification/B-08a_compose_table.md`.

**10 of 10 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| C1 | The stack is assembled top of the composition first, so the bottom layer is painted over everything above it | yes | `and hands the renderer them bottom first, so layer 4 composites last (expected layer-1, layer-2, layer-3, layer-4, got layer-4, layer-3, layer-2, l...` |
| C2 | A layer switched off is drawn anyway | yes | `a layer switched off is left out of the frame (expected layer-1, layer-3, layer-4, got layer-1, layer-2, layer-3, layer-4)` |
| C3 | A frame the composition does not have is rendered instead of refused | yes | `a frame past the end of the composition is refused, not clamped to the last one (expected COMMAND_INVALID_VALUE, got the render was attempted anyway)` |
| C4 | Opacity is read at the composition's first frame rather than at the frame being rendered | yes | `halfway between the two keyframes, frame 12 (expected 0.500000, got 0.000000)` |
| C5 | The anchor point and the position are passed to the transform the wrong way round | yes | `the anchor point lands on the layer's position (expected (960.0, 540.0), got (-1620.0, -930.0))` |
| C6 | A drawing that is not on disk is not noticed here, so it is reported as a broken file rather than a missing one | yes | `a file the project points at that is not on disk is a different fault, named differently (expected MEDIA_MISSING: layer3/layer3_000_moved_away.png ...` |
| C7 | A layer carrying a track matte is drawn silently, with no warning that the matte was ignored | yes | `a track matte, which this build does not render, is reported rather than ignored (expected PROJECT_FEATURE_UNSUPPORTED: Layer layer4 has a track ma...` |
| C8 | The composition asked for is ignored and the project's first composition is rendered instead | yes | `a composition the project does not have is refused (expected COMMAND_TARGET_MISSING, got a frame was planned from nothing)` |
| C9 | The asset record's colour and alpha interpretation is ignored, so every sequence is read as sRGB and straight | yes | `an asset recorded as already linear is not put through the transfer function again (expected 0.862745, got 0.715694)` |
| C10 | The frame's width and height are swapped | yes | `at the composition's own extent (expected 1920x1080, got 1080x1920)` |


## T-08 exporting a frame range to a PNG sequence

`src/export.rs, src/lib.rs, src/color.rs`, checked by `tests/t08_export.rs`, whose table is `verification/T-08_export_table.md`.

**12 of 12 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| E1 | The export range loses its last frame, so every shot comes out one frame short | yes | `frames 0 to 239 is 240 files, because document 07 includes both ends (expected 240, got 239)` |
| E2 | A range that ends before it starts is accepted and quietly exports nothing | yes | `a range that ends before it starts is refused, not silently swapped (expected Failed, COMMAND_INVALID_VALUE, 0 files, got Completed, , 0 files)` |
| E3 | A naming pattern with no frame number is accepted, so every frame overwrites one file | yes | `a naming pattern with no frame number is refused before anything is written (expected COMMAND_INVALID_VALUE, 0 files, got EXPORT_BLOCKED_MISSING_ME...` |
| E4 | The default missing-drawing check is skipped, so an export with a hole in it runs | yes | `an export whose range contains a missing drawing is blocked, and writes nothing (expected Blocked, 0 files, got Completed, 48 files)` |
| E5 | A gap in a sequence is not treated as a missing drawing, so the export is not blocked | yes | `an export whose range contains a missing drawing is blocked, and writes nothing (expected Blocked, 0 files, got Completed, 48 files)` |
| E6 | Cancellation is never noticed, so the stop button does nothing | yes | `an export cancelled before it starts writes nothing and claims nothing (expected Cancelled, 0 files, EXPORT_CANCELLED, got Completed, 100 files, )` |
| E7 | A job that was refused, blocked or cancelled still reports success | yes | `a refused export never reports success (expected false, got true)` |
| E8 | A write failure is reported under the cancellation identifier, as if you had asked for it | yes | `it names the frame and the path that failed (expected EXPORT_WRITE_FAILED: Frame 14 could not be written., got no write failure was reported)` |
| E9 | A negative composition frame loses its minus sign in the file name | yes | `a negative frame number keeps its sign in front of the padded digits (D-29) (expected neg_-0012.png, got neg_0012.png)` |
| E10 | Output produced with a parked feature bypassed carries no note that it is incomplete | yes | `and so does the file itself, where it cannot be separated from the picture (expected incomplete: a layer carrying a parked feature was drawn withou...` |
| E11 | Straight alpha is written without unpremultiplying, so half-transparent paint exports dark | yes | `frame 12 shows drawing 6, whose bar covers x 960 to 1119 (expected R 255, B 0, A 128, got R 188, B 0, A 128)` |
| E12 | Sixteen-bit output is quantised against the eight-bit maximum | yes | `and the same straight pixel is the same colour at the deeper precision (expected R 65535, B 0, A 32768, got R 255, B 0, A 128)` |


## T-07 export half: a project saved, reopened and exported to the same files

`src/persist.rs`, checked by `tests/t07e_roundtrip_export.rs`, whose table is `verification/T-07e_roundtrip_table.md`.

**8 of 8 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| P1 | A switched-off layer is written to the file as switched on | yes | `the switched-off layer is still in the file and still switched off (expected false, got true)` |
| P2 | The last exposure span is dropped on the way into the file | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20170 bytes against 20030)` |
| P3 | One drawing is left out of the asset's frame list when the project is saved | yes | `layer 3's asset still has no drawing 7, because that file is a deliberate defect (expected 0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, got 0, 1, 2, 3, 4, 5,...` |
| P4 | Scale is read back as a percentage instead of a factor, so a reopened project renders a hundred times too large | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20457 bytes against 20465)` |
| P5 | Layer order is written to the file upside down | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20457 bytes against 20457)` |
| P6 | Exposure sheets are silently ignored when a project is opened | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20457 bytes against 3708)` |
| P7 | A layer's last frame is lost on every save | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20457 bytes against 20457)` |
| P8 | Drawing numbers shift by one when a project is opened, so every cel is the wrong one | yes | `saving the reopened project reproduces the file it came from, byte for byte (expected identical, got the two files differ: 20457 bytes against 20458)` |


## H-01 the whole picture against an independent compositor

`src/render.rs, src/color.rs, src/lib.rs`, checked by `tests/h01_whole_picture.rs`, whose table is `verification/H-01_whole_picture_table.md`.

**6 of 6 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| H1 | Every layer is drawn one pixel to the right | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 1128297 pix...` |
| H2 | The sRGB curve drifts by less than a hundredth of an exponent, dimming the whole frame | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 2073600 pix...` |
| H3 | Red and blue are swapped when a layer is put over what is under it | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 235054 pixe...` |
| H4 | The topmost layer is left out of every frame | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 160801 pixe...` |
| H5 | The last row of every tile is never drawn, leaving seams across the frame | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 17280 pixel...` |
| H6 | Cels are composited with straight colour instead of premultiplied | yes | `frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces (expected 0 pixels differ, got 199357 pixe...` |


## H-02 the whole picture with the layers moved and scaled

`src/render.rs, src/compose.rs`, checked by `tests/h02_transformed_picture.rs`, whose table is `verification/H-02_transformed_table.md`.

**6 of 6 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| X1 | Bilinear weights are the wrong way round, so a resampled layer leans half a pixel the wrong way | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |
| X2 | Sample neighbours are taken from the pixel corner instead of the pixel centre | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |
| X3 | A sample off the edge of a layer takes the border pixel instead of transparent black | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |
| X4 | The transform chain is composed in the wrong order, so a layer is scaled about the wrong point | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |
| X5 | Layer opacity fades the alpha but not the colour, so a half-faded layer stays fully painted | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |
| X6 | Scale is read as a percentage rather than a unit factor, against D-22 | yes | `frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 (expected 0 pixels differ by m...` |


## B-10 the whole 240-frame shot exported twice

`src/export.rs, src/media.rs, src/diagnostics.rs`, checked by `tests/b10_full_shot.rs`, whose table is `verification/B-10_full_shot_table.md`.

**6 of 6 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| Y1 | The last frame of the range is never exported, so every shot is one frame short | yes | `the whole shot is asked for and the whole shot is written (expected 240 frames requested, 240 written, completed, got 239 frames requested, 239 wri...` |
| Y2 | Frame numbers are padded one digit narrower than the pattern asks | yes | `the first and last file are named for their own frame, four digits wide (expected shot_0000.png and shot_0239.png, got shot_000.png and shot_239.pn...` |
| Y3 | A missing drawing is filled with the nearest one that exists, which document 28 forbids | yes | `twenty affected frames are reported as three in full and one summary, per D-25 (expected 4 gap diagnostics, got 0 gap diagnostics); see verificatio...` |
| Y4 | Affected frames are summarised as one unbroken range, hiding which frames are actually affected | yes | `the summary names every affected frame and counts the ones it did not log (expected Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134 ...` |
| Y5 | Frame-level warnings are never rate-limited during a render, so a long shot logs twenty of them | yes | `twenty affected frames are reported as three in full and one summary, per D-25 (expected 4 gap diagnostics, got 20 gap diagnostics); see verificati...` |
| Y6 | The summary that stands in for the suppressed warnings is never appended | yes | `twenty affected frames are reported as three in full and one summary, per D-25 (expected 4 gap diagnostics, got 3 gap diagnostics); see verificatio...` |


## B-11 the dependency and licence record against the build

`docs/DEPENDENCIES.md`, checked by `tests/b11_dependency_record.rs`, whose table is `verification/B-11_record_table.md`.

**7 of 7 breaks caught.**

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| Z1 | A crate the build links is left out of the record entirely | yes | `every crate the build resolves has a row in the record (expected none missing from the record, got missing from the record: rayon); see verificatio...` |
| Z2 | A row keeps a version the build no longer resolves | yes | `every row carries the exact version the build resolved, not a range or an older one (expected no version disagrees, got png: record says 0.18.0, th...` |
| Z3 | The record names a crate that is not in the build at all | yes | `the record names no crate the build does not use (expected no rows without a crate, got rows without a crate: openssl); see verification/B-11_recor...` |
| Z4 | A dependency Cargo.toml names by hand is recorded as though something else pulled it in | yes | `the record marks as direct exactly the three dependencies Cargo.toml asks for (expected png, rayon, serde_json, got rayon, serde_json); see verific...` |
| Z5 | A row names the crate but leaves its licence blank | yes | `every row names a licence (expected every row names a licence, got no licence named for: memchr-2.8.3); see verification/B-11_record_table.md` |
| Z6 | Only one of the two versions of a crate the build resolves twice is recorded | yes | `every row carries the exact version the build resolved, not a range or an older one (expected no version disagrees, got miniz_oxide: record says 0....` |
| Z7 | A crate's archived licence text is gone from the repository, leaving only its name in the table | yes | `every crate's own licence text is archived in this repository, not merely named (expected every crate has an archived licence, got nothing archived...` |


## B-08 the preview resolution and the playback clock

`src/preview.rs`, checked by `tests/b08_preview.rs`, whose table is `verification/B-08_preview_table.md`.

**10 of 10 breaks caught.**

The two things this unit decides are the ones the owner decided on 2026-09-05: D-33, that the
preview starts at draft resolution with the difference always shown, and D-32, that playback
holds real time and drops the frames it cannot deliver. Each break below is a plausible way to
get one of those slightly wrong rather than an obvious deletion - a preview that crops the
top-left corner instead of shrinking the picture, a clock that rounds to the nearest frame
instead of holding each frame for its own interval, a drop count that includes the frame that
was actually shown.

| # | What was broken | Caught | The check that failed |
|---|---|---|---|
| P1 | Draft is half the composition on each axis instead of the quarter SP-05 measured | yes | `draft is SP-05's measured extent for this composition (expected 480x270, got 960x540); see verification/B-08_preview_table.md` |
| P2 | The draft extent truncates, so a composition that does not divide by four loses its last column | yes | `a composition that does not divide by four keeps its last column and row (expected 481x271, got 480x270); see verification/B-08_preview_table.md` |
| P3 | The draft frame is a crop of the top-left corner: the extent shrinks and the layers do not | yes | `at draft, the layers are scaled with the extent rather than cropped by it (expected 480,270 480,270 480,270 480,270, got 1920,1080 1920,1080 1920,1080...` |
| P4 | A draft preview claims to be what the export will write, so no indicator is ever shown | yes | `at draft, R-06a's indication that preview differs from export is on (expected Draft, differs from export: true, got Draft, differs from export: false)...` |
| P5 | Full resolution quietly renders at draft resolution, and says it is full | yes | `full is the composition's own extent, unchanged (expected 1920x1080, got 480x270); see verification/B-08_preview_table.md` |
| P6 | The playback clock rounds to the nearest frame instead of holding each frame for its own interval | yes | `a nanosecond before the first frame ends, frame 0 is still on screen (expected 0, got 1); see verification/B-08_preview_table.md` |
| P7 | The frame that was actually shown is counted as dropped as well, overstating the loss | yes | `asked once every third frame time, each answer is three frames on and two were passed over undrawn (expected 0+0 3+2 6+2 9+2, got 0+0 3+3 6+3 9+3); se...` |
| P8 | Playback runs off the end of the work area instead of looping inside it | yes | `playback loops inside the work area rather than running past its end (expected 10 13 12 11 10, got 10 13 16 19 22); see verification/B-08_preview_tabl...` |
| P9 | A clock that runs backwards drags the picture backwards with it | yes | `if the clock the caller supplies runs backwards, the frame is held and nothing is counted as dropped (expected frame 6, skipped 0, got frame 3, skippe...` |
| P10 | The frame at rest is the end of the work area rather than its start | yes | `at rest, before playback, the work area's first frame is shown (expected 10, got 13); see verification/B-08_preview_table.md` |

One break was tried first and withdrawn, and it is worth recording because it changed the code
rather than the fixture. `scale_plan` returns a full-resolution plan untouched instead of
scaling it by one, and the comment above it said that was what guaranteed a full-resolution
preview matched an export byte for byte. Removing the early return survived: composing a scale
of exactly one multiplies each coefficient by one, which is exact, and no pixel moves. The
comment was overclaiming, so the comment was corrected - the early return is insurance against a
future draft path that gains a half-pixel correction or a rounding step, not today's guarantee -
and the break was replaced with one that can actually be seen: full resolution quietly rendering
at draft resolution while still calling itself full.

Two rows were added to the table before this pass rather than after it, to close a gap the
extent rows could not see. Every check on a draft frame's size would still pass if the extent
shrank and the layers did not, which is a crop of the top-left corner at the right size and of
the wrong thing. The two rows apply each layer's transform to the composition's far corner and
say where it must land - (480, 270) at draft, (1920, 1080) at full - which is arithmetic rather
than a measurement, and P3 is the break that would have got through without them.


## First pass: the six that got through, and what was added

The import fixture was the weak one. These six breaks left every test passing, which means a
build with any of them in it would have looked correct. Each is now covered by a named row in
`verification/B-03_import_table.md`; that table grew from 46 checks to 60.

| Break that survived | Why nothing noticed | The row added |
|---|---|---|
| When two files claim the same drawing number, the later one wins instead of the first | The test checked that a warning was raised and that one file was imported, but never which one | *two files claiming drawing 7: the first in sorted order is the one imported* |
| The rejected duplicate still votes on the sequence's naming pattern | Nothing checked the pattern in the presence of a duplicate | *two files claiming drawing 7: only the imported file describes the pattern* |
| A sequence numbered from 100 is reported as missing drawings 0 to 99 | Every case in the test started at drawing 0 | *a sequence starting at 100: the drawings below it are not missing* |
| The selection is grouped in whatever order the file dialog gave it | The test always handed files over already sorted | *the same two files selected in the opposite order import identically* |
| A run of missing drawings swallows the gap after it, so 101-103 and 106-108 is reported as 101-108 | No case had two separate holes | *the gap warning names runs and keeps the hole between them* |
| The naming pattern is the first name seen rather than the commonest | No case had a majority naming with an odd file out | *two names of one shape and one of another: the pattern is the commonest* |

## Second pass: the twelve that got through, and what was added

Four units, 52 breaks, twelve survivors. Three of the twelve are the kind of fault that is
invisible while it happens and expensive afterwards: an undo that runs and does nothing, a
fingerprint that moves from one drawing to another, and a good project file replaced by one the
application can no longer open.

| Unit | Break that survived | Why nothing noticed | The row added |
|---|---|---|---|
| B-05 | Moving a layer lands it one place below where it was asked to go | Every reorder in the test moved a layer to the end of the stack, where "before this layer" and "after it" mean the same place | *reorder: an index in the middle of the stack is the place the layer lands* |
| B-05 | Deleting a layer drops it from the visible order but leaves its record behind | The test read the order, which looked right; nothing asked whether the layer itself was gone | *the deleted layer's record is gone, not merely unlisted* |
| B-05 | Redo remembers the wrong prior state, so the next undo runs and changes nothing | No case undid anything after a redo | *undo after redo still undoes* |
| B-05 | A drag that wanders and returns to where it started still becomes an undo item | Every drag in the test ended somewhere new | *a drag that ends where it started leaves no history item* |
| B-05a | The transform turns the layer first and stretches it afterwards, instead of the other way round | Every rotation in the table used a uniform scale, where the two orders give the same answer | *a non-uniform scale is applied in the layer's own axes, before the rotation turns them*, and the half-weight row beneath it |
| B-05a | A layer set to zero opacity is drawn at full strength | Opacity was tested at 0.5 and at 1.0, never at 0 | *a layer at zero opacity contributes nothing at all, not a faint one* |
| B-05b | A layer name that reduces to nothing but separators keeps them all | The only unusual name in the test was checked for being ASCII, which a row of dashes also is | *an ID that reduces to separators keeps what is left of it, not the separators* |
| B-05b | A very long layer name is not shortened before it becomes a file name | Same reason: two hundred characters are as ASCII as three | *an ID of two hundred characters is shortened to a file name a filesystem accepts* |
| B-09 | Drawing numbers are written in alphabetical order, so drawing 10 lands before drawing 2 | Every fixture project stops at drawing 2, where alphabetical and numeric order agree | *drawing numbers are written in numeric order, so drawing 2 comes before drawing 10* |
| B-09 | Two assets are allowed to claim the same ID | No fixture project contains a duplicate ID | *two asset records claiming the same ID are refused, not silently deduplicated* |
| B-09 | Data this build does not understand is preserved onto the first asset instead of the one it came from | The fixture projects have one asset each, so first and correct are the same record | *each asset keeps its own preserved field, not the first asset's* |
| B-09 | Saving skips the step that checks the file can be opened again before writing it | Every save in the test was of a project that was valid anyway | *a project this build could not reopen is refused instead of written*, and *the good project it would have replaced is still there, byte for byte* |

## Seventh pass: the two that got through, and what was added

| Unit | What was broken | Why the fixture missed it | Added |
|---|---|---|---|
| H-02 | The transform chain is composed in the wrong order, so a layer is scaled about the wrong point | Every layer in the first draft had its anchor at the origin, where `T(-anchor)` is the identity and the order cannot matter | Layer 3 is now anchored at the centre of the frame, so *scale about the anchor* and *scale about the origin* are different pictures |
| H-02 | A sample off the edge of a layer takes the border pixel instead of transparent black | Every layer being sampled was transparent at its own border, where clamping to the edge and reading transparent black give the same answer | Layer 1, the only cel whose paint reaches its own edge, is now scaled to 50%, so most of the frame samples past that edge |

## Eighth pass: the one that got through, twice, and what it showed

| Unit | What was broken | Why the fixture missed it | Added |
|---|---|---|---|
| B-10 | A missing drawing is filled with the nearest one that exists, which document 28 forbids in as many words | The row that should have caught it sampled *one pixel*, at the place layer 3's bar sits on the frames either side of the gap. A substituted drawing still paints a bar, just 160 pixels further along, so that sample saw sky either way | The whole frame is now scanned for layer 3's exact paint colour, with a positive control requiring the frames either side of the gap to contain it |

That fixture change was not enough on its own, and the second attempt is the more useful half.
The break still survived, and the reason was not the fixture: the mutation had been made in
`Sequence::decode`, which carries document 28's "do not substitute adjacent frame" rule in its
own doc comment and which **no production path calls**. The renderer resolves a drawing through
`time::resolve_in` and decodes the resulting path itself. So the rule was being enforced twice,
once where the render actually goes and once in a function only the tests reach, and a break in
the second is unreachable. The break was remade on the live path and caught there. The lesson is
not about this fixture: a mutation that survives may mean the test is weak, or it may mean the
code it was made in cannot affect the result, and those two look identical from the outside.

None of the nineteen survivors, across every pass, is a bug in the build as it stands. They are
things the build was free to get wrong later without anything saying so.

A further handful of breaks were caught only by a crash rather than by a named row - the test
stopped and printed a stack trace instead of showing the owner a table with one line marked
wrong. Every such place found in this pass now reports a plain sentence and carries on, so a
future failure there reads like the rest of the table.

## What this pass did not cover

Every unit merged to `main` has now been broken on purpose at least once. As of B-08a that
includes the assembly itself - the join between a saved project and a rendered frame - and as of
T-08 it includes the files that leave the application: an export one frame short, a stop button
that does nothing, a job that reports success after being refused, and half-transparent paint
written dark are all breaks that were made and caught.

The gap this section named until now - the *picture*, which no table looked at as a whole - is
closed by H-01. Four frames of the reference shot are composited twice, once by the real
renderer and once by a deliberately naive compositor written from document 21 inside the test,
and all 2,073,600 pixels of each frame have to agree exactly. What that still does not cover is
a misreading of document 21 itself: both compositors were written from the same document, so an
error in the reading would appear in both. It also covers four frames rather than all 240. B-10 does now cover all 240, but for a
different property: that two exports of the shot are byte for byte the same, and that the
twenty frames with no drawing have none of that layer's paint in them. Neither of those is
a claim that the picture is right. Both cover
only the reference shot, whose layers all sit at the identity transform - the sampling rules for
a layer that is moved, turned or scaled are checked by B-05a's table, pixel by named pixel,
rather than here. Nothing has been
mutation-tested against timing or memory behaviour either, which is measurement rather than
correctness and belongs with the performance work in document 24.
