# D-562: CC Vignette

B-442: `core.cc_vignette` takes After Effects' CC Vignette controls, Amount, Angle of View, Center and Pin Highlights: a second effect beside D-126's Vignette, the light falling off as a lens's does (cos^4 of the angle it reaches the film at), lightening with a negative amount, bright pixels kept by Pin Highlights. Every expected pixel is `Fixtures/cc_vignette/expected_cc_vignette.json`, written by `tools/cc_vignette_reference.py` before this code existed and printed in document 25 as FX-CCVIG-001 to 025. Tolerance 2e-5.

## FX-CCVIG-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CCVIG-001 frame 0: The settings as they start: amount 100, angle of view 90, about the middle, pin highlights 0. Each pixel's linear light is multiplied by the cos^4 falloff: the bottom right pixel keeps 0.289 of it, the middle four keep 0.989; nothing is lightened, and the empty corner stays empty. | largest difference 1.9e-7 | yes |
| FX-CCVIG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-002 frame 0: Amount 50: the square root of FX-CCVIG-001's gain at every pixel, the bottom right keeping 0.538. | largest difference 1.9e-7 | yes |
| FX-CCVIG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CCVIG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-004 frame 0: Amount -100: the edges lightened by the inverse of FX-CCVIG-001's gain, every channel held at 1: the skin near the corners turns white, the middle barely changes. | largest difference 2.5e-7 | yes |
| FX-CCVIG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-005 frame 0: Amount -50: lightened by the inverse square root, less than FX-CCVIG-004. | largest difference 2.4e-7 | yes |
| FX-CCVIG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-006 frame 0: Angle of view 0: no falloff at all: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CCVIG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-007 frame 0: Angle of view 150: a wide lens, a much stronger falloff, the bottom right keeping 0.0059 of its light. | largest difference 1.6e-7 | yes |
| FX-CCVIG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-008 frame 0: Angle of view 30: a long lens, a gentle falloff, the bottom right keeping 0.887. | largest difference 2.1e-7 | yes |
| FX-CCVIG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-009 frame 0: Centre 0, 0, the top left corner: the falloff about that corner, so the top left pixel is all but untouched and the bottom right darkest of all. | largest difference 1.9e-7 | yes |
| FX-CCVIG-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-010 frame 0: Pin highlights 100: each pixel keeps more of itself the brighter it is: the skin (luma 0.86) loses only about a seventh of what FX-CCVIG-001 takes from it, the dark line along row 7 nearly all of it. | largest difference 2.2e-7 | yes |
| FX-CCVIG-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-011 frame 0: Pin highlights 50: half way between FX-CCVIG-001's gain and FX-CCVIG-010's at every pixel. | largest difference 2.0e-7 | yes |
| FX-CCVIG-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-012 frame 0: Amount -100, pin highlights 100: the lightening held back on bright pixels as the darkening is. | largest difference 2.0e-7 | yes |
| FX-CCVIG-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-013 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-CCVIG-002 and frame 4 FX-CCVIG-001. | largest difference 1.9e-7 | yes |
| FX-CCVIG-013 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-CCVIG-002 and frame 4 FX-CCVIG-001. | largest difference 1.9e-7 | yes |
| FX-CCVIG-013 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-CCVIG-002 and frame 4 FX-CCVIG-001. | largest difference 1.9e-7 | yes |
| FX-CCVIG-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-014 frame 0: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-CCVIG-001; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-CCVIG-014 frame 2: Amount eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-CCVIG-001; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-CCVIG-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-015 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-CCVIG-001, frame 2 is about 25, 25 and frame 4 is FX-CCVIG-009. | largest difference 1.9e-7 | yes |
| FX-CCVIG-015 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-CCVIG-001, frame 2 is about 25, 25 and frame 4 is FX-CCVIG-009. | largest difference 2.0e-7 | yes |
| FX-CCVIG-015 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-CCVIG-001, frame 2 is about 25, 25 and frame 4 is FX-CCVIG-009. | largest difference 1.9e-7 | yes |
| FX-CCVIG-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-016 frame 0: Angle of view keyed from 0 at frame 0 to 150 at frame 4, linear: frame 0 the drawing, frame 4 FX-CCVIG-007. | largest difference 1.9e-7 | yes |
| FX-CCVIG-016 frame 2: Angle of view keyed from 0 at frame 0 to 150 at frame 4, linear: frame 0 the drawing, frame 4 FX-CCVIG-007. | largest difference 1.9e-7 | yes |
| FX-CCVIG-016 frame 4: Angle of view keyed from 0 at frame 0 to 150 at frame 4, linear: frame 0 the drawing, frame 4 FX-CCVIG-007. | largest difference 1.6e-7 | yes |
| FX-CCVIG-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-017 frame 0: FX-CCVIG-001 moved three pixels right: the same, moved; the vignette moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-CCVIG-017 frame 3: FX-CCVIG-001 moved three pixels right: the same, moved; the vignette moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-CCVIG-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-018 frame 0: A directional blur, direction 90 and length 4, then CC Vignette as it starts: the blur grew the layer two pixels on every side, and the falloff is still the drawing's own, about its own middle. | largest difference 1.9e-7 | yes |
| FX-CCVIG-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCVIG-019 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-019 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-020 frame 0: Amount -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-020 frame 4: Amount -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-021 frame 0: Angle of view 180, above 179. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-021 frame 4: Angle of view 180, above 179. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-022 frame 0: Angle of view -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-022 frame 4: Angle of view -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-023 frame 0: Pin highlights 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-023 frame 4: Pin highlights 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-024 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-024 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCVIG-025 frame 0: Pin highlights keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-025 frame 4: Pin highlights keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCVIG-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ccvig_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ccvig_012.json is saved with its four settings | {"amount":-100,"angle_of_view":90,"center":[50,50],"pin_highlights":100} | yes |
| fx_ccvig_019.json is refused in a sentence naming amount | CC Vignette's amount runs from -100 to 100, and this is 101. | yes |
| fx_ccvig_020.json is refused in a sentence naming amount | CC Vignette's amount runs from -100 to 100, and this is -101. | yes |
| fx_ccvig_021.json is refused in a sentence naming angle_of_view | CC Vignette's angle of view runs from 0 to 179, and this is 180. | yes |
| fx_ccvig_022.json is refused in a sentence naming angle_of_view | CC Vignette's angle of view runs from 0 to 179, and this is -1. | yes |
| fx_ccvig_023.json is refused in a sentence naming pin_highlights | CC Vignette's pin highlights runs from 0 to 100, and this is 101. | yes |
| fx_ccvig_024.json is refused in a sentence naming center | CC Vignette's center runs from -1000 to 1000, and this is -1001. | yes |
| a file with a CC Vignette whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Vignette without its angle of view is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Vignette whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | CC Vignette's amount runs from -100 to 100, and this is 101. | yes |
| angle of view 180 is refused with a sentence, and nothing changes | CC Vignette's angle of view runs from 0 to 179, and this is 180. | yes |
| pin highlights keyed to 150 is refused with a sentence, and nothing changes | CC Vignette's pin highlights runs from 0 to 100, and this is 150. | yes |
| centre keyed to 50, -1001 is refused with a sentence, and nothing changes | CC Vignette's center runs from -1000 to 1000, and this is -1001. | yes |
| amount -40, pin highlights 60 is taken | taken | yes |
| angle of view keyed from 0 to 179 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ccvig_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ccvig_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ccvig_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ccvig_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Vignette as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2059470 pixels changed | yes |
| the reference shot, CC Vignette as added on three layers, frame 0, Full | largest difference 1 of 255, 1059 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette as added on three layers, frame 100, Full | largest difference 1 of 255, 1116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette as added on three layers, frame 239, Full | largest difference 1 of 255, 1020 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette as added on three layers, frame 0, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette as added on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette as added on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071725 pixels changed | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 0, Full | largest difference 1 of 255, 816 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 100, Full | largest difference 1 of 255, 623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 239, Full | largest difference 1 of 255, 663 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 0, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 100, Draft | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette amount -60, angle of view 150, pin highlights 50 on three layers, frame 239, Draft | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069889 pixels changed | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 0, Full | largest difference 1 of 255, 1424 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 100, Full | largest difference 1 of 255, 1239 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 239, Full | largest difference 1 of 255, 1332 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 0, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Vignette angle of view 120 about 20, 80 on three layers, frame 239, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-562 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| amount 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| angle of view 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (amount 100, angle of view 90): both far corners darker than the street's, the middle within 3 levels a channel; draws cleanly | [], corners 275 and 87 against 519 and 186, middle 400 against 400 | yes |
| 3_amount_minus_100.png, amount -100: the corners lighter than the street's and no pixel darker; draws cleanly | [], corners 733 and 362 against 519 and 186, none darker: true | yes |
| 4_angle_of_view_150.png, a wide lens (angle of view 150): the corners darker still than 2_as_added.png's; draws cleanly | [], corners 21 and 3 against 2's 275 and 87 | yes |
| 5_pin_highlights_100.png, as added with Pin Highlights 100: no pixel darker than in 2_as_added.png, and the picture nearer the street; draws cleanly | [], none darker than 2: true, largest difference from the street 38 against 2's 108 | yes |
| 6_centre_top_left.png, the centre at the top left corner, angle of view 120: the top left as the street, the bottom right darkest; draws cleanly | [], top left 519 against 519, bottom right 3 against 186 | yes |

## Result

169 of 169 checks pass.
