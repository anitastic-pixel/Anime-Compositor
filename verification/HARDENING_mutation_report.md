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

**55 breaks were made across five units. All 55 were caught.**

Six of them were not caught the first time. They are listed at the end, with what was added.


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


## The six that got through first, and what was added

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

None of these six is a bug in the build as it stands. They are things the build was free to get
wrong later without anything saying so.

## What this pass did not cover

B-05 (the document model, its commands and its undo) and B-05a (transform and opacity) have not
been broken on purpose yet. Their tables pass; whether their tables *could* fail is not yet
known. B-05b (project save and load) and B-09 (the render trace) are likewise untested this way.
That is the next hardening item.
