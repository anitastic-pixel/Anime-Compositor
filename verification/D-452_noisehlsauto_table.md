# D-452: Noise HLS Auto

B-332, after After Effects' Noise HLS Auto: D-451's Noise HLS (three noises laid on each shown pixel's hue, lightness and saturation through D-113's HSL; Uniform, Squared, Grain) whose noise moves by itself: its depth is the frame times Noise Animation Speed, so at speed 1 every frame is a whole new noise and at 0 it holds still, as D-443's Add Grain reads its speed. The covering is kept. Every expected pixel is `Fixtures/noisehlsauto/expected_noisehlsauto.json`, written by `tools/noisehlsauto_reference.py` before this code existed and printed in document 25 as FX-NOISEHLSAUTO-001 to 024. Tolerance 2e-5.

## FX-NOISEHLSAUTO-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NOISEHLSAUTO-001 frame 0: The settings as they start: Uniform, Lightness 10, speed 1: each shown pixel's lightness moves up to 0.1 either way, a whole new noise every frame; the covering kept and the empty pixels left. | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-001 frame 1: The settings as they start: Uniform, Lightness 10, speed 1: each shown pixel's lightness moves up to 0.1 either way, a whole new noise every frame; the covering kept and the empty pixels left. | largest difference 2.7e-7 | yes |
| FX-NOISEHLSAUTO-001 frame 2: The settings as they start: Uniform, Lightness 10, speed 1: each shown pixel's lightness moves up to 0.1 either way, a whole new noise every frame; the covering kept and the empty pixels left. | largest difference 2.8e-7 | yes |
| FX-NOISEHLSAUTO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-002 frame 0: Speed 0: the noise holds still, every frame Noise HLS's at phase 0 (FX-NOISEHLS-001). | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-002 frame 2: Speed 0: the noise holds still, every frame Noise HLS's at phase 0 (FX-NOISEHLS-001). | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-003 frame 1: Speed 0.5, Lightness 30: frame 1 is half way from the first noise to the next, Noise HLS's phase 180 (FX-NOISEHLS-010); frame 2 is the next noise. | largest difference 3.5e-7 | yes |
| FX-NOISEHLSAUTO-003 frame 2: Speed 0.5, Lightness 30: frame 1 is half way from the first noise to the next, Noise HLS's phase 180 (FX-NOISEHLS-010); frame 2 is the next noise. | largest difference 4.2e-7 | yes |
| FX-NOISEHLSAUTO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-004 frame 0: Hue 50, Lightness 0: each coloured pixel's hue turns up to 90 degrees either way, differently each frame; the grey has no hue and is left. | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-004 frame 3: Hue 50, Lightness 0: each coloured pixel's hue turns up to 90 degrees either way, differently each frame; the grey has no hue and is left. | largest difference 2.8e-7 | yes |
| FX-NOISEHLSAUTO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-005 frame 1: Saturation 50, Lightness 0, at frame 1. | largest difference 2.6e-7 | yes |
| FX-NOISEHLSAUTO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-006 frame 2: Squared, Lightness 30: the noise pushed out towards its ends. | largest difference 4.3e-7 | yes |
| FX-NOISEHLSAUTO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-007 frame 0: Grain, Grain Size 4, Lightness 30, speed 0.25: soft cells four pixels across, a new grain every four frames, gliding between. | largest difference 2.7e-7 | yes |
| FX-NOISEHLSAUTO-007 frame 1: Grain, Grain Size 4, Lightness 30, speed 0.25: soft cells four pixels across, a new grain every four frames, gliding between. | largest difference 2.8e-7 | yes |
| FX-NOISEHLSAUTO-007 frame 4: Grain, Grain Size 4, Lightness 30, speed 0.25: soft cells four pixels across, a new grain every four frames, gliding between. | largest difference 3.3e-7 | yes |
| FX-NOISEHLSAUTO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-008 frame 1: Grain, Grain Size 2.5, Hue 40, Saturation 40, Lightness 0, speed 2: two new grains a frame. | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-008 frame 2: Grain, Grain Size 2.5, Hue 40, Saturation 40, Lightness 0, speed 2: two new grains a frame. | largest difference 2.4e-7 | yes |
| FX-NOISEHLSAUTO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-009 frame 0: Hue, Lightness and Saturation 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-009 frame 3: Hue, Lightness and Saturation 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-010 frame 2: Hue, Lightness and Saturation 30 moved three pixels right: the noise is the drawing's own, so it moves with it. | largest difference 6.9e-7 | yes |
| FX-NOISEHLSAUTO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-011 frame 2: Hue, Lightness and Saturation 30 after a Motion Tile that grows the layer: the noise is worked in the drawing's own pixels. | largest difference 6.9e-7 | yes |
| FX-NOISEHLSAUTO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-012 frame 0: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-012 frame 2: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 3.5e-7 | yes |
| FX-NOISEHLSAUTO-012 frame 4: Lightness keyed from 0 at frame 0 to 40 at frame 4: frame 0 is the drawing. | largest difference 7.3e-7 | yes |
| FX-NOISEHLSAUTO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-013 frame 0: Speed keyed from 0 at frame 0 to 2 at frame 4, Lightness 30: the speed at the frame times the frame, so frame 2 is speed 1's frame 2 and frame 4 is depth 8. | largest difference 4.0e-7 | yes |
| FX-NOISEHLSAUTO-013 frame 2: Speed keyed from 0 at frame 0 to 2 at frame 4, Lightness 30: the speed at the frame times the frame, so frame 2 is speed 1's frame 2 and frame 4 is depth 8. | largest difference 4.2e-7 | yes |
| FX-NOISEHLSAUTO-013 frame 4: Speed keyed from 0 at frame 0 to 2 at frame 4, Lightness 30: the speed at the frame times the frame, so frame 2 is speed 1's frame 2 and frame 4 is depth 8. | largest difference 4.0e-7 | yes |
| FX-NOISEHLSAUTO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-014 frame 1: Grain, Grain Size 0.5, Lightness 30: cells of half a pixel. | largest difference 3.9e-7 | yes |
| FX-NOISEHLSAUTO-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-015 frame 0: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: the controls together. | largest difference 3.1e-7 | yes |
| FX-NOISEHLSAUTO-015 frame 1: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: the controls together. | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-015 frame 2: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: the controls together. | largest difference 3.1e-7 | yes |
| FX-NOISEHLSAUTO-015 frame 3: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: the controls together. | largest difference 3.1e-7 | yes |
| FX-NOISEHLSAUTO-015 frame 4: Grain, Grain Size 3, Hue 20, Lightness 25, Saturation 60, speed 0.75: the controls together. | largest difference 2.9e-7 | yes |
| FX-NOISEHLSAUTO-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEHLSAUTO-016 frame 0: Hue 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-016 frame 4: Hue 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-017 frame 0: Lightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-017 frame 4: Lightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-018 frame 0: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-018 frame 4: Saturation 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-019 frame 0: Grain Size 0.25, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-019 frame 4: Grain Size 0.25, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-020 frame 0: Speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-020 frame 4: Speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-021 frame 0: Speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-021 frame 4: Speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-022 frame 0: Noise "Squared", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-022 frame 4: Noise "Squared", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-023 frame 0: Noise "film", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-023 frame 4: Noise "film", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEHLSAUTO-024 frame 0: Saturation keyed to 120 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-024 frame 4: Saturation keyed to 120 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEHLSAUTO-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehlsauto_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisehlsauto_013.json is saved with its words and numbers as written, and the speed's keys kept | {"animation_speed":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":2}]},"grain_size":1,"hue":0,"lightness":30,"noise":"uniform","saturation":0} | yes |
| fx_noisehlsauto_015.json is saved with its words and numbers as written | {"animation_speed":0.75,"grain_size":3,"hue":20,"lightness":25,"noise":"grain","saturation":60} | yes |
| fx_noisehlsauto_016.json is refused in a sentence | Noise HLS Auto's hue runs from 0 to 100, and this is 101. | yes |
| fx_noisehlsauto_017.json is refused in a sentence | Noise HLS Auto's lightness runs from 0 to 100, and this is -1. | yes |
| fx_noisehlsauto_018.json is refused in a sentence | Noise HLS Auto's saturation runs from 0 to 100, and this is 150. | yes |
| fx_noisehlsauto_019.json is refused in a sentence | Noise HLS Auto's grain size runs from 0.5 to 100, and this is 0.25. | yes |
| fx_noisehlsauto_020.json is refused in a sentence | Noise HLS Auto's animation speed runs from 0 to 10, and this is 11. | yes |
| fx_noisehlsauto_021.json is refused in a sentence | Noise HLS Auto's animation speed runs from 0 to 10, and this is -1. | yes |
| fx_noisehlsauto_022.json is refused in a sentence | Noise HLS Auto's noise is one of uniform, squared, grain, and this is "Squared". | yes |
| fx_noisehlsauto_023.json is refused in a sentence | Noise HLS Auto's noise is one of uniform, squared, grain, and this is "film". | yes |
| a file with a Noise HLS Auto with no `noise` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise HLS Auto whose noise is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise HLS Auto with no `animation_speed` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue 101 is refused with a sentence, and nothing changes | Noise HLS Auto's hue runs from 0 to 100, and this is 101. | yes |
| saturation -5 is refused with a sentence, and nothing changes | Noise HLS Auto's saturation runs from 0 to 100, and this is -5. | yes |
| grain size 0.4 is refused with a sentence, and nothing changes | Noise HLS Auto's grain size runs from 0.5 to 100, and this is 0.4. | yes |
| animation speed 10.5 is refused with a sentence, and nothing changes | Noise HLS Auto's animation speed runs from 0 to 10, and this is 10.5. | yes |
| noise "Grain" is refused with a sentence, and nothing changes | Noise HLS Auto's noise is one of uniform, squared, grain, and this is "Grain". | yes |
| lightness keyed to 120 is refused with a sentence, and nothing changes | Noise HLS Auto's lightness runs from 0 to 100, and this is 120. | yes |
| Grain, hue 35, lightness 20, saturation 55, grain size 2.5, speed 0.4 is taken | taken | yes |
| lightness keyed from 0 to 40 is taken | taken | yes |
| animation speed keyed from 0 to 2 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehlsauto_001.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_003.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_007.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_013.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisehlsauto_015.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisehlsauto_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisehlsauto_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2046555 pixels changed | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 0, Full | largest difference 1 of 255, 933 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 100, Full | largest difference 1 of 255, 974 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 239, Full | largest difference 1 of 255, 810 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 100, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto as added (Uniform, lightness 10, speed 1) on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2001250 pixels changed | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 0, Full | largest difference 1 of 255, 4623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 100, Full | largest difference 1 of 255, 4934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 239, Full | largest difference 1 of 255, 3827 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 0, Draft | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 100, Draft | largest difference 1 of 255, 109 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Squared, hue 60, saturation 40, lightness 0, speed 0.3 on three layers, frame 239, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072908 pixels changed | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 0, Full | largest difference 1 of 255, 1010 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 100, Full | largest difference 1 of 255, 1085 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 239, Full | largest difference 1 of 255, 939 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 0, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1 on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2055659 pixels changed | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 0, Full | largest difference 1 of 255, 905 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 100, Full | largest difference 1 of 255, 1126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 239, Full | largest difference 1 of 255, 877 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise HLS Auto Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4 on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-452 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: the street flecked lighter and darker pixel by pixel, a lightness tenth moving a channel by no more than a fifth; draws cleanly | [], 127673 pixels changed of 129600, the largest by 51 of 255 | yes |
| 3_as_added_frame_1.png, frame 1, as added, frame 1: flecked the same way, a pattern of its own; draws cleanly | [], 127718 pixels changed of 129600, the largest by 51 of 255 | yes |
| 4_hue_60_frame_5.png, frame 5, Hue 60, Lightness 0, frame 5: the colours flecked round the wheel, each pixel keeping its lightness; draws cleanly | [], 126879 pixels changed of 129600, the largest by 110 of 255 | yes |
| 5_grain_4_saturation_60_frame_24.png, frame 24, Grain, grain size 4, Saturation 60, Lightness 20, speed 0.25, frame 24: soft blotches of stronger and weaker colour; draws cleanly | [], 129571 pixels changed of 129600, the largest by 132 of 255 | yes |
| 2_as_added.png and 3_as_added_frame_1.png differ: it moves by itself, with nothing keyed | 128149 pixels differ of 129600 | yes |
| with speed 0 the street at frame 24 is the street at frame 0: the noise holds still | 0 pixels differ | yes |

## Result

187 of 187 checks pass.
