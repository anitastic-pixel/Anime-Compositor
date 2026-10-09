# B-235: Path Stroke

D-356, P0-22: effects that draw along paths, proved with Path Stroke, after After Effects' Stroke (Generate). Every expected pixel is `Fixtures/stroke/expected_stroke.json`, written by `tools/stroke_reference.py` before this code existed and printed in document 25 as FX-STROKE-001 to 046. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-STROKE-001 to 046 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STROKE-001 frame 0: Mask 1, a box from (2, 2) to (13, 7) of mode None, stroked with the defaults but Brush Size 3: a white line 3 pixels wide all round the box, over the night sky and the empty half, Hardness 75, Spacing 15, Start 0, End 100, On Original Image. | largest difference 2.7e-8 | yes |
| FX-STROKE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-002 frame 0: End 50: from the top-left corner along the top and down the right side, half the box's 32 pixels. | largest difference 2.7e-8 | yes |
| FX-STROKE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-003 frame 0: Start 25, End 75: from 8 pixels along to 24. | largest difference 2.7e-8 | yes |
| FX-STROKE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-004 frame 0: Start 75, End 25: taken smaller first, FX-STROKE-003's frame. | largest difference 2.7e-8 | yes |
| FX-STROKE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-005 frame 0: Start 40, End 40: nothing is drawn; the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-STROKE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-006 frame 0: Spacing 100, Hardness 0: round dabs a brush apart, each soft, a string of beads. | largest difference 2.6e-8 | yes |
| FX-STROKE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-007 frame 0: Spacing 0: the brush laid all along, a smooth line. | largest difference 2.6e-8 | yes |
| FX-STROKE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-008 frame 0: Hardness 0: soft from its middle out. | largest difference 2.9e-8 | yes |
| FX-STROKE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-009 frame 0: Hardness 100: hard, smoothed over one pixel. | largest difference 2.7e-8 | yes |
| FX-STROKE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-010 frame 0: Brush Size 6. | largest difference 2.9e-8 | yes |
| FX-STROKE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-011 frame 0: Opacity 50: half covered at most. | largest difference 2.6e-8 | yes |
| FX-STROKE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-012 frame 0: Colour #ff3020, a red. | largest difference 2.6e-8 | yes |
| FX-STROKE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-013 frame 0: On Transparent: the stroke alone, the night sky gone. | largest difference 1.3e-8 | yes |
| FX-STROKE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-014 frame 0: Reveal Original Image: the night sky only under the stroke, the colour unused. | largest difference 1.1e-8 | yes |
| FX-STROKE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-015 frame 0: Brush Size 0: nothing is drawn; the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-STROKE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-016 frame 0: Brush Size 0, On Transparent: nothing at all. | largest difference 0.0e0 | yes |
| FX-STROKE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-017 frame 0: Two masks, the box and a small box from (5, 4) to (9, 6), Path mask 2: the small box alone. | largest difference 2.5e-8 | yes |
| FX-STROKE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-018 frame 0: Two masks, All Masks on: both. | largest difference 2.7e-8 | yes |
| FX-STROKE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-019 frame 0: Two masks, All Masks on, End 50: each mask's own first half. | largest difference 2.7e-8 | yes |
| FX-STROKE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-020 frame 0: Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4, linear: the box draws on, then the small box, as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, frame 3 the box and 1 pixel of the small box, frame 4 both. | largest difference 7.3e-9 | yes |
| FX-STROKE-020 frame 1: Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4, linear: the box draws on, then the small box, as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, frame 3 the box and 1 pixel of the small box, frame 4 both. | largest difference 2.7e-8 | yes |
| FX-STROKE-020 frame 2: Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4, linear: the box draws on, then the small box, as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, frame 3 the box and 1 pixel of the small box, frame 4 both. | largest difference 2.9e-8 | yes |
| FX-STROKE-020 frame 3: Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4, linear: the box draws on, then the small box, as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, frame 3 the box and 1 pixel of the small box, frame 4 both. | largest difference 2.7e-8 | yes |
| FX-STROKE-020 frame 4: Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4, linear: the box draws on, then the small box, as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, frame 3 the box and 1 pixel of the small box, frame 4 both. | largest difference 2.7e-8 | yes |
| FX-STROKE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-021 frame 0: Stroke Sequentially on with All Masks off: the one mask, FX-STROKE-002's frame. | largest difference 2.7e-8 | yes |
| FX-STROKE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-022 frame 0: A curved mask, a circle 8 across about (8, 5): the stroke follows the curve. | largest difference 3.0e-8 | yes |
| FX-STROKE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-023 frame 0: Start keyed from 0 to 50 and End from 50 to 100, frames 0 to 4: half the box travels round it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-023 frame 2: Start keyed from 0 to 50 and End from 50 to 100, frames 0 to 4: half the box travels round it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-023 frame 4: Start keyed from 0 to 50 and End from 50 to 100, frames 0 to 4: half the box travels round it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-024 frame 0: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the stroke follows it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-024 frame 2: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the stroke follows it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-024 frame 4: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the stroke follows it, frames 0, 2 and 4. | largest difference 2.7e-8 | yes |
| FX-STROKE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-025 frame 0: End keyed from 4 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100, is held at 100, and is End 100. | largest difference 1.8e-8 | yes |
| FX-STROKE-025 frame 2: End keyed from 4 at frame 0 to 100 at frame 4, eased past its end: frame 2 would pass 100, is held at 100, and is End 100. | largest difference 2.7e-8 | yes |
| FX-STROKE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-026 frame 0: FX-STROKE-001 moved three pixels right: the stroke moves with the drawing. | largest difference 2.7e-8 | yes |
| FX-STROKE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-027 frame 0: After a Motion Tile that grows the layer: the mask is the drawing's own, so the frame is FX-STROKE-001's. | largest difference 2.7e-8 | yes |
| FX-STROKE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-028 frame 0: The box's mask of mode Add: the drawing is cut to the box first, then stroked, the stroke's outer half over nothing. | largest difference 2.7e-8 | yes |
| FX-STROKE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-029 frame 0: Two masks, the first switched off, All Masks on: the small box alone. | largest difference 2.5e-8 | yes |
| FX-STROKE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-030 frame 0: Path mask 1.5: its floor, mask 1, FX-STROKE-001's frame. | largest difference 2.7e-8 | yes |
| FX-STROKE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-031 frame 0: No masks at all. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-031 frame 4: No masks at all. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-031: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-032 frame 0: Path mask 3, of two. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-032 frame 4: Path mask 3, of two. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-032: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-033 frame 0: Path mask 1, switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-033 frame 4: Path mask 1, switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-033: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-034 frame 0: All Masks on, every mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-034 frame 4: All Masks on, every mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-034: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-035 frame 0: Brush Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-035 frame 4: Brush Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-036 frame 0: Brush Hardness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-036 frame 4: Brush Hardness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-037 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-037 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-038 frame 0: Start -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-038 frame 4: Start -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-039 frame 0: End 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-039 frame 4: End 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-040 frame 0: Spacing 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-040 frame 4: Spacing 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-041 frame 0: Path mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-041 frame 4: Path mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-042 frame 0: A colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-042 frame 4: A colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-043 frame 0: Paint Style "paint", which is not "on_original", "on_transparent" or "reveal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-043 frame 4: Paint Style "paint", which is not "on_original", "on_transparent" or "reveal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-044 frame 0: All Masks "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-044 frame 4: All Masks "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-045 frame 0: Stroke Sequentially "maybe", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-045 frame 4: Stroke Sequentially "maybe", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STROKE-046 frame 0: End keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-046 frame 4: End keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the paths compose finds are not saved | {"all_masks":"off","brush_hardness":75,"brush_size":3,"color":"#ffffff","end":100,"mask":1,"opacity":100,"paint_style":"on_original","spacing":15,"start":0,"stroke_sequentially":"off"} | yes |
| fx_stroke_042.json is refused in a sentence | Path Stroke's colour is written #rrggbb, and this is "#12345". | yes |
| fx_stroke_043.json is refused in a sentence | Path Stroke's paint style is "on_original", "on_transparent" or "reveal", and this is "paint". | yes |
| fx_stroke_044.json is refused in a sentence | Path Stroke's all masks is "off" or "on", and this is "yes". | yes |
| fx_stroke_045.json is refused in a sentence | Path Stroke's stroke sequentially is "off" or "on", and this is "maybe". | yes |
| a file with a brush size written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no paint style is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| brush size 201 is refused with a sentence, and nothing changes | Path Stroke's brush size runs from 0 to 200, and this is 201. | yes |
| paint style "Reveal" is refused with a sentence, and nothing changes | Path Stroke's paint style is "on_original", "on_transparent" or "reveal", and this is "Reveal". | yes |
| End keyed to 120 is refused with a sentence, and nothing changes | Path Stroke's end runs from 0 to 100, and this is 120. | yes |
| End 60, On Transparent, is taken | taken | yes |
| End keyed from 0 to 100, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_020.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_stroke_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_stroke_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_stroke_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_stroke_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 21022 pixels changed | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 100, Full | largest difference 1 of 255, 1399 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke as added (mask 1, white, Brush Size 2) on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 230543 pixels changed | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 0, Full | largest difference 1 of 255, 3612 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 100, Full | largest difference 1 of 255, 1182 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 239, Full | largest difference 1 of 255, 1604 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 0, Draft | largest difference 1 of 255, 127 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 0, Full | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 100, Full | largest difference 1 of 255, 97 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 239, Full | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 0, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent on three layers, frame 239, Draft | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2070255 pixels changed | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 0, Full | largest difference 1 of 255, 242 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 100, Full | largest difference 1 of 255, 429 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 239, Full | largest difference 1 of 255, 290 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Path Stroke All Masks, Brush Size 60, Hardness 0, Reveal Original Image on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a street with a circle mask, in `verification/D-356 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| write_on_strip.png (frames 0, 6, 12, 18 and 24, each also alone as write_on_fNN.png), End keyed from 0 at frame 0 to 100 at frame 24: the line draws itself round the circle clockwise from the top; nothing changes off the circle | pixels changed per frame [0, 1435, 2819, 4210, 5541] | yes |
| travel_strip.png (frames 0, 6, 12, 18 and 24, each also alone as travel_fNN.png), Start keyed from 0 to 75 and End from 25 to 100: a quarter of the circle chases round it; nothing changes off the circle | pixels changed per frame [1435, 1438, 1447, 1441, 1435] | yes |
| beads.png, Spacing 100, Hardness 0, Brush Size 16: soft round dabs a brush apart, a string of beads | [], 8369 pixels changed, 0 off the circle | yes |
| reveal.png, Reveal Original Image, Brush Size 30: the street shows only along the circle | [], 119437 pixels changed, 109500 off the circle | yes |

## Result

247 of 247 checks pass.
