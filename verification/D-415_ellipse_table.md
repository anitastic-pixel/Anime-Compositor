# D-415: Ellipse

B-294, after After Effects' Ellipse: the outline of the ellipse Width by Height about Center, a share of the drawing's own size. A pixel's distance from the outline is taken to first order, d = |k - 1| k / g with k the ellipse's level and g its gradient (exact for a circle; min(a, b) at the centre), and the outline drawn as Beam's line (D-207): Thickness wide, Softness per cent of it a ramp at least a pixel wide, Inside Color along its middle and Outside Color at its edges, over the layer (Composite On Original on) or alone. The layer never grows. Every expected pixel is `Fixtures/ellipse/expected_ellipse.json`, written by `tools/ellipse_reference.py` before this code existed and printed in document 25 as FX-ELLIPSE-001 to 027. Tolerance 2e-5.

## FX-ELLIPSE-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ELLIPSE-001 frame 0: The settings as they start: centre in the middle, 200 by 200, thickness 8, softness 50, white inside, blue outside, composited: the outline lies far outside the small frame, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-002 frame 0: 12 by 8, thickness 2, softness 0: a sharp outline 2 pixels thick round the middle over the cel, white along its middle, blue at its edges. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-003 frame 0: FX-ELLIPSE-002 with Composite On Original off: the outline alone, the rest clear. | largest difference 2.5e-8 | yes |
| FX-ELLIPSE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-004 frame 0: FX-ELLIPSE-003 at softness 100: the whole width of the outline ramps. | largest difference 3.0e-8 | yes |
| FX-ELLIPSE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-005 frame 0: 10 by 10, thickness 2, alone: a circle of radius 5, the distance exact. | largest difference 2.8e-8 | yes |
| FX-ELLIPSE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-006 frame 0: Thickness 4, softness 50, alone. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-007 frame 0: Orange inside, violet outside, thickness 4, alone: orange along the middle, violet at the edges. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-008 frame 0: Thickness 0: nothing drawn, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-009 frame 0: 8 by 6 alone, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the outline slides left, 1 pixel a frame. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-009 frame 2: 8 by 6 alone, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the outline slides left, 1 pixel a frame. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-009 frame 4: 8 by 6 alone, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame 4, linear: the outline slides left, 1 pixel a frame. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-010 frame 0: Width keyed from 8 at frame 0 to 16 at frame 4, linear, alone: frame 2 is 12 across. | largest difference 2.7e-8 | yes |
| FX-ELLIPSE-010 frame 2: Width keyed from 8 at frame 0 to 16 at frame 4, linear, alone: frame 2 is 12 across. | largest difference 2.5e-8 | yes |
| FX-ELLIPSE-010 frame 4: Width keyed from 8 at frame 0 to 16 at frame 4, linear, alone: frame 2 is 12 across. | largest difference 2.8e-8 | yes |
| FX-ELLIPSE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-011 frame 0: FX-ELLIPSE-002 moved three pixels right: the outline moves with the layer. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-012 frame 0: After a Motion Tile that grows the layer: the centre is the drawing's own, so the frame is FX-ELLIPSE-002's. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-013 frame 0: FX-ELLIPSE-007 with its colours in capitals, #FF8000 and #6450A0: the same. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-014 frame 0: 20 by 10, the centre at (-25, 50) per cent, outside the drawing, alone: the outline's right end, at x 6, reaches into the frame's left side. | largest difference 2.7e-8 | yes |
| FX-ELLIPSE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-015 frame 0: Thickness keyed from 2 at frame 0 to 0 at frame 4 past its end by an ease: held at 0, frame 4 the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-015 frame 4: Thickness keyed from 2 at frame 0 to 0 at frame 4 past its end by an ease: held at 0, frame 4 the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-016 frame 0: 4 by 10, tall, alone: the outline narrow across, long down. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-017 frame 0: Thickness 1, softness 0, alone: a hairline, each pixel covered by how much of it lies within half a pixel of the outline. | largest difference 2.9e-8 | yes |
| FX-ELLIPSE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-018 frame 0: Width 1, height 1, thickness 2, alone: a dot at the middle. | largest difference 2.1e-8 | yes |
| FX-ELLIPSE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ELLIPSE-019 frame 0: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-019 frame 4: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-020 frame 0: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-020 frame 4: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-021 frame 0: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-021 frame 4: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-022 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-022 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-023 frame 0: Centre 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-023 frame 4: Centre 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-024 frame 0: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-024 frame 4: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-025 frame 0: Inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-025 frame 4: Inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-026 frame 0: Outside colour "red", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-026 frame 4: Outside colour "red", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ELLIPSE-027 frame 0: Width keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-027 frame 4: Width keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ELLIPSE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ellipse_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ellipse_013.json is saved with its colours in small letters, its words and numbers as written | {"center":[50,50],"composite":"off","height":8,"inside_color":"#ff8000","outside_color":"#6450a0","softness":0,"thickness":4,"width":12} | yes |
| fx_ellipse_019.json is refused in a sentence | Ellipse's width runs from 1 to 10000, and this is 0. | yes |
| fx_ellipse_020.json is refused in a sentence | Ellipse's height runs from 1 to 10000, and this is 10001. | yes |
| fx_ellipse_021.json is refused in a sentence | Ellipse's thickness runs from 0 to 10000, and this is -1. | yes |
| fx_ellipse_022.json is refused in a sentence | Ellipse's softness runs from 0 to 100, and this is 101. | yes |
| fx_ellipse_023.json is refused in a sentence | Ellipse's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_ellipse_024.json is refused in a sentence | Ellipse's composite is "on" or "off", and this is "yes". | yes |
| fx_ellipse_025.json is refused in a sentence | Ellipse's inside colour is written #rrggbb, and this is "#12345". | yes |
| fx_ellipse_026.json is refused in a sentence | Ellipse's outside colour is written #rrggbb, and this is "red". | yes |
| a file with an Ellipse with no `composite` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Ellipse with a width in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Ellipse whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 0 is refused with a sentence, and nothing changes | Ellipse's width runs from 1 to 10000, and this is 0. | yes |
| composite "yes" is refused with a sentence, and nothing changes | Ellipse's composite is "on" or "off", and this is "yes". | yes |
| inside colour "white" is refused with a sentence, and nothing changes | Ellipse's inside colour is written #rrggbb, and this is "white". | yes |
| width keyed to 10001 is refused with a sentence, and nothing changes | Ellipse's width runs from 1 to 10000, and this is 10001. | yes |
| 12 by 8, thickness 3, softness 40, orange inside, violet outside, alone is taken | taken | yes |
| centre keyed from 50, 50 to 25, 50 is taken | taken | yes |
| width keyed from 8 to 16 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ellipse_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ellipse_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ellipse_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_ellipse_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_ellipse_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 7541 pixels changed | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 0, Full | largest difference 1 of 255, 3642 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 100, Full | largest difference 1 of 255, 1391 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 239, Full | largest difference 1 of 255, 1694 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse as it starts (an outline 200 across in the middle, over the layer) on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 114182 pixels changed | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 0, Full | largest difference 1 of 255, 3771 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 100, Full | largest difference 1 of 255, 1326 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 239, Full | largest difference 1 of 255, 1743 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 100, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 900 by 400, thickness 30, softness 80 on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 0, Full | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 100, Full | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 239, Full | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 0, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 100, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 600 by 600 alone, thickness 20 on three layers, frame 239, Draft | largest difference 1 of 255, 14 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 3413 pixels changed | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 0, Full | largest difference 1 of 255, 3739 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 100, Full | largest difference 1 of 255, 1411 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 239, Full | largest difference 1 of 255, 1756 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 130 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 200 by 800, a hairline 1 thick, softness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 99466 pixels changed | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 0, Full | largest difference 1 of 255, 3785 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 100, Full | largest difference 1 of 255, 1454 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 239, Full | largest difference 1 of 255, 1776 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 100, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 500 by 300 at 20, 70, thickness 60, orange and violet on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 24 pixels changed | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 4000 by 1500, thickness 120, softness 100, past the frame on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 51067 pixels changed | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 0, Full | largest difference 1 of 255, 3782 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 100, Full | largest difference 1 of 255, 1419 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 1200 by 900 centred outside at 120, 50, thickness 40 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 0, Full | largest difference 1 of 255, 646 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 100, Full | largest difference 1 of 255, 670 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 239, Full | largest difference 1 of 255, 718 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 0, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 100, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ellipse 300 by 150 alone, thickness 200, softness 100, thicker than it is tall on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-415 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a glowing outline 200 across in the middle, white along its middle and blue at its edges, over the street; draws cleanly | [], 7521 pixels changed | yes |
| 3_neon_alone.png, a wide orange neon oval alone, the rest clear; draws cleanly | [], 129600 pixels changed | yes |
| 4_tall_violet.png, a tall thin oval, white fading to violet, over the street; draws cleanly | [], 4202 pixels changed | yes |
| 5_halo.png, a thick soft halo centred left of the middle, over the street; draws cleanly | [], 64939 pixels changed | yes |

## Result

209 of 209 checks pass.
