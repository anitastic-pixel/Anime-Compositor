# B-72: gradient map

D-129, accepted by the owner on 2026-09-26, the seventh of the second batch of ten. Every expected pixel is `Fixtures/gradient_map/expected_gradient_map.json`, written by `tools/gradient_map_reference.py` before this code existed and printed in document 25 as FX-GRADMAP-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GRADMAP-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRADMAP-001 frame 0: The settings as they start: black, #808080 at the midpoint 50, white, amount 100. Every pixel that shows turns grey by its brightness, red, green and blue alike: the white stays white, the black stays black, the skin is the lightest of the rest and the line the darkest; a pixel at half covering takes the same grey as its colour at full covering, at its own covering, and the empty pixels stay empty. | largest difference 1.0e-7 | yes |
| FX-GRADMAP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-003 frame 0: Amount 50: every pixel halfway, in linear light, between the drawing and FX-GRADMAP-001. | largest difference 1.7e-7 | yes |
| FX-GRADMAP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-004 frame 0: A sunset ramp, #2a1650, #c85a50 at 50, #ffe6b4: the black turns #2a1650 and the white #ffe6b4 exactly, each at its own covering; the line is deep violet, the red and the blue a dusky red, the skin and its shadow peach. | largest difference 8.8e-8 | yes |
| FX-GRADMAP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-005 frame 0: Midpoint 25: the midtone grey is reached at a quarter of the brightness, so every pixel between black and white is lighter than in FX-GRADMAP-001, and black and white are as they were. | largest difference 8.1e-8 | yes |
| FX-GRADMAP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-006 frame 0: Midpoint 75: every pixel between black and white is darker than in FX-GRADMAP-001, and black and white are as they were. | largest difference 1.5e-7 | yes |
| FX-GRADMAP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-007 frame 0: Midpoint 1, the least: every pixel between black and white is at or above the midpoint, so each lies on the grey-to-white half of the ramp, at least #808080; black stays black. | largest difference 6.4e-8 | yes |
| FX-GRADMAP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-008 frame 0: Midpoint 99, the most: every pixel between black and white lies on the black-to-grey half, at most #808080; white stays white. | largest difference 3.0e-8 | yes |
| FX-GRADMAP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-009 frame 0: All three colours #6450a0: every pixel that shows is #6450a0 at its own covering, whatever its brightness. | largest difference 3.0e-8 | yes |
| FX-GRADMAP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-010 frame 0: Shadow white and highlight black, the ramp turned over: a negative in grey; the white turns black, the black turns white, and the line, the darkest colour, is now the lightest but the black. | largest difference 4.0e-8 | yes |
| FX-GRADMAP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-011 frame 0: Only the midtone changed, to #ff0000: black and white stay black and white, and every pixel between is reddened, its green and blue the same and below its red. | largest difference 1.4e-7 | yes |
| FX-GRADMAP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-012 frame 0: Midpoint keyed from 25 at frame 0 to 75 at frame 4, linear: frame 0 is FX-GRADMAP-005, frame 2, at 50, is FX-GRADMAP-001, frame 4 is FX-GRADMAP-006. | largest difference 8.1e-8 | yes |
| FX-GRADMAP-012 frame 2: Midpoint keyed from 25 at frame 0 to 75 at frame 4, linear: frame 0 is FX-GRADMAP-005, frame 2, at 50, is FX-GRADMAP-001, frame 4 is FX-GRADMAP-006. | largest difference 1.0e-7 | yes |
| FX-GRADMAP-012 frame 4: Midpoint keyed from 25 at frame 0 to 75 at frame 4, linear: frame 0 is FX-GRADMAP-005, frame 2, at 50, is FX-GRADMAP-001, frame 4 is FX-GRADMAP-006. | largest difference 1.5e-7 | yes |
| FX-GRADMAP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-013 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GRADMAP-003, frame 4 is FX-GRADMAP-001. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-013 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GRADMAP-003, frame 4 is FX-GRADMAP-001. | largest difference 1.7e-7 | yes |
| FX-GRADMAP-013 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GRADMAP-003, frame 4 is FX-GRADMAP-001. | largest difference 1.0e-7 | yes |
| FX-GRADMAP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-014 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 2 is held at 100 and is FX-GRADMAP-001, as frame 4 is. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-014 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 2 is held at 100 and is FX-GRADMAP-001, as frame 4 is. | largest difference 1.0e-7 | yes |
| FX-GRADMAP-014 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 2 is held at 100 and is FX-GRADMAP-001, as frame 4 is. | largest difference 1.0e-7 | yes |
| FX-GRADMAP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-015 frame 0: FX-GRADMAP-004 with its colours written in capitals: the same. | largest difference 8.8e-8 | yes |
| FX-GRADMAP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-016 frame 0: FX-GRADMAP-004 moved three pixels right: the same, moved. | largest difference 8.8e-8 | yes |
| FX-GRADMAP-016 frame 3: FX-GRADMAP-004 moved three pixels right: the same, moved. | largest difference 8.8e-8 | yes |
| FX-GRADMAP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRADMAP-017 frame 0: Midpoint 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-017 frame 4: Midpoint 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-018 frame 0: Midpoint 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-018 frame 4: Midpoint 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-019 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-019 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-020 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-020 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-021 frame 0: Midpoint keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-021 frame 4: Midpoint keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-022 frame 0: A shadow colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-022 frame 4: A shadow colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRADMAP-023 frame 0: A highlight colour written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-023 frame 4: A highlight colour written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRADMAP-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | GradientMap { shadow_color: "#000000", midtone_color: "#808080", highlight_color: "#ffffff", midpoint: 40.0, amount: 80.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gradmap_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gradmap_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `midtone_color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a midpoint that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| midpoint 0 is refused with a sentence, and nothing changes | Gradient Map's midpoint runs from 1 to 99, and this is 0. | yes |
| midpoint 100 is refused with a sentence, and nothing changes | Gradient Map's midpoint runs from 1 to 99, and this is 100. | yes |
| amount 101 is refused with a sentence, and nothing changes | Gradient Map's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Gradient Map's amount runs from 0 to 100, and this is -1. | yes |
| shadow colour "#12345" is refused with a sentence, and nothing changes | Gradient Map's shadow colour is written #rrggbb, and this is "#12345". | yes |
| midtone colour "grey" is refused with a sentence, and nothing changes | Gradient Map's midtone colour is written #rrggbb, and this is "grey". | yes |
| highlight colour "white" is refused with a sentence, and nothing changes | Gradient Map's highlight colour is written #rrggbb, and this is "white". | yes |
| midpoint keyed to 150 is refused with a sentence, and nothing changes | Gradient Map's midpoint runs from 1 to 99, and this is 150. | yes |
| midpoint 99 and amount 100, the tops, is taken | taken | yes |
| midpoint 1 and amount 0, the bottoms, is taken | taken | yes |
| amount keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gradmap_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gradmap_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

90 of 90 checks pass.
