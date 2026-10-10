# D-402: Tritone

B-281: `core.tritone` takes After Effects' Highlights, Midtones, Shadows and Blend With Original, a second name over Gradient Map's engine (as D-383 and D-394): Gradient Map with its midpoint at the middle and its amount one hundred less the blend. Every expected pixel is `Fixtures/tritone/expected_tritone.json`, written by `tools/tritone_reference.py` before this code existed and printed in document 25 as FX-TRITONE-001 to 015; Gradient Map's FX-GRADMAP-001 to 023 rerun unchanged. Tolerance 2e-5.

## FX-TRITONE-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TRITONE-001 frame 0: The settings as they start: shadows #000000, midtones #8c7355, a sepia brown, highlights #ffffff, blend 0. A sepia print of the drawing by its lightness; black stays black and white white; the yellow at half covering takes the same colour as the yellow, at its own covering. | largest difference 1.5e-7 | yes |
| FX-TRITONE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-002 frame 0: Blend With Original 100: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-TRITONE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-003 frame 0: Blend With Original 50: every pixel halfway, in linear light, between the drawing and FX-TRITONE-001. | largest difference 1.4e-7 | yes |
| FX-TRITONE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-004 frame 0: Shadows #1a2a6c, a navy, midtones #c0392b, a red, highlights #fdf3a7, a pale yellow: a night-to-sunset colouring; black turns exactly the navy and white the pale yellow. | largest difference 1.5e-7 | yes |
| FX-TRITONE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-005 frame 0: All three #6450a0: every pixel that shows is #6450a0 at its own covering, whatever its lightness. | largest difference 3.0e-8 | yes |
| FX-TRITONE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-006 frame 0: FX-TRITONE-004 with its colours written in capitals: the same. | largest difference 1.5e-7 | yes |
| FX-TRITONE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-007 frame 0: FX-TRITONE-004 at blend 70: each pixel 30 per cent of the way, in linear light, toward its colour. | largest difference 1.3e-7 | yes |
| FX-TRITONE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-008 frame 0: Blend keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TRITONE-003, frame 4 is FX-TRITONE-001. | largest difference 1.9e-7 | yes |
| FX-TRITONE-008 frame 2: Blend keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TRITONE-003, frame 4 is FX-TRITONE-001. | largest difference 1.4e-7 | yes |
| FX-TRITONE-008 frame 4: Blend keyed from 100 at frame 0 to 0 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TRITONE-003, frame 4 is FX-TRITONE-001. | largest difference 1.5e-7 | yes |
| FX-TRITONE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-009 frame 0: FX-TRITONE-004 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-TRITONE-009 frame 3: FX-TRITONE-004 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-TRITONE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TRITONE-010 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-010 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TRITONE-011 frame 0: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-011 frame 4: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TRITONE-012 frame 0: Blend With Original keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-012 frame 4: Blend With Original keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TRITONE-013 frame 0: Highlights written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-013 frame 4: Highlights written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TRITONE-014 frame 0: Midtones written "brown", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-014 frame 4: Midtones written "brown", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TRITONE-015 frame 0: Shadows written "#00000g", not a hex digit. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-015 frame 4: Shadows written "#00000g", not a hex digit. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TRITONE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Gradient Map, the engine under the second name: its fixtures, unchanged

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

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tritone_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tritone_006.json, its colours written in capitals, is saved in small letters, as Gradient Map's are | "#fdf3a7" "#c0392b" "#1a2a6c" | yes |
| fx_tritone_007.json is saved with its four settings | {"blend_with_original":70,"highlights":"#fdf3a7","midtones":"#c0392b","shadows":"#1a2a6c"} | yes |
| fx_tritone_010.json is refused in a sentence naming blend_with_original | Tritone's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_tritone_011.json is refused in a sentence naming blend_with_original | Tritone's blend with original runs from 0 to 100, and this is -1. | yes |
| fx_tritone_013.json is refused in a sentence naming Highlights | Tritone's Highlights is written #rrggbb, and this is "#12345". | yes |
| fx_tritone_014.json is refused in a sentence naming Midtones | Tritone's Midtones is written #rrggbb, and this is "brown". | yes |
| fx_tritone_015.json is refused in a sentence naming Shadows | Tritone's Shadows is written #rrggbb, and this is "#00000g". | yes |
| a file with a Tritone whose blend is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tritone without its Midtones is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tritone whose Shadows is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blend 101 is refused with a sentence, and nothing changes | Tritone's blend with original runs from 0 to 100, and this is 101. | yes |
| Midtones #fff is refused with a sentence, and nothing changes | Tritone's Midtones is written #rrggbb, and this is "#fff". | yes |
| blend keyed to 150 is refused with a sentence, and nothing changes | Tritone's blend with original runs from 0 to 100, and this is 150. | yes |
| night to sunset is taken | taken | yes |
| blend keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tritone_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tritone_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tritone_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tritone_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tritone_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tritone_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tritone_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Tritone as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Tritone as added on three layers, frame 0, Full | largest difference 1 of 255, 1213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone as added on three layers, frame 100, Full | largest difference 1 of 255, 1126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone as added on three layers, frame 239, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone as added on three layers, frame 0, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone as added on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone as added on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073596 pixels changed | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 0, Full | largest difference 1 of 255, 549 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 100, Full | largest difference 1 of 255, 627 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 239, Full | largest difference 1 of 255, 592 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 100, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone night to sunset at blend 70 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Tritone one colour three times on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tritone one colour three times on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-402 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| blend with original 100 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (white, a sepia brown, black): the street in browns, every middling pixel redder than it is blue; draws cleanly | [], all warm: true | yes |
| 3_night_to_sunset.png, shadows navy (#1a2a6c), midtones red (#c0392b), highlights pale yellow (#fdf3a7): the dark parts bluer than red, the light parts redder than blue; draws cleanly | [], dark parts blue: true, light parts yellow: true | yes |
| 4_blend_70.png, the same three at blend 70: the street with a light wash of them, nearer the street than 3 is; draws cleanly | [], largest difference from the street 43 against 3's 149 | yes |
| Tritone as added against Gradient Map with the same colours, midpoint 50 and amount 100: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Tritone night to sunset against Gradient Map with the same colours, midpoint 50 and amount 100: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Tritone night to sunset at blend 70 against Gradient Map with the same colours, midpoint 50 and amount 30: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Tritone one colour three times at blend 25 against Gradient Map with the same colours, midpoint 50 and amount 75: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |

## Result

179 of 179 checks pass.
