# B-33b: glow

D-89, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/glow/expected_glow.json`, written by `tools/glow_reference.py` before this code existed and printed in document 25 as FX-GLOW-001 to 033. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GLOW-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLOW-001 frame 0: Bright parts, threshold 60, radius 4: the yellow patch and the brown one, whose brightest channel is exactly 60 %, glow; the purple does not, though the light of the others reaches it. The glow spreads into the empty space round the patches, which shows it. | largest difference 3.3e-7 | yes |
| FX-GLOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-GLOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-003 frame 0: Radius 0: nothing spreads, and each glowing pixel is added onto itself, twice as bright. | largest difference 3.1e-7 | yes |
| FX-GLOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-004 frame 0: Threshold 100: nothing is that bright, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-GLOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-005 frame 0: Threshold 0: every pixel that shows glows, the purple too. | largest difference 3.3e-7 | yes |
| FX-GLOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-006 frame 0: Threshold 61: the brown, at exactly 60 %, no longer glows. | largest difference 3.3e-7 | yes |
| FX-GLOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-007 frame 0: Chosen colours, the purple chosen: only the purple glows. | largest difference 1.5e-7 | yes |
| FX-GLOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-008 frame 0: The purple chosen one step off in blue, tolerance 0: nothing is chosen, and the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-GLOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-009 frame 0: The same at tolerance 1: the purple is chosen again, and this is FX-GLOW-007. | largest difference 1.5e-7 | yes |
| FX-GLOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-010 frame 0: Chosen colours with none chosen: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-GLOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-011 frame 0: Intensity 2.5: the glow is two and a half times as strong, and where it adds past white it is not cut off. | largest difference 5.4e-7 | yes |
| FX-GLOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-012 frame 0: Operation Screen: the glow is screened on, which never passes white. | largest difference 1.6e-7 | yes |
| FX-GLOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-013 frame 0: Tint #ff4000: the glow is orange, whatever the colour that lit it. | largest difference 2.2e-7 | yes |
| FX-GLOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-014 frame 0: After Effects' own defaults, threshold 60, radius 10, intensity 1, Add. | largest difference 2.0e-7 | yes |
| FX-GLOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-015 frame 0: The patches half covering: they glow as in FX-GLOW-001, at half the strength. | largest difference 1.6e-7 | yes |
| FX-GLOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-016 frame 0: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLOW-003, frame 2 is FX-GLOW-001, frame 4 is radius 8. | largest difference 3.1e-7 | yes |
| FX-GLOW-016 frame 2: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLOW-003, frame 2 is FX-GLOW-001, frame 4 is radius 8. | largest difference 3.3e-7 | yes |
| FX-GLOW-016 frame 4: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLOW-003, frame 2 is FX-GLOW-001, frame 4 is radius 8. | largest difference 2.2e-7 | yes |
| FX-GLOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-017 frame 0: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001, frame 4 has the purple glowing too. | largest difference 1.5e-7 | yes |
| FX-GLOW-017 frame 2: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001, frame 4 has the purple glowing too. | largest difference 3.3e-7 | yes |
| FX-GLOW-017 frame 4: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001, frame 4 has the purple glowing too. | largest difference 3.3e-7 | yes |
| FX-GLOW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-018 frame 0: Intensity keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001. | largest difference 1.5e-7 | yes |
| FX-GLOW-018 frame 2: Intensity keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001. | largest difference 3.3e-7 | yes |
| FX-GLOW-018 frame 4: Intensity keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001. | largest difference 3.9e-7 | yes |
| FX-GLOW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-019 frame 0: FX-GLOW-001 moved three pixels right: the glow is done on the drawing before it is moved, and the part of it that spread past the drawing's left edge now shows in columns 1 and 2. | largest difference 3.3e-7 | yes |
| FX-GLOW-019 frame 3: FX-GLOW-001 moved three pixels right: the glow is done on the drawing before it is moved, and the part of it that spread past the drawing's left edge now shows in columns 1 and 2. | largest difference 3.3e-7 | yes |
| FX-GLOW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-020 frame 0: Radius 2.5: a radius is not rounded, so this is neither radius 2 nor 3. | largest difference 3.9e-7 | yes |
| FX-GLOW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-021 frame 0: FX-GLOW-007 and FX-GLOW-013 with the colours written in capitals: the same. | largest difference 1.5e-7 | yes |
| FX-GLOW-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-022 frame 0: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-022 frame 4: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-023 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-023 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-024 frame 0: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-024 frame 4: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-025 frame 0: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-025 frame 4: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-026 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-026 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-027 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-027 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-028 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-028 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-029 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-029 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-030 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-030 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-031 frame 0: A tint written "orange". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-031 frame 4: A tint written "orange". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-032 frame 0: Operation "multiply", which is not Add or Screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-032 frame 4: Operation "multiply", which is not Add or Screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-033 frame 0: Based on "dark", which is not bright or colors. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-033 frame 4: Based on "dark", which is not bright or colors. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far the light reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 4 grows the drawing's bounds by 4 pixels on each side | 4 | yes |
| radius 2.5 grows the drawing's bounds by 3 pixels on each side | 3 | yes |
| radius 0 grows the drawing's bounds by 0 pixels on each side | 0 | yes |
| radius 500 grows the drawing's bounds by 500 pixels on each side | 500 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_021.json, written in capitals, is saved in small letters | ["#3c286e"] and "#ff4000" | yes |
| a file with no `based_on` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a tint that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a radius that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 501 is refused with a sentence, and nothing changes | Glow's radius runs from 0 to 500, and this is 501. | yes |
| radius -1 is refused with a sentence, and nothing changes | Glow's radius runs from 0 to 500, and this is -1. | yes |
| threshold 101 is refused with a sentence, and nothing changes | Glow's threshold runs from 0 to 100, and this is 101. | yes |
| intensity 11 is refused with a sentence, and nothing changes | Glow's intensity runs from 0 to 10, and this is 11. | yes |
| intensity -1 is refused with a sentence, and nothing changes | Glow's intensity runs from 0 to 10, and this is -1. | yes |
| tolerance 256 is refused with a sentence, and nothing changes | Glow's tolerance runs from 0 to 255, and this is 256. | yes |
| nine colours is refused with a sentence, and nothing changes | Glow takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| the tint "orange" is refused with a sentence, and nothing changes | Glow's colour is empty or written # and six hexadecimal digits, such as #ff4000, and this is "orange". | yes |
| operation "multiply" is refused with a sentence, and nothing changes | Glow's operation is "add" or "screen", and this is "multiply". | yes |
| based on "dark" is refused with a sentence, and nothing changes | Glow is based on "bright" or "colors", and this is "dark". | yes |
| radius keyed to 600 is refused with a sentence, and nothing changes | Glow's radius runs from 0 to 500, and this is 600. | yes |
| intensity keyed to 20 is refused with a sentence, and nothing changes | Glow's intensity runs from 0 to 10, and this is 20. | yes |
| radius 500, threshold 100, intensity 10 and tolerance 255 with eight colours, the ends of the ranges and the other choices, are taken | taken | yes |
| radius 0, threshold 0, intensity 0 and tolerance 0 with no colour, the ends of the ranges and the other choices, are taken | taken | yes |
| chosen colours, Screen and a tint, the ends of the ranges and the other choices, are taken | taken | yes |
| radius keyed from 0 to 8, the ends of the ranges and the other choices, are taken | taken | yes |
| undo four times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glow_019.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## The draft preview (D-66)

| Check | The build's answer | Matches |
| --- | --- | --- |
| an adjustment layer's radius 12 is radius 3 on the quarter-size draft frame, and its threshold 60 and intensity 2 stay as they are | [(3.0, 60.0, 2.0)] | yes |

## Result

126 of 126 checks pass.
