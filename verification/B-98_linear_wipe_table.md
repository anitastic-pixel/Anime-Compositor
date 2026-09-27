# B-98: linear wipe

D-155, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-second of the third batch. Every expected pixel is `Fixtures/linear_wipe/expected_linear_wipe.json`, written by `tools/linear_wipe_reference.py` before this code existed and printed in document 25 as FX-LWIPE-001 to 032. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LWIPE-001 to 031 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LWIPE-001 frame 0: The settings as they start, completion 0, angle 90, feather 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LWIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-002 frame 0: Completion 25: the edge is a quarter of the way across from the left, at x = 4, so columns 0 to 3 are gone, all four channels 0, and columns 4 to 15 are the drawing's own, exactly. | largest difference 1.9e-7 | yes |
| FX-LWIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-003 frame 0: Completion 50: the left half, columns 0 to 7, is gone; the right half is untouched. | largest difference 1.9e-7 | yes |
| FX-LWIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-004 frame 0: Completion 100: every pixel is transparent, the soft edges too. | largest difference 0.0e0 | yes |
| FX-LWIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-005 frame 0: Completion 50, feather 4: the edge is soft over four pixels round x = 8; columns 0 to 5 are gone, columns 6, 7, 8 and 9 keep 1/8, 3/8, 5/8 and 7/8 of every channel, and columns 10 to 15 are untouched. | largest difference 1.9e-7 | yes |
| FX-LWIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-006 frame 0: Angle 270, completion 25: the wipe comes from the right, the edge at x = 12; columns 12 to 15 are gone, the soft skin edge with them. | largest difference 1.9e-7 | yes |
| FX-LWIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-007 frame 0: Angle 0, completion 30: the edge moves up, so the wipe comes from the bottom, the edge at y = 7; rows 7 to 9 are gone. | largest difference 1.9e-7 | yes |
| FX-LWIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-008 frame 0: Angle 180, completion 30: the edge moves down, so the wipe comes from the top, the edge at y = 3; rows 0 to 2 are gone. | largest difference 1.9e-7 | yes |
| FX-LWIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-009 frame 0: Angle 45, completion 40: the wipe comes from the bottom left corner towards the top right along a diagonal; a pixel stays whole where x - y is at least 1 and is gone otherwise, the bottom left triangle wiped. | largest difference 1.9e-7 | yes |
| FX-LWIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-010 frame 0: Angle 135, completion 50, feather 6: from the top left corner, with a soft diagonal edge; the top left is gone, the bottom right untouched, and between them each pixel keeps a part that grows towards the bottom right. | largest difference 1.7e-7 | yes |
| FX-LWIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-011 frame 0: Angle 450, a whole turn past 90, completion 25: exactly FX-LWIPE-002. | largest difference 1.9e-7 | yes |
| FX-LWIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-012 frame 0: Angle -270, completion 25: -270 is 90 the other way round, exactly FX-LWIPE-002. | largest difference 1.9e-7 | yes |
| FX-LWIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-013 frame 0: Completion 50, feather 10000, the most: the edge is so soft that every pixel keeps about half of itself, from 0.49925 of it at the left to 0.50075 at the right. | largest difference 1.1e-7 | yes |
| FX-LWIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-014 frame 0: Completion 0 with angle 45 and feather 20: completion 0 changes nothing, whatever the other settings. | largest difference 1.9e-7 | yes |
| FX-LWIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-015 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns 0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right. | largest difference 1.9e-7 | yes |
| FX-LWIPE-015 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns 0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right. | largest difference 1.9e-7 | yes |
| FX-LWIPE-015 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns 0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right. | largest difference 1.9e-7 | yes |
| FX-LWIPE-015 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns 0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right. | largest difference 1.9e-7 | yes |
| FX-LWIPE-015 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 1 is FX-LWIPE-002, frame 2 FX-LWIPE-003, frame 3 columns 0 to 11 gone, and frame 4 FX-LWIPE-004; the edge sweeps left to right. | largest difference 0.0e0 | yes |
| FX-LWIPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-016 frame 0: Completion held at 25 from frame 0, then 50 from frame 3: frames 0 and 2 are FX-LWIPE-002, frames 3 and 4 FX-LWIPE-003. | largest difference 1.9e-7 | yes |
| FX-LWIPE-016 frame 2: Completion held at 25 from frame 0, then 50 from frame 3: frames 0 and 2 are FX-LWIPE-002, frames 3 and 4 FX-LWIPE-003. | largest difference 1.9e-7 | yes |
| FX-LWIPE-016 frame 3: Completion held at 25 from frame 0, then 50 from frame 3: frames 0 and 2 are FX-LWIPE-002, frames 3 and 4 FX-LWIPE-003. | largest difference 1.9e-7 | yes |
| FX-LWIPE-016 frame 4: Completion held at 25 from frame 0, then 50 from frame 3: frames 0 and 2 are FX-LWIPE-002, frames 3 and 4 FX-LWIPE-003. | largest difference 1.9e-7 | yes |
| FX-LWIPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-017 frame 0: Completion eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-LWIPE-004, every pixel transparent. | largest difference 1.9e-7 | yes |
| FX-LWIPE-017 frame 2: Completion eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-LWIPE-004, every pixel transparent. | largest difference 0.0e0 | yes |
| FX-LWIPE-017 frame 4: Completion eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-LWIPE-004, every pixel transparent. | largest difference 0.0e0 | yes |
| FX-LWIPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-018 frame 0: Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: the edge turns round, from the left (columns 0 to 4 gone) through the top left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at frame 4 (columns 11 to 15 gone). | largest difference 1.9e-7 | yes |
| FX-LWIPE-018 frame 1: Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: the edge turns round, from the left (columns 0 to 4 gone) through the top left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at frame 4 (columns 11 to 15 gone). | largest difference 1.9e-7 | yes |
| FX-LWIPE-018 frame 2: Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: the edge turns round, from the left (columns 0 to 4 gone) through the top left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at frame 4 (columns 11 to 15 gone). | largest difference 1.9e-7 | yes |
| FX-LWIPE-018 frame 3: Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: the edge turns round, from the left (columns 0 to 4 gone) through the top left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at frame 4 (columns 11 to 15 gone). | largest difference 1.9e-7 | yes |
| FX-LWIPE-018 frame 4: Angle keyed from 90 at frame 0 to 270 at frame 4, linear, completion 30: the edge turns round, from the left (columns 0 to 4 gone) through the top left, the top at frame 2 (FX-LWIPE-008) and the top right, to the right at frame 4 (columns 11 to 15 gone). | largest difference 1.9e-7 | yes |
| FX-LWIPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-019 frame 0: Feather keyed from 0 at frame 0 to 8 at frame 4, linear, completion 50: frame 0 is FX-LWIPE-003, frame 2 FX-LWIPE-005, and frame 4 soft over eight pixels, columns 4 to 11. | largest difference 1.9e-7 | yes |
| FX-LWIPE-019 frame 2: Feather keyed from 0 at frame 0 to 8 at frame 4, linear, completion 50: frame 0 is FX-LWIPE-003, frame 2 FX-LWIPE-005, and frame 4 soft over eight pixels, columns 4 to 11. | largest difference 1.9e-7 | yes |
| FX-LWIPE-019 frame 4: Feather keyed from 0 at frame 0 to 8 at frame 4, linear, completion 50: frame 0 is FX-LWIPE-003, frame 2 FX-LWIPE-005, and frame 4 soft over eight pixels, columns 4 to 11. | largest difference 1.9e-7 | yes |
| FX-LWIPE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-020 frame 0: FX-LWIPE-002 on the layer moved three pixels right: the edge is the drawing's own and moves with it, so the drawing's columns 0 to 3, now 3 to 6, are gone. | largest difference 1.9e-7 | yes |
| FX-LWIPE-020 frame 3: FX-LWIPE-002 on the layer moved three pixels right: the edge is the drawing's own and moves with it, so the drawing's columns 0 to 3, now 3 to 6, are gone. | largest difference 1.9e-7 | yes |
| FX-LWIPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-021 frame 0: A drop shadow two pixels left, the layer moved three pixels right, completion 0: the drawing with its shadow, untouched, the shadow's two columns out past the drawing's left edge in columns 1 and 2 too. | largest difference 1.9e-7 | yes |
| FX-LWIPE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-022 frame 0: The same with completion 1: the edge is at x = 0.16, just inside the drawing's left edge, so the drawing is whole but the shadow's grown columns, left of the edge at x = -1.5 and -0.5, are gone. | largest difference 1.9e-7 | yes |
| FX-LWIPE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-023 frame 0: The same with angle 270 and completion 99: the wipe from the right has reached x = -0.16, so all of the drawing is gone but the shadow's grown columns, beyond the edge, stay. | largest difference 3.0e-8 | yes |
| FX-LWIPE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-024 frame 0: The same with completion 100: every pixel is transparent, the shadow's grown columns too. | largest difference 0.0e0 | yes |
| FX-LWIPE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LWIPE-025 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-025 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-026 frame 0: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-026 frame 4: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-027 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-027 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-028 frame 0: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-028 frame 4: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-029 frame 0: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-029 frame 4: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-030 frame 0: Feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-030 frame 4: Feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LWIPE-031 frame 0: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-031 frame 4: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LWIPE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-LWIPE-032, in dispute (D-164, proposed)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LWIPE-032, in dispute (D-164, proposed): the build refuses fx_lwipe_032.json as a fault in its shape, as it does a number written as a word in every effect; the case expects it kept with a warning | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the feather, 8 to 4, and nothing else | LinearWipe { completion: 50.0, angle: 90.0, feather: 4.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lwipe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lwipe_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `feather` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an angle that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion -1 is refused with a sentence, and nothing changes | Linear Wipe's completion runs from 0 to 100, and this is -1. | yes |
| completion 100.5 is refused with a sentence, and nothing changes | Linear Wipe's completion runs from 0 to 100, and this is 100.5. | yes |
| angle 3601 is refused with a sentence, and nothing changes | Linear Wipe's angle runs from -3600 to 3600, and this is 3601. | yes |
| angle -3601 is refused with a sentence, and nothing changes | Linear Wipe's angle runs from -3600 to 3600, and this is -3601. | yes |
| feather -1 is refused with a sentence, and nothing changes | Linear Wipe's feather runs from 0 to 10000, and this is -1. | yes |
| feather 10001 is refused with a sentence, and nothing changes | Linear Wipe's feather runs from 0 to 10000, and this is 10001. | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Linear Wipe's completion runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lwipe_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lwipe_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lwipe_020.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lwipe_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

118 of 118 checks pass.
