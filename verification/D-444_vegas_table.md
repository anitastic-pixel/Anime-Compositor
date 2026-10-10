# B-324: Vegas

D-444, after After Effects' Vegas (Generate): dashes along the layer's masks or a shape layer's paths, each fading from its start to its end, turning round as Rotation moves. The owner's decision of 2026-10-10, "Masks only for now": Image Contours, After Effects' default, is refused with a sentence. Every expected pixel is `Fixtures/vegas/expected_vegas.json`, written by `tools/vegas_reference.py` before this code existed and printed in document 25 as FX-VEGAS-001 to 060. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-VEGAS-001 to 060 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-VEGAS-001 frame 0: Mask 1, the box from (2, 2) to (13, 7), 32 pixels round, of mode None; Vegas as added but Segments 4: four dashes end to end, each 8 pixels long, white and whole at its start fading to nothing at its end, over the night sky and the empty half. | largest difference 3.3e-8 | yes |
| FX-VEGAS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-002 frame 0: As added, 32 segments: 32 dashes of 1 pixel each, a fine shimmer all round the box. | largest difference 2.8e-8 | yes |
| FX-VEGAS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-003 frame 0: Segments 4, Length 0.5, Even: four 4-pixel dashes spread round the box, 8 pixels apart. | largest difference 3.2e-8 | yes |
| FX-VEGAS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-004 frame 0: Segments 4, Length 0.5, Bunched: four 4-pixel dashes end to end round the first half of the box; the second half bare. | largest difference 3.2e-8 | yes |
| FX-VEGAS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-005 frame 0: FX-VEGAS-003 with Rotation 45: every dash an eighth of the way on, 4 pixels, into the gaps. | largest difference 3.2e-8 | yes |
| FX-VEGAS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-006 frame 0: FX-VEGAS-004 with Rotation -90: the dashes a quarter of the way back, running on past the box's first corner from its end. | largest difference 3.2e-8 | yes |
| FX-VEGAS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-007 frame 0: FX-VEGAS-004 with Rotation keyed from 0 at frame 0 to 360 at frame 4, linear: the train goes once round; frames 0, 1, 2 and 4, frame 4 FX-VEGAS-004's. | largest difference 3.2e-8 | yes |
| FX-VEGAS-007 frame 1: FX-VEGAS-004 with Rotation keyed from 0 at frame 0 to 360 at frame 4, linear: the train goes once round; frames 0, 1, 2 and 4, frame 4 FX-VEGAS-004's. | largest difference 3.2e-8 | yes |
| FX-VEGAS-007 frame 2: FX-VEGAS-004 with Rotation keyed from 0 at frame 0 to 360 at frame 4, linear: the train goes once round; frames 0, 1, 2 and 4, frame 4 FX-VEGAS-004's. | largest difference 3.2e-8 | yes |
| FX-VEGAS-007 frame 4: FX-VEGAS-004 with Rotation keyed from 0 at frame 0 to 360 at frame 4, linear: the train goes once round; frames 0, 1, 2 and 4, frame 4 FX-VEGAS-004's. | largest difference 3.2e-8 | yes |
| FX-VEGAS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-008 frame 0: Two masks, the box and a small box from (5, 4) to (9, 6), All Masks on, Segments 2, Length 0.5, Random Phase off: each box's dashes start at its own first point. | largest difference 2.9e-8 | yes |
| FX-VEGAS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-009 frame 0: FX-VEGAS-008 with Random Phase on, Random Seed 1: each box's dashes start somewhere of their own. | largest difference 2.7e-8 | yes |
| FX-VEGAS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-010 frame 0: FX-VEGAS-009 with Random Seed 2: other starts. | largest difference 2.9e-8 | yes |
| FX-VEGAS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-011 frame 0: FX-VEGAS-008 with Random Seed 7 and Random Phase off: the seed is unused, FX-VEGAS-008's frame. | largest difference 2.9e-8 | yes |
| FX-VEGAS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-012 frame 0: FX-VEGAS-001 with Blend Mode Transparent: the dashes alone, the night gone. | largest difference 2.3e-8 | yes |
| FX-VEGAS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-013 frame 0: FX-VEGAS-001 with Blend Mode Under: the dashes behind the layer, seen only on the empty right half. | largest difference 2.3e-8 | yes |
| FX-VEGAS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-014 frame 0: FX-VEGAS-001 with Blend Mode Stencil: the night only under the dashes, the colour unused. | largest difference 2.3e-8 | yes |
| FX-VEGAS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-015 frame 0: Segments 4, a red #ff3020, Width 4, Hardness 0.5. | largest difference 2.9e-8 | yes |
| FX-VEGAS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-016 frame 0: Segments 4, Width 3, Hardness 1: hard edges, smoothed over one pixel. | largest difference 3.2e-8 | yes |
| FX-VEGAS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-017 frame 0: Segments 4, Length 0.5, Even, Start Opacity 1 and End Opacity 1: whole dashes, no fade. | largest difference 2.8e-8 | yes |
| FX-VEGAS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-018 frame 0: Segments 4, Start Opacity 0.2, End Opacity 1, Mid-point Opacity 0.5 at Mid-point Position 0.25: faint, quickly brighter, then whole at the end. | largest difference 3.0e-8 | yes |
| FX-VEGAS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-019 frame 0: Segments 4, Start and End Opacity 1, Mid-point Opacity -1: each dash dips to nothing at its middle. | largest difference 3.2e-8 | yes |
| FX-VEGAS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-020 frame 0: Segments 4, Mid-point Position 0, Mid-point Opacity -0.4: the first line unused, each dash starting at 0.6 and fading to nothing. | largest difference 1.5e-8 | yes |
| FX-VEGAS-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-021 frame 0: Segments 4, Mid-point Position 1, Mid-point Opacity 0.5: the first line the whole dash, from 1 down to M, 0.5, at its end; End Opacity unused. | largest difference 2.8e-8 | yes |
| FX-VEGAS-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-022 frame 0: Length 0: nothing is drawn; the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-VEGAS-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-023 frame 0: Width 0: nothing is drawn; the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-VEGAS-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-024 frame 0: Width 0, Stencil: nothing at all. | largest difference 0.0e0 | yes |
| FX-VEGAS-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-025 frame 0: Segments 1, Length 0.75: one long dash three quarters of the way round, fading from the box's top-left corner. | largest difference 2.8e-8 | yes |
| FX-VEGAS-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-026 frame 0: Segments 4.7: its floor, 4, FX-VEGAS-001's frame. | largest difference 3.3e-8 | yes |
| FX-VEGAS-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-027 frame 0: A curved mask, a circle 8 across about (8, 5), Segments 6, Length 0.6, Even: six dashes follow the curve. | largest difference 3.3e-8 | yes |
| FX-VEGAS-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-028 frame 0: Two masks, Path mask 2: the small box alone, Segments 4. | largest difference 2.8e-8 | yes |
| FX-VEGAS-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-029 frame 0: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the dashes follow it, frames 0, 2 and 4. | largest difference 3.3e-8 | yes |
| FX-VEGAS-029 frame 2: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the dashes follow it, frames 0, 2 and 4. | largest difference 2.8e-8 | yes |
| FX-VEGAS-029 frame 4: The mask's path keyed from the box at frame 0 to the box two rows lower at frame 4: the dashes follow it, frames 0, 2 and 4. | largest difference 3.1e-8 | yes |
| FX-VEGAS-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-030 frame 0: FX-VEGAS-001 moved three pixels right: the dashes move with the drawing. | largest difference 3.3e-8 | yes |
| FX-VEGAS-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-031 frame 0: After a Motion Tile that grows the layer: the mask is the drawing's own, so the frame is FX-VEGAS-001's. | largest difference 3.3e-8 | yes |
| FX-VEGAS-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-032 frame 0: The box's mask of mode Add: the drawing is cut to the box first, then the dashes drawn, their outer half over nothing. | largest difference 2.9e-8 | yes |
| FX-VEGAS-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-033 frame 0: Width 3 and Segments keyed from 1 at frame 0 to 8 at frame 4: more, shorter dashes; frames 0, 2 and 4. | largest difference 2.6e-8 | yes |
| FX-VEGAS-033 frame 2: Width 3 and Segments keyed from 1 at frame 0 to 8 at frame 4: more, shorter dashes; frames 0, 2 and 4. | largest difference 2.9e-8 | yes |
| FX-VEGAS-033 frame 4: Width 3 and Segments keyed from 1 at frame 0 to 8 at frame 4: more, shorter dashes; frames 0, 2 and 4. | largest difference 3.2e-8 | yes |
| FX-VEGAS-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-034 frame 0: A shape layer holding the box as a shape with no fill and no stroke, Stroke Shapes, Segments 4: the dashes over nothing, FX-VEGAS-012's frame. | largest difference 2.3e-8 | yes |
| FX-VEGAS-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-035 frame 0: An open shape, a roof of two legs from (2, 7) up to (8, 2) and down to (14, 7), Segments 3, Length 0.5, Rotation 300: the open path has no bottom, and a dash that passes its end runs on from its start. | largest difference 2.4e-8 | yes |
| FX-VEGAS-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-036 frame 0: A star of five points as a shape, Segments 5, Length 0.5, Even, Width 1: dashes on its arms. | largest difference 2.9e-8 | yes |
| FX-VEGAS-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-037 frame 0: A shape layer with the box as a shape and the small box as a mask of mode None, Stroke Masks, Segments 4: the small box, the mask, alone. | largest difference 2.3e-8 | yes |
| FX-VEGAS-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-VEGAS-038 frame 0: No masks at all. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-038 frame 4: No masks at all. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-038: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-VEGAS-039 frame 0: Path mask 3, of two. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-039 frame 4: Path mask 3, of two. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-039: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-VEGAS-040 frame 0: All Masks on, every mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-040 frame 4: All Masks on, every mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-040: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-VEGAS-041 frame 0: Stroke Shapes on the night drawing, which is not a shape layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-041 frame 4: Stroke Shapes on the night drawing, which is not a shape layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-VEGAS-041: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-VEGAS-042 frame 0: Stroke "image_contours", After Effects' Image Contours, which is not built: refused with a sentence, never drawn along the masks instead. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-042 frame 4: Stroke "image_contours", After Effects' Image Contours, which is not built: refused with a sentence, never drawn along the masks instead. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-043 frame 0: Stroke "edges", which is not "masks", "shapes" or "image_contours". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-043 frame 4: Stroke "edges", which is not "masks", "shapes" or "image_contours". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-044 frame 0: Segments 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-044 frame 4: Segments 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-045 frame 0: Segments 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-045 frame 4: Segments 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-046 frame 0: Length 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-046 frame 4: Length 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-047 frame 0: Width 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-047 frame 4: Width 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-048 frame 0: Hardness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-048 frame 4: Hardness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-049 frame 0: Start Opacity -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-049 frame 4: Start Opacity -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-050 frame 0: Mid-point Opacity 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-050 frame 4: Mid-point Opacity 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-051 frame 0: Mid-point Position -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-051 frame 4: Mid-point Position -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-052 frame 0: End Opacity 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-052 frame 4: End Opacity 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-053 frame 0: Blend Mode "add", which is not "transparent", "over", "under" or "stencil". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-053 frame 4: Blend Mode "add", which is not "transparent", "over", "under" or "stencil". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-054 frame 0: Segment Distribution "random", which is not "bunched" or "even". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-054 frame 4: Segment Distribution "random", which is not "bunched" or "even". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-055 frame 0: Random Phase "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-055 frame 4: Random Phase "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-056 frame 0: A colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-056 frame 4: A colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-057 frame 0: All Masks "maybe", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-057 frame 4: All Masks "maybe", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-058 frame 0: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-058 frame 4: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-059 frame 0: Path mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-059 frame 4: Path mask 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-VEGAS-060 frame 0: Rotation keyed to 360001 at frame 4, above 360000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-060 frame 4: Rotation keyed to 360001 at frame 4, above 360000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-VEGAS-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vegas_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_vegas_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the paths compose finds are not saved | {"all_masks":"off","blend_mode":"over","color":"#ffffff","end_opacity":0,"hardness":0,"length":1,"mask":1,"mid_point_opacity":0,"mid_point_position":0.5,"random_phase":"off","random_seed":1,"rotation":0,"segment_distribution":"bunched","segments":4,"start_opacity":1,"stroke":"masks","width":2} | yes |
| fx_vegas_042.json is refused in a sentence | Vegas's Image Contours stroke is not built yet: it draws only along masks and shape paths, so set Stroke to Masks or Shapes. | yes |
| fx_vegas_043.json is refused in a sentence | Vegas's stroke is "masks", "shapes" or "image_contours", and this is "edges". | yes |
| fx_vegas_053.json is refused in a sentence | Vegas's blend mode is "over", "under", "transparent" or "stencil", and this is "add". | yes |
| fx_vegas_054.json is refused in a sentence | Vegas's segment distribution is "bunched" or "even", and this is "random". | yes |
| fx_vegas_055.json is refused in a sentence | Vegas's random phase is "off" or "on", and this is "yes". | yes |
| fx_vegas_056.json is refused in a sentence | Vegas's colour is written #rrggbb, and this is "#12345". | yes |
| fx_vegas_057.json is refused in a sentence | Vegas's all masks is "off" or "on", and this is "maybe". | yes |
| fx_vegas_042.json's Image Contours is kept as written | {"all_masks":"off","blend_mode":"over","color":"#ffffff","end_opacity":0,"hardness":0,"length":1,"mask":1,"mid_point_opacity":0,"mid_point_position":0.5,"random_phase":"off","random_seed":1,"rotation":0,"segment_distribution":"bunched","segments":32,"start_opacity":1,"stroke":"image_contours","width":2} | yes |
| a file with a width written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no blend mode is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 201 is refused with a sentence, and nothing changes | Vegas's width runs from 0 to 200, and this is 201. | yes |
| stroke "image_contours" is refused with a sentence, and nothing changes | Vegas's Image Contours stroke is not built yet: it draws only along masks and shape paths, so set Stroke to Masks or Shapes. | yes |
| blend mode "Over" is refused with a sentence, and nothing changes | Vegas's blend mode is "over", "under", "transparent" or "stencil", and this is "Over". | yes |
| Segments keyed to 1001 is refused with a sentence, and nothing changes | Vegas's segments runs from 1 to 1000, and this is 1001. | yes |
| Rotation 90, Even, is taken | taken | yes |
| Rotation keyed from 0 to 360, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vegas_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vegas_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vegas_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vegas_029.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vegas_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_vegas_035.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_vegas_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames (expected 10); the same warnings: true | yes |
| fx_vegas_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| fx_vegas_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames (expected 0); the same warnings: true | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 20740 pixels changed | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 100, Full | largest difference 1 of 255, 1404 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 239, Full | largest difference 1 of 255, 1776 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 100, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas as added (mask 1, 32 segments, white, Width 2) on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 213094 pixels changed | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 0, Full | largest difference 1 of 255, 3471 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 100, Full | largest difference 1 of 255, 1358 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 239, Full | largest difference 1 of 255, 1697 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 100, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5 on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 73718 pixels changed | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 0, Full | largest difference 1 of 255, 3449 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 100, Full | largest difference 1 of 255, 1255 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 239, Full | largest difference 1 of 255, 1382 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 0, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 100, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6 on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 0, Full | largest difference 1 of 255, 547 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 100, Full | largest difference 1 of 255, 834 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 239, Full | largest difference 1 of 255, 684 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 0, Draft | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 3, Width 60, Transparent on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073531 pixels changed | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 0, Full | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 100, Full | largest difference 1 of 255, 339 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 239, Full | largest difference 1 of 255, 258 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 0, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Vegas All Masks, Segments 8, Length 0.7, Width 50, Stencil on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a street with a circle mask, in `verification/D-444 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| chase_strip.png (frames 0, 6, 12, 18 and 24, each also alone as chase_fNN.png), five orange dashes spread round the circle, Rotation keyed from 0 to 360: they chase once round, frame 24 back where frame 0 was; nothing changes off the circle | pixels changed per frame [3730, 3712, 3747, 3736, 3730]; frame 6 differs from frame 0 in 5421 pixels; frame 24 against frame 0, largest difference 0 of 255 | yes |
| tails.png, Segments 4, Width 16, Hardness 0.5: four dashes end to end, each whole at its start and fading to nothing at its end, comet tails | [], 10308 pixels changed, 0 off the circle | yes |
| stencil.png, Segments 6, Length 0.6, Width 30, Stencil: the street shows only inside the dashes | [], 128921 pixels changed, 109500 off the circle | yes |
| the street with Stroke Image Contours: drawn without the effect, and a sentence says why | ["EFFECT_PARAMETER_INVALID Layer town's core.vegas has a setting this build cannot use, so it is not drawn. Vegas's Image Contours stroke is not built yet: it draws only along masks and shape paths, so set Stroke to Masks or Shapes. Frame 0 is drawn without the effect, which is kept as it was."] | yes |

## Result

314 of 314 checks pass.
