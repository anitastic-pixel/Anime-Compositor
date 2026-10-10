# D-418: Fill

B-297, after After Effects' Fill: the masks chosen as Path Stroke's (Fill Mask n, its floor taken, of the masks switched on with two points or more, or every one with All Masks; 0 for the whole layer), each covered by ADR-016's 4 by 4 even-odd rule and joined as a screen, feathered by a Gaussian of sigma = feather / 2 each way, turned over by Invert, then P (1 - k) + C P.a k with k = covering x Opacity: the layer's alpha is kept. A mask asked for and missing leaves the layer as it is with a warning each frame. The layer never grows. Every expected pixel is `Fixtures/fill/expected_fill.json`, written by `tools/fill_reference.py` before this code existed and printed in document 25 as FX-FILL-001 to 044. Tolerance 2e-5.

## FX-FILL-001 to 044 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FILL-001 frame 0: The settings as they start: Fill Mask 0 (None), red, opacity 100: the whole cel red, its line, skin and shadow alike, the soft left column half red and half see-through as it was, the empty border still empty. | largest difference 3.0e-8 | yes |
| FX-FILL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-002 frame 0: Fill Mask 1, a box from (2, 2) to (13, 7) of mode None: red inside the box, the cel as it was outside it. | largest difference 1.9e-7 | yes |
| FX-FILL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-003 frame 0: FX-FILL-002 with Invert: red outside the box, the cel inside it. | largest difference 1.9e-7 | yes |
| FX-FILL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-004 frame 0: FX-FILL-002 at opacity 50: half way to red inside the box. | largest difference 1.9e-7 | yes |
| FX-FILL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-005 frame 0: Horizontal Feather 4: the box's left and right edges soften over about two pixels each side, its top and bottom stay sharp. | largest difference 1.9e-7 | yes |
| FX-FILL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-006 frame 0: Vertical Feather 4: the top and bottom soften, the sides stay sharp. | largest difference 1.5e-7 | yes |
| FX-FILL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-007 frame 0: Horizontal Feather 3 and Vertical Feather 6: soft both ways, more down than across. | largest difference 1.4e-7 | yes |
| FX-FILL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-008 frame 0: Both feathers 4 with Invert: the soft band turned over with the rest. | largest difference 1.8e-7 | yes |
| FX-FILL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-009 frame 0: A curved mask, a circle 8 across about (8, 5): its edge pixels part red, as much as of each pixel's sixteen samples falls inside. | largest difference 1.9e-7 | yes |
| FX-FILL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-010 frame 0: A triangle with a sloped edge from (14, 1) to (1, 9): the pixels it crosses part red. | largest difference 1.9e-7 | yes |
| FX-FILL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-011 frame 0: Two masks, the box and a small box inside it, Fill Mask 2: the small box alone. | largest difference 1.9e-7 | yes |
| FX-FILL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-012 frame 0: Two overlapping masks, (1, 1) to (7, 6) and (5, 3) to (12, 9), All Masks on: both filled, the overlap once. | largest difference 1.9e-7 | yes |
| FX-FILL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-013 frame 0: The same two, All Masks on, both feathers 3: soft edges, the overlap still no redder than full. | largest difference 1.6e-7 | yes |
| FX-FILL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-014 frame 0: Two masks, the first switched off, All Masks on: the second alone, FX-FILL-012's right box only. | largest difference 1.9e-7 | yes |
| FX-FILL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-015 frame 0: The box's mask of mode Add: the cel is cut to the box first, then filled, so the box is red and the rest empty. | largest difference 0.0e0 | yes |
| FX-FILL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-016 frame 0: Colour #3080ff, a blue. | largest difference 1.9e-7 | yes |
| FX-FILL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-017 frame 0: The same blue written in capitals, #3080FF: the same. | largest difference 1.9e-7 | yes |
| FX-FILL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-018 frame 0: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the fill follows it, frames 0, 2 and 4. | largest difference 1.9e-7 | yes |
| FX-FILL-018 frame 2: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the fill follows it, frames 0, 2 and 4. | largest difference 1.9e-7 | yes |
| FX-FILL-018 frame 4: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the fill follows it, frames 0, 2 and 4. | largest difference 1.9e-7 | yes |
| FX-FILL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-019 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the cel, frame 2 half way, frame 4 FX-FILL-002's. | largest difference 1.9e-7 | yes |
| FX-FILL-019 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the cel, frame 2 half way, frame 4 FX-FILL-002's. | largest difference 1.9e-7 | yes |
| FX-FILL-019 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the cel, frame 2 half way, frame 4 FX-FILL-002's. | largest difference 1.9e-7 | yes |
| FX-FILL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-020 frame 0: Horizontal Feather keyed from 0 at frame 0 to 8 at frame 4: sharp, then softer. | largest difference 1.9e-7 | yes |
| FX-FILL-020 frame 2: Horizontal Feather keyed from 0 at frame 0 to 8 at frame 4: sharp, then softer. | largest difference 1.9e-7 | yes |
| FX-FILL-020 frame 4: Horizontal Feather keyed from 0 at frame 0 to 8 at frame 4: sharp, then softer. | largest difference 1.9e-7 | yes |
| FX-FILL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-021 frame 0: FX-FILL-002 moved three pixels right: the fill moves with the layer. | largest difference 1.9e-7 | yes |
| FX-FILL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-022 frame 0: After a Motion Tile that grows the layer: the mask is the drawing's own, so the frame is FX-FILL-002's. | largest difference 1.9e-7 | yes |
| FX-FILL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-023 frame 0: Fill Mask 1.5: its floor, mask 1, FX-FILL-002's frame. | largest difference 1.9e-7 | yes |
| FX-FILL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-024 frame 0: Fill Mask 0 with Invert: nothing is filled, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-FILL-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-025 frame 0: Opacity 0: the cel as it was. | largest difference 1.9e-7 | yes |
| FX-FILL-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-026 frame 0: A mask of two points encloses nothing: the cel as it was. Inverted, FX-FILL-027, everything is filled. | largest difference 1.9e-7 | yes |
| FX-FILL-026: what opening it warns of, and what frame 4 warns of | ["MASK_INVALID_OUTLINE"] and ["MASK_INVALID_OUTLINE"] | yes |
| FX-FILL-027 frame 0: The two-point mask, inverted: the whole cel red, FX-FILL-001's frame. | largest difference 3.0e-8 | yes |
| FX-FILL-027: what opening it warns of, and what frame 4 warns of | ["MASK_INVALID_OUTLINE"] and ["MASK_INVALID_OUTLINE"] | yes |
| FX-FILL-028 frame 0: A mask from far left of the layer to x 12, Horizontal Feather 6: the covering is worked out past the layer's edge too, so the left edge stays fully red and only the edge at x 12 softens. | largest difference 1.2e-7 | yes |
| FX-FILL-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-029 frame 0: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 1.9e-7 | yes |
| FX-FILL-029 frame 2: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 1.9e-7 | yes |
| FX-FILL-029 frame 4: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 1.9e-7 | yes |
| FX-FILL-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-030 frame 0: Fill Mask 0 with All Masks on and two masks: All Masks wins, FX-FILL-012's frame. | largest difference 1.9e-7 | yes |
| FX-FILL-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FILL-031 frame 0: Fill Mask 1 with no masks at all. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-031 frame 4: Fill Mask 1 with no masks at all. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-031: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-FILL-032 frame 0: Fill Mask 3, of two. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-032 frame 4: Fill Mask 3, of two. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-032: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-FILL-033 frame 0: Fill Mask 1, switched off. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-033 frame 4: Fill Mask 1, switched off. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-033: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-FILL-034 frame 0: All Masks on, every mask switched off. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-034 frame 4: All Masks on, every mask switched off. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-034: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-FILL-035 frame 0: All Masks on with no masks at all. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-035 frame 4: All Masks on with no masks at all. Nothing to fill: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-FILL-035: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-FILL-036 frame 0: Fill Mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-036 frame 4: Fill Mask -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-037 frame 0: Fill Mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-037 frame 4: Fill Mask 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-038 frame 0: Horizontal Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-038 frame 4: Horizontal Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-039 frame 0: Vertical Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-039 frame 4: Vertical Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-040 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-040 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-041 frame 0: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-041 frame 4: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-042 frame 0: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-042 frame 4: Invert "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-043 frame 0: All Masks "maybe", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-043 frame 4: All Masks "maybe", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FILL-044 frame 0: Opacity keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-044 frame 4: Opacity keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FILL-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fill_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fill_017.json is saved with its colour in small letters, its words and numbers as written, no paths | {"all_masks":"off","color":"#3080ff","horizontal_feather":0,"invert":"off","mask":1,"opacity":100,"vertical_feather":0} | yes |
| fx_fill_036.json is refused in a sentence | Fill's mask runs from 0 to 1000, and this is -1. | yes |
| fx_fill_037.json is refused in a sentence | Fill's mask runs from 0 to 1000, and this is 1001. | yes |
| fx_fill_038.json is refused in a sentence | Fill's horizontal feather runs from 0 to 1000, and this is -1. | yes |
| fx_fill_039.json is refused in a sentence | Fill's vertical feather runs from 0 to 1000, and this is 1001. | yes |
| fx_fill_040.json is refused in a sentence | Fill's opacity runs from 0 to 100, and this is 101. | yes |
| fx_fill_041.json is refused in a sentence | Fill's colour is written #rrggbb, and this is "#12345". | yes |
| fx_fill_042.json is refused in a sentence | Fill's invert is "off" or "on", and this is "yes". | yes |
| fx_fill_043.json is refused in a sentence | Fill's all masks is "off" or "on", and this is "maybe". | yes |
| a file with a Fill with no `opacity` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fill with a mask in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fill whose invert is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| mask -1 is refused with a sentence, and nothing changes | Fill's mask runs from 0 to 1000, and this is -1. | yes |
| vertical feather 1001 is refused with a sentence, and nothing changes | Fill's vertical feather runs from 0 to 1000, and this is 1001. | yes |
| all masks "maybe" is refused with a sentence, and nothing changes | Fill's all masks is "off" or "on", and this is "maybe". | yes |
| invert "yes" is refused with a sentence, and nothing changes | Fill's invert is "off" or "on", and this is "yes". | yes |
| colour "red" is refused with a sentence, and nothing changes | Fill's colour is written #rrggbb, and this is "red". | yes |
| opacity keyed to 101 is refused with a sentence, and nothing changes | Fill's opacity runs from 0 to 100, and this is 101. | yes |
| mask 2, All Masks, blue, inverted, feathers 3 and 6, opacity 70 is taken | taken | yes |
| opacity keyed from 0 to 100 is taken | taken | yes |
| horizontal feather keyed from 0 to 8 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fill_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fill_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fill_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_fill_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fill_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_042.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_043.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fill_044.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Fill as added (the whole layer, red): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill as added (the whole layer, red) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 0, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 239, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill the whole layer, blue at 50 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 512315 pixels changed | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 100, Full | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 100, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 1 (a circle of radius 400), orange on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1971854 pixels changed | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 0, Full | largest difference 1 of 255, 1366 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 100, Full | largest difference 1 of 255, 2305 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 239, Full | largest difference 1 of 255, 1614 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 0, Draft | largest difference 1 of 255, 102 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 2, inverted, feathers 30 and 10, opacity 80 on three layers, frame 239, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1076070 pixels changed | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 0, Full | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 100, Full | largest difference 1 of 255, 310 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 239, Full | largest difference 1 of 255, 117 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 0, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 100, Draft | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill All Masks, feathers 60, violet on three layers, frame 239, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 541554 pixels changed | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 0, Full | largest difference 1 of 255, 2599 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 239, Full | largest difference 1 of 255, 1773 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fill mask 3, vertical feather 200, green at 60 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-418 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its two masks cut nothing); draws cleanly | [] | yes |
| 2_as_added.png, as added: the whole street red, its clear pixels left clear; draws cleanly | [], 129600 pixels changed of 129600 | yes |
| 3_mask_orange.png, mask 1, orange: a hard orange disc in the middle, the rest the street; draws cleanly | [], 20340 pixels changed of 129600 | yes |
| 4_mask_feathered.png, mask 1, feathers 24, orange: the disc with a soft edge; draws cleanly | [], 39440 pixels changed of 129600 | yes |
| 5_inverted_dark.png, mask 1 inverted, black at 60 per cent: everything outside the disc darkened, the disc left as the street; draws cleanly | [], 109712 pixels changed of 129600 | yes |
| 6_all_masks_blue.png, All Masks, horizontal feather 8, blue at 70 per cent: both discs blue, softened left and right; draws cleanly | [], 32888 pixels changed of 129600 | yes |

## Result

274 of 274 checks pass.
