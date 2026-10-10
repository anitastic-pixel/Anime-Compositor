# D-451: Noise HLS

B-331, after After Effects' Noise HLS: three noises from -1 to 1, one each for hue, lightness and saturation, laid on each shown pixel through D-113's HSL: the hue turns up to half the wheel at Hue 100, the lightness and saturation move up to 1 at 100, held in 0 to 1. Uniform is one number a pixel, Squared pushes it towards its ends, Grain is smooth in cells of Grain Size pixels. Noise Phase moves through the noise, a new noise a turn. The covering is kept. Every expected pixel is `Fixtures/noisehls/expected_noisehls.json`, written by `tools/noisehls_reference.py` before this code existed and printed in document 25 as FX-NOISEHLS-001 to 026. Tolerance 2e-5.

## FX-NOISEHLS-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NOISEHLS-001 frame 0: The settings as they start: Uniform, Lightness 10: each shown pixel's lightness moves up to 0.1 either way, its own amount, keeping its hue and saturation and its covering; the empty pixels are left; the same on every frame. | largest difference 2.9e-7 | yes |
| FX-NOISEHLS-001 frame 2: The settings as they start: Uniform, Lightness 10: each shown pixel's lightness moves up to 0.1 either way, its own amount, keeping its hue and saturation and its covering; the empty pixels are left; the same on every frame. | largest difference 2.9e-7 | yes |
| FX-NOISEHLS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-002 frame 0: Hue 50, Lightness 0: each coloured pixel's hue turns up to 90 degrees either way, its lightness and saturation kept; the grey has no hue and is left. | largest difference 2.9e-7 | yes |
| FX-NOISEHLS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-003 frame 0: Saturation 50, Lightness 0: the saturation moves up to 0.5 either way, held in 0 and 1, so the full primaries only lose saturation and the grey only gains it. | largest difference 2.5e-7 | yes |
| FX-NOISEHLS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-004 frame 0: Lightness 100: the lightness moves up to 1 either way, held, so some pixels turn white or black. | largest difference 2.9e-7 | yes |
| FX-NOISEHLS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-005 frame 0: Hue, Lightness and Saturation 30 together, each with its own noise. | largest difference 6.0e-7 | yes |
| FX-NOISEHLS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-006 frame 0: Squared, Lightness 30: the noise pushed out towards its ends, so every pixel's lightness moves at least as far as in Uniform, the same way. | largest difference 4.4e-7 | yes |
| FX-NOISEHLS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-007 frame 0: Grain, Grain Size 1, Lightness 30: smooth noise in cells of a pixel. | largest difference 3.0e-7 | yes |
| FX-NOISEHLS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-008 frame 0: Grain, Grain Size 4, Lightness 30: the cells four pixels across, so neighbouring pixels move nearly together. | largest difference 2.7e-7 | yes |
| FX-NOISEHLS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-009 frame 0: Grain, Grain Size 2.5, Hue 40, Saturation 40, Lightness 0. | largest difference 2.3e-7 | yes |
| FX-NOISEHLS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-010 frame 0: Uniform at phase 180, Lightness 30: half way between the noise at depth 0 and at depth 1. | largest difference 3.5e-7 | yes |
| FX-NOISEHLS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-011 frame 0: Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Lightness 30: a new noise each turn, reached smoothly; frame 1 is phase 180, FX-NOISEHLS-010's. | largest difference 4.0e-7 | yes |
| FX-NOISEHLS-011 frame 1: Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Lightness 30: a new noise each turn, reached smoothly; frame 1 is phase 180, FX-NOISEHLS-010's. | largest difference 3.5e-7 | yes |
| FX-NOISEHLS-011 frame 2: Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Lightness 30: a new noise each turn, reached smoothly; frame 1 is phase 180, FX-NOISEHLS-010's. | largest difference 4.2e-7 | yes |
| FX-NOISEHLS-011 frame 4: Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Lightness 30: a new noise each turn, reached smoothly; frame 1 is phase 180, FX-NOISEHLS-010's. | largest difference 4.2e-7 | yes |
| FX-NOISEHLS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-012 frame 0: Phase -90, Hue 100: the phase runs below 0 too, a quarter of the way back from depth 0 to depth -1. | largest difference 3.0e-7 | yes |
| FX-NOISEHLS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-013 frame 0: Hue, Lightness and Saturation 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-013 frame 2: Hue, Lightness and Saturation 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-014 frame 0: FX-NOISEHLS-005 moved three pixels right: the noise is the drawing's own, so it moves with it. | largest difference 6.0e-7 | yes |
| FX-NOISEHLS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-015 frame 0: After a Motion Tile that grows the layer: the noise is worked in the drawing's own pixels, so the frame is FX-NOISEHLS-005's. | largest difference 6.0e-7 | yes |
| FX-NOISEHLS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-016 frame 0: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-016 frame 2: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 3.6e-7 | yes |
| FX-NOISEHLS-016 frame 4: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 6.6e-7 | yes |
| FX-NOISEHLS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-017 frame 0: Grain, Grain Size 0.5, Lightness 30: cells of half a pixel. | largest difference 4.1e-7 | yes |
| FX-NOISEHLS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-018 frame 0: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to 500: the controls together. | largest difference 3.1e-7 | yes |
| FX-NOISEHLS-018 frame 1: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to 500: the controls together. | largest difference 2.6e-7 | yes |
| FX-NOISEHLS-018 frame 2: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to 500: the controls together. | largest difference 3.3e-7 | yes |
| FX-NOISEHLS-018 frame 3: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to 500: the controls together. | largest difference 3.0e-7 | yes |
| FX-NOISEHLS-018 frame 4: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, phase keyed 0 to 500: the controls together. | largest difference 3.7e-7 | yes |
| FX-NOISEHLS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLS-019 frame 0: Hue 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-019 frame 4: Hue 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-020 frame 0: Lightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-020 frame 4: Lightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-021 frame 0: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-021 frame 4: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-022 frame 0: Grain Size 0.25, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-022 frame 4: Grain Size 0.25, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-023 frame 0: Noise Phase 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-023 frame 4: Noise Phase 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-024 frame 0: Noise "Uniform", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-024 frame 4: Noise "Uniform", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-025 frame 0: Noise "grainy", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-025 frame 4: Noise "grainy", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLS-026 frame 0: Lightness keyed to 120 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-026 frame 4: Lightness keyed to 120 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehls_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehls_018.json is saved with its words and numbers as written, and Noise Phase's keys kept | {"grain_size":3,"hue":20,"lightness":25,"noise":"grain","noise_phase":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":500}]},"saturation":60} | yes |
| fx_noisehls_019.json is refused in a sentence | Noise HLS's hue runs from 0 to 100, and this is 101. | yes |
| fx_noisehls_020.json is refused in a sentence | Noise HLS's lightness runs from 0 to 100, and this is -1. | yes |
| fx_noisehls_021.json is refused in a sentence | Noise HLS's saturation runs from 0 to 100, and this is 150. | yes |
| fx_noisehls_022.json is refused in a sentence | Noise HLS's grain size runs from 0.5 to 100, and this is 0.25. | yes |
| fx_noisehls_023.json is refused in a sentence | Noise HLS's noise phase runs from -100000 to 100000, and this is 200000. | yes |
| fx_noisehls_024.json is refused in a sentence | Noise HLS's noise is one of uniform, squared, grain, and this is "Uniform". | yes |
| fx_noisehls_025.json is refused in a sentence | Noise HLS's noise is one of uniform, squared, grain, and this is "grainy". | yes |
| a file with a Noise HLS with no `noise` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise HLS whose noise is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise HLS whose hue is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue 101 is refused with a sentence, and nothing changes | Noise HLS's hue runs from 0 to 100, and this is 101. | yes |
| saturation -5 is refused with a sentence, and nothing changes | Noise HLS's saturation runs from 0 to 100, and this is -5. | yes |
| grain size 0.4 is refused with a sentence, and nothing changes | Noise HLS's grain size runs from 0.5 to 100, and this is 0.4. | yes |
| noise phase 100001 is refused with a sentence, and nothing changes | Noise HLS's noise phase runs from -100000 to 100000, and this is 100001. | yes |
| noise "Grain" is refused with a sentence, and nothing changes | Noise HLS's noise is one of uniform, squared, grain, and this is "Grain". | yes |
| lightness keyed to 120 is refused with a sentence, and nothing changes | Noise HLS's lightness runs from 0 to 100, and this is 120. | yes |
| Grain, hue 35, lightness 20, saturation 55, grain size 2.5, phase 240 is taken | taken | yes |
| lightness keyed from 0 to 40 is taken | taken | yes |
| noise phase keyed from 0 to 720 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehls_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_011.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehls_018.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehls_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_noisehls_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehls_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehls_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2046150 pixels changed | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 0, Full | largest difference 1 of 255, 933 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 100, Full | largest difference 1 of 255, 1093 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 239, Full | largest difference 1 of 255, 836 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS as added (Uniform, lightness 10) on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2001444 pixels changed | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 0, Full | largest difference 1 of 255, 4623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 100, Full | largest difference 1 of 255, 4991 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 239, Full | largest difference 1 of 255, 4041 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 96 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Squared, hue 60, saturation 40, lightness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 109 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072478 pixels changed | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 0, Full | largest difference 1 of 255, 1002 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 100, Full | largest difference 1 of 255, 1067 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 239, Full | largest difference 1 of 255, 894 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 0, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 100, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200 on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2059511 pixels changed | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 0, Full | largest difference 1 of 255, 905 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 100, Full | largest difference 1 of 255, 1138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 239, Full | largest difference 1 of 255, 790 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-451 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: the street flecked lighter and darker pixel by pixel, a lightness tenth moving a channel by no more than a fifth; draws cleanly | [], 127673 pixels changed of 129600, the largest by 51 of 255 | yes |
| 3_hue_60.png, frame 0, Hue 60, Lightness 0: the colours flecked round the wheel, each pixel keeping its lightness; draws cleanly | [], 126860 pixels changed of 129600, the largest by 110 of 255 | yes |
| 4_grain_4_saturation_60.png, frame 0, Grain, grain size 4, Saturation 60, Lightness 20: soft blotches of stronger and weaker colour; draws cleanly | [], 129561 pixels changed of 129600, the largest by 134 of 255 | yes |
| 5_phase_frame_24.png, frame 24, Squared, Lightness 25, phase keyed 0 to 470 over the shot, at frame 24; draws cleanly | [], 129098 pixels changed of 129600, the largest by 127 of 255 | yes |

## Result

184 of 184 checks pass.
