# B-908: Gradient's Ramp Scatter and Blend With Original (D-1029)

D-1029 adds After Effects' Ramp Scatter as `ramp_scatter` (0 to 256, keyable, absent 0): each pixel's place along the ramp is moved by up to ramp_scatter / 512 with a seeded number fixed to the drawing, against banding; and Blend With Original as `blend_with_original` (0 to 100, keyable, absent 0): the layer as it was mixed back in by that share. A file without them draws as before. Every expected pixel is `Fixtures/gradient/expected_gradient.json`, written by `tools/gradient_reference.py` before this code existed and printed in document 25 as FX-GRAD-023 to 032.

## FX-GRAD-023 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRAD-023 frame 0: Ramp Scatter 64 on FX-GRAD-002 (normal, both opacities 100): the ramp grained, each pixel's place along it moved by up to an eighth of the ramp either way, fixed to the drawing; pixels in one row no longer all alike. | largest difference 3.0e-8 | yes |
| FX-GRAD-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-024 frame 0: Ramp Scatter 0 and Blend With Original 0 written: FX-GRAD-002 exactly. | largest difference 3.0e-8 | yes |
| FX-GRAD-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-025 frame 0: Blend With Original 50 on FX-GRAD-002: every pixel halfway between the drawing and FX-GRAD-002's. | largest difference 1.1e-7 | yes |
| FX-GRAD-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-026 frame 0: Blend With Original 100: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-GRAD-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-027 frame 0: Ramp Scatter keyed from 0 at frame 0 to 256 at frame 4: frame 0 is FX-GRAD-002, frame 2 scatter 128, frame 4 the most, half the ramp either way. | largest difference 3.0e-8 | yes |
| FX-GRAD-027 frame 2: Ramp Scatter keyed from 0 at frame 0 to 256 at frame 4: frame 0 is FX-GRAD-002, frame 2 scatter 128, frame 4 the most, half the ramp either way. | largest difference 3.1e-8 | yes |
| FX-GRAD-027 frame 4: Ramp Scatter keyed from 0 at frame 0 to 256 at frame 4: frame 0 is FX-GRAD-002, frame 2 scatter 128, frame 4 the most, half the ramp either way. | largest difference 4.2e-8 | yes |
| FX-GRAD-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-028 frame 0: FX-GRAD-023 moved three pixels right: the grain is fixed to the drawing, so it moves with it. | largest difference 3.0e-8 | yes |
| FX-GRAD-028 frame 3: FX-GRAD-023 moved three pixels right: the grain is fixed to the drawing, so it moves with it. | largest difference 3.0e-8 | yes |
| FX-GRAD-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-029 frame 0: Ramp Scatter 256 with white to white: a scattered place along one colour is still that colour, so this is FX-GRAD-013. | largest difference 3.0e-8 | yes |
| FX-GRAD-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRAD-030 frame 0: Ramp Scatter 257, above 256. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-030 frame 4: Ramp Scatter 257, above 256. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-031 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-031 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRAD-032 frame 0: Blend With Original keyed to -1 at frame 4, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-032 frame 4: Blend With Original keyed to -1 at frame 4, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRAD-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: neither setting, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grad_001.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_002.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_003.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_004.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_005.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_006.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_007.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_008.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_009.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_010.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_011.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_012.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_013.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_014.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_015.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_016.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_017.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_018.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_019.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_020.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_021.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |
| fx_grad_022.json (neither setting) is saved as it was written (letters' case aside), and draws frames 0 and 4 bit for bit as its twin with both written 0 | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grad_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grad_023.json is saved with Ramp Scatter 64 and no Blend With Original written | {"blend":"normal","end":[50,100],"end_color":"#6450a0","end_opacity":100,"ramp_scatter":64,"shape":"linear","start":[50,0],"start_color":"#ffffff","start_opacity":100} | yes |
| fx_grad_025.json is saved with Blend With Original 50 | {"blend":"normal","blend_with_original":50,"end":[50,100],"end_color":"#6450a0","end_opacity":100,"shape":"linear","start":[50,0],"start_color":"#ffffff","start_opacity":100} | yes |
| fx_grad_001.json (neither) is saved with neither written | {"blend":"multiply","end":[50,100],"end_color":"#6450a0","end_opacity":50,"shape":"linear","start":[50,0],"start_color":"#ffffff","start_opacity":0} | yes |
| fx_grad_030.json (Ramp Scatter 257) is refused in a sentence naming ramp scatter | Gradient's ramp scatter runs from 0 to 256, and this is 257. | yes |
| fx_grad_031.json (Blend With Original 101) is refused in a sentence naming blend with original | Gradient's blend with original runs from 0 to 100, and this is 101. | yes |
| a file with a Gradient whose Ramp Scatter is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview keeps Ramp Scatter 64, not a distance | Gradient { shape: "linear", start: [50.0, 0.0], end: [50.0, 100.0], start_color: "#ffffff", end_color: "#6450a0", start_opacity: 100.0, end_opacity: 100.0, blend: "normal", ramp_scatter: 64.0, blend_with_original: 0.0 } | yes |
| Ramp Scatter never grows the layer | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| ramp scatter 256.5 is refused with a sentence, and nothing changes | Gradient's ramp scatter runs from 0 to 256, and this is 256.5. | yes |
| blend with original keyed to 150 is refused with a sentence, and nothing changes | Gradient's blend with original runs from 0 to 100, and this is 150. | yes |
| ramp scatter 40, blend with original 25 is taken | taken | yes |
| ramp scatter keyed from 0 to 256 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grad_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grad_027.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grad_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grad_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| the card draws a Gradient with both settings 0 as before; one with either above 0 stays on the processor, so those frames are the processor's | see the rows below | yes |
| FX-GRAD-001, fx_grad_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-002, fx_grad_002.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-003, fx_grad_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-004, fx_grad_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-005, fx_grad_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-006, fx_grad_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-007, fx_grad_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-008, fx_grad_008.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-009, fx_grad_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-010, fx_grad_010.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-011, fx_grad_011.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-012, fx_grad_012.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-013, fx_grad_013.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-014, fx_grad_014.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-015, fx_grad_015.json, frames [0, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-016, fx_grad_016.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-017, fx_grad_017.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-018, fx_grad_018.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-019, fx_grad_019.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-020, fx_grad_020.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-021, fx_grad_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-022, fx_grad_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-023, fx_grad_023.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-024, fx_grad_024.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-025, fx_grad_025.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-026, fx_grad_026.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-027, fx_grad_027.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 2 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-028, fx_grad_028.json, frames [0, 3] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-029, fx_grad_029.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-030, fx_grad_030.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-031, fx_grad_031.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-GRAD-032, fx_grad_032.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, the night sky, both settings 0 written (on the card): the processor's frame 100 differs from the shot without it | 2073600 pixels changed | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 0, Full | largest difference 1 of 255, 928 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 100, Full | largest difference 1 of 255, 1112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 239, Full | largest difference 1 of 255, 952 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 0, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 100, Draft | largest difference 1 of 255, 116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, both settings 0 written (on the card), frame 239, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's): the processor's frame 100 differs from the shot without it | 2073600 pixels changed | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 0, Full | largest difference 1 of 255, 54615 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 100, Full | largest difference 1 of 255, 54136 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 239, Full | largest difference 1 of 255, 54081 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 0, Draft | largest difference 1 of 255, 3326 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 100, Draft | largest difference 1 of 255, 3302 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, the night sky, Ramp Scatter 64, Blend With Original 40 (the processor's), frame 239, Draft | largest difference 1 of 255, 3302 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/B-908 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_scatter_0.png and its four-times corner 1_scatter_0_zoom.png: the street under the night sky at Ramp Scatter 0; draws cleanly | []; 0 pixels differ from Scatter 0, by up to 0 levels | yes |
| 2_scatter_64.png and its four-times corner 2_scatter_64_zoom.png: the street under the night sky at Ramp Scatter 64; draws cleanly, and differs from Scatter 0 over many pixels, by at most a third of the light | []; 126482 pixels differ from Scatter 0, by up to 17 levels | yes |
| 3_scatter_256.png and its four-times corner 3_scatter_256_zoom.png: the street under the night sky at Ramp Scatter 256; draws cleanly, and differs from Scatter 0 over many pixels, by at most a third of the light | []; 128427 pixels differ from Scatter 0, by up to 66 levels | yes |
| 4_blend_with_original_50.png: the night sky at Blend With Original 50, the street showing through half; draws cleanly and differs from 1_scatter_0.png | []; up to 157 levels from Scatter 0 | yes |

## Result

123 of 123 checks pass.
