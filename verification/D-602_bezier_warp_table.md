# D-602: Bezier Warp

B-481: `core.bezier_warp` takes After Effects' Bezier Warp controls (the four vertices, the eight tangents, Quality) and bends the drawing to the outline they make, through D-601's forward mesh. Every expected pixel is `Fixtures/bezier_warp/expected_bezier_warp.json`, written by `tools/bezier_warp_reference.py` before this code existed and printed in document 25 as FX-BEZIER-001 to 017. Tolerance 2e-5.

## FX-BEZIER-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BEZIER-001 frame 0: The settings as they start: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BEZIER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-002 frame 0: The top left vertex pulled in to (25, 25), the tangents left where they start: the top and left edges bend in to meet it, and the corner it left is clear. | largest difference 2.3e-7 | yes |
| FX-BEZIER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-003 frame 0: Both top tangents pulled down to 60 per cent: the top edge sags into the drawing in a curve, the band of rows above it clear, deepest in the middle. | largest difference 2.5e-7 | yes |
| FX-BEZIER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-004 frame 0: Both right tangents pulled in to 50 per cent across: the right edge pinched in to a curve, the right middle columns clear. | largest difference 2.5e-7 | yes |
| FX-BEZIER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-005 frame 0: FX-BEZIER-003 at Quality 1: the same sag cut into 4 by 4 cells, so its curve is a coarser chain of straight pieces and the frame differs. | largest difference 2.5e-7 | yes |
| FX-BEZIER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-006 frame 0: An S: the top edge's tangents at 50 and -50 per cent down, the bottom's at 150 and 50: both edges wave the same way, the stripes between bent into an S. | largest difference 2.5e-7 | yes |
| FX-BEZIER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-007 frame 0: Every point moved into the square from 25 to 75 per cent, the tangents still at the thirds: no curve, the drawing at half its size in the middle, as an even squeeze draws it. | largest difference 1.9e-7 | yes |
| FX-BEZIER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-008 frame 0: Every point mirrored left for right in place: the outline is the same square, run the other way, so the drawing is drawn mirrored. | largest difference 1.9e-7 | yes |
| FX-BEZIER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-009 frame 0: The top left vertex keyed from (0, 0) at frame 0 to (25, 25) at frame 4, linear: frame 0 the drawing, frame 4 FX-BEZIER-002, frame 2 half way. | largest difference 1.9e-7 | yes |
| FX-BEZIER-009 frame 2: The top left vertex keyed from (0, 0) at frame 0 to (25, 25) at frame 4, linear: frame 0 the drawing, frame 4 FX-BEZIER-002, frame 2 half way. | largest difference 3.1e-7 | yes |
| FX-BEZIER-009 frame 4: The top left vertex keyed from (0, 0) at frame 0 to (25, 25) at frame 4, linear: frame 0 the drawing, frame 4 FX-BEZIER-002, frame 2 half way. | largest difference 2.3e-7 | yes |
| FX-BEZIER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-010 frame 0: FX-BEZIER-002 with the layer moved three pixels right: the bent drawing moves with it. | largest difference 2.3e-7 | yes |
| FX-BEZIER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-011 frame 0: The bottom right vertex dragged past the top left one to (-20, -20): the outline folds over itself; where it overlaps, the part later in the grid is drawn on top. | largest difference 2.5e-7 | yes |
| FX-BEZIER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-012 frame 0: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with the sag: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-BEZIER-012 frame 2: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with the sag: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-BEZIER-012 frame 4: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with the sag: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-BEZIER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-013 frame 0: The left top tangent flung out to (300, -200): the left edge loops far out past the drawing and back, and what is bent past the edges is cut off. | largest difference 2.5e-7 | yes |
| FX-BEZIER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEZIER-014 frame 0: Quality 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-014 frame 4: Quality 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEZIER-015 frame 0: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-015 frame 4: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEZIER-016 frame 0: The top left vertex at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-016 frame 4: The top left vertex at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEZIER-017 frame 0: The bottom left tangent keyed to (100, 600) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-017 frame 4: The bottom left tangent keyed to (100, 600) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEZIER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bezier_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bezier_006.json is saved with its thirteen settings, each point two numbers | {"bottom_left_tangent":[33.333333333333336,150],"bottom_right_tangent":[66.66666666666667,50],"bottom_right_vertex":[100,100],"left_bottom_tangent":[0,66.66666666666667],"left_bottom_vertex":[0,100],"left_top_tangent":[0,33.333333333333336],"quality":8,"right_bottom_tangent":[100,66.66666666666667],"right_top_tangent":[100,33.333333333333336],"right_top_vertex":[100,0],"top_left_tangent":[33.333333333333336,50],"top_left_vertex":[0,0],"top_right_tangent":[66.66666666666667,-50]} | yes |
| fx_bezier_014.json is refused in a sentence naming Bezier Warp and its quality | Bezier Warp's quality runs from 1 to 10, and this is 11. | yes |
| fx_bezier_015.json is refused in a sentence naming Bezier Warp and its quality | Bezier Warp's quality runs from 1 to 10, and this is 0. | yes |
| fx_bezier_016.json is refused in a sentence naming Bezier Warp and its top left vertex | Bezier Warp's top left vertex runs from -400 to 500, and this is -401. | yes |
| a file with a Bezier Warp whose quality is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bezier Warp without its top left vertex is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bezier Warp whose right top vertex is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| quality 0.5 is refused with a sentence, and nothing changes | Bezier Warp's quality runs from 1 to 10, and this is 0.5. | yes |
| bottom right vertex at (100, 501) is refused with a sentence, and nothing changes | Bezier Warp's bottom right vertex runs from -400 to 500, and this is 501. | yes |
| top left tangent keyed to (-500, 0) is refused with a sentence, and nothing changes | Bezier Warp's top left tangent runs from -400 to 500, and this is -500. | yes |
| top tangents at 60 per cent down, quality 3 is taken | taken | yes |
| left bottom vertex keyed from (0, 100) to (20, 80) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bezier_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bezier_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bezier_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bezier_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bezier_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bezier_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_bezier_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2021755 pixels changed | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 0, Full | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 100, Full | largest difference 1 of 255, 560 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 239, Full | largest difference 1 of 255, 122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 0, Draft | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp the top edge sagging (top tangents at 60 per cent) on three layers, frame 239, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1992406 pixels changed | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 0, Full | largest difference 1 of 255, 4079 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 100, Full | largest difference 1 of 255, 939 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 239, Full | largest difference 1 of 255, 849 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 0, Draft | largest difference 1 of 255, 152 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp an S at quality 2 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2004213 pixels changed | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 0, Full | largest difference 1 of 255, 610 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 100, Full | largest difference 1 of 255, 667 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 239, Full | largest difference 1 of 255, 770 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 0, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bezier Warp folded (bottom right vertex at (-20, -20)) on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-602 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect, every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| as added, every point where it starts: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_sag.png, both top tangents pulled down to 60 per cent: the top edge sags in a curve, clear above it, deepest in the middle and nothing cleared at the corners; draws cleanly | [], clear rows from the top at the left, a quarter, the middle: 0, 91, 121 | yes |
| 3_half_size.png, every point moved into the square from 25 to 75 per cent: the street at half size in the middle, a quarter of the frame covered, the rest clear; draws cleanly | [], 32640 of 129600 pixels covered | yes |
| 4_s_curve.png, the top edge's tangents at 50 and -50 per cent down and the bottom's at 150 and 50: the street waved into an S, clear above the top edge's dip on the left and none on the right; draws cleanly | [], clear rows from the top at a quarter and three quarters across: 38, 0 | yes |
| 5_corner_pulled.png and 5b_corner_pulled_quality_1.png, the top left vertex pulled to (25, 25): the corner it left is clear and the far corner kept (two pixels in, past the edge's smoothing); at Quality 1 the bend is coarser, so the two differ; both draw cleanly | [] [], 40838 pixels differ between the qualities | yes |

## Result

121 of 121 checks pass.
