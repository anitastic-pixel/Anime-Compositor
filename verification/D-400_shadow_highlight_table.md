# D-400: Shadow/Highlight

B-279, after After Effects' Shadow/Highlight: the shadows lifted and the highlights brought down by each pixel's surroundings, by darktable's shadows and highlights rule (src/iop/shadhi.c, Gaussian softening): the lightness blurred and inverted, laid on each pixel's own lightness by an overlay, gated by the tonal width, the colour scaled to follow. Every expected pixel is `Fixtures/shadow_highlight/expected_shadow_highlight.json`, written by `tools/shadow_highlight_reference.py` before this code existed and printed in document 25 as FX-SHHI-001 to 021. Tolerance 2e-5.

## FX-SHHI-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHHI-001 frame 0: The settings as they start (shadow 50, highlight 0, widths 50, radii 30, colour correction 20): the dark rows lifted, most where the blur round them is darkest; black and white stay as they are. | largest difference 2.1e-7 | yes |
| FX-SHHI-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-002 frame 0: Both amounts 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-SHHI-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-003 frame 0: Shadow 100: the overlay laid four times (2 squared), lifted further than FX-SHHI-001. | largest difference 4.3e-7 | yes |
| FX-SHHI-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-004 frame 0: Highlight 50 alone at radius 2, shadow 0: the light pixels darkened where the blur round them is light; white and black stay. | largest difference 2.1e-7 | yes |
| FX-SHHI-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-005 frame 0: Highlight 100 alone at radius 2: darkened further. | largest difference 2.7e-7 | yes |
| FX-SHHI-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-006 frame 0: Shadow 60 and highlight 40 together: the highlights first, then the shadows. | largest difference 2.4e-7 | yes |
| FX-SHHI-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-007 frame 0: Shadow tonal width 30: only pixels whose surroundings are darker change, less than FX-SHHI-001. | largest difference 2.1e-7 | yes |
| FX-SHHI-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-008 frame 0: Shadow tonal width 100: every pixel's surroundings count as shadow, more than FX-SHHI-001. | largest difference 2.1e-7 | yes |
| FX-SHHI-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-009 frame 0: Shadow radius 1: each pixel judged by its nearest neighbours, so the lift follows the columns' own brightness. | largest difference 1.9e-7 | yes |
| FX-SHHI-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-010 frame 0: Shadow radius 0: each pixel judged by its own lightness alone. | largest difference 1.9e-7 | yes |
| FX-SHHI-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-011 frame 0: Shadow 70 at radius 2 and highlight 60 at radius 6: two blurs, one for each. | largest difference 1.6e-7 | yes |
| FX-SHHI-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-012 frame 0: Colour correction 0: the lifted shadows lose colour (their chroma scaled by how far they are from white). | largest difference 2.1e-7 | yes |
| FX-SHHI-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-013 frame 0: Colour correction 100: the lifted shadows keep their colour's strength, chroma scaled with the lightness. | largest difference 2.1e-7 | yes |
| FX-SHHI-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-014 frame 0: Highlight tonal width 100, highlight 80, radius 3, shadow 0: the highlights' widest reach. | largest difference 3.7e-7 | yes |
| FX-SHHI-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-015 frame 0: Shadow amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 shadow 50 (FX-SHHI-001), frame 4 as FX-SHHI-003. | largest difference 1.9e-7 | yes |
| FX-SHHI-015 frame 2: Shadow amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 shadow 50 (FX-SHHI-001), frame 4 as FX-SHHI-003. | largest difference 2.1e-7 | yes |
| FX-SHHI-015 frame 4: Shadow amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 untouched, frame 2 shadow 50 (FX-SHHI-001), frame 4 as FX-SHHI-003. | largest difference 4.3e-7 | yes |
| FX-SHHI-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-016 frame 0: FX-SHHI-011 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SHHI-016 frame 3: FX-SHHI-011 moved three pixels right: the same, moved. | largest difference 1.6e-7 | yes |
| FX-SHHI-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHHI-017 frame 0: Shadow amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-017 frame 4: Shadow amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHHI-018 frame 0: Highlight amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-018 frame 4: Highlight amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHHI-019 frame 0: Shadow tonal width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-019 frame 4: Shadow tonal width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHHI-020 frame 0: Highlight radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-020 frame 4: Highlight radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHHI-021 frame 0: Colour correction -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-021 frame 4: Colour correction -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHHI-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shhi_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shhi_011.json is saved with its seven settings | {"color_correction":20,"highlight_amount":60,"highlight_radius":6,"highlight_tonal_width":50,"shadow_amount":70,"shadow_radius":2,"shadow_tonal_width":50} | yes |
| fx_shhi_017.json is refused in a sentence naming its shadow amount | Shadow/Highlight's shadow amount runs from 0 to 100, and this is 101. | yes |
| fx_shhi_018.json is refused in a sentence naming its highlight amount | Shadow/Highlight's highlight amount runs from 0 to 100, and this is -1. | yes |
| fx_shhi_019.json is refused in a sentence naming its shadow tonal width | Shadow/Highlight's shadow tonal width runs from 0 to 100, and this is 101. | yes |
| fx_shhi_020.json is refused in a sentence naming its highlight radius | Shadow/Highlight's highlight radius runs from 0 to 500, and this is 501. | yes |
| fx_shhi_021.json is refused in a sentence naming its color correction | Shadow/Highlight's color correction runs from 0 to 100, and this is -1. | yes |
| a file with a Shadow/Highlight whose shadow amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shadow/Highlight without its shadow amount is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shadow amount 101 is refused with a sentence, and nothing changes | Shadow/Highlight's shadow amount runs from 0 to 100, and this is 101. | yes |
| highlight radius 501 is refused with a sentence, and nothing changes | Shadow/Highlight's highlight radius runs from 0 to 500, and this is 501. | yes |
| shadow amount keyed to 150 is refused with a sentence, and nothing changes | Shadow/Highlight's shadow amount runs from 0 to 100, and this is 150. | yes |
| highlight 70 at radius 4 is taken | taken | yes |
| shadow amount keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shhi_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shhi_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shhi_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shhi_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shhi_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shhi_016.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shhi_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shhi_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Shadow/Highlight as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 393693 pixels changed | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 0, Full | largest difference 1 of 255, 1623 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 100, Full | largest difference 1 of 255, 1548 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 239, Full | largest difference 1 of 255, 1551 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1909133 pixels changed | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 0, Full | largest difference 1 of 255, 1220 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 100, Full | largest difference 1 of 255, 1124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 239, Full | largest difference 1 of 255, 1133 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 80, highlight 60, radii 12 and 40 on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2067300 pixels changed | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 0, Full | largest difference 1 of 255, 1011 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 100, Full | largest difference 1 of 255, 1057 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 239, Full | largest difference 1 of 255, 989 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 100, widths 100, colour correction 100 on three layers, frame 239, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-400 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| both amounts 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (shadow 50, radius 30): the dark doorways and shade lifted; no pixel more than a level darker; draws cleanly | [], 24997 pixels lifted, 0 darker | yes |
| 3_shadow_100.png, shadow 100: lifted further; draws cleanly | [], 31851 pixels changed | yes |
| 4_highlight_80.png, shadow 0, highlight 80 at radius 4: the sky and lit walls brought down; no pixel more than a level brighter; draws cleanly | [], 93096 pixels lowered, 0 brighter | yes |
| 5_both.png, shadow 70 and highlight 60 at radius 10: the street's range drawn in from both ends; draws cleanly | [], 125481 pixels changed | yes |
| 6_colour_0.png and 7_colour_100.png, shadow 100 with colour correction 0 and 100: the lifted shadows paler at 0, more coloured at 100; draw cleanly | [] [], total saturation 7916477 and 7966887 | yes |

## Result

140 of 140 checks pass.
