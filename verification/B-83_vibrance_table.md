# B-83: vibrance

D-140, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the seventh of the third batch. Every expected pixel is `Fixtures/vibrance/expected_vibrance.json`, written by `tools/vibrance_reference.py` before this code existed and printed in document 25 as FX-VIBRANCE-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-VIBRANCE-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-VIBRANCE-001 frame 0: Vibrance 0 and saturation 0, the settings as they start: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-002 frame 0: Vibrance 100: the dull colours wake up most, the tan pushed 1.84 times as far from its grey and the skin 1.78 times (its red held at the top), the vivid blue and red trace only about 1.38 times (the blue's blue held at the top); the pure red, as vivid as can be, is left exactly as it is, and the grey is left as it is, having no colour to push. | largest difference 1.3e-7 | yes |
| FX-VIBRANCE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-003 frame 0: Vibrance -100: the dull colours fade most, the tan to a near grey (0.16 of its colour left) and the skin to 0.22, while the blue and red trace keep about 0.62 of theirs and the pure red keeps all of it. | largest difference 1.1e-7 | yes |
| FX-VIBRANCE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-004 frame 0: Saturation 100: every colour pushed twice as far from its grey, dull or vivid alike, each channel held between 0 and 1 (the skin's red, the blue's blue and the red trace's red held at the top); the pure red stays pure red, and the grey stays grey. | largest difference 1.4e-7 | yes |
| FX-VIBRANCE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-005 frame 0: Saturation -100: every colour becomes its own grey, the brightness kept: the pure red turns the grey #363636 but for rounding (54.2), and the grey stays as it is. | largest difference 9.1e-8 | yes |
| FX-VIBRANCE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-006 frame 0: Vibrance 50 and saturation -50: the two pull against each other, so the dull colours barely move (the line keeps 0.98 of its colour, the tan 0.92) while the vivid ones lose most (the blue and red trace keep about 0.69, the pure red half). | largest difference 1.6e-7 | yes |
| FX-VIBRANCE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-007 frame 0: Vibrance 100 and saturation -100: each colour keeps exactly the share of its colour that was missing from it, 1 - s: the tan keeps 0.84, the skin 0.78, the blue 0.38, and the pure red none, turning grey as in FX-VIBRANCE-005. | largest difference 1.7e-7 | yes |
| FX-VIBRANCE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-008 frame 0: Vibrance -100 and saturation 100: the other way round, each colour pushed 1 + s times from its grey, so the vivid colours move most (the blue's blue and the red trace's red held at the top) and the line barely moves (1.04). | largest difference 2.2e-7 | yes |
| FX-VIBRANCE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-009 frame 0: Vibrance 40 and saturation 20, an everyday grade: every colour pushed 1.2 + 0.4 (1 - s) times from its grey, the dull ones more (the tan 1.54) than the vivid (the blue 1.35), the skin's red held at the top; the grey stays, and the pure red, pushed past both its ends, stays pure red. | largest difference 1.5e-7 | yes |
| FX-VIBRANCE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-010 frame 0: Vibrance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 vibrance 50, frame 4 FX-VIBRANCE-002. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-010 frame 2: Vibrance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 vibrance 50, frame 4 FX-VIBRANCE-002. | largest difference 1.5e-7 | yes |
| FX-VIBRANCE-010 frame 4: Vibrance keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 vibrance 50, frame 4 FX-VIBRANCE-002. | largest difference 1.3e-7 | yes |
| FX-VIBRANCE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-011 frame 0: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-VIBRANCE-005, frame 2, both at 0, is the drawing untouched, and frame 4 is FX-VIBRANCE-004. | largest difference 9.1e-8 | yes |
| FX-VIBRANCE-011 frame 2: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-VIBRANCE-005, frame 2, both at 0, is the drawing untouched, and frame 4 is FX-VIBRANCE-004. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-011 frame 4: Saturation keyed from -100 at frame 0 to 100 at frame 4: frame 0 is FX-VIBRANCE-005, frame 2, both at 0, is the drawing untouched, and frame 4 is FX-VIBRANCE-004. | largest difference 1.4e-7 | yes |
| FX-VIBRANCE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-012 frame 0: Vibrance eased from 0 at frame 0 to -100 at frame 4 on a curve that overshoots: at frame 2 it has gone past -100 and is held there, so frames 2 and 4 are both FX-VIBRANCE-003. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-012 frame 2: Vibrance eased from 0 at frame 0 to -100 at frame 4 on a curve that overshoots: at frame 2 it has gone past -100 and is held there, so frames 2 and 4 are both FX-VIBRANCE-003. | largest difference 1.1e-7 | yes |
| FX-VIBRANCE-012 frame 4: Vibrance eased from 0 at frame 0 to -100 at frame 4 on a curve that overshoots: at frame 2 it has gone past -100 and is held there, so frames 2 and 4 are both FX-VIBRANCE-003. | largest difference 1.1e-7 | yes |
| FX-VIBRANCE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-013 frame 0: FX-VIBRANCE-009 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-VIBRANCE-013 frame 3: FX-VIBRANCE-009 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-VIBRANCE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VIBRANCE-014 frame 0: Vibrance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-014 frame 4: Vibrance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIBRANCE-015 frame 0: Vibrance -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-015 frame 4: Vibrance -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIBRANCE-016 frame 0: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-016 frame 4: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIBRANCE-017 frame 0: Saturation -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-017 frame 4: Saturation -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VIBRANCE-018 frame 0: Vibrance keyed to -150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-018 frame 4: Vibrance keyed to -150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-VIBRANCE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | Vibrance { vibrance: 40.0, saturation: 20.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vibrance_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vibrance_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `saturation` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a vibrance that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| vibrance 101 is refused with a sentence, and nothing changes | Vibrance's vibrance runs from -100 to 100, and this is 101. | yes |
| vibrance -101 is refused with a sentence, and nothing changes | Vibrance's vibrance runs from -100 to 100, and this is -101. | yes |
| saturation 101 is refused with a sentence, and nothing changes | Vibrance's saturation runs from -100 to 100, and this is 101. | yes |
| saturation -101 is refused with a sentence, and nothing changes | Vibrance's saturation runs from -100 to 100, and this is -101. | yes |
| vibrance keyed to -150 is refused with a sentence, and nothing changes | Vibrance's vibrance runs from -100 to 100, and this is -150. | yes |
| vibrance -100 and saturation 100, the ends, is taken | taken | yes |
| vibrance 100 and saturation -100, the other ends, is taken | taken | yes |
| saturation keyed from -100 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vibrance_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vibrance_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vibrance_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

72 of 72 checks pass.
