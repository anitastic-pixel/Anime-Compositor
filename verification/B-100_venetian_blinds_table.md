# B-100: venetian blinds

D-157, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-fourth of the third batch. Every expected pixel is `Fixtures/venetian_blinds/expected_venetian_blinds.json`, written by `tools/venetian_blinds_reference.py` before this code existed and printed in document 25 as FX-BLINDS-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BLINDS-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLINDS-001 frame 0: The settings as they start, completion 0, angle 0, width 20, feather 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BLINDS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-002 frame 0: Completion 50, width 4, angle 0: level slats four pixels tall from the drawing's top edge, each cleared from its bottom edge up to its middle, so rows 2, 3, 6 and 7 are transparent and every other pixel is the drawing's exactly. | largest difference 1.9e-7 | yes |
| FX-BLINDS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-003 frame 0: Completion 60 with the starting width 20: one slat is taller than the whole drawing, reaching down to row 19, and 60 per cent of it, 12 pixels, is cleared up from there, so only rows 8 and 9 are transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-004 frame 0: Completion 100, width 4, feather 3: every pixel is transparent, the feather too. | largest difference 0.0e0 | yes |
| FX-BLINDS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-005 frame 0: Completion 50, width 4, angle 90: upright slats four columns wide, each cleared from its left edge, so columns 0, 1, 4, 5, 8, 9, 12 and 13 are transparent and the rest exact. | largest difference 1.9e-7 | yes |
| FX-BLINDS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-006 frame 0: Completion 50, width 4, angle 180: level slats cleared from their top edge down, so rows 0, 1, 4, 5, 8 and 9 are transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-007 frame 0: Completion 50, width 4, angle 270: upright slats cleared from their right edge, so columns 2, 3, 6, 7, 10, 11, 14 and 15 are transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-008 frame 0: Completion 50, width 4, angle 30: slats tilted to fall 30 degrees to the right, each cleared from its lower edge; every pixel is either kept exactly or transparent, about half of those shown cleared. | largest difference 1.9e-7 | yes |
| FX-BLINDS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-009 frame 0: Completion 50, width 4, angle -270, three quarter turns the other way: the same direction as 90, so exactly FX-BLINDS-005. | largest difference 1.9e-7 | yes |
| FX-BLINDS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-010 frame 0: Completion 50, width 2, angle 0: slats two pixels tall, so every other row, 1, 3, 5, 7 and 9, is transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-011 frame 0: Completion 50, width 5, angle 0, feather 2: rows 0 and 1 of each slat are kept, the moving edge at its middle row is at half covering (rows 2 and 7), and rows 3, 4, 8 and 9 are transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-012 frame 0: Completion 30, width 5, feather 4: a graded edge, each slat's rows kept at 1, 1, 0.95, 0.7 and 0.45 from its top down; the covering jumps back to 1 at the next slat's top, as a slat's fixed edge stays hard. | largest difference 1.9e-7 | yes |
| FX-BLINDS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-013 frame 0: Completion 50, width 4, feather 10000, the most: the feather is far wider than a slat, so every shown pixel is kept at about half, within 0.0002 of it, the drawing fading evenly. | largest difference 1.1e-7 | yes |
| FX-BLINDS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-014 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; a pixel once cleared never comes back. | largest difference 1.9e-7 | yes |
| FX-BLINDS-014 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; a pixel once cleared never comes back. | largest difference 1.9e-7 | yes |
| FX-BLINDS-014 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; a pixel once cleared never comes back. | largest difference 1.9e-7 | yes |
| FX-BLINDS-014 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; a pixel once cleared never comes back. | largest difference 1.9e-7 | yes |
| FX-BLINDS-014 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, width 4, linear: frame 0 untouched, then rows 3 and 7 go, then 2, 3, 6 and 7 (frame 2 is FX-BLINDS-002), then all but rows 0, 4 and 8, and frame 4 is empty; a pixel once cleared never comes back. | largest difference 0.0e0 | yes |
| FX-BLINDS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-015 frame 0: Completion held at 25 from frame 0, then 75 from frame 3, width 4: frames 0 and 2 have rows 3 and 7 cleared, frames 3 and 4 all but rows 0, 4 and 8. | largest difference 1.9e-7 | yes |
| FX-BLINDS-015 frame 2: Completion held at 25 from frame 0, then 75 from frame 3, width 4: frames 0 and 2 have rows 3 and 7 cleared, frames 3 and 4 all but rows 0, 4 and 8. | largest difference 1.9e-7 | yes |
| FX-BLINDS-015 frame 3: Completion held at 25 from frame 0, then 75 from frame 3, width 4: frames 0 and 2 have rows 3 and 7 cleared, frames 3 and 4 all but rows 0, 4 and 8. | largest difference 1.9e-7 | yes |
| FX-BLINDS-015 frame 4: Completion held at 25 from frame 0, then 75 from frame 3, width 4: frames 0 and 2 have rows 3 and 7 cleared, frames 3 and 4 all but rows 0, 4 and 8. | largest difference 1.9e-7 | yes |
| FX-BLINDS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-016 frame 0: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, width 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 1.9e-7 | yes |
| FX-BLINDS-016 frame 2: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, width 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 0.0e0 | yes |
| FX-BLINDS-016 frame 4: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, width 4: at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both every pixel transparent. | largest difference 0.0e0 | yes |
| FX-BLINDS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-017 frame 0: Angle keyed from 0 at frame 0 to 60 at frame 4, completion 50, width 4: frame 0 is FX-BLINDS-002, frame 2 is FX-BLINDS-008 (angle 30), and frame 4 tilts further, to 60; the slats turn. | largest difference 1.9e-7 | yes |
| FX-BLINDS-017 frame 2: Angle keyed from 0 at frame 0 to 60 at frame 4, completion 50, width 4: frame 0 is FX-BLINDS-002, frame 2 is FX-BLINDS-008 (angle 30), and frame 4 tilts further, to 60; the slats turn. | largest difference 1.9e-7 | yes |
| FX-BLINDS-017 frame 4: Angle keyed from 0 at frame 0 to 60 at frame 4, completion 50, width 4: frame 0 is FX-BLINDS-002, frame 2 is FX-BLINDS-008 (angle 30), and frame 4 tilts further, to 60; the slats turn. | largest difference 1.9e-7 | yes |
| FX-BLINDS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-018 frame 0: FX-BLINDS-005 moved three pixels right: the slats are the drawing's own, so it is the same, moved. | largest difference 1.9e-7 | yes |
| FX-BLINDS-018 frame 3: FX-BLINDS-005 moved three pixels right: the slats are the drawing's own, so it is the same, moved. | largest difference 1.9e-7 | yes |
| FX-BLINDS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLINDS-019 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-019 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-020 frame 0: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-020 frame 4: Completion 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-021 frame 0: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-021 frame 4: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-022 frame 0: Width 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-022 frame 4: Width 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-023 frame 0: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-023 frame 4: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-024 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-024 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLINDS-025 frame 0: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-025 frame 4: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BLINDS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-BLINDS-026, in dispute (D-164, proposed)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLINDS-026, in dispute (D-164, proposed): the build refuses fx_blinds_026.json as a fault in its shape, as it does a number written as a word in every effect; the case expects it kept with a warning | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the width, 20 to 10, and the feather, 6 to 3 | VenetianBlinds { completion: 50.0, angle: 0.0, width: 10.0, feather: 3.0 } | yes |
| a quarter-size draft of width 1.5 holds the width at 1, the least the command takes, rather than dropping the effect | VenetianBlinds { completion: 50.0, angle: 0.0, width: 1.0, feather: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blinds_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blinds_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `width` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a feather that is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion -1 is refused with a sentence, and nothing changes | Venetian Blinds's completion runs from 0 to 100, and this is -1. | yes |
| completion 100.5 is refused with a sentence, and nothing changes | Venetian Blinds's completion runs from 0 to 100, and this is 100.5. | yes |
| width 0.5 is refused with a sentence, and nothing changes | Venetian Blinds's width runs from 1 to 10000, and this is 0.5. | yes |
| width 10001 is refused with a sentence, and nothing changes | Venetian Blinds's width runs from 1 to 10000, and this is 10001. | yes |
| feather -1 is refused with a sentence, and nothing changes | Venetian Blinds's feather runs from 0 to 10000, and this is -1. | yes |
| angle 3601 is refused with a sentence, and nothing changes | Venetian Blinds's angle runs from -3600 to 3600, and this is 3601. | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Venetian Blinds's completion runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blinds_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blinds_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blinds_018.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

102 of 102 checks pass.
