# D-580: Card Wipe

B-460, after After Effects' Card Wipe (Transition): the layer cut into a grid of cards that flip over one after another to show the Back Layer, in its own fake 3D. Adobe's page names the controls and says what each does in a sentence but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/card_wipe_reference.py`: each card turns about its own middle when the sweep (the flip order, shuffled by Timing Randomness) reaches it, over Transition Width; a camera inside the effect (X, Y and Z Rotation, X, Y Position, Z Position in focal lengths, Focal Length on a 36 mm frame) sends one ray a pixel and the nearest card it meets is drawn, its front the picture and its back the Back Layer (clear when none), lit by a distant or point light with ambient, diffuse and a Blinn-Phong highlight. The composition camera and lights are not used. Every expected pixel is `Fixtures/card_wipe/expected_card_wipe.json`, written by the tool before this code existed and printed in document 25 as FX-CW-001 to 056. Tolerance 2e-5.

## FX-CW-001 to 056 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CW-001 frame 0: As added: nothing has flipped, so the picture is unchanged. | largest difference 1.5e-7 | yes |
| FX-CW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-002 frame 0: Completion 100 with no back layer: every card turned over, its back clear. | largest difference 0.0e0 | yes |
| FX-CW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-003 frame 0: Completion 50, width 50, left to right, four columns: the first column turned over (clear), the second three quarters, the third a quarter, the fourth not begun. | largest difference 1.5e-7 | yes |
| FX-CW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-004 frame 0: FX-CW-003 with the back layer: the turned cards show its share of the back. | largest difference 1.5e-7 | yes |
| FX-CW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-005 frame 0: Completion 100 with the back layer: the back picture whole, upright, its white marker at the top left. | largest difference 1.2e-7 | yes |
| FX-CW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-006 frame 0: FX-CW-004 turning about the y axis. | largest difference 1.7e-7 | yes |
| FX-CW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-007 frame 0: FX-CW-004 turning the negative way. | largest difference 1.5e-7 | yes |
| FX-CW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-008 frame 0: Right to left. | largest difference 1.5e-7 | yes |
| FX-CW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-009 frame 0: Top to bottom. | largest difference 1.5e-7 | yes |
| FX-CW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-010 frame 0: Bottom to top. | largest difference 1.5e-7 | yes |
| FX-CW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-011 frame 0: Top left to bottom right. | largest difference 1.9e-7 | yes |
| FX-CW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-012 frame 0: Bottom left to top right. | largest difference 1.8e-7 | yes |
| FX-CW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-013 frame 0: Top right to bottom left. | largest difference 1.7e-7 | yes |
| FX-CW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-014 frame 0: Bottom right to top left. | largest difference 2.0e-7 | yes |
| FX-CW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-015 frame 0: Flip order by the gradient layer `ramp`: dark cards (top right) first. | largest difference 6.2e-7 | yes |
| FX-CW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-016 frame 0: Flip order gradient with no gradient layer: every card at the same point, 60 per cent of the way over. | largest difference 0.0e0 | yes |
| FX-CW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-017 frame 0: Timing randomness 1, seed 7: the order wholly random. | largest difference 1.6e-7 | yes |
| FX-CW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-018 frame 0: FX-CW-017 with seed 8: another order. | largest difference 1.5e-7 | yes |
| FX-CW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-019 frame 0: Random axis and direction, seed 3, timing randomness 0.5. | largest difference 1.9e-7 | yes |
| FX-CW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-020 frame 0: Width 0: the cards in the first half turned over, the rest not. | largest difference 1.5e-7 | yes |
| FX-CW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-021 frame 0: Card scale 0.5, not begun: smaller cards, gaps between them. | largest difference 1.5e-7 | yes |
| FX-CW-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-022 frame 0: Card scale 0.5 at completion 40. | largest difference 1.5e-7 | yes |
| FX-CW-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-023 frame 0: Card scale 1.5, four rows, not begun: larger cards overlapping, the first in rows then columns on top. | largest difference 2.1e-7 | yes |
| FX-CW-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-024 frame 0: Card scale 1.5, four rows, at completion 40. | largest difference 2.1e-7 | yes |
| FX-CW-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-025 frame 0: Columns follow rows, rows 3 (columns 4 ignored): three by three. | largest difference 1.5e-7 | yes |
| FX-CW-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-026 frame 0: Rows 5 by columns 8. | largest difference 2.2e-7 | yes |
| FX-CW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-027 frame 0: The camera turned 30 degrees about y. | largest difference 2.4e-7 | yes |
| FX-CW-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-028 frame 0: The camera turned -20 about x and 15 about z. | largest difference 2.4e-7 | yes |
| FX-CW-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-029 frame 0: The camera at x 40, y 60 per cent: the cards moved right and up. | largest difference 2.1e-7 | yes |
| FX-CW-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-030 frame 0: Z position 1.7: the camera farther away, the cards smaller. | largest difference 1.8e-7 | yes |
| FX-CW-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-031 frame 0: Focal length 20 with the camera turned 40 about y: a wide lens. | largest difference 1.9e-7 | yes |
| FX-CW-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-032 frame 0: A distant light from the top left, depth 0.5. | largest difference 1.4e-7 | yes |
| FX-CW-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-033 frame 0: A point light in the middle, depth 0.3. | largest difference 1.5e-7 | yes |
| FX-CW-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-034 frame 0: Specular 1, sharpness 20: a highlight on the turning cards. | largest difference 2.0e-7 | yes |
| FX-CW-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-035 frame 0: Light colour #ffcc88 at intensity 1.5. | largest difference 2.0e-7 | yes |
| FX-CW-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-036 frame 0: Ambient 0.5, diffuse 0.25. | largest difference 1.1e-7 | yes |
| FX-CW-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-037 frame 0: Completion keyed 0 at frame 0 to 100 at frame 4: frame 0 is the picture, frame 2 FX-CW-004, frame 4 FX-CW-005. | largest difference 1.5e-7 | yes |
| FX-CW-037 frame 2: Completion keyed 0 at frame 0 to 100 at frame 4: frame 0 is the picture, frame 2 FX-CW-004, frame 4 FX-CW-005. | largest difference 1.5e-7 | yes |
| FX-CW-037 frame 4: Completion keyed 0 at frame 0 to 100 at frame 4: frame 0 is the picture, frame 2 FX-CW-004, frame 4 FX-CW-005. | largest difference 1.2e-7 | yes |
| FX-CW-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-038 frame 0: FX-CW-004 on the holder moved 3 right and 1 down: the same, moved. | largest difference 1.5e-7 | yes |
| FX-CW-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-039 frame 0: A back layer not in the composition, `gone`: clear backs, as FX-CW-003, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CW-039 frame 4: A back layer not in the composition, `gone`: clear backs, as FX-CW-003, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-CW-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CW-040 frame 0: The back layer with its in point at frame 4: clear backs before it, FX-CW-004 at it. | largest difference 1.5e-7 | yes |
| FX-CW-040 frame 4: The back layer with its in point at frame 4: clear backs before it, FX-CW-004 at it. | largest difference 1.5e-7 | yes |
| FX-CW-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-041 frame 0: Ten rows, one column, turning about x: venetian blinds. | largest difference 1.6e-7 | yes |
| FX-CW-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-042 frame 0: Flip axis "z". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-042 frame 4: Flip axis "z". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-043 frame 0: Flip order "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-043 frame 4: Flip order "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-044 frame 0: Rows 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-044 frame 4: Rows 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-045 frame 0: Columns 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-045 frame 4: Columns 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-046 frame 0: Card scale -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-046 frame 4: Card scale -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-047 frame 0: Card scale 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-047 frame 4: Card scale 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-048 frame 0: Timing randomness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-048 frame 4: Timing randomness 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-049 frame 0: Z position 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-049 frame 4: Z position 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-050 frame 0: Focal length 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-050 frame 4: Focal length 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-051 frame 0: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-051 frame 4: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-052 frame 0: Light colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-052 frame 4: Light colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-053 frame 0: Highlight sharpness 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-053 frame 4: Highlight sharpness 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-054 frame 0: A back layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-054 frame 4: A back layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-055 frame 0: Rows & Columns "both". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-055 frame 4: Rows & Columns "both". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-056 frame 0: Completion keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-056 frame 4: Completion keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview leaves every setting as it is, since each is a share of the drawing, a count or an angle | CardWipe { transition_completion: 50.0, transition_width: 50.0, back_layer: String("back"), back_map: None, rows_columns: "independent", rows: 2.0, columns: 4.0, card_scale: 1.0, flip_axis: "x", flip_direction: "positive", flip_order: "left_to_right", gradient_layer: String(""), gradient_map: None, timing_randomness: 0.0, random_seed: 0.0, x_rotation: 0.0, y_rotation: 0.0, z_rotation: 0.0, xy_position: [50.0, 50.0], z_position: 1.0, focal_length: 50.0, light_type: "distant", light_intensity: 1.0, light_color: "#ffffff", light_position: [50.0, 50.0], light_depth: 1.0, ambient_light: 0.25, diffuse_reflection: 0.75, specular_reflection: 0.0, highlight_sharpness: 5.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cw_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_004.json is saved with its twenty-eight settings, the back layer, completion 50, X, Y Position [50, 50] | {"ambient_light":0.25,"back_layer":"back","card_scale":1,"columns":4,"diffuse_reflection":0.75,"flip_axis":"x","flip_direction":"positive","flip_order":"left_to_right","focal_length":50,"gradient_layer":"","highlight_sharpness":5,"light_color":"#ffffff","light_depth":1,"light_intensity":1,"light_position":[50,50],"light_type":"distant","random_seed":0,"rows":2,"rows_columns":"independent","specular_reflection":0,"timing_randomness":0,"transition_completion":50,"transition_width":50,"x_rotation":0,"xy_position":[50,50],"y_rotation":0,"z_position":1,"z_rotation":0} | yes |
| fx_cw_054.json, its back layer written as the number 3, is saved as the number 3 | 3 | yes |
| the back and gradient pictures the effect reads are never saved | None None | yes |
| fx_cw_042.json is refused in a sentence naming flip axis | Card Wipe's flip axis is one of "x", "y", "random", and this is "z". | yes |
| fx_cw_043.json is refused in a sentence naming flip order | Card Wipe's flip order is one of "left_to_right", "right_to_left", "top_to_bottom", "bottom_to_top", "top_left_to_bottom_right", "bottom_left_to_top_right", "top_right_to_bottom_left", "bottom_right_to_top_left", "gradient", and this is "diagonal". | yes |
| fx_cw_044.json is refused in a sentence naming rows | Card Wipe's rows runs from 1 to 1000, and this is 0. | yes |
| fx_cw_045.json is refused in a sentence naming columns | Card Wipe's columns runs from 1 to 1000, and this is 1001. | yes |
| fx_cw_046.json is refused in a sentence naming card scale | Card Wipe's card scale runs from 0 to 10, and this is -1. | yes |
| fx_cw_047.json is refused in a sentence naming card scale | Card Wipe's card scale runs from 0 to 10, and this is 11. | yes |
| fx_cw_048.json is refused in a sentence naming timing randomness | Card Wipe's timing randomness runs from 0 to 1, and this is 1.5. | yes |
| fx_cw_049.json is refused in a sentence naming z position | Card Wipe's z position runs from 0.01 to 10, and this is 0. | yes |
| fx_cw_050.json is refused in a sentence naming focal length | Card Wipe's focal length runs from 1 to 1000, and this is 0. | yes |
| fx_cw_051.json is refused in a sentence naming light type | Card Wipe's light type is one of "distant", "point", and this is "spot". | yes |
| fx_cw_052.json is refused in a sentence naming light colour | Card Wipe's light colour is written #rrggbb, and this is "white". | yes |
| fx_cw_053.json is refused in a sentence naming highlight sharpness | Card Wipe's highlight sharpness runs from 1 to 100, and this is 0. | yes |
| fx_cw_054.json is refused in a sentence naming back layer | Card Wipe's back layer is the name of a layer of this composition, and this is 3. | yes |
| fx_cw_055.json is refused in a sentence naming rows & columns | Card Wipe's rows & columns is one of "independent", "columns_follow_rows", and this is "both". | yes |
| a file with a Card Wipe whose rows is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Wipe whose X, Y Position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Wipe with no flip order is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Wipe with no back layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-580)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cw_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_049.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_052.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_054.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_055.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_056.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1291691 pixels changed | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 0, Full | largest difference 1 of 255, 616 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 100, Full | largest difference 1 of 255, 285 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 239, Full | largest difference 1 of 255, 445 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 100, Draft | largest difference 1 of 255, 21 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 40, the background layer-4 the back on three layers, frame 239, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073598 pixels changed | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 0, Full | largest difference 1 of 255, 577 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 100, Full | largest difference 1 of 255, 517 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 239, Full | largest difference 1 of 255, 676 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 0, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe completion 60 by the gradient layer-4, random axis and direction, shuffled, the camera turned, a point light with a highlight on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2031911 pixels changed | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 0, Full | largest difference 1 of 255, 216 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 100, Full | largest difference 1 of 255, 162 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 239, Full | largest difference 1 of 255, 199 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 0, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe card scale 0.7, 20 rows by 30 columns, no back, completion 50 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1295668 pixels changed | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 100, Full | largest difference 1 of 255, 258 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 100, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Card Wipe Transition Completion keyed 0 to 100 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-580 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_halfway_left_to_right.png: drawn without a warning, the street changed | 68902 pixels changed; pixel (210, 118) [123, 145, 164, 255]; warnings [] | yes |
| 2_halfway_turned_camera_lit.png: drawn without a warning, the street changed | 129600 pixels changed; pixel (210, 118) [97, 116, 133, 255]; warnings [] | yes |
| 3_gradient_order.png: drawn without a warning, the street changed | 75836 pixels changed; pixel (210, 118) [105, 124, 140, 255]; warnings [] | yes |
| 4_shuffled_random_flips.png: drawn without a warning, the street changed | 67187 pixels changed; pixel (210, 118) [163, 191, 216, 255]; warnings [] | yes |
| 5_small_cards_no_back.png: drawn without a warning, the street changed | 114787 pixels changed; pixel (210, 118) [0, 0, 0, 0]; warnings [] | yes |
| 6_venetian_blinds.png: drawn without a warning, the street changed | 129600 pixels changed; pixel (210, 118) [0, 0, 0, 0]; warnings [] | yes |
| 7_all_turned_over.png: drawn without a warning, the street changed | 15444 pixels changed; pixel (210, 118) [163, 191, 216, 255]; warnings [] | yes |

## Result

300 of 300 checks pass.
