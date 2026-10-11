# D-447: Brush Strokes

B-327: `core.brush_strokes` takes After Effects' Brush Strokes controls (Stroke Angle, Brush Size, Stroke Length, Stroke Density, Stroke Randomness, Paint Surface, Blend With Original) with our Random Seed and Animate. Every expected pixel is `Fixtures/brush_strokes/expected_brush_strokes.json`, written by `tools/brush_strokes_reference.py` before this code existed and printed in document 25 as FX-BRUSH-001 to 029. Tolerance 2e-5.

## FX-BRUSH-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BRUSH-001 frame 0: The settings as they start: angle 135, brush 2, length 8, density 1, randomness 1, on the original: the card smeared down and to the right in strokes, new ones each frame. | largest difference 1.5e-7 | yes |
| FX-BRUSH-001 frame 1: The settings as they start: angle 135, brush 2, length 8, density 1, randomness 1, on the original: the card smeared down and to the right in strokes, new ones each frame. | largest difference 2.1e-7 | yes |
| FX-BRUSH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-002 frame 0: Animate off: the same strokes on frames 0 and 3. | largest difference 1.5e-7 | yes |
| FX-BRUSH-002 frame 3: Animate off: the same strokes on frames 0 and 3. | largest difference 1.5e-7 | yes |
| FX-BRUSH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-003 frame 0: Random Seed 7, held: a different set of strokes from FX-BRUSH-002's. | largest difference 1.9e-7 | yes |
| FX-BRUSH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-004 frame 0: Stroke Length 0, held: dots, not strokes. | largest difference 1.9e-7 | yes |
| FX-BRUSH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-005 frame 0: Stroke Angle 90, Randomness 0, on transparent, held: level strokes running right, each the colour where it starts. | largest difference 1.9e-7 | yes |
| FX-BRUSH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-006 frame 0: Stroke Angle 0, held: strokes running up. | largest difference 2.0e-7 | yes |
| FX-BRUSH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-007 frame 0: Randomness 0, held: every stroke the same length and width, on a regular grid. | largest difference 2.1e-7 | yes |
| FX-BRUSH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-008 frame 0: Randomness 2, held: the strokes tilt further from the angle. | largest difference 1.9e-7 | yes |
| FX-BRUSH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-009 frame 0: Brush Size 4, held: fatter strokes. | largest difference 2.0e-7 | yes |
| FX-BRUSH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-010 frame 0: Density 3, held: more strokes, overlapping. | largest difference 1.9e-7 | yes |
| FX-BRUSH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-011 frame 0: Density 0.4, held: fewer strokes, the card showing between them. | largest difference 1.9e-7 | yes |
| FX-BRUSH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-012 frame 0: Paint on transparent, held: between the strokes nothing shows. | largest difference 3.3e-8 | yes |
| FX-BRUSH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-013 frame 0: Paint on white, held: white between the strokes. | largest difference 4.2e-8 | yes |
| FX-BRUSH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-014 frame 0: Paint on black, held: black between the strokes. | largest difference 3.3e-8 | yes |
| FX-BRUSH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-015 frame 0: Blend With Original 100: the card as it was. | largest difference 1.9e-7 | yes |
| FX-BRUSH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-016 frame 0: Blend With Original 50, held: halfway between FX-BRUSH-002 and the card. | largest difference 1.8e-7 | yes |
| FX-BRUSH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-017 frame 0: Brush Size keyed from 0.5 at frame 0 to 4 at frame 4, held. | largest difference 1.9e-7 | yes |
| FX-BRUSH-017 frame 2: Brush Size keyed from 0.5 at frame 0 to 4 at frame 4, held. | largest difference 2.0e-7 | yes |
| FX-BRUSH-017 frame 4: Brush Size keyed from 0.5 at frame 0 to 4 at frame 4, held. | largest difference 2.0e-7 | yes |
| FX-BRUSH-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-018 frame 0: FX-BRUSH-002 moved three pixels right: the strokes are the drawing's own, so they move with it. | largest difference 1.5e-7 | yes |
| FX-BRUSH-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-019 frame 0: After a Motion Tile that grows the layer: the strokes sit where they did, and pick their colours from the grown buffer. | largest difference 1.9e-7 | yes |
| FX-BRUSH-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-020 frame 0: Angle 200, brush 1.5, length 5.5, density 1.8, randomness 0.6, on black, blend 20, seed 31: the controls together. | largest difference 1.9e-7 | yes |
| FX-BRUSH-020 frame 3: Angle 200, brush 1.5, length 5.5, density 1.8, randomness 0.6, on black, blend 20, seed 31: the controls together. | largest difference 2.0e-7 | yes |
| FX-BRUSH-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BRUSH-021 frame 0: Brush Size 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-021 frame 4: Brush Size 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-022 frame 0: Stroke Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-022 frame 4: Stroke Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-023 frame 0: Stroke Density 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-023 frame 4: Stroke Density 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-024 frame 0: Stroke Randomness 2.5, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-024 frame 4: Stroke Randomness 2.5, above 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-025 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-025 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-026 frame 0: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-026 frame 4: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-027 frame 0: Paint Surface "White", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-027 frame 4: Paint Surface "White", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-028 frame 0: Animate "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-028 frame 4: Animate "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BRUSH-029 frame 0: Stroke Length keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-029 frame 4: Stroke Length keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BRUSH-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_brush_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_brush_020.json is saved with its nine settings and no frame | {"animate":"on","blend_with_original":20,"brush_size":1.5,"paint_surface":"black","random_seed":31,"stroke_angle":200,"stroke_density":1.8,"stroke_length":5.5,"stroke_randomness":0.6} | yes |
| fx_brush_021.json is refused in a sentence naming Brush Strokes and its brush size | Brush Strokes's brush size runs from 0.5 to 20, and this is 0.4. | yes |
| fx_brush_022.json is refused in a sentence naming Brush Strokes and its stroke length | Brush Strokes's stroke length runs from 0 to 100, and this is 101. | yes |
| fx_brush_023.json is refused in a sentence naming Brush Strokes and its stroke density | Brush Strokes's stroke density runs from 0.1 to 4, and this is 0.05. | yes |
| fx_brush_024.json is refused in a sentence naming Brush Strokes and its stroke randomness | Brush Strokes's stroke randomness runs from 0 to 2, and this is 2.5. | yes |
| fx_brush_025.json is refused in a sentence naming Brush Strokes and its blend with original | Brush Strokes's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_brush_026.json is refused in a sentence naming Brush Strokes and its random seed | Brush Strokes's random seed runs from 0 to 100000, and this is -1. | yes |
| fx_brush_027.json is refused in a sentence naming Brush Strokes and its paint surface | Brush Strokes's paint surface is one of original, transparent, white, black, and this is "White". | yes |
| fx_brush_028.json is refused in a sentence naming Brush Strokes and its animate | Brush Strokes's animate is "on" or "off", and this is "yes". | yes |
| a file with a Brush Strokes whose brush size is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Brush Strokes without its stroke length is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Brush Strokes whose paint surface is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| brush size 0.4 is refused with a sentence, and nothing changes | Brush Strokes's brush size runs from 0.5 to 20, and this is 0.4. | yes |
| paint surface White is refused with a sentence, and nothing changes | Brush Strokes's paint surface is one of original, transparent, white, black, and this is "White". | yes |
| stroke length keyed to 150 is refused with a sentence, and nothing changes | Brush Strokes's stroke length runs from 0 to 100, and this is 150. | yes |
| brush size 4 on white is taken | taken | yes |
| stroke angle keyed from 0 to 360 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_brush_001.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_brush_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_brush_017.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_brush_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_brush_020.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_brush_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_brush_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Brush Strokes as added (new strokes each frame): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1297192 pixels changed | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 0, Full | largest difference 1 of 255, 2755 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 100, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 239, Full | largest difference 1 of 255, 1176 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes as added (new strokes each frame) on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2027658 pixels changed | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes on black, brush 4, length 20, held on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1356895 pixels changed | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 0, Full | largest difference 1 of 255, 1903 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 100, Full | largest difference 1 of 255, 740 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 239, Full | largest difference 1 of 255, 745 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 100, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Brush Strokes angle 90, randomness 2, density 3, blend 30 on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-447 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png and 2b_as_added_frame_1.png, as added: the street repainted in short strokes leaning down and to the right; frame 1 has new strokes, so the two differ; both draw cleanly | [] [], pixels differing from the street 50694, between the frames 61833 | yes |
| 3_held.png, New Strokes Each Frame off: frame 3 the same as frame 0 byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| 4_level_strokes.png, angle 90, randomness 0, length 30: long level strokes running right, so fewer edges across than the street has, and fewer across than down; draws cleanly | [], edges across/down: street 2536/2492, strokes 1516/3076 | yes |
| 5_sparse_on_black.png, density 0.3 on black: scattered strokes with black between them, more pure black pixels than the street; draws cleanly | [], pure black pixels: street 0, strokes 111691 | yes |
| 6_brush_8.png, brush size 8, length 24: big blocky dabs, fewer edges than 3_held.png's size 2; draws cleanly | [], edges across+down: size 2 15975, size 8 9481 | yes |
| 7_blend_50.png, Blend With Original 50: every pixel between the street and 3_held.png; at 100 the street itself byte for byte; both draw cleanly | [] [], between: true, 100 the street: true | yes |

## Result

181 of 181 checks pass.
