# D-407: Detail-preserving Upscale

B-286, after After Effects' Detail-preserving Upscale: the layer softened by Reduce Noise (a Gaussian of sigma Reduce Noise / 50), grown by (Scale - 100) / 200 of its size on each side, every pixel read through a Lanczos-3 filter across then down, then sharpened by Detail / 50 of the difference from itself blurred at sigma Scale / 200, and held within what can be shown. Every expected pixel is `Fixtures/detail_upscale/expected_detail_upscale.json`, written by `tools/detail_upscale_reference.py` before this code existed and printed in document 25 as FX-UPSCALE-001 to 020. Tolerance 2e-5. The owner chose on 2026-10-10 the scale's range ("1000%, like AE") and Fit to Comp Width and Height as one-time buttons; a layer grown past 30000 pixels is not drawn and EFFECT_LAYER_TOO_LARGE says so.

## FX-UPSCALE-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-UPSCALE-001 frame 0: The settings as they start: scale 100, no softening, Detail 20: the drawing the same size, its edges a little sharpened (an unsharp mask of 0.4 at sigma 0.5). | largest difference 2.4e-7 | yes |
| FX-UPSCALE-001 frame 4: The settings as they start: scale 100, no softening, Detail 20: the drawing the same size, its edges a little sharpened (an unsharp mask of 0.4 at sigma 0.5). | largest difference 2.4e-7 | yes |
| FX-UPSCALE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-002 frame 0: Scale 100, Detail 0: the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-003 frame 0: Scale 200, Detail 0: Lanczos-3 alone; the layer grows to 32 by 20 and fills the composition. | largest difference 3.9e-7 | yes |
| FX-UPSCALE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-004 frame 0: Scale 200, Detail 20 (as added): FX-UPSCALE-003 sharpened at sigma 1. | largest difference 4.6e-7 | yes |
| FX-UPSCALE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-005 frame 0: Scale 200, Detail 100: sharpened twice over, halos at the line and the band. | largest difference 6.6e-7 | yes |
| FX-UPSCALE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-006 frame 0: Scale 150, Detail 0: the layer grows 4 each side across and 3 down (24 by 16), the middle unmoved. | largest difference 3.5e-7 | yes |
| FX-UPSCALE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-007 frame 0: Scale 200, Reduce Noise 100, Detail 0: softened at sigma 2 before the enlarging. | largest difference 3.8e-7 | yes |
| FX-UPSCALE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-008 frame 0: Scale 200, Reduce Noise 50, Detail 50: softened at sigma 1, enlarged, sharpened by 1. | largest difference 5.6e-7 | yes |
| FX-UPSCALE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-009 frame 0: Scale 300: the layer 48 by 30, cut by the composition's edges. | largest difference 2.9e-7 | yes |
| FX-UPSCALE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-010 frame 0: Scale 100, Reduce Noise 100, Detail 0: softened only. | largest difference 2.6e-7 | yes |
| FX-UPSCALE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-011 frame 0: Scale keyed from 100 at frame 0 to 300 at frame 4, linear, Detail 0: as it is, then 200 at frame 2, then 300. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-011 frame 2: Scale keyed from 100 at frame 0 to 300 at frame 4, linear, Detail 0: as it is, then 200 at frame 2, then 300. | largest difference 3.9e-7 | yes |
| FX-UPSCALE-011 frame 4: Scale keyed from 100 at frame 0 to 300 at frame 4, linear, Detail 0: as it is, then 200 at frame 2, then 300. | largest difference 2.2e-7 | yes |
| FX-UPSCALE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-012 frame 0: Detail keyed from 0 at frame 0 to 100 at frame 4, scale 200: frame 0 is FX-UPSCALE-003, frame 4 FX-UPSCALE-005. | largest difference 3.9e-7 | yes |
| FX-UPSCALE-012 frame 2: Detail keyed from 0 at frame 0 to 100 at frame 4, scale 200: frame 0 is FX-UPSCALE-003, frame 4 FX-UPSCALE-005. | largest difference 5.2e-7 | yes |
| FX-UPSCALE-012 frame 4: Detail keyed from 0 at frame 0 to 100 at frame 4, scale 200: frame 0 is FX-UPSCALE-003, frame 4 FX-UPSCALE-005. | largest difference 6.6e-7 | yes |
| FX-UPSCALE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-013 frame 0: Scale 200, Detail 20, the layer moved 3 pixels right: FX-UPSCALE-004 moved; columns 0 to 2 empty. | largest difference 4.6e-7 | yes |
| FX-UPSCALE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-014 frame 0: Scale eased from 200 at frame 0 to 100 at frame 4 on a curve that overshoots, Detail 0: at frame 2 it would pass below 100 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 3.9e-7 | yes |
| FX-UPSCALE-014 frame 2: Scale eased from 200 at frame 0 to 100 at frame 4 on a curve that overshoots, Detail 0: at frame 2 it would pass below 100 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-014 frame 4: Scale eased from 200 at frame 0 to 100 at frame 4 on a curve that overshoots, Detail 0: at frame 2 it would pass below 100 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-UPSCALE-015 frame 0: Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-015 frame 4: Scale 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UPSCALE-016 frame 0: Scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-016 frame 4: Scale 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UPSCALE-017 frame 0: Reduce Noise -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-017 frame 4: Reduce Noise -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UPSCALE-018 frame 0: Reduce Noise 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-018 frame 4: Reduce Noise 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UPSCALE-019 frame 0: Detail -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-019 frame 4: Detail -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-UPSCALE-020 frame 0: Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-020 frame 4: Detail 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-UPSCALE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_upscale_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_upscale_008.json is saved with its scale, Reduce Noise and Detail | {"detail":50,"reduce_noise":50,"scale":200} | yes |
| fx_upscale_015.json is refused in a sentence | Detail-preserving Upscale's scale runs from 100 to 1000, and this is 99. | yes |
| fx_upscale_016.json is refused in a sentence | Detail-preserving Upscale's scale runs from 100 to 1000, and this is 1001. | yes |
| fx_upscale_017.json is refused in a sentence | Detail-preserving Upscale's reduce noise runs from 0 to 100, and this is -1. | yes |
| fx_upscale_018.json is refused in a sentence | Detail-preserving Upscale's reduce noise runs from 0 to 100, and this is 101. | yes |
| fx_upscale_019.json is refused in a sentence | Detail-preserving Upscale's detail runs from 0 to 100, and this is -1. | yes |
| fx_upscale_020.json is refused in a sentence | Detail-preserving Upscale's detail runs from 0 to 100, and this is 101. | yes |
| a file with an Upscale with no `detail` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Upscale with its scale in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Upscale with two numbers for Detail is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scale 99 is refused with a sentence, and nothing changes | Detail-preserving Upscale's scale runs from 100 to 1000, and this is 99. | yes |
| Reduce Noise 101 is refused with a sentence, and nothing changes | Detail-preserving Upscale's reduce noise runs from 0 to 100, and this is 101. | yes |
| Detail keyed to 101 is refused with a sentence, and nothing changes | Detail-preserving Upscale's detail runs from 0 to 100, and this is 101. | yes |
| scale 250, Reduce Noise 30, Detail 40 is taken | taken | yes |
| scale keyed from 100 to 400 is taken | taken | yes |
| Detail keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_upscale_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_upscale_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Too large for this build: EFFECT_LAYER_TOO_LARGE (document 28)

| Check | The build's answer | Matches |
| --- | --- | --- |
| three Motion Tiles make the 16-pixel layer 16000 wide; an Upscale of 200 would make it 32000, past 30000, so the processor leaves the Upscale out and says so | warnings [EFFECT_LAYER_TOO_LARGE]; 0 pixels differ from the frame without the Upscale | yes |
| the same frame asked of the card: the Upscale left out the same way and the same warning | largest difference 0 of 255 from the processor; warnings [EFFECT_LAYER_TOO_LARGE] | yes |
| the card's own plan refuses it before drawing, with the same warning | [EFFECT_LAYER_TOO_LARGE]; 0 on the card | yes |
| an Upscale of 187.5 makes it exactly 30000 wide, which is drawn | warnings []; 608 pixels differ from the frame without the Upscale | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_upscale_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_upscale_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_upscale_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; the same warnings: true | yes |
| fx_upscale_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_upscale_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 296689 pixels changed | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 0, Full | largest difference 1 of 255, 1650 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 100, Full | largest difference 1 of 255, 1077 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 239, Full | largest difference 1 of 255, 791 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 0, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale as it starts (scale 100, Detail 20) on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2061293 pixels changed | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 0, Full | largest difference 1 of 255, 1099 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 100, Full | largest difference 1 of 255, 1024 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 239, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 100, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 200, Detail 0 on three layers, frame 239, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2043280 pixels changed | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 0, Full | largest difference 1 of 255, 1144 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 100, Full | largest difference 1 of 255, 1035 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 239, Full | largest difference 1 of 255, 1029 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 0, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 150, Reduce Noise 50, Detail 50 on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2003284 pixels changed | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 0, Full | largest difference 1 of 255, 1471 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 100, Full | largest difference 1 of 255, 952 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 239, Full | largest difference 1 of 255, 850 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Detail-preserving Upscale scale 125, Detail 100 on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-407 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: the same size, edges a little sharpened; draws cleanly | [], 11937 pixels changed | yes |
| 3_scale_200.png, scale 200: twice the size about the middle, cut by the frame's edges; draws cleanly | [], 127417 pixels changed | yes |
| 4_scale_1000.png, scale 1000, the most: ten times the size, the middle of the street filling the frame; draws cleanly | [], 127978 pixels changed | yes |
| 5_soft.png, scale 150, Reduce Noise 100, Detail 0: enlarged and softened; draws cleanly | [], 121813 pixels changed | yes |
| 6_sharp.png, scale 150, Detail 100: enlarged and sharpened hard, light halos at the edges; draws cleanly | [], 116802 pixels changed | yes |

## Result

155 of 155 checks pass.
