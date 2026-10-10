# D-413: Checkerboard

B-292, after After Effects' Checkerboard: cells Width by Width (Width Slider), Width by Height (Width & Height Sliders) or the anchor-to-corner rectangle (Corner Point) from Anchor, a share of the drawing's own size; on each axis the distance to the nearer cell edge gives a straight ramp max(feather, 1) pixels wide, the covering (1 + fx fy) / 2, the cell from the anchor coloured. The pattern, Color times the covering times Opacity, replaces the layer for None or is laid on it by document 21's layer blend (Normal, Add, Multiply, Screen, Overlay, Soft Light, Stencil Alpha). The layer never grows. Every expected pixel is `Fixtures/checkerboard/expected_checkerboard.json`, written by `tools/checkerboard_reference.py` before this code existed and printed in document 25 as FX-CHECK-001 to 033. Tolerance 2e-5.

## FX-CHECK-001 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHECK-001 frame 0: The settings as they start: anchor in the middle, (50, 50) per cent, Width Slider 64, white, opacity 100, blending mode none: the cell that starts at the anchor runs off the bottom-right, so the top-left and bottom-right quarters are white and the other two clear; the cel itself is gone. | largest difference 0.0e0 | yes |
| FX-CHECK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-002 frame 0: Width 4: white squares 4 pixels across in a checker, the anchor (8, 5) on a corner of them, the others clear. | largest difference 0.0e0 | yes |
| FX-CHECK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-003 frame 0: Width 4 with the anchor at (53.125, 55) per cent, (8.5, 5.5) pixels: the edges run through pixel centres, so the pixels on them are half white. | largest difference 0.0e0 | yes |
| FX-CHECK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-004 frame 0: Width & Height Sliders, width 4 and height 2: rectangles 4 across and 2 down. | largest difference 0.0e0 | yes |
| FX-CHECK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-005 frame 0: Corner Point, the corner at (75, 70) per cent, (12, 7) pixels: rectangles 4 by 2, FX-CHECK-004's. | largest difference 0.0e0 | yes |
| FX-CHECK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-006 frame 0: Corner Point with the corner on the anchor: each cell held at 1 pixel, a checker of single pixels. | largest difference 0.0e0 | yes |
| FX-CHECK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-007 frame 0: Width 4, feather width 2: the upright edges ramp over 2 pixels, the level ones stay sharp. | largest difference 0.0e0 | yes |
| FX-CHECK-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-008 frame 0: Width 4, both feathers 4: no pixel wholly white or wholly clear, a soft weave. | largest difference 0.0e0 | yes |
| FX-CHECK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-009 frame 0: Width 4, orange #ff8000 at opacity 50: orange squares at half covering. | largest difference 4.1e-10 | yes |
| FX-CHECK-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-010 frame 0: Width 4, normal: white squares over the cel, the cel showing in the clear ones. | largest difference 1.9e-7 | yes |
| FX-CHECK-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-011 frame 0: Width 4, violet #6450a0, multiply: the cel darkened and tinted in the squares, as it was in the others. | largest difference 1.9e-7 | yes |
| FX-CHECK-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-012 frame 0: Width 4, violet, screen: the cel lightened in the squares. | largest difference 1.9e-7 | yes |
| FX-CHECK-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-013 frame 0: Width 4, violet, add: the violet added in the squares. | largest difference 1.9e-7 | yes |
| FX-CHECK-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-014 frame 0: Width 4, violet, overlay. | largest difference 2.0e-7 | yes |
| FX-CHECK-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-015 frame 0: Width 4, violet, soft light. | largest difference 2.0e-7 | yes |
| FX-CHECK-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-016 frame 0: Width 4, stencil alpha: the cel shows only through the squares. | largest difference 1.9e-7 | yes |
| FX-CHECK-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-017 frame 0: Width 4, normal at opacity 0: the cel exactly as it was. | largest difference 1.9e-7 | yes |
| FX-CHECK-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-018 frame 0: Width 4, the anchor keyed from (50, 50) at frame 0 to (75, 50) at frame 4, linear: the squares slide right, 1 pixel a frame; frame 4, moved one square, is the clear and white swapped. | largest difference 0.0e0 | yes |
| FX-CHECK-018 frame 2: Width 4, the anchor keyed from (50, 50) at frame 0 to (75, 50) at frame 4, linear: the squares slide right, 1 pixel a frame; frame 4, moved one square, is the clear and white swapped. | largest difference 0.0e0 | yes |
| FX-CHECK-018 frame 4: Width 4, the anchor keyed from (50, 50) at frame 0 to (75, 50) at frame 4, linear: the squares slide right, 1 pixel a frame; frame 4, moved one square, is the clear and white swapped. | largest difference 0.0e0 | yes |
| FX-CHECK-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-019 frame 0: Width keyed from 2 at frame 0 to 8 at frame 4, linear: the squares grow; frame 2 is width 5. | largest difference 0.0e0 | yes |
| FX-CHECK-019 frame 2: Width keyed from 2 at frame 0 to 8 at frame 4, linear: the squares grow; frame 2 is width 5. | largest difference 2.2e-16 | yes |
| FX-CHECK-019 frame 4: Width keyed from 2 at frame 0 to 8 at frame 4, linear: the squares grow; frame 2 is width 5. | largest difference 0.0e0 | yes |
| FX-CHECK-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-020 frame 0: FX-CHECK-010 moved three pixels right: the squares move with the layer. | largest difference 1.9e-7 | yes |
| FX-CHECK-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-021 frame 0: After a Motion Tile that grows the layer: the anchor is the drawing's own, so the frame is FX-CHECK-002's. | largest difference 0.0e0 | yes |
| FX-CHECK-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-022 frame 0: FX-CHECK-009 with its colour in capitals, #FF8000: the same. | largest difference 4.1e-10 | yes |
| FX-CHECK-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-023 frame 0: Width 3, the anchor at (-100, -100) per cent, outside the drawing: the checker still lies across the whole layer. | largest difference 2.2e-15 | yes |
| FX-CHECK-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-024 frame 0: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, width 4, none: held at 0, frame 4 is clear everywhere. | largest difference 0.0e0 | yes |
| FX-CHECK-024 frame 4: Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, width 4, none: held at 0, frame 4 is clear everywhere. | largest difference 0.0e0 | yes |
| FX-CHECK-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHECK-025 frame 0: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-025 frame 4: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-026 frame 0: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-026 frame 4: Height 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-027 frame 0: Feather width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-027 frame 4: Feather width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-028 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-028 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-029 frame 0: Anchor 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-029 frame 4: Anchor 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-030 frame 0: Size From "corner", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-030 frame 4: Size From "corner", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-031 frame 0: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-031 frame 4: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-032 frame 0: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-032 frame 4: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHECK-033 frame 0: Width keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-033 frame 4: Width keyed to 10001 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHECK-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_check_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_check_022.json is saved with its colour in small letters, its words and numbers as written | {"anchor":[50,50],"blending_mode":"none","color":"#ff8000","corner":[60,60],"feather_height":0,"feather_width":0,"height":64,"opacity":50,"size_from":"width_slider","width":4} | yes |
| fx_check_025.json is refused in a sentence | Checkerboard's width runs from 1 to 10000, and this is 0. | yes |
| fx_check_026.json is refused in a sentence | Checkerboard's height runs from 1 to 10000, and this is 10001. | yes |
| fx_check_027.json is refused in a sentence | Checkerboard's feather width runs from 0 to 10000, and this is -1. | yes |
| fx_check_028.json is refused in a sentence | Checkerboard's opacity runs from 0 to 100, and this is 101. | yes |
| fx_check_029.json is refused in a sentence | Checkerboard's anchor runs from -1000 to 1000, and this is 1001. | yes |
| fx_check_030.json is refused in a sentence | Checkerboard's size from is "corner_point", "width_slider" or "width_and_height_sliders", and this is "corner". | yes |
| fx_check_031.json is refused in a sentence | Checkerboard's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| fx_check_032.json is refused in a sentence | Checkerboard's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Checkerboard with no `size_from` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Checkerboard with a width in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Checkerboard whose anchor is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 0 is refused with a sentence, and nothing changes | Checkerboard's width runs from 1 to 10000, and this is 0. | yes |
| size from "corner" is refused with a sentence, and nothing changes | Checkerboard's size from is "corner_point", "width_slider" or "width_and_height_sliders", and this is "corner". | yes |
| blending mode "darken" is refused with a sentence, and nothing changes | Checkerboard's blending mode is "none", "normal", "add", "multiply", "screen", "overlay", "soft_light" or "stencil_alpha", and this is "darken". | yes |
| colour "white" is refused with a sentence, and nothing changes | Checkerboard's colour is written #rrggbb, and this is "white". | yes |
| width keyed to 10001 is refused with a sentence, and nothing changes | Checkerboard's width runs from 1 to 10000, and this is 10001. | yes |
| Width & Height Sliders 3 by 2, feathers 1 and 2, opacity 60, orange, Multiply is taken | taken | yes |
| anchor keyed from 50, 50 to 75, 50 is taken | taken | yes |
| opacity keyed from 100 to 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_check_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_check_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_check_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_check_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_check_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard as it starts (white squares 64 across from the middle, None) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1272064 pixels changed | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 0, Full | largest difference 1 of 255, 1343 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 100, Full | largest difference 1 of 255, 1687 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 239, Full | largest difference 1 of 255, 1169 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Normal, orange at 70 per cent, width 100, feathers 16 and 8 on three layers, frame 239, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1036800 pixels changed | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 0, Full | largest difference 1 of 255, 1267 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 100, Full | largest difference 1 of 255, 718 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 239, Full | largest difference 1 of 255, 812 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 100, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Corner Point (anchor 10, 10, corner 15, 20), violet, Multiply on three layers, frame 239, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1709338 pixels changed | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 0, Full | largest difference 1 of 255, 1231 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 100, Full | largest difference 1 of 255, 685 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 239, Full | largest difference 1 of 255, 873 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 0, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard Width & Height Sliders 200 by 50, feather 30, Overlay on three layers, frame 239, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1036800 pixels changed | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 0, Full | largest difference 1 of 255, 2382 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 100, Full | largest difference 1 of 255, 754 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 239, Full | largest difference 1 of 255, 907 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 100, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 40, Soft Light, blue on three layers, frame 239, Draft | largest difference 1 of 255, 21 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1036800 pixels changed | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 0, Full | largest difference 1 of 255, 2367 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 100, Full | largest difference 1 of 255, 776 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 239, Full | largest difference 1 of 255, 894 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 100, Draft | largest difference 1 of 255, 21 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 128, anchor -30, 130, Screen on three layers, frame 239, Draft | largest difference 1 of 255, 21 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1036800 pixels changed | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 0, Full | largest difference 1 of 255, 2554 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 100, Full | largest difference 1 of 255, 834 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 239, Full | largest difference 1 of 255, 1449 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 32, Add at 50 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1749599 pixels changed | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 0, Full | largest difference 1 of 255, 2088 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 100, Full | largest difference 1 of 255, 1148 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 239, Full | largest difference 1 of 255, 1050 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 0, Draft | largest difference 1 of 255, 582 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 100, Draft | largest difference 1 of 255, 634 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Checkerboard width 90, feathers 40, Stencil Alpha on three layers, frame 239, Draft | largest difference 1 of 255, 547 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-413 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: white squares 64 across from the middle in place of the street, the others clear; draws cleanly | [], 129600 pixels changed | yes |
| 3_normal_feathered.png, Normal, orange at 60 per cent, squares 40 across with soft edges over the street; draws cleanly | [], 98352 pixels changed | yes |
| 4_corner_point.png, Corner Point, rectangles from 10, 10 to 25, 20 per cent, violet, Multiply: the street darkened in every second one; draws cleanly | [], 64800 pixels changed | yes |
| 5_overlay.png, Width & Height Sliders 60 by 20, blue, Overlay; draws cleanly | [], 64800 pixels changed | yes |
| 6_stencil.png, Stencil Alpha, squares 50 across, feathered 20: the street seen only through the squares; draws cleanly | [], 104100 pixels changed | yes |

## Result

235 of 235 checks pass.
