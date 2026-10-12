# D-584: Contour Paint, Match to Nearest Custom Color

B-462: `core.contour_paint`'s Reduction Method takes After Effects' Match to Nearest Custom Color as `match_to_nearest_custom_color`, with D-87's chosen colours (`colors`, up to eight `#rrggbb`) as the Custom Color Set, each pixel filled with the custom colour nearest it in CIE L*a*b* (CIE76). Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by `tools/contour_paint_reference.py` before this code existed and printed in document 25 as FX-CONTOUR-033 to 041. Tolerance 2e-5.

## FX-CONTOUR-033 to 041 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CONTOUR-033 frame 0: Match to Nearest Custom Color with four custom colours, a brick red, a blue, a mid grey and a cream, Over Smeared Image: every pixel one of the four, each the nearest as the eye sees it, which for some pixels is not the nearest by red, green and blue. | largest difference 2.2e-8 | yes |
| FX-CONTOUR-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-034 frame 0: The same, Fills Only with Low Strength 38: the four colours only where the flow is at least 38 per cent strong, clear elsewhere. | largest difference 2.2e-8 | yes |
| FX-CONTOUR-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-035 frame 0: Match to Nearest Custom Color with no custom colours: nothing is filled, the smear alone, FX-CONTOUR-001. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-036 frame 0: One custom colour, the blue: every pixel blue. | largest difference 1.9e-8 | yes |
| FX-CONTOUR-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-037 frame 0: The two blocks with clear round them, pure red and pure blue as the custom colours: the red block red, the blue block blue, every covering kept, every clear pixel clear. | largest difference 0.0e0 | yes |
| FX-CONTOUR-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-038 frame 0: The four custom colours kept but K-Means chosen: they are not used, FX-CONTOUR-010. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-039 frame 0: A custom colour written "#c0303", five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-039 frame 4: A custom colour written "#c0303", five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-040 frame 0: Nine custom colours, past eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-040 frame 4: Nine custom colours, past eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CONTOUR-041 frame 0: A custom colour written "red", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-041 frame 4: A custom colour written "red", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_033.json is saved with its fourteen settings, the four custom colours among them, and no palette | {"apply_to_alpha":"off","cleanliness":50,"colors":["#c03030","#3050c0","#808080","#f0f0c0"],"draw_color_fills":"over_smeared_image","edge_map_layer":"","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":8,"quantize_flow_angles":0,"reduction_method":"match_to_nearest_custom_color","smear_iterations":3,"stylization":50,"view":"final_result"} | yes |
| fx_contour_010.json, from before the set, opens with none and saves without it | {"apply_to_alpha":"off","cleanliness":50,"draw_color_fills":"over_smeared_image","edge_map_layer":"","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":4,"quantize_flow_angles":0,"reduction_method":"k_means","smear_iterations":3,"stylization":50,"view":"final_result"} | yes |
| fx_contour_039.json is refused in a sentence naming "#c0303" | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#c0303". | yes |
| fx_contour_040.json is refused in a sentence naming up to eight colours | Contour Paint takes up to eight colours, and this has 9. | yes |
| fx_contour_041.json is refused in a sentence naming "red" | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "red". | yes |
| a file with a Contour Paint whose custom colours are one word, not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| nine custom colours is refused with a sentence, and nothing changes | Contour Paint takes up to eight colours, and this has 9. | yes |
| a custom colour "#12" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12". | yes |
| Match to Nearest Custom Color with white and black is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_034.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_037.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint fills Over Smeared Image matched to five custom colours on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Fills Only, Low Strength 10, matched to two custom colours on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-584 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_k_means_5_colours.png, fills by K-Means in 5 colours, for comparison: colours the program picks from the street; draws cleanly | [], 5 colours | yes |
| 3_custom_5_colours.png, Match to Nearest Custom Color with five chosen colours (a night navy, a blue, a skin tone, a red, an off-white): every pixel one of those five; draws cleanly | [], colours used 5 | yes |
| 4_custom_2_colours.png, only a dark blue and a cream: the street in two tones, dark where it is dark and cream where it is light; draws cleanly | [], colours {[32, 32, 64], [240, 224, 192]} | yes |
| 5_custom_no_colours.png, Match to Nearest Custom Color with no colours chosen: nothing is filled, the smear alone, the same as the effect without fills; draws cleanly | [], the same as the smear alone true | yes |

## Result

68 of 68 checks pass.
