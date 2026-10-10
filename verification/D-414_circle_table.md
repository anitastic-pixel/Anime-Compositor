# D-414: Circle

B-293, after After Effects' Circle: a ring from Ri to Ro about Center, a share of the drawing's own size (None: Radius and 0; Edge Radius: the larger and smaller of Radius and Edge Radius; Thickness: Radius and Radius - Thickness, the ring inside the radius; Thickness * Radius: the thickness Thickness times Radius / 100; Thickness & Feather * Radius: the feathers too). The covering is a straight ramp max(feather, 1) pixels wide centred on Ro, times one on Ri when Ri > 0, turned over by Invert Circle. The shape, Color times the covering times Opacity, replaces the layer for None or is laid on it by document 21's layer blend (Normal, Add, Multiply, Screen, Overlay, Soft Light, Stencil Alpha). The layer never grows. Every expected pixel is `Fixtures/circle/expected_circle.json`, written by `tools/circle_reference.py` before this code existed and printed in document 25 as FX-CIRCLE-001 to 040. Tolerance 2e-5.

## FX-CIRCLE-001 to 040 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CIRCLE-001 frame 0: The settings as they start: centre in the middle, radius 75, edge none, white, opacity 100, blending mode none: the disk covers the whole small frame, white everywhere, the cel gone. | largest difference 0.0e0 | yes |
| FX-CIRCLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-002 frame 0: Radius 4: a white disk 8 across at (8, 5), the pixels on its rim partly covered, the rest clear. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-003 frame 0: Radius 4, outer feather 4: the rim ramps over 4 pixels, from 2 inside to 2 outside. | largest difference 2.6e-8 | yes |
| FX-CIRCLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-004 frame 0: Edge Radius, radius 4, edge radius 2: a ring from 2 to 4. | largest difference 2.3e-8 | yes |
| FX-CIRCLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-005 frame 0: Edge Radius, radius 2, edge radius 4: the same ring, the larger of the two outside. | largest difference 2.3e-8 | yes |
| FX-CIRCLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-006 frame 0: Thickness 2, radius 4: the ring inside the radius, from 2 to 4, FX-CIRCLE-004's. | largest difference 2.3e-8 | yes |
| FX-CIRCLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-007 frame 0: Thickness * Radius, thickness 50, radius 4: thickness 50 per cent of the radius, 2, FX-CIRCLE-006's. | largest difference 2.3e-8 | yes |
| FX-CIRCLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-008 frame 0: Thickness & Feather * Radius, thickness 50, outer feather 50, inner feather 25, radius 4: thickness 2, feathers 2 and 1, as Thickness with those. | largest difference 2.4e-8 | yes |
| FX-CIRCLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-009 frame 0: Thickness 2, radius 4, inner feather 2: the inner edge soft, the outer sharp. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-010 frame 0: Thickness 10, radius 4: thicker than the radius, a whole disk, FX-CIRCLE-002's. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-011 frame 0: Radius 4, Invert Circle on: white everywhere but the disk, clear in it. | largest difference 2.7e-8 | yes |
| FX-CIRCLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-012 frame 0: Radius 4, orange #ff8000 at opacity 50: an orange disk at half covering. | largest difference 6.0e-9 | yes |
| FX-CIRCLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-013 frame 0: Radius 4, normal: a white disk over the cel, the cel round it. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-014 frame 0: Radius 4, violet #6450a0, multiply: the cel darkened in the disk. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-015 frame 0: Radius 4, violet, screen: the cel lightened in the disk. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-016 frame 0: Radius 4, violet, add. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-017 frame 0: Radius 4, violet, overlay. | largest difference 2.0e-7 | yes |
| FX-CIRCLE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-018 frame 0: Radius 4, violet, soft light. | largest difference 2.0e-7 | yes |
| FX-CIRCLE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-019 frame 0: Radius 4, stencil alpha: the cel seen only through the disk. | largest difference 2.0e-7 | yes |
| FX-CIRCLE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-020 frame 0: Radius 4, normal at opacity 0: the cel exactly as it was. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-021 frame 0: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: the disk grows; frame 2 is radius 4. | largest difference 1.9e-8 | yes |
| FX-CIRCLE-021 frame 2: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: the disk grows; frame 2 is radius 4. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-021 frame 4: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: the disk grows; frame 2 is radius 4. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-022 frame 0: Radius 3, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the disk slides left, 1 pixel a frame. | largest difference 1.7e-8 | yes |
| FX-CIRCLE-022 frame 2: Radius 3, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the disk slides left, 1 pixel a frame. | largest difference 1.7e-8 | yes |
| FX-CIRCLE-022 frame 4: Radius 3, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the disk slides left, 1 pixel a frame. | largest difference 1.7e-8 | yes |
| FX-CIRCLE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-023 frame 0: FX-CIRCLE-013 moved three pixels right: the disk moves with the layer. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-024 frame 0: After a Motion Tile that grows the layer: the centre is the drawing's own, so the frame is FX-CIRCLE-002's. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-025 frame 0: FX-CIRCLE-012 with its colour in capitals, #FF8000: the same. | largest difference 6.0e-9 | yes |
| FX-CIRCLE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-026 frame 0: Radius 10, the centre at (-25, 50) per cent, outside the drawing: the left edge of the frame covered, the right clear. | largest difference 2.6e-8 | yes |
| FX-CIRCLE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-027 frame 0: Edge none with an inner feather of 3: the inner feather is not used, FX-CIRCLE-002's. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-028 frame 0: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, radius 4: held at 0, frame 4 clear everywhere. | largest difference 1.2e-8 | yes |
| FX-CIRCLE-028 frame 4: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, radius 4: held at 0, frame 4 clear everywhere. | largest difference 0.0e0 | yes |
| FX-CIRCLE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-029 frame 0: Radius 0, edge none: nothing covered, clear everywhere. | largest difference 0.0e0 | yes |
| FX-CIRCLE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CIRCLE-030 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-030 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-031 frame 0: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-031 frame 4: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-032 frame 0: Edge "ring", not one of its five words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-032 frame 4: Edge "ring", not one of its five words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-033 frame 0: Edge thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-033 frame 4: Edge thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-034 frame 0: Outer feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-034 frame 4: Outer feather 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-035 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-035 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-036 frame 0: Centre 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-036 frame 4: Centre 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-037 frame 0: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-037 frame 4: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-038 frame 0: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-038 frame 4: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-039 frame 0: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-039 frame 4: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CIRCLE-040 frame 0: Radius keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-040 frame 4: Radius keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CIRCLE-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_circle_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_circle_025.json is saved with its colour in small letters, its words and numbers as written | {"blending_mode":"none","center":[50,50],"color":"#ff8000","edge":"none","edge_thickness":10,"feather_inner":0,"feather_outer":0,"invert":"off","opacity":50,"radius":4} | yes |
| fx_circle_030.json is refused in a sentence | Circle's radius runs from 0 to 10000, and this is -1. | yes |
| fx_circle_031.json is refused in a sentence | Circle's radius runs from 0 to 10000, and this is 10001. | yes |
| fx_circle_032.json is refused in a sentence | Circle's edge is "none", "edge_radius", "thickness", "thickness_radius" or "thickness_feather_radius", and this is "ring". | yes |
| fx_circle_033.json is refused in a sentence | Circle's edge thickness runs from 0 to 10000, and this is -1. | yes |
| fx_circle_034.json is refused in a sentence | Circle's feather outer runs from 0 to 10000, and this is 10001. | yes |
| fx_circle_035.json is refused in a sentence | Circle's opacity runs from 0 to 100, and this is 101. | yes |
| fx_circle_036.json is refused in a sentence | Circle's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_circle_037.json is refused in a sentence | Circle's invert is "off" or "on", and this is "yes". | yes |
| fx_circle_038.json is refused in a sentence | Circle's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| fx_circle_039.json is refused in a sentence | Circle's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Circle with no `edge` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Circle with a radius in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Circle whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius -1 is refused with a sentence, and nothing changes | Circle's radius runs from 0 to 10000, and this is -1. | yes |
| edge "ring" is refused with a sentence, and nothing changes | Circle's edge is "none", "edge_radius", "thickness", "thickness_radius" or "thickness_feather_radius", and this is "ring". | yes |
| invert "yes" is refused with a sentence, and nothing changes | Circle's invert is "off" or "on", and this is "yes". | yes |
| blending mode "darken" is refused with a sentence, and nothing changes | Circle's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| colour "white" is refused with a sentence, and nothing changes | Circle's colour is written #rrggbb, and this is "white". | yes |
| radius keyed to 10001 is refused with a sentence, and nothing changes | Circle's radius runs from 0 to 10000, and this is 10001. | yes |
| radius 3, Thickness 1, feathers 2 and 1, inverted, orange at 60, Multiply is taken | taken | yes |
| centre keyed from 50, 50 to 25, 50 is taken | taken | yes |
| radius keyed from 2 to 6 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_circle_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_021.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_circle_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_circle_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_circle_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_circle_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle as it starts (a white disk of radius 75 in the middle, None) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 152042 pixels changed | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 0, Full | largest difference 1 of 255, 928 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 100, Full | largest difference 1 of 255, 1304 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 239, Full | largest difference 1 of 255, 1024 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 0, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 100, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 200, feather outer 40, orange at 70 per cent, Normal on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 55326 pixels changed | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 0, Full | largest difference 1 of 255, 2213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 100, Full | largest difference 1 of 255, 1159 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 239, Full | largest difference 1 of 255, 1158 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 100, Draft | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply on three layers, frame 239, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 20424 pixels changed | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 100, Full | largest difference 1 of 255, 1363 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 100, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness 30 at radius 120, centre 30, 70, Overlay on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 36065 pixels changed | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 0, Full | largest difference 1 of 255, 2411 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 100, Full | largest difference 1 of 255, 1222 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 239, Full | largest difference 1 of 255, 1351 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 0, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 100, Draft | largest difference 1 of 255, 28 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness * Radius 25 at radius 160, Soft Light on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 51019 pixels changed | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 0, Full | largest difference 1 of 255, 2096 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 100, Full | largest difference 1 of 255, 1252 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 239, Full | largest difference 1 of 255, 1404 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle Thickness & Feather * Radius 20, feathers 10 and 5, Screen on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1959831 pixels changed | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 0, Full | largest difference 1 of 255, 3755 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 100, Full | largest difference 1 of 255, 829 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 239, Full | largest difference 1 of 255, 1663 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 220 inverted, feather 60, Add at 50 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 263854 pixels changed | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 0, Full | largest difference 1 of 255, 374 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 100, Full | largest difference 1 of 255, 879 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 239, Full | largest difference 1 of 255, 487 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 0, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Circle radius 250 inverted, feather 80, Stencil Alpha on three layers, frame 239, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-414 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a white disk of radius 75 in the middle in place of the street, the rest clear; draws cleanly | [], 129600 pixels changed | yes |
| 3_normal_feathered.png, Normal, an orange disk at 60 per cent, radius 140 with a soft edge 50 wide, over the street; draws cleanly | [], 77657 pixels changed | yes |
| 4_ring_multiply.png, Thickness 25 at radius 120, violet, Multiply: a dark ring on the street; draws cleanly | [], 20850 pixels changed | yes |
| 5_vignette.png, inverted, black at 80 per cent, feather 120: a vignette darkening the corners; draws cleanly | [], 97480 pixels changed | yes |
| 6_stencil.png, Stencil Alpha, radius 130, feather 40: the street seen only through the disk; draws cleanly | [], 91528 pixels changed | yes |

## Result

268 of 268 checks pass.
