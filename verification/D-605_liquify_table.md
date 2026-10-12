# D-605: Liquify

B-484: `core.liquify` takes After Effects' Liquify controls (Tool, Brush Size, Brush Pressure, Distortion Percentage, Distortion Mesh Offset) and a stroke typed as points, drags the brush along it in dabs, and draws the drawing through D-601's backward field. Every expected pixel is `Fixtures/liquify/expected_liquify.json`, written by `tools/liquify_reference.py` before this code existed and printed in document 25 as FX-LIQUIFY-001 to 021. Tolerance 2e-5.

## FX-LIQUIFY-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIQUIFY-001 frame 0: The settings as they start: Warp, brush 64, pressure 50, and no stroke: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-002 frame 0: Warp, brush 8, pressure 50, stroked across the middle from a quarter to three quarters: the stripes under the stroke pushed right. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-003 frame 0: Twirl Clockwise, brush 10, pressure 100, one dab in the middle: the stripes there turned clockwise. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-004 frame 0: Twirl Counterclockwise, as FX-LIQUIFY-003: turned the other way. | largest difference 2.7e-7 | yes |
| FX-LIQUIFY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-005 frame 0: Pucker, brush 10, pressure 100, one dab in the middle: the drawing drawn in toward it. | largest difference 2.6e-7 | yes |
| FX-LIQUIFY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-006 frame 0: Bloat, as FX-LIQUIFY-005: swollen out from it. | largest difference 2.6e-7 | yes |
| FX-LIQUIFY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-007 frame 0: Shift Pixels, stroked as FX-LIQUIFY-002: the stripes moved square to the stroke, up the picture, to its left. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-008 frame 0: FX-LIQUIFY-002 at Distortion Percentage 200: pushed twice as far. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-009 frame 0: FX-LIQUIFY-002 with the Distortion Mesh Offset 3 pixels right: the push lands three pixels on. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-010 frame 0: FX-LIQUIFY-002 with the layer moved three pixels right: the pushed drawing moves with it. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-011 frame 0: Distortion Percentage keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 4 FX-LIQUIFY-002, frame 2 between. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-011 frame 2: Distortion Percentage keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 4 FX-LIQUIFY-002, frame 2 between. | largest difference 2.0e-7 | yes |
| FX-LIQUIFY-011 frame 4: Distortion Percentage keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 4 FX-LIQUIFY-002, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-012 frame 0: The stroke keyed from across the middle at frame 0 to rising to the top right at frame 4: the push turns upward. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-012 frame 2: The stroke keyed from across the middle at frame 0 to rising to the top right at frame 4: the push turns upward. | largest difference 3.1e-7 | yes |
| FX-LIQUIFY-012 frame 4: The stroke keyed from across the middle at frame 0 to rising to the top right at frame 4: the push turns upward. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-013 frame 0: Warp, brush 6, pressure 80, stroked down and back up in a V: the push follows each leg. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-014 frame 0: Brush Pressure keyed from 20 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-014 frame 2: Brush Pressure keyed from 20 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-014 frame 4: Brush Pressure keyed from 20 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-015 frame 0: Warp with a brush of 1000, wider than the drawing: the whole drawing pushed right almost evenly, four pixels, clear down the left. | largest difference 2.5e-7 | yes |
| FX-LIQUIFY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIQUIFY-016 frame 0: Tool "freeze", not one of the six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-016 frame 4: Tool "freeze", not one of the six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-017 frame 0: Brush Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-017 frame 4: Brush Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-018 frame 0: Brush Pressure 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-018 frame 4: Brush Pressure 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-019 frame 0: A stroke of three numbers, half a point over. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-019 frame 4: A stroke of three numbers, half a point over. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-020 frame 0: Distortion Percentage 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-020 frame 4: Distortion Percentage 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIQUIFY-021 frame 0: The stroke keyed to end at 300 per cent across at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-021 frame 4: The stroke keyed to end at 300 per cent across at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LIQUIFY-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_liquify_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_liquify_001.json is saved with its six settings, the tool as a word and the stroke as an empty list | {"brush_pressure":50,"brush_size":64,"distortion_mesh_offset":[0,0],"distortion_percentage":100,"stroke":[],"tool":"warp"} | yes |
| fx_liquify_016.json is refused in a sentence naming Liquify and its tool | Liquify's tool is warp, twirl_clockwise, twirl_counterclockwise, pucker, bloat or shift_pixels, and this is "freeze". | yes |
| fx_liquify_017.json is refused in a sentence naming Liquify and its brush size | Liquify's brush size runs from 1 to 1000, and this is 0. | yes |
| fx_liquify_018.json is refused in a sentence naming Liquify and its brush pressure | Liquify's brush pressure runs from 1 to 100, and this is 101. | yes |
| fx_liquify_019.json is refused in a sentence naming Liquify and its stroke | Liquify's stroke is none, or x then y for each point, and this has 3 numbers. | yes |
| fx_liquify_020.json is refused in a sentence naming Liquify and its distortion percentage | Liquify's distortion percentage runs from 0 to 200, and this is 201. | yes |
| a file with a Liquify whose tool is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Liquify whose stroke is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Liquify whose offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Liquify without its brush size is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tool freeze is refused with a sentence, and nothing changes | Liquify's tool is warp, twirl_clockwise, twirl_counterclockwise, pucker, bloat or shift_pixels, and this is "freeze". | yes |
| brush size 0 is refused with a sentence, and nothing changes | Liquify's brush size runs from 1 to 1000, and this is 0. | yes |
| a stroke of three numbers is refused with a sentence, and nothing changes | Liquify's stroke is none, or x then y for each point, and this has 3 numbers. | yes |
| the stroke keyed to end at 300 per cent across is refused with a sentence, and nothing changes | Liquify's stroke runs from -100 to 200, and this is 300. | yes |
| Twirl Clockwise, brush 30, one dab in the middle is taken | taken | yes |
| distortion percentage keyed from 0 to 200 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_liquify_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_liquify_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_liquify_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_liquify_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 190894 pixels changed | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 0, Full | largest difference 1 of 255, 2608 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 100, Full | largest difference 1 of 255, 973 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 239, Full | largest difference 1 of 255, 1112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 0, Draft | largest difference 1 of 255, 85 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Warp, brush 200, stroked across the middle on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 104641 pixels changed | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 0, Full | largest difference 1 of 255, 2092 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 100, Full | largest difference 1 of 255, 1139 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 239, Full | largest difference 1 of 255, 965 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 0, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 100, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Pucker, brush 400, pressure 70, one dab in the middle on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 241980 pixels changed | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 0, Full | largest difference 1 of 255, 2505 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 100, Full | largest difference 1 of 255, 934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 239, Full | largest difference 1 of 255, 761 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 0, Draft | largest difference 1 of 255, 103 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 100, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Liquify Twirl Clockwise, brush 600, 150 per cent, offset 20 by 10 on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-605 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect, every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| As added, with nothing painted: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_warp.png, Warp, brush 120, dragged across the middle from a fifth to four fifths: the street under the stroke smeared right, the corners untouched; draws cleanly | [], 10477 pixels differ from the street, top left corner untouched: true | yes |
| 3_twirl_clockwise.png, Twirl Clockwise, brush 200, pressure 100, one dab in the middle: the middle of the street turned; draws cleanly | [], 13489 pixels differ from the street, top left corner untouched: true | yes |
| 4_twirl_counterclockwise.png, the same dab turning the other way, so it differs from 3_twirl_clockwise.png; draws cleanly | [], 17492 pixels differ from 3_twirl_clockwise.png | yes |
| 5_pucker.png, Pucker, the same dab: the middle of the street shrunk toward the dab; draws cleanly | [], 13377 pixels differ from the street | yes |
| 6_bloat.png, Bloat, the same dab: the middle swollen out instead, so it differs from 5_pucker.png; draws cleanly | [], 17618 pixels differ from 5_pucker.png | yes |
| 7_shift_pixels.png, Shift Pixels along 2_warp.png's stroke: the street under it moved square to the stroke, so it differs from 2_warp.png; draws cleanly | [], 30031 pixels differ from 2_warp.png | yes |

## Result

147 of 147 checks pass.
