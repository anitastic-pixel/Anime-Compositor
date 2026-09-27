# B-94: twirl

D-151, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the eighteenth of the third batch. Every expected pixel is `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before this code existed and printed in document 25 as FX-TWIRL-001 to 021. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TWIRL-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TWIRL-001 frame 0: The settings as they start: angle 90, radius 50, about the middle, the point (8, 5). The whole drawing lies inside the circle, so every pixel is turned clockwise, by about 87 degrees beside the centre and by about 61 at the corners: the two lines bend into a spiral about the middle, and column 15 and rows 2 to 9 of column 0, which take their picture from past the drawing's top and bottom edges, are empty. Pixels deep in the skin take skin and look unchanged. | largest difference 2.5e-7 | yes |
| FX-TWIRL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-002 frame 0: Angle -90: the same twirl the other way, counter-clockwise; each pixel takes its picture from the point FX-TWIRL-001 takes it from, mirrored across the line from the centre to the pixel. | largest difference 2.5e-7 | yes |
| FX-TWIRL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-003 frame 0: Angle 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-TWIRL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-004 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-TWIRL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-005 frame 0: Radius 4: only the pixels whose centres are less than 4 pixels from the middle are turned, the crossing of the two lines swirled; every pixel 4 or more from it is kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-006 frame 0: Angle 360, radius 8: a whole turn at the very centre, less further out, so the lines wind round the middle; the corners, 8 or more from it, are kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-007 frame 0: Angle 3600, the most: ten turns at the centre, the picture wound so tightly that the middle pixels sample it almost at random. | largest difference 2.6e-7 | yes |
| FX-TWIRL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-008 frame 0: Radius 10000: the falloff is so slow that every pixel is turned by within a fifth of a degree of 90, nearly a plain quarter turn about the middle: the drawing stands on end in columns 3 to 12, the line down column 8 now runs along row 5 and the line along row 5 runs down column 7, and the three columns on either side, turned in from past the drawing's top and bottom, are empty or nearly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-009 frame 0: Centre 0, 0, the top left corner, radius 6: only the pixels less than 6 from that corner are turned, the soft edge among them; the rest is kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-010 frame 0: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 1.9e-7 | yes |
| FX-TWIRL-010 frame 2: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 2.5e-7 | yes |
| FX-TWIRL-010 frame 4: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 2.5e-7 | yes |
| FX-TWIRL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-011 frame 0: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 1.9e-7 | yes |
| FX-TWIRL-011 frame 2: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 2.5e-7 | yes |
| FX-TWIRL-011 frame 4: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 2.5e-7 | yes |
| FX-TWIRL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-012 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 2.5e-7 | yes |
| FX-TWIRL-012 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 2.5e-7 | yes |
| FX-TWIRL-012 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 2.5e-7 | yes |
| FX-TWIRL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-013 frame 0: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would pass 3600, is held at 3600, and is FX-TWIRL-007; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-TWIRL-013 frame 2: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would pass 3600, is held at 3600, and is FX-TWIRL-007; frame 0 is the drawing. | largest difference 2.6e-7 | yes |
| FX-TWIRL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-014 frame 0: FX-TWIRL-001 moved three pixels right: the same, moved; the twirl moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.5e-7 | yes |
| FX-TWIRL-014 frame 3: FX-TWIRL-001 moved three pixels right: the same, moved; the twirl moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.5e-7 | yes |
| FX-TWIRL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-015 frame 0: A directional blur, direction 90 and length 4, then angle 90 about centre 25, 25: the blur grew the layer two pixels on every side, its grown pixels are turned too, and the centre is still the drawing's own point (4, 2.5), not a point of the grown layer. | largest difference 2.7e-7 | yes |
| FX-TWIRL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-016 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-016 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-017 frame 0: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-017 frame 4: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-018 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-018 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-019 frame 0: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-019 frame 4: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-020 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-020 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-021 frame 0: Radius keyed to 20000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-021 frame 4: Radius keyed to 20000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the radius, 50 to 25, and nothing else | Twirl { angle: 90.0, radius: 25.0, center: [50.0, 50.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `center` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an angle that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| angle 3601 is refused with a sentence, and nothing changes | Twirl's angle runs from -3600 to 3600, and this is 3601. | yes |
| angle -3601 is refused with a sentence, and nothing changes | Twirl's angle runs from -3600 to 3600, and this is -3601. | yes |
| radius -1 is refused with a sentence, and nothing changes | Twirl's radius runs from 0 to 10000, and this is -1. | yes |
| radius 10001 is refused with a sentence, and nothing changes | Twirl's radius runs from 0 to 10000, and this is 10001. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Twirl's center runs from -1000 to 1000, and this is 1001. | yes |
| radius keyed to 20000 is refused with a sentence, and nothing changes | Twirl's radius runs from 0 to 10000, and this is 20000. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| angle keyed from 0 to 180 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twirl_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twirl_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twirl_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

85 of 85 checks pass.
