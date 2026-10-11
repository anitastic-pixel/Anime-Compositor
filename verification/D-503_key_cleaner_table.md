# D-503: Key Cleaner

B-383, after After Effects' Key Cleaner, the rule ours (Adobe publishes none): D-352's Refine Soft Matte fit at the Additional Edge Radius (its floor): over the edge region, the pixels whose square of that radius holds a covered pixel and a pixel not fully covered, the covering becomes the guided filter of the covering by the straight sRGB colours, held inside 0 to 1; then Alpha Contrast steepens it, ramp(a, 0.5, 1 - contrast / 100); an empty pixel that gains covering takes the covering-weighted average colour about it. Strength mixes the result back toward the layer, premultiplied. Reduce Chatter, a smoothing over time, is not built. The layer never grows. Every expected pixel is `Fixtures/key_cleaner/expected_key_cleaner.json`, written by `tools/key_cleaner_reference.py` before this code existed and printed in document 25 as FX-KCLEAN-001 to 018. Tolerance 2e-5.

## FX-KCLEAN-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KCLEAN-001 frame 0: Key Cleaner as added: Additional Edge Radius 10, Alpha Contrast 0, Strength 100. Within 10 pixels of the matte's edge, on so small a drawing nearly all of it, the covering is fitted to the colours: the strands' covering follows their orange and teal rows, the rectangle's teal ring and right side lose some, and empty pixels near the matte gain a little, in the colours about them. | largest difference 1.1e-7 | yes |
| FX-KCLEAN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-002 frame 0: Additional Edge Radius 4: the fit is worked over smaller squares, so it follows the colours more closely. | largest difference 1.7e-7 | yes |
| FX-KCLEAN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-003 frame 0: Additional Edge Radius 0, Alpha Contrast 0: nothing to do, the output is the drawing. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-004 frame 0: Additional Edge Radius 4, Alpha Contrast 50: FX-KCLEAN-002's covering steepened, below a quarter gone and above three quarters full. | largest difference 3.1e-7 | yes |
| FX-KCLEAN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-005 frame 0: Additional Edge Radius 4, Alpha Contrast 100: every pixel full or empty, full where FX-KCLEAN-002's covering is above a half. | largest difference 1.0e-7 | yes |
| FX-KCLEAN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-006 frame 0: Additional Edge Radius 0, Alpha Contrast 50: no fit, only the part-covered pixels steepened: the strands and the half-covered right edge. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-007 frame 0: FX-KCLEAN-002 at Strength 50: halfway between the drawing and FX-KCLEAN-002, premultiplied. | largest difference 1.2e-7 | yes |
| FX-KCLEAN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-008 frame 0: Strength 0: the output is the drawing. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-009 frame 0: Additional Edge Radius keyed from 0 at frame 0 to 6 at frame 4: whole pixels, so frame 1 (1.5) works at 1, frame 2 at 3 and frame 3 (4.5) at 4. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-009 frame 1: Additional Edge Radius keyed from 0 at frame 0 to 6 at frame 4: whole pixels, so frame 1 (1.5) works at 1, frame 2 at 3 and frame 3 (4.5) at 4. | largest difference 1.6e-7 | yes |
| FX-KCLEAN-009 frame 2: Additional Edge Radius keyed from 0 at frame 0 to 6 at frame 4: whole pixels, so frame 1 (1.5) works at 1, frame 2 at 3 and frame 3 (4.5) at 4. | largest difference 1.4e-7 | yes |
| FX-KCLEAN-009 frame 3: Additional Edge Radius keyed from 0 at frame 0 to 6 at frame 4: whole pixels, so frame 1 (1.5) works at 1, frame 2 at 3 and frame 3 (4.5) at 4. | largest difference 1.7e-7 | yes |
| FX-KCLEAN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-010 frame 0: Strength keyed from 0 at frame 0 to 100 at frame 4: the drawing, then halfway at frame 2, then FX-KCLEAN-001. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-010 frame 2: Strength keyed from 0 at frame 0 to 100 at frame 4: the drawing, then halfway at frame 2, then FX-KCLEAN-001. | largest difference 1.2e-7 | yes |
| FX-KCLEAN-010 frame 4: Strength keyed from 0 at frame 0 to 100 at frame 4: the drawing, then halfway at frame 2, then FX-KCLEAN-001. | largest difference 1.1e-7 | yes |
| FX-KCLEAN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-011 frame 0: FX-KCLEAN-001 moved three pixels right: worked in the drawing's own space, so the same, moved. | largest difference 1.1e-7 | yes |
| FX-KCLEAN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-012 frame 0: Additional Edge Radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-012 frame 4: Additional Edge Radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-013 frame 0: Additional Edge Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-013 frame 4: Additional Edge Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-014 frame 0: Alpha Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-014 frame 4: Alpha Contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-015 frame 0: Alpha Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-015 frame 4: Alpha Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-016 frame 0: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-016 frame 4: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-017 frame 0: Strength -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-017 frame 4: Strength -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KCLEAN-018 frame 0: Strength keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-018 frame 4: Strength keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kclean_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_001.json is saved with its three numbers as written | {"additional_edge_radius":10,"alpha_contrast":0,"strength":100} | yes |
| fx_kclean_012.json is refused in a sentence | Key Cleaner's additional edge radius runs from 0 to 100, and this is 101. | yes |
| fx_kclean_013.json is refused in a sentence | Key Cleaner's additional edge radius runs from 0 to 100, and this is -1. | yes |
| fx_kclean_014.json is refused in a sentence | Key Cleaner's alpha contrast runs from 0 to 100, and this is 101. | yes |
| fx_kclean_015.json is refused in a sentence | Key Cleaner's alpha contrast runs from 0 to 100, and this is -1. | yes |
| fx_kclean_016.json is refused in a sentence | Key Cleaner's strength runs from 0 to 100, and this is 101. | yes |
| fx_kclean_017.json is refused in a sentence | Key Cleaner's strength runs from 0 to 100, and this is -1. | yes |
| a file with a Key Cleaner with no `strength` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Key Cleaner with a radius in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| additional edge radius 101 is refused with a sentence, and nothing changes | Key Cleaner's additional edge radius runs from 0 to 100, and this is 101. | yes |
| alpha contrast -1 is refused with a sentence, and nothing changes | Key Cleaner's alpha contrast runs from 0 to 100, and this is -1. | yes |
| strength 101 is refused with a sentence, and nothing changes | Key Cleaner's strength runs from 0 to 100, and this is 101. | yes |
| strength keyed to 101 is refused with a sentence, and nothing changes | Key Cleaner's strength runs from 0 to 100, and this is 101. | yes |
| radius 4, contrast 30, strength 75 is taken | taken | yes |
| additional edge radius keyed from 0 to 6 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview halves Additional Edge Radius and keeps Alpha Contrast and Strength | KeyCleaner { additional_edge_radius: 4.0, alpha_contrast: 30.0, strength: 75.0 } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kclean_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255 (the cleaner is drawn on the processor; the card draws what follows it)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kclean_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kclean_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Key Cleaner as added on the cels: the processor's frame 100 differs from the shot without the Key Cleaner, so the comparisons below test the effect | 13094 pixels changed | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 0, Full | largest difference 1 of 255, 5190 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 100, Full | largest difference 1 of 255, 6179 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 239, Full | largest difference 1 of 255, 5176 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 0, Draft | largest difference 1 of 255, 16885 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 100, Draft | largest difference 1 of 255, 16983 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner as added on the cels on three layers, frame 239, Draft | largest difference 1 of 255, 17084 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added: the processor's frame 100 differs from the shot without the Key Cleaner, so the comparisons below test the effect | 683913 pixels changed | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 0, Full | largest difference 1 of 255, 19934 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 100, Full | largest difference 1 of 255, 20018 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 239, Full | largest difference 1 of 255, 18832 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 0, Draft | largest difference 1 of 255, 17393 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 100, Draft | largest difference 1 of 255, 17795 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract (black point 70, softness 60), then Key Cleaner as added on three layers, frame 239, Draft | largest difference 1 of 255, 17622 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60: the processor's frame 100 differs from the shot without the Key Cleaner, so the comparisons below test the effect | 672038 pixels changed | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 0, Full | largest difference 1 of 255, 19287 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 100, Full | largest difference 1 of 255, 18409 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 239, Full | largest difference 1 of 255, 18281 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 0, Draft | largest difference 1 of 255, 17305 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 100, Draft | largest difference 1 of 255, 17725 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Extract, then Key Cleaner radius 4, contrast 40, strength 60 on three layers, frame 239, Draft | largest difference 1 of 255, 17507 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-503 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_keyed.png, the street keyed by Extract (black point 70, softness 60), no cleaner; draws cleanly | [] | yes |
| 2_cleaned.png, then Key Cleaner as added (radius 10): the soft edges of the key fitted to the street's colours; draws cleanly | [], 14334 pixels changed of 129600 | yes |
| 3_radius_3.png, then Key Cleaner radius 3: a tighter fit; draws cleanly | [], 203 pixels changed of 129600 | yes |
| 4_contrast_100.png, then Key Cleaner with Alpha Contrast 100: every pixel kept or gone; draws cleanly | [], 9036 pixels changed of 129600 | yes |
| 5_strength_50.png, then Key Cleaner at Strength 50: halfway back to the key alone; draws cleanly | [], 8768 pixels changed of 129600 | yes |

## Result

132 of 132 checks pass.
