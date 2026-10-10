# D-409: Lens Chromatic Aberration

B-288: `core.chromatic_aberration`'s new form, PLUGINS.md's pick #3 with its RGB Separation row as a mode: radial (a lens falloff, weak in the middle and strong at the edges) or offset (a straight line at an angle), a scale for each channel, and fringe blur. A file without `mode` is D-120's form and draws by D-120's code, untouched. Every expected pixel is `Fixtures/lens_chromatic_aberration/expected_lens_chromatic_aberration.json`, written by `tools/lens_chromatic_aberration_reference.py` before this code existed and printed in document 25 as FX-LENSCA-001 to 026. Tolerance 2e-5.

## FX-LENSCA-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LENSCA-001 frame 0: The new form's settings as they start (radial, amount 3, falloff 0, red 100, green 0, blue -100, no blur): the same picture as D-120's FX-CHROMA-001, red fringing outward, blue inward. | largest difference 2.5e-7 | yes |
| FX-LENSCA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-002 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LENSCA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-003 frame 0: Falloff 100 at amount 6: the shift grows with the distance cubed, so the skin block, near the middle, barely fringes, while the white block's far corners still split. | largest difference 2.5e-7 | yes |
| FX-LENSCA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-004 frame 0: Falloff 50 at amount 6: between FX-LENSCA-003 and no falloff. | largest difference 2.5e-7 | yes |
| FX-LENSCA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-005 frame 0: Scales red 0, green 100, blue 0: only green moves outward; red and blue stay, so magenta fringes inside the blocks' outer edges and green ones outside. | largest difference 1.9e-7 | yes |
| FX-LENSCA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-006 frame 0: Red 200, blue -50: red twice as far out as at the start, blue half as far in. | largest difference 2.5e-7 | yes |
| FX-LENSCA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-007 frame 0: Fringe blur 50 at amount 6: each fringe smeared along its own shift over half its length, softer than FX-CHROMA-003's. | largest difference 2.4e-7 | yes |
| FX-LENSCA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-008 frame 0: Fringe blur 100 at amount 6 with falloff 100: the smear over the whole shift, strongest at the edges. | largest difference 2.5e-7 | yes |
| FX-LENSCA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-009 frame 0: Offset mode, amount 2, angle 90: red moved exactly two pixels right, blue two left, green kept, the same at every pixel. | largest difference 1.9e-7 | yes |
| FX-LENSCA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-010 frame 0: Offset mode, amount 1.5, angle 0: red moved a pixel and a half up, blue as far down, each a blend of two rows. | largest difference 1.9e-7 | yes |
| FX-LENSCA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-011 frame 0: Offset mode, amount 3, angle 45, fringe blur 100: red and blue smeared along the diagonal over their whole shift. | largest difference 2.1e-7 | yes |
| FX-LENSCA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-012 frame 0: Offset mode, angle keyed from 0 at frame 0 to 360 at frame 4, linear: frames 0 and 4 the same, frame 2 the split turned over (red down). | largest difference 1.9e-7 | yes |
| FX-LENSCA-012 frame 2: Offset mode, angle keyed from 0 at frame 0 to 360 at frame 4, linear: frames 0 and 4 the same, frame 2 the split turned over (red down). | largest difference 1.9e-7 | yes |
| FX-LENSCA-012 frame 4: Offset mode, angle keyed from 0 at frame 0 to 360 at frame 4, linear: frames 0 and 4 the same, frame 2 the split turned over (red down). | largest difference 1.9e-7 | yes |
| FX-LENSCA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-013 frame 0: Falloff keyed from 0 at frame 0 to 100 at frame 4 at amount 6: frame 0 D-120's split at 6, frame 4 FX-LENSCA-003. | largest difference 2.5e-7 | yes |
| FX-LENSCA-013 frame 2: Falloff keyed from 0 at frame 0 to 100 at frame 4 at amount 6: frame 0 D-120's split at 6, frame 4 FX-LENSCA-003. | largest difference 2.5e-7 | yes |
| FX-LENSCA-013 frame 4: Falloff keyed from 0 at frame 0 to 100 at frame 4 at amount 6: frame 0 D-120's split at 6, frame 4 FX-LENSCA-003. | largest difference 2.5e-7 | yes |
| FX-LENSCA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-014 frame 0: FX-LENSCA-009 moved three pixels right: the same, moved; the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.9e-7 | yes |
| FX-LENSCA-014 frame 3: FX-LENSCA-009 moved three pixels right: the same, moved; the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.9e-7 | yes |
| FX-LENSCA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-015 frame 0: Radial about 0, 50 with falloff 100 at amount 6: weak by the left edge's middle, strong at the right edge. | largest difference 1.3e-7 | yes |
| FX-LENSCA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-016 frame 0: Offset mode with centre 0, 0 and falloff 100: the centre and the falloff are radial's alone, so the picture is FX-LENSCA-009's. | largest difference 1.9e-7 | yes |
| FX-LENSCA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-017 frame 0: Radial mode with angle 0: the angle is offset's alone, so the picture is FX-LENSCA-001's. | largest difference 2.5e-7 | yes |
| FX-LENSCA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-018 frame 0: Green 100, red and blue 100 too, offset mode amount 1, angle 270: the whole drawing moved a pixel left, as one. | largest difference 1.9e-7 | yes |
| FX-LENSCA-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENSCA-019 frame 0: Mode "spiral", not a mode. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-019 frame 4: Mode "spiral", not a mode. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-020 frame 0: Falloff 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-020 frame 4: Falloff 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-021 frame 0: Red scale 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-021 frame 4: Red scale 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-022 frame 0: Blue scale -201, below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-022 frame 4: Blue scale -201, below -200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-023 frame 0: Fringe blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-023 frame 4: Fringe blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-024 frame 0: Angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-024 frame 4: Angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-025 frame 0: Amount 101 in the new form, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-025 frame 4: Amount 101 in the new form, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENSCA-026 frame 0: Green scale keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-026 frame 4: Green scale keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENSCA-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: D-120's form, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chroma_001.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_001.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_002.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_002.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_003.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_003.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_004.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_004.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_005.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_005.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_006.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_006.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_007.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_007.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_008.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_008.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_009.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_009.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_010.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_010.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |
| fx_chroma_011.json (no mode) opens as D-120's form and is saved as it was written, with no new settings | D-120's form; saved the same | yes |
| fx_chroma_011.json and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit | bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lensca_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lensca_011.json is saved with its nine settings, in offset mode | {"amount":3,"angle":45,"blue_scale":-100,"center":[50,50],"falloff":0,"fringe_blur":100,"green_scale":0,"mode":"offset","red_scale":100} | yes |
| fx_lensca_019.json is refused in a sentence naming mode | Chromatic Aberration's mode is "radial" or "offset", and this is "spiral". | yes |
| fx_lensca_020.json is refused in a sentence naming falloff | Chromatic Aberration's falloff runs from 0 to 100, and this is 101. | yes |
| fx_lensca_021.json is refused in a sentence naming red_scale | Chromatic Aberration's red scale runs from -200 to 200, and this is 201. | yes |
| fx_lensca_022.json is refused in a sentence naming blue_scale | Chromatic Aberration's blue scale runs from -200 to 200, and this is -201. | yes |
| fx_lensca_023.json is refused in a sentence naming fringe_blur | Chromatic Aberration's fringe blur runs from 0 to 100, and this is -1. | yes |
| fx_lensca_024.json is refused in a sentence naming angle | Chromatic Aberration's angle runs from -3600 to 3600, and this is 3601. | yes |
| fx_lensca_025.json is refused in a sentence naming amount | Chromatic Aberration's amount runs from 0 to 100, and this is 101. | yes |
| a file with a new-form Chromatic Aberration without its falloff is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a new-form Chromatic Aberration whose falloff is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a new-form Chromatic Aberration whose mode is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the amount, a distance, and leaves the scales, shares of it | LensChromaticAberration { mode: "offset", amount: 1.0, center: [50.0, 50.0], angle: 90.0, falloff: 0.0, red_scale: 100.0, green_scale: 0.0, blue_scale: -100.0, fringe_blur: 0.0 } | yes |
| it grows the drawing's bounds by nothing | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| mode spiral is refused with a sentence, and nothing changes | Chromatic Aberration's mode is "radial" or "offset", and this is "spiral". | yes |
| falloff 101 is refused with a sentence, and nothing changes | Chromatic Aberration's falloff runs from 0 to 100, and this is 101. | yes |
| red scale 201 is refused with a sentence, and nothing changes | Chromatic Aberration's red scale runs from -200 to 200, and this is 201. | yes |
| fringe blur keyed to 150 is refused with a sentence, and nothing changes | Chromatic Aberration's fringe blur runs from 0 to 100, and this is 150. | yes |
| offset mode with fringe blur 60 is taken | taken | yes |
| falloff keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lensca_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lensca_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lensca_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lensca_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1410825 pixels changed | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 0, Full | largest difference 1 of 255, 3225 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 100, Full | largest difference 1 of 255, 2121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 239, Full | largest difference 1 of 255, 1947 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 0, Draft | largest difference 1 of 255, 104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial, amount 20, falloff 100 on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1690978 pixels changed | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 0, Full | largest difference 1 of 255, 1537 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 100, Full | largest difference 1 of 255, 934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 239, Full | largest difference 1 of 255, 786 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 0, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50 on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1785144 pixels changed | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 0, Full | largest difference 1 of 255, 2142 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 100, Full | largest difference 1 of 255, 2078 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 239, Full | largest difference 1 of 255, 1729 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 0, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Chromatic Aberration offset at 30 degrees, amount 8, fringe blur 100 on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-409 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_no_falloff.png and 3_lens_falloff_100.png, amount 12: with falloff 100 the middle of the street barely splits while the corners still do; both draw cleanly | [] []; the middle moved 96083 without falloff, 3014 with; the top left corner 220889 and 182943 | yes |
| 4_offset_rgb_separation.png, offset mode, amount 6, angle 90: red is the street's red six pixels to the left, blue six to the right, everywhere the street covers; draws cleanly | []; red 126360 and blue 126360 of 126360 pixels | yes |
| 5_fringe_blur_100.png, 3_lens_falloff_100.png with fringe blur 100: the coloured fringes smeared outward, softer; draws cleanly | [], red and blue sideways contrast 522828 against 537034 | yes |
| 6_green_only.png, scales red 0, green 100, blue 0: only green moves, outward; red and blue stay where the street covers; draws cleanly | []; 0 covered pixels changed red or blue, 38252 changed green | yes |

## Result

193 of 193 checks pass.
