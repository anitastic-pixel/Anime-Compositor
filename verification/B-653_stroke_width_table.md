# B-653: Path Stroke's Brush Size is the brush's radius (D-773)

Seven tutorials from seven creators read After Effects' Stroke line 1.64 to 2.2 times Brush Size across (median 1.91; 1.91 to 1.96 at Hardness 100), where ours drew 1 times. Path Stroke's brush now has radius r = Brush Size, so the line is 2 Brush Size across, and its dabs are Spacing per cent of that width apart. The widths are counted on a hard brush (Hardness 100, Spacing 0) along the top edge of a box mask, as the rows of one column covered at least half; the fixtures' numbers are the reference's, whose sizes D-773 halved so their pixels stay.

## The line's width, in pixels covered at least half across the top edge

| Check | The build's answer | Matches |
| --- | --- | --- |
| Brush Size 2, Hardness 100: 4 pixels across, twice the size (it was 2 before D-773) | 4 pixels | yes |
| Brush Size 10, Hardness 100: 20 pixels across, twice the size (it was 10 before D-773) | 20 pixels | yes |
| Brush Size 25, Hardness 100: 50 pixels across, twice the size (it was 25 before D-773) | 50 pixels | yes |

## The dabs: Spacing per cent of the width, 2 Brush Size, apart

| Check | The build's answer | Matches |
| --- | --- | --- |
| Brush Size 10, Spacing 100: a dab every 20 pixels along the edge (row 49) | runs start at [57, 77, 97, 117, 137, 157, 177, 197] | yes |
| Brush Size 10, Spacing 50: a dab every 10 pixels along the edge (row 49) | runs start at [47, 57, 67, 77, 87, 97, 107, 117, 127, 137, 147, 157, 167, 177, 187, 197] | yes |
| Brush Size 6, Spacing 100: a dab every 12 pixels along the edge (row 45) | runs start at [42, 54, 66, 78, 90, 102, 114, 126, 138, 150, 162, 174, 186, 198] | yes |

## The fixtures, at the reference's numbers (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STROKE-001 frame 0: Mask 1, a box from (2, 2) to (13, 7) of mode None, stroked with the defaults but Brush Size 1.5: a white line 3 pixels wide all round the box (D-773: Brush Size is the radius; 3 before), over the night sky and the empty half, Hardness 75, Spacing 15, Start 0, End 100, On Original Image. | largest difference 2.7e-8 | yes |
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
| FX-STROKE-010 frame 0: Brush Size 3: a brush 6 pixels wide (D-773; 6 before). | largest difference 2.9e-8 | yes |
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

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_001.json as saved before D-773 (Brush Size 3, its file at a259ec22) opens with no warning | [] | yes |
| and saves back holding what it held: Brush Size still 3, nothing added | the same | yes |
| and draws a line twice as wide, with no warning: FX-STROKE-010's frame (the 6-pixel brush Brush Size 6 drew before) | largest difference 2.9e-8; warnings [] | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| Brush Size 10, Hardness 100, Spacing 0, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| Brush Size 25, Hardness 50, Spacing 0, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| Brush Size 10, Spacing 100, On Original Image, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| Brush Size 6, Hardness 79, Spacing 15, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_stroke_001.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_stroke_006.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_stroke_010.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_stroke_022.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

134 of 134 checks pass.
