# D-442: Scribble

B-322, after After Effects' Scribble: the chosen closed masks (one, all, or all combined by their modes), or a band along their outline, crossed by lines Spacing apart at Angle, each run Path Overlap past the edge, joined at alternate ends by turns as round as Curviness says, every variation drawn from Random Seed and changed Wiggles/Second times a second (Static never, Jumpy at once, Smooth gliding); trimmed from Start to End along the scribble's length (all masks as one length when Fill Paths Sequentially is on) and drawn with Path Stroke's round brush Stroke Width across, on the layer, on transparent or revealing the layer. With no usable mask it draws nothing and says EFFECT_PATH_MISSING every frame. Every expected pixel is `Fixtures/scribble/expected_scribble.json`, written by `tools/scribble_reference.py` before this code existed and printed in document 25 as FX-SCRIBBLE-001 to 071. Tolerance 2e-5.

## FX-SCRIBBLE-001 to 071 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SCRIBBLE-001 frame 0: The settings as added on the box from (2, 2) to (13, 7): white lines 5 wide at 45 degrees, 5 apart, a little curved and wiggling, covering most of the box. | largest difference 1.8e-7 | yes |
| FX-SCRIBBLE-001 frame 4: The settings as added on the box from (2, 2) to (13, 7): white lines 5 wide at 45 degrees, 5 apart, a little curved and wiggling, covering most of the box. | largest difference 1.5e-7 | yes |
| FX-SCRIBBLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-002 frame 0: Angle 0, Stroke Width 1, Spacing 2.5, no curve or wiggle: two red lines across the box at y 3.25 and 5.75, joined at the right by a straight turn, a Z laid on its side. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-003 frame 0: FX-SCRIBBLE-002 at Angle 90: the lines run up the box, 2.5 apart from its left side. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-004 frame 0: FX-SCRIBBLE-002 at Angle 30: sloped lines, each cut where it leaves the box. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-005 frame 0: FX-SCRIBBLE-002 at Angle -45. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-006 frame 0: FX-SCRIBBLE-002 with Curviness 100: each turn a round loop out past the box's side. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-007 frame 0: FX-SCRIBBLE-002 with Spacing 1.5: four lines, closer. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-008 frame 0: FX-SCRIBBLE-002 with Stroke Width 2: thicker lines. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-009 frame 0: FX-SCRIBBLE-002 with Path Overlap 1.5: each line runs 1.5 past the box at both ends. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-010 frame 0: FX-SCRIBBLE-002 with Path Overlap -3: each line pulled 3 inside the box at both ends. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-011 frame 0: FX-SCRIBBLE-002 with Opacity 50: half covered at most. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-012 frame 0: FX-SCRIBBLE-002 On Transparent: the lines alone, the cel gone. | largest difference 2.3e-9 | yes |
| FX-SCRIBBLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-013 frame 0: FX-SCRIBBLE-002 Reveal Original Image: the cel only under the lines. | largest difference 1.6e-7 | yes |
| FX-SCRIBBLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-014 frame 0: FX-SCRIBBLE-002 with Stroke Width 0: nothing drawn, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-015 frame 0: FX-SCRIBBLE-002 with End 50: only the first half of the scribble's length. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-016 frame 0: Start 30 and End 80 at Spacing 1.5: the middle half. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-017 frame 0: End keyed from 0 at frame 0 to 100 at frame 8: the scribble draws on, nothing at frame 0, the whole at frame 8. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-017 frame 4: End keyed from 0 at frame 0 to 100 at frame 8: the scribble draws on, nothing at frame 0, the whole at frame 8. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-017 frame 8: End keyed from 0 at frame 0 to 100 at frame 8: the scribble draws on, nothing at frame 0, the whole at frame 8. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-018 frame 0: Fill Type Centered Edge, Edge Width 2: lines over the band 1 each side of the box's outline, a gap in the middle. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-019 frame 0: Fill Type Inside Edge, Edge Width 2: the band 2 inside the outline. | largest difference 1.6e-7 | yes |
| FX-SCRIBBLE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-020 frame 0: Fill Type Outside Edge, Edge Width 1.5: the band 1.5 outside the outline. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-021 frame 0: Right Edge on the box drawn clockwise: FX-SCRIBBLE-019's frame. | largest difference 1.6e-7 | yes |
| FX-SCRIBBLE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-022 frame 0: Left Edge on the box drawn clockwise: the outside band, as Outside Edge at Edge Width 2. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-023 frame 0: Left Edge on the box drawn the other way: the inside band, FX-SCRIBBLE-019's frame. | largest difference 1.6e-7 | yes |
| FX-SCRIBBLE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-024 frame 0: A curved mask, a circle 8 across: the lines cut at its curve. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-025 frame 0: A concave arrow head: on the lines that cross it twice the pen lifts over the notch. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-026 frame 0: Two masks, Mask 2: the small box alone. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-027 frame 0: Two overlapping boxes, All Masks, Fill Paths Sequentially on, End 50: each box scribbled on its own, one length, so only the first box's lines and part of the second's. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-028 frame 0: FX-SCRIBBLE-027 with Fill Paths Sequentially off: each box trimmed to its own half. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-029 frame 0: All Masks Using Modes, the second box Add: one region, both boxes together, On Transparent. | largest difference 3.0e-8 | yes |
| FX-SCRIBBLE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-030 frame 0: Using Modes, the second box Subtract: the first box less the second. | largest difference 2.6e-8 | yes |
| FX-SCRIBBLE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-031 frame 0: Using Modes, the second box Intersect: their common part only. | largest difference 6.3e-10 | yes |
| FX-SCRIBBLE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-032 frame 0: Using Modes, the second box Difference: either but not both. | largest difference 3.0e-8 | yes |
| FX-SCRIBBLE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-033 frame 0: Using Modes, the second box Add and inverted: the first box and everything outside the second. | largest difference 2.3e-9 | yes |
| FX-SCRIBBLE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-034 frame 0: Using Modes with a single mask in Subtract: the whole buffer less the box. | largest difference 2.3e-9 | yes |
| FX-SCRIBBLE-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-035 frame 0: Using Modes, the second mask of mode None: left out, the first box alone. | largest difference 2.3e-9 | yes |
| FX-SCRIBBLE-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-036 frame 0: Single Mask on an inverted mask: inversion is not used outside Using Modes, FX-SCRIBBLE-002's frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-037 frame 0: Variations on, Wiggle Type Static: uneven spacing, overshoots and loops, the same at every frame. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-037 frame 4: Variations on, Wiggle Type Static: uneven spacing, overshoots and loops, the same at every frame. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-038 frame 0: The same, Jumpy at 6 a second: a new scribble every 4 frames, frames 0 and 2 alike, 4 new. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-038 frame 2: The same, Jumpy at 6 a second: a new scribble every 4 frames, frames 0 and 2 alike, 4 new. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-038 frame 4: The same, Jumpy at 6 a second: a new scribble every 4 frames, frames 0 and 2 alike, 4 new. | largest difference 1.5e-7 | yes |
| FX-SCRIBBLE-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-039 frame 0: The same, Smooth: frame 2 half way between frames 0 and 4's scribbles. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-039 frame 2: The same, Smooth: frame 2 half way between frames 0 and 4's scribbles. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-039 frame 4: The same, Smooth: frame 2 half way between frames 0 and 4's scribbles. | largest difference 1.5e-7 | yes |
| FX-SCRIBBLE-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-040 frame 0: FX-SCRIBBLE-037 with Random Seed 7: a different scribble. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-041 frame 0: Smooth at 0 wiggles a second: still, every frame FX-SCRIBBLE-037's. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-041 frame 4: Smooth at 0 wiggles a second: still, every frame FX-SCRIBBLE-037's. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-042 frame 0: FX-SCRIBBLE-002 with the layer moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-043 frame 0: Mask 1.5: its floor, mask 1, FX-SCRIBBLE-002's frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-044 frame 0: A mask of two points encloses nothing: Inside draws nothing; Centered Edge, FX-SCRIBBLE-045, scribbles the band about it. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-044: what opening it warns of, and what frame 4 warns of | ["MASK_INVALID_OUTLINE"] and ["MASK_INVALID_OUTLINE"] | yes |
| FX-SCRIBBLE-045 frame 0: The two-point mask, Centered Edge, Edge Width 2. | largest difference 2.0e-7 | yes |
| FX-SCRIBBLE-045: what opening it warns of, and what frame 4 warns of | ["MASK_INVALID_OUTLINE"] and ["MASK_INVALID_OUTLINE"] | yes |
| FX-SCRIBBLE-046 frame 0: Angle keyed from 0 at frame 0 to 90 at frame 4: the lines turn. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-046 frame 2: Angle keyed from 0 at frame 0 to 90 at frame 4: the lines turn. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-046 frame 4: Angle keyed from 0 at frame 0 to 90 at frame 4: the lines turn. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-047 frame 0: Opacity keyed from 40 to 100 by an ease that passes its end: held at 100 at frame 2. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-047 frame 2: Opacity keyed from 40 to 100 by an ease that passes its end: held at 100 at frame 2. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-047 frame 4: Opacity keyed from 40 to 100 by an ease that passes its end: held at 100 at frame 2. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-048 frame 0: Colour written in capitals, #FF3020: FX-SCRIBBLE-002's frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SCRIBBLE-049 frame 0: Mask 1 with no masks at all. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-049 frame 4: Mask 1 with no masks at all. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-049: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SCRIBBLE-050 frame 0: Mask 3, of two. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-050 frame 4: Mask 3, of two. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-050: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SCRIBBLE-051 frame 0: Mask 1, switched off. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-051 frame 4: Mask 1, switched off. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-051: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SCRIBBLE-052 frame 0: All Masks, every mask switched off. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-052 frame 4: All Masks, every mask switched off. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-052: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SCRIBBLE-053 frame 0: All Masks Using Modes with no masks at all. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-053 frame 4: All Masks Using Modes with no masks at all. Nothing to scribble: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-053: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-SCRIBBLE-054 frame 0: Mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-054 frame 4: Mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-055 frame 0: Edge Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-055 frame 4: Edge Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-056 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-056 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-057 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-057 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-058 frame 0: Stroke Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-058 frame 4: Stroke Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-059 frame 0: Curviness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-059 frame 4: Curviness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-060 frame 0: Spacing 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-060 frame 4: Spacing 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-061 frame 0: Path Overlap -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-061 frame 4: Path Overlap -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-062 frame 0: End 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-062 frame 4: End 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-063 frame 0: Wiggles/Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-063 frame 4: Wiggles/Second -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-064 frame 0: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-064 frame 4: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-065 frame 0: Scribble "some_masks", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-065 frame 4: Scribble "some_masks", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-065: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-066 frame 0: Fill Type "outline", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-066 frame 4: Fill Type "outline", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-066: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-067 frame 0: Fill Paths Sequentially "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-067 frame 4: Fill Paths Sequentially "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-067: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-068 frame 0: Wiggle Type "wobbly", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-068 frame 4: Wiggle Type "wobbly", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-068: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-069 frame 0: Composite "glow", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-069 frame 4: Composite "glow", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-069: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-070 frame 0: Colour "#12345", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-070 frame 4: Colour "#12345", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-070: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SCRIBBLE-071 frame 0: Stroke Width keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-071 frame 4: Stroke Width keyed to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SCRIBBLE-071: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scribble_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_scribble_048.json is saved with its colour in small letters | "#ff3020" | yes |
| fx_scribble_017.json is saved with its words and numbers as written, End's keys kept, and no masks or time | {"angle":0,"color":"#ff3020","composite":"on_original","curviness":0,"curviness_variation":0,"edge_width":2,"end":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":8,"interp":"linear","value":100}]},"fill_paths_sequentially":"on","fill_type":"inside","mask":1,"opacity":100,"path_overlap":0,"path_overlap_variation":0,"random_seed":1,"scribble":"single_mask","spacing":1.5,"spacing_variation":0,"start":0,"stroke_width":1,"wiggle_type":"static","wiggles_per_second":6} | yes |
| fx_scribble_054.json is refused in a sentence | Scribble's mask runs from 1 to 1000, and this is 0. | yes |
| fx_scribble_055.json is refused in a sentence | Scribble's edge width runs from 0 to 1000, and this is -1. | yes |
| fx_scribble_056.json is refused in a sentence | Scribble's opacity runs from 0 to 100, and this is 101. | yes |
| fx_scribble_057.json is refused in a sentence | Scribble's angle runs from -3600 to 3600, and this is 3601. | yes |
| fx_scribble_058.json is refused in a sentence | Scribble's stroke width runs from 0 to 1000, and this is 1001. | yes |
| fx_scribble_059.json is refused in a sentence | Scribble's curviness runs from 0 to 100, and this is 101. | yes |
| fx_scribble_060.json is refused in a sentence | Scribble's spacing runs from 1 to 1000, and this is 0.5. | yes |
| fx_scribble_061.json is refused in a sentence | Scribble's path overlap runs from -1000 to 1000, and this is -1001. | yes |
| fx_scribble_062.json is refused in a sentence | Scribble's end runs from 0 to 100, and this is 101. | yes |
| fx_scribble_063.json is refused in a sentence | Scribble's wiggles per second runs from 0 to 100, and this is -1. | yes |
| fx_scribble_064.json is refused in a sentence | Scribble's random seed runs from 0 to 100000, and this is 100001. | yes |
| fx_scribble_065.json is refused in a sentence | Scribble's scribble is one of single_mask, all_masks, all_masks_using_modes, and this is "some_masks". | yes |
| fx_scribble_066.json is refused in a sentence | Scribble's fill type is one of inside, centered_edge, inside_edge, outside_edge, left_edge, right_edge, and this is "outline". | yes |
| fx_scribble_067.json is refused in a sentence | Scribble's fill paths sequentially is "off" or "on", and this is "yes". | yes |
| fx_scribble_068.json is refused in a sentence | Scribble's wiggle type is one of static, jumpy, smooth, and this is "wobbly". | yes |
| fx_scribble_069.json is refused in a sentence | Scribble's composite is one of on_original, on_transparent, reveal, and this is "glow". | yes |
| fx_scribble_070.json is refused in a sentence | Scribble's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Scribble with no `composite` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scribble whose composite is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Scribble whose fill paths sequentially is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| mask 0 is refused with a sentence, and nothing changes | Scribble's mask runs from 1 to 1000, and this is 0. | yes |
| spacing 0.5 is refused with a sentence, and nothing changes | Scribble's spacing runs from 1 to 1000, and this is 0.5. | yes |
| end 101 is refused with a sentence, and nothing changes | Scribble's end runs from 0 to 100, and this is 101. | yes |
| scribble "some_masks" is refused with a sentence, and nothing changes | Scribble's scribble is one of single_mask, all_masks, all_masks_using_modes, and this is "some_masks". | yes |
| fill type "outline" is refused with a sentence, and nothing changes | Scribble's fill type is one of inside, centered_edge, inside_edge, outside_edge, left_edge, right_edge, and this is "outline". | yes |
| colour "red" is refused with a sentence, and nothing changes | Scribble's colour is written #rrggbb, and this is "red". | yes |
| wiggle type "wobbly" is refused with a sentence, and nothing changes | Scribble's wiggle type is one of static, jumpy, smooth, and this is "wobbly". | yes |
| stroke width keyed to 2000 is refused with a sentence, and nothing changes | Scribble's stroke width runs from 0 to 1000, and this is 2000. | yes |
| All Masks Using Modes, mask 2, Outside Edge 6, blue, opacity 70, angle -30, stroke 3, curviness 40 and 10, spacing 7 and 2, overlap -1 and 3, 10 to 90, off, Jumpy 12, seed 7, reveal is taken | taken | yes |
| end keyed from 0 to 100 is taken | taken | yes |
| angle keyed from 0 to 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scribble_001.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scribble_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scribble_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scribble_029.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scribble_039.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_scribble_045.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_scribble_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_scribble_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_043.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_044.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_045.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_046.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_047.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_048.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_scribble_049.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_050.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_051.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_052.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_053.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_054.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_055.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_056.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_057.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_058.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_059.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_060.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_061.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_062.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_063.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_064.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_065.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_066.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_067.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_068.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_069.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_070.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_scribble_071.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 911094 pixels changed | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 0, Full | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 100, Full | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 239, Full | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 0, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 100, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble as added (white lines 5 across over the mask) on three layers, frame 239, Draft | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 100, Full | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 239, Full | largest difference 1 of 255, 572 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2007233 pixels changed | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 100, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100 on three layers, frame 239, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 325623 pixels changed | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 0, Full | largest difference 1 of 255, 2588 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 100, Full | largest difference 1 of 255, 1025 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 239, Full | largest difference 1 of 255, 1238 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 100, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Scribble angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6 on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-442 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its star mask of mode None changes nothing); draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: white lines 5 across at 45 degrees over the star; draws cleanly | [], 32931 pixels changed of 129600 | yes |
| 3_red_loose.png, frame 0, red, stroke 3, spacing 12, curviness 60 and 30, varied: a loose hand-drawn hatch over the star; draws cleanly | [], 11311 pixels changed of 129600 | yes |
| 4_outline_transparent.png, frame 0, Centered Edge 24, yellow, On Transparent: a hatched band along the star's outline, the street gone; draws cleanly | [], 129600 pixels changed of 129600 | yes |
| 5_reveal_half.png, frame 24, Reveal, End keyed 0 to 100 over two seconds, halfway: half the star's scribble, the street seen only through it; draws cleanly | [], 117332 pixels changed of 129600 | yes |

## Result

392 of 392 checks pass.
