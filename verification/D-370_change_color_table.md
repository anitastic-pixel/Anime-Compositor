# D-370: Change Color

B-249, after After Effects' Change Color: pixels near one colour, by RGB, hue or chroma, within a tolerance and a soft edge, turned round the colour wheel and made lighter or darker, stronger or weaker; the mask it made can be shown or turned over. Every expected pixel is `Fixtures/change_color/expected_change_color.json`, written by `tools/change_color_reference.py` before this code existed and printed in document 25 as FX-CHCOLOR-001 to 024. Tolerance 2e-5.

## FX-CHCOLOR-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHCOLOR-001 frame 0: The settings as they start: every transform 0, so the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-002 frame 0: Hue transform 120, matching red by hue, tolerance 15: the red column turns green at every darkness, and the skin column, whose hue is within 15 per cent of red's, turns too; everything else stays. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-003 frame 0: The same with softness 8: the orange and the warm shadow and midtone, just past the tolerance, turn part of the way; the warm highlight, further, stays. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-004 frame 0: FX-CHCOLOR-003's settings, view mask: the mask as greys, white where the colour is changed fully, black where it is left, grey in between. | largest difference 1.5e-6 | yes |
| FX-CHCOLOR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-005 frame 0: FX-CHCOLOR-003's settings, invert on: everything but the reds and skin turns round the wheel; the greys, having no hue, stay grey. | largest difference 2.8e-7 | yes |
| FX-CHCOLOR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-006 frame 0: FX-CHCOLOR-004 with invert on: the mask turned round. | largest difference 1.3e-6 | yes |
| FX-CHCOLOR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-007 frame 0: Lightness -50: the reds and skin half way to black. | largest difference 4.2e-7 | yes |
| FX-CHCOLOR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-008 frame 0: Lightness 50: the reds and skin half way to white. | largest difference 1.2e-7 | yes |
| FX-CHCOLOR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-009 frame 0: Saturation -100: the reds and skin turn grey at their own lightness. | largest difference 1.3e-7 | yes |
| FX-CHCOLOR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-010 frame 0: Saturation 60, softness 8: the reds stay pure, the skin and the warm tones grow stronger. | largest difference 5.4e-7 | yes |
| FX-CHCOLOR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-011 frame 0: Matching by RGB, colour #f6d6be (the skin), tolerance 20, softness 10, hue 180 and lightness -30: the skin and the colours close to it in all three channels, the warm highlight and white among them, are changed; red, far in RGB, is not. | largest difference 3.7e-7 | yes |
| FX-CHCOLOR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-012 frame 0: Matching by chroma, colour #00ff00, tolerance 30, softness 15, hue 180: the bright greens are changed, the darker greens, whose colour signal is weaker, less, and cyan, near in hue but far in chroma, not at all. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-013 frame 0: Hue transform keyed from 0 at frame 0 to 240 at frame 4, linear, softness 8: frame 0 untouched, frame 2 FX-CHCOLOR-003, frame 4 the reds turned blue. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-013 frame 2: Hue transform keyed from 0 at frame 0 to 240 at frame 4, linear, softness 8: frame 0 untouched, frame 2 FX-CHCOLOR-003, frame 4 the reds turned blue. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-013 frame 4: Hue transform keyed from 0 at frame 0 to 240 at frame 4, linear, softness 8: frame 0 untouched, frame 2 FX-CHCOLOR-003, frame 4 the reds turned blue. | largest difference 2.8e-7 | yes |
| FX-CHCOLOR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-014 frame 0: FX-CHCOLOR-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-014 frame 3: FX-CHCOLOR-003 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHCOLOR-015 frame 0: Hue transform 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-015 frame 4: Hue transform 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-016 frame 0: Lightness transform 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-016 frame 4: Lightness transform 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-017 frame 0: Saturation transform -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-017 frame 4: Saturation transform -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-018 frame 0: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-018 frame 4: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-019 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-019 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-020 frame 0: Color to change "#ff00", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-020 frame 4: Color to change "#ff00", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-021 frame 0: Match colors "lab", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-021 frame 4: Match colors "lab", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-022 frame 0: Match colors "Hue": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-022 frame 4: Match colors "Hue": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-023 frame 0: View "matte", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-023 frame 4: View "matte", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHCOLOR-024 frame 0: Invert mask "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-024 frame 4: Invert mask "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHCOLOR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chcolor_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_chcolor_011.json is saved with its view, transforms, colour, tolerance, softness, match and invert | {"color_to_change":"#f6d6be","hue_transform":180,"invert_mask":"off","lightness_transform":-30,"match_colors":"rgb","saturation_transform":0,"softness":10,"tolerance":20,"view":"corrected"} | yes |
| fx_chcolor_020.json is refused in a sentence | Change Color's colour to change is written #rrggbb, and this is "#ff00". | yes |
| fx_chcolor_021.json is refused in a sentence | Change Color's match colors is "rgb", "hue" or "chroma", and this is "lab". | yes |
| fx_chcolor_022.json is refused in a sentence | Change Color's match colors is "rgb", "hue" or "chroma", and this is "Hue". | yes |
| fx_chcolor_023.json is refused in a sentence | Change Color's view is "corrected" or "mask", and this is "matte". | yes |
| fx_chcolor_024.json is refused in a sentence | Change Color's invert mask is "off" or "on", and this is "yes". | yes |
| a file with a Change Color with no `color_to_change` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Change Color whose tolerance is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue transform 3600.5 is refused with a sentence, and nothing changes | Change Color's hue transform runs from -3600 to 3600, and this is 3600.5. | yes |
| lightness transform -100.5 is refused with a sentence, and nothing changes | Change Color's lightness transform runs from -100 to 100, and this is -100.5. | yes |
| softness 100.5 is refused with a sentence, and nothing changes | Change Color's softness runs from 0 to 100, and this is 100.5. | yes |
| match colors "luma" is refused with a sentence, and nothing changes | Change Color's match colors is "rgb", "hue" or "chroma", and this is "luma". | yes |
| tolerance keyed to 120 is refused with a sentence, and nothing changes | Change Color's tolerance runs from 0 to 100, and this is 120. | yes |
| the mask view, green by chroma at 30 and 15, inverted, is taken | taken | yes |
| hue transform keyed from 0 to 240 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chcolor_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chcolor_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chcolor_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chcolor_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chcolor_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chcolor_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_chcolor_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_chcolor_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_chcolor_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Change Color red by hue turned 120, softness 8: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 81312 pixels changed | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 0, Full | largest difference 1 of 255, 3796 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 100, Full | largest difference 1 of 255, 1421 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 0, Draft | largest difference 1 of 255, 130 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color red by hue turned 120, softness 8 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 0, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 239, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color the mask view, inverted, softness 8 on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 542989 pixels changed | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 0, Full | largest difference 1 of 255, 4779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 100, Full | largest difference 1 of 255, 2522 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 239, Full | largest difference 1 of 255, 2611 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 0, Draft | largest difference 1 of 255, 152 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color skin by RGB at 20 and 10, turned 180 and darker on three layers, frame 239, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 319447 pixels changed | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 0, Full | largest difference 1 of 255, 3924 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 100, Full | largest difference 1 of 255, 1629 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 239, Full | largest difference 1 of 255, 1993 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Change Color green by chroma at 30 and 15, stronger by 60 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-370 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (red by hue at 15, every transform 0): nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_red_walls_to_green.png, red by hue turned 120: the red-brown houses turn green, the lit windows and blue houses stay as they were; draws cleanly | [], a red wall [70, 180, 90] from [180, 90, 70], a window [255, 240, 170], a blue wall [90, 140, 170] | yes |
| 4_mask_view.png, the mask view: white where red was picked, black elsewhere; draws cleanly | [], a red wall [255, 255, 255], a blue wall [0, 0, 0], every pixel grey: true | yes |
| 5_only_red_kept.png, the mask turned over and saturation -100: everything goes grey but the red-brown houses; draws cleanly | [], a red wall [180, 90, 70], a blue wall [130, 130, 130] from [90, 140, 170] | yes |

## Result

163 of 163 checks pass.
