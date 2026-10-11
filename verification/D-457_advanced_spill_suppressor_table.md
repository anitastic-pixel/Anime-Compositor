# D-457: Advanced Spill Suppressor

B-337, after After Effects' Advanced Spill Suppressor (Keying): the green or blue a screen throws onto a keyed subject taken back out. Adobe names the controls but publishes no formula, so the rule is ours, stated in `tools/advanced_spill_suppressor_reference.py`: Standard cuts green or blue, whichever leads, to the larger other channel; Ultra cuts the key's screen channel toward a blend of the other two (Spill Range), weighted by a hue window (Tolerance), gives part back (Spill Color Correction), greys by the share taken (Desaturate) and puts back lost brightness (Luma Correction). Every expected pixel is `Fixtures/advanced_spill_suppressor/expected_advanced_spill_suppressor.json`, written by the tool before this code existed and printed in document 25 as FX-SPILL-001 to 028. Tolerance 2e-5.

## FX-SPILL-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPILL-001 frame 0: The settings as they start: Standard at 100%. The green screen, the green-spilled skin and edge, pure green, the leaf, the blue screen, the blue-spilled skin, pure blue and the shirt lose their leading green or blue down to the next channel; black, white, grey, yellow and cyan are untouched. | largest difference 1.3e-7 | yes |
| FX-SPILL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-002 frame 0: Standard, suppression 0: the drawing exactly as it is. | largest difference 1.3e-7 | yes |
| FX-SPILL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-003 frame 0: Standard, suppression 50: half of each lead taken out. | largest difference 1.3e-7 | yes |
| FX-SPILL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-004 frame 0: Ultra as it starts: green key, tolerance 50, desaturate 25, spill range 50, correction 50, luma 50. The greens cut toward the mean of red and blue, half the cut given back to red and blue, a little greyer, half the lost brightness put back; yellow and cyan, 60 degrees off, a third counted; the blues, further than 90 degrees, untouched. | largest difference 1.6e-7 | yes |
| FX-SPILL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-005 frame 0: Ultra, blue key: the blue screen, blue skin, blue and the shirt corrected; the greens untouched. | largest difference 1.5e-7 | yes |
| FX-SPILL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-006 frame 0: Ultra, tolerance 0: the drawing exactly as it is. | largest difference 1.3e-7 | yes |
| FX-SPILL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-007 frame 0: Ultra, tolerance 100: the hue window reaches round to the opposite hue, so yellow-greens and blue-greens count fully nearer the key. | largest difference 1.5e-7 | yes |
| FX-SPILL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-008 frame 0: Ultra, spill range 0, nothing given back, no desaturate, no luma: green cut to the larger other channel times the hue weight. | largest difference 1.3e-7 | yes |
| FX-SPILL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-009 frame 0: Ultra, spill range 100, otherwise as FX-SPILL-008: green cut to the smaller other channel, a harder cut. | largest difference 1.3e-7 | yes |
| FX-SPILL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-010 frame 0: Ultra, spill colour correction 100, no desaturate, no luma: all that is taken from green given back to red and blue in halves (pure green ends a half magenta). | largest difference 1.3e-7 | yes |
| FX-SPILL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-011 frame 0: Ultra, desaturate 100, no correction, no luma: a spilled pixel greyed by the share of its green taken out; pure green, whose green all goes, ends black. | largest difference 1.3e-7 | yes |
| FX-SPILL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-012 frame 0: Ultra, luma correction 100, no correction, no desaturate: every changed pixel keeps its brightness (until a channel reaches full). | largest difference 1.3e-7 | yes |
| FX-SPILL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-013 frame 0: Ultra with a sampled screen green, #3cbe50: the window centred on that hue instead of pure green's. | largest difference 1.7e-7 | yes |
| FX-SPILL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-014 frame 0: Ultra with a red key: red is the screen channel, so warm colours near red's hue (yellow, the skins, the leaf) lose some red; the greens, cyan and blues are not touched. | largest difference 1.5e-7 | yes |
| FX-SPILL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-015 frame 0: Ultra with a grey key: a grey has no hue, so nothing is spill; the drawing as it is. | largest difference 1.3e-7 | yes |
| FX-SPILL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-016 frame 0: Standard, suppression keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 as 50, frame 4 as FX-SPILL-001. | largest difference 1.3e-7 | yes |
| FX-SPILL-016 frame 2: Standard, suppression keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 as 50, frame 4 as FX-SPILL-001. | largest difference 1.3e-7 | yes |
| FX-SPILL-016 frame 4: Standard, suppression keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 as 50, frame 4 as FX-SPILL-001. | largest difference 1.3e-7 | yes |
| FX-SPILL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-017 frame 0: Ultra, tolerance keyed from 20 at frame 0 to 80 at frame 4. | largest difference 1.3e-7 | yes |
| FX-SPILL-017 frame 2: Ultra, tolerance keyed from 20 at frame 0 to 80 at frame 4. | largest difference 1.6e-7 | yes |
| FX-SPILL-017 frame 4: Ultra, tolerance keyed from 20 at frame 0 to 80 at frame 4. | largest difference 1.6e-7 | yes |
| FX-SPILL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-018 frame 0: FX-SPILL-004 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SPILL-018 frame 3: FX-SPILL-004 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SPILL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPILL-019 frame 0: Suppression 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-019 frame 4: Suppression 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-020 frame 0: Suppression -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-020 frame 4: Suppression -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-021 frame 0: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-021 frame 4: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-022 frame 0: Desaturate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-022 frame 4: Desaturate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-023 frame 0: Spill range 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-023 frame 4: Spill range 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-024 frame 0: Spill colour correction 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-024 frame 4: Spill colour correction 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-025 frame 0: Luma correction -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-025 frame 4: Luma correction -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-026 frame 0: Method "Standard": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-026 frame 4: Method "Standard": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-027 frame 0: Method "auto", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-027 frame 4: Method "auto", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPILL-028 frame 0: Key colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-028 frame 4: Key colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-SPILL-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spill_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spill_013.json is saved with its eight settings, the method Ultra and the key #3cbe50 | {"desaturate":25,"key_color":"#3cbe50","luma_correction":50,"method":"ultra","spill_color_correction":50,"spill_range":50,"suppression":100,"tolerance":50} | yes |
| fx_spill_019.json is refused in a sentence naming suppression | Advanced Spill Suppressor's suppression runs from 0 to 100, and this is 101. | yes |
| fx_spill_020.json is refused in a sentence naming suppression | Advanced Spill Suppressor's suppression runs from 0 to 100, and this is -1. | yes |
| fx_spill_021.json is refused in a sentence naming tolerance | Advanced Spill Suppressor's tolerance runs from 0 to 100, and this is 101. | yes |
| fx_spill_022.json is refused in a sentence naming desaturate | Advanced Spill Suppressor's desaturate runs from 0 to 100, and this is -1. | yes |
| fx_spill_023.json is refused in a sentence naming spill range | Advanced Spill Suppressor's spill range runs from 0 to 100, and this is 101. | yes |
| fx_spill_024.json is refused in a sentence naming spill color correction | Advanced Spill Suppressor's spill color correction runs from 0 to 100, and this is 101. | yes |
| fx_spill_025.json is refused in a sentence naming luma correction | Advanced Spill Suppressor's luma correction runs from 0 to 100, and this is -1. | yes |
| fx_spill_026.json is refused in a sentence naming method | Advanced Spill Suppressor's method is "standard" or "ultra", and this is "Standard". | yes |
| fx_spill_027.json is refused in a sentence naming method | Advanced Spill Suppressor's method is "standard" or "ultra", and this is "auto". | yes |
| fx_spill_028.json is refused in a sentence naming key colour | Advanced Spill Suppressor's key colour is written #rrggbb, and this is "green". | yes |
| a file with an Advanced Spill Suppressor whose suppression is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Advanced Spill Suppressor without its method is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Advanced Spill Suppressor without its key colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| suppression 101 is refused with a sentence, and nothing changes | Advanced Spill Suppressor's suppression runs from 0 to 100, and this is 101. | yes |
| method "auto" is refused with a sentence, and nothing changes | Advanced Spill Suppressor's method is "standard" or "ultra", and this is "auto". | yes |
| key colour "green" is refused with a sentence, and nothing changes | Advanced Spill Suppressor's key colour is written #rrggbb, and this is "green". | yes |
| tolerance keyed to 150 is refused with a sentence, and nothing changes | Advanced Spill Suppressor's tolerance runs from 0 to 100, and this is 150. | yes |
| Ultra with a blue key is taken | taken | yes |
| suppression keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spill_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spill_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spill_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spill_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spill_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spill_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spill_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spill_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1713772 pixels changed | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 0, Full | largest difference 1 of 255, 4256 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 100, Full | largest difference 1 of 255, 2555 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 239, Full | largest difference 1 of 255, 2441 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 0, Draft | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Standard at 100, as added on three layers, frame 239, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 845514 pixels changed | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 0, Full | largest difference 1 of 255, 2975 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 100, Full | largest difference 1 of 255, 1588 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 239, Full | largest difference 1 of 255, 1175 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 0, Draft | largest difference 1 of 255, 135 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra as added, green key on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1302428 pixels changed | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 0, Full | largest difference 1 of 255, 3879 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 100, Full | largest difference 1 of 255, 963 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 239, Full | largest difference 1 of 255, 1929 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 0, Draft | largest difference 1 of 255, 137 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Advanced Spill Suppressor Ultra, blue key, tolerance 80, desaturate 60, spill range 30, correction 90, luma 20 on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-457 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly | [] | yes |
| suppression 0 changes nothing: the shot byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_standard.png, Standard at 100: the screen turns grey, the green edge of the face clears; pixels whose red leads (the face) untouched; no green or blue lead left anywhere; the blue shirt loses its blue too, Standard's known limit; draws cleanly | [], red-led untouched: true, no lead left: true, 4530 edge pixels cleared, shirt hit: true | yes |
| 3_ultra_green_key.png, Ultra as added (green key): the screen and the face's green edge cleaned; the blue shirt untouched; no pixel's green rises; draws cleanly | [], shirt untouched: true, no green rises: true, 100441 pixels lost green | yes |
| 4_ultra_correction.png, Ultra with spill colour correction 100, no desaturate, no luma: every pixel that loses green gains red and blue, so the cleaned screen turns a muted pink; draws cleanly | [], red and blue never fall where green fell: true | yes |
| 5_street_standard.png, the street with Standard at 100: a scene with no screen still loses every green or blue lead (the sky greys to its green); Standard is for keyed shots only; draws cleanly | [], sky's blue down to its green: true | yes |
| 6_street_ultra.png, the street with Ultra as added (green key): every pixel 91 degrees or more round the colour wheel from green (most of the sky, the red and blue walls) untouched; draws cleanly | [], 70860 such pixels, untouched: true, 20994 pixels changed | yes |
| by hand: the skin (200, 214, 160), Standard at 100, gives [200, 200, 160]; draws cleanly | [], (200, 200, 160) | yes |
| by hand: the skin (200, 214, 160), Standard at 50, gives [200, 207, 160]; draws cleanly | [], (200, 207, 160) | yes |

## Result

184 of 184 checks pass.
