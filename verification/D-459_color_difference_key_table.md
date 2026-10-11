# D-459: Color Difference Key

B-339, after After Effects' Color Difference Key (Keying): a matte from a key colour as two partial mattes, A and B, each with its own levels, screened together and taken through levels of its own as the layer's covering. Adobe describes the effect but publishes no formula, so the rule is ours, after Stefan Ihringer's reverse engineering (bildfehler.de, 2007), stated in `tools/color_difference_key_reference.py`: each pixel's hue against a window round the key (partial B) and 60 degrees short of it (partial A), weighted by saturation and laid over lightness as a hard light. Every expected pixel is `Fixtures/color_difference_key/expected_color_difference_key.json`, written by the tool before this code existed and printed in document 25 as FX-CDK-001 to 026. Tolerance 2e-5.

## FX-CDK-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CDK-001 frame 0: The settings as they start: a blue key, the final view, every levels plain. Blue and cyan (partial A's centre, 60 degrees short of blue) lose all their covering, the blue screen most of it, the violet half; grey keeps three quarters, black and white all of theirs; green, red and yellow, far from blue, keep all of it. Colour is kept, only the covering falls. | largest difference 1.2e-7 | yes |
| FX-CDK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-002 frame 0: View source: the layer exactly as it is, whatever the levels. | largest difference 1.2e-7 | yes |
| FX-CDK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-003 frame 0: View partial A uncorrected: partial A as grey, its levels not applied. | largest difference 1.2e-7 | yes |
| FX-CDK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-004 frame 0: View partial A: partial A through its levels (in 40 to 200, gamma 1.5, out 10 to 245). | largest difference 1.7e-7 | yes |
| FX-CDK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-005 frame 0: View partial B uncorrected: partial B as grey; blue near black, the colours far from blue's hue bright. | largest difference 6.7e-8 | yes |
| FX-CDK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-006 frame 0: View partial B: partial B through its levels (in 30 to 180, gamma 0.7, out 0 to 250). | largest difference 1.5e-7 | yes |
| FX-CDK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-007 frame 0: View matte uncorrected: the screen of the two uncorrected partials; every levels set and ignored. | largest difference 8.5e-8 | yes |
| FX-CDK-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-008 frame 0: View matte: the corrected partials screened, through the matte's levels (in 60 to 220, gamma 1.3). | largest difference 2.1e-7 | yes |
| FX-CDK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-009 frame 0: Final with every levels set: the covering times the corrected matte of FX-CDK-008. | largest difference 2.7e-7 | yes |
| FX-CDK-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-010 frame 0: A green key, #00ff00: green and yellow (partial A's centre) fall, the green screen most of the way; blue keeps all its covering. | largest difference 1.3e-7 | yes |
| FX-CDK-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-011 frame 0: A sampled screen green, #28b43c, partial B in 0 to 128: the screen's hue window, sharpened. | largest difference 1.7e-7 | yes |
| FX-CDK-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-012 frame 0: View matte with matte gamma 0.4: the matte darkened toward the key. | largest difference 1.4e-7 | yes |
| FX-CDK-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-013 frame 0: View matte with matte in black and in white both 150: a hard step, white where the matte is 150 or more, black below. | largest difference 3.0e-8 | yes |
| FX-CDK-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-014 frame 0: View partial A with output black 255 and output white 0: partial A turned over. | largest difference 6.7e-8 | yes |
| FX-CDK-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-015 frame 0: A grey key, #808080: a grey's hue is read as 0, so the key is red's: red and magenta fall, blue keeps its covering. | largest difference 9.5e-8 | yes |
| FX-CDK-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-016 frame 0: Final, partial B in white keyed from 255 at frame 0 to 100 at frame 4, linear. | largest difference 1.2e-7 | yes |
| FX-CDK-016 frame 2: Final, partial B in white keyed from 255 at frame 0 to 100 at frame 4, linear. | largest difference 1.3e-7 | yes |
| FX-CDK-016 frame 4: Final, partial B in white keyed from 255 at frame 0 to 100 at frame 4, linear. | largest difference 2.2e-7 | yes |
| FX-CDK-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-017 frame 0: View matte, matte gamma keyed from 1 at frame 0 to 3 at frame 4. | largest difference 8.5e-8 | yes |
| FX-CDK-017 frame 2: View matte, matte gamma keyed from 1 at frame 0 to 3 at frame 4. | largest difference 8.0e-8 | yes |
| FX-CDK-017 frame 4: View matte, matte gamma keyed from 1 at frame 0 to 3 at frame 4. | largest difference 3.7e-7 | yes |
| FX-CDK-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-018 frame 0: FX-CDK-009 moved three pixels right: the same, moved. | largest difference 2.7e-7 | yes |
| FX-CDK-018 frame 3: FX-CDK-009 moved three pixels right: the same, moved. | largest difference 2.7e-7 | yes |
| FX-CDK-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CDK-019 frame 0: Partial A in black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-019 frame 4: Partial A in black 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-020 frame 0: Partial B out white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-020 frame 4: Partial B out white -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-021 frame 0: Partial A gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-021 frame 4: Partial A gamma 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-022 frame 0: Matte gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-022 frame 4: Matte gamma 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-023 frame 0: Matte in white 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-023 frame 4: Matte in white 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-024 frame 0: View "Final": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-024 frame 4: View "Final": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-025 frame 0: View "all": After Effects' four-up view is not one here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-025 frame 4: View "all": After Effects' four-up view is not one here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CDK-026 frame 0: Key colour "blue", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-026 frame 4: Key colour "blue", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CDK-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cdk_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cdk_011.json is saved with its fifteen settings, the key #28b43c and partial B in white 128 | {"key_color":"#28b43c","matte_gamma":1,"matte_in_black":0,"matte_in_white":255,"partial_a_gamma":1,"partial_a_in_black":0,"partial_a_in_white":255,"partial_a_out_black":0,"partial_a_out_white":255,"partial_b_gamma":1,"partial_b_in_black":0,"partial_b_in_white":128,"partial_b_out_black":0,"partial_b_out_white":255,"view":"final"} | yes |
| fx_cdk_019.json is refused in a sentence naming partial a in black | Color Difference Key's partial a in black runs from 0 to 255, and this is 256. | yes |
| fx_cdk_020.json is refused in a sentence naming partial b out white | Color Difference Key's partial b out white runs from 0 to 255, and this is -1. | yes |
| fx_cdk_021.json is refused in a sentence naming partial a gamma | Color Difference Key's partial a gamma runs from 0.1 to 10, and this is 11. | yes |
| fx_cdk_022.json is refused in a sentence naming matte gamma | Color Difference Key's matte gamma runs from 0.1 to 10, and this is 0.05. | yes |
| fx_cdk_023.json is refused in a sentence naming matte in white | Color Difference Key's matte in white runs from 0 to 255, and this is 256. | yes |
| fx_cdk_024.json is refused in a sentence naming view | Color Difference Key's view is one of "final", "source", "partial_a_uncorrected", "partial_a", "partial_b_uncorrected", "partial_b", "matte_uncorrected", "matte", and this is "Final". | yes |
| fx_cdk_025.json is refused in a sentence naming view | Color Difference Key's view is one of "final", "source", "partial_a_uncorrected", "partial_a", "partial_b_uncorrected", "partial_b", "matte_uncorrected", "matte", and this is "all". | yes |
| fx_cdk_026.json is refused in a sentence naming key colour | Color Difference Key's key colour is written #rrggbb, and this is "blue". | yes |
| a file with a Color Difference Key whose matte gamma is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Difference Key without its view is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Difference Key without its matte gamma is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| partial A gamma 11 is refused with a sentence, and nothing changes | Color Difference Key's partial a gamma runs from 0.1 to 10, and this is 11. | yes |
| view "all" is refused with a sentence, and nothing changes | Color Difference Key's view is one of "final", "source", "partial_a_uncorrected", "partial_a", "partial_b_uncorrected", "partial_b", "matte_uncorrected", "matte", and this is "all". | yes |
| key colour "blue" is refused with a sentence, and nothing changes | Color Difference Key's key colour is written #rrggbb, and this is "blue". | yes |
| matte in white keyed to 300 is refused with a sentence, and nothing changes | Color Difference Key's matte in white runs from 0 to 255, and this is 300. | yes |
| view matte with a green key is taken | taken | yes |
| matte gamma keyed from 1 to 3 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cdk_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cdk_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cdk_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cdk_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cdk_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cdk_018.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cdk_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cdk_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Difference Key as added (blue key, final): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2035200 pixels changed | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 0, Full | largest difference 1 of 255, 2784 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 100, Full | largest difference 1 of 255, 1201 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 239, Full | largest difference 1 of 255, 1882 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 0, Draft | largest difference 1 of 255, 633 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 100, Draft | largest difference 1 of 255, 559 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key as added (blue key, final) on three layers, frame 239, Draft | largest difference 1 of 255, 614 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073554 pixels changed | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 0, Full | largest difference 1 of 255, 1374 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 100, Full | largest difference 1 of 255, 1634 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 239, Full | largest difference 1 of 255, 1416 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 0, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key view matte, green key, matte gamma 0.6 on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 463372 pixels changed | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 0, Full | largest difference 1 of 255, 3196 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 100, Full | largest difference 1 of 255, 1305 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 239, Full | largest difference 1 of 255, 1334 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 0, Draft | largest difference 1 of 255, 213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 100, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Difference Key final, key #28b43c, every levels set on three layers, frame 239, Draft | largest difference 1 of 255, 137 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-459 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly | [] | yes |
| view source changes nothing, whatever the levels: the shot byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_matte_uncorrected.png, the matte as grey, key #28b43c (the screen's green), no levels: the screen darkest (103 by hand), the face lighter (182), the shirt lightest (226); the screen is not yet black, which is what the levels are for; draws cleanly | [], screen 103, face 182, shirt 226 | yes |
| 3_matte.png, the corrected matte with matte in black 110 and in white 180: the screen black, the face and the shirt white, the face's spilled green edge grey; draws cleanly | [], screen 0, face 255, edge 131, shirt 255 | yes |
| 4_final.png, Final Output with the same levels: the screen see-through, the face and the shirt exactly as they were, the spilled edge part covered; wherever anything is left its colour is the shot's (the key takes covering, never colour, so the green spill stays for a spill suppressor); draws cleanly | [], screen covering 0, face and shirt untouched: true, colour kept: true, edge covering 131 | yes |
| by hand: pure blue, as added, keeps covering 0 of 255; draws cleanly | [], covering 0 | yes |
| by hand: the violet (128, 64, 255), as added, keeps covering 128 of 255; draws cleanly | [], covering 128 | yes |
| by hand: pure green, as added, keeps covering 255 of 255; draws cleanly | [], covering 255 | yes |

## Result

171 of 171 checks pass.
