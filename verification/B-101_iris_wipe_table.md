# B-101: iris wipe

D-158, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-fifth of the third batch. Every expected pixel is `Fixtures/iris_wipe/expected_iris_wipe.json`, written by `tools/iris_wipe_reference.py` before this code existed and printed in document 25 as FX-IRIS-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-IRIS-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-IRIS-001 frame 0: The settings as they start: completion 0, centre 50, 50, feather 0, invert off: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-IRIS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-002 frame 0: Completion 100: every pixel is transparent, all four channels 0. | largest difference 0.0e0 | yes |
| FX-IRIS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-003 frame 0: Completion 50: the circle about the middle, (8, 5), has closed to half the distance to the farthest corner, a radius of 4.717. The 68 pixels whose centres lie inside it are kept exactly, a round patch of skin and band; every pixel outside it is transparent, the line box and the soft column among them. | largest difference 1.9e-7 | yes |
| FX-IRIS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-004 frame 0: Completion 50, invert on: a hole of the same radius, 4.717, opened from the middle: the 68 pixels inside it are transparent and every other pixel is kept exactly, so FX-IRIS-003 and this add up to the drawing. | largest difference 1.9e-7 | yes |
| FX-IRIS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-005 frame 0: Completion 25: the circle has closed a quarter of the way, to a radius of 7.075: the 128 pixels inside it are kept, and the 32 outside it, the whole of columns 0 and 15 and the ends of rows 0, 1, 8 and 9, are gone. | largest difference 1.9e-7 | yes |
| FX-IRIS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-006 frame 0: Completion 75: the circle has closed to a radius of 2.358, and the pixels inside it are the four by four block of columns 6 to 9 and rows 3 to 6, at this size a square, not yet a round shape; everything else is transparent. | largest difference 1.9e-7 | yes |
| FX-IRIS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-007 frame 0: Completion 25, invert on: a hole of radius 2.358, the same circle as FX-IRIS-006's, so the same four by four block is transparent and every other pixel kept: the two add up to the drawing. | largest difference 1.9e-7 | yes |
| FX-IRIS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-008 frame 0: Completion 50, feather 4: the circle's edge is soft over 4 pixels about the same radius, 4.717. Pixels within 2.717 of the middle are kept exactly, those 6.717 or more away are gone, and between them each pixel is faded as a whole, all four channels by one amount that falls with the distance; the soft column's half covering is faded further. | largest difference 2.0e-7 | yes |
| FX-IRIS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-009 frame 0: Completion 50, feather 4, invert on: the soft hole, each pixel faded by one minus FX-IRIS-008's amount, so the two add up to the drawing. | largest difference 1.9e-7 | yes |
| FX-IRIS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-010 frame 0: Completion 50, feather 10000, the most: the edge is so wide that every pixel is faded to within a thousandth of half, a gentle fade of the whole drawing rather than a circle. | largest difference 1.3e-7 | yes |
| FX-IRIS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-011 frame 0: Completion 0 with feather 4 and invert on: completion 0 is the input exactly, whatever the other settings: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-IRIS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-012 frame 0: Completion 100 with feather 4 and invert on: completion 100 is every pixel transparent, whatever the other settings. | largest difference 0.0e0 | yes |
| FX-IRIS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-013 frame 0: Centre 25, 50, completion 50: the circle closes on (4, 5); its farthest corner is 13 away, so its radius is 6.5, reaching past the drawing's left edge, and every pixel from column 10 on is gone. | largest difference 1.9e-7 | yes |
| FX-IRIS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-014 frame 0: Centre 100, 100, completion 50: the circle closes on the drawing's bottom right corner, a quarter of it showing, radius 9.434: the pixels near that corner are kept, the top left gone. | largest difference 1.9e-7 | yes |
| FX-IRIS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-015 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept pixels among the last's. | largest difference 1.9e-7 | yes |
| FX-IRIS-015 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept pixels among the last's. | largest difference 1.9e-7 | yes |
| FX-IRIS-015 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept pixels among the last's. | largest difference 1.9e-7 | yes |
| FX-IRIS-015 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept pixels among the last's. | largest difference 1.9e-7 | yes |
| FX-IRIS-015 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 1 FX-IRIS-005, frame 2 FX-IRIS-003, frame 3 FX-IRIS-006, frame 4 FX-IRIS-002: the circle closes, each frame's kept pixels among the last's. | largest difference 0.0e0 | yes |
| FX-IRIS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-016 frame 0: Completion 50, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is centre 37.5, 50, and frame 4 is FX-IRIS-013. | largest difference 1.9e-7 | yes |
| FX-IRIS-016 frame 2: Completion 50, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is centre 37.5, 50, and frame 4 is FX-IRIS-013. | largest difference 1.9e-7 | yes |
| FX-IRIS-016 frame 4: Completion 50, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is centre 37.5, 50, and frame 4 is FX-IRIS-013. | largest difference 1.9e-7 | yes |
| FX-IRIS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-017 frame 0: Completion 50, feather keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is FX-IRIS-008, and frame 4 is feather 8. | largest difference 1.9e-7 | yes |
| FX-IRIS-017 frame 2: Completion 50, feather keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is FX-IRIS-008, and frame 4 is feather 8. | largest difference 2.0e-7 | yes |
| FX-IRIS-017 frame 4: Completion 50, feather keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-IRIS-003, frame 2 is FX-IRIS-008, and frame 4 is feather 8. | largest difference 1.8e-7 | yes |
| FX-IRIS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-018 frame 0: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-IRIS-002, as frame 4 is; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-IRIS-018 frame 2: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-IRIS-002, as frame 4 is; frame 0 is the drawing. | largest difference 0.0e0 | yes |
| FX-IRIS-018 frame 4: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-IRIS-002, as frame 4 is; frame 0 is the drawing. | largest difference 0.0e0 | yes |
| FX-IRIS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-019 frame 0: FX-IRIS-003 moved three pixels right: the same, moved; the circle is worked in the drawing's own space and moves with it, nothing grows, and the three columns left of the drawing stay empty. | largest difference 1.9e-7 | yes |
| FX-IRIS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-020 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-020 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-021 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-021 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-022 frame 0: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-022 frame 4: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-023 frame 0: Feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-023 frame 4: Feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-024 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-024 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-025 frame 0: Invert "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-025 frame 4: Invert "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-026 frame 0: Invert "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-026 frame 4: Invert "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-027 frame 0: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-027 frame 4: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the feather, 8 to 4, and nothing else | IrisWipe { completion: 50.0, center: [50.0, 50.0], feather: 4.0, invert: "off" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iris_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `invert` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion -1 is refused with a sentence, and nothing changes | Iris Wipe's completion runs from 0 to 100, and this is -1. | yes |
| completion 101 is refused with a sentence, and nothing changes | Iris Wipe's completion runs from 0 to 100, and this is 101. | yes |
| feather -1 is refused with a sentence, and nothing changes | Iris Wipe's feather runs from 0 to 10000, and this is -1. | yes |
| feather 10001 is refused with a sentence, and nothing changes | Iris Wipe's feather runs from 0 to 10000, and this is 10001. | yes |
| centre 1001, 50 is refused with a sentence, and nothing changes | Iris Wipe's center runs from -1000 to 1000, and this is 1001. | yes |
| invert "yes" is refused with a sentence, and nothing changes | Iris Wipe's invert is "off" or "on", and this is "yes". | yes |
| invert "On" is refused with a sentence, and nothing changes | Iris Wipe's invert is "off" or "on", and this is "On". | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Iris Wipe's completion runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iris_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iris_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iris_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

104 of 104 checks pass.
