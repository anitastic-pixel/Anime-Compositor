# D-456: Deband

B-336, this program's own Deband on the control list of CelFX's Deband (P_Deband, whose page gives the names only). Each pixel's edge weight is its largest step to its four neighbours, in levels, against Edge Threshold and Edge Softness. Inside the flat areas it is blurred across then down by a normalised Gaussian that edges lend nothing to and that stops at them (after Gastal and Oliveira's edge-stopping separable filters, 2011); then each pixel reads its colour from up to Diffusion Amount away (D-119's hash), and triangular dither of Noise Amount levels is added (Lipshitz, Wannamaker and Vanderkooy, 1992), each reseeded every N frames by P0-23's held step and each pulled back on edges by its reduction. Edge Only shows the edge weight as grey; Selected Edge lays it over the result in red. The rule, ranges and starting values are ours. Every expected pixel is `Fixtures/deband/expected_deband.json`, written by `tools/deband_reference.py` before this code existed and printed in document 25 as FX-DEBAND-001 to 024. Tolerance 2e-5.

## FX-DEBAND-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DEBAND-001 frame 0: The settings as added (threshold 4, softness 2, blur 8, diffusion 1, noise 1, every reduction 100, reseeding every frame) on `banded`: the bands smoothed, a little dither, the edge to the dark block kept; frame 1 has fresh dither. | largest difference 5.9e-8 | yes |
| FX-DEBAND-001 frame 1: The settings as added (threshold 4, softness 2, blur 8, diffusion 1, noise 1, every reduction 100, reseeding every frame) on `banded`: the bands smoothed, a little dither, the edge to the dark block kept; frame 1 has fresh dither. | largest difference 6.0e-8 | yes |
| FX-DEBAND-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-002 frame 0: Blur, diffusion and noise all 0: the drawing as it is. | largest difference 5.7e-8 | yes |
| FX-DEBAND-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-003 frame 0: Blur 8 alone: the bands smoothed, the edge pixels exactly as drawn. | largest difference 5.7e-8 | yes |
| FX-DEBAND-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-004 frame 0: Blur 8 with Edge Blur Reduction 0: the edge pixels take the flat ground's blur too. | largest difference 5.8e-8 | yes |
| FX-DEBAND-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-005 frame 0: Threshold 0, softness 0, blur 8: every pixel with any step is an edge, and a flat pixel meets only its own colour before an edge stops the blur: the drawing as it is. | largest difference 5.7e-8 | yes |
| FX-DEBAND-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-006 frame 0: Threshold 255, blur 8: nothing is an edge, the whole layer blurred, the dark block's edge too. | largest difference 5.7e-8 | yes |
| FX-DEBAND-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-007 frame 0: Threshold 2, softness 2, blur 8: the band steps of 3 levels are half edges, half held back. | largest difference 6.0e-8 | yes |
| FX-DEBAND-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-008 frame 0: Diffusion 3 alone: pixels read from up to 3 away inside the flat ground; the edge pixels kept. | largest difference 5.7e-8 | yes |
| FX-DEBAND-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-009 frame 0: Diffusion 3, Edge Diffusion Reduction 0: the edge pixels read from flat ground too. | largest difference 5.7e-8 | yes |
| FX-DEBAND-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-010 frame 0: Noise 4 alone: dither on the flat ground, none on the edge pixels. | largest difference 7.1e-8 | yes |
| FX-DEBAND-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-011 frame 0: Noise 4, Edge Noise Reduction 0: dither everywhere. | largest difference 7.1e-8 | yes |
| FX-DEBAND-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-012 frame 0: Both reseed intervals 0: the dither and diffusion never change, frame 3 as frame 0. | largest difference 5.9e-8 | yes |
| FX-DEBAND-012 frame 3: Both reseed intervals 0: the dither and diffusion never change, frame 3 as frame 0. | largest difference 5.9e-8 | yes |
| FX-DEBAND-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-013 frame 0: Both reseed intervals 2: frames 0 and 1 the same, frame 2 changed. | largest difference 5.9e-8 | yes |
| FX-DEBAND-013 frame 1: Both reseed intervals 2: frames 0 and 1 the same, frame 2 changed. | largest difference 5.9e-8 | yes |
| FX-DEBAND-013 frame 2: Both reseed intervals 2: frames 0 and 1 the same, frame 2 changed. | largest difference 6.0e-8 | yes |
| FX-DEBAND-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-014 frame 0: View Edge Only, threshold 2: the edge weight as grey, white on the edge to the block, mid grey on the 3-level band steps, black on the flat ground. | largest difference 9.8e-7 | yes |
| FX-DEBAND-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-015 frame 0: View Selected Edge: FX-DEBAND-001 with the edges laid over it in red. | largest difference 5.9e-8 | yes |
| FX-DEBAND-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-016 frame 0: `holes`, with one empty pixel and a half-covered column: the empty pixel stays empty and the column keeps its covering. | largest difference 5.8e-8 | yes |
| FX-DEBAND-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-017 frame 0: `flat`, one colour, as added: no edges, the blur changes nothing, only the dither shows. | largest difference 7.9e-8 | yes |
| FX-DEBAND-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-018 frame 0: Blur keyed from 0 at frame 0 to 16 at frame 4, nothing else: frame 0 the drawing, then smoother. | largest difference 5.7e-8 | yes |
| FX-DEBAND-018 frame 2: Blur keyed from 0 at frame 0 to 16 at frame 4, nothing else: frame 0 the drawing, then smoother. | largest difference 5.7e-8 | yes |
| FX-DEBAND-018 frame 4: Blur keyed from 0 at frame 0 to 16 at frame 4, nothing else: frame 0 the drawing, then smoother. | largest difference 5.7e-8 | yes |
| FX-DEBAND-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-019 frame 0: Both reseed intervals 2.6: their whole part, FX-DEBAND-013. | largest difference 5.9e-8 | yes |
| FX-DEBAND-019 frame 1: Both reseed intervals 2.6: their whole part, FX-DEBAND-013. | largest difference 5.9e-8 | yes |
| FX-DEBAND-019 frame 2: Both reseed intervals 2.6: their whole part, FX-DEBAND-013. | largest difference 6.0e-8 | yes |
| FX-DEBAND-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-020 frame 0: View "edges", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-020 frame 4: View "edges", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEBAND-021 frame 0: Edge Threshold 300, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-021 frame 4: Edge Threshold 300, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEBAND-022 frame 0: Blur Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-022 frame 4: Blur Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEBAND-023 frame 0: Noise Reseed Interval 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-023 frame 4: Noise Reseed Interval 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DEBAND-024 frame 0: Noise Amount keyed to 40 at frame 4, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-024 frame 4: Noise Amount keyed to 40 at frame 4, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_deband_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_018.json is saved with its word and numbers as written, Blur Amount's keys kept, and no frame | {"blur_amount":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":16}]},"diffusion_amount":0,"diffusion_reseed_interval":1,"edge_blur_reduction":100,"edge_diffusion_reduction":100,"edge_noise_reduction":100,"edge_softness":2,"edge_threshold":4,"noise_amount":0,"noise_reseed_interval":1,"view":"normal"} | yes |
| fx_deband_020.json is refused in a sentence naming "edges" | Deband's view is one of normal, edge_only, selected_edge, and this is "edges". | yes |
| fx_deband_021.json is refused in a sentence naming 300 | Deband's edge threshold runs from 0 to 255, and this is 300. | yes |
| fx_deband_022.json is refused in a sentence naming -1 | Deband's blur amount runs from 0 to 100, and this is -1. | yes |
| fx_deband_023.json is refused in a sentence naming 101 | Deband's noise reseed interval runs from 0 to 100, and this is 101. | yes |
| fx_deband_024.json is refused in a sentence naming 40 | Deband's noise amount runs from 0 to 32, and this is 40. | yes |
| a file with a Deband with no `view` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| view "edges" is refused with a sentence, and nothing changes | Deband's view is one of normal, edge_only, selected_edge, and this is "edges". | yes |
| blur amount -1 is refused with a sentence, and nothing changes | Deband's blur amount runs from 0 to 100, and this is -1. | yes |
| noise amount keyed to 40 is refused with a sentence, and nothing changes | Deband's noise amount runs from 0 to 32, and this is 40. | yes |
| view Edge Only, blur 12, diffusion 2 is taken | taken | yes |
| blur amount keyed from 0 to 16 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_deband_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_001.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_deband_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_deband_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot with Deband as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1126175 pixels changed | yes |
| the reference shot, Deband as added on three layers, frame 0, Full | largest difference 1 of 255, 1622 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband as added on three layers, frame 100, Full | largest difference 1 of 255, 1794 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband as added on three layers, frame 239, Full | largest difference 1 of 255, 1426 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband as added on three layers, frame 0, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband as added on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband as added on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot with Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1973790 pixels changed | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 0, Full | largest difference 1 of 255, 587 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 100, Full | largest difference 1 of 255, 484 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 239, Full | largest difference 1 of 255, 428 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 0, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 100, Draft | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge on three layers, frame 239, Draft | largest difference 1 of 255, 18 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot with Deband threshold keyed 2 to 20, Edge Blur Reduction 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1417603 pixels changed | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 0, Full | largest difference 1 of 255, 11976 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 100, Full | largest difference 1 of 255, 5560 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 239, Full | largest difference 1 of 255, 6580 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 0, Draft | largest difference 1 of 255, 152 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Deband threshold keyed 2 to 20, Edge Blur Reduction 50 on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a banded dusk sky, in `verification/D-456 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_banded.png, the sky as drawn: bands 18 rows tall, each a step of 3 levels; draws cleanly | [], each row on average 0.67 levels from the smooth gradient (0_smooth.png); largest step between rows 3.00 levels | yes |
| blur, diffusion and noise 0: the sky untouched; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, Deband as added: closer to the smooth gradient than 1, smaller steps between rows, and the hill's edge keeps at least nine tenths of its step; draws cleanly | [], each row on average 0.37 levels from the smooth gradient; largest step between rows 0.60 levels; edge 303.6 against 303.6 as drawn | yes |
| Deband as added, frame 1: fresh dither, so it differs from frame 0 | 63997 pixels differ | yes |
| 3_strong.png, blur 40, diffusion 3, noise 2: smaller steps between rows than 1, and the hill's edge still keeps nine tenths of its step; draws cleanly | [], largest step between rows 0.60 levels against 3.00 in 1; edge 303.6 against 303.6 as drawn | yes |
| 4_edge_only.png, View Edge Only: the hill's outline white, the sky's 3-level bands black (not edges); draws cleanly | [], 796 white pixels, 128804 black of 129600 | yes |
| 5_selected_edge.png, View Selected Edge: 2 with the hill's outline in red, as many red pixels as 4 has white; draws cleanly | [], 796 red pixels | yes |

## Result

157 of 157 checks pass.
