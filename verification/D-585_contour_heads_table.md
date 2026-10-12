# D-585: Contour Paint, Arrowheads

B-463: `core.contour_paint`'s Edge Map view draws a head at the end of each flow line, as After Effects' Edge Map draws arrows: two strokes a fifth of the cell long, back along the line turned 30 degrees each way. `arrowheads` is `on` or `off`, `on` when added, `off` for a file without it (so a file from before D-585 draws as it did, D-304's way), written only when `on`. Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by `tools/contour_paint_reference.py` before this code existed and printed in document 25 as FX-CONTOUR-042 to 047. Tolerance 2e-5.

## FX-CONTOUR-042 to 047 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CONTOUR-042 frame 0: View Edge Map with Arrowheads on, as added: FX-CONTOUR-014's two cyan lines, each with a head of two short strokes at its end along the flow. | largest difference 2.4e-7 | yes |
| FX-CONTOUR-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-043 frame 0: The same with Arrowheads off, written: FX-CONTOUR-014 exactly, as a file without the setting. | largest difference 2.4e-7 | yes |
| FX-CONTOUR-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-044 frame 0: Arrowheads on, Offset Flow Angles 180: the same lines, the heads at their other ends. | largest difference 2.4e-7 | yes |
| FX-CONTOUR-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-045 frame 0: Arrowheads on with the ramp as the External Edge Map: FX-CONTOUR-018's line straight down the first cell, its head where the line meets the drawing's lower edge. | largest difference 2.6e-8 | yes |
| FX-CONTOUR-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-046 frame 0: Arrowheads on with View Final Result: the heads belong to the Edge Map, so FX-CONTOUR-001. | largest difference 1.7e-7 | yes |
| FX-CONTOUR-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-047 frame 0: Arrowheads written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-047 frame 4: Arrowheads written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CONTOUR-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The cases from before D-585 still pass

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CONTOUR-014 frame 0: View Edge Map: the flow's strength in grey, two cyan lines along the flow in the two cells. | largest difference 2.4e-7 | yes |
| FX-CONTOUR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CONTOUR-018 frame 0: Edge Map with the ramp as the External Edge Map: the grey the same down each column, and one cyan line straight down the first cell (the second cell's middle lies past the drawing's right edge, and its line with it). | largest difference 1.4e-8 | yes |
| FX-CONTOUR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_contour_042.json is saved with its fourteen settings, Arrowheads on among them | {"apply_to_alpha":"off","arrowheads":"on","cleanliness":50,"draw_color_fills":"none","edge_map_layer":"","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":8,"quantize_flow_angles":0,"reduction_method":"k_means","smear_iterations":3,"stylization":50,"view":"edge_map"} | yes |
| fx_contour_043.json, Arrowheads written off, is saved with it as written, so the file holds what it held | {"apply_to_alpha":"off","arrowheads":"off","cleanliness":50,"draw_color_fills":"none","edge_map_layer":"","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":8,"quantize_flow_angles":0,"reduction_method":"k_means","smear_iterations":3,"stylization":50,"view":"edge_map"} | yes |
| fx_contour_014.json, from before the heads, opens with Arrowheads off and saves without them | opens with "off"; saved {"apply_to_alpha":"off","cleanliness":50,"draw_color_fills":"none","edge_map_layer":"","high_strength":100,"low_strength":0,"offset_flow_angles":0,"posterization_level":8,"quantize_flow_angles":0,"reduction_method":"k_means","smear_iterations":3,"stylization":50,"view":"edge_map"} | yes |
| fx_contour_047.json is refused in a sentence naming "yes" | Contour Paint's arrowheads is "off" or "on", and this is "yes". | yes |
| a file with a Contour Paint whose arrowheads is a number, not a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Arrowheads "yes" is refused with a sentence, and nothing changes | Contour Paint's arrowheads is "off" or "on", and this is "yes". | yes |
| Arrowheads on is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_042.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_044.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_contour_045.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_contour_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_contour_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on: the processor's frame 100 differs from the lines without heads, so the comparisons below test the heads | 1769 pixels changed | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80: the processor's frame 100 differs from the lines without heads, so the comparisons below test the heads | 9682 pixels changed | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 239, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Contour Paint Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-585 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_edge_map_no_heads.png, the street's Edge Map with Arrowheads off, as a file from before them: grey strength with short cyan lines; draws cleanly | [], 8343 cyan pixels | yes |
| 2_edge_map_arrowheads.png, the same with Arrowheads on: each line now ends in a small V-shaped head, so there is more cyan and the flow's direction can be read; draws cleanly | [], 12302 cyan pixels | yes |
| 3_arrowheads_offset_180.png, Arrowheads on and Offset Flow Angles 180: the same lines, every head at the other end; draws cleanly | [], differs from 2: true | yes |
| 4_final_result_unchanged.png, Arrowheads on with View Final Result: the heads belong to the Edge Map, so the painted street is the same as with them off; draws cleanly | [], the same as with them off: true | yes |

## Result

61 of 61 checks pass.
