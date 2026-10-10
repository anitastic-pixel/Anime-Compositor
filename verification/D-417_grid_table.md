# D-417: Grid

B-296, after After Effects' Grid: lines Border pixels thick along the edges of cells w by h from Anchor, a share of the drawing's own size (the cell as Checkerboard's: from Anchor to Corner for Corner Point, Width both ways for Width Slider, Width by Height for Width & Height Sliders). On each axis a pixel is covered by the share of a box max(feather, 1) pixels wide about it that falls inside the two nearest lines, held to 1; the axes join as gx + gy - gx gy, turned over by Invert Grid. The grid, Color times the covering times Opacity, replaces the layer for None or is laid on it by document 21's layer blend (Normal, Add, Multiply, Screen, Overlay, Soft Light, Stencil Alpha). The layer never grows. Every expected pixel is `Fixtures/grid/expected_grid.json`, written by `tools/grid_reference.py` before this code existed and printed in document 25 as FX-GRID-001 to 038. Tolerance 2e-5.

## FX-GRID-001 to 038 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GRID-001 frame 0: The settings as they start: anchor in the middle, Corner Point at (60, 60) per cent, cells 1.6 by 1 pixels in this small drawing, border 2, white, blending mode none: the lines are thicker than the cells, so the whole layer is the grid's white and the cel is gone. | largest difference 0.0e0 | yes |
| FX-GRID-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-002 frame 0: Width Slider 4, border 1, the anchor at (8, 5): the lines run along pixel edges, so they are two half-covered pixels wide, and three quarters where they cross. | largest difference 0.0e0 | yes |
| FX-GRID-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-003 frame 0: Width Slider 4, border 1, the anchor at (8.5, 5.5): the lines run through pixel centres, so columns 0, 4, 8, 12 and rows 1, 5, 9 are white, the rest clear. | largest difference 0.0e0 | yes |
| FX-GRID-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-004 frame 0: Width & Height Sliders 4 by 2, border 1, the anchor at (8, 5). | largest difference 0.0e0 | yes |
| FX-GRID-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-005 frame 0: Corner Point with the corner at (75, 70) per cent, (12, 7) pixels: cells 4 by 2, FX-GRID-004's. | largest difference 0.0e0 | yes |
| FX-GRID-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-006 frame 0: Border 0: no grid at all, the layer clear. | largest difference 0.0e0 | yes |
| FX-GRID-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-007 frame 0: Width 4, border 2, feather width 2: the upright lines ramp over 2 pixels each side, the level ones stay sharp. | largest difference 0.0e0 | yes |
| FX-GRID-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-008 frame 0: Width 5, border 3, both feathers 4: soft lines both ways. | largest difference 0.0e0 | yes |
| FX-GRID-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-009 frame 0: FX-GRID-003 with Invert Grid: the cells white, the lines clear. | largest difference 0.0e0 | yes |
| FX-GRID-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-010 frame 0: FX-GRID-003 in orange #ff8000 at opacity 50. | largest difference 4.1e-10 | yes |
| FX-GRID-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-011 frame 0: FX-GRID-003, normal: white lines over the cel. | largest difference 1.9e-7 | yes |
| FX-GRID-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-012 frame 0: FX-GRID-003 in violet #6450a0, multiply: the cel darkened and tinted on the lines, as it was elsewhere. | largest difference 1.9e-7 | yes |
| FX-GRID-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-013 frame 0: The same, screen. | largest difference 1.9e-7 | yes |
| FX-GRID-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-014 frame 0: The same, add. | largest difference 1.9e-7 | yes |
| FX-GRID-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-015 frame 0: The same, overlay. | largest difference 2.0e-7 | yes |
| FX-GRID-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-016 frame 0: The same, soft light. | largest difference 2.0e-7 | yes |
| FX-GRID-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-017 frame 0: FX-GRID-003, stencil alpha: the grid as a mask, the cel kept only on the lines. | largest difference 1.9e-7 | yes |
| FX-GRID-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-018 frame 0: FX-GRID-003, normal at opacity 0: the cel exactly as it was. | largest difference 1.9e-7 | yes |
| FX-GRID-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-019 frame 0: Width 4, border 1, the anchor keyed from (53.125, 55) at frame 0 to (78.125, 55) at frame 4, linear: the lines slide right 1 pixel a frame; frame 4, moved one cell, is frame 0. | largest difference 0.0e0 | yes |
| FX-GRID-019 frame 2: Width 4, border 1, the anchor keyed from (53.125, 55) at frame 0 to (78.125, 55) at frame 4, linear: the lines slide right 1 pixel a frame; frame 4, moved one cell, is frame 0. | largest difference 0.0e0 | yes |
| FX-GRID-019 frame 4: Width 4, border 1, the anchor keyed from (53.125, 55) at frame 0 to (78.125, 55) at frame 4, linear: the lines slide right 1 pixel a frame; frame 4, moved one cell, is frame 0. | largest difference 0.0e0 | yes |
| FX-GRID-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-020 frame 0: Width 4, the border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is border 2. | largest difference 0.0e0 | yes |
| FX-GRID-020 frame 2: Width 4, the border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is border 2. | largest difference 0.0e0 | yes |
| FX-GRID-020 frame 4: Width 4, the border keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is border 2. | largest difference 0.0e0 | yes |
| FX-GRID-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-021 frame 0: FX-GRID-011 moved three pixels right: the grid moves with the layer. | largest difference 1.9e-7 | yes |
| FX-GRID-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-022 frame 0: After a Motion Tile that grows the layer: the anchor is the drawing's own, so the frame is FX-GRID-003's. | largest difference 0.0e0 | yes |
| FX-GRID-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-023 frame 0: FX-GRID-010 with its colour in capitals, #FF8000: the same. | largest difference 4.1e-10 | yes |
| FX-GRID-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-024 frame 0: Width 2, border 3: the lines wider than the cells, every pixel white. | largest difference 0.0e0 | yes |
| FX-GRID-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-025 frame 0: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, FX-GRID-003, none: held at 0, frame 4 is clear everywhere. | largest difference 0.0e0 | yes |
| FX-GRID-025 frame 4: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, FX-GRID-003, none: held at 0, frame 4 is clear everywhere. | largest difference 0.0e0 | yes |
| FX-GRID-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-026 frame 0: Invert Grid with stencil alpha: the cel kept only in the cells. | largest difference 1.9e-7 | yes |
| FX-GRID-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-027 frame 0: Width 3, border 0.5, the anchor at (-100, -100) per cent, outside the drawing: thin lines still across the whole layer, each pixel covered by the share of it a line crosses. | largest difference 1.8e-15 | yes |
| FX-GRID-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GRID-028 frame 0: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-028 frame 4: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-029 frame 0: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-029 frame 4: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-030 frame 0: Border -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-030 frame 4: Border -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-031 frame 0: Feather height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-031 frame 4: Feather height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-032 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-032 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-033 frame 0: Anchor 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-033 frame 4: Anchor 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-034 frame 0: Size From "corner", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-034 frame 4: Size From "corner", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-035 frame 0: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-035 frame 4: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-036 frame 0: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-036 frame 4: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-037 frame 0: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-037 frame 4: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GRID-038 frame 0: Border keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-038 frame 4: Border keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GRID-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grid_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_grid_023.json is saved with its colour in small letters, its words and numbers as written | {"anchor":[53.125,55],"blending_mode":"none","border":1,"color":"#ff8000","corner":[60,60],"feather_height":0,"feather_width":0,"height":64,"invert":"off","opacity":50,"size_from":"width_slider","width":4} | yes |
| fx_grid_028.json is refused in a sentence | Grid's width runs from 1 to 10000, and this is 0. | yes |
| fx_grid_029.json is refused in a sentence | Grid's height runs from 1 to 10000, and this is 10001. | yes |
| fx_grid_030.json is refused in a sentence | Grid's border runs from 0 to 10000, and this is -1. | yes |
| fx_grid_031.json is refused in a sentence | Grid's feather height runs from 0 to 10000, and this is 10001. | yes |
| fx_grid_032.json is refused in a sentence | Grid's opacity runs from 0 to 100, and this is 101. | yes |
| fx_grid_033.json is refused in a sentence | Grid's anchor runs from -1000 to 1000, and this is 1001. | yes |
| fx_grid_034.json is refused in a sentence | Grid's size from is "corner_point", "width_slider" or "width_and_height_sliders", and this is "corner". | yes |
| fx_grid_035.json is refused in a sentence | Grid's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| fx_grid_036.json is refused in a sentence | Grid's colour is written #rrggbb, and this is "#12345". | yes |
| fx_grid_037.json is refused in a sentence | Grid's invert is "off" or "on", and this is "yes". | yes |
| a file with a Grid with no `border` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Grid with a border in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Grid whose anchor is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| border -1 is refused with a sentence, and nothing changes | Grid's border runs from 0 to 10000, and this is -1. | yes |
| size from "corner" is refused with a sentence, and nothing changes | Grid's size from is "corner_point", "width_slider" or "width_and_height_sliders", and this is "corner". | yes |
| invert "yes" is refused with a sentence, and nothing changes | Grid's invert is "off" or "on", and this is "yes". | yes |
| blending mode "darken" is refused with a sentence, and nothing changes | Grid's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| colour "white" is refused with a sentence, and nothing changes | Grid's colour is written #rrggbb, and this is "white". | yes |
| border keyed to 10001 is refused with a sentence, and nothing changes | Grid's border runs from 0 to 10000, and this is 10001. | yes |
| Width Slider 4, border 1, feathers 2 and 1, inverted, orange at 60, Multiply is taken | taken | yes |
| anchor keyed from 50, 50 to 25, 50 is taken | taken | yes |
| border keyed from 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grid_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_grid_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_grid_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_grid_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_grid_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid as it starts (cells from the middle to 60, 60 per cent, border 2, white, None) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 163216 pixels changed | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 0, Full | largest difference 1 of 255, 3566 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 239, Full | largest difference 1 of 255, 1723 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 100, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 100, border 4, orange at 70 per cent, Normal on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 752384 pixels changed | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 0, Full | largest difference 1 of 255, 3163 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 100, Full | largest difference 1 of 255, 1388 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 239, Full | largest difference 1 of 255, 1668 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 0, Draft | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width & Height 120 by 80, border 10, feathers 6 and 12, violet, Multiply on three layers, frame 239, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1983168 pixels changed | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 0, Full | largest difference 1 of 255, 226 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 100, Full | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 239, Full | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 0, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 100, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid anchor 30, 70, Width Slider 90, border 3, inverted, Overlay on three layers, frame 239, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 949742 pixels changed | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 0, Full | largest difference 1 of 255, 2104 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 100, Full | largest difference 1 of 255, 1091 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 239, Full | largest difference 1 of 255, 1354 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 150, border 20, feathers 20, Soft Light on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 237600 pixels changed | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 0, Full | largest difference 1 of 255, 3462 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 100, Full | largest difference 1 of 255, 1294 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 239, Full | largest difference 1 of 255, 1615 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Corner Point 55, 60, border 5, violet, Screen on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 128040 pixels changed | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 0, Full | largest difference 1 of 255, 3548 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 100, Full | largest difference 1 of 255, 1398 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 239, Full | largest difference 1 of 255, 1693 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 107 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 64, border 1.5, Add at 50 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1705183 pixels changed | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 0, Full | largest difference 1 of 255, 630 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 100, Full | largest difference 1 of 255, 417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 239, Full | largest difference 1 of 255, 478 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 0, Draft | largest difference 1 of 255, 19 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 100, Draft | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Grid Width Slider 200, border 30, feathers 10, Stencil Alpha on three layers, frame 239, Draft | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-417 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: thin white lines over a clear layer in place of the street, cells a tenth of the picture across and down; draws cleanly | [], 129600 pixels changed | yes |
| 3_normal_lines.png, Normal, Width Slider 40, border 3, white at 80 per cent: square graph-paper lines over the street; draws cleanly | [], 25056 pixels changed | yes |
| 4_feathered_multiply.png, Width & Height 60 by 30, border 6, feathers 6, violet, Multiply: soft dark lines on the street, the cells twice as wide as tall; draws cleanly | [], 67392 pixels changed | yes |
| 5_inverted_dark.png, inverted, Width Slider 48, border 4, black at 50 per cent, Normal: the cells darkened, the lines left as the street; draws cleanly | [], 110000 pixels changed | yes |
| 6_stencil.png, Stencil Alpha, Width Slider 32, border 8: the street seen only through the lines, the cells clear; draws cleanly | [], 71280 pixels changed | yes |

## Result

260 of 260 checks pass.
