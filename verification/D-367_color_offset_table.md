# D-367: Color Offset

B-246, after CycoreFX's CC Color Offset: each colour channel turned round by its own phase, what runs past white brought back by wrapping, folding (solarize) or folding smoothly (polarize). Every expected pixel is `Fixtures/color_offset/expected_color_offset.json`, written by `tools/color_offset_reference.py` before this code existed and printed in document 25 as FX-COFFSET-001 to 016. Tolerance 2e-5.

## FX-COFFSET-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-COFFSET-001 frame 0: All phases 0, the settings as they start: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-COFFSET-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-002 frame 0: Red phase 90, wrap: red raised a quarter of the range, and what passes white comes round from black, so the reddest colours turn dark red. | largest difference 1.5e-7 | yes |
| FX-COFFSET-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-003 frame 0: Red 90, green 180, blue 270, wrap: each channel turned its own way, the colours scrambled into new ones. | largest difference 1.5e-7 | yes |
| FX-COFFSET-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-004 frame 0: All three -90, wrap: every value lowered a quarter, and what passes black comes round from white, so the darks turn light. | largest difference 1.2e-7 | yes |
| FX-COFFSET-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-005 frame 0: All three 180, solarize: values below half rise by half, values above half are folded back down from white. | largest difference 1.7e-7 | yes |
| FX-COFFSET-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-006 frame 0: All three 360, solarize: one whole turn folds every value over, the picture's negative. | largest difference 6.7e-8 | yes |
| FX-COFFSET-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-007 frame 0: All three 720, solarize: two turns bring the picture back. | largest difference 1.9e-7 | yes |
| FX-COFFSET-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-008 frame 0: All three 180, polarize: like FX-COFFSET-005 but smooth, with no sharp fold at white. | largest difference 3.1e-7 | yes |
| FX-COFFSET-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-009 frame 0: All three 360, polarize: one whole turn is the negative, as solarize's. | largest difference 6.7e-8 | yes |
| FX-COFFSET-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-010 frame 0: Red 45, green -135, blue 600, polarize: each channel its own way round. | largest difference 1.3e-7 | yes |
| FX-COFFSET-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-011 frame 0: Red phase keyed from 0 at frame 0 to 400 at frame 4, linear, wrap: frame 0 untouched, frame 2 red 200, frame 4 red 400. | largest difference 1.9e-7 | yes |
| FX-COFFSET-011 frame 2: Red phase keyed from 0 at frame 0 to 400 at frame 4, linear, wrap: frame 0 untouched, frame 2 red 200, frame 4 red 400. | largest difference 1.3e-7 | yes |
| FX-COFFSET-011 frame 4: Red phase keyed from 0 at frame 0 to 400 at frame 4, linear, wrap: frame 0 untouched, frame 2 red 200, frame 4 red 400. | largest difference 1.7e-7 | yes |
| FX-COFFSET-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-012 frame 0: FX-COFFSET-003 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-COFFSET-012 frame 3: FX-COFFSET-003 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-COFFSET-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COFFSET-013 frame 0: Red phase 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-013 frame 4: Red phase 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COFFSET-014 frame 0: Blue phase -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-014 frame 4: Blue phase -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COFFSET-015 frame 0: Overflow "mirror", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-015 frame 4: Overflow "mirror", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COFFSET-016 frame 0: Overflow "Wrap": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-016 frame 4: Overflow "Wrap": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COFFSET-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_coffset_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_coffset_010.json is saved with its three phases and overflow | {"blue_phase":600,"green_phase":-135,"overflow":"polarize","red_phase":45} | yes |
| fx_coffset_015.json is refused in a sentence | Color Offset's overflow is "wrap", "solarize" or "polarize", and this is "mirror". | yes |
| a file with a Color Offset with no `blue_phase` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Offset whose overflow is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red phase 3600.5 is refused with a sentence, and nothing changes | Color Offset's red phase runs from -3600 to 3600, and this is 3600.5. | yes |
| green phase -3600.5 is refused with a sentence, and nothing changes | Color Offset's green phase runs from -3600 to 3600, and this is -3600.5. | yes |
| overflow "Polarize", written with a capital is refused with a sentence, and nothing changes | Color Offset's overflow is "wrap", "solarize" or "polarize", and this is "Polarize". | yes |
| blue phase keyed to 4000 is refused with a sentence, and nothing changes | Color Offset's blue phase runs from -3600 to 3600, and this is 4000. | yes |
| red 90, green 180, blue 270, solarize, is taken | taken | yes |
| green phase keyed from 0 to 720 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_coffset_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_coffset_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_coffset_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_coffset_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_coffset_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_coffset_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_coffset_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_coffset_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_coffset_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_coffset_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_coffset_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_coffset_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 0, Full | largest difference 1 of 255, 230054 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 100, Full | largest difference 1 of 255, 218647 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 239, Full | largest difference 1 of 255, 228634 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 0, Draft | largest difference 1 of 255, 8393 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 100, Draft | largest difference 1 of 255, 8008 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 90, green 180, blue 270, wrap on three layers, frame 239, Draft | largest difference 1 of 255, 8478 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073547 pixels changed | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 0, Full | largest difference 1 of 255, 901633 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 100, Full | largest difference 1 of 255, 887952 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 239, Full | largest difference 1 of 255, 902619 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 0, Draft | largest difference 1 of 255, 21341 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 100, Draft | largest difference 1 of 255, 21416 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset all three 180, solarize on three layers, frame 239, Draft | largest difference 1 of 255, 21420 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 0, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Offset red 45, green -135, blue 600, polarize on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-367 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_wrap_red_120.png, red phase 120, wrap: reds past two thirds come round to dark; draws cleanly | [] | yes |
| 3_solarize_all_180.png, all three 180, solarize: the darks lifted, the lights folded down; draws cleanly | [] | yes |
| 4_polarize_all_360.png, all three 360, polarize: the street's negative; draws cleanly | [] | yes |
| 5_polarize_mixed.png, red 60, green -150, blue 240, polarize: shifted psychedelic colours; draws cleanly | [] | yes |
| 4_polarize_all_360.png is the street's negative, each channel 255 less its value, within 1 level | largest difference 0 of 255 | yes |

## Result

114 of 114 checks pass.
